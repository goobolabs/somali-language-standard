# Audit record — Weer fudud

- **Resource path:** `resources/naxwe/09-weer-fudud.md`
- **Collection / family:** naxwe / primary grammar
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; conservative SLS cleanup applied and awaiting review
- **Audit started:** 2026-08-12
- **File size at audit start:** 234 lines; 1,465 words; 9,220 bytes
- **Resource SHA-256 at audit start:**
  `3da14a0a9357212e1c45b0d9c73fe245f020ab40b3950de5a0ccf241f44b5e3a`
- **Resource-text changes during audit:** none

## Target output model

This file is a coherent primary-grammar chapter rather than an OCR transcript.
It has one H1, five H2 sections, three H3 subsections, one valid Markdown table
with six data rows, one fenced tree diagram, and 25 numbered forms/examples on
24 entry lines (32 numbered references in total when later cross-references are
included). It has no front matter, exercises, scan debris, malformed
tables, or broken reading order.

Cleanup should preserve its complete progression from predicates and their
arguments through subject case, focus particles, reduced pronouns, and the
simple-sentence tree. It should retain all numbered source material and all six
rows of the focus/order table. The cleanup must correct the internally reversed
word-order statements in the `baa/ayaa` rules, resolve the contradiction between
the third-person zero object and the example marked wrong for lacking an overt
object, scope the tree and final sentence so they do not contradict imperative
and nominal predicates, and repair supported terminology and wording defects.
Source-specific or insufficiently supported forms must be retained with a note
rather than silently normalized.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-19 | reviewed | N09-R001 |
| 21-36 | reviewed | N09-R002 |
| 38-58 | reviewed | N09-R003 |
| 60-71 | reviewed | N09-R004 |
| 73-92 | reviewed | N09-R005 |
| 94-140 | reviewed | N09-R006 |
| 141-155 | reviewed | N09-R007 |
| 157-180 | reviewed | N09-R008 |
| 182-207 | reviewed | N09-R009, N09-R010 |
| 209-234 | reviewed | N09-R011 |
| whole file | reviewed | N09-R012 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| N09-R001 | 1-19 | Clean introduction to word order and the predicate, with two contrastive opening sentences, one complete imperative, and one incomplete sequence; source numbering jumps from 4 to 14 later / low | `resources/naxwe/ereyfur.md` supports `weer`, `khabar`, and `weer fudud`; `resources/naxwe/12-noocyada-weeraha.md` independently treats imperatives as sentences; the numbering belongs to the primary source series | Retain all four examples, the word-order explanation, and the predicate-completion contrast. Preserve every source number and gap; add a brief numbering note rather than filling examples 5-13. | `repository-supported`; `intentional-retained`; `structural-only` |
| N09-R002 | 21-36 | Useful predicate-valency account and one-, two-, and three-argument contrast; `Miiska waa yar yahay` is inconsistent with the chapter's later nominative-subject discussion, but the repository does not independently establish the exact repair / high | `resources/naxwe/11-weerta-adag.md` also uses `mawduuc` for predicate arguments; the glossary instead maps `mawduuc` to English “topic,” creating a source/glossary terminology conflict; lines 75-81 introduce nominative `-u` and pitch marking | Retain the valency sequence, examples 14-15, and three-branch explanation. Define `mawduuc` explicitly as this source's argument term rather than silently replacing it. Mark `Miiska waa yar yahay` unresolved during cleanup; do not change it to `Miisku...` without source or independent paradigm evidence. | `repository-supported` in part; `terminology-clarification`; `unresolved`; `intentional-retained` |
| N09-R003 | 38-58 | The semantic-selection and nominal-predicate discussion is valuable; technical heading `Weli` is absent from the glossary, while `Adigu waa arday` occurs nowhere else in the repository and is not safe to normalize by intuition / medium | The same primary grammar uses `Weli` in chapters 11 and 12, so it is not isolated OCR noise; `ereyfur.md` supports `khabar magaceed`; chapters 13 and 16 independently support `Cali waa macallin`; no repository parallel resolves example 21 | Retain the subsection, selection contrast, examples 20-21, tense-bearing `ahaa` construction, and omitted-subject example. Briefly identify `Weli` as a source heading/relationship label and mark `Adigu waa arday` as an unresolved source form; do not invent `baad tahay` or another replacement. | `intentional-retained`; `repository-supported` in part; `unresolved` |
| N09-R004 | 60-71 | The definition of a short/simple sentence and preposition-dependent noun phrases covers important material, but it first calls the relevant elements non-arguments and then analyzes recipient `Cali` as a predicate-selected prepositional argument; it also says an OM may consist of a whole sentence / high | Chapters 6 and 7 distinguish direct objects, preposition-dependent complements, and verb valency; chapter 10 defines OM structure and permits clause-containing modifiers without equating every complete sentence with an OM | Retain the definition, `Aadan buug buu Cali u keenay`, and the `ka cararay` contrast. Recast the category prose so an OM is a form class that may function as an argument, including a preposition-dependent one; describe clause-containing OM structure precisely and avoid claiming that an arbitrary full sentence is itself an OM. | `repository-supported`; `logic-correction`; `classification-correction` |
| N09-R005 | 73-92 | Subject, nominative case, and agreement account is useful; subsection 9.1.2 is incorrectly promoted to H2, and agreement uses `jinsiga`, which the glossary reserves for biological sex rather than grammatical gender; regional causation for `baa/ayaa` variation is unsupported / medium | `ereyfur.md` gives `cayn` = gender, `jinsi` = sex, `kays yeelaale` = nominative case, and `qurub diiradeed`; dictionary `baa` records `aa/ayaa/yaa` as variants but does not establish a geographic division | Retain examples 37 and 40, nominative marking, agreement dimensions, and the functional equivalence of `baa/ayaa`. Demote 9.1.2 to H3, replace grammatical `jinsiga` with `caynta`, and remove or qualify only the unsupported claim that the variant choice is determined by region. | `repository-supported`; `heading-repair`; `terminology-alignment`; `scope-correction` |
| N09-R006 | 94-140 | Core `baa/ayaa` focus analysis and complete six-order table are valuable, but Xeer II says the subject follows the verb although examples 14-15 place it between the focused object and verb; Xeer III and the table correctly distinguish that position from a genuinely postverbal subject / fatal | The same-file rows 136-139 show `LY-(buu/baa) Y F` as optional and `LY-(buu/baa) F Y` as requiring the subject clitic; chapter 12 relies on these word-order rules | Retain every example and all six table rows. Correct Xeer II to “subject follows the focused OM but precedes the verb”; keep Xeer III as the summary and make its prose match the table exactly. Do not alter grammaticality judgments or create further orders. | `same-file-exact`; `logic-correction`; `intentional-retained` |
| N09-R007 | 141-155 | Third-person and first/second-person extensions preserve useful coverage, but the final paragraph again says omission is optional when the subject follows either the verb or the focused OM, contradicting Xeer III and the final two table rows / fatal | Lines 122-139 require a clitic for a postverbal subject and permit omission only when the subject follows the focused OM but precedes the verb | Retain the gender/person examples and the discourse note about omitted free pronouns. Remove `falka ama` from the optionality statement and state the exact licensed order; keep `Moos baan cunay` without asserting an absent postverbal subject. | `same-file-exact`; `logic-correction`; `scope-correction` |
| N09-R008 | 157-180 | Important `waa` contrast and four predicate patterns; numbering skips 9.2.2, the opening claim that `waa` or `baa/ayaa` can never be absent from any declarative is too broad, and line 170 has malformed `Meel kale oo waa uga dhaca weer waa galad` / high | Chapter 12 uses `ma` in negative declaratives and treats several sentence environments, so the absolute particle requirement needs an affirmative-focus scope; chapters 13 and 16 support verbal, adjectival, nominal, and demonstrative uses of `waa`; ordinary repository spelling supports `qalad`, not `galad`, here | Preserve source heading 9.2.3 and document the missing 9.2.2 rather than renumbering it. Scope the exclusivity rule to the affirmative focus constructions being described; retain examples 41, 43, 44, 51, and 52. Repair line 170 grammatically and change only the clear contextual typo `galad` to `qalad`. | `repository-supported`; `scope-correction`; `grammar-correction`; `orthographic-correction`; `intentional-retained` |
| N09-R009 | 182-203 | The reduced subject/object contrast and overt first/second-person object clitics are useful, but the categorical rule and example 7 contradict the immediately following third-person zero-object analysis: `Cali Xasan buu arkay` cannot be rejected merely because it lacks an overt object clitic / fatal | Cleaned chapter 4 explicitly retains third-person object `∅` and says its interpretation depends on the verb and discourse; this file's lines 197-199 state the same zero analysis; chapter 6 supports overt `i, ku, na, idin` combinations | Retain examples 1-2, example 9, the zero analysis, and preposition-dependent object contrast. Recast the rule to distinguish overt first/second-person object clitics from third-person zero. Remove the asterisk and false explanation from example 7, or label its intended reading unresolved if source context requires a different judgment; do not add an overt third-person clitic. | `repository-supported`; `same-file-exact`; `logic-correction`; `classification-correction` |
| N09-R010 | 205-207 | `Wuu kaaga qaaday` is a useful complex form, but its stated decomposition `ku + u + ka` conflicts with the maintained combination table / high | Chapter 6 places `kaaga` in the second-person-object row under the two-preposition result `kaga`; the same table gives `ku + u + ka` as `kuuga`, not `kaaga` | Retain `kaaga` and its example, but align the segmentation and role explanation with chapter 6's table. Do not infer a new surface form or preserve the incorrect `ku + u + ka` equation. | `repository-supported`; `morphological-correction` |
| N09-R011 | 209-234 | The tree and `Axmed buug buu qoray` analysis are useful as one transitive focus construction, but the diagram treats all particles uniformly inside KF and the final universal “one OM and one verb” requirement contradicts this chapter's own `Kaaley!`, omitted subject, and nominal predicates / fatal | Lines 13, 48-58, and 178-180 supply the internal counterexamples; chapter 10 supports OM structure; chapters 11-12 cover complex and nondeclarative extensions; chapter 16 preserves `W = OM + OF` only as a source-specific model | Retain the complete tree and example, explicitly label them as the source's schematic analysis of this transitive `baa` pattern, and avoid generalizing the exact branches to every `baa/ayaa/waa` construction. Replace the conclusion with a scoped statement: a sentence has a predicate and its licensed arguments, while a subject may be implicit and a predicate need not be a lexical verb. | `same-file-exact`; `interpretive-qualification`; `logic-correction`; `intentional-retained` |
| N09-R012 | whole file | The chapter has 25 numbered forms/examples on 24 entry lines, 32 numbered references in total, multiple gaps and resets, one complete table, and no navigation to prerequisite/detailed chapters / medium | Extant source numbers are 1-4, 14-15, 20-21, 37, 40, 11-12, 14-15, 32-33, 41, 43-44, 51-52, 1-2, 7, and 9; chapters 4, 6, 10, and 12 plus the glossary resolve terminology and dependent systems | Preserve all 25 numbered forms/examples, every reset and gap, the six-row table, and the tree. Add a source-numbering note and concise links to reduced pronouns, prepositions, OM structure, sentence types, and terminology. Add no replacement examples or reconstructed missing subsection. | `structural-only`; `intentional-retained`; `repository-supported` |

