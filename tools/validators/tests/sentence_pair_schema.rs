use std::path::{Path, PathBuf};

use sls_validators::validate_path;

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

#[test]
fn valid_sentence_pair_passes_shared_metadata_contract() {
    let report = validate_path(
        &repository_path("schemas/sentence-pair.schema.json"),
        &repository_path("tools/validators/tests/fixtures/valid-sentence-pair.json"),
    )
    .expect("schemas and fixture should be readable");

    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn invalid_sentence_pair_fails_governed_fields() {
    let report = validate_path(
        &repository_path("schemas/sentence-pair.schema.json"),
        &repository_path("tools/validators/tests/fixtures/invalid-sentence-pair.json"),
    )
    .expect("schemas and fixture should be readable");

    assert!(!report.is_valid());
    for expected_path in [
        "/sls_id",
        "/register",
        "/translation_type",
        "/domain",
        "/metadata",
    ] {
        assert!(
            report
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.instance_path == expected_path),
            "missing diagnostic for {expected_path}: {:?}",
            report.diagnostics
        );
    }
    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("so"))
    );
}

#[test]
fn valid_example_sentence_passes_shared_metadata_contract() {
    let report = validate_path(
        &repository_path("schemas/example-sentence.schema.json"),
        &repository_path("tools/validators/tests/fixtures/valid-example-sentence.json"),
    )
    .expect("schemas and fixture should be readable");

    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn invalid_example_sentence_fails_governed_fields() {
    let report = validate_path(
        &repository_path("schemas/example-sentence.schema.json"),
        &repository_path("tools/validators/tests/fixtures/invalid-example-sentence.json"),
    )
    .expect("schemas and fixture should be readable");

    assert!(!report.is_valid());
    for expected_path in [
        "/sls_id",
        "/so",
        "/en",
        "/dialect",
        "/register",
        "/metadata",
    ] {
        assert!(
            report
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.instance_path == expected_path),
            "missing diagnostic for {expected_path}: {:?}",
            report.diagnostics
        );
    }
}
