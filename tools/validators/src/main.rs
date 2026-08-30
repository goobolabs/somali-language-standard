use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use sls_validators::validate_path;

fn main() -> ExitCode {
    match run(env::args().skip(1)) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("error: {error}");
            eprintln!();
            eprintln!("{}", usage());
            ExitCode::FAILURE
        }
    }
}

fn run(arguments: impl IntoIterator<Item = String>) -> Result<ExitCode, String> {
    let (schema_path, input_path) = parse_arguments(arguments)?;
    let report = validate_path(&schema_path, &input_path)?;

    if report.is_valid() {
        println!(
            "validated {} record(s) from {} against {}",
            report.records_checked,
            input_path.display(),
            schema_path.display()
        );
        return Ok(ExitCode::SUCCESS);
    }

    for diagnostic in &report.diagnostics {
        eprintln!("{diagnostic}");
    }
    eprintln!(
        "validation failed with {} error(s) across {} record(s)",
        report.diagnostics.len(),
        report.records_checked
    );
    Ok(ExitCode::FAILURE)
}

fn parse_arguments(
    arguments: impl IntoIterator<Item = String>,
) -> Result<(PathBuf, PathBuf), String> {
    let mut arguments = arguments.into_iter();
    match arguments.next().as_deref() {
        Some("validate") => {}
        Some(command) => return Err(format!("unknown command: {command}")),
        None => return Err("missing validate command".to_owned()),
    }

    let mut schema = None;
    let mut input = None;
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--schema" => {
                schema = Some(PathBuf::from(
                    arguments
                        .next()
                        .ok_or_else(|| "--schema requires a path".to_owned())?,
                ));
            }
            "--input" => {
                input = Some(PathBuf::from(
                    arguments
                        .next()
                        .ok_or_else(|| "--input requires a path".to_owned())?,
                ));
            }
            unknown => return Err(format!("unknown argument: {unknown}")),
        }
    }

    Ok((
        schema.ok_or_else(|| "missing --schema path".to_owned())?,
        input.ok_or_else(|| "missing --input path".to_owned())?,
    ))
}

fn usage() -> &'static str {
    "Usage: sls-validate validate --schema <schema.json> --input <data.json|data.jsonl>"
}

#[cfg(test)]
mod tests {
    use super::parse_arguments;
    use std::path::PathBuf;

    #[test]
    fn parses_validate_arguments() {
        let result = parse_arguments([
            "validate".to_owned(),
            "--schema".to_owned(),
            "schema.json".to_owned(),
            "--input".to_owned(),
            "data.jsonl".to_owned(),
        ]);

        assert_eq!(
            result,
            Ok((PathBuf::from("schema.json"), PathBuf::from("data.jsonl")))
        );
    }

    #[test]
    fn rejects_missing_input() {
        let result = parse_arguments([
            "validate".to_owned(),
            "--schema".to_owned(),
            "schema.json".to_owned(),
        ]);

        assert_eq!(result, Err("missing --input path".to_owned()));
    }
}