## Proposed SLS-native blueprint

The cleaned chapter should retain its title and present sequence:

1. word order and the predicate;
2. predicate valency, semantic selection, and verbal/nominal predicates;
3. simple-sentence arguments, preposition-dependent OMs, subject case, and
   agreement;
4. `baa/ayaa` position and subject-clitic rules, including the complete
   six-order table;
5. the distinct `waa` pattern;
6. reduced subject and object pronouns, including third-person zero and
   preposition combinations; and
7. the source's tree analysis of `Axmed buug buu qoray`, carefully scoped to
   that construction.

All 25 numbered forms/examples should remain with their original source
numbers, gaps, and resets. The six table rows and tree should remain. Cleanup
should repair only the exact contradictions and terminology defects identified
above, retain unresolved `Miiska waa yar yahay` and `Adigu waa arday` with
source-status notes, and add resolving links rather than expanding the chapter.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-12 with the
  instruction, "go ahead."
- **Approved finding IDs:** N09-R001 through N09-R012
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no
- **Decision requested:** review and approve the cleaned resource before the
  cleanup-approval or complete stages are marked.

## Audit validation

- Resource SHA-256 after audit:
  `3da14a0a9357212e1c45b0d9c73fe245f020ab40b3950de5a0ccf241f44b5e3a`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, five H2, three H3, one Markdown table with six
  data rows, and one fenced tree diagram.
