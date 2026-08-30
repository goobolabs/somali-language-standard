# Governance

This document defines who decides what in the Somali Language Standard (SLS)
project, and how. The full rationale is in
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) (§2, §22–§32), which remains the
source of truth; this document is the operational summary.

SLS uses a **maintainer-steward model**: the maintainers review and advance the
project's content, and correctness is defended after publication by an open
correction channel rather than before publication by appointed gatekeepers.
Anyone may report an error in any standard at any stage; every substantive
report is answered in writing, in public, in that standard's review log.

---

## Roles

### Maintainers

Own the repository and everything in it: CI, schemas, tooling, releases,
repository administration, **and** the linguistic content. A maintainer reviews
contributions, records the reasoning behind each decision, and approves
lifecycle transitions.

This is a deliberate simplification. The alternative — requiring an independent
linguist reviewer and an independent technical reviewer before anything can
advance — describes people the project does not have, and a rule that cannot be
followed is worse than a modest one that can.

### The correction channel

The counterweight to a single approver is that nothing is ever closed.

- Any person may report a suspected error in any standard at any lifecycle
  stage, `Stable` included, by opening an issue.
- Every substantive report is recorded in that standard's review log
  (`docs/standards/SLS-XXXX-review-log.md`) with a disposition — `accepted`,
  `accepted-editorial`, `deferred`, `declined`, or `open` — and a written
  resolution. None is closed silently.
- A confirmed error that invalidates a MUST-level requirement produces a
  corrected version, or returns the standard to `Review`.

A `Stable` SLS standard is therefore not a claim that experts have signed it
off. It is a claim that every rule traces to cited evidence, every objection
raised so far has a written answer, and any new objection will get one.

### Language Council and Domain Editors — optional

Neither body is currently constituted, and no gate in this project requires
one. They remain available: if the project later wants a Council (3–7 named,
term-limited members) or per-domain editors, they are established through the
ordinary change process, and the authority they take over is named at that
time.

## Decision making

| Decision | Who decides |
|---|---|
| Day-to-day merges (additive data, docs fixes) | Any Maintainer, after the review gate is met |
| Terminology promotion to `status: standard` | Maintainer, rationale recorded |
| Spec/standard ratification to `Stable` | Maintainer, after the gates in the table below |
| Schema changes, CI, release engineering | Maintainers |
| New terminology domain, new numbering block | Maintainer, rationale recorded |
| Governance changes (this document) | Maintainer, through the RFC process it describes |

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

| Transition | Approver | Gate |
|---|---|---|
| — → Draft | Any contributor | Minimum template sections filled |
| Draft → Proposed | Maintainer | Opens a ≥14-day public comment period |
| Proposed → Review | Maintainer | Comment period closed with no unresolved objection |
| Review → Candidate | Maintainer | Maintainer review recorded, every finding dispositioned |
| Candidate → Stable | Maintainer | Soak period served (a real artifact implements it), a second ≥14-day comment period closed cleanly, dependencies `Stable` or waived |
| Stable → Deprecated | Maintainer | Named `superseded_by` or a documented retirement rationale |
| Deprecated → Archived | Maintainer | ≥2 MINOR repository releases since deprecation (mechanical) |

Every transition is recorded in the standard's review log with its approver,
date, and the evidence satisfying the gate.

A standard cannot become `Stable` while any of its declared dependencies is
below `Stable`, unless the maintainers record an explicit waiver and its
rationale.

## Review process

Every content pull request requires **one maintainer review** — content
correctness and schema conformance — plus passing CI. Reviews happen in public,
on the pull request, and the reasoning is recorded rather than left implicit.

### Reporting an error

This is the part that matters most to anyone outside the project, and it is
open to everyone:

1. Open an issue naming the standard and the requirement (for example
   "SLS-0003 G14-R4"), or comment on that standard's public-comment issue.
2. Say what is wrong. The most valuable report is not "this rule is incorrect"
   but "this rule is right about its own example and wrong in general" — a rule
   stated too broadly turns correct Somali into a reported error in every tool
   that adopts SLS.
3. Name the evidence where you can: attested usage, a dialect, a published
   grammar, a corpus. A rule changes on evidence, not on assertion — including
   the maintainers' own.

Your report is copied into the standard's review log with a disposition and a
written resolution. If it is declined, the reason is recorded too. Reports are
welcome against `Stable` standards; no stage is beyond correction.

## Conflict resolution

1. **Discuss first.** Most disagreements are resolved in the issue/PR thread
   with evidence: attested usage, prior scholarship, corpus data.
2. **Escalate to the maintainers**, who decide and record the rationale in the
   relevant issue and in the standard's review log.
3. **A maintainer who is party to the dispute says so.** Where the maintainers
   are themselves the source of the disputed rule, the disagreement and its
   resolution are recorded verbatim, so a later reader can weigh both sides
   rather than only the outcome.
4. **Genuine variation is documented, not suppressed.** Where real dialectal
   or stylistic variation exists, SLS records alternatives (e.g.
   `so_term_alternatives`) rather than forcing false consensus.
5. Code-of-conduct violations are handled separately under
   [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Future governance evolution

Governance is expected to evolve as the project matures:

- **Now:** the maintainers review and advance content; the correction channel
  is the external check. No standard's progress depends on recruiting anyone.
- **If contributors accumulate:** trusted reviewers can be named per domain, and
  a Language Council can be constituted, through the ordinary change process.
  Neither is a prerequisite for anything today, and neither is promised on a
  schedule.
- **Post v1.0:** partnerships with universities and language bodies may add
  institutional seats; a foundation or fiscal host may be adopted for
  sustainability (ARCHITECTURE.md §20, Phase 4).
- Changes to this document follow the same RFC process they describe.
