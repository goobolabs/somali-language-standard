# SLS-0001 Review Log

The public-comment record for **SLS-0001 Somali Alphabet Standard**. Required by
[`SLS-0000`](../../standards/SLS-0000-standards-process.md) R9: a
`Proposed → Review` transition may be recorded only when the comment period has
closed with no unresolved objection; R18 requires every transition to be recorded
with its date, approver, and gate evidence; and R17 requires every substantive
error report to be answered here in writing. This file is where all of that is
evidenced.

- **Standard:** SLS-0001 Somali Alphabet Standard
- **Status:** Review
- **Version:** 0.1.1
- **Comment period opened:** 2026-07-10 (published in [pull request #1](https://github.com/goobolabs/somali-language-standard/pull/1))
- **Earliest close (≥14 days):** 2026-07-24
- **Comment venue:** [issue #9](https://github.com/goobolabs/somali-language-standard/issues/9)
- **Normative document:** [`spec/orthography/0001-alphabet.md`](../../spec/orthography/0001-alphabet.md)

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
| M-1 | 2026-08-30 | maintainer audit | The Scope section attributed digraph case pairing to R11. R11 governs excluded letters; case pairing is R12. | `accepted-editorial` | Corrected the cross-reference to R12 in 0.1.1. |
| M-2 | 2026-08-30 | maintainer audit | The Definitions entry for the glottal stop and the Edge Cases bullets wrote the letter as U+0027 (`'af`, `su'aal`), presenting it as the canonical form. R6 requires U+02BC and R7 forbids U+0027 in released records, and the Examples table already marks the U+0027 spelling as non-conforming. | `accepted-editorial` | Rewrote those occurrences with canonical U+02BC (`ʼaf`, `suʼaal`) in 0.1.1. U+0027 is retained only where a line demonstrates the non-canonical alias. |

## Transition record

| Transition | Date | Approver | Gate evidence |
| --- | --- | --- | --- |
| Draft → Proposed | 2026-07-10 | Founding maintainer, acting as interim Council | [pull request #1](https://github.com/goobolabs/somali-language-standard/pull/1) |
| Proposed → Review | 2026-08-30 | Maintainer (sharafdin) | Comment period opened 2026-07-10 in [pull request #1](https://github.com/goobolabs/somali-language-standard/pull/1), 14-day minimum elapsed 2026-07-24; no comment received and no `open` item in this log |
| Review → Candidate | *not recorded* | Maintainer | Requires a recorded maintainer review with every finding dispositioned (SLS-0000 R9) |
