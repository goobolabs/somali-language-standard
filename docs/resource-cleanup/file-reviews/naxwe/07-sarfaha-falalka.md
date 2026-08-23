# Audit record — Sarfaha falalka

- **Resource path:** `resources/naxwe/07-sarfaha-falalka.md`
- **Collection / family:** naxwe / primary grammar
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-12
- **File size at audit start:** 438 lines; 2,690 words; 16,299 bytes
- **Resource SHA-256 at audit start:**
  `417566d983d4b9e6867ae212f3e7930f36ade5023d36adee2eb4c192cb5fa957`
- **Resource-text changes during audit:** none

## Target output model

This file is already a coherent primary-grammar chapter, not an OCR-shaped
transcript. It has one H1, three H2 sections, nine H3 subsections, eight H4
subsections, eleven valid Markdown tables with 66 data rows, and 32 numbered
examples on 29 entry lines. It contains no front matter, exercises, scan
debris, false headings, malformed tables, or broken reading order.

Cleanup should preserve the full sequence from verb inflection through tense,
aspect, mood, conjugation classes, stative and prefix-conjugating verbs,
auxiliaries, transitivity, verbal extensions, and deverbal nouns. It should
retain every numbered example and all well-supported table data; correct the
uniquely established `gashay`, `qortaan`, `cuni lahaa`, prefix-verb, and future
forms; repair two reversed derivational explanations; distinguish habitual
past from past progressive; qualify a few categorical descriptions; and leave
the unsupported feminine agent forms explicitly unresolved rather than
inventing replacements.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-24 | reviewed | N07-R001 |
| 26-61 | reviewed | N07-R002 |
| 63-87 | reviewed | N07-R003 |
| 89-133 | reviewed | N07-R004 |
| 135-186 | reviewed | N07-R005 |
| 187-219 | reviewed | N07-R006 |
| 220-250 | reviewed | N07-R007 |
| 251-277 | reviewed | N07-R008 |
| 278-317 | reviewed | N07-R009 |
| 318-344 | reviewed | N07-R010 |
| 345-383 | reviewed | N07-R011 |
| 384-399 | reviewed | N07-R012 |
| 400-416 | reviewed | N07-R013 |
| 417-438 | reviewed | N07-R014 |
| whole file | reviewed | N07-R015 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| N07-R001 | 1-24 | Clean opening and supported `sal`/`nadooc` analysis; source numbering begins at 1 and then jumps to 7-8 / low | `resources/naxwe/ereyfur.md` and dictionary entries support `yeele`, `sal`, and `nadooc`; `resources/sarfe/02-falalka.md` uses the same person, number, gender, tense, aspect, and mood framework | Retain the opening example, two-constituent teaching model, both segmented examples, and all six inflectional dimensions. Preserve the source numbers and document their gaps rather than renumbering them. | `repository-supported`; `intentional-retained`; `structural-only` |
| N07-R002 | 26-61 | Person, number, and third-person-singular gender paradigms are coherent, but `gal + tay → qashay` combines the root of one verb with the past form of another / high | Dictionary `gal³` gives `(galay, gashay; geli)`; `qashay` belongs to unrelated roots such as `maqal`/`shaqal`; the other three morphophonological examples agree with their dictionary paradigms | Retain the complete seven-row person table and examples 19-22. Correct only `gal + tay → qashay` to `gal + tay → gashay`; do not infer or add further alternations. | `repository-supported`; `paradigm-correction` |
| N07-R003 | 63-87 | Complete three-tense overview and seven-person `cun` paradigm; the opening definition of present as only speech-time occurrence is narrower than the chapter's own later habitual and near-future uses / medium | `resources/sarfe/02-falalka.md` reproduces all 21 person/tense forms; lines 117-121 of this file independently give progressive, habitual, and near-future present uses | Retain examples 25-27 and every table cell. Phrase the tense definitions as temporal orientation, then let the aspect section state the present's actual uses; do not add another tense or paradigm. | `repository-supported`; `scope-correction`; `intentional-retained` |
| N07-R004 | 89-133 | Valuable aspect distinction and complete five-column `tag` paradigm; prose says the present has two aspects while listing three labels, and the simple-past definition incorrectly makes one-time punctuality obligatory / high | The displayed table has present `socota` and `caadaley`, not present `fudud`; `resources/sarfe/02-falalka.md` and chapter 8 support the same past/present forms; simple past morphology itself does not encode “mar qura” | Retain examples 30-32, both usage examples, and all 35 paradigm cells. Correct the two present labels to `socota iyo caadaley`; describe simple past as a completed/bounded presentation without requiring one occurrence or instantaneous duration. Keep the source's lack of a future-aspect table as its scoped presentation, not a universal prohibition. | `repository-supported`; `logic-correction`; `scope-correction` |
| N07-R005 | 135-186 | Five-mood treatment is structurally complete, but advisory 2nd-plural `aad qorteen` is a past form and conditional `cunlahaa` omits the infinitive marker / high | `resources/naxwe/12-noocyada-weeraha.md` has advisory `ad sheegtaan`; the same file's dependent table has `aad sheegtaan`; chapter 8 gives conditional `cuni lahaa` and `ma cuni lahayn` | Retain all mood definitions, examples, and person rows. Correct `aad qorteen` to `aad qortaan` and `(waa) cunlahaa` to `(waa) cuni lahaa`; preserve `aan/aad` in this source while linking the detailed sentence-type account, which uses `an/ad`. | `repository-supported`; `paradigm-correction`; `form-correction`; `intentional-retained` |
| N07-R006 | 187-219 | Clear three-class suffix-conjugation account and complete comparison table; several example verbs are source classification choices rather than independently demonstrated paradigms / medium | `resources/sarfe/02-falalka.md` supports the imperative diagnostics and all five comparison rows; dictionary entries support the listed imperative forms, including class-II `kari/samee` and class-III `xiro/raadso` | Retain all three classes, every listed diagnostic example, and all comparison-table cells. Add navigation to the maintained compact paradigm and chapter 8; do not expand the classifications from analogy. | `repository-supported`; `intentional-retained` |
| N07-R007 | 220-250 | Useful stative/adjectival-verb paradigm and source inventory; the absolute absence of progressive forms and the two lexical lists are broader than the independently demonstrated evidence / medium | `resources/sarfe/02-falalka.md` supports the complete `adag` paradigm; `ereyfur.md` supports `fal sifo`; dictionary entries support many but not every lexical classification in the two lists | Retain the seven-person `adag` table and both complete source lists. Present the progressive restriction and original/derived division as the chapter's class description, not a newly generalized repository rule; do not remove unusual members without source evidence. | `repository-supported` in part; `interpretive-qualification`; `intentional-retained` |
| N07-R008 | 251-277 | Prefix-conjugation comparison contains repeated overextended singular forms `aqaanaa`, gives a bare past `aqiin` without its person prefix, and mistakes the 1st-singular present `aal` for the root `ool` / high | Chapter 8 and `resources/sarfe/02-falalka.md` give `aqaan/taqaan/yaqaan/naqaan`, past `iqiin/tiqiin/niqiin`, and roots `aqoon, ool, dheh/iri, imow, ahow`; their present paradigm shows `aal` only as the 1st-singular form of `ool` | Retain the full prefix-vs-suffix comparison and `ahow` paradigms. Correct singular and 1st-plural `aqaanaa` forms to `aqaan`, illustrate the past as `aqaan → iqiin`, and replace root `aal` with `ool`; link chapter 8 for the full irregular paradigms. | `repository-supported`; `paradigm-correction`; `root-form-correction` |
| N07-R009 | 278-317 | Important two-group auxiliary account; the `jir` example is habitual past but is labeled past progressive, two future auxiliaries lack final `-a`, and plural prose uses singular possessive agreement / high | This file's own aspect table and chapter 8 distinguish `cunayay` (past progressive) from `cuni jiray` (habitual past); `resources/naxwe/06-sarfaha-iskuxireyaasha.md` has `Waa laga hadli doonaa`; the future paradigm has 3rd-singular `doonaa`; `resources/sarfe/03-dhismaha-ereyga.md` repeats the topic but does not resolve the contradictory label | Retain both auxiliary groups, all listed verbs, and examples 7-8. Relabel `jir` here as forming habitual past, correct `doona` to `doonaa` in both displayed sentences, and repair `ma lumiyaan micnihiisa` to plural agreement. Do not import new auxiliary examples. | `repository-supported`; `classification-correction`; `form-correction`; `grammar-correction` |
| N07-R010 | 318-344 | Coherent transitive/intransitive account with six useful examples; semantic lists are tendencies, not exhaustive class definitions / medium | Dictionary entries support the displayed verbs and valencies; chapters 6 and 9 independently distinguish direct objects from preposition-marked complements; dictionary `ilmo¹` permits a collective child/children reading | Retain all six examples, including `Ilmuhu ... way gubeen`, and the `u` contrast. Mark the semantic groupings as frequent patterns rather than categorical membership; link the detailed complement and sentence accounts. | `repository-supported`; `interpretive-qualification`; `intentional-retained` |
| N07-R011 | 345-383 | Causative `-in/-i` treatment, examples, allomorphy, and five-row derivation table are internally coherent; `kariyay` and table `kariyey` are supported variants and should not be silently collapsed / medium | Dictionary `kari` records past `-iyay/-isay`; `resources/sarfe/03-dhismaha-ereyga.md` supports `-in`, all five derivation rows, and the same distribution summary | Retain examples 7-8, the complete allomorph account, `kariyay/karisay`, and every derivation row. Clarify that the displayed tense/mood distribution belongs to this source analysis and preserve the attested `-ay/-ey` spelling variants. | `repository-supported`; `intentional-retained`; `form-clarification` |
| N07-R012 | 384-399 | Middle/reflexive `-an` treatment, six paired examples, and three surface forms are supported as the source's derivational analysis / medium | `resources/sarfe/03-dhismaha-ereyga.md` independently records `-an/-o/-at`; dictionary entries support `dhaqan`, `beeran`, and the `gub/gubo` alternation, though their lexical meanings extend beyond the glosses here | Retain all six numbered forms and the `-an/-o/-at` distribution. Present the self-directed readings as readings of these examples, not exhaustive dictionary definitions; add no reconstructed forms. | `repository-supported`; `interpretive-qualification`; `intentional-retained` |
| N07-R013 | 400-416 | Compact inventory of six further extensions; the `-oob` and `-ood` descriptions reverse the derivational direction by saying a verb makes a noun, while all examples show nouns making verbs / high | `resources/sarfe/03-dhismaha-ereyga.md` gives `biyo → biyoob` and `dhaxan → dhaxmood`; dictionary entries support `fur/furmay`, `caro/carood`, `dheer`, `biyo`, and `oday` | Retain all six suffixes and every example. Correct the two descriptions to say that the suffix makes a verb from a noun; retain `yeele dahsoon` as the source label alongside a clarifying description of the patient becoming subject. | `repository-supported`; `logic-correction`; `terminology-clarification` |
| N07-R014 | 417-438 | Three-way deverbal-noun account and examples 87-89 are useful; the opening reverses noun/verb direction, the last sentence is grammatically incomplete, and `abuurto, karise, barate` do not establish the claimed single feminine `-te` rule / high | `resources/sarfe/03-dhismaha-ereyga.md` supports the masculine zero nouns, feminine action nouns, and masculine `-e/-ye` agents; dictionary supports `qosol/qoslid`, `karin/karis`, `abuure`, but repository searches find no independent entries for `abuurto`, `karise`, or `barate` | Retain examples 87-89, all three noun categories, suffix sets, genders, and masculine agent examples. Reverse the opening to describe nouns derived from verbs and repair the final grammar. Preserve the three feminine forms only as unresolved source forms, explicitly avoiding a general paradigm or guessed replacements until external/source verification exists. | `repository-supported` in part; `logic-correction`; `grammar-correction`; `unresolved`; `intentional-retained` |
| N07-R015 | whole file | The chapter has 32 surviving numbered examples across several source series, including gaps and resets, and currently has no navigation links / medium | Extant numbers are 1; 7-8; 19-22; 25-27; 30-32; a reset to 7-8; a reset to 1-8; 29-34; and 87-89; chapters 6, 8, 9, and 12 plus morphology chapters 2-3 provide maintained detail | Preserve every number, reset, and example exactly except the separately approved form corrections inside examples. Add a short source-numbering note and resolving links; never fill missing numbers, silently renumber, or create absent examples. | `structural-only`; `unresolved`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned chapter should retain its present title and complete sequence:

1. the verb as predicate and the `sal`/`nadooc` division;
2. person, number, and gender inflection;
3. present, past, and future tense;
4. simple, progressive, and habitual aspect;
5. indicative, conditional, imperative, advisory, and dependent moods;
6. the three suffix-conjugation classes;
7. stative/adjectival and prefix-conjugating verbs;
8. both auxiliary groups;
9. transitive and intransitive valency;
10. causative `-in`, middle/reflexive `-an`, and the smaller extensions; and
11. root, action, and agent nouns derived from verbs.

All 32 numbered examples should remain with their original source numbers,
including every gap and reset. All eleven tables should remain. Cleanup should
change only the uniquely supported paradigm/form cells identified above,
qualify categorical prose without discarding its topic coverage, and mark the
three feminine agent forms unresolved. Recommended navigation targets are
`06-sarfaha-iskuxireyaasha.md`, `08-hogatuska-baradigmaha-falalka.md`,
`09-weer-fudud.md`, `12-noocyada-weeraha.md`,
`../sarfe/02-falalka.md`, and `../sarfe/03-dhismaha-ereyga.md`.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-12 with the
  instruction, "go ahead."
- **Approved finding IDs:** N07-R001 through N07-R015
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes
- **Decision requested:** review and approve the cleaned resource before the
  cleanup-approval or complete stages are marked.

