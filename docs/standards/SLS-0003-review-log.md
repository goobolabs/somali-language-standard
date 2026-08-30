# SLS-0003 Review Log

The public-comment record for **SLS-0003 Somali Grammar Standard**. Required by
[`SLS-0000`](../../standards/SLS-0000-standards-process.md) R9: a
`Proposed → Review` transition may be recorded only when the comment period has
closed with no unresolved objection; R18 requires every transition to be recorded
with its date, approver, and gate evidence; and R17 requires every substantive
error report to be answered here in writing. This file is where all of that is
evidenced.

- **Standard:** SLS-0003 Somali Grammar Standard
- **Status:** Proposed
- **Version:** 0.1.3
- **Comment period opened:** 2026-08-23 (published in [pull request #7](https://github.com/goobolabs/somali-language-standard/pull/7))
- **Earliest close (≥14 days):** 2026-09-06
- **Comment venue:** [issue #11](https://github.com/goobolabs/somali-language-standard/issues/11)
- **Normative document:** [`spec/grammar/0018-somali-grammar-standard.md`](../../spec/grammar/0018-somali-grammar-standard.md)

The comment period is open. It cannot close before 2026-09-06. One comment is `open`. The reviewer-facing summary of this
standard is [`SLS-0003-reviewer-packet.md`](SLS-0003-reviewer-packet.md); the
role is described in [`docs/REVIEWERS.md`](../REVIEWERS.md).

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
| M-1 | 2026-08-30 | maintainer audit | The standard states that its implemented topic specifications are accepted for formal public comment, but topic files 0010–0017 each carry the local spec-note status `Draft`. A reader arriving directly at a topic file could not tell it was under comment. The split is deliberate and documented in `spec/0000-index.md`, but it was not visible in the topic files themselves. | `accepted-editorial` | Added a lifecycle banner to each of the eight topic files in 0.1.1, naming SLS-0003 as the governing standard and pointing to this log. The `status` fields are unchanged. |
| M-2 | 2026-08-30 | maintainer audit | Two different `ha` particles are standardized in different files with no cross-reference: G15-R4 requires prohibitive `ha` with a negative verb form (`*Waxba ha keen.` is non-conforming), while G12-R4 licenses the third-person directive `Isagu ha qoro!`, where `ha` precedes an affirmative form. An implementation reading G15-R4 alone would report the licensed directive as an error. | `accepted-editorial` | Added a cross-reference in both topic files' Edge Cases in 0.1.2, stating that the two constructions are distinguished by the following verb form rather than by the particle. No requirement changed. Whether G15-R4's wording is safe as written is put to the reviewer as packet question Q1. |
| M-3 | 2026-08-30 | maintainer audit | The G13-R2 example rows for inclusive `innaga` and exclusive `annaga` are metalinguistic descriptions (`annaga — dhegeystaha kuma jiro`) rather than Somali sentence pairs, so the rule has no example a validator or a reader can use to see the contrast in a real clause. | `open` | Needs a native-speaker minimal pair; the maintainers will not invent one. Requested as packet question Q3 and posted in issue #11. Blocks the `Proposed → Review` transition until answered or the rule is narrowed. |

## Transition record

| Transition | Date | Approver | Gate evidence |
| --- | --- | --- | --- |
| Draft → Proposed | 2026-08-23 | Founding maintainer, acting as interim Council | [pull request #7](https://github.com/goobolabs/somali-language-standard/pull/7) |
| Proposed → Review | *not recorded* | Maintainer | Requires the comment period closed with no `open` comment (SLS-0000 R9) |
