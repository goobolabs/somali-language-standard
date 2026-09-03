# SLS Validators

Rust validators for the machine-readable Somali Language Standard records.

## Run locally

From the repository root, validate either a JSON object or every record in a
JSONL file:

```sh
cargo run -p sls-validators -- validate \
  --schema schemas/metadata-common.schema.json \
  --input tools/validators/tests/fixtures/valid-metadata.json
```

The command exits with status 0 only when the schema is valid and every input
record conforms. Validation is offline, JSON Schema `format` assertions are
enabled, and JSONL diagnostics include the failing record's line number.
Sibling `*.schema.json` files are loaded into a local registry, allowing shared
references such as `metadata-common.schema.json` without network access. Every
loaded schema must have a unique `$id` and a SemVer `schema_version`.

## Test

```sh
cargo test --workspace
```

The fixture suite includes accepted and deliberately invalid metadata records.
The invalid fixture is expected to fail validation; the test verifies that it
does.
