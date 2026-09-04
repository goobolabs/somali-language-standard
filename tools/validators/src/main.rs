use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use sls_validators::{check_repository, validate_path};

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Validate {
        schema_path: PathBuf,
        input_path: PathBuf,
    },
    Check {
        root: PathBuf,
    },
}

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
    let command = parse_arguments(arguments)?;
    let (report, success_message) = match command {
        Command::Validate {
            schema_path,
            input_path,
        } => {
            let report = validate_path(&schema_path, &input_path)?;
            let message = format!(
                "validated {} record(s) from {} against {}",
                report.records_checked,
                input_path.display(),
                schema_path.display()
            );
            (report, message)
        }
        Command::Check { root } => {
            let report = check_repository(&root)?;
            let message = format!(
                "repository check passed for {} routed record(s) under {}",
                report.records_checked,
                root.display()
            );
            (report, message)
        }
    };

    if report.is_valid() {
        println!("{success_message}");
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

fn parse_arguments(arguments: impl IntoIterator<Item = String>) -> Result<Command, String> {
    let mut arguments = arguments.into_iter();
    match arguments.next().as_deref() {
        Some("validate") => parse_validate_arguments(arguments),
        Some("check") => parse_check_arguments(arguments),
        Some(command) => Err(format!("unknown command: {command}")),
        None => Err("missing command".to_owned()),
    }
}

fn parse_validate_arguments(
    mut arguments: impl Iterator<Item = String>,
) -> Result<Command, String> {
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

    Ok(Command::Validate {
        schema_path: schema.ok_or_else(|| "missing --schema path".to_owned())?,
        input_path: input.ok_or_else(|| "missing --input path".to_owned())?,
    })
}

fn parse_check_arguments(mut arguments: impl Iterator<Item = String>) -> Result<Command, String> {
    let mut root = PathBuf::from(".");
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--root" => {
                root = PathBuf::from(
                    arguments
                        .next()
                        .ok_or_else(|| "--root requires a path".to_owned())?,
                );
            }
            unknown => return Err(format!("unknown argument: {unknown}")),
        }
    }
    Ok(Command::Check { root })
}

fn usage() -> &'static str {
    "Usage:\n  sls-validate validate --schema <schema.json> --input <data.json|data.jsonl>\n  sls-validate check [--root <repository>]"
}

#[cfg(test)]
mod tests {
    use super::{Command, parse_arguments};
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
            Ok(Command::Validate {
                schema_path: PathBuf::from("schema.json"),
                input_path: PathBuf::from("data.jsonl"),
            })
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

    #[test]
    fn parses_repository_check_with_default_root() {
        assert_eq!(
            parse_arguments(["check".to_owned()]),
            Ok(Command::Check {
                root: PathBuf::from(".")
            })
        );
    }

    #[test]
    fn parses_repository_check_root() {
        assert_eq!(
            parse_arguments([
                "check".to_owned(),
                "--root".to_owned(),
                "repository".to_owned(),
            ]),
            Ok(Command::Check {
                root: PathBuf::from("repository")
            })
        );
    }
}
