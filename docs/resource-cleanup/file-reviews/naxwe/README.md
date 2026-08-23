# Audit record — Naxwe README

- **Resource path:** `resources/naxwe/README.md`
- **Collection / family:** naxwe / collection map and conventions
- **Priority:** P3
- **Method:** repository-only, line-by-line documentation audit
- **Audit status:** approved; cleanup applied; awaiting maintainer cleanup review
- **Audit started:** 2026-08-12
- **File size at audit start:** 40 lines; 237 words; 1,958 bytes
- **Resource SHA-256 at audit start:**
  `20432a8b7c82ab3ac1ce00bc8bc5094ea170911e544736e38c220db256c4562b`
- **Resource-text changes during this audit:** none
- **Pre-audit state note:** the baseline already contains the two curation-
  policy bullets at lines 27-32 (`topic-focused SLS references` and permitted
  reordering/condensation). They predate this audit and will be preserved.

## Target output model

This is a compact collection README, not a grammar chapter or OCR transcript.
It has one H1, two H2 headings, one fenced layout block with nine logical file
or range entries, six top-level convention bullets, three nested numbering
bullets, and no Markdown links. Its file map is complete: all eighteen content
chapters, the source registry, glossary, and README exist.

Cleanup should preserve every mapped file and all six convention topics while
making the README accurate for the cleaned collection. It should identify
`resources/` as descriptive evidence rather than the normative specification
layer; distinguish the maintained core chapters from source-specific
supplementary grammars; scope Somali-prose and English-gloss conventions to
content files; repair the primary chapter grouping; preserve the asterisk and
topic-focused-curation policies with appropriate qualifications; and replace
the inert code map with compact local links.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-6 | reviewed | NRD-R001 |
| 8-21 | reviewed | NRD-R002, NRD-R003 |
| 23-26 | reviewed | NRD-R004 |
| 27-32 | reviewed | NRD-R005 |
| 33-34 | reviewed | NRD-R006 |
| 35-38 | reviewed | NRD-R007 |
| 39-40 | reviewed | NRD-R008 |
| whole file | reviewed | NRD-R009 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| NRD-R001 | 1-6 | The broad-coverage and AI/NLP purpose are useful, but “full scope” is exhaustive and “source-of-truth document” conflates this descriptive resource collection with the normative SLS layer / fatal | `docs/RESOURCES.md` and `docs/ARCHITECTURE.md` explicitly define `resources/` as curated descriptive evidence and `spec/` as normative rules; `resources/README.md` calls this directory the evidence base | Retain the title, broad grammar purpose, downstream AI/NLP/MT uses, and non-textbook status. Replace “full scope” with broad coverage, call the folder a collection/evidence library rather than one document, and link the evidence-versus-normative explanation. | `repository-supported`; `classification-correction`; `scope-correction`; `intentional-retained` |
| NRD-R002 | 8-21 | The layout accurately lists all files, but the code fence is non-navigable and its spacing is uneven / low | Filesystem validation finds exactly eighteen content Markdown files, `00-sources.md`, `ereyfur.md`, and this README; the cleaned source registry already links the same collection | Preserve all nine logical entries and their primary/supplementary/glossary/README roles. Convert the code block to a compact Markdown table with local links; do not add or remove resources. | `repository-supported`; `structural-only`; `navigation-update`; `intentional-retained` |
| NRD-R003 | 12-18 | “Primary” and “supplementary” labels are correct but do not state precedence; the opening then lets source-specific supplementary analyses appear equally normative / high | N13-N17 review records repeatedly require source-specific terminology and analyses to be attributed and mapped without overriding canonical chapters; files 00-12 are the maintained core sequence | Retain all five supplementary files. Add a concise role note: files 00-12 are the core maintained grammar; files 13-17 preserve and map other source analyses and do not silently override the core or normative `spec/` rules. | `repository-supported`; `scope-correction`; `provenance-clarification`; `intentional-retained` |
| NRD-R004 | 23-26 | “All prose is in Somali” incorrectly includes this English README and `00-sources.md`; “English gloss on first use” is a useful tendency but not a verified guarantee for every technical term / high | The README and source registry are English metadata documents; `ereyfur.md` is trilingual; content chapters are Somali-first and contain many, but not universally every, parenthetical English gloss | Scope Somali-first prose to grammar content files. Say English glosses may be supplied when useful and direct systematic Somali/English/Italian mappings to the glossary; do not promise one in every chapter at first occurrence. | `same-file-exact`; `repository-supported`; `scope-correction`; `intentional-retained` |
| NRD-R005 | 27-32 | Topic-focused curation, exclusion of book apparatus, reordering/condensation, and canonical linking accurately describe the approved cleanup method; wording says “files” where registry/README/glossary have different roles / medium | `resources/README.md`, the source registry, and N00-N17 review records support these content-cleanup rules; the two bullets were already present before this audit | Preserve both bullets and every listed exclusion. Scope them to grammar content resources, state that uncertain material is excluded or source-labeled rather than reconstructed, and link the source registry/cleanup evidence without treating metadata files as source-book rewrites. | `repository-supported`; `scope-correction`; `intentional-retained` |
| NRD-R006 | 33-34 | Literal-asterisk convention is valid, but an unqualified statement can make every source judgment look like a repository-wide ban / medium | Nine cleaned content files contain escaped starred forms; their approved review records repeatedly scope judgments to the intended construction or reading | Retain the asterisk convention. Add that the surrounding prose determines whether a judgment is canonical, construction-specific, or an attributed source analysis; do not remove or renumber examples. | `repository-supported`; `judgment-qualification`; `intentional-retained` |
| NRD-R007 | 35-38 | Three-part numbering applies only to primary files 00-12; the first label understates file 00's language/thought role, and the 02-08 word-class list omits conjunctions/linkers while calling file 06 only “preposition” / high | Actual H1 titles show 00 language/thought, 01 word/phonology, 02-05 nominal classes, 06 `iskuxireyaal` with horyaalayaal and xiriiriyeyaal, 07-08 verbs/paradigms, and 09-12 sentence structure | Preserve the three-way map but explicitly scope it to the primary sequence. Reword 00-01 as language, word, and sound; 02-08 as noun/determiner/pronoun/numeral/linker-preposition/verb morphology; and 09-12 as simple sentence, OM, complex sentence, and sentence types. Add a separate supplementary role rather than extending primary numbering. | `same-file-exact`; `repository-supported`; `classification-correction`; `scope-correction`; `intentional-retained` |
| NRD-R008 | 39-40 | Three-column/no-definition description is exact, but saying terms “are explained in context within the chapters above” is not verified for every one of 268 entries and “above” is imprecise for a file map / medium | TSV validation finds one three-field header, 268 three-field data rows, and no duplicate Somali heads. The glossary contains mappings rather than definitions; chapters explain terms they use, but no repository check establishes contextual definitions for all 268 | Retain the three-column/no-definition description and add the verified row count only if useful. Say chapters explain terms in their relevant contexts, not that every glossary entry is necessarily defined. Link the glossary and identify its languages. | `repository-supported`; `scope-correction`; `navigation-update`; `intentional-retained` |
| NRD-R009 | whole file | Complete but non-navigable map, no source/evidence link, and no explicit canonicality boundary / medium | All mapped targets and `docs/RESOURCES.md` exist; `00-sources.md` now supplies cleaned provenance and audit links; the live tracker records workflow state | Preserve one H1, the Layout and Conventions sections, all nine logical map entries, all six convention subjects, and the three primary grouping rows. Add links to every mapped file/range, the source registry, and the evidence-layer documentation. Do not hard-code transient cleanup counts or mark this README complete during cleanup. | `structural-only`; `repository-supported`; `navigation-update`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned README should retain its short sequence:

1. title and Somali-grammar collection purpose;
2. a clear descriptive-evidence versus normative-specification note;
3. a linked layout table containing the source registry, primary range, five
   supplementary files, glossary, and README;
4. core/supplementary precedence;
5. Somali-first prose and terminology/gloss conventions;
6. topic-focused curation and excluded book apparatus;
7. the scoped asterisk convention;
8. the corrected three-part map for primary files 00-12; and
9. the exact three-column glossary description.

No mapped file, role, or convention topic should be removed. No transient
approval count, unverified completeness claim, normative grammar rule, or new
linguistic example should be introduced.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-12 with the
  instruction, "go ahead for the one we audited."
- **Approved finding IDs:** NRD-R001 through NRD-R009
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no
- **Decision requested:** review and approve the cleaned README before the
  cleanup-approval or complete stages are marked.

## Audit validation

- Resource SHA-256 after audit:
  `20432a8b7c82ab3ac1ce00bc8bc5094ea170911e544736e38c220db256c4562b`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, two H2 headings, one fenced block with nine
  logical entries, six top-level convention bullets, three nested numbering
  bullets, and no Markdown links.
- Layout checked: eighteen content Markdown files, one source registry, one
  glossary, and this README; every listed target exists and no collection file
  is omitted.
- Glossary cross-check: one three-field header, 268 three-field data rows, no
  duplicate Somali head, and no definitions column.
