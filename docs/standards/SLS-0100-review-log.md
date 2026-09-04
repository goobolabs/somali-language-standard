# SLS-0100 Review Log

The decision and correction record for **SLS-0100 Dictionary Standard**.
[`SLS-0000`](../../standards/SLS-0000-standards-process.md) R17 requires every
substantive error report to receive a written disposition at every lifecycle
stage, and R18 requires every lifecycle transition to record its date,
approver, and gate evidence.

- **Standard:** SLS-0100 Dictionary Standard
- **Status:** Draft
- **Version:** 0.1.0
- **Draft opened:** 2026-09-04
- **Formal comment period:** not opened
- **Draft venue:** [Milestone 4 issue #24](https://github.com/goobolabs/somali-language-standard/issues/24)
- **Normative document:** [`spec/lexicon/0100-dictionary-standard.md`](../../spec/lexicon/0100-dictionary-standard.md)
- **Evidence map:** [`SLS-0100-evidence-map.md`](SLS-0100-evidence-map.md)
- **Pilot packet:** [`SLS-0100-reviewer-packet.md`](SLS-0100-reviewer-packet.md)

This log does not claim that public comment has begun. Issue #24 is the
structural drafting venue. If SLS-0100 later moves to `Proposed`, this file will
record the publication date, dedicated comment venue, and earliest permitted
close date.

## How to report a finding

Comment on issue #24 while the standard is in Draft, or open a pull request
against the normative document. Every substantive finding is copied into the
table below and resolved in writing. A schema-valid pilot entry is not treated
as linguistically reviewed unless its entry-level decision is recorded.

## Dispositions

| Disposition | Meaning |
|---|---|
| `accepted` | The change was made in the standard or pilot policy. |
| `accepted-editorial` | Applied as a PATCH-level correction with no normative effect. |
| `deferred` | Valid, but assigned to a named later standard, version, or review batch. |
| `declined` | Not adopted; the recorded reason explains why. |
| `open` | Not resolved; blocks the affected transition or data approval. |

## Findings

| ID | Date | Source | Finding | Disposition | Resolution |
|---|---|---|---|---|---|
| D-1 | 2026-09-04 | maintainer audit — source/schema comparison | Requiring a non-empty plural string for every noun would force an invented form for a reviewed mass or non-plural reading. | accepted | Expand the schema to allow JSON `null` only for a reviewed absence of an ordinary plural; missing review is not encoded as `null`. |
| D-2 | 2026-09-04 | maintainer audit — source/schema comparison | A required Boolean loanword flag forces `false` where the principal source gives no etymology. | accepted | Expand `is_loanword` to allow JSON `null` for unresolved status; `null` is not interpreted as non-loanword. |
| D-3 | 2026-09-04 | maintainer audit — source-rights comparison | The dictionary's compiler, edition, and republication rights remain incomplete, while SLS data is licensed CC BY 4.0. | accepted | Use the source for factual checking and write new bilingual glosses; do not copy source definitions into SLS data without compatible rights and explicit provenance. |
| D-4 | 2026-09-04 | maintainer audit — category comparison | Dictionary grammatical codes do not all map one-to-one onto the nine SLS-0003 primary word classes. | accepted | Publish only reviewed mappings; ambiguous codes such as general `qr` stay out of automated conversion. |

## Entry-review record

No pilot entry has been approved or assigned an `sls:lex:` identifier. Answers
to the pilot packet will be recorded here verbatim before corresponding data is
created.

| Entry decision | Date | Source | Maintainer answer | Disposition | Data effect |
|---|---|---|---|---|---|
| — | — | — | *(none recorded yet)* | — | — |

## Transition record

| Transition | Date | Approver | Gate evidence |
|---|---|---|---|
| planned → Draft | 2026-09-04 | Maintainer | [Issue #24](https://github.com/goobolabs/somali-language-standard/issues/24); formal template completed; [evidence map](SLS-0100-evidence-map.md) and pilot boundaries recorded |
| Draft → Proposed | *not recorded* | Maintainer | Requires resolved Draft findings and an explicitly opened ≥14-day public-comment period |
