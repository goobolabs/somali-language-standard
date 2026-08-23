# Audit record — Sarfaha magacyada

- **Resource path:** `resources/naxwe/02-sarfaha-magacyada.md`
- **Collection / family:** naxwe / primary grammar
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-12
- **File size at audit start:** 344 lines; 1,813 words; 11,358 bytes
- **Resource SHA-256 at audit start:** `a9b96567872468cd5dc641da8495a4d99b24b8deb6728d2ca9337a001a7ed3cf`
- **Resource-text changes during audit:** none

## Target output model

This file is already a coherent primary-grammar chapter rather than an OCR
transcript. It has one H1, three H2 sections, six H3 subsections, two H4
subsections, twelve valid two-column tables, one valid three-column table, one
classification tree, and 14 numbered examples. It contains no front matter,
exercises, scan debris, false headings, or damaged reading order.

Cleanup should preserve the chapter's full treatment of noun meaning and form,
grammatical gender, codkac, number, nouns without ordinary plurals, gender
polarity, and masculine and feminine plural classes. It should correct the one
uniquely supported textual defect, distinguish useful diagnostics from
exceptionless definitions, scope source generalizations, identify source-only
forms and accent rows, and add links to the maintained morphology, determiner,
phonology, and alphabet resources.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-19 | reviewed | N02-R001 |
| 20-44 | reviewed | N02-R002 |
| 45-61 | reviewed | N02-R003 |
| 62-108 | reviewed | N02-R004 |
| 109-161 | reviewed | N02-R005 |
| 162-178 | reviewed | N02-R006 |
| 179-207 | reviewed | N02-R007 |
| 208-224 | reviewed | N02-R008 |
| 225-239 | reviewed | N02-R009 |
| 240-283 | reviewed | N02-R010 |
| 284-300 | reviewed | N02-R011 |
| 301-313 | reviewed | N02-R012 |
| 314-344 | reviewed | N02-R013 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| N02-R001 | 1-19 | Clean title, word-class introduction, and seven noun examples; `dir` is the source's term for word class and no OCR damage is present / low | `resources/naxwe/ereyfur.md`; same-file semantic and formal diagnostics; `resources/naxwe/13-aasaaska-naxwaha.md` | Retain the title, source term, all examples, and the meaning/form distinction. Add navigation to the word-class overview without replacing the source terminology. | `intentional-retained`; `structural-only` |
| N02-R002 | 20-44 | Coherent semantic classification into proper/common and concrete/abstract nouns; `caadyaal` and `cillanaad` are repository-supported, while the feature-based definition of `nin` is the source's semantic model / low | `resources/naxwe/ereyfur.md` entries `caadyaal`, `cillanaad`, `magac caadyaal`, and `magac cillanaad`; `resources/qaamuus/11-c.md`; `resources/naxwe/00-luqadda-iyo-fekerka.md` | Retain the classification, examples, and source analysis. Present the feature description as an explanatory model rather than a complete SLS definition of every noun. | `repository-supported`; `intentional-retained` |
| N02-R003 | 45-61 | Useful formal diagnostics, but the conclusion treats plural formation and determiner attachment as a complete noun definition even though the next section documents nouns without ordinary plurals; `markaantaana` is an isolated malformed form / high | Same file lines 179-193; repository-wide search finds `markaantaana` only here and repeatedly supports the construction `isla markaasna`; `resources/naxwe/03-sarfaha-tifaftireyaasha.md` | Retain both diagnostics and examples. Correct `markaantaana` to `markaasna`. State that these are common or sufficient diagnostics rather than requirements every noun must satisfy; link determiner details to chapter 3. | `repository-supported` |
| N02-R004 | 62-108 | Clear agreement demonstration and valid table; the prose equates natural sex and grammatical gender for animate nouns more categorically than agreement evidence warrants, and the abbreviated `-ku/-gu`, `-tu/-du` lists are examples rather than the full article allomorph system / medium | `resources/sarfe/01-magacyada.md`; `resources/naxwe/03-sarfaha-tifaftireyaasha.md` §§3.1-3.1.1; same-file examples 8-11 | Retain the table and all four agreement examples. Explain that natural sex often guides gender for relevant animate referents, while grammatical gender is established by agreement. Label the article forms as examples and route the complete allomorph inventory to chapter 3. | `repository-supported`; `intentional-retained` |
| N02-R005 | 109-161 | Valuable introductory codkac material and three valid tables, but masculine/feminine placement and `-e/-o` endings are expressed too generally; many accented pairs occur only here, while `golo` and `gato` have no independent lexical match and must not be silently mapped to `gole` or another form / high | `resources/naxwe/01-ereyada.md` codkac scope note; `resources/sarfe/01-magacyada.md`; `resources/dhawaaq/05-codadka-sare.md`; dictionary supports several unaccented headwords and `ardo` as a form of `arday`, but repository-wide searches find `golo` and nominal `gato` only here | Retain all tables as the source's introductory inventory. Scope the placement and ending observations to the nouns/classes represented. Mark source-only accented rows, `golo`, and `gato` explicitly unresolved; do not alter accents or infer lexical replacements. Link detailed codkac treatment. | `repository-supported` in part; `unresolved`; `intentional-retained` |
| N02-R006 | 162-178 | Clean number introduction and valid table; the opening states that a noun has singular and plural forms before the file immediately identifies nouns without ordinary plurals / medium | Same file lines 179-193; `resources/sarfe/01-magacyada.md`; `resources/naxwe/ereyfur.md` supports `sal` and `dhammaad` terminology | Retain the table, stem/ending distinction, and examples. Limit the opening statement to count nouns or nouns that participate in the singular/plural contrast. | `repository-supported`; `structural-only` |
| N02-R007 | 179-207 | Coherent non-plural-noun discussion and intact classification tree; mass, collective, and abstract classes and examples are repository-supported, though countability can depend on sense and usage / low | `resources/sarfe/01-magacyada.md` §Magacyada aan wadar yeelan; dictionary entries support the listed headwords, including `oon`; `resources/naxwe/13-aasaaska-naxwaha.md` | Retain the three classes, all examples, and the tree. Frame the classification as the chapter's ordinary-use analysis rather than an absolute ban on every possible counted use. | `repository-supported`; `intentional-retained` |
| N02-R008 | 208-224 | Clear gender-polarity examples; the basic lab→dheddig, dheddig→lab, and monosyllabic lab→lab patterns are independently summarized, but “all other nouns” is stronger than a compact class overview needs / medium | `resources/sarfe/01-magacyada.md` §Isbeddelka caynta; same-file plural classes; `resources/naxwe/03-sarfaha-tifaftireyaasha.md` §Qodobka iyo tirada | Retain examples 12-14 and the core pattern. Present it as the plural-class system described in this chapter and link the morphology summary; avoid asserting that no lexical or class exception can exist. | `repository-supported`; `intentional-retained` |
| N02-R009 | 225-239 | Clean first masculine plural class with a valid table; all four forms are repository-supported / low | `resources/sarfe/01-magacyada.md` group L1; `resources/naxwe/13-aasaaska-naxwaha.md`; dictionary and wordlists | Retain the subsection, rule, and complete table. Add no new paradigm. | `repository-supported` |
| N02-R010 | 240-283 | Readable `-o` subclasses and four valid tables/examples; most alternations are supported, but precise ending lists should be presented as class patterns, and `qamuun/qamuunno` has no independent match—only dictionary `kamuun`, whose recorded gender does not uniquely support substitution / high | `resources/sarfe/01-magacyada.md` groups L2a-L2d; `resources/sarfe/04-isbeddelka-codka.md`; dictionary and wordlists support `waraf`, `jilib`, `garab`, `ilig`, and other rows; repository-wide search finds `qamuun(ka)/qamuunno(da)` only here | Retain the class divisions and all secure rows. Label `qamuun/qamuunno` unresolved and do not change it to `kamuun/kamuunno` by inference. Describe the ending inventories as patterns for the listed classes rather than productive rules for every noun with those letters. | `repository-supported` in part; `unresolved` |
| N02-R011 | 284-300 | Clean `-yaal`, `-aan`, and Arabic-pattern plural material; `tukayaal`, `dhagxaan`, `ugxaan`, `macallimiin`, `duruus`, and `awliyo` are independently supported / low | `resources/sarfe/01-magacyada.md` groups L3-L5; `resources/qaamuus/06-d.md` entries `dersi/duruus`; `resources/qaamuus/19-w.md` entry `weli`; `resources/qaamuus/22-a.md` entry `awliyo`; wordlists | Retain the tables and examples. Link the morphology summary and preserve `qaabka af-carbeedka` as a source classification rather than expanding its etymology. | `repository-supported`; `intentional-retained` |
| N02-R012 | 301-313 | Useful codkac-only plural class and valid table; `áwr`, `díbi/dibí`, and `mádax/madáx` are independently supported, while the exact accented `Cárab/Caráb` and `órgi/orgí` rows occur only here / medium | `resources/sarfe/04-isbeddelka-codka.md` supports three rows; `resources/sarfe/01-magacyada.md` group L6; repository-wide accented-form search | Retain the table. Mark `Cárab/Caráb` and `órgi/orgí` as source-only accent rows; do not alter their accent placement. Link detailed codkac resources. | `repository-supported`; `unresolved` for two rows |
| N02-R013 | 314-344 | Clean feminine plural section with one valid three-column and one valid two-column table; major `-o`, vowel-loss, `-y-`, and `-oyin` patterns are independently supported, while some exact lexical plurals remain source-specific / low | `resources/sarfe/01-magacyada.md`; `resources/sarfe/04-isbeddelka-codka.md`; dictionary supports `tummaati` and the other principal headwords | Retain both tables, comments, and all forms. Present them as the chapter's feminine plural classes, link the morphology summary, and do not normalize the source-specific spellings or apostrophe form by inference. | `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned chapter should remain titled **2. Sarfaha magacyada** and retain
its current sequence:

1. noun identification through meaning and common formal diagnostics;
2. proper/common and concrete/abstract noun classes;
3. grammatical gender established through agreement, with natural sex treated
   as a semantic guide rather than an exceptionless identity;
4. codkac and gender, with source-only rows visibly labeled;
5. singular/plural contrasts and nouns without ordinary plurals;
6. the existing classification tree and gender-polarity examples;
7. masculine plural classes L1-L6; and
8. feminine plural classes and their morphophonological changes.

The cleanup should add concise links to `01-ereyada.md`,
`03-sarfaha-tifaftireyaasha.md`, `sarfe/01-magacyada.md`,
`sarfe/04-isbeddelka-codka.md`, and the codkac resource. It must not add
new noun or sentence examples, reconstruct unattested accents, replace
`qamuun`, `golo`, or `gato` by analogy, or convert compact class descriptions
into universal rules.

All 14 numbered examples, the classification tree, and all 13 tables should be
retained unless an approved finding explicitly requires a scope note or textual
correction.

## Audit approval

- **Audit approval:** approved by the maintainer on 2026-08-12 with the
  instruction, "go ahead."
- **Approved finding IDs:** N02-R001 through N02-R013
- **Deferred by default:** `golo`, `gato`, `qamuun/qamuunno`, source-only
  accented rows, and source-specific lexical plurals remain unresolved;
  approval does not authorize inferred replacements.

## Cleanup result and review

- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

### Applied cleanup

- Corrected the uniquely supported malformed form `markaantaana` to
  `markaasna` while retaining the noun diagnostics as non-universal tests.
- Scoped the feature analysis, natural-sex discussion, codkac observations,
  countability model, gender-polarity pattern, and plural ending lists to the
  source model or represented classes.
- Identified `golo`, `gato`, `qamuun/qamuunno`, `Cárab/Caráb`, and
  `órgi/orgí` as unresolved source forms or source-only accent rows; none was
  silently normalized or reconstructed.
- Added ten resolving local links to the maintained word-class, determiner,
  noun-morphology, sound-change, alphabet, and codkac resources.

### Deliberately retained

- All 14 numbered examples.
- All rows of the twelve two-column tables and the one three-column table.
- The complete fenced classification tree.
- The chapter sequence, all source classifications, and every unresolved
  lexical form.

### Validation

- `git diff --check`: passed.
- Original/current numbered-example comparison: no differences.
- Original/current Markdown-table-row comparison: no differences.
- Original/current fenced-tree comparison: no differences.
- Structure: one H1, three H2, six H3, two H4; 14 numbered examples; twelve
  two-column tables; one three-column table; two fence lines.
- Local-link resolution: 10 checked, 0 missing.
- Provenance TSV validation: every row has 10 tab-separated fields.
- Malformed form: 0 occurrences of `markaantaana`; corrected form occurs once.
- Cleaned file size: 386 lines; 2,030 words; 13,517 bytes.
- Cleaned SHA-256:
  `8767d56dfea7f099f8377eff2df88c94fda299bb27707df9303b89abfaef3d61`.
