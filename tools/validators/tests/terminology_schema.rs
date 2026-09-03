use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;
use sls_validators::validate_path;

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

#[test]
fn controlled_domain_vocabulary_passes_its_schema() {
    let report = validate_path(
        &repository_path("schemas/terminology-domains.schema.json"),
        &repository_path("data/terminology/_domains.json"),
    )
    .expect("schema and controlled vocabulary should be readable");

    assert!(report.is_valid(), "{:?}", report.diagnostics);
    assert_eq!(report.records_checked, 1);
}

#[test]
fn controlled_domains_match_the_reserved_standards_registry() {
    let domains: Value = serde_json::from_str(
        &fs::read_to_string(repository_path("data/terminology/_domains.json"))
            .expect("controlled vocabulary should be readable"),
    )
    .expect("controlled vocabulary should be JSON");
    let registry: Value = serde_json::from_str(
        &fs::read_to_string(repository_path("standards/registry.json"))
            .expect("standards registry should be readable"),
    )
    .expect("standards registry should be JSON");

    let domains = domains["domains"]
        .as_array()
        .expect("domains should be an array");
    assert_eq!(domains.len(), 20);

    let mut codes = BTreeSet::new();
    let mut slugs = BTreeSet::new();
    let mut standard_ids = BTreeSet::new();
    let standards = registry["standards"]
        .as_array()
        .expect("standards should be an array");

    for domain in domains {
        let code = domain["code"].as_str().expect("code should be a string");
        let slug = domain["slug"].as_str().expect("slug should be a string");
        let name = domain["name"].as_str().expect("name should be a string");
        let standard_id = domain["standard_id"]
            .as_str()
            .expect("standard_id should be a string");

        assert!(codes.insert(code), "duplicate domain code: {code}");
        assert!(slugs.insert(slug), "duplicate domain slug: {slug}");
        assert!(
            standard_ids.insert(standard_id),
            "duplicate terminology standard: {standard_id}"
        );

        let standard = standards
            .iter()
            .find(|standard| standard["id"].as_str() == Some(standard_id))
            .unwrap_or_else(|| panic!("missing terminology standard: {standard_id}"));
        assert_eq!(
            standard["title"].as_str(),
            Some(format!("{name} Terminology Standard").as_str())
        );
        assert_eq!(standard["category"].as_str(), Some("terminology"));
    }
}

#[test]
fn valid_terminology_entry_passes_shared_metadata_contract() {
    let report = validate_path(
        &repository_path("schemas/terminology-entry.schema.json"),
        &repository_path("tools/validators/tests/fixtures/valid-terminology-entry.json"),
    )
    .expect("schemas and fixture should be readable");

    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn invalid_terminology_entry_fails_governed_fields() {
    let report = validate_path(
        &repository_path("schemas/terminology-entry.schema.json"),
        &repository_path("tools/validators/tests/fixtures/invalid-terminology-entry.json"),
    )
    .expect("schemas and fixture should be readable");

    assert!(!report.is_valid());
    for expected_path in [
        "/sls_id",
        "/domain",
        "/so_term_alternatives",
        "/part_of_speech",
        "/coinage_type",
        "/status",
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
            .any(|diagnostic| diagnostic.message.contains("definition_so"))
    );
    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("example_so"))
    );
}
