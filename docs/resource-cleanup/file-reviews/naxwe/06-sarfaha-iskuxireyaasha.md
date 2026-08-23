# Audit record — Sarfaha iskuxireyaasha

- **Resource path:** `resources/naxwe/06-sarfaha-iskuxireyaasha.md`
- **Collection / family:** naxwe / primary grammar
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-12
- **File size at audit start:** 249 lines; 1,364 words; 7,733 bytes
- **Resource SHA-256 at audit start:**
  `15a0bad00b9faa2189eee35569895aa4c3e5cf9661b407c326dde410fedfa713`
- **Resource-text changes during audit:** none

## Target output model

This file is already a structured primary-grammar chapter rather than an
OCR-shaped transcript. It has one H1, two H2 sections, twelve H3 subsections,
four valid Markdown tables with 29 total rows, and 58 numbered entries in two
source series. It contains no front matter, exercises, scan debris, false
headings, broken reading order, or malformed Markdown.

Cleanup should preserve the complete treatment of the four simple
prepositions, their combinations with each other and with object pronouns,
and the conjunctions `iyo`, `-na`, `oo`, `ee`, `-se`, `ama/mise`, and
`laakiin/haseyeeshee`. It should make one uniquely supported paradigm
correction (`is + ku = isku`), repair a clear word-boundary error, broaden two
definitions whose own later examples exceed their stated scope, qualify an
unsupported historical claim, distinguish the different constructions shown
under `oo` and `ee`, retain one unresolved source form without guessing, and
add navigation to the maintained pronoun, verb-valency, simple-sentence,
noun-phrase, and complex-sentence accounts.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-13 | reviewed | N06-R001 |
| 15-43 | reviewed | N06-R002 |
| 20-150 | reviewed | N06-R003 |
| 44-79 | reviewed | N06-R004 |
| 80-100 | reviewed | N06-R005 |
| 101-115 | reviewed | N06-R006 |
| 116-130 | reviewed | N06-R007 |
| 131-150 | reviewed | N06-R008 |
| 152-161 | reviewed | N06-R009 |
| 163-181 | reviewed | N06-R010 |
| 182-200 | reviewed | N06-R011 |
| 201-222 | reviewed | N06-R012 |
| 223-230 | reviewed | N06-R013 |
| 231-240 | reviewed | N06-R014 |
| 242-249 | reviewed | N06-R015 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| N06-R001 | 1-13 | Clean two-part taxonomy and supported pedagogical analogy; opening writes bound conjunction `-na` as bare `na` / low | `resources/qaamuus/24-i.md` defines `iskuxireyaal` as the umbrella for `xiriiriyeyaal` and `horyaalayaal`; `resources/naxwe/ereyfur.md` and dictionary abbreviations support all three terms; dictionary entries support `masaamiir` and `faseexad`; `resources/qaamuus/18-n.md` records conjunction `(-na)` | Retain the title, taxonomy, invariant-form point, and analogy. Write the bound conjunction as `-na` and add no new category. | `repository-supported`; `form-clarification`; `intentional-retained` |
| N06-R002 | 15-43 | The subject/preposition introduction overstates that adding any non-subject noun requires a preposition, although its own three-way valency list correctly includes direct objects and two-object verbs; `lama haystaan weerta` is malformed / high | `resources/naxwe/07-sarfaha-falalka.md` distinguishes transitive direct objects from preposition-marked complements; `resources/naxwe/09-weer-fudud.md` describes subjects, direct objects, and preposition-dependent noun phrases; dictionary entry `horyaale` specifies an indirect object | Retain examples 1-4 and all three verb groups. Recast the opening in terms of verb valency: subjects need no preposition, direct objects can be licensed without one, and some other complements require one. Repair the malformed sentence without adding examples; link chapters 7 and 9. | `repository-supported`; `logic-correction`; `grammar-correction` |
| N06-R003 | 20-150 | The preposition example series deliberately retains source labels but has gaps 5-12, 35-40, and 43-44; silent renumbering would break source traceability / medium | Same-file sequence has 30 extant entries labeled 1-4, 13-34, 41-42, and 45-46; no omitted source text is present elsewhere in the repository | Preserve every number and example exactly. Add one note that numbering follows the selected source examples and gaps do not authorize reconstruction. | `structural-only`; `unresolved`; `intentional-retained` |
| N06-R004 | 44-79 | The four-preposition inventory and most contextual labels are supported, but the statement that frequent use caused loss of original meanings is unsupported diachrony; example 15 describes degree/intensity as manner; `Muriiddi` occurs nowhere else and example 25's `la walwalsanaa`/“wax la haysto” analysis has no independent match / high | Chapters 13, 16, and 17 support `u, ku, ka, la` and the surrounding examples; dictionary `muriid` is masculine, but the repository does not uniquely choose `Muriidkii` versus `Muriidku`; dictionary `walwalsan` records `ka`, not the displayed `la`; `aad u jecel` transparently expresses degree in the displayed sentence | Retain examples 13-24 and the four-preposition sequence; describe contextual polysemy without asserting a historical cause and label example 15 as degree/intensification. Retain example 25 exactly as an unresolved source example and explicitly avoid guessing its noun ending or replacing its preposition; do not present “wax la haysto” as an established SLS function. | `repository-supported` in part; `interpretive-qualification`; `unresolved`; `intentional-retained` |
| N06-R005 | 80-100 | Useful complete simple-combination table and two examples; `Marka weer ka laga` has a clear word-boundary error, while `Nuqul` is source terminology also used in chapter 9 / medium | All nine resulting forms occur elsewhere in the repository; `resources/naxwe/09-weer-fudud.md` uses `nuqullo` for combined forms; ordinary syntax uniquely supports `Marka weerka laga` | Correct `weer ka` to `weerka`; retain all nine rows and examples 26-27. Clarify the result-column label if useful while retaining `nuqul` as the source term. | `repository-supported`; `word-boundary-correction`; `intentional-retained` |
| N06-R006 | 101-115 | Four-column object-pronoun/preposition table and examples are coherent; prose shifts between singular collective and plural agreement and lacks navigation / low | Dictionary entries support object forms `i, ku, na, idin`; chapter 4 gives the maintained object-pronoun paradigm; chapter 9 supports these combinations | Retain every cell and examples 28-30. Smooth only the agreement in the introduction and link chapters 4 and 9; infer no additional form. | `repository-supported`; `grammar-clarification` |
| N06-R007 | 116-130 | One uniquely established paradigm error: both `is + ku` and `is + ka` are shown as `iska`, erasing the contrast between `isku` and `iska` / high | Repository usage has extensive independent evidence for both `isku` and `iska`; the same table correctly gives `is + ka = iska`; dictionary `is` supports the reflexive/reciprocal pronoun; glossary supports `magacuyaal celis` and `magacuyaal qoflaawe` | Correct only `is+ku = iska` to `is+ku = isku`. Retain all other cells and examples 31-34; align the prose labels with `magacuyaal qoflaawe` and `magacuyaal celis`. | `repository-supported`; `paradigm-correction`; `terminology-alignment` |
| N06-R008 | 131-150 | Dense two-preposition/object-pronoun paradigm and impersonal combinations; all displayed outcomes have repository usage, though several rare forms have little evidence outside this source / medium | Repository-wide exact-form checks find all 24 table outcomes and all six impersonal outcomes; chapter 9 independently analyzes `kaaga`; chapters 4 and 17 support the component pronouns and prepositions | Retain every table cell, all six inline impersonal combinations, and examples 41-42 and 45-46. Add no inferred variants; label the table as the source's gathered paradigm and link the detailed syntax. | `repository-supported`; `intentional-retained` |
| N06-R009 | 152-161 | Opening definition limits conjunctions to equal, independent words or clauses, but later `oo` and `ee` examples include adjective linking, dependent constructions, and a noun phrase / high | Dictionary `xiriiriye` gives the coordinating core; chapters 10, 11, 13, 16, and 17 also document word-, phrase-, clause-, and dependent uses | Retain both transformation examples. Broaden the chapter introduction to say that conjunctions link words, phrases, or clauses, then identify equal independent coordination as the pattern illustrated here rather than the whole class; link chapters 10 and 11. | `repository-supported`; `scope-correction` |
| N06-R010 | 163-181 | `iyo` and bound `-na` sections are clean and repository-supported; attachment prose can be made more direct / low | Dictionary entries `iyo` and `na²`; chapters 13, 16, and 17; same-file examples | Retain examples 3-9 and all three `-na` attachment patterns. Preserve the hyphen in the heading and describe its attachment to the first word of the second proposition without changing any example. | `repository-supported`; `structural-only` |
| N06-R011 | 182-200 | The `oo` introduction has agreement errors; example 11 joins predicate modifiers rather than two complete declaratives, and example 14 is a dependent simultaneous/temporal construction rather than ordinary coordination of two independent verbs / high | Dictionary `oo` supports declarative linking and simultaneous actions; chapter 11 treats `oo` in additional relative/descriptive clauses; chapter 16 explicitly notes its descriptive use | Retain examples 10-14 exactly. Repair agreement; distinguish coordination in example 10, predicate-description in example 11, imperative sequencing in examples 12-13, and the simultaneous dependent construction in example 14; link chapter 11 without inventing new examples. | `repository-supported`; `classification-correction`; `grammar-correction` |
| N06-R012 | 201-222 | The several `ee` environments are supported, but example 18's actions are different/contrastive rather than inherently “opposite”; the noun-phrase use needs navigation / medium | Dictionary `ee` explicitly distinguishes non-free and free clauses; chapter 10 analyzes `ee` between noun-phrase modifiers; chapters 11 and 16 support dependent/descriptive functions | Retain examples 15-18 and the bracketed noun phrase. Replace the absolute “opposite actions” label with a contrast/difference description and link the maintained noun-phrase analysis. | `repository-supported`; `interpretive-qualification` |
| N06-R013 | 223-230 | Correct adversative classification and two clean examples; `oo isku daraa` has subject-verb agreement error / low | Dictionary `se` identifies it as a clause-contrasting conjunction attached at the end of another word; chapters 13 and 17 support `-se` | Retain examples 19-20 and the comparison with `-na`; repair the agreement only and preserve bound spelling `-se`. | `repository-supported`; `grammar-correction` |
| N06-R014 | 231-240 | Useful ordinary `ama`/`mise` contrast and four examples; the question/non-question division is stated categorically, while `joogeysaa/tegeysaa` are source spellings with no uniquely established paired normalization in this repository / medium | Dictionary `ama` supports nominal and clausal alternatives; `mise²` is marked interrogative; chapters 13 and 17 support both; repository evidence supports `tegaysaa` but does not independently establish `joogaysaa` as the unique replacement for this paired source wording | Retain all four examples and both displayed spellings. Present `ama` versus interrogative `mise` as the ordinary contrast illustrated here, not an exhaustive ban; do not normalize only half of the paired verb forms. | `repository-supported`; `interpretive-qualification`; `intentional-retained` |
| N06-R015 | 242-249 | `laakiin` and variant `haseyeeshee` are supported adversatives; comparing both globally to multifunctional `ee` is too broad, and example 25 contains interrogative `sow` without needing an inferred textual repair / medium | Dictionary entries support `laakiin`, `haseyeeshee`, and interrogative `sow`; chapters 13 and 17 classify the first two as adversative connectors | Retain the heading and examples 25-28 exactly, including `haseyeeshee` and source punctuation. Describe the two forms directly as adversative/contrastive and avoid implying that every use of `ee` has the same meaning. | `repository-supported`; `scope-correction`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned chapter should retain its title and existing sequence:

