# SLS-0000 Review Log

The public-comment record for **SLS-0000 SLS Standards Process Standard**. Required by
[`SLS-0000`](../../standards/SLS-0000-standards-process.md) R9: a
`Proposed → Review` transition may be recorded only when the comment period has
closed with no unresolved objection; R18 requires every transition to be recorded
with its date, approver, and gate evidence; and R17 requires every substantive
error report to be answered here in writing. This file is where all of that is
evidenced.

- **Standard:** SLS-0000 SLS Standards Process Standard
- **Status:** Review
- **Version:** 0.2.0
- **Comment period opened:** 2026-07-10 (published in [pull request #1](https://github.com/goobolabs/somali-language-standard/pull/1))
- **Earliest close (≥14 days):** 2026-07-24
- **Comment venue:** [issue #8](https://github.com/goobolabs/somali-language-standard/issues/8)
- **Normative document:** [`standards/SLS-0000-standards-process.md`](../../standards/SLS-0000-standards-process.md)

The 14-day minimum elapsed on 2026-07-24 with no public comment recorded, satisfying the `Proposed → Review` gate in SLS-0000 R9. The transition was approved on 2026-08-30 and is recorded below. Comment remains open: under SLS-0000 R17 an error report is accepted at any stage, and the next gate (`Review → Candidate`) needs a recorded maintainer review with every finding dispositioned.

## How to comment

Comment on the venue issue above, or open a pull request against the normative
document. Every substantive comment is copied into the table below with a
disposition and a written resolution; nothing is closed silently. Comments
that arrive after the earliest close date are still recorded — the date is a
minimum, not a deadline — and under SLS-0000 R17 an error report is accepted at
any lifecycle stage, `Stable` included.

## Dispositions

| Disposition | Meaning |
| --- | --- |
| `accepted` | The change was made in the standard. |
| `accepted-editorial` | Applied as a PATCH-level fix with no normative effect. |
| `deferred` | Valid, but assigned to a later standard or version; the target is named. |
| `declined` | Not adopted; the recorded reason states why. |
| `open` | Under discussion; blocks the `Proposed → Review` transition. |

## Comments

| ID | Date | Source | Comment | Disposition | Resolution |
| --- | --- | --- | --- | --- | --- |
| M-1 | 2026-08-30 | maintainer audit | The front-matter `implements` array omitted `standards/SLS-0000-standards-process.md`, although `registry.json` and `standards/meta/SLS-0000.json` both list it. R7 requires a standard's normative prose to live in a file named by its own `implements` array, so the document did not satisfy its own requirement. | `accepted-editorial` | Added the file to the front-matter `implements` array in 0.1.1. No normative text changed. |
| M-2 | 2026-08-30 | maintainer decision | The process required an independent linguist reviewer and an independent technical reviewer before any standard could pass `Review → Candidate`, and a Language Council vote before `Stable`. Neither exists, and none can be recruited on the project's timeline, so every finished standard was blocked on people rather than on content. A rule that cannot be followed is worse than a modest one that can. | `accepted` | Governance simplified in 0.2.0: a maintainer approves every lifecycle transition; the two-reviewer gate is removed; new R17 makes the correction channel normative (anyone may report an error at any stage, `Stable` included, and every substantive report gets a recorded disposition and written resolution); new R18 requires every transition to record date, approver, and gate evidence; a second ≥14-day comment period is added before `Stable`. Mirrored in `GOVERNANCE.md`, `CONTRIBUTING.md`, and `docs/ARCHITECTURE.md` §2, §16, §25, §26. The trade-off is stated openly in `docs/REVIEWERS.md`: a `Stable` SLS standard is a self-certification backed by cited evidence and an answerable public record, not by independent sign-off. |

## Transition record

| Transition | Date | Approver | Gate evidence |
| --- | --- | --- | --- |
| Draft → Proposed | 2026-07-10 | Founding maintainer, acting as interim Council | [pull request #1](https://github.com/goobolabs/somali-language-standard/pull/1) |
| Proposed → Review | 2026-08-30 | Maintainer (sharafdin) | Comment period opened 2026-07-10 in [pull request #1](https://github.com/goobolabs/somali-language-standard/pull/1), 14-day minimum elapsed 2026-07-24; no comment received and no `open` item in this log |
| Review → Candidate | *not recorded* | Maintainer | Requires a recorded maintainer review with every finding dispositioned (SLS-0000 R9) |
