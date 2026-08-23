# Audit record — Weerta adag

- **Resource path:** `resources/naxwe/11-weerta-adag.md`
- **Collection / family:** naxwe / primary grammar
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-12
- **File size at audit start:** 249 lines; 1,451 words; 9,426 bytes
- **Resource SHA-256 at audit start:**
  `1682268e2ecea74fe130839d2631bb2970aabe51b6b4bc2d53bafcfd338aa9eb`
- **Resource-text changes during audit:** none

## Target output model

This is a coherent primary-grammar chapter rather than an OCR transcript. It
has one H1, six H2 headings, five H3 headings, 36 numbered example entries, two
Markdown tables with seven data rows total, two fenced diagrams, and no links.
It has no front matter, exercises, scan debris, malformed tables, or broken
reading order.

Cleanup should preserve its complete coverage of complement clauses, relative
clauses, omitted arguments, the four relative-clause verb patterns, stative
modifiers, restrictive/appositive relatives, and time, condition, concession,
cause, and purpose relations. Every numbered example and all seven table rows
should remain. Two diagrams should remain as diagrams but be rebuilt to match
the surrounding prose and examples. Exact logic, label, heading, and spelling
errors should be corrected; categorical or source-only analyses should be
scoped rather than silently promoted to repository-wide SLS rules.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-25 | reviewed | N11-R001 |
| 27-50 | reviewed | N11-R002 |
| 51-66 | reviewed | N11-R003 |
| 68-76 | reviewed | N11-R004 |
| 77-116 | reviewed | N11-R005 |
| 117-146 | reviewed | N11-R006 |
| 148-170 | reviewed | N11-R007 |
| 172-200 | reviewed | N11-R008 |
| 202-218 | reviewed | N11-R009 |
| 220-228 | reviewed | N11-R010 |
| 229-249 | reviewed | N11-R011 |
| whole file | reviewed | N11-R012 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| N11-R001 | 1-25 | Useful definition and complement/relative distinction, but saying a dependent clause cannot “make a sentence” is circular, and the universal claim that every dependent verb is in dependent mood contradicts example 23's extended indicative form and the later four-way paradigm / fatal | `ereyfur.md` supports `weer dhimman`, `hab dhimman`, `weer dhammaystir`, and `weer faahfaahineed`; dictionary `in²` and same-file example 23 support an extended indicative complement-subject form; later lines 136-145 explicitly assign different paradigms by construction | Retain examples 1-2 and the two-way chapter map. Say a dependent clause cannot normally stand as an independent sentence in the intended construction. Scope absence of focus particles and dependent mood to the constructions where they occur; state that later sections identify the licensed exceptions/forms rather than asserting one form for every subordinate clause. | `repository-supported`; `same-file-exact`; `logic-correction`; `scope-correction`; `intentional-retained` |
| N11-R002 | 27-50 | `in` complementizer, examples 7-8 and 15-16, and tense-selection topic are valuable; however, the list of verbs requiring only `joogto` and the claim about “most” other verbs are supported only by this source / medium | Dictionary `in²` independently defines `in` as linking subject/object complement clauses and gives parallel examples; `ereyfur.md` supports `dhammaystire`; repository searches do not independently verify the complete `doon, sug, qaadso, tali, weyddiiso, isku day, bilow, joogi` selection list | Retain all four examples, the `in` account, and the full lexical list. Present the tense-selection generalization and inventory as this chapter's analysis, not an exhaustive SLS rule; link maintained tense/aspect and sentence-type resources without adding paradigms. | `repository-supported` in part; `interpretive-qualification`; `intentional-retained` |
| N11-R003 | 51-66 | Subject-complement coverage is important, but `Weer dhammaystiri` has broken agreement; the text calls object-clause order `Y+F` instead of `LY+F`; and example 23 is actually the alternate pre-predicate subject-clause order, not evidence for the preceding object contrast / fatal | Examples 15-16 have complement layeele before the main predicate (`LY+F`); examples 20-22 have main predicate before complement subject (`F+Y`); example 23 and dictionary `in²` support a complement subject before predicate (`Y+F`) with an extended indicative verb | Retain examples 20-23 and `yimaaddaa`. Correct `Weer dhammaystiri` to `Weer dhammaystirku`, correct the object label to `LY+F`, and describe example 23 separately as the alternate `Y+F` subject order. Preserve `timaaddid` as a source form because the repository does not uniquely resolve it. | `repository-supported`; `same-file-exact`; `grammar-correction`; `label-correction`; `unresolved`; `intentional-retained` |
| N11-R004 | 68-76 | Correctly introduces a relative clause as modifier inside OM, but the starred `Buugaggii aan kuu keenay` is a well-formed nominal phrase/fragment even though it is not a complete independent declarative sentence / medium | Cleaned chapter 10 treats head plus relative clause as an OM; ordinary nominal phrases can occur as fragments or answers; adding `way fiican yihiin` makes the intended full declarative complete | Retain examples 1-2 and their contrast. Scope the asterisk/judgment to “not a complete independent sentence in this intended use,” rather than labeling the nominal string intrinsically ungrammatical. Link chapter 10. | `repository-supported`; `judgment-qualification`; `intentional-retained` |
| N11-R005 | 77-116 | The head-gap account and examples 3 and 7-9 are central, but the first tree places `(warqadda qortay)` under OF instead of inside the subject OM; the starred filled-gap sentence can have another appositive/nested reading; and the final clitic summary reverses chapter 9 and its own examples 7-9 / fatal | The prose at lines 68-97 and chapter 10 put the relative clause inside OM. Cleaned chapter 9's exact order table makes the subject clitic optional for reconstructed `LY+Y+F`, demonstrated here by 7 without and 8 with `uu`; it is required for reconstructed `Y+LY+F`, represented by 9 with `uu` | Retain every example and the head/gap rule. Rebuild the tree so subject OM contains head `gabadhii` plus relative clause `warqadda qortay`, while `waa walaashay` is OF/KF. Scope the filled-gap asterisk to the intended co-reference reading. Rewrite annotations 7-9 clearly and reverse the erroneous summary: `LY+Y` optional (7/8), `Y+LY` required (9). | `same-file-exact`; `repository-supported`; `diagram-repair`; `logic-correction`; `judgment-qualification`; `intentional-retained` |
| N11-R006 | 117-146 | Complete four-way table and examples 14-17 preserve valuable source analysis; 11.2.2 is incorrectly promoted to H2, and the reduced/extended/dependent-form assignments are not independently demonstrated elsewhere / medium | `ereyfur.md` supports `baradigme kooban`, `baradigme fidsan`, and `hab dhimman`; example 16 recurs in chapter 16, but the complete four-way generalization and verbal endings occur only here | Demote 11.2.2 to H3. Retain examples 14-17, every table row, and the three form descriptions as this primary source's analysis. Keep the shorter first-row table example alongside the fuller numbered example; do not infer missing persons or new forms. | `repository-supported` in part; `heading-repair`; `interpretive-qualification`; `intentional-retained` |
| N11-R007 | 148-170 | Stative-modifier topic and examples 1, 6, and 7 are useful; `Weli` is an undefined repeated source heading, hidden `ah` is a source analysis, and the `Meel gabow` diagram is isolated, duplicates `meel` inside OF, and does not represent the immediately preceding examples / fatal | Chapter 7 independently supports stative verbs combining with `ahow` in present paradigms but does not establish a universally hidden `ah`; `Weli` recurs as a primary-source heading but is absent from `ereyfur.md`; `Meel gabow` occurs nowhere else, while `Nin wanaagsan` is the actual displayed OM | Retain the heading, all three numbered forms, the past/present contrast, and the hidden-`ah` analysis explicitly as the source's account. Add the same provenance note for `Weli` used in chapters 9 and 12. Rebuild the diagram from `Nin wanaagsan`, showing head `nin` and modifier `wanaagsan` with source-analyzed hidden `ah`; do not retain `Meel gabow` as an unexplained authoritative example. | `repository-supported` in part; `unresolved`; `diagram-repair`; `interpretive-qualification`; `intentional-retained` |
| N11-R008 | 172-200 | Restrictive/appositive distinction, three-row table, and examples are valuable, but the table says an indefinite head cannot take an appositive relative while example 6 immediately labels indefinite `Gaari cusub oo cagaaran` appositive; `oo/ee` distribution is then stated categorically despite `oo` also coordinating modifiers / fatal | `ereyfur.md` supports both relative-clause terms; chapter 6 independently analyzes `oo` as coordinating modifiers (`wiil dheer oo cad`) and gives broader functions for both `oo` and `ee`; no other source resolves this chapter's contradictory label | Retain the definitions, all three table rows, and examples 6-7. Present the table as the source's distributional model. Reclassify example 6 conservatively as an `oo`-linked modifier sequence that does not independently prove appositive status; keep example 7 as the source's restrictive `ee` pattern. Remove the internally contradictory absolute `oo/ee` rule and link chapter 6. | `repository-supported`; `same-file-exact`; `classification-correction`; `scope-correction`; `intentional-retained` |
| N11-R009 | 202-218 | Four time examples preserve important adverbial coverage, but the section calls all forms relative clauses and all `markii, intii, inta, ilaa` nominal heads; repository categories instead distinguish a linker, a dependent form, and a preposition / high | Dictionary gives `markii` as `xi` (linker), `inta` as a dependent form (`fk`), and `ilaa` as `h` (preposition), including a parallel `Ilaa ... aan soo noqonayo` example; chapter 10 supports only the nominal-relative subset | Retain examples 1-4 and their four temporal relations. Recast §11.5 as adverbial/dependent constructions of several structural types rather than one relative-clause class, and identify the repository-supported categories without forcing a single analysis. Retain `foogga sare`, `galabka`, and `ka imanayo` as unresolved source wording rather than guessing replacements. | `repository-supported`; `classification-correction`; `scope-correction`; `unresolved`; `intentional-retained` |
| N11-R010 | 220-228 | Condition and concession examples are useful, but `haddii` is called a nominal head in the conclusion and `inkastoo` is classified as an appositive relative without independent evidence / high | Dictionary classifies `haddii` as a linker introducing conditional/temporal clauses; chapter 17 independently uses `Haddaad timaaddo` as a conditional; repository evidence does not establish the claimed appositive-relative category for `inkastoo` | Retain examples 6-8 unchanged. Identify `haddii` as a conditional linker and `inkastoo` as the source's concessive linker/construction; do not call either a nominal head or silently validate the appositive-relative classification. | `repository-supported` in part; `classification-correction`; `unresolved`; `intentional-retained` |
| N11-R011 | 229-249 | Cause and purpose coverage is valuable, but `ximmayn` is unsupported where `xirmayn` is required; `maxaa yeelay` is structurally unlike a head-relative construction; and `inuu` is wrongly decomposed as `in` plus preposition `u` although `uu` is the subject clitic / fatal | Dictionary supports `xirmayn`, records `maxaayeelay` as a lexical form, defines `in²` as complementizer, and chapter 4 gives `uu` as the third-person masculine subject clitic; in example 14 the separate preposition `u` occurs with main `baranayaa`, not inside `inuu` | Retain examples 11-12, 14, and 16. Correct `ximmayn` to `xirmayn`; distinguish causal `maxaa yeelay` from head-relative constructions; explain `inuu = in + uu`, while the separate `u` belongs to the main predicate. Treat `si` as the purpose-linking form shown in example 16. Replace the unsupported cross-language conclusion and false noun-head list with a compact summary of the distinct temporal, conditional, concessive, causal, and purpose constructions. | `repository-supported`; `morphological-correction`; `orthographic-correction`; `classification-correction`; `scope-correction`; `intentional-retained` |
| N11-R012 | whole file | The chapter contains 36 numbered entries with gaps and resets, two tables, two diagrams needing repair, and no navigation to prerequisite or dependent systems / medium | Extant example series are 1-2; 7-8; 15-16; 20-23; 1-3; 7-9; 14-17; 1, 6-7; 6-7; 1-4; 6-8; 11-12; and 14, 16; chapters 4, 6, 7, 9, 10, and 12 plus the glossary provide maintained detail | Preserve every numbered entry, reset, gap, and all seven table rows. Retain two fenced diagrams but rebuild their branches/labels under N11-R005 and N11-R007. Add a source-numbering note and concise resolving links; add no inferred examples, missing numbers, tables, or paradigms. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned chapter should retain its title and sequence:

