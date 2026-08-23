# Audit record — Hogatuska baradigmaha falalka

- **Resource path:** `resources/naxwe/08-hogatuska-baradigmaha-falalka.md`
- **Collection / family:** naxwe / primary grammar
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; conservative SLS cleanup applied and awaiting review
- **Audit started:** 2026-08-12
- **File size at audit start:** 127 lines; 920 words; 4,888 bytes
- **Resource SHA-256 at audit start:**
  `ace12dd10d36b338211d9ab294977b98ebc70c2733582f5cb6cda68ae46ecc72`
- **Resource-text changes during audit:** none

## Target output model

This file is a compact paradigm supplement rather than an OCR transcript. It
has one H1, four H2 sections, three H3 subsections, six valid Markdown tables
with 36 data rows, no numbered examples, and no links. It contains no front
matter, exercises, scan debris, malformed tables, or broken reading order.

Cleanup should preserve all six tables and the full coverage of regular
negative forms, stative verbs, five prefix-conjugating verbs, past forms, and
advisory/imperative negation. It should narrow an opening that promises every
mood although the chapter presents selected negative and irregular paradigms;
distinguish a complete negative imperative from its bare verb form; separate
the particles for imperative, advisory, and dependent negation; align six
spaced `adag` affirmative cells with the maintained joined paradigm; restore
the missing `ma` in the stative future; correct present `ma aqoon` to
`ma aqaan`; acknowledge `ma ahayn` as an exception to the claimed `-in`
ending; and replace the oversimplified final table with a source-supported
summary that keeps imperative and advisory negation distinct.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-7 | reviewed | N08-R001 |
| 9-20 | reviewed | N08-R002 |
| 21-48 | reviewed | N08-R003 |
| 50-66 | reviewed | N08-R004 |
| 68-89 | reviewed | N08-R005 |
| 91-105 | reviewed | N08-R006 |
| 107-113 | reviewed | N08-R007 |
| 115-127 | reviewed | N08-R008 |
| whole file | reviewed | N08-R009 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| N08-R001 | 1-7 | Supported title concept, but body spelling `baaradigmayaasha` differs from glossary `baradigme`, and the opening promises all conjugation classes “hab kasta” although the file only presents selected negative and irregular paradigms / medium | Dictionary `hogatus¹` supports the title word; `resources/naxwe/ereyfur.md` records `baradigme`, `baradigme fidsan`, and `baradigme kooban`; the file itself contains no complete mood-by-class paradigm | Retain the title and purpose as a paradigm guide. Normalize the body spelling to `baradigmayaasha` and scope the promise to the selected negative, stative, and prefix-conjugating paradigms actually present; do not add missing moods or inferred tables. | `repository-supported`; `terminology-alignment`; `scope-correction` |
| N08-R002 | 9-20 | The five class-specific negative verb forms are useful, but prose says the negative imperative “comes before” a suffix and the table omits required particle `ha`, so its entries are not complete negative imperatives / high | `resources/naxwe/12-noocyada-weeraha.md` gives complete `Waxba ha keenin/keenina`; chapter 17 retains `ha sheegin` and `ha seexan`; `resources/sarfe/02-falalka.md` supports the five bare forms but mirrors this compact table | Retain all five class rows and label the third column as the negative verb form. Explain that a complete negative imperative places `ha` before that form and plural may add final `-a`; link chapter 12. Do not manufacture full person paradigms for every row. | `repository-supported`; `classification-correction`; `form-clarification`; `intentional-retained` |
| N08-R003 | 21-48 | The `cun` table and dependent/advisory examples are valuable, but the particle explanation conflates advisory `yaan` with imperative `ha…n`, and the claim that all `-in/-n` negatives are person-invariant is broader than the stative and irregular evidence / high | Chapter 12 distinguishes declarative `ma`, imperative `ha`, advisory `yaan/yuusan/yaanay`, and dependent forms such as `uusan`; it limits person-neutral `-in/-o` to the relevant negative verbal paradigm; this file's next table is person-sensitive | Retain all seven `cun` rows, `ma cunayn(in)`, and both transformation examples. Separate the four negative environments and scope person-neutrality to the displayed regular `cun` forms rather than every form containing `-in/-n`. Preserve `ma cunayn(in)` as source notation because the repository does not uniquely select one expansion. | `repository-supported`; `classification-correction`; `scope-correction`; `unresolved`; `intentional-retained` |
| N08-R004 | 50-66 | Heading prose names both `adag` and `fiican` but gives only an `adag` table; six affirmative cells split forms that chapters 7 and morphology join; final future “negative” lacks `ma`; saying all later forms simply take `-ayn` misdescribes auxiliary constructions / high | `resources/naxwe/07-sarfaha-falalka.md` and `resources/sarfe/02-falalka.md` give joined `adagtahay, adagyahay, adagnahay, adagtihiin, adagyihiin`; the displayed negative cells occur only here; the regular future negative pattern in both chapters includes `ma ... doono` | Describe the table as `adag` only. Join the six affirmative cells to the maintained paradigm, retain all seven negative cells as this source's gathered paradigm, and correct the future to `ma adkaan doono`. Explain that `ma adkayn` contains `-ayn`, while habitual/future examples negate their auxiliary; do not invent a missing `fiican` table. | `repository-supported` in part; `paradigm-alignment`; `form-correction`; `classification-correction`; `intentional-retained` |
| N08-R005 | 68-89 | Complete 35-cell present paradigm is coherent; introduction has singular agreement for plural `qaarkood`; present `aqaan → ma aqoon` incorrectly substitutes a past negative, while `ma yaalo` lacks enough independent evidence for silent normalization / high | Cleaned chapter 7 and `resources/sarfe/02-falalka.md` support all 35 affirmative cells and roots `aqoon, ool, dheh/iri, imow, ahow`; repository prose repeatedly has present `ma aqaan/ma yaqaan`, while narrative `ma aqoon` is past; repository spelling varies between `yaalo` and `yaallo` without a uniquely maintained negative entry | Retain the complete table and all other negative examples. Repair agreement, correct only `aqaan → ma aqoon` to `aqaan → ma aqaan`, and retain `ma yaalo` explicitly as a source spelling rather than guessing `ma yaallo`; link chapter 7 and compact morphology. | `repository-supported`; `grammar-correction`; `tense-correction`; `unresolved`; `intentional-retained` |
| N08-R006 | 91-105 | Complete 35-cell past paradigm supplies important irregular data; prose says every negative ends in `-in` but its own final example `ma ahayn` does not, and says `ma` follows rather than precedes / high | Dictionary `aqoo` supports `yiqiin/tiqiin`; dictionary `al²` supports `yiil/tiil`; dictionary `dheh` supports `yiri/tiri`; dictionary `imow` supports `yimid/timid`; chapters 7 and 13 support the surrounding past series; `ma ahayn` is independently widespread | Retain every past-table cell, including parenthetical `iillay/tiillay/niillay` and `iri/idhi` variants. State that the first four displayed negative forms use `-in/-n`, while `ahow` has exceptional `ma ahayn`, and correct the particle order. Treat rare `ma aqoonin`, `ma oranin`, and `ma imanin` as source forms rather than silently replacing them with corpus variants. | `repository-supported` in part; `logic-correction`; `grammar-correction`; `intentional-retained` |
| N08-R007 | 107-113 | Useful positive and negative advisory forms for all five irregular verbs, but the prose describes only third-person `ha` forms and does not constitute the full paradigm promised at the opening / medium | Chapter 12 gives the seven-person advisory pattern and negative `yuusan/yaanay`; chapter 7 supplies the general advisory table; repository evidence supports `yaqaanno` and `yaallo`, while the remaining displayed forms follow the source's irregular series | Retain all ten forms exactly. Identify them as selected third-person-positive and mixed-person-negative examples, link the full advisory paradigm, and do not infer the missing person cells. | `repository-supported`; `scope-correction`; `intentional-retained` |
| N08-R008 | 115-127 | Three-row conclusion conflates imperative `ha` with advisory `yaan`, reduces dependent negation to `aan…n`, and gives one suffix rule for regular, stative, and irregular verbs; final sentence is repetitive and says irregular roots cannot change although the chapter displays alternation / high | Chapter 12 distinguishes declarative, imperative, advisory, and dependent environments; chapters 7-8 display prefix and vowel alternation; glossary supports `qurub diidmo`; same-file `ma ahayn` and stative table disprove a universal suffix summary | Retain a compact conclusion but separate `Ebyoon`, `Amar`, `Talo`, and `Dhimman` rows, describe endings as paradigm-dependent, and point to the examples above. Recast the final sentence to say prefix verbs follow special paradigms rather than claiming their stems do not change. | `repository-supported`; `classification-correction`; `logic-correction`; `scope-correction` |
| N08-R009 | whole file | No scan debris or numbered-source issue; the six dense tables currently have no links to their prerequisite or sentence-level explanations / low | Chapters 7 and 12 provide the maintained positive paradigms and sentence environments; `resources/sarfe/02-falalka.md` provides the compact cross-reference; `ereyfur.md` provides canonical terminology | Preserve the complete structure and every table. Add resolving links to chapters 7 and 12, compact verb morphology, and the glossary where terminology is introduced; add no examples beyond the approved clarifications. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned chapter should retain its title and present compact sequence:

1. a scoped explanation of what the paradigm guide covers;
2. the five conjugation-class forms used in negative imperatives;
3. the seven-row `cun` tense/aspect/mood negative comparison;
4. dependent and advisory negative formation;
5. the seven-person stative `adag` paradigm;
6. present and past paradigms for `aqoon, ool, dheh/iri, imow, ahow`;
7. selected advisory forms for those irregular verbs; and
8. a corrected conclusion separating declarative, imperative, advisory, and
   dependent negation.

All six tables and all 36 data rows should remain. Cleanup should change only
the six affirmative `adag` spacing cells, `ma aqoon` → `ma aqaan`, the missing
`ma` in `ma adkaan doono`, and descriptive/header cells required to distinguish
bare negative verb forms and the four negative environments. Rare forms and
variants should remain unless the audit identifies a unique correction.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-12 with the
  instruction, "go ahead."
- **Approved finding IDs:** N08-R001 through N08-R009
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no
- **Decision requested:** review and approve the cleaned resource before the
  cleanup-approval or complete stages are marked.

## Audit validation

- Resource SHA-256 after audit:
  `ace12dd10d36b338211d9ab294977b98ebc70c2733582f5cb6cda68ae46ecc72`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, four H2, three H3, six Markdown tables with 36
  data rows.
- Numbered content: none in this resource.
- Existing local links: none; all proposed targets exist.
- Source forms deliberately unresolved: `ma cunayn(in)`, `ma yaalo`,
  `ma aqoonin`, `ma oranin`, and `ma imanin`.

## Cleanup result and review

### Applied cleanup

- Scoped the opening to the selected negative, stative, and irregular
  paradigms actually present and aligned body terminology with glossary
  `baradigme`.
- Relabeled the five class-specific entries as negative verb forms, explained
  complete imperative `ha cunin/ha cunina`, and linked the sentence-level
  imperative account.
- Separated declarative `ma`, imperative `ha`, advisory
  `yaan/yuusan/yaanay`, and dependent `uusan` environments; limited the
  person-neutrality statement to displayed past `ma cunin`.
- Retained `ma cunayn(in)` with an explicit unresolved-source note instead of
  selecting an expansion without repository evidence.
- Joined six affirmative stative cells to `adagtahay, adagyahay,
  adagnahay, adagtihiin, adagyihiin`, retained all seven negative cells, and
  restored `ma` in `ma adkaan doono`.
- Repaired plural agreement in the prefix-verb introduction and corrected
  present `ma aqoon` to `ma aqaan` while retaining `ma yaalo` as an explicitly
  unresolved source spelling.
- Preserved all 35 present and all 35 past irregular-paradigm cells; corrected
  the claim that every past negative ends in `-in` by identifying exceptional
  `ma ahayn` and corrected the position of `ma`.
- Scoped the ten advisory forms as selected examples and linked the full
  person paradigm.
- Rebuilt the concluding summary so `Amar` and `Talo` have separate rows and
  endings are described as paradigm-dependent rather than universal.
- Added six resolving link occurrences to four maintained grammar,
  morphology, and glossary targets.

### Deliberately retained

- All five negative-form rows, all seven `cun` rows, all seven stative
  negative cells, and every one of the 70 present/past irregular cells.
- All 33 non-summary paradigm data rows; only the six approved affirmative
  stative cells changed internally. The misleading three-row conclusion was
  replaced by the approved four-row environment summary.
- Parenthetical `ma cunayn(in)`, `iillay/tiillay/niillay`, and `iri/idhi`
  variants, plus rare `ma aqoonin`, `ma oranin`, and `ma imanin`.
- Source spelling `ma yaalo`, accompanied by its unresolved note, and all ten
  advisory forms.

### Cleanup validation

- `git diff --check`: passed.
- Structure retained: one H1, four H2, three H3, and six Markdown tables.
- Table preservation: all 33 original non-summary data rows remain; the
  conclusion now has four rows, for 37 current data rows total.
- Original/current paradigm comparison: six approved affirmative `adag`
  cells changed; all other non-summary table cells are unchanged.
- Required inline corrections verified: `ma aqaan` and
  `ma adkaan doono` are present; erroneous present `ma aqoon` and future
  `adkaan doono` without `ma` are absent.
- Unresolved/source-form preservation verified for `ma cunayn(in)`,
  `ma yaalo`, `ma aqoonin`, `ma oranin`, and `ma imanin`.
- Local links: 6 occurrences checked across 4 unique targets; no missing files
  or requested heading anchors.
- Numbered content: none in the original or cleaned resource.
- Provenance TSV validation: every row has 10 tab-separated fields.
- Post-cleanup navigation follow-up: the glossary link now targets the approved
  bilingual `ereyfur.md`; no paradigm or explanatory content changed.
- Cleaned file size: 164 lines; 1,102 words; 6,498 bytes.
- Cleaned SHA-256:
  `971c1d926f48211062a58d85266d27ef5e8aefad2826e97d512117446d9ebce7`.