- Numbered content checked: 25 numbered forms/examples on 24 entry lines and 32
  numbered references in total; all source gaps and resets recorded.
- Existing local links: none; all proposed target files exist.
- Source forms deliberately unresolved: `Miiska waa yar yahay` and
  `Adigu waa arday`.

## Cleanup result and review

### Applied cleanup

- Added a source-numbering note and preserved every original gap and reset.
- Defined `mawduuc` as this primary source's argument term while retaining it,
  and linked the parallel use in chapter 11.
- Retained `Miiska waa yar yahay` and `Adigu waa arday` unchanged with explicit
  unresolved-source notes rather than conjectural replacements.
- Retained technical heading `Weli`, identified its repeated source use, and
  recorded that the glossary does not define it.
- Recast the OM discussion so a predicate argument may have OM form and an OM
  may contain a relative clause; retained both preposition-dependent examples.
- Corrected the 9.1.2 heading level, aligned grammatical `jinsiga` with
  glossary `caynta`, and removed the unsupported geographic explanation of
  `baa/ayaa` variation.
- Corrected both reversed optionality statements: omission is optional when the
  subject follows the focused OM but precedes the verb, not when it follows the
  verb. All six order-table rows and judgments remain unchanged.
- Documented the source's missing 9.2.2, scoped the `waa`/`baa/ayaa` requirement
  to the affirmative constructions under discussion, and repaired the malformed
  line containing `galad` to a grammatical statement with `qalad`.
