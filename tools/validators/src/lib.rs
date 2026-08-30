use std::fmt;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use jsonschema::Validator;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub input: PathBuf,
    pub line: Option<usize>,
    pub instance_path: String,
    pub message: String,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.input.display())?;
        if let Some(line) = self.line {
            write!(formatter, ":{line}")?;
        }
        if !self.instance_path.is_empty() {
            write!(formatter, " {}", self.instance_path)?;
        }
        write!(formatter, ": {}", self.message)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    pub records_checked: usize,
    pub diagnostics: Vec<Diagnostic>,
}

impl ValidationReport {
    pub fn is_valid(&self) -> bool {
        self.diagnostics.is_empty()
    }
}

pub fn validate_path(schema_path: &Path, input_path: &Path) -> Result<ValidationReport, String> {
    let schema = read_json(schema_path, "schema")?;
    validate_schema_contract(schema_path, &schema)?;

    let validator = jsonschema::options()
        .should_validate_formats(true)
        .should_ignore_unknown_formats(false)
        .offline()
        .build(&schema)
        .map_err(|error| {
            format!(
                "could not compile schema {}: {error}",
                schema_path.display()
            )
        })?;

    match input_path
        .extension()
        .and_then(|extension| extension.to_str())
    {
        Some("json") => validate_json(input_path, &validator),
        Some("jsonl") => validate_jsonl(input_path, &validator),
        _ => Err(format!(
            "unsupported input format for {}; expected .json or .jsonl",
            input_path.display()
        )),
    }
}

fn read_json(path: &Path, kind: &str) -> Result<Value, String> {
    let content = fs::read_to_string(path)
        .map_err(|error| format!("could not read {kind} {}: {error}", path.display()))?;
    serde_json::from_str(&content).map_err(|error| {
        format!(
            "could not parse {kind} {} at line {}, column {}: {error}",
            path.display(),
            error.line(),
            error.column()
        )
    })
}

fn validate_schema_contract(schema_path: &Path, schema: &Value) -> Result<(), String> {
    let schema_version = schema.get("schema_version").and_then(Value::as_str);
    if schema_version.is_none_or(str::is_empty) {
        return Err(format!(
            "schema {} must carry a non-empty top-level schema_version",
            schema_path.display()
        ));
    }

    jsonschema::meta::validate(schema).map_err(|error| {
        format!(
            "schema {} does not conform to its declared meta-schema at {}: {error}",
            schema_path.display(),
            error.instance_path()
        )
    })
}

fn validate_json(input_path: &Path, validator: &Validator) -> Result<ValidationReport, String> {
    let instance = read_json(input_path, "input")?;
    Ok(ValidationReport {
        records_checked: 1,
        diagnostics: collect_diagnostics(input_path, None, validator, &instance),
    })
}

fn validate_jsonl(input_path: &Path, validator: &Validator) -> Result<ValidationReport, String> {
    let input = fs::File::open(input_path)
        .map_err(|error| format!("could not read input {}: {error}", input_path.display()))?;
    let mut diagnostics = Vec::new();
    let mut records_checked = 0;

    for (index, line) in BufReader::new(input).lines().enumerate() {
        let line_number = index + 1;
        let line = line.map_err(|error| {
            format!(
                "could not read input {} at line {line_number}: {error}",
                input_path.display()
            )
        })?;
        if line.trim().is_empty() {
            diagnostics.push(Diagnostic {
                input: input_path.to_path_buf(),
                line: Some(line_number),
                instance_path: String::new(),
                message: "blank lines are not allowed in JSONL files".to_owned(),
            });
            continue;
        }

        records_checked += 1;
        match serde_json::from_str::<Value>(&line) {
            Ok(instance) => diagnostics.extend(collect_diagnostics(
                input_path,
                Some(line_number),
                validator,
                &instance,
            )),
            Err(error) => diagnostics.push(Diagnostic {
                input: input_path.to_path_buf(),
                line: Some(line_number),
                instance_path: String::new(),
                message: format!("invalid JSON at column {}: {error}", error.column()),
            }),
        }
    }

    Ok(ValidationReport {
        records_checked,
        diagnostics,
    })
}

fn collect_diagnostics(
    input_path: &Path,
    line: Option<usize>,
    validator: &Validator,
    instance: &Value,
) -> Vec<Diagnostic> {
    validator
        .iter_errors(instance)
        .map(|error| Diagnostic {
            input: input_path.to_path_buf(),
            line,
            instance_path: error.instance_path().to_string(),
            message: error.to_string(),
        })
        .collect()
}
