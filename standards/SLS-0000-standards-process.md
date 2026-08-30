---
sls_id: SLS-0000
title: SLS Standards Process Standard
version: 0.1.1
status: Proposed
category: meta
owner: language-council
reviewers: []
dependencies: []
implements:
  - standards/SLS-0000-standards-process.md
  - standards/TEMPLATE.md
  - standards/registry.json
  - standards/REGISTRY.md
publication_date: null
supersedes: null
superseded_by: null
revision_history:
  - version: 0.1.0
    date: 2026-07-10
    change: Initial draft — distilled from ARCHITECTURE.md Part II (§22–§32)
  - version: 0.1.1
    date: 2026-08-30
    change: "Editorial: added the standard's own normative file to `implements`; linked the public review log"
---

<!--
SLS-0000 is the self-describing meta-standard for the SLS standards catalog. It
is co-located in standards/ (rather than spec/) because it governs the framework
itself, analogous to IETF's own process being published as BCP 9 / RFC 2026. It
is normative for how every other SLS-XXXX standard is written, numbered, and
promoted. Its authoritative rationale lives in docs/ARCHITECTURE.md §22–§32; this
document is the standards-track surface of that design.
-->

## Abstract

This standard defines the process by which every SLS standard is identified,
structured, versioned, governed, and promoted through its lifecycle. It is
self-describing: SLS-0000 is itself an SLS standard and conforms to the process
it defines. It establishes the `SLS-NNNN` identifier space, the formal document
template, the seven-stage lifecycle, the dependency rules between standards, and
the compliance model that external adopters use to make precise conformance
claims.

## Purpose

Without a defined process, a numbered standards catalog drifts by accretion:
identifiers get reused, documents diverge in structure, and "stable" is claimed
on top of shifting foundations. SLS-0000 exists so that the SLS catalog has the
same shape of authority as Unicode's UAX series, W3C Recommendations, and IETF
RFCs — a versioned, dependency-checked, citable catalog. It is relied on by
standard authors (how to write one), maintainers (how to validate the registry),
the Language Council (how to ratify), and external adopters (how to cite and
claim compliance).

## Scope

**In scope:** the identifier scheme (§ Numbering), the formal template, the
lifecycle and who approves each transition, per-standard versioning, the
dependency model, the compliance model, and the registry integration that makes
these machine-checkable.

**Out of scope:** the *content* of any individual standard (that lives in the
standard itself); the governance roles themselves (defined in `GOVERNANCE.md`
and ARCHITECTURE.md §2, which this standard reuses rather than redefines); the
repository-level SemVer, `schema_version`, and ordinary spec `status` axes
(ARCHITECTURE.md §14) — this standard adds only the fourth, per-standard axis.

## Definitions

- **Standard** — a normative rule-set assigned a permanent `SLS-NNNN` identifier
  and wrapped in the formal template (§ Template).
- **Registry** — `standards/registry.json` (machine-readable source of truth)
  and its human mirror `standards/REGISTRY.md`.
- **Meta record** — `standards/meta/SLS-NNNN.json`, the per-standard governance
  metadata; it never duplicates normative prose.
- **`implements`** — the list of physical files whose content a standard governs.
- **Language Council**, **Domain Editor**, **Maintainer** — the governance roles
  defined in `GOVERNANCE.md`; reused here unchanged.

## Normative Requirements

### Identifiers

- **R1.** Every standard **MUST** have a permanent identifier of the form
  `SLS-NNNN` (four zero-padded digits).
- **R2.** Identifiers **MUST NOT** be reused or renumbered. A retired standard's
  number stays retired forever, marked `Archived`, never recycled.
- **R3.** Identifiers **MUST** be assigned from the reserved category blocks:

  | Block | Category |
  |---|---|
  | `SLS-0000` | Meta (the standards process) |
  | `SLS-0001`–`0099` | Foundation (alphabet, orthography, grammar, punctuation, …) |
  | `SLS-0100`–`0199` | Lexicon |
  | `SLS-0200`–`0299` | Terminology (one ID per domain) |
  | `SLS-0300`–`0399` | Translation |
  | `SLS-0400`–`0499` | Writing & Style |
  | `SLS-0500`–`0599` | AI & Computational |
  | `SLS-0600`–`0699` | Reserved — Speech & Phonology |
  | `SLS-0700`–`0799` | Reserved — Dialects & Regional Variation |
  | `SLS-0800`–`0899` | Reserved — Historical & Alternate Scripts |
  | `SLS-0900`–`0999` | Reserved — Compliance, Certification & Testing |
  | `SLS-1000`+ | Open allocation (Council-gated) |

- **R4.** Opening a new macro-category block (`SLS-1000`+) **MUST** be approved
  by the Language Council through this standard's own change process; it **MUST
  NOT** be done by unilateral PR.

### Document structure

- **R5.** A document assigned an `SLS-NNNN` identifier **MUST** use the formal
  template in `standards/TEMPLATE.md`, including all required front-matter fields
  and all required sections, of which **Compliance Requirements** is one.
- **R6.** Normative statements **MUST** use RFC 2119 keywords (MUST, MUST NOT,
  SHOULD, SHOULD NOT, MAY) and **SHOULD** be individually numbered so the
  Compliance Requirements checklist can reference them.
- **R7.** A standard's normative prose **MUST** live in the file(s) named by its
  `implements` array; the `standards/meta/SLS-NNNN.json` record **MUST NOT**
  duplicate that prose.

### Lifecycle

- **R8.** A standard **MUST** occupy exactly one of seven lifecycle stages:

  ```
  Draft → Proposed → Review → Candidate → Stable → Deprecated → Archived
  ```

