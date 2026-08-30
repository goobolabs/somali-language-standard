use std::path::{Path, PathBuf};

use sls_validators::validate_path;

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

#[test]
fn valid_metadata_json_passes() {
    let report = validate_path(
        &repository_path("schemas/metadata-common.schema.json"),
        &repository_path("tools/validators/tests/fixtures/valid-metadata.json"),
    )
    .expect("schema and fixture should be readable");

    assert!(report.is_valid(), "{:?}", report.diagnostics);
    assert_eq!(report.records_checked, 1);
}

#[test]
fn invalid_metadata_json_fails() {
    let report = validate_path(
        &repository_path("schemas/metadata-common.schema.json"),
        &repository_path("tools/validators/tests/fixtures/invalid-metadata.json"),
    )
    .expect("schema and fixture should be readable");

    assert!(!report.is_valid());
    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("source"))
    );
    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.instance_path == "/date_added")
    );
    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.instance_path == "/review_status")
    );
}

#[test]
fn invalid_jsonl_reports_the_record_line() {
    let report = validate_path(
        &repository_path("schemas/metadata-common.schema.json"),
        &repository_path("tools/validators/tests/fixtures/invalid-metadata.jsonl"),
    )
    .expect("schema and fixture should be readable");

    assert!(!report.is_valid());
    assert_eq!(report.records_checked, 2);
    assert!(
        report
            .diagnostics
            .iter()
            .all(|diagnostic| diagnostic.line == Some(2))
    );
}

#[test]
fn blank_jsonl_lines_are_rejected() {
    let report = validate_path(
        &repository_path("schemas/metadata-common.schema.json"),
        &repository_path("tools/validators/tests/fixtures/blank-line-metadata.jsonl"),
    )
    .expect("schema and fixture should be readable");

    assert!(!report.is_valid());
    assert_eq!(report.records_checked, 2);
    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic.line == Some(2) && diagnostic.message.contains("blank lines")
    }));
}
