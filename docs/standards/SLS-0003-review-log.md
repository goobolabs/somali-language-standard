# SLS-0003 Review Log

The public-comment record for **SLS-0003 Somali Grammar Standard**. Required by
[`SLS-0000`](../../standards/SLS-0000-standards-process.md) R9: a
`Proposed → Review` transition may be recorded only when the comment period has
closed with no unresolved objection, and this file is where that is evidenced.

- **Standard:** SLS-0003 Somali Grammar Standard
- **Status:** Proposed
- **Version:** 0.1.1
- **Comment period opened:** 2026-08-23 (published in [pull request #7](https://github.com/goobolabs/somali-language-standard/pull/7))
- **Earliest close (≥14 days):** 2026-09-06
- **Comment venue:** [issue #11](https://github.com/goobolabs/somali-language-standard/issues/11)
- **Normative document:** [`spec/grammar/0018-somali-grammar-standard.md`](../../spec/grammar/0018-somali-grammar-standard.md)

The comment period is open. It cannot close before 2026-09-06. Implementation Phase 3 additionally requires a linguist engaged as a recurring reviewer.

## How to comment

Comment on the venue issue above, or open a pull request against the normative
document. Every substantive comment is copied into the table below with a
disposition and a written resolution; nothing is closed silently. Comments
that arrive after the earliest close date are still recorded — the date is a
minimum, not a deadline.

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

## Transition record

| Transition | Date | Approver | Gate evidence |
| --- | --- | --- | --- |
| Draft → Proposed | 2026-08-23 | Founding maintainer, acting as interim Council | [pull request #7](https://github.com/goobolabs/somali-language-standard/pull/7) |
| Proposed → Review | *not recorded* | Domain Editor | Requires the comment period closed with no `open` comment, and reviewers assigned |
