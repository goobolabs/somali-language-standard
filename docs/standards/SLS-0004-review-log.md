# SLS-0004 Review Log

The public-comment record for **SLS-0004 Somali Punctuation Standard**. Required by
[`SLS-0000`](../../standards/SLS-0000-standards-process.md) R9: a
`Proposed → Review` transition may be recorded only when the comment period has
closed with no unresolved objection; R18 requires every transition to be recorded
with its date, approver, and gate evidence; and R17 requires every substantive
error report to be answered here in writing. This file is where all of that is
evidenced.

- **Standard:** SLS-0004 Somali Punctuation Standard
- **Status:** Proposed
- **Version:** 0.2.0
- **Comment period opened:** 2026-08-23 (published in [pull request #7](https://github.com/goobolabs/somali-language-standard/pull/7))
- **Earliest close (≥14 days):** 2026-09-06
- **Comment venue:** [issue #12](https://github.com/goobolabs/somali-language-standard/issues/12)
- **Normative document:** [`spec/orthography/0004-punctuation.md`](../../spec/orthography/0004-punctuation.md)

The comment period is open. It cannot close before 2026-09-06. No comment is `open`: the R16 conflict recorded as M-2 was resolved in 0.2.0, and comment on that resolution is invited in issue #12.

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
| M-1 | 2026-08-30 | maintainer audit | The Edge Cases long-dash bullet was ungrammatical: “Interruption use is attested, but exact Unicode character, and R14–R15 define U+2014…”. | `accepted-editorial` | Rewrote the sentence in 0.1.1 without changing its meaning. |
| M-2 | 2026-08-30 | maintainer audit | R16 offers `" "` / `' '` as one of three accepted quotation profiles, and its own final sentence states that straight U+0027 MUST NOT be classified as quotation punctuation until SLS-0001 glottal-stop normalization has resolved its lexical use. A conforming writer cannot use the straight single-quote half of the profile the same rule offers. The Edge Cases “Quotation profiles” bullet repeats the offer. | `accepted` | Resolved 2026-08-30 in 0.2.0. R16 now lists curly double/single, straight double (U+0022), and guillemets as the accepted profiles, and excludes straight U+0027 outright: SLS-0001 R7 reserves it as the glottal-stop input alias, and one character cannot be normalized as a letter and read as a delimiter in the same text. Because the straight profile has no second level, R16 requires a curly single or single-guillemet pair for a quotation nested inside a straight-double pair. The Edge Cases quotation-profile bullet, the R16 example row, and compliance row C13 were updated to match. MAJOR by SLS-0000 R11, taken on the minor position pre-1.0. |

## Transition record

| Transition | Date | Approver | Gate evidence |
| --- | --- | --- | --- |
| Draft → Proposed | 2026-08-23 | Founding maintainer, acting as interim Council | [pull request #7](https://github.com/goobolabs/somali-language-standard/pull/7) |
| Proposed → Review | *not recorded* | Maintainer | Requires the comment period closed with no `open` comment (SLS-0000 R9) |
