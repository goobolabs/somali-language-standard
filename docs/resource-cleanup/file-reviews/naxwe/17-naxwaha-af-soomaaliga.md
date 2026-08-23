# Audit record — Naxwaha Af Soomaaliga

- **Resource path:** `resources/naxwe/17-naxwaha-af-soomaaliga.md`
- **Collection / family:** naxwe / supplementary grammar
- **Priority:** P1
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-12
- **File size at audit start:** 5,307 lines; 31,738 words; 208,946 bytes
- **Resource SHA-256 at audit start:** `45ed024278ae9fd58ada79597bf57c2c6dd735de948b1d987d95cf1593f42ced`
- **Resource-text changes during audit:** none

## Target output model

The final file should be an SLS-native, topic-organized supplementary grammar,
not a cleaned facsimile of the source book. It should keep the recoverable
linguistic coverage of the source while excluding cover matter, contents,
exercises, political and institutional commentary, page-order duplication,
false headings, and unrecoverable OCR tables.

The source uses a substantial author-specific terminology system, including
`jaangooyo xaaladley`, `managaadeyaal`, `nagaadeyaal`, `lihi`, `joogto
dareerto`, `joogto taagan`, `jimayn`, `sugeyaal`, `farriinley`, `xisley`, and
`magacyo falidaysan`. Repository-wide searches show that several of these
labels occur only in this source. They may be retained as attributed source
terms and mapped to canonical SLS topics, but they must not silently replace
the terminology or analysis in the canonical grammar chapters.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-110 | reviewed | N17-R001 |
| 111-236 | reviewed | N17-R002 |
| 237-611 | reviewed | N17-R003 |
| 612-775 | reviewed | N17-R004 |
| 776-1037 | reviewed | N17-R005 |
| 1038-1312 | reviewed | N17-R006 |
| 1313-1411 | reviewed | N17-R007 |
| 1412-1743 | reviewed | N17-R008 |
| 1744-2228 | reviewed | N17-R009 |
| 2229-2475 | reviewed | N17-R010 |
| 2476-2889 | reviewed | N17-R011 |
| 2890-3169 | reviewed | N17-R012 |
| 3170-3596 | reviewed | N17-R013 |
| 3597-3802 | reviewed | N17-R014 |
| 3803-3935 | reviewed | N17-R015 |
| 3936-4064 | reviewed | N17-R016 |
| 4065-4372 | reviewed | N17-R017 |
| 4373-4609 | reviewed | N17-R018 |
| 4610-4969 | reviewed | N17-R019 |
| 4970-5271 | reviewed | N17-R020 |
| 5272-5307 | reviewed | N17-R021 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| N17-R001 | 1-110 | Cover, author note, ministry names, political preface, and a badly corrupted contents list with missing and duplicated numbers / fatal | `resources/naxwe/00-sources.md`; same-file section boundaries | Retain only the resource title. Keep author attribution in `00-sources.md`; exclude all front matter and the book contents from the topic resource. Recover section order from the body rather than the damaged contents list. | `structural-only`; `unresolved` for damaged contents |
| N17-R002 | 111-236 | General definition of speech, language-policy and colonial commentary, alphabet table, sounds, syllables, and word segmentation contain dense OCR damage and unsupported historical claims / fatal | `resources/naxwe/01-ereyada.md`; `resources/dhawaaq/03-shibbanayaasha.md`, `04-shaqaallada.md`; `resources/sarfe/03-dhismaha-ereyga.md` | Exclude the political and historical exposition. Retain a short source-attributed outline of sound, letter, syllable, and word segmentation only where readable; delegate inventories and analysis to canonical phonology and morphology resources. Do not reconstruct the damaged alphabet table. | `repository-supported`; `unresolved` for damaged forms and claims |
| N17-R003 | 237-611 | Noun definition, proper/common and singular/group distinctions, gender, personal-name suffixes, seven plural patterns, collective plural, and irregular plurals include valuable data but many broken columns and broad historical claims / fatal | `resources/naxwe/02-sarfaha-magacyada.md`; `resources/sarfe/01-magacyada.md`; same-file parallel singular/plural pairs | Preserve the noun taxonomy and representative, recoverable paradigms. Map gender and number to canonical SLS resources. Retain author-specific plural grouping only with a source label; do not silently validate all seven classes or repair uncertain examples by inference. | `repository-supported`; `intentional-retained`; `unresolved` for damaged rows |
| N17-R004 | 612-775 | Review exercises, drills, foreign-loan discussion, profession and calendar lists, collective nouns, articles, and an unresolved claim about neuter gender / high | `resources/naxwe/02-sarfaha-magacyada.md`, `03-sarfaha-tifaftireyaasha.md`; `resources/naxwe/00-sources.md` | Exclude all exercises and classroom prompts. Retain compact, recoverable discussions of borrowed noun morphology, collective nouns, and articles; mark the author's disputed gender remarks as source-specific or omit them where they add no reliable rule. | `structural-only`; `intentional-retained`; `unresolved` |
| N17-R005 | 776-1037 | Author-specific `jaangooyo xaaladley`, noun/article/gender combinations, demonstratives, free and clitic pronouns (`managaadeyaal`/`nagaadeyaal`), plus corrupted tables and exercises / fatal | `resources/naxwe/03-sarfaha-tifaftireyaasha.md`, `04-sarfaha-magacuyaallada.md`, `09-weer-fudud.md`; repository-wide terminology search | Preserve the recoverable case/article, demonstrative, and pronoun contrasts. Present the two source terms only as historical/source labels alongside canonical free and reduced pronoun terminology. Exclude drills and do not reconstruct incomplete tables. | `repository-supported`; `intentional-retained`; `unresolved` for paradigms |
| N17-R006 | 1038-1312 | Verb roots and stems, six verb classes, tense inventory, present progressive, affirmative/interrogative/negative paradigms, and review exercises are heavily interleaved / fatal | `resources/naxwe/07-sarfaha-falalka.md`, `08-hogatuska-baradigmaha-falalka.md`; same-file repeated endings | Retain the recoverable class inventory and tense/aspect topic coverage as source analysis. Replace duplicate or damaged paradigm blocks with canonical links and compact verified examples. Exclude exercises and uncertain person/number forms. | `repository-supported`; `intentional-retained`; `unresolved` |
| N17-R007 | 1313-1411 | `Lihi` covers two- and three-noun possession, possessive pronoun suffixes, and relational suffixes `-eed/-ood/-yeed`; the paradigm has column loss and OCR substitutions / high | `resources/naxwe/03-sarfaha-tifaftireyaasha.md`, `04-sarfaha-magacuyaallada.md`, `10-dhismaha-oraah-magaceedyada.md` | Preserve all three possession topics and map `lihi` explicitly to canonical possession. Retain only complete suffix forms and examples supported by the source or canonical chapters. | `repository-supported`; `intentional-retained`; `unresolved` |
| N17-R008 | 1412-1743 | Habitual/stative `joogto taagan`, regular conjugation classes, irregular/prefix-conjugating verbs, affirmative, interrogative, and negative paradigms contain extensive column interleaving / fatal | `resources/naxwe/07-sarfaha-falalka.md`, `08-hogatuska-baradigmaha-falalka.md`; same-file repeated examples | Preserve the tense/aspect distinction and coverage of regular and irregular verbs as source-specific analysis. Use compact verified paradigms or canonical links; never fill missing cells by analogy alone. | `repository-supported`; `intentional-retained`; `unresolved` |
| N17-R009 | 1744-2228 | Adjective derivation, adjective compounds, reduplication, comparison/intensification, copular paradigms, five interrogative classes, and a long exercise block mix usable content with malformed headings and tables / fatal | `resources/naxwe/03-sarfaha-tifaftireyaasha.md`, `05-sarfaha-tirada.md`, `12-noocyada-weeraha.md`; `resources/sarfe/03-dhismaha-ereyga.md` | Retain adjective formation, reduplication, comparison, and the five interrogative semantic groups. Condense repeated copular paradigms, exclude exercises, and link full question structure to canonical chapters. | `repository-supported`; `unresolved` for damaged forms |
| N17-R010 | 2229-2475 | Simple past and habitual past (`tagto dareerto`) paradigms include recoverable endings, irregular verbs, affirmative/interrogative/negative patterns, political examples, and OCR damage / fatal | `resources/naxwe/07-sarfaha-falalka.md`, `08-hogatuska-baradigmaha-falalka.md` | Preserve both past-time topics and representative verified forms. Exclude political examples and duplicate damaged paradigms; map terminology to canonical tense/aspect labels without erasing the source terms. | `repository-supported`; `intentional-retained`; `unresolved` |
| N17-R011 | 2476-2889 | Comparison (`jimayn`), noun and clause conjunctions, near future, future with `doon`, and affirmative/negative paradigms include a false OCR heading and duplicated table material / fatal | `resources/naxwe/03-sarfaha-tifaftireyaasha.md`, `06-sarfaha-iskuxireyaasha.md`, `07-sarfaha-falalka.md`, `11-weerta-adag.md` | Retain comparison levels, conjunction functions, and the source's near/far future distinction as attributed analysis. Remove the false heading and duplicate table debris; use canonical links for full conjunction and future systems. | `repository-supported`; `intentional-retained`; `unresolved` |
| N17-R012 | 2890-3169 | Subject/object definitions, free/clitic pronouns, impersonal `la/loo`, transitivity, three clause orders, and object-pronoun groups are interrupted by unreadable scan diagrams at lines 3043-3062 / fatal | `resources/naxwe/04-sarfaha-magacuyaallada.md`, `07-sarfaha-falalka.md`, `09-weer-fudud.md`; same-file readable order tables | Preserve recoverable subject, object, transitivity, impersonal, and word-order observations. Present flexible orders as source examples, not a universal SLS rule. Remove the unreadable diagram and do not reconstruct lost labels. | `repository-supported`; `intentional-retained`; `unresolved` for diagram |
| N17-R013 | 3170-3596 | Compound/postpositional expressions, verbal horyaalayaal, reflexive `is`, pitch-marked pairs, `la/loo`, person-object paradigms, two blocks of unreadable diagram debris, and extensive exercises / fatal | `resources/naxwe/04-sarfaha-magacuyaallada.md`, `06-sarfaha-iskuxireyaasha.md`, `09-weer-fudud.md`; `resources/naxwe/01-ereyada.md` for codkac | Retain the four canonical horyaalayaal and recoverable compound-location expressions, object pronouns, `is`, `la/loo`, and source pitch contrasts. Exclude scan debris and all exercises. Use `codkac` when giving the canonical cross-link, while preserving `ku dhufasho` only as the source's label. | `repository-supported`; `intentional-retained`; `unresolved` |
| N17-R014 | 3597-3802 | Conditional clauses, conjunctions, present/future/counterfactual paradigms, negative conditions, and source terminology contain useful structure but many corrupt endings / fatal | `resources/naxwe/11-weerta-adag.md` §11.5.2; `resources/naxwe/12-noocyada-weeraha.md`; same-file parallel clauses | Retain conditional types and connective inventory in compact form. Keep only complete examples and verified endings; delegate full clause analysis to the canonical complex-sentence chapter. | `repository-supported`; `unresolved` |
| N17-R015 | 3803-3935 | Eight claimed adverb classes—time, place, manner, quantity, interrogative, purpose, contrast, and related functions—contain overlaps with conjunctions and damaged examples / high | `resources/naxwe/06-sarfaha-iskuxireyaasha.md`, `11-weerta-adag.md`; same-file category lists | Preserve the source's recoverable adverb classification as attributed analysis, with representative intact examples. Map overlapping conjunction/subordinate-clause functions to canonical resources instead of asserting a new SLS classification. | `intentional-retained`; `repository-supported`; `unresolved` |
| N17-R016 | 3936-4064 | New-word formation, agentive and relational suffixes, pitch contrasts, and verb reduplication are mixed with historical commentary and many corrupt derivations / fatal | `resources/naxwe/01-ereyada.md`; `resources/sarfe/03-dhismaha-ereyga.md`, `04-isbeddelka-codka.md`; `resources/dhawaaq/05-codadka-sare.md` | Retain recoverable derivational patterns and reduplication. Exclude language-policy commentary and uncertain equations; cross-link canonical derivation and codkac discussions. | `repository-supported`; `unresolved` |
| N17-R017 | 4065-4372 | Cardinal/ordinal numbers, counted-noun morphology, nine pronoun groups, and extensive noun suffix inventories include clear topic coverage but numerous malformed forms and loan-history claims / fatal | `resources/naxwe/02-sarfaha-magacyada.md`, `04-sarfaha-magacuyaallada.md`, `05-sarfaha-tirada.md`; `resources/sarfe/01-magacyada.md` | Retain the cardinal/ordinal distinction, counted-noun behavior, pronoun categories, and recoverable suffix classes. Delegate complete paradigms to canonical chapters and keep loan-source remarks only when useful and explicitly attributed. | `repository-supported`; `intentional-retained`; `unresolved` |
| N17-R018 | 4373-4609 | Vowel deletion/alternation in stems, modal/evidential particles, the author's `sugeyaal` analysis of `waa/baa/ayaa/yaa`, imperative classes, and negative imperatives contain useful examples and corrupt paradigms / fatal | `resources/naxwe/07-sarfaha-falalka.md`, `09-weer-fudud.md`, `12-noocyada-weeraha.md`; `resources/sarfe/04-isbeddelka-codka.md` | Preserve the vowel-alternation topic, modal particles, imperatives, and the historical `sugeyaal` account as explicitly source-specific. Use the canonical focus analysis for SLS guidance; do not endorse the source's `waa/baa/ayaa/yaa` equivalences silently. | `repository-supported`; `intentional-retained`; `unresolved` |
| N17-R019 | 4610-4969 | `Xisleyda` groups warnings, address, surprise, insults/disgust, silence, backchannels, curses, blessings, and animal calls; direct/indirect speech and interrupted-event constructions follow, then exercises / fatal | `resources/naxwe/11-weerta-adag.md`, `12-noocyada-weeraha.md`; repository dictionaries and wordlists for individual forms | Preserve the expressive-language categories, animal-directed calls, direct/indirect speech, and linked-event topic as source-specific evidence. Use a restrained representative sample, retain culturally sensitive forms only as linguistic data, exclude political examples and all exercises, and do not repair uncertain expressions by guesswork. | `intentional-retained`; `repository-supported`; `unresolved` |
| N17-R020 | 4970-5271 | Antonym pairs, gender assignment for place names, nominalization suffixes, and deverbal/derived nouns contain damaged tables, questionable generalizations, and many unique forms / fatal | `resources/naxwe/01-ereyada.md`, `02-sarfaha-magacyada.md`, `07-sarfaha-falalka.md`; `resources/sarfe/01-magacyada.md`, `03-dhismaha-ereyga.md` | Retain recoverable antonymy, place-name agreement examples, nominalization, and derived-noun patterns. Mark place-name gender and source suffix classes as attributed analysis; do not generalize from corrupted tables or silently normalize unique forms. | `repository-supported`; `intentional-retained`; `unresolved` |
| N17-R021 | 5272-5307 | Collapsed multilingual bibliography with missing entries, broken ordering, truncated titles, and publication fragments / fatal | `resources/naxwe/00-sources.md`; same-file author/title | Exclude the book bibliography from the topic resource. Do not infer missing citations. Keep the resource's known attribution in the collection source inventory. | `structural-only`; `unresolved` |

## Proposed SLS-native blueprint

The rewritten file should be titled **Naxwaha Af Soomaaliga: dulmar
dhammaystiran** and contain:

1. a scope note distinguishing source-specific terminology from canonical SLS
   terminology;
2. a concise sound, syllable, and word-structure orientation with links to
   canonical phonology and morphology;
3. nouns: noun types, gender, number, plural classes, collective nouns, and
   articles;
4. case/article behavior, demonstratives, pronouns, and possession;
5. verb roots, conjugation classes, regular and irregular verbs;
6. tense, aspect, mood, questions, negation, and imperatives, using compact
   verified paradigms and links to the canonical verb guide;
7. adjectives, derivation, reduplication, comparison, and interrogatives;
8. subjects, objects, transitivity, focus, impersonal `la/loo`, reflexive `is`,
   object pronouns, and clause order;
9. horyaalayaal, conjunctions, conditionals, adverbs, and complex clauses;
10. word formation, codkac contrasts, numeral behavior, nominalization, and
    derived nouns;
11. modal and expressive items, direct/indirect speech, and linked-event
    constructions;
12. place-name agreement as attributed source evidence; and
13. a final table of canonical SLS references.

The cleanup must preserve the overall linguistic coverage of all nineteen book
parts while removing book-only material and repeated drills. It may condense
large paradigms only when the topic remains represented and a canonical SLS
resource supplies the detailed system.

No new linguistic example may be introduced during cleanup. Every retained
example and claim must map to a complete source passage or an identified
canonical repository resource. Source-only terminology and analyses must stay
visibly attributed. Unrecoverable cells, examples, diagrams, bibliography
entries, and word forms must not be reconstructed by analogy.

## Audit approval

- **Audit approval:** approved by the maintainer on 2026-08-12 with the
  instruction, “go ahead.”
- **Approved finding IDs:** N17-R001 through N17-R021
- **Deferred finding IDs:** unresolved tables, diagrams, word forms, examples,
  claims, and bibliography entries remain excluded or attributed as recorded;
  approval does not authorize reconstruction by inference.

## Cleanup result and review

The SLS-native cleanup was applied on 2026-08-12.

Applied:

- replaced the 5,307-line book/PDF-shaped OCR transcript with a 767-line
  topical reference;
- organized the retained content into thirteen sections covering sound and
  word structure, nouns, modifiers and pronouns, verbs, adjectives and
  questions, clause roles, horyaalayaal and object forms, conjunctions and
  conditions, word formation, numerals, expressive language, place-name
  agreement, nominalization, and canonical references;
- retained all nineteen book parts through their recoverable topic content,
  while consolidating repeated paradigms and classroom drills;
- preserved `jaangooyo xaaladley`, `managaadeyaal`, `nagaadeyaal`, `lihi`,
  `joogto dareerto`, `joogto taagan`, `jimayn`, `sugeyaal`, `farriinley`,
  `xisley`, and `ku dhufasho` as visibly source-specific terminology;
