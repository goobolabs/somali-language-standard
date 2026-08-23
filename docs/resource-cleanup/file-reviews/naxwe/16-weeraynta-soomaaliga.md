# Audit record — WEEraynta Soomaaliga

- **Resource path:** `resources/naxwe/16-weeraynta-soomaaliga.md`
- **Collection / family:** naxwe / supplementary grammar
- **Priority:** P1
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-12
- **File size at audit start:** 675 lines; 4,727 words; 27,145 bytes
- **Resource-text changes during audit:** none

## Target output model

The final file must be an SLS-native syntax resource rather than a page-order
OCR transcript. The audit separates usable analyses of simple and complex
Somali sentences from scan debris, false Markdown headings, damaged tree
diagrams, foreign-language comparisons, and a corrupted multilingual glossary.

The source uses the notation `W` (weer), `OM` (oraah magaceed), and `OF`
(oraah faleed), and it calls a relative/dependent construction `weer aano`.
These source-specific choices may be retained with explicit labels, but they
must not silently replace the terminology or constituent analysis used by the
canonical SLS syntax chapters.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-7 | reviewed | N16-R001 |
| 8-62 | reviewed | N16-R002 |
| 63-95 | reviewed | N16-R003 |
| 96-159 | reviewed | N16-R004 |
| 160-196 | reviewed | N16-R005 |
| 197-272 | reviewed | N16-R006 |
| 273-321 | reviewed | N16-R007 |
| 322-423 | reviewed | N16-R008 |
| 424-477 | reviewed | N16-R009 |
| 478-537 | reviewed | N16-R010 |
| 538-675 | reviewed | N16-R011 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| N16-R001 | 1-7 | Title followed by unreadable scan debris and three false Markdown headings / fatal | `resources/naxwe/00-sources.md`; same-file title | Retain one normalized topic title, `Weeraynta Soomaaliga`. Remove the unreadable opening fragments and false headings. Preserve the inventory's `Xilliyada 5aad` note only as source metadata, not as content. | `structural-only`; `unresolved` for debris |
| N16-R002 | 8-62 | Introduction to linguistic levels, phonemes, syllables, morphemes, words, phrases, sentences, semantics, and synchronic/diachronic linguistics contains pervasive OCR substitutions and malformed examples / fatal | `resources/naxwe/00-luqadda-iyo-fekerka.md`, `01-ereyada.md`; `resources/sarfe/03-dhismaha-ereyga.md`; `resources/dhawaaq/`; `resources/naxwe/ereyfur.md` | Retain only the recoverable hierarchy `cod → alan → morfiim → erey → oraah → weer`, with complete source examples where available. Link phonology and morphology details to canonical resources. Exclude damaged historical-linguistics exposition and do not reconstruct its foreign-language comparisons. | `repository-supported`; `unresolved` for damaged prose and examples |
| N16-R003 | 63-95 | Definition and scope of syntax, sentence productivity, and the simple-sentence rule `W = OM + OF` are readable in outline but affected by OCR damage / high | `resources/naxwe/09-weer-fudud.md`, `10-dhismaha-oraah-magaceedyada.md`; same-file repeated `OM`/`OF` analyses | Retain the source's `W = OM + OF` model as explicitly source-specific notation. Summarize the recoverable distinction between noun phrase and verb phrase; route canonical simple-sentence structure to chapters 9 and 10. Exclude speculative or damaged introductory prose. | `intentional-retained`; `repository-supported` for cross-links |
| N16-R004 | 96-159 | Simple-sentence tree analyses, focus particle `baa`, subject/object placement, free pronouns, and imperative examples are represented as false headings and broken diagram fragments / fatal | `resources/naxwe/09-weer-fudud.md`, especially `baa/ayaa`, subject pronouns, and tree diagram; `resources/naxwe/12-noocyada-weeraha.md` for imperatives | Retain only complete source sentences and concise source-supported observations about `baa`. Do not reconstruct tree branches or transformation steps whose nodes are damaged. Delegate the complete focus and imperative systems to canonical chapters. | `repository-supported`; `unresolved` for diagrams and damaged transformations |
| N16-R005 | 160-196 | Constituents allowed in the source's `OM`—definite articles, demonstratives, interrogatives, and possessives—contain useful examples but pervasive word-level OCR / high | `resources/naxwe/10-dhismaha-oraah-magaceedyada.md`; `02-sarfaha-magacyada.md`, `03-sarfaha-tifaftireyaasha.md`, `04-sarfaha-magacuyaallada.md` | Retain a compact inventory of the recoverable OM constituents and only complete examples. Do not silently correct damaged forms such as the heading-like sentence fragments; link full paradigms and phrase structure to canonical resources. | `repository-supported`; `unresolved` for damaged tokens |
| N16-R006 | 197-272 | `waa` versus `baa`, nominal and adjectival predicates, copular comparison, demonstratives, and interrogatives are mixed with false headings, English comparison text, and OCR errors / fatal | `resources/naxwe/09-weer-fudud.md` §§9.2.1 and 9.2.3; `resources/naxwe/12-noocyada-weeraha.md`; same-file contrastive statements | Retain the source's recoverable contrast that `baa` marks the OM in its analysis while `waa` marks the OF, explicitly linked to canonical focus treatment. Preserve only intact Somali examples. Exclude English translations and do not infer corrupted copular or interrogative forms. | `repository-supported`; `unresolved` for damaged examples and foreign comparison |
| N16-R007 | 273-321 | Horyaalayaasha `u`, `ku`, `ka`, and `la`, their placement before the verb, and several semantic roles are mixed with damaged trees and a foreign-language comparison / fatal | `resources/naxwe/06-sarfaha-iskuxireyaasha.md` §§6.1-6.1.1; `resources/naxwe/09-weer-fudud.md`; same-file complete role statements | Retain a compact source-supported overview of the four horyaalayaal and complete Somali examples only. Remove damaged tree notation and the Italian comparison; link to the canonical horyaale resource. | `repository-supported`; `unresolved` for diagrams and corrupted example forms |
| N16-R008 | 322-423 | Questions formed with `ma` and `miyaa`, OM/OF focus, particle position, pronoun attachment, and nominal predicates contain valuable analysis but extensive OCR damage / fatal | `resources/naxwe/12-noocyada-weeraha.md` §§12.2.1-12.2.2; `resources/naxwe/09-weer-fudud.md`; same-file repeated question/declarative pairs | Replace the damaged exposition with a compact comparison grounded in the canonical chapter. Retain complete source question pairs only when their wording is recoverable. Do not reconstruct corrupt paradigms or use uncertain spellings to state new rules. | `repository-supported`; `unresolved` for damaged forms |
| N16-R009 | 424-477 | Complex sentences, a main/dependent distinction, the source-specific term `weer aano`, relative-clause analyses, four claimed structural patterns, and examples contain missing text and corrupted tree layouts / fatal | `resources/naxwe/11-weerta-adag.md` §§11.1-11.4; `resources/naxwe/10-dhismaha-oraah-magaceedyada.md`; repository-wide search finds `weer aano` only in this source | Retain `weer aano` only as an explicitly source-specific label and map its recoverable function to canonical `weer faahfaahineed`. Preserve complete source sentences; do not recreate the four-pattern table or damaged trees from inference. | `intentional-retained`; `repository-supported` for canonical comparison; `unresolved` for lost structures |
| N16-R010 | 478-537 | Conjunctions `-na`, `oo`, and `ee`, relative clauses, an `in` complement clause, and two ambiguity demonstrations are mixed with severe OCR and false headings / fatal | `resources/naxwe/06-sarfaha-iskuxireyaasha.md` §6.2; `resources/naxwe/11-weerta-adag.md` §§11.1 and 11.4; same-file repeated clause boundaries | Retain concise functions for recoverable conjunctions and the distinction between relative and complement clauses, using only intact examples. Exclude ambiguous sentences whose intended readings are damaged; link to canonical conjunction and complex-sentence resources. | `repository-supported`; `unresolved` for corrupted readings |
| N16-R011 | 538-675 | Somali/Italian/English glossary has no valid table structure, many OCR-damaged terms and translations, duplicated/incomplete entries, and a truncated ending / fatal | `resources/naxwe/ereyfur.md`; canonical grammar, morphology, and phonology terminology; repository dictionaries and wordlists | Exclude the collapsed glossary from the syntax resource. Preserve source-specific terminology only where defined in recoverable topical prose. Do not infer translation pairings; link to the maintained `ereyfur.md`. | `unresolved`; `repository-supported` for glossary delegation |