1. dependent sentences and the distinction between complement and relative
   clauses;
2. `in` complement clauses as objects and subjects, including both subject
   positions;
3. relative clauses inside OM, the head/gap relation, and corrected subject-
   clitic order;
4. the complete four-way table of main-clause and relative-clause roles;
5. stative relative modifiers and the source's hidden-`ah` analysis;
6. restrictive and appositive relatives, with the `oo` inconsistency resolved
   conservatively; and
7. structurally distinguished temporal, conditional, concessive, causal, and
   purpose dependent/linking constructions.

All 36 numbered entries and seven table rows should remain with their source
numbers, gaps, and resets. Both diagrams should remain, but the first should
put the relative clause inside OM and the second should use the actual `Nin
wanaagsan` example. Cleanup should change no source-only form unless the audit
identifies a unique correction, and should leave `timaaddid`, `foogga sare`,
`galabka`, and `ka imanayo` explicitly unresolved.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-12 with the
  instruction, "go ahead."
- **Approved finding IDs:** N11-R001 through N11-R012
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes
- **Decision requested:** review and approve the cleaned resource before the
  cleanup-approval or complete stages are marked.

## Audit validation

- Resource SHA-256 after audit:
  `1682268e2ecea74fe130839d2631bb2970aabe51b6b4bc2d53bafcfd338aa9eb`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, six H2, five H3, two Markdown tables with seven
  data rows total, and two fenced diagrams.
