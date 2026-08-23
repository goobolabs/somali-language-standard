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
- **SLS-0000** SLS Standards Process Standard → `Proposed` — the self-describing
  meta-standard distilled from ARCHITECTURE.md Part II.
- **SLS-0001** Somali Alphabet Standard → `Proposed` — letter inventory (21
  consonants + 5 vowels), canonical collation order, digraph rules, glottal-stop
  representation, vowel-length notation, excluded letters, and Unicode code
  points (`spec/orthography/0001-alphabet.md`).
- **SLS-0002** Somali Orthography Standard → `Proposed` — evidence-mapped
  candidate rules for word boundaries, bound conjunction clitics, gemination,
  morphophonemic spelling, compounds, unmarked tone, and adapted loanwords
  (`spec/orthography/0002-spelling-rules.md`).
- **SLS-0003** Somali Grammar Standard → `Proposed` — eight topic documents
  covering word classes, noun and verb morphology, pronouns, clause structure,
  negation, questions, and bounded error diagnostics, plus a formal lifecycle
  and compliance wrapper (`spec/grammar/0010`–`0018`).
- **SLS-0004** Somali Punctuation Standard → `Proposed` — evidence-mapped
  candidate rules for sentence endings, syntax-sensitive commas, colons,
  quotations, parentheses, and the hyphen/dash boundary
  (`spec/orthography/0004-punctuation.md`).
- **SLS-0005** Somali Capitalization Standard → `Proposed` — conservative
  candidate rules for sentence beginnings, proper names, calendar names,
  *Soomaali*, common seasons and directions, and digraph case, with the
  supplementary-source limitation retained
  (`spec/orthography/0003-capitalization.md`).
- `spec/0000-index.md` — the specification index.
- Four evidence maps and the Milestone 2 lifecycle-readiness record under
  `docs/standards/`.
- `VERSION` set to the Milestone 2 target repository version `0.2.0`.
- Completed eight-collection resources evidence baseline: 145 files audited,
  cleaned, maintainer-approved, and documented with 501 approved provenance
  records.

### Changed

- Promoted SLS-0000 and SLS-0001 from `Draft` to `Proposed`, opening their
  public comment period.
- Completed the SLS-0002 through SLS-0005 proposal package. Their ≥14-day
  public-comment clocks begin when the proposal branch is published.
- Marked the resources prerequisite complete for normative drafting.

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
