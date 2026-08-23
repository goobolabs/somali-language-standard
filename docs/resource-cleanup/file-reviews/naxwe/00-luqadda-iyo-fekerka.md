# Audit record — Luqadda iyo fekerka

- **Resource path:** `resources/naxwe/00-luqadda-iyo-fekerka.md`
- **Collection / family:** naxwe / primary grammar
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-12
- **File size at audit start:** 87 lines; 542 words; 3,390 bytes
- **Resource SHA-256 at audit start:** `d82c02ba85df80b88c0509ac4c794db45881298b753dc1038ed89c06c2bb1ce6`
- **Resource-text changes during audit:** none

## Target output model

This file is already a compact, coherent primary-grammar chapter rather than a
page-order OCR transcript. It has one H1, four valid H2 sections, one
well-formed table, and no cover matter, exercises, scan debris, false headings,
or damaged reading order.

Cleanup should therefore be conservative. It should retain the chapter's
discussion of language and thought, semantic categorization, words and
sentences, grammatical metalanguage, and shared properties of spoken
languages. It should repair only repository-supported forms, expose unresolved
source terms rather than guessing at them, qualify overbroad universal claims,
and add links to the detailed SLS chapters where useful.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-9 | reviewed | N00-R001 |
| 10-22 | reviewed | N00-R002 |
| 23-41 | reviewed | N00-R003 |
| 42-65 | reviewed | N00-R004 |
| 66-73 | reviewed | N00-R005 |
| 74-87 | reviewed | N00-R006 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| N00-R001 | 1-9 | Clean title and introductory discussion of society, language, communication, and thought; the relationship between language and thought is presented as a broad introductory claim / low | `resources/naxwe/16-weeraynta-soomaaliga.md` links back to this chapter for meaning; `resources/naxwe/00-sources.md` identifies the 1998 primary grammar | Retain the title and introduction. Present the language/thought relationship as the chapter's orientation rather than expanding it into a new scientific claim. No structural repair is needed. | `intentional-retained` |
| N00-R002 | 10-22 | Clear discussion of names, concrete and abstract referents; `jaceyl` conflicts with the repository's maintained headword and canonical grammar spelling / medium | `resources/naxwe/02-sarfaha-magacyada.md` lines 43 and 191; `resources/sarfe/01-magacyada.md` line 65; `resources/qaamuus/03-j.md` entry `jacayl`; `resources/madax-ereyo/03-j.md` | Retain the explanation and examples. Correct only `jaceyl` to `jacayl`; preserve `binii-aadanka` because it is supported in the same primary grammar, dictionary variants, and wordlist. | `repository-supported` for `jacayl`; `intentional-retained` for `binii-aadanka` |
| N00-R003 | 23-41 | Semantic hierarchy from `geed` and `xayawaan` to subgroups is clear and the table is structurally valid; the `kalluun` row intentionally has no example; `goolleey` has no independent repository match / medium | `resources/qaamuus/12-g.md` and `resources/madax-ereyo/12-g.md` support `gallayr`; dictionaries and wordlists support `naasley` and `xamaarato`; repository-wide search finds `goolleey` only here | Retain the hierarchy, table, and the empty-cell dash. Do not invent a fish example. Retain `goolleey` only as a visibly unresolved source form or defer it; do not silently normalize it. Preserve the final extension from nouns to properties and actions. | `structural-only`; `repository-supported` for supported group terms; `unresolved` for `goolleey` |
| N00-R004 | 42-65 | Clear transition from words to sentences, with valid nested list and five numbered examples; `aaraa'` is repository-supported, `labo` is a documented variant, but repeated `dugsan` has no matching lexical or grammatical evidence and its intended relation is unclear / high | `resources/naxwe/01-ereyada.md` uses the same building-block analogy; `resources/naxwe/09-weer-fudud.md` and `resources/naxwe/ereyfur.md` support `weer`; `resources/qaamuus/22-a.md` supports `aaraa'`; repository-wide searches find the three `ka dugsan` examples only here, while the dictionary supports `dugso/dugsoon` with different meanings | Retain the word-to-sentence explanation, the first two examples, list structure, and the topic of relations between entities. Do not guess a replacement for `dugsan`. During cleanup either retain the three-example set with an explicit unresolved-source note or omit that set while preserving the stated relation topic. | `repository-supported` for terminology; `intentional-retained` for `labo`; `unresolved` for `dugsan` |
| N00-R005 | 66-73 | Clean metalinguistic discussion with Somali terms followed by English glosses; no OCR or structural defect / low | `resources/naxwe/ereyfur.md` supports `magac/noun`, `fal/verb`, `sifo/adjective`, and `horyaale/preposition`; detailed chapters 02, 03, 06, and 07 cover the categories | Retain the section and its glosses. Add concise links to the detailed category chapters rather than expanding definitions here. Do not normalize source prose unnecessarily. | `repository-supported`; `structural-only` |
| N00-R006 | 74-87 | Coherent conclusion about common communicative functions and structural description, but statements that all languages use sounds, words, and sentences and that one descriptive toolkit fits every language are broader than the repository can establish / medium | `resources/naxwe/01-ereyada.md` supports the sound/word/sentence progression for the spoken-language scope of this grammar; `resources/dhawaaq/` covers Somali speech sounds; no independent repository source establishes the universal claim | Retain the conclusion as the source's framing but scope it explicitly to the spoken languages under discussion and to the descriptive purpose of this grammar. Preserve the final focus on Somali-specific properties. Do not add unsupported typological claims. | `intentional-retained`; `unresolved` as a universal claim |