## Proposed SLS-native blueprint

The rewritten file should be titled **Weeraynta Soomaaliga: weedhaha fudud iyo
kuwa adag** and contain:

1. a scope note explaining the source-specific notation `W`, `OM`, and `OF`;
2. the recoverable hierarchy from sound to sentence, with canonical links;
3. the source's simple-sentence model `W = OM + OF`;
4. noun-phrase constituents: articles, demonstratives, interrogatives, and
   possessives;
5. a compact source/canonical comparison of `baa` and `waa`;
6. the horyaalayaasha `u`, `ku`, `ka`, and `la`;
7. questions with `ma` and `miyaa`, delegated in detail to chapter 12;
8. complex sentences, including the source term `weer aano` alongside the
   canonical term `weer faahfaahineed`;
9. conjunctions and complement clauses; and
10. a final table of canonical SLS references.

The rewrite must exclude opening scan debris, false headings, broken tree
diagrams, unrecoverable transformation sequences, foreign-language comparison
sentences, the damaged ambiguity demonstrations, and the collapsed
Somali/Italian/English glossary.

No new linguistic example may be introduced during cleanup. Every retained
example and rule must map either to a complete source passage or an identified
canonical repository resource. Source-specific notation and terminology must
remain visibly labeled rather than silently normalized.

