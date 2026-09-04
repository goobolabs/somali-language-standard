# Phase 4 Record-Format Audit

- **Status:** Phase 4 implementation complete; first hosted run pending
- **Last updated:** 2026-09-04
- **Scope:** Milestone 3 / Phase 4 machine-readable schemas and validators

## Current repository state

The schema and dataset directories were placeholders when Phase 4 began. The
repository had no schema files, structured linguistic JSONL records, Rust
workspace, or executable validator. Existing JSON files under `standards/`
describe the standards registry and lifecycle metadata; they are not instances
of the planned linguistic record schemas.

Implementation began with the shared metadata contract and a
record-type-independent validator, then added the payload contracts listed
below. The linguistic data directories remain empty except for controlled
configuration; synthetic fixtures test structure without becoming evidence.

## Schema routing

| Schema | Planned records | Current routing state |
|---|---|---|
| `metadata-common.schema.json` | The `metadata` object in every linguistic JSONL record | Implemented as the shared Phase 4 foundation |
| `lexicon-entry.schema.json` | `data/lexicon/core/**/*.jsonl` and `data/lexicon/loanwords.jsonl` | Implemented; no data records yet |
| `terminology-entry.schema.json` | Domain files under `data/terminology/` | Implemented; no terminology data records yet |
| `example-sentence.schema.json` | `data/corpora/example-sentences.jsonl` | Implemented; no corpus sentence records yet |
| `sentence-pair.schema.json` | `data/translation/**/*.jsonl` | Implemented; no translation data records yet |
| `grammar-rule.schema.json` | `data/grammar/rules.jsonl` | Implemented; no grammar-rule records yet |
| `style-example.schema.json` | `data/style/**/*.jsonl` | Implemented; no style-example records yet |
| `benchmark-item.schema.json` | `benchmarks/**/*.jsonl` | Implemented; no benchmark records yet |
| `correction-pair.schema.json` | Correction records under `ai/datasets/` | Implemented; no correction records yet |

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

The validator loads every sibling `*.schema.json` into a local registry. A
payload schema can therefore reuse `metadata-common.schema.json` without a
network request. Duplicate schema IDs and invalid schema versions stop
validation before any data records are accepted.

The `check` command walks the repository's implemented record paths, assigns
each file to its schema, and then performs cross-record checks. It rejects
duplicate permanent IDs; unknown terminology domains; terminology IDs whose
code disagrees with their domain; unknown standard or rule references; and
missing grammar-spec paths. Diagnostics retain JSONL line numbers.

Routing deliberately excludes `data/lexicon/morphology/`,
`data/lexicon/frequency/`, terminology files prefixed with `_`, and AI datasets
whose filenames do not end in `correction-pairs.jsonl`. Those record families
do not yet have schemas and must not be misvalidated as another type.

### Lexicon contract

Lexicon entries use the permanent `sls:lex:NNNNNN` ID shape and the nine
canonical Somali word-class labels from SLS-0003 G10-R1. `Tifaftire` is not a
tenth primary value; entries in that subcategory use `tilmaame` at the primary
level. Dialect values use lowercase BCP 47 tags, with `so` for Standard Somali.
Regional profiles use an approved private-use form such as `so-x-maay`.

Every `magac` entry must record reviewed grammatical gender and a lexical
plural, as required by SLS-0003 G11-R1 and G11-R3. Plural forms remain lexical
evidence and are never generated by this schema. An entry explicitly marked as
a loanword must include its origin. Definitions require a numbered sense,
English definition, and Somali gloss. References to example sentences use
permanent `sls:sent:NNNNNN` identifiers. Those references resolve against the
example-sentence contract and are checked for dangling IDs by the repository
validator.

### Terminology contract

Terminology records distinguish the controlled domain slug from the shorter
code embedded in `sls:term:<code>:NNNNNN`. The 20 launch mappings come directly
from reserved standards SLS-0200 through SLS-0219. Existing architecture codes
`ai`, `ml`, and `cybersec` are retained; other domains use their full slug
rather than introducing new abbreviations without a prior repository decision.

Terms require English and Somali forms and definitions, an SLS-0003 primary
word class, the documented coinage type, and the terminology lifecycle status.
English and Somali examples are optional but must occur as a pair. Reviewers
are recorded only in shared metadata. Validator fixtures are explicitly marked
as synthetic structural records and are not terminology evidence.

### Sentence-pair contract

Bilingual translation records use permanent `sls:pair:NNNNNN` identifiers and
require non-empty English and Standard Somali text, a register label, an
explicit `literal` or `natural` translation type, and shared provenance. A
terminology domain is optional, because general and idiom pairs need not be
domain-specific; when present, it uses a slug from `_domains.json` and will be
checked by the cross-reference validator.

The register remains a lower-kebab-case label until the planned style
standards establish a controlled vocabulary. Validator fixtures are synthetic
structural records and make no claim about Somali translation quality.

### Grammar-rule and style-example contracts

`data/grammar/rules.jsonl` is the canonical machine-readable companion path;
the normative Markdown rule remains authoritative. Each record points to its
governing standard and `spec/grammar/` file, records every normative keyword,
and requires at least one positive and one negative explained example. This
preserves the existing rule-and-example compliance surface without pretending
that a JSON export supersedes the reviewed prose.

Style examples live under `data/style/`, grouped by register when records are
introduced. Each record carries a permanent ID, its governing SLS-04xx
standard, a preferred/avoid pair, and the rationale. Optional terminology
domains use `_domains.json` slugs and remain a cross-reference check.

### Benchmark and correction contracts

The benchmark schema is a strict common envelope for `benchmarks/**/*.jsonl`:
suite-bearing permanent ID, task, string input and expected output,
explanation, three-level difficulty, and provenance. Optional rule references
are preserved. Suite-specific structured fields are contained in `task_data`,
which later task schemas can narrow without admitting arbitrary top-level
properties.

Correction records under `ai/datasets/` require the incorrect and corrected
forms, an explanation, and at least one rule or standard reference. This keeps
training records traceable to the rule that justifies a correction rather than
encoding unexplained rewrites. All test fixtures for these four contracts are
synthetic structural data.

## Resolved routing decisions

1. Parallel translation data uses the implemented `data/translation/` path.
2. AI prompts use `ai/prompts/`; instruction, fine-tuning, and correction
   records use `ai/datasets/`. Generated RAG chunks remain separate under
   `ai/rag-knowledge/chunks/` when that build phase begins.
3. `sentence-pair.schema.json` governs bilingual records under
   `data/translation/`. Monolingual corpus examples need a separate future
   profile rather than optionalizing one bilingual contract into ambiguity.
4. Machine-readable grammar companions use `data/grammar/rules.jsonl`, while
   structured style examples use register-grouped files under `data/style/`.
   The corresponding normative documents in `spec/` remain authoritative.

No payload-routing question remains open. Automatic file-to-schema routing and
the first cross-record integrity checks are implemented by `sls-validate
check`; future record families will extend its route table with their schemas.

## CI enforcement

`.github/workflows/validate.yml` is active for pull requests and pushes to
`main`. It pins the repository's Rust 1.85.0 minimum, checks formatting, treats
all Clippy warnings as errors, runs the complete validator test suite, and runs
the repository-wide check. The tests contain both accepted and deliberately
malformed records, so the workflow exercises success and failure paths without
committing invented linguistic content.

The first hosted workflow result can only be observed after this branch is
pushed. Until then, the workflow is implementation-complete and locally
verified, but not yet confirmed on a GitHub runner.