1. `iskuxireyaal` as the umbrella for prepositions and conjunctions;
2. verb valency and the four simple prepositions `u, ku, ka, la`;
3. simple preposition combinations;
4. combinations with object, impersonal, and reflexive pronouns;
5. two-preposition paradigms;
6. conjunctions joining words, phrases, predicates, and clauses;
7. the individual treatments of `iyo`, `-na`, `oo`, `ee`, and `-se`;
8. alternative `ama/mise`; and
9. adversative `laakiin/haseyeeshee`.

All 58 numbered example sentences/forms should remain, including the unresolved
example containing `Muriiddi`, the source spellings `joogeysaa/tegeysaa`, and
the source punctuation of example 25 in the second series. Their explanatory
labels may receive only the scoped corrections listed above. All table data
cells should remain except the uniquely approved correction `is+ku = iska` →
`is+ku = isku`. Source-number gaps should be documented, not filled or
renumbered.

Recommended links are `04-sarfaha-magacuyaallada.md`,
`07-sarfaha-falalka.md`, `09-weer-fudud.md`,
`10-dhismaha-oraah-magaceedyada.md`, and `11-weerta-adag.md`.

## Audit approval

- **Audit approval:** approved by the maintainer on 2026-08-12 with the
  instruction, "go ahead."
- **Approved finding IDs:** N06-R001 through N06-R015
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Cleanup result and review

### Applied cleanup

- Marked bound `-na`, retained the supported `iskuxireyaal` taxonomy, and
  documented the source-number gaps without filling or renumbering them.
- Recast the preposition introduction around verb valency, repaired the
  malformed complement explanation, and removed the unsupported historical
  claim about loss of original meanings.
- Clarified the contextual functions of the four prepositions, relabeled the
  degree use in `aad u jecel`, and retained example 25 with an explicit
  unresolved note instead of guessing a repair for `Muriiddi` or its
  preposition.
- Corrected `weer ka` to `weerka`, clarified the `nuqul` table header, and
  aligned the impersonal/reflexive terminology with the glossary.
- Corrected the single paradigm cell `is+ku = iska` to `is+ku = isku` while
  preserving every other table-data cell.
- Broadened the conjunction definition, distinguished the constructions shown
  under `oo`, qualified the contrast described under `ee`, repaired the `-se`
  agreement, and scoped the ordinary `ama/mise` distinction.
- Added six resolving links to the maintained pronoun, verb, simple-sentence,
  noun-phrase, and complex-sentence resources.

### Deliberately retained

- All 58 numbered example sentences/forms in their original wording and all
  original source numbers, including gaps 5-12, 35-40, and 43-44 in the first
  series.
- The unresolved `Muriiddi` sentence, paired source spellings
  `joogeysaa/tegeysaa`, `haseyeeshee`, and the punctuation of the `sow`
  example.
- Every table-data row except the uniquely supported `iska` → `isku`
  correction; every rare object/preposition and impersonal combination remains.
- The original two-section topic sequence and all twelve H3 topics.

### Validation

- `git diff --check`: passed.
- Original/current numbered italic-form comparison: no differences across all
  58 entries.
- Original/current table comparison: exactly one descriptive header change and
  one approved data-cell correction, `is+ku = iska` → `is+ku = isku`.
- Structure: one H1, two H2, twelve H3; 58 numbered entries; four Markdown
  tables with 29 total rows.
- Local-link resolution: 6 occurrences checked, 0 missing targets or anchors.
- Provenance TSV validation: every row has 10 tab-separated fields.
- Unresolved/source-form preservation: `Muriiddi`, `joogeysaa`, `tegeysaa`,
  and `haseyeeshee` all remain.
- Cleaned file size: 276 lines; 1,460 words; 9,058 bytes.
- Cleaned SHA-256:
  `cb94879cde20dbabcc77ac252c44f643fece72241d79b3b4c6dde97136e353d6`.
