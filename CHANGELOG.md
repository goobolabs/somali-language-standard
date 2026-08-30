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
- One public-comment tracking issue per `Proposed` standard (#8–#13), giving
  each comment period a venue; previously the periods ran with nowhere to file
  a comment that would be counted.
- A review log per standard (`docs/standards/SLS-000X-review-log.md`) recording
  every substantive comment with a disposition and written resolution, plus
  `docs/standards/REVIEW-LOG-TEMPLATE.md` for future standards.
- [`docs/REVIEWERS.md`](docs/REVIEWERS.md) — what the native-speaker/linguist
  and technical reviewer roles involve, what a *recurring* reviewer commits to,
  how a review is recorded, and the current open positions.
- [`docs/standards/SLS-0003-reviewer-packet.md`](docs/standards/SLS-0003-reviewer-packet.md)
  — all 42 grammar rules with their conforming and non-conforming examples in
  one page, plus twelve priority questions that need a native-speaker
  judgment. Reviewing SLS-0003 no longer requires reading the repository.
- A lifecycle banner on grammar topic files `spec/grammar/0010`–`0017` stating
  that they keep the local spec-note status `Draft` while their governing
  standard SLS-0003 is `Proposed` and under comment.
- Completed eight-collection resources evidence baseline: 145 files audited,
  cleaned, maintainer-approved, and documented with 501 approved provenance
  records.

### Changed

- **Governance simplified to a maintainer-steward model.** A maintainer now
  approves every lifecycle transition, from `Draft` through `Stable`. The
  correction channel replaces pre-publication gatekeeping: anyone may report an
  error in any standard at any stage, `Stable` included, and every substantive
  report gets a recorded disposition and a written resolution. `SLS-0000` gains
  R17 (correction channel) and R18 (transition record) with their compliance
  rows, and a second ≥14-day comment period is required before `Stable`.
  Mirrored in `GOVERNANCE.md`, `CONTRIBUTING.md`, `docs/ARCHITECTURE.md`
  (§2, §16, §25, §26), `docs/REVIEWERS.md`, the implementation plan, and the
  roadmap. Rationale and trade-off recorded in
  [`docs/standards/SLS-0000-review-log.md`](docs/standards/SLS-0000-review-log.md)
  as M-2.
- Bumped **SLS-0000** to `0.2.0` — MAJOR by its own R11 (MUST-level requirements
  changed), taken on the minor position because the standard is pre-1.0.
- `owner` changed from `language-council` to `maintainers` across the registry,
  the per-standard meta records, and every standard's front matter.
- Promoted SLS-0000 and SLS-0001 from `Draft` to `Proposed`, opening their
  public comment period.
- Completed the SLS-0002 through SLS-0005 proposal package. Their ≥14-day
  public-comment clocks begin when the proposal branch is published.
- Marked the resources prerequisite complete for normative drafting.
- Bumped SLS-0003 to `0.1.2` (PATCH — editorial cross-references only).
- Bumped SLS-0000 through SLS-0005 to `0.1.1` (PATCH — editorial only; no
  requirement, example, or evidence mapping changed, and no standard advanced
  a lifecycle stage).

### Fixed

- **SLS-0000** — added the standard's own normative file to its front-matter
  `implements` array; without it the document did not satisfy its own R7.
- **SLS-0001** — corrected the Scope cross-reference for digraph case pairing
  from R11 to R12, and rewrote the glottal stop in the Definitions and Edge
  Cases prose as canonical U+02BC, which R6 requires and R7 makes mandatory
  for released records.
- **SLS-0004** — repaired an ungrammatical sentence in the long-dash edge
  case.
- **SLS-0003** — cross-referenced the two `ha` particles. G15-R4 governs
  prohibitive `ha` with a negative verb form; G12-R4 separately licenses the
  third-person directive `Isagu ha qoro!` with an affirmative form. Neither
  file mentioned the other, so an implementation reading G15-R4 alone would
  report the licensed directive as an error.

### Removed

- The two-reviewer gate — one independent native-speaker/linguist reviewer plus
  one independent technical reviewer — previously required before
  `Review → Candidate`, and the Language Council vote previously required before
  `Stable`. Neither body existed, so finished standards were blocked on
  recruitment rather than on content. The Language Council and Domain Editor
  roles remain documented as optional bodies that may be constituted later
  through the ordinary change process; no gate depends on them.

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