- **R9.** Stage transitions **MUST** be approved by the authority named below and
  **MUST NOT** skip stages (except that a standard **MAY** be withdrawn from any
  pre-Stable stage back to Draft):

  | Transition | Approver | Gate |
  |---|---|---|
  | — → Draft | Any contributor + Domain Editor sponsor | Minimum template sections filled |
  | Draft → Proposed | Domain Editor | Opens ≥14-day public comment period |
  | Proposed → Review | Domain Editor | Comment period closed with no unresolved objection |
  | Review → Candidate | The two required reviewers | ≥1 linguist **and** ≥1 technical reviewer approve |
  | Candidate → Stable | Language Council (majority vote) | Soak period: ≥1 real artifact implements it for one release cycle |
  | Stable → Deprecated | Language Council | Named `superseded_by` or documented retirement rationale |
  | Deprecated → Archived | Maintainers (mechanical) | ≥2 MINOR repo releases since deprecation |

- **R10.** A standard **MUST NOT** become `Stable` while any standard in its
  `dependencies` is below `Stable`, unless the Language Council records an
  explicit waiver with rationale.

### Versioning

- **R11.** Every standard **MUST** carry an independent SemVer `version` field,
  distinct from the repository `VERSION`:
  - **MAJOR** — a MUST/MUST NOT requirement changes or is removed (breaking).
  - **MINOR** — a new Recommendation, example, or non-binding clarification.
  - **PATCH** — editorial fixes with no normative effect.

### Dependencies

- **R12.** The set of all `dependencies` edges across the registry **MUST** form
  a directed acyclic graph (DAG); cycles are invalid.

### Registry integrity

- **R13.** For every standard, the `status`, `version`, `title`, and `id` **MUST**
  agree across three places: `standards/registry.json`, the standard's
  `standards/meta/SLS-NNNN.json`, and the standard document's own front-matter.
- **R14.** Every path in a standard's `implements` array **MUST** resolve to an
  existing file.

### Compliance claims

- **R15.** Compliance **MUST** be claimed per standard, per version — never as a
  blanket "SLS-compliant." A claim **MUST** state one of four levels: Fully
  Compliant, Compliant, Partially Compliant, or Non-Compliant.
- **R16.** A Partially Compliant claim **MUST** enumerate which Compliance
  Requirements are and are not met; "partial" without a stated gap list is not a
  valid claim.

## Recommendations

- Authors SHOULD open a GitHub Discussion before drafting a standard in a new
  category, to confirm the identifier allocation with a Domain Editor.
- A standard SHOULD list, in `implements`, the narrowest set of files that fully
  contain its normative content — neither more nor fewer.
- Reviewers SHOULD record substantive review outcomes in the pull request so the
  lifecycle transition is auditable after the fact.

## Examples

**Valid identifier allocation:** `SLS-0001` (Foundation block) for the Alphabet
Standard; `SLS-0200` (first Terminology slot) for the Artificial Intelligence
terminology standard.

**Invalid:** reassigning `SLS-0001` to a different topic after the alphabet
standard is archived (violates R2); a Grammar standard reaching `Stable` while
its dependency `SLS-0002` is still in `Review` (violates R10).

## Edge Cases

- **A standard with no natural home in `spec/`** (e.g. this one, or a future
  AI-resource standard) is created as a new file in the directory closest to what
  it governs, following the extensibility pattern in ARCHITECTURE.md §5/§30. It
  still uses the formal template and appears in the registry like any other.
- **A document that is both a local spec note and a numbered standard** (e.g.
  `spec/orthography/0001-alphabet.md`) carries both the local `id` and the global
  `sls_id` in its front-matter; neither its filename nor location changes.
- **Reserved but unopened blocks** (`SLS-0600`+) have no entries until their
  workstream begins; the reservation itself is not a standard.

## Compliance Requirements

| # | Requirement | Traces to | Level |
|---|---|---|---|
| C1 | Standard has a unique, never-reused `SLS-NNNN` id in the correct block | R1, R2, R3 | MUST |
| C2 | Document uses the formal template with all required sections | R5 | MUST |
| C3 | Normative statements use numbered RFC 2119 keywords | R6 | MUST |
| C4 | Current lifecycle stage is valid and was reached via the approved gate | R8, R9 | MUST |
| C5 | Not `Stable` while any dependency is below `Stable` (absent a recorded waiver) | R10 | MUST |
| C6 | Carries an independent SemVer `version` | R11 | MUST |
| C7 | Dependencies introduce no cycle | R12 | MUST |
| C8 | `status`/`version`/`title`/`id` agree across registry, meta, and front-matter | R13 | MUST |
| C9 | All `implements` paths exist | R14 | MUST |

## References

- `docs/ARCHITECTURE.md` §22–§32 — the SLS Standards Framework (authoritative
  rationale for this standard).
- `GOVERNANCE.md` — roles and decision-making reused by this process.
- IETF BCP 9 / RFC 2026 — *The Internet Standards Process* (analogous
  self-describing process standard).
- W3C Process Document — Recommendation-track maturity stages.
- IETF RFC 2119 — normative keyword definitions.
- [`SLS-0000 review log`](../docs/standards/SLS-0000-review-log.md) — the public-comment record for this standard.

## Revision History

| Version | Date | Change |
|---|---|---|
| 0.1.0 | 2026-07-10 | Initial draft — distilled from ARCHITECTURE.md Part II (§22–§32) |
| 0.1.1 | 2026-08-30 | Editorial: added the standard's own normative file to `implements`; linked the public review log |