## Audit approval

- **Audit approval:** approved by the maintainer on 2026-08-12 with the
  instruction, "go ahead."
- **Approved finding IDs:** N16-R001 through N16-R011
- **Deferred finding IDs:** unresolved diagrams, tokens, examples, and glossary
  pairings remain excluded or delegated as recorded; approval does not
  authorize reconstruction by inference.

## Cleanup result and review

The SLS-native cleanup was applied on 2026-08-12.

Applied:

- replaced the 675-line OCR-shaped transcript with a 250-line topical syntax
  reference;
- organized the retained content into nine sections covering linguistic
  levels, simple sentences, OM structure, `baa/waa`, horyaalayaal, questions,
  complex sentences, conjunctions, and canonical references;
- preserved `W`, `OM`, `OF`, and `weer aano` as explicitly source-specific
  notation rather than silently replacing the canonical SLS analysis;
- retained only complete source examples or examples already present in the
  identified canonical syntax chapters;
- distinguished source examples from examples imported from canonical SLS
  chapters; and
- delegated full focus, horyaale, question, relative-clause, complement-clause,
  and conjunction systems to `naxwe/06` and `naxwe/09`-`12`.

Excluded:

- opening scan debris and all false Markdown headings;
- broken tree diagrams and unrecoverable transformation sequences;
- damaged historical-linguistics and foreign-language comparisons;
- corrupted examples that required more than uniquely supported OCR repair;
- the damaged ambiguity demonstrations; and
- the collapsed Somali/Italian/English glossary and uncertain translations.

Validation:

- `git diff --check`: passed;
- one H1, nine H2 topic sections, and eight H3 subsections;
- all local Markdown links resolve;
- every Markdown table has consistent column counts;
- all correction-log rows have ten TSV fields;
- retained examples were found in the original source or their identified
  canonical SLS chapters; and
- the post-cleanup glossary link now targets the approved bilingual
  `ereyfur.md`; no grammar content changed; and
- final size: 250 lines; 1,341 words; 8,652 bytes.
- Current SHA-256:
  `6cdb3f5ff62d862d28bebc638d281f0b4aa9d543d6dce1f78c100b9d27f2d915`.

- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes
