# Audit record — Sarfaha tifaftireyaasha

- **Resource path:** `resources/naxwe/03-sarfaha-tifaftireyaasha.md`
- **Collection / family:** naxwe / primary grammar
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; conservative SLS cleanup applied and awaiting review
- **Audit started:** 2026-08-12
- **File size at audit start:** 216 lines; 1,260 words; 8,148 bytes
- **Resource SHA-256 at audit start:**
  `686709b107dd27e4c41e710b676942d6e079718685ad1bb36d6879cfff455e84`
- **Resource-text changes during audit:** none

## Target output model

This file is already a compact primary-grammar chapter rather than an OCR
transcript. It has one H1, four H2 sections, five H3 subsections, two valid
two-column tables, two valid three-column tables, and 22 numbered example
lines. It has no front matter, exercises, scan furniture, false headings,
fenced debris, or damaged reading order.

Cleanup should retain the full treatment of definite articles and their
allomorphs, demonstratives, interrogative determiners, possessive paradigms,
possessive gender, and possessive structure. It should scope categorical
claims, distinguish repository-supported demonstratives from independently
classified indefinite pronouns, label two unmatched forms without inferring
repairs, and add navigation to the noun, pronoun, noun-phrase, question, and
morphophonology resources.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-10 | reviewed | N03-R001 |
| 11-32 | reviewed | N03-R002 |
| 34-56 | reviewed | N03-R003 |
| 58-75 | reviewed | N03-R004 |
| 77-104 | reviewed | N03-R005 |
| 106-119 | reviewed | N03-R006 |
| 121-153 | reviewed | N03-R007 |
| 155-181 | reviewed | N03-R008 |
| 183-191 | reviewed | N03-R009 |
| 193-216 | reviewed | N03-R010 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| N03-R001 | 1-10 | Clean title and four-part determiner taxonomy; no navigation is present / low | `resources/naxwe/ereyfur.md` supports `tifaftire`, `qodob`, and `lahaansho`; `resources/naxwe/04-sarfaha-magacuyaallada.md`; `resources/naxwe/12-noocyada-weeraha.md` | Retain the title, taxonomy, and terminology. Add concise links to the detailed pronoun and question chapters without adding another category. | `repository-supported`; `structural-only` |
| N03-R002 | 11-32 | Clear definiteness and gender-agreement examples, but shared knowledge is presented as the whole meaning of definiteness, `-ka/-ta` are described as invariant base forms, and the ban on articles with all proper nouns is categorical / medium | `resources/sarfe/01-magacyada.md` supports the gender contrast; `resources/naxwe/02-sarfaha-magacyada.md` and the next subsection show allomorphy; the repository does not establish an exceptionless proper-noun ban | Retain all four example lines and the common/shared-knowledge explanation. Identify `-ka/-ta` as underlying or citation forms whose allomorphs follow, and frame the proper-noun statement as the ordinary pattern in this chapter rather than a universal prohibition. | `repository-supported` in part; `intentional-retained`; `unresolved` as an absolute claim |
| N03-R003 | 34-56 | Coherent masculine and feminine article-allomorph inventory with complete examples; the letter-ending lists function as the source's class summary rather than an independently exhaustive specification / medium | Dictionary entries independently support the gender of principal bases; `resources/naxwe/01-ereyada.md` and `resources/sarfe/04-isbeddelka-codka.md` support assimilation; corpus material supports `hu'` and `huga` | Retain every alternation and example. Describe the inventories as the patterns gathered in this chapter, link the maintained morphophonology summary, and do not expand them into an unverified exhaustive paradigm. | `repository-supported`; `intentional-retained` |
| N03-R004 | 58-75 | Useful number/gender distinction and intact `-kii/-tii` examples; the remote-or-past explanation is an introductory source analysis, not a complete account of discourse deixis or tense / medium | `resources/sarfe/01-magacyada.md` supports plural gender; `resources/naxwe/04-sarfaha-magacuyaallada.md` also records independent uses of `kii/tii`; no separate canonical resource establishes the two stated conditions as exhaustive | Retain the plural-gender account and both examples. Scope the `-kii/-tii` description as common uses in this analysis and cross-link independent pronominal use; do not add a new tense theory. | `repository-supported` in part; `intentional-retained` |
| N03-R005 | 77-104 | Clean near/far demonstrative explanation and valid singular/plural table; `gasacan` has no independent repository occurrence and its intended base or reading is not uniquely established / high | `resources/sarfe/01-magacyada.md` supports `ninkan`, `naagtaas`, and `buuggaas`; repository-wide exact-form search finds `gasacan` only here, while no exact canonical replacement is recorded | Retain all four numbered examples and every table row. Mark `gasacan` as an unresolved source form and do not silently replace it with an inferred form such as `gacantan`. | `repository-supported` in part; `unresolved` |
| N03-R006 | 106-119 | `keer/teer` are valid demonstrative forms, but `koo/too` are placed in the same distance table even though maintained lexical and grammar resources classify them as indefinite standalone pronouns / high | `resources/qaamuus/15-k.md` entries `keer¹/keer²`; `resources/qaamuus/02-t.md` entries `teer¹/teer²`; `resources/naxwe/17-naxwaha-af-soomaaliga.md` supports `keer/teer/kuweer`; dictionary entries `koo²/too¹` and `resources/naxwe/04-sarfaha-magacuyaallada.md` classify `koo/too` as `magacuyaal aan cayinnayn` | Retain `keer/teer` as the less-common demonstrative pair. Retain the information that `koo/too` exist, but distinguish their repository classification as independent indefinite pronouns instead of presenting them as suffixal determiners parallel to `-kan/-tan`. | `repository-supported`; `classification-change` |
| N03-R007 | 121-153 | Core `-kee/-tee` analysis, independent uses, and most table forms are supported; the statement that the suffix alone makes the question omits focus/syntax detail, and `wadakee` has no independent match / high | `resources/naxwe/12-noocyada-weeraha.md` §§12.2.3.1-12.2.3.2 supports `-ee`, focus, and independent `kee/tee`; `resources/naxwe/04-sarfaha-magacuyaallada.md` supports `kee/tee/kuwee`; exact-form search finds `wadakee` only here and the dictionary records `wada` as a particle, not the required noun | Retain the question/statement contrast, the three independent examples, and the full table. Link the syntax account, qualify the suffix-only explanation as introductory, and label `wadakee` unresolved without inferring `wadehee`, `waddadee`, or another repair. | `repository-supported` in part; `unresolved` |
| N03-R008 | 155-181 | Clean possessive introduction and complete eight-row person paradigm, including inclusive/exclusive first-person plural / low | `resources/naxwe/04-sarfaha-magacuyaallada.md`; `resources/naxwe/17-naxwaha-af-soomaaliga.md`; dictionary entries `kayga`, `kaaga`, `kiisa`, `keeda`, `keenna`, `kayaga`, `kiinna`, and `kooda` | Retain both introductory examples, every paradigm row, and the possessor-gender explanation. Add a link to the independent-pronoun and possessive-pronoun treatment; normalize no forms. | `repository-supported` |
| N03-R009 | 183-191 | Clear distinction between the gender of the possessed noun and the gender of the possessor; examples are internally coherent / low | Masculine/feminine possessive series in `resources/naxwe/04-sarfaha-magacuyaallada.md` and dictionary `k-/t-` possessive entries | Retain both numbered examples and all five inline forms. Clarify only through a cross-link; no paradigm expansion is required. | `repository-supported`; `intentional-retained` |
| N03-R010 | 193-216 | Valuable compositional analysis and four kinship examples; treating every possessive as possessive-plus-article and extending the bare form to kinship terms, body parts, and `saaxiib` is source analysis not independently specified as an exceptionless rule / medium | Dictionary and `resources/naxwe/17-naxwaha-af-soomaaliga.md` support possessive suffixes; `resources/naxwe/10-dhismaha-oraah-magaceedyada.md` supports broader possessive phrase structure; no maintained resource independently enumerates the full bare-possessive class stated here | Retain the decomposition, both pieces of internal evidence, and examples 16-19. Present the decomposition and lexical class as the chapter's analysis, link broader noun-phrase structure, and avoid extending the list to unattested nouns. | `repository-supported` in part; `intentional-retained`; `unresolved` as a universal analysis |

## Proposed SLS-native blueprint

The cleaned chapter should retain the current title and sequence:

1. the four determiner types;
2. definite articles, gender agreement, allomorphs, number, and `-kii/-tii`;
3. demonstratives and their singular/plural behavior;
4. interrogative `-kee/-tee` and independent forms;
5. the full possessive-person paradigm;
6. possessed-noun gender; and
7. the source's compositional account of possessive suffixes.

All 22 numbered example lines and all rows of the four tables should remain.
Cleanup may add scope notes and repository links, but it must not invent
replacements for `gasacan` or `wadakee`, silently turn source analyses into
SLS-wide rules, or discard `koo/too`; those two forms should be retained with
their repository-supported classification made explicit.

Recommended links are `02-sarfaha-magacyada.md`,
`04-sarfaha-magacuyaallada.md`, `10-dhismaha-oraah-magaceedyada.md`,
`12-noocyada-weeraha.md`, `../sarfe/01-magacyada.md`, and
`../sarfe/04-isbeddelka-codka.md`.

## Audit approval

- **Audit approval:** approved by the maintainer on 2026-08-12 with the
  instruction, "go ahead."
- **Approved finding IDs:** N03-R001 through N03-R010
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Cleanup result and review

### Applied cleanup

- Added navigation to the maintained noun, pronoun, noun-phrase, question,
  noun-morphology, and morphophonology resources.
- Identified `-ka/-ta` as base forms and scoped the shared-knowledge,
  proper-noun, allomorph, and `-kii/-tii` descriptions to the chapter's
  introductory analysis.
- Retained `gasacan` and `wadakee` exactly as written while explicitly marking
  both as unresolved source forms for which the repository supplies no unique
  correction.
- Retained `keer/teer` as dictionary-supported demonstratives and retained the
  source's `koo/too` table row with a note recording their maintained SLS
  classification as independent indefinite pronouns.
- Scoped the interrogative and possessive-composition explanations and linked
  their fuller syntactic and pronominal treatments.

### Deliberately retained

- All 22 numbered example lines, with no textual differences from the
  pre-cleanup resource.
- Every row of the two two-column and two three-column Markdown tables, with no
  textual differences from the pre-cleanup resource.
- The original heading sequence, all possessive paradigm forms, and every
  source example.

### Validation

- `git diff --check`: passed.
- Original/current numbered-example comparison: no differences.
- Original/current Markdown-table-row comparison: no differences.
- Structure: one H1, four H2, five H3; 22 numbered example lines; two
  two-column tables; two three-column tables.
- Local-link resolution: 10 occurrences checked, 0 missing.
- Provenance TSV validation: every row has 10 tab-separated fields.
- Unresolved-source preservation: `gasacan` and `wadakee` each remain once.
- Cleaned file size: 263 lines; 1,519 words; 10,573 bytes.
- Cleaned SHA-256:
  `ba2eb43534d5244a9e294f763978dde540088c3be66a9a91b9ba4633f3a9776a`.
