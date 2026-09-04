use std::path::{Path, PathBuf};

use sls_validators::{ValidationReport, validate_path};

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn report(schema: &str, fixture: &str) -> ValidationReport {
    validate_path(&repository_path(schema), &repository_path(fixture))
        .expect("schemas and fixture should be readable")
}

fn assert_invalid_paths(report: &ValidationReport, expected_paths: &[&str]) {
    assert!(!report.is_valid());
    for expected_path in expected_paths {
        assert!(
            report
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.instance_path == *expected_path),
            "missing diagnostic for {expected_path}: {:?}",
            report.diagnostics
        );
    }
}

#[test]
fn valid_grammar_rule_passes() {
    let report = report(
        "schemas/grammar-rule.schema.json",
        "tools/validators/tests/fixtures/valid-grammar-rule.json",
    );
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn invalid_grammar_rule_fails_governed_fields() {
    let report = report(
        "schemas/grammar-rule.schema.json",
        "tools/validators/tests/fixtures/invalid-grammar-rule.json",
    );
    assert_invalid_paths(
        &report,
        &[
            "/sls_id",
            "/rule_id",
            "/standard_id",
            "/spec_path",
            "/requirement",
            "/normative_terms",
            "/positive_examples",
            "/metadata",
        ],
    );
    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic
            .instance_path
            .starts_with("/negative_examples/0/")
    }));
}

#[test]
fn valid_style_example_passes() {
    let report = report(
        "schemas/style-example.schema.json",
        "tools/validators/tests/fixtures/valid-style-example.json",
    );
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn invalid_style_example_fails_governed_fields() {
    let report = report(
        "schemas/style-example.schema.json",
        "tools/validators/tests/fixtures/invalid-style-example.json",
    );
    assert_invalid_paths(
        &report,
        &[
            "/sls_id",
            "/standard_id",
            "/register",
            "/preferred",
            "/avoid",
            "/rationale",
            "/domains",
            "/metadata",
        ],
    );
}

#[test]
fn valid_benchmark_item_passes() {
    let report = report(
        "schemas/benchmark-item.schema.json",
        "tools/validators/tests/fixtures/valid-benchmark-item.json",
    );
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn invalid_benchmark_item_fails_governed_fields() {
    let report = report(
        "schemas/benchmark-item.schema.json",
        "tools/validators/tests/fixtures/invalid-benchmark-item.json",
    );
    assert_invalid_paths(
        &report,
        &[
            "/sls_id",
            "/task",
            "/input",
            "/expected_output",
            "/explanation",
            "/difficulty",
            "/rule_refs",
            "/task_data",
            "/metadata",
        ],
    );
}

#[test]
fn valid_correction_pair_passes() {
    let report = report(
        "schemas/correction-pair.schema.json",
        "tools/validators/tests/fixtures/valid-correction-pair.json",
    );
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn invalid_correction_pair_fails_governed_fields() {
    let report = report(
        "schemas/correction-pair.schema.json",
        "tools/validators/tests/fixtures/invalid-correction-pair.json",
    );
    assert_invalid_paths(
        &report,
        &[
            "/sls_id",
            "/category",
            "/incorrect",
            "/corrected",
            "/explanation",
            "/rule_refs",
            "/register",
            "/metadata",
        ],
    );
}