- Preserved pre-audit wording: both topic-focused curation bullets at original
  lines 27-32 remain untouched during audit.

## Cleanup result and review

### Applied cleanup

- Retained the title, broad descriptive-grammar purpose, downstream AI/NLP/MT
  uses, non-textbook status, Layout and Conventions sections, all nine mapped
  entries, all six convention subjects, and the three-part primary map.
- Replaced the exhaustive “full scope” and “source-of-truth document” claims
  with the repository-supported role of this collection as descriptive
  evidence, linked to `docs/RESOURCES.md` and the normative `spec/` layer.
- Converted the non-navigable layout code fence into a two-column Markdown
  table. All eighteen chapters, the source inventory, glossary, and README
  remain represented; no collection resource was added or removed.
- Identified files 00-12 as the maintained descriptive core and files 13-17 as
  source-specific supplementary analyses that do not silently override the
  core or normative specification.
- Scoped Somali-first prose to grammar content, made English glosses optional
  where useful, and directed systematic Somali/English/Italian mappings to the
  glossary.
- Preserved the topic-focused curation policy and every listed exclusion.
  Added the approved safeguard that uncertain material is excluded or
  source-labeled instead of silently reconstructed.
- Preserved the asterisk convention and qualified its judgments as canonical,
  construction-specific, or attributed according to surrounding prose.
- Corrected the primary chapter map to include language and thought, linkers,
  verb paradigms, noun phrases, and the actual roles of files 00-12.
- Preserved the exact three-column/no-definition glossary description while
  removing the unsupported implication that every glossary entry is defined
  in a chapter.

### Deliberately retained

- The English metadata-document format used by this README and the source
  inventory.
- Every collection file/range and its primary, supplementary, glossary, or
  metadata role.
- All original convention topics: language and terminology, topic-focused
  curation, reordering and linking, starred judgments, chapter numbering, and
  glossary format.
- All excluded book-apparatus categories and the policy of linking maintained
  detail instead of copying it.

### Cleanup validation

- Cleanup approval and completion remain unmarked pending maintainer review.
- `git diff --check`: passed.
- Structure: one H1, two H2 headings, one valid two-column Markdown table with
  all nine logical entries, six top-level convention bullets, and three nested
  primary-numbering bullets.
- Resource preservation: all eighteen content chapters, the source inventory,
  glossary, and README remain mapped; no resource or convention topic was
  added to or removed from the collection.
- Local links: all 13 occurrences resolve across 12 unique existing targets.
- Role checks: “full scope” and “source-of-truth document” are absent;
  descriptive evidence, normative `spec/`, core 00-12, and supplementary
  13-17 boundaries are present.
- Glossary checks: the current map identifies one Somali-English grammar-
  terminology resource with no definitions; no unverified every-entry coverage
  claim remains.
- Provenance TSV validation: every row has 10 tab-separated fields.
- Post-cleanup navigation follow-up: the maintainer's approved glossary scope
  amendment replaced the TSV/Markdown pair with the single bilingual
  `ereyfur.md`; no other README content or workflow state changed.
- Cleaned file size after that navigation follow-up: 53 lines; 378 words;
  3,094 bytes.
- Cleaned SHA-256:
  `b38161693a934793135363e9bd45270d518f03f6038c8feaa4f846c2aa1fff96`.