- Numbered content checked: 36 example entries and 44 numbered references in
  total; all source gaps and resets recorded.
- Existing local links: none; all proposed target files exist.
- Source forms deliberately unresolved: `timaaddid`, `foogga sare`, `galabka`,
  and `ka imanayo`.

## Cleanup result and review

### Applied cleanup

- Scoped the definition of a dependent sentence and the focus/dependent-mood
  diagnostics so they no longer contradict the chapter's later extended and
  construction-specific verb forms.
- Added a source-numbering note and retained the complement/relative-clause
  chapter map and examples 1-2.
- Retained the complete complement-selecting verb inventory as the source's
  analysis, qualified its completeness, and linked maintained tense/aspect
  treatment.
- Corrected `Weer dhammaystiri` to `Weer dhammaystirku`; separated complement
  object `LY+F`, post-predicate complement subject `F+Y`, and pre-predicate
  complement subject `Y+F`. Example 23 and `yimaaddaa` remain.
- Scoped the asterisk on `Buugaggii aan kuu keenay` to its failure as a full
  independent declarative in the intended use and linked the OM prerequisite.
- Rebuilt the first tree so the subject OM contains head `gabadhii` and relative
  clause `warqadda qortay`, while `waa walaashay` forms the predicate branch.
- Scoped the filled-gap asterisk to the intended co-reference reading and
  retained the possible alternate `Maryam`-subject reading.
- Reannotated examples 7-9 and corrected the reversed clitic rule: reconstructed
  `LY+Y+F` permits omission (7/8), whereas `Y+LY+F` requires the subject clitic
  (9). Linked file 09's exact order table.
- Demoted 11.2.2 to H3 and retained every row of the four-way relative-clause
  table as the primary source's analysis rather than expanding it into a new
  person paradigm.
- Retained technical source heading `Weli` with a glossary-status note, scoped
  hidden `ah` as the source's analysis, and rebuilt the second diagram from the
  actual example `Nin wanaagsan` instead of isolated `Meel gabow`.
- Retained the restrictive/appositive definitions and complete three-row table,
  but resolved its conflict with indefinite example 6 by treating that example
  as an `oo`-linked modifier sequence that does not alone prove appositive
  status. Linked chapter 6's broader `oo/ee` treatment.
- Recast §11.5 as several dependent constructions with adverbial relations.
  Distinguished temporal, conditional, concessive, causal, and purpose forms
  without removing any example.
- Corrected `ximmayn` to `xirmayn`; distinguished `maxaa yeelay` from a
  head-relative structure; corrected the analysis of `inuu` to `in + uu` and
  kept the main predicate's separate preposition `u` distinct.
- Replaced the unsupported cross-language conclusion and false single noun-head
  analysis with a compact inventory of the distinct constructions.
- Added seven resolving links to seven maintained grammar/glossary targets.

### Deliberately retained

- All 36 numbered example entries, every source number, gap, and reset.
- Both Markdown tables and all seven original data rows; no table judgment or
  example cell was changed.
- Both diagram topics, rebuilt only to agree with the approved prose and actual
  numbered examples.
- Complete coverage of complement clauses, relative clauses, head/gap
  relations, four verb-form cases, stative modifiers, restrictive/appositive
  relatives, and the five adverbial semantic relations.
- Source forms `timaaddid`, `foogga sare`, `galabka`, and `ka imanayo`, each
  accompanied by an unresolved/source-status note.
- All source lexical inventories and the source's four-way relative-clause
  analysis, with qualifications rather than deletions.

### Cleanup validation

- `git diff --check`: passed.
- Numbered content: all 36 numbered entry lines and their numbers remain; 43
  numbered references remain after the approved clitic-summary rewrite removed
  one duplicate prose cross-reference.
- Tables: both original tables and all seven original data rows are byte-for-
  byte unchanged.
- Diagrams: two opening and two closing fences remain; both approved diagrams
  now use examples from the surrounding text.
- Structure: one H1, five H2 headings, six H3 headings, two valid Markdown
  tables, and two fenced diagrams.
- Required repairs verified: `dhammaystiri`, `ximmayn`, the wrong object `Y+F`
  label, the reversed clitic summary, false `in + u` analysis, and isolated
  `Meel gabow` diagram are absent.
- Unresolved/source forms verified: `timaaddid`, `foogga sare`, `galabka`, and
  `ka imanayo` remain unchanged.
- Local links: seven occurrences across seven unique existing targets; the
  requested chapter 6 and chapter 9 anchors exist.
- Provenance TSV validation: every row has 10 tab-separated fields.
- Post-cleanup navigation follow-up: both glossary references now name the
  approved bilingual `ereyfur.md`; no grammar content changed.
- Cleaned file size: 313 lines; 1,937 words; 13,006 bytes.
- Cleaned SHA-256:
  `69f5b9aa759d2daa2fec4935fb2ca53f0ae3c7f6405d3b719519217d9eded0c0`.