## Audit validation

- Resource SHA-256 after audit:
  `417566d983d4b9e6867ae212f3e7930f36ade5023d36adee2eb4c192cb5fa957`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, three H2, nine H3, eight H4, eleven Markdown
  tables, 66 data rows.
- Numbered content checked: 32 examples/forms on 29 entry lines; all source
  gaps and resets recorded.
- Existing local links: none; proposed targets all exist.
- Unsupported forms left unresolved: `abuurto`, `karise`, `barate`.

## Cleanup result and review

### Applied cleanup

- Documented the source-number gaps and resets without filling or renumbering
  them.
- Corrected `gal + tay → qashay` to `gal + tay → gashay`, advisory
  `aad qorteen` to `aad qortaan`, and conditional `cunlahaa` to
  `cuni lahaa`.
- Scoped the present-tense definition, corrected the contradictory
  present-aspect labels to `socota` and `caadaley`, and removed the claim that
  simple past necessarily describes one instantaneous occurrence.
- Corrected the five overextended `aqaanaa` cells to `aqaan`, restored the
  first-person past prefix in `iqiin`, and distinguished root `ool` from its
  first-person present form `aal`.
- Corrected the auxiliary account from past progressive to habitual past,
  repaired both future forms to `doonaa`, and repaired plural agreement in
  the group-B explanation.
- Qualified the stative, valency, causative-allomorph, and middle/reflexive
  descriptions without removing their classifications, lists, or examples.
- Corrected the noun-to-verb direction of `-oob` and `-ood`, corrected the
  opening direction of the deverbal-noun section, and clarified the
  unaccusative description.
- Retained `abuurto`, `karise`, and `barate` as explicitly unresolved source
  forms instead of deriving a general feminine-agent rule or guessing
  replacements.
- Added eight resolving link occurrences to six maintained verb, sentence,
  preposition, and morphology resources.

### Deliberately retained

- All 32 numbered examples/forms in their original wording and with every
  original source number, gap, and reset.
- All eleven tables and every original table cell except the seven approved
  paradigm/form corrections.
- The complete treatment of inflection, tense, aspect, mood, conjugation
  classes, stative and prefix verbs, auxiliaries, valency, verbal extensions,
  and deverbal nouns.
- Source-supported spelling variants `kariyay` and `kariyey`, all lexical
  lists, and the three unresolved feminine agent forms.

### Cleanup validation

- `git diff --check`: passed.
- Original/current numbered-entry comparison: no differences across all 29
  entry lines containing 32 numbered examples/forms.
- Original/current table comparison: exactly seven approved cell changes —
  `qorteen` → `qortaan`, `cunlahaa` → `cuni lahaa`, and five `aqaanaa` →
  `aqaan` cells.
- Structure retained: one H1, three H2, nine H3, eight H4, eleven Markdown
  tables with 66 data rows.
- Local links: 8 occurrences checked across 6 unique targets; no missing files
  or requested heading anchors.
- Deprecated-error search: no remaining `qashay`, `qorteen`, `cunlahaa`,
  `aqaanaa`, root-list `aal`, malformed future `doona`, reversed
  `fal ka dhigaa magac`, or `laakiin ka yaraan` occurrence.
- Provenance TSV validation: every row has 10 tab-separated fields.
- Cleaned file size: 474 lines; 2,853 words; 18,011 bytes.
- Cleaned SHA-256:
  `c84c7f0b1d89b74686988fdb5530b347b1bb34d486c16fda11a88f806d75aad5`.

## Primary-PDF structural amendment — 2026-08-12

The source contents on PDF page 12 confirms the heading `7.3.2.3 Lifaaqyo
kale`. The cleaned resource already retained its six suffix descriptions, so
the numbered heading was restored without changing that approved content.
