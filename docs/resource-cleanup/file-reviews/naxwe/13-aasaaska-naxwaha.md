# Audit record — Aasaaska Naxwaha Af Soomaaliga

- **Resource path:** `resources/naxwe/13-aasaaska-naxwaha.md`
- **Collection / family:** naxwe / supplementary grammar
- **Priority:** P1
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-12
- **File size at audit start:** 859 lines; 5,136 words; 29,108 bytes
- **Resource-text changes during audit:** none

## Target output model

The final file must be an SLS-native topic resource, not a cleaned book or PDF
transcription. The audit therefore distinguishes usable linguistic content from
publishing matter, exercises, duplicated paradigms, dated claims, and OCR
debris.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-100 | reviewed | N13-R001 |
| 101-200 | reviewed | N13-R002 |
| 201-300 | reviewed | N13-R003 |
| 301-400 | reviewed | N13-R004 |
| 401-500 | reviewed | N13-R005 |
| 501-600 | reviewed | N13-R006 |
| 601-700 | reviewed | N13-R007 |
| 701-800 | reviewed | N13-R008 |
| 801-859 | reviewed | N13-R009 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| N13-R001 | 1-100 | Cover, ministry, author, printer, place, preface, committee signature, false headings, column interleaving, and dated unsupported claims about Somali and dialects / fatal | `resources/README.md`; `resources/naxwe/README.md`; canonical phonology and dialect resources | Remove all book apparatus. Retain only a concise definition of grammar and its three descriptive levels. Do not carry forward the claim that Somali is unchanged or lacks substantial dialect variation. | `structural-only`; `repository-supported` for the grammar model |
| N13-R002 | 101-200 | Damaged sound inventory followed by useful parts-of-speech, article, and noun classifications mixed with exercises and OCR errors / high | `naxwe/01-ereyada.md`, `02-sarfaha-magacyada.md`; `dhawaaq/03-shibbanayaasha.md`, `04-shaqaallada.md` | Replace the duplicated sound inventory with canonical links. Retain concise definitions of the nine word classes, articles, and noun classes; remove exercises. | `repository-supported` |
| N13-R003 | 201-300 | Noun gender, number, plural patterns, group nouns, examples, and exercises; several rows are damaged or duplicated / high | `naxwe/02-sarfaha-magacyada.md`; `sarfe/01-magacyada.md` | Summarize gender and number diagnostics with representative examples. Delegate complete plural paradigms to canonical noun resources; remove exercises and damaged duplicate rows. | `repository-supported` |
| N13-R004 | 301-400 | Tail of group-noun material, political examples, exercises, compounding, and incomplete pronoun paradigms / high | `naxwe/01-ereyada.md`, `04-sarfaha-magacuyaallada.md`; `sarfe/03-dhismaha-ereyga.md` | Retain productive compound types and a compact pronoun classification. Remove exercises, dated political examples, and incomplete paradigm debris. | `repository-supported` |
| N13-R005 | 401-500 | Reflexive pronouns and modifier classes contain useful definitions and agreement examples but are interspersed with exercises and corrupted forms / high | `naxwe/03-sarfaha-tifaftireyaasha.md`, `04-sarfaha-magacuyaallada.md` | Retain reflexive, qualitative, possessive, and demonstrative categories with representative forms. Cross-link full paradigms; remove exercises and damaged filler. | `repository-supported` |
| N13-R006 | 501-600 | Demonstrative, numeral, ordinal, and interrogative modifiers lead into the verb chapter; content is partly usable but duplicates canonical chapters and contains exercises / high | `naxwe/03-sarfaha-tifaftireyaasha.md`, `05-sarfaha-tirada.md`, `07-sarfaha-falalka.md` | Keep the modifier taxonomy and short numeral/ordinal rules. Begin a concise verb overview; delegate full tables and remove exercises. | `repository-supported` |
| N13-R007 | 601-700 | Verb stem endings, regular/irregular classes, auxiliaries, tense, mood, copular forms, and transitivity are mixed with OCR errors and exercises / high | `naxwe/07-sarfaha-falalka.md`; `sarfe/02-falalka.md`, `03-dhismaha-ereyga.md` | Retain a compact verb classification and representative examples. Use current SLS terms and canonical links; remove drills and unrecoverable rows. | `repository-supported` |
| N13-R008 | 701-800 | Adverb categories and conjunctions contain useful taxonomy but many damaged examples, exercises, and page-column collisions / high | `naxwe/06-sarfaha-iskuxireyaasha.md`; syntax chapters `09`-`12` | Retain definitions and representative examples for adverbs and conjunctions. Remove exercises and malformed sequences. | `repository-supported` |
| N13-R009 | 801-859 | Prepositions and interjections are followed by exercises, scan debris, and a book contents/index block / high | `naxwe/06-sarfaha-iskuxireyaasha.md`; repository-wide search for interjection terminology | Retain the four basic prepositions and the two interjection classes, which add useful coverage. Remove all exercises, scan debris, and the closing book index. | `repository-supported`; `structural-only` for exclusions |

## Proposed SLS-native blueprint

The rewritten file should be titled **Qaybaha hadalka iyo aasaaska naxwaha
Af-Soomaaliga** and contain:

1. grammar and its three descriptive levels;
2. the nine word classes in one overview table;
3. articles;
4. nouns: general/proper, concrete/abstract, gender, and number;
5. compounds;
6. pronouns;
7. modifiers: qualitative, possessive, demonstrative, numeral, and
   interrogative;
8. verbs: stem, regularity, auxiliaries, tense, mood, copular forms, and
   transitivity;
9. adverbs;
10. conjunctions;
11. prepositions;
12. interjections; and
13. links to the detailed canonical SLS resources.

The rewrite must exclude front matter, author and committee material,
publishing details, preface/conclusion, exercises, answer prompts, pagination,
the closing index, political examples that add no linguistic value, duplicated
full paradigms, and unrecoverable OCR fragments.

## Audit approval

- **Audit approval:** approved by the maintainer on 2026-08-12 with the
  instruction, "go ahead."
- **Approved finding IDs:** N13-R001 through N13-R009
- **Deferred finding IDs:** none; unusable OCR fragments and book-only material
  are approved for exclusion rather than preservation.

## Rewrite result and review

The SLS-native rewrite was applied on 2026-08-12.

Applied:

- replaced the 859-line book/PDF-shaped transcript with a 410-line topical
  reference;
- removed all publishing, author, committee, preface, exercise, answer-prompt,
  page-number, scan-debris, and index material;
- organized the retained material into twelve SLS sections covering grammar
  levels, the nine word classes, and canonical resource links;
- retained compact, corrected descriptions of articles, nouns, compounds,
  pronouns, modifiers, verbs, adverbs, conjunctions, prepositions, and
  interjections;
- preserved the useful distinction between expressive and sound-imitative
  interjections; and
- replaced duplicated full paradigms with links to the canonical `naxwe/`,
  `sarfe/`, and `dhawaaq/` resources.

Excluded:

- unsupported claims that Somali is unchanged or lacks substantial dialect
  variation;
- politically dated examples without distinctive linguistic value;
- incomplete paradigms and examples whose OCR damage prevented a reliable
  reading; and
- material whose only purpose was to reproduce the source book's page order.

Validation:

- `git diff --check`: passed;
- one H1, twelve H2 sections, and twenty-one H3 subsections;
- all local Markdown links resolve;
- no publishing metadata, exercises, index text, or known OCR debris remains;
  and
- final size: 410 lines; 2,244 words; 13,571 bytes.

- **Rewrite approval:** approved 2026-08-23
- **Complete:** yes
