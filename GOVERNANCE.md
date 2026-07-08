# Governance

This document defines who decides what in the Somali Language Standard (SLS)
project, and how. The full rationale is in
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) (§2, §22–§32), which remains the
source of truth; this document is the operational summary.

SLS uses a lightweight **council model** — similar to CPython's PEP editors or
W3C working groups: enough authority to resolve disputes, not so much that it
gates contribution.

---

## Roles

### Maintainers

Own the technical infrastructure: CI, schemas, tooling, releases, repository
administration. Maintainers hold **technical** authority, not linguistic
authority — they decide *how* the standard is built and validated, not *what*
the Somali language is.

### Language Council

The final authority on orthography, grammar, and disputed terminology. The
Council:

- is small (3–7 members), named in this file once constituted, and
  term-limited;
- ratifies spec documents and standards as `Stable` by majority vote;
- approves new terminology domains and new standard-number categories;
- resolves disputes that Domain Editors cannot.

> **Current status:** During Phase 0, the founding maintainer acts as interim
> Council. The Council will be formally constituted before any standard is
> ratified `Stable` (see ROADMAP.md).

### Domain Editors

One per terminology domain (Artificial Intelligence, Medicine, Law, ...).
Responsible for reviewing that domain's glossary contributions and sponsoring
that domain's proposals. A domain may be served by a single trusted
contributor initially; **domains without an editor default to Council
review**.

## Decision making

| Decision | Who decides |
|---|---|
| Day-to-day merges (additive data, docs fixes) | Any Maintainer, after the review gate is met |
| Terminology promotion to `status: standard` | Domain Editor (or Council if no editor) |
| Spec/standard ratification to `Stable` | Language Council, majority vote |
| Schema changes, CI, release engineering | Maintainers |
| New terminology domain, new numbering block | Language Council |
| Governance changes (this document) | Language Council |

The default decision mode is **lazy consensus**: proposals that receive no
objection within their comment period proceed. Votes are only taken where this
document requires them or consensus fails.

## The RFC process

Every non-trivial normative change follows an RFC-style lifecycle. There are
two tiers:

**Ordinary spec documents** (`spec/*.md`) use the four-stage lifecycle:

```text
Draft → Review → Stable → Deprecated
```

**Numbered standards** (`SLS-XXXX`, the formal standards catalog) use the
stricter seven-stage lifecycle:

```text
Draft → Proposed → Review → Candidate → Stable → Deprecated → Archived
```

Key gates (full definitions in ARCHITECTURE.md §25–§26):

| Transition | Approver |
|---|---|
| — → Draft | Any contributor, with a Domain Editor sponsor |
| Draft → Proposed | Domain Editor accepts for formal comment (14-day minimum comment period) |
| Proposed → Review | Domain Editor, once the comment period closes cleanly |
| Review → Candidate | The two required reviewers (linguist + technical) |
| Candidate → Stable | Language Council majority vote, after a soak period in which at least one real artifact implements the standard |
| Stable → Deprecated | Language Council only; requires a named successor or documented rationale |
| Deprecated → Archived | Maintainers (mechanical, date-triggered) |

A standard cannot become `Stable` while any of its declared dependencies is
below `Stable`, unless the Council records an explicit waiver.

## Review process

Every content pull request requires:

1. **One native-speaker / linguist reviewer** — is the content correct Somali?
2. **One technical reviewer** — does it conform to schemas, IDs, and formats?

Plus Domain Editor sign-off for terminology promotions, and Council approval
for lifecycle transitions as listed above. Reviews happen in public, on the
pull request.

## Conflict resolution

1. **Discuss first.** Most disagreements are resolved in the issue/PR thread
   with evidence: attested usage, prior scholarship, corpus data.
2. **Escalate to the Domain Editor** for domain-scoped disputes.
3. **Escalate to the Language Council** for anything cross-domain, or where a
   Domain Editor is a party to the dispute. The Council decides by majority
   vote and records the rationale in the relevant issue.
4. **Genuine variation is documented, not suppressed.** Where real dialectal
   or stylistic variation exists, SLS records alternatives (e.g.
   `so_term_alternatives`) rather than forcing false consensus.
5. Code-of-conduct violations are handled separately under
   [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Future governance evolution

Governance is expected to evolve as the project matures:

- **Phase 0–1 (now):** founding maintainer acts as interim Council; early
  trusted contributors are recruited as Domain Editors.
- **Before v1.0:** the Language Council is formally constituted (3–7 named,
  term-limited members) — a prerequisite for ratifying any standard `Stable`.
- **Post v1.0:** partnerships with universities and language bodies may add
  institutional seats; a foundation or fiscal host may be adopted for
  sustainability (ARCHITECTURE.md §20, Phase 4).
- Changes to this document follow the same RFC process they describe and
  require Council approval.