- Reconciled the reduced-object rule with third-person zero marking. The source
  sentence `Cali Xasan buu arkay` remains, but its contradictory asterisk and
  missing-clitic explanation were replaced by the approved zero-object reading.
- Corrected the decomposition of `kaaga` against chapter 6 while retaining the
  form and example `Wuu kaaga qaaday`.
- Retained the tree, scoped it to the displayed transitive `baa` construction,
  and replaced the contradictory universal “one OM and one verb” conclusion
  with a predicate-and-arguments account that permits implicit subjects and
  nominal predicates.
- Added seven resolving link occurrences to six maintained grammar and glossary
  targets.

### Deliberately retained

- All 25 numbered forms/examples on their original 24 entry lines, with every
  number, reset, and gap. The wording of example 7 remains; only its approved
  grammaticality mark and explanation changed.
- Every row and judgment in the six-row `baa/ayaa` order table.
- The complete predicate, valency, subject-case, focus, reduced-pronoun, and
  tree topic sequence.
- Source terms `mawduuc` and `Weli`, with provenance clarification rather than
  silent replacement.
- Unresolved source forms `Miiska waa yar yahay` and `Adigu waa arday`.

### Cleanup validation

- `git diff --check`: passed.
- Numbered content: 25 forms/examples on 24 entry lines and 32 numbered
  references in total; no source number was added, removed, or renumbered.
- Focus table: all six original data rows and their judgments are unchanged.
- Structure: one H1, four H2 headings, four H3 headings, one valid Markdown
  table, and one fenced tree diagram.
- Required repairs verified: the two reversed optionality statements, `galad`,
  the `ku + u + ka` decomposition, and the false missing-third-person-clitic
  explanation are absent.
- Unresolved/source-form preservation verified for `Miiska waa yar yahay` and
  `Adigu waa arday`.
- Local links: seven occurrences checked across six unique existing targets;
  the requested chapter-6 heading anchor exists.
- Provenance TSV validation: every row has 10 tab-separated fields.
- Post-cleanup navigation follow-up: both glossary references now target the
  approved bilingual `ereyfur.md`; no grammar content changed.
- Cleaned file size: 275 lines; 1,720 words; 11,328 bytes.
- Cleaned SHA-256:
  `5ac2df43e70ac6c95c8e9ddf1cfc7fc0369e632a53fa40c51062dec0d37216ea`.

## Primary-PDF content amendment — 2026-08-12

The scan disproves the cleanup note that source section 9.2.2 was absent.
Source-backed headings 9.1.3, 9.2.2, 9.2.2.1 and 9.2.3.1 were restored from
PDF pages 183–184 and 192–193. The new 9.1.3 summary preserves the source's
old-information/new-information contrast and examples 47–52. Section 9.2.2
now preserves examples 32–37 and the source rule that first/second-person
subject clitics are required regardless of constituent order. No exercises
were restored.
