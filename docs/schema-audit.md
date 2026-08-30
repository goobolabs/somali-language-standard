# Phase 4 Record-Format Audit

- **Status:** Active implementation record
- **Date:** 2026-08-31
- **Scope:** Milestone 3 / Phase 4 machine-readable schemas and validators

## Current repository state

The schema and dataset directories were placeholders when Phase 4 began. The
repository had no schema files, structured linguistic JSONL records, Rust
workspace, or executable validator. Existing JSON files under `standards/`
describe the standards registry and lifecycle metadata; they are not instances
of the planned linguistic record schemas.

The first implementation slice therefore establishes the shared metadata
contract and a record-type-independent validator before defining payload fields
for the individual record types.

## Planned schema routing

| Schema | Planned records | Current routing state |
|---|---|---|
| `metadata-common.schema.json` | The `metadata` object in every linguistic JSONL record | Implemented as the shared Phase 4 foundation |
| `lexicon-entry.schema.json` | `data/lexicon/**/*.jsonl` | Path exists; no records yet |
| `terminology-entry.schema.json` | Domain files under `data/terminology/` | Path exists; controlled domain vocabulary not yet implemented |
| `sentence-pair.schema.json` | Translation pairs and selected corpus records | Path name must be reconciled before routing is enforced |
| `grammar-rule.schema.json` | Machine-readable grammar requirements | No canonical record path is defined yet |
| `style-example.schema.json` | Structured style-guide examples | No canonical record path is defined yet |
| `benchmark-item.schema.json` | `benchmarks/**/*.jsonl` | Placeholder path exists; no records yet |
| `correction-pair.schema.json` | Correction datasets under `ai/` | Architecture and implemented scaffold paths differ |

## Decisions in this slice

### JSON Schema dialect and schema versions

Every schema uses JSON Schema draft 2020-12 and carries a top-level
`schema_version`. The initial schema version is `1.0.0`. Repository release
versions, standard document versions, and schema versions remain independent.

### Required common metadata

Every metadata object requires:

- `contributor`
- `source`
- `date_added`
- `review_status`
- `license`
- `schema_version`

The Phase 4 audit found that the old architecture example did not list `source`
or `schema_version` as required even though the repository principles and
contribution rules require provenance and schema-version completeness. The
architecture, style guide, and contribution rules are now synchronized with
the strict contract so an incomplete record cannot pass validation.

Optional fields are `reviewers`, `date_modified`, and `since_version`.
Unknown metadata keys are rejected. New fields must first be added to the
schema so contributors do not create incompatible record shapes inline.

### Review and licensing values

`review_status` is restricted to the lifecycle already documented by the
architecture: `draft`, `reviewed`, `verified`, or `deprecated`. Dataset records
are licensed under `CC BY 4.0`, so the metadata contract uses that exact value.
Code and schemas remain under the repository's MIT code license.

### Validation behavior

The Rust validator:

- validates the schema itself against its declared meta-schema;
- asserts JSON Schema `format` values, including ISO 8601 dates;
- validates `.json` as one instance and `.jsonl` one record per line;
- rejects blank JSONL lines;
- reports the source line for JSONL parse and validation errors; and
- runs offline so validation never depends on a remote schema fetch.

## Open routing decisions

1. `docs/ARCHITECTURE.md` names `data/translation-pairs/`, but the implemented
   repository and `IMPLEMENTATION_PLAN.md` use `data/translation/`.
2. The architecture names `ai/correction-datasets/`, while the implemented
   scaffold currently has `ai/datasets/`.
3. Grammar-rule and style-example schemas need canonical hand-authored or
   generated record locations before file-to-schema routing can be enforced.
4. Sentence pairs used as standalone corpus examples may need a distinct
   schema profile from bilingual translation pairs.

These questions must be resolved before the corresponding payload schemas and
automatic directory routing are marked complete. They do not weaken the common
metadata contract.
