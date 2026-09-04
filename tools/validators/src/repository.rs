use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::{Diagnostic, ValidationReport, validate_path};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RecordKind {
    DomainConfig,
    Lexicon,
    ExampleSentence,
    Terminology,
    SentencePair,
    GrammarRule,
    StyleExample,
    Benchmark,
    CorrectionPair,
}

#[derive(Debug)]
struct Route {
    kind: RecordKind,
    schema: &'static str,
    input: PathBuf,
}

#[derive(Debug)]
struct Record {
    kind: RecordKind,
    input: PathBuf,
    line: Option<usize>,
    value: Value,
}

pub fn check_repository(root: &Path) -> Result<ValidationReport, String> {
    let routes = discover_routes(root)?;
    let mut report = ValidationReport {
        records_checked: 0,
        diagnostics: Vec::new(),
    };
    let mut records = Vec::new();

    for route in &routes {
        let route_report = validate_path(&root.join("schemas").join(route.schema), &route.input)?;
        report.records_checked += route_report.records_checked;
        report.diagnostics.extend(route_report.diagnostics);
        records.extend(read_records(route)?);
    }

    check_cross_references(root, &records, &mut report.diagnostics)?;
    report.diagnostics.sort_by(|left, right| {
        (&left.input, left.line, &left.instance_path, &left.message).cmp(&(
            &right.input,
            right.line,
            &right.instance_path,
            &right.message,
        ))
    });
    Ok(report)
}

fn discover_routes(root: &Path) -> Result<Vec<Route>, String> {
    let mut routes = Vec::new();
    let domain_config = root.join("data/terminology/_domains.json");
    if !domain_config.is_file() {
        return Err(format!(
            "required terminology domain vocabulary is missing: {}",
            domain_config.display()
        ));
    }
    add_exact_route(
        &mut routes,
        domain_config,
        RecordKind::DomainConfig,
        "terminology-domains.schema.json",
    );
    add_jsonl_routes(
        &mut routes,
        &root.join("data/lexicon/core"),
        RecordKind::Lexicon,
        "lexicon-entry.schema.json",
        |_| true,
    )?;
    add_exact_route(
        &mut routes,
        root.join("data/lexicon/loanwords.jsonl"),
        RecordKind::Lexicon,
        "lexicon-entry.schema.json",
    );
    add_exact_route(
        &mut routes,
        root.join("data/corpora/example-sentences.jsonl"),
        RecordKind::ExampleSentence,
        "example-sentence.schema.json",
    );
    add_jsonl_routes(
        &mut routes,
        &root.join("data/terminology"),
        RecordKind::Terminology,
        "terminology-entry.schema.json",
        |path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| !name.starts_with('_'))
        },
    )?;
    add_jsonl_routes(
        &mut routes,
        &root.join("data/translation"),
        RecordKind::SentencePair,
        "sentence-pair.schema.json",
        |_| true,
    )?;
    add_exact_route(
        &mut routes,
        root.join("data/grammar/rules.jsonl"),
        RecordKind::GrammarRule,
        "grammar-rule.schema.json",
    );
    add_jsonl_routes(
        &mut routes,
        &root.join("data/style"),
        RecordKind::StyleExample,
        "style-example.schema.json",
        |_| true,
    )?;
    add_jsonl_routes(
        &mut routes,
        &root.join("benchmarks"),
        RecordKind::Benchmark,
        "benchmark-item.schema.json",
        |_| true,
    )?;
    add_jsonl_routes(
        &mut routes,
        &root.join("ai/datasets"),
        RecordKind::CorrectionPair,
        "correction-pair.schema.json",
        |path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| {
                    name == "correction-pairs.jsonl" || name.ends_with("-correction-pairs.jsonl")
                })
        },
    )?;

    routes.sort_by(|left, right| left.input.cmp(&right.input));
    Ok(routes)
}

fn add_exact_route(
    routes: &mut Vec<Route>,
    input: PathBuf,
    kind: RecordKind,
    schema: &'static str,
) {
    if input.is_file() {
        routes.push(Route {
            kind,
            schema,
            input,
        });
    }
}

fn add_jsonl_routes(
    routes: &mut Vec<Route>,
    directory: &Path,
    kind: RecordKind,
    schema: &'static str,
    include: impl Fn(&Path) -> bool + Copy,
) -> Result<(), String> {
    if !directory.exists() {
        return Ok(());
    }

    let mut entries = fs::read_dir(directory)
        .map_err(|error| format!("could not read {}: {error}", directory.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("could not read {}: {error}", directory.display()))?;
    entries.sort_by_key(fs::DirEntry::path);

    for entry in entries {
        let file_type = entry
            .file_type()
            .map_err(|error| format!("could not inspect {}: {error}", entry.path().display()))?;
        if file_type.is_dir() {
            add_jsonl_routes(routes, &entry.path(), kind, schema, include)?;
        } else if file_type.is_file()
            && entry.path().extension().and_then(|value| value.to_str()) == Some("jsonl")
            && include(&entry.path())
        {
            routes.push(Route {
                kind,
                schema,
                input: entry.path(),
            });
        }
    }
    Ok(())
}

fn read_records(route: &Route) -> Result<Vec<Record>, String> {
    if route.input.extension().and_then(|value| value.to_str()) == Some("json") {
        let content = fs::read_to_string(&route.input)
            .map_err(|error| format!("could not read {}: {error}", route.input.display()))?;
        return Ok(serde_json::from_str(&content)
            .ok()
            .map(|value| {
                vec![Record {
                    kind: route.kind,
                    input: route.input.clone(),
                    line: None,
                    value,
                }]
            })
            .unwrap_or_default());
    }

    let input = fs::File::open(&route.input)
        .map_err(|error| format!("could not read {}: {error}", route.input.display()))?;
    let mut records = Vec::new();
    for (index, line) in BufReader::new(input).lines().enumerate() {
        let line = line.map_err(|error| {
            format!(
                "could not read {} at line {}: {error}",
                route.input.display(),
                index + 1
            )
        })?;
        if let Ok(value) = serde_json::from_str(&line) {
            records.push(Record {
                kind: route.kind,
                input: route.input.clone(),
                line: Some(index + 1),
                value,
            });
        }
    }
    Ok(records)
}

fn check_cross_references(
    root: &Path,
    records: &[Record],
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<(), String> {
    let standards = load_standard_ids(root)?;
    let mut domains = BTreeMap::new();
    let mut domain_codes = BTreeSet::new();
    let mut domain_standard_ids = BTreeSet::new();

    for record in records
        .iter()
        .filter(|record| record.kind == RecordKind::DomainConfig)
    {
        if let Some(values) = record.value.get("domains").and_then(Value::as_array) {
            for (index, value) in values.iter().enumerate() {
                let Some(slug) = value.get("slug").and_then(Value::as_str) else {
                    continue;
                };
                let Some(code) = value.get("code").and_then(Value::as_str) else {
                    continue;
                };
                let Some(standard_id) = value.get("standard_id").and_then(Value::as_str) else {
                    continue;
                };
                if domains.insert(slug.to_owned(), code.to_owned()).is_some() {
                    push_diagnostic(
                        diagnostics,
                        record,
                        format!("/domains/{index}/slug"),
                        format!("duplicate terminology domain slug {slug:?}"),
                    );
                }
                if !domain_codes.insert(code) {
                    push_diagnostic(
                        diagnostics,
                        record,
                        format!("/domains/{index}/code"),
                        format!("duplicate terminology domain code {code:?}"),
                    );
                }
                if !domain_standard_ids.insert(standard_id) {
                    push_diagnostic(
                        diagnostics,
                        record,
                        format!("/domains/{index}/standard_id"),
                        format!("duplicate terminology standard {standard_id:?}"),
                    );
                }
                if !standards.contains(standard_id) {
                    push_diagnostic(
                        diagnostics,
                        record,
                        format!("/domains/{index}/standard_id"),
                        format!("unknown standard reference {standard_id:?}"),
                    );
                }
            }
        }
    }

    let mut seen_ids: BTreeMap<&str, (&Path, Option<usize>)> = BTreeMap::new();
    let mut rule_refs = BTreeSet::new();
    for record in records {
        if let Some(sls_id) = record.value.get("sls_id").and_then(Value::as_str)
            && let Some((first_path, first_line)) =
                seen_ids.insert(sls_id, (&record.input, record.line))
        {
            let first_location = match first_line {
                Some(line) => format!("{}:{line}", first_path.display()),
                None => first_path.display().to_string(),
            };
            push_diagnostic(
                diagnostics,
                record,
                "/sls_id".to_owned(),
                format!("duplicate sls_id {sls_id:?}; first defined at {first_location}"),
            );
        }

        if record.kind == RecordKind::GrammarRule
            && let (Some(rule_id), Some(standard_id)) = (
                record.value.get("rule_id").and_then(Value::as_str),
                record.value.get("standard_id").and_then(Value::as_str),
            )
        {
            rule_refs.insert(rule_id.to_owned());
            rule_refs.insert(format!("{standard_id}:{rule_id}"));
        }
    }

    for record in records {
        match record.kind {
            RecordKind::Terminology => {
                check_domain(record, &domains, diagnostics);
                check_terminology_code(record, &domains, diagnostics);
            }
            RecordKind::SentencePair => check_domain(record, &domains, diagnostics),
            RecordKind::GrammarRule => {
                check_standard(record, &standards, diagnostics);
                check_spec_path(root, record, diagnostics);
            }
            RecordKind::StyleExample => {
                check_standard(record, &standards, diagnostics);
                check_domains(record, &domains, diagnostics);
            }
            RecordKind::Benchmark | RecordKind::CorrectionPair => {
                check_rule_refs(record, &standards, &rule_refs, diagnostics);
            }
            RecordKind::Lexicon => {
                check_sls_id_references(record, "example_sentences", &seen_ids, diagnostics)
            }
            RecordKind::DomainConfig | RecordKind::ExampleSentence => {}
        }
    }
    Ok(())
}

fn check_sls_id_references(
    record: &Record,
    field: &str,
    ids: &BTreeMap<&str, (&Path, Option<usize>)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let Some(values) = record.value.get(field).and_then(Value::as_array) {
        for (index, value) in values.iter().enumerate() {
            if let Some(reference) = value.as_str()
                && !ids.contains_key(reference)
            {
                push_diagnostic(
                    diagnostics,
                    record,
                    format!("/{field}/{index}"),
                    format!("unknown sls_id reference {reference:?}"),
                );
            }
        }
    }
}

fn load_standard_ids(root: &Path) -> Result<BTreeSet<String>, String> {
    let path = root.join("standards/registry.json");
    let content = fs::read_to_string(&path)
        .map_err(|error| format!("could not read {}: {error}", path.display()))?;
    let registry: Value = serde_json::from_str(&content)
        .map_err(|error| format!("could not parse {}: {error}", path.display()))?;
    Ok(registry
        .get("standards")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|standard| standard.get("id").and_then(Value::as_str))
        .map(str::to_owned)
        .collect())
}

fn check_domain(
    record: &Record,
    domains: &BTreeMap<String, String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let Some(domain) = record.value.get("domain").and_then(Value::as_str)
        && !domains.contains_key(domain)
    {
        push_diagnostic(
            diagnostics,
            record,
            "/domain".to_owned(),
            format!("unknown terminology domain {domain:?}"),
        );
    }
}

fn check_domains(
    record: &Record,
    domains: &BTreeMap<String, String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let Some(values) = record.value.get("domains").and_then(Value::as_array) {
        for (index, value) in values.iter().enumerate() {
            if let Some(domain) = value.as_str()
                && !domains.contains_key(domain)
            {
                push_diagnostic(
                    diagnostics,
                    record,
                    format!("/domains/{index}"),
                    format!("unknown terminology domain {domain:?}"),
                );
            }
        }
    }
}

fn check_terminology_code(
    record: &Record,
    domains: &BTreeMap<String, String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(domain) = record.value.get("domain").and_then(Value::as_str) else {
        return;
    };
    let Some(expected_code) = domains.get(domain) else {
        return;
    };
    let Some(code) = record
        .value
        .get("sls_id")
        .and_then(Value::as_str)
        .and_then(|value| value.split(':').nth(2))
    else {
        return;
    };
    if code != expected_code {
        push_diagnostic(
            diagnostics,
            record,
            "/sls_id".to_owned(),
            format!(
                "terminology ID code {code:?} does not match domain {domain:?} code {expected_code:?}"
            ),
        );
    }
}

fn check_standard(
    record: &Record,
    standards: &BTreeSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let Some(standard_id) = record.value.get("standard_id").and_then(Value::as_str)
        && !standards.contains(standard_id)
    {
        push_diagnostic(
            diagnostics,
            record,
            "/standard_id".to_owned(),
            format!("unknown standard reference {standard_id:?}"),
        );
    }
}

fn check_spec_path(root: &Path, record: &Record, diagnostics: &mut Vec<Diagnostic>) {
    if let Some(spec_path) = record.value.get("spec_path").and_then(Value::as_str)
        && !root.join(spec_path).is_file()
    {
        push_diagnostic(
            diagnostics,
            record,
            "/spec_path".to_owned(),
            format!("specification path does not exist: {spec_path}"),
        );
    }
}

fn check_rule_refs(
    record: &Record,
    standards: &BTreeSet<String>,
    rules: &BTreeSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if let Some(values) = record.value.get("rule_refs").and_then(Value::as_array) {
        for (index, value) in values.iter().enumerate() {
            if let Some(reference) = value.as_str()
                && !standards.contains(reference)
                && !rules.contains(reference)
            {
                push_diagnostic(
                    diagnostics,
                    record,
                    format!("/rule_refs/{index}"),
                    format!("unknown rule or standard reference {reference:?}"),
                );
            }
        }
    }
}

fn push_diagnostic(
    diagnostics: &mut Vec<Diagnostic>,
    record: &Record,
    instance_path: String,
    message: String,
) {
    diagnostics.push(Diagnostic {
        input: record.input.clone(),
        line: record.line,
        instance_path,
        message,
    });
}
