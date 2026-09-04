# Style Guide

Conventions for everything written in this repository — prose, data, and
configuration. Where this guide and [`ARCHITECTURE.md`](ARCHITECTURE.md)
appear to disagree, the architecture wins; report the discrepancy.

---

## General principles

- **Machine-first, human-readable.** Normative facts live in structured
  records; prose explains and contextualizes.
- **Consistency beats preference.** Follow the existing pattern in the file or
  directory you are editing, even if you would have chosen differently.
- **English for meta-documentation, Somali for content.** Project docs,
  schemas, and field names are in English; the language data itself is Somali
  (with English glosses where the schema calls for them).

## Markdown

- One `# H1` per document, matching the document's purpose. Structure below it
  with `##`/`###`; never skip heading levels.
- Wrap prose at ~80 characters. Do not wrap inside tables or code fences.
- Use fenced code blocks with a language tag (` ```json `, ` ```text `).
- Use reference-style or inline links with descriptive text — never bare URLs
  in prose, never "click here."
- Use tables for enumerable facts (examples, field lists); keep explanation in
  surrounding prose.
- Spec documents **must** carry YAML front-matter with the required fields
  (`id`, `title`, `status`, `since_version`, `category`, `supersedes`).
  Front-matter keys are `lower_snake_case`.
- Somali example text is written in normal orthography, not italics. Gloss
  tables use the three-column form: Somali | Gloss | English.

## JSON

- Used for schemas, manifests, and fixed configuration only — never for
  datasets.
- 2-space indentation, UTF-8, no BOM, LF line endings, trailing newline at end
  of file.
- Keys are `lower_snake_case` (e.g. `part_of_speech`, `review_status`).
- No comments (JSON has none); if a value needs explanation, the schema's
  `description` field is the place.
- Schemas use JSON Schema draft 2020-12 and are named `*.schema.json`.

## JSONL

- One complete JSON object per line; one record = one line = one diff hunk.
- No blank lines, no trailing commas, no pretty-printing inside a line.
- UTF-8 throughout. Somali text is stored as written — never escaped into
  `\uXXXX` sequences where a literal character works.
- Every record carries the common metadata block (`contributor`, `source`,
  `date_added`, `review_status`, `license`, `schema_version`) as defined by
  `schemas/metadata-common.schema.json`.
- Records are **append-only**: add new lines at the end of the file;
  deprecate rather than delete; never re-sort a file in the same PR that adds
  content.

## File naming

- `lower-kebab-case` for all files and directories: `machine-learning.jsonl`,
  `getting-started.md`.
- Extensions are meaningful and fixed: `.jsonl` for datasets, `.md` for prose,
  `.schema.json` for schemas, `.yml` for CI workflows.
- Domain glossary files are named after the domain in full English kebab-case
  — never abbreviated (`artificial-intelligence.jsonl`, not `ai.jsonl`).
- Spec files are numbered within reserved blocks:
  `spec/orthography/0001-alphabet.md`, `spec/grammar/0012-verb-system-....md`.
  Numbers are never reused.
- Root-level project documents (README, CONTRIBUTING, ...) use the
  conventional UPPERCASE names.

## IDs

- Format: `sls:<type>:<six-digit-sequence>`, with a stable category segment
  where its schema requires one — e.g. `sls:lex:000123`,
  `sls:term:ai:000045`, `sls:sent:004421`, `sls:pair:001987`,
  `sls:rule:grammar:000001`, `sls:style:000001`,
  `sls:bench:grammar:000012`, and `sls:corr:grammar:000001`.
- IDs are **permanent and append-only**: never renumbered, never reused, even
  when the entry is deprecated. External systems cite these IDs directly.
- Numbered standards use the separate `SLS-NNNN` scheme (four digits,
  zero-padded) defined in ARCHITECTURE.md §23.
- Language tags follow BCP 47: `so` for standard Somali, `en` for English;
  dialects use private-use extensions (`so-x-maay`) until formally namespaced.

## Dates, versions, and metadata

- Dates are ISO 8601 (`2026-07-08`), always absolute, never relative.
- Versions are SemVer (`1.2.0`). Remember the axes are decoupled: repository
  release, per-schema `schema_version`, per-spec `status`, and per-standard
  `version` are independent.
- Enum-like values (`review_status`, `coinage_type`, `status`) are
  `lower-kebab-case` or `lower_snake_case` exactly as the schema defines —
  never invent new values inline; propose them via the discussion track.

## Formatting and examples

- Every normative rule in a spec document needs at least one **positive** and
  one **negative** example.
- Examples in datasets must be real, natural Somali — invented sentences are
  acceptable only when clearly marked and reviewed by a native speaker.
- Keep example sentences short enough to diff and review comfortably (aim for
  under ~140 characters).
- When citing an entry, cite its `sls_id`, not its content — content can be
  revised; IDs cannot.

## Commit messages and PRs

Covered in [CONTRIBUTING.md](../CONTRIBUTING.md#commit-conventions):
Conventional Commits, imperative mood, ≤72-character summary, DCO sign-off on
every commit.