## Proposed SLS-native blueprint

The cleaned chapter should remain titled **0. Luqadda iyo fekerka** and keep
its compact introductory character:

1. an opening scope paragraph on language, communication, and thought;
2. names and semantic categorization, including concrete and abstract
   referents;
3. the `geed` and `xayawaan` category hierarchy and its existing table;
4. words as building blocks of sentences, preserving only unambiguous
   examples;
5. grammatical metalanguage with links to the relevant SLS chapters;
6. shared properties of the spoken-language scope discussed by the source,
   followed by Somali-specific analysis; and
7. a short set of detailed SLS references only if it improves navigation.

The cleanup must not turn this short introductory chapter into a general
linguistics essay. It must not invent an example for the empty `kalluun` row,
guess a replacement for `goolleey` or `dugsan`, or silently convert the
source's philosophical and typological framing into SLS policy.

No new linguistic example may be introduced. Existing wording and layout
should remain unless an approved finding requires a correction, qualification,
or link.

## Audit approval

- **Audit approval:** approved by the maintainer on 2026-08-12 with the
  instruction, "go ahead."
- **Approved finding IDs:** N00-R001 through N00-R006
- **Deferred by default:** `goolleey`, `dugsan`, and the universal scope of
  lines 74-87 remain unresolved; approval authorizes only the handling
  described above, not an inferred correction.

## Cleanup result and review

The conservative SLS cleanup was applied on 2026-08-12.

Applied:

- retained the chapter's title, four topic sections, semantic hierarchy,
  category table, word-to-sentence explanation, and all source examples;
- corrected `jaceyl` to repository-supported `jacayl`;
- retained `goolleey` and the three `dugsan` examples without inferring new
  forms, adding explicit source-status notes for both unresolved readings;
- linked the four metalinguistic terms to the detailed SLS chapters; and
- scoped the concluding claims to spoken languages and to the descriptive
  framework used by this chapter.

Deferred:

- no lexical replacement was inferred for `goolleey` or `dugsan`;
- no example was invented for the empty `kalluun` row; and
- no new linguistic example or typological claim was added.

Validation:

- `git diff --check`: passed;
- one H1 and four H2 topic sections, matching the approved structure;
- the existing two-column category table remains structurally valid;
- all four added local Markdown links resolve;
- all five new correction-log rows have ten TSV fields;
- no occurrence of the unsupported spelling `jaceyl` remains in the resource;
  and
- final size: 98 lines; 582 words; 3,795 bytes; SHA-256
  `1172c4174e94ebfb212e577e599ecb14770607dabb19a12436b96067c90fbd16`.

- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes
