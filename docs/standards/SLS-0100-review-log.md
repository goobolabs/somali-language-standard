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

Batch 1 is complete. The four answers below are copied verbatim from the
maintainer's native-speaker review. Batch 2 and Batch 3 remain unanswered.

| Entry decision | Date | Source | Maintainer answer | Disposition | Data effect |
|---|---|---|---|---|---|
| MR-1 | 2026-09-04 | maintainer review (native speaker) — packet LQ1 | Gender and Plural: Both are correct. It is masculine (*baabuurka*) and its plural is *baabuurro*.<br><br>Somali Gloss: The gloss "Gaadiid matoor ku socda oo rakaab ama xamuul qaada" is clear and accurate.<br><br>Loanword Status: Yes, it is a known loanword. It comes from Arabic (*bābūr*), which originally comes from Italian (*vapore*). | accepted | Add reviewed entry `sls:lex:000001` with `is_loanword: true` and the recorded Arabic/Italian origin. |
| MR-2 | 2026-09-04 | maintainer review (native speaker) — packet LQ2 | Senses: The two senses are correct and distinct. Sense 1 refers to an adult female human (*qof dumar ah*). Sense 2 refers to a wife or female spouse (*xaas*).<br><br>Plural and Loanword Status: Yes, *naago* is the correct lexical plural. It is safe to mark *naag* as a native Somali word and not a loanword. | accepted | Add reviewed entry `sls:lex:000002` with two senses, plural `naago`, and `is_loanword: false`. |
| MR-3 | 2026-09-04 | maintainer review (native speaker) — packet LQ3 | Commendatory Sense: Yes, you should include the commendatory sense as a third sense. In Somali lexicography and daily speech, *nin* is often used to describe a brave, honorable, or capable man (*nin rag ah*).<br><br>Confirmations:<br>Plural: *niman* (Correct).<br>Gender: Masculine singular (*ninka*) (Correct).<br>Loanword Status: Native Somali word (not a loanword) (Correct). | accepted | Add reviewed entry `sls:lex:000003` with three senses, plural `niman`, and `is_loanword: false`. |
| MR-4 | 2026-09-04 | maintainer review (native speaker) — packet LQ4 | Somali Gloss: The gloss "Qalab gacan-qabsi iyo af wax gooya leh" works well. But it can also apply to tools like axes. To make it more precise, you can add how it is used, such as "oo loo isticmaalo jarista cuntada ama walxaha yaryar" (used for cutting food or small items).<br><br>Treatment of middi: Treat *middi* as a phonetic variant of *mindi*. You should keep *mindi* as the main entry and list *middi* as a variant form pointing back to *mindi*. Do not create a separate full entry for it initially. | accepted | Add reviewed entry `sls:lex:000004` with the narrower gloss and `variants: ["middi"]`; add the reviewed variant field to the schema and R8. |

## Transition record

| Transition | Date | Approver | Gate evidence |
|---|---|---|---|
| planned → Draft | 2026-09-04 | Maintainer | [Issue #24](https://github.com/goobolabs/somali-language-standard/issues/24); formal template completed; [evidence map](SLS-0100-evidence-map.md) and pilot boundaries recorded |
| Draft → Proposed | *not recorded* | Maintainer | Requires resolved Draft findings and an explicitly opened ≥14-day public-comment period |
