use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};
use sls_validators::check_repository;

struct TestRepository {
    root: PathBuf,
}

impl TestRepository {
    fn new(name: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should follow the Unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "sls-validator-test-{}-{name}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&root).expect("temporary repository should be creatable");
        let repository_schemas = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../schemas");
        let schema_target = root.join("schemas");
        fs::create_dir_all(&schema_target).expect("schema directory should be creatable");
        for entry in fs::read_dir(repository_schemas).expect("schemas should be readable") {
            let entry = entry.expect("schema entry should be readable");
            if entry
                .file_name()
                .to_str()
                .is_some_and(|name| name.ends_with(".schema.json"))
            {
                fs::copy(entry.path(), schema_target.join(entry.file_name()))
                    .expect("schema should be copied");
            }
        }
        Self { root }
    }

    fn write_json(&self, relative: &str, value: &Value) {
        let path = self.root.join(relative);
        fs::create_dir_all(path.parent().expect("fixture path should have a parent"))
            .expect("fixture directory should be creatable");
        fs::write(
            path,
            serde_json::to_vec_pretty(value).expect("fixture should serialize"),
        )
        .expect("fixture should be writable");
    }

    fn write_jsonl(&self, relative: &str, values: &[Value]) {
        let mut content = values
            .iter()
            .map(|value| serde_json::to_string(value).expect("fixture should serialize"))
            .collect::<Vec<_>>()
            .join("\n");
        content.push('\n');
        let path = self.root.join(relative);
        fs::create_dir_all(path.parent().expect("fixture path should have a parent"))
            .expect("fixture directory should be creatable");
        fs::write(path, content).expect("fixture should be writable");
    }

    fn write_text(&self, relative: &str, content: &str) {
        let path = self.root.join(relative);
        fs::create_dir_all(path.parent().expect("fixture path should have a parent"))
            .expect("fixture directory should be creatable");
        fs::write(path, content).expect("fixture should be writable");
    }
}