- retained compact source evidence for plural classes, collective nouns,
  possession, tense/aspect, comparison, word order, `la/loo`, `is`,
  conditionals, adverbs, derivation, expressive items, reported speech,
  place-name agreement, and nominalization; and
- routed full paradigms and canonical analysis to the corresponding SLS
  grammar, morphology, and phonology resources.

Excluded:

- cover, ministry, author-note, contents, publication, and bibliography
  apparatus;
- political, colonial, and institutional commentary;
- all exercises, answer prompts, repeated drills, and page-order duplication;
- unreadable alphabet, paradigm, and syntax diagrams;
- false Markdown headings and raw OCR character debris;
- uncertain paradigm cells, derivational equations, etymologies, and
  bibliography entries that required reconstruction by inference; and
- duplicated examples where a compact representative set preserves the topic.

Validation:

- `git diff --check`: passed;
- the rewritten file has one H1, thirteen H2 topic sections, and thirty-nine
  H3 subsections;
- all local Markdown links resolve;
- every Markdown table has a consistent column count;
- all correction-log rows have ten TSV fields;
- the original resource remains recoverable from Git, and every retained topic
  maps to an approved finding; and
- the post-cleanup glossary link now targets the approved bilingual
  `ereyfur.md`; no grammar content changed; and
- final size: 767 lines; 4,161 words; 27,854 bytes.
- Current SHA-256:
  `3a3a21b96763d4f664b644798e97cedf084589ea3670bfc3e5282592037471dc`.

- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes
