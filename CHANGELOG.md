# Changelog

All notable changes to the Somali Language Standard are documented in this
file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) §14 for how repository
releases, schema versions, and per-standard versions relate.

## [Unreleased]

### Added

- Standards framework (`standards/`) per ARCHITECTURE.md §30: `TEMPLATE.md`
  (the §24 formal template), `registry.json` (machine-readable source of truth,
  seeded with the full 53-standard launch set), `REGISTRY.md` (human mirror),
  and `meta/` governance records.
- **SLS-0000** SLS Standards Process Standard → `Draft` — the self-describing
  meta-standard distilled from ARCHITECTURE.md Part II.
- **SLS-0001** Somali Alphabet Standard → `Draft` — letter inventory (21
  consonants + 5 vowels), canonical collation order, digraph rules, glottal-stop
  representation, vowel-length notation, excluded letters, and Unicode code
  points (`spec/orthography/0001-alphabet.md`).
- `spec/0000-index.md` — the specification index.

## [0.1.0] - 2026-07-08

### Added

- Initial repository foundation:
  - Directory structure for `spec/`, `schemas/`, `data/` (lexicon,
    terminology, translation, corpora), `ai/` (prompts, datasets),
    `benchmarks/`, `tools/`, `examples/`, and `docs/`.
  - Core project documents: `README.md`, `CONTRIBUTING.md`,
    `CODE_OF_CONDUCT.md`, `GOVERNANCE.md`, `SECURITY.md`.
  - Dual licensing: `LICENSE-CODE` (MIT, full text) for code and schemas,
    `LICENSE-DATA` (CC BY 4.0, full legal code) for all linguistic content.
  - Documentation: `docs/GETTING_STARTED.md`, `docs/STYLE_GUIDE.md`,
    `docs/FAQ.md` (alongside the pre-existing `docs/ARCHITECTURE.md`).
  - Planning documents: `ROADMAP.md`, `IMPLEMENTATION_PLAN.md`.
  - GitHub templates: issue templates (bug report, feature request,
    terminology proposal, standard proposal) and a pull request template.
  - Placeholder CI workflows: `validate.yml`, `docs.yml`, `release.yml`
    (TODO-only; no CI logic implemented yet).

[Unreleased]: https://github.com/goobolabs/somali-language-standard/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/goobolabs/somali-language-standard/releases/tag/v0.1.0