impl Drop for TestRepository {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn metadata() -> Value {
    json!({
        "contributor": "SLS maintainers",
        "source": "Synthetic structural fixture; not language data",
        "date_added": "2026-09-04",
        "review_status": "draft",
        "license": "CC BY 4.0",
        "schema_version": "1.0.0",
        "since_version": "0.3.0"
    })
}

fn create_valid_repository(name: &str) -> TestRepository {
    let repository = TestRepository::new(name);
    repository.write_json(
        "standards/registry.json",
        &json!({
            "standards": [
                { "id": "SLS-0003" },
                { "id": "SLS-0200" },
                { "id": "SLS-0400" }
            ]
        }),
    );
    repository.write_json(
        "data/terminology/_domains.json",
        &json!({
            "schema_version": "1.0.0",
            "domains": [
                {
                    "code": "ai",
                    "slug": "artificial-intelligence",
                    "name": "Artificial Intelligence",
                    "standard_id": "SLS-0200"
                }
            ]
        }),
    );
    repository.write_jsonl(
        "data/lexicon/core/a.jsonl",
        &[lexicon_record("sls:sent:000001")],
    );
    repository.write_jsonl(
        "data/corpora/example-sentences.jsonl",
        &[json!({
            "sls_id": "sls:sent:000001",
            "so": "synthetic-so-example",
            "dialect": "so",
            "metadata": metadata()
        })],
    );
    repository.write_jsonl(
        "data/terminology/artificial-intelligence.jsonl",
        &[terminology_record("sls:term:ai:000001")],
    );
    repository.write_jsonl(
        "data/translation/general/en-so.jsonl",
        &[json!({
            "sls_id": "sls:pair:000001",
            "en": "synthetic-en",
            "so": "synthetic-so",
            "register": "general",
            "translation_type": "natural",
            "domain": "artificial-intelligence",
            "metadata": metadata()
        })],
    );
    repository.write_text(
        "spec/grammar/0010-synthetic-rule.md",
        "# Synthetic rule fixture\n",
    );
    repository.write_jsonl(
        "data/grammar/rules.jsonl",
        &[grammar_record("spec/grammar/0010-synthetic-rule.md")],
    );
    repository.write_jsonl(
        "data/style/formal.jsonl",
        &[style_record("SLS-0400", "artificial-intelligence")],
    );
    repository.write_jsonl(
        "benchmarks/grammar/items.jsonl",
        &[benchmark_record("G10-R1")],
    );
    repository.write_jsonl(
        "ai/datasets/grammar-correction-pairs.jsonl",
        &[correction_record("SLS-0003:G10-R1")],
    );
    repository.write_text(
        "data/lexicon/morphology/noun-paradigms.jsonl",
        "not-routed-until-a-morphology-schema-exists\n",
    );
    repository.write_text("data/terminology/_template.jsonl", "not-routed-template\n");
    repository.write_text(
        "ai/datasets/sft-somali-v1.jsonl",
        "not-routed-until-an-instruction-schema-exists\n",
    );
    repository
}

fn terminology_record(sls_id: &str) -> Value {
    json!({
        "sls_id": sls_id,
        "domain": "artificial-intelligence",
        "en_term": "synthetic-en-term",
        "so_term": "synthetic-so-term",
        "definition_en": "Synthetic definition.",
        "definition_so": "synthetic-definition",
        "part_of_speech": "magac",
        "coinage_type": "neologism",
        "status": "proposed",
        "metadata": metadata()
    })
}

fn lexicon_record(sentence_id: &str) -> Value {
    json!({
        "sls_id": "sls:lex:000001",
        "word": "synthetic-headword",
        "part_of_speech": "magac",
        "gender": "masculine",
        "plural": "synthetic-plural",
        "dialect": "so",
        "definitions": [
            { "sense": 1, "en": "synthetic", "so_gloss": "synthetic-gloss" }
        ],
        "is_loanword": false,
        "example_sentences": [sentence_id],
        "metadata": metadata()
    })
}

fn grammar_record(spec_path: &str) -> Value {
    json!({
        "sls_id": "sls:rule:grammar:000001",
        "rule_id": "G10-R1",
        "standard_id": "SLS-0003",
        "spec_path": spec_path,
        "requirement": "Synthetic MUST requirement.",
        "normative_terms": ["MUST"],
        "positive_examples": [
            { "text": "synthetic-positive", "explanation": "Synthetic positive." }
        ],
        "negative_examples": [
            { "text": "synthetic-negative", "explanation": "Synthetic negative." }
        ],
        "metadata": metadata()
    })
}

fn style_record(standard_id: &str, domain: &str) -> Value {
    json!({
        "sls_id": "sls:style:000001",
        "standard_id": standard_id,
        "register": "formal",
        "preferred": "synthetic-preferred",
        "avoid": "synthetic-avoid",
        "rationale": "Synthetic rationale.",
        "domains": [domain],
        "metadata": metadata()
    })
}

fn benchmark_record(rule_reference: &str) -> Value {
    json!({
        "sls_id": "sls:bench:grammar:000001",
        "task": "grammar-correction",
        "input": "synthetic-input",
        "expected_output": "synthetic-output",
        "explanation": "Synthetic explanation.",
        "difficulty": "beginner",
        "rule_refs": [rule_reference],
        "metadata": metadata()
    })
}

fn correction_record(rule_reference: &str) -> Value {
    json!({
        "sls_id": "sls:corr:grammar:000001",
        "category": "grammar",
        "incorrect": "synthetic-incorrect",
        "corrected": "synthetic-corrected",
        "explanation": "Synthetic explanation.",
        "rule_refs": [rule_reference],
        "metadata": metadata()
    })
}

#[test]
fn repository_check_routes_every_record_type_and_resolves_references() {
    let repository = create_valid_repository("valid");
    let report = check_repository(&repository.root).expect("repository check should run");

    assert!(report.is_valid(), "{:?}", report.diagnostics);
    assert_eq!(report.records_checked, 9);
}

#[test]
fn repository_check_reports_duplicate_and_dangling_references() {
    let repository = create_valid_repository("invalid-cross-references");
    repository.write_jsonl(
        "data/terminology/artificial-intelligence.jsonl",
        &[
            terminology_record("sls:term:ml:000001"),
            terminology_record("sls:term:ml:000001"),
        ],
    );
    repository.write_jsonl(
        "data/translation/general/en-so.jsonl",
        &[json!({
            "sls_id": "sls:pair:000001",
            "en": "synthetic-en",
            "so": "synthetic-so",
            "register": "general",
            "translation_type": "natural",
            "domain": "unknown-domain",
            "metadata": metadata()
        })],
    );
    repository.write_jsonl(
        "data/lexicon/core/a.jsonl",
        &[lexicon_record("sls:sent:999999")],
    );
    repository.write_jsonl(
        "data/grammar/rules.jsonl",
        &[grammar_record("spec/grammar/0099-missing-rule.md")],
    );
    repository.write_jsonl(
        "data/style/formal.jsonl",
        &[style_record("SLS-0499", "unknown-domain")],
    );
    repository.write_jsonl(
        "benchmarks/grammar/items.jsonl",
        &[benchmark_record("G99-R99")],
    );
    repository.write_jsonl(
        "ai/datasets/grammar-correction-pairs.jsonl",
        &[correction_record("SLS-0999")],
    );

    let report = check_repository(&repository.root).expect("repository check should run");
    assert!(!report.is_valid());
    for expected_message in [
        "duplicate sls_id",
        "does not match domain",
        "unknown terminology domain",
        "unknown standard reference",
        "specification path does not exist",
        "unknown rule or standard reference",
        "unknown sls_id reference",
    ] {
        assert!(
            report
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message.contains(expected_message)),
            "missing diagnostic containing {expected_message:?}: {:?}",
            report.diagnostics
        );
    }
}
