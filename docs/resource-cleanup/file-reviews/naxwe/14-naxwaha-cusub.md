# Audit record — Naxwaha Cusub ee Af Soomaaliga

- **Resource path:** `resources/naxwe/14-naxwaha-cusub.md`
- **Collection / family:** naxwe / supplementary grammar
- **Priority:** P1
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-12
- **File size at audit start:** 2,095 lines; 14,103 words; 79,379 bytes
- **Resource-text changes during audit:** none

## Target output model

The final file must be an SLS-native topic resource rather than a cleaned book
or page-order transcription. The audit therefore separates usable linguistic
analysis from covers, author and institutional metadata, contents pages,
prefatory matter, exercises, answer keys, a collapsed multilingual glossary,
and duplicated or interleaved OCR columns.

Standard material already covered by canonical `naxwe/`, `dhawaaq/`, and
`qoraal/` resources should be summarized and linked rather than
reconstructed from damaged tables. Source-specific analyses may be retained,
but they must be identified as the source's model rather than silently
presented as universal SLS rules.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-63 | reviewed | N14-R001 |
| 64-145 | reviewed | N14-R002 |
| 146-240 | reviewed | N14-R003 |
| 241-375 | reviewed | N14-R004 |
| 376-459 | reviewed | N14-R005 |
| 460-624 | reviewed | N14-R006 |
| 625-741 | reviewed | N14-R007 |
| 742-884 | reviewed | N14-R008 |
| 885-1,074 | reviewed | N14-R009 |
| 1,075-1,139 | reviewed | N14-R010 |
| 1,140-1,231 | reviewed | N14-R011 |
| 1,232-1,518 | reviewed | N14-R012 |
| 1,519-1,709 | reviewed | N14-R013 |
| 1,710-1,866 | reviewed | N14-R014 |
| 1,867-2,095 | reviewed | N14-R015 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| N14-R001 | 1-63 | Duplicate cover/title material, author and university metadata, damaged contents, page numbers, and introduction text interleaved through the contents / fatal | `resources/naxwe/00-sources.md`; `resources/qoraal/00-sources.md`; `resources/dhawaaq/00-sources.md`; same-file title repetitions | Remove all cover and contents apparatus from the content file. Retain one topic title. Record the repository-supported author form `Maxamed Xaaji Xuseen Raabi` in the source inventory rather than the content file; do not reconstruct damaged contents or pagination. | `structural-only`; `repository-supported` for the author form |
| N14-R002 | 64-145 | Book purpose, audience, chapter descriptions, proposed alphabet commentary, exercise instructions, and glossary description are duplicated and crossed with the opening phonology column / fatal | `resources/naxwe/README.md`; canonical phonology, orthography, and grammar collections | Exclude book-purpose, audience, exercise, and appendix descriptions. Retain only a short resource scope assembled from intact topic labels; do not preserve the page-order introduction or its authorial claims as SLS policy. | `structural-only`; `unresolved` for damaged prose |
| N14-R003 | 146-240 | Alphabet, letter/sound distinction, an author-specific 23-symbol proposal, examples, and four exercises are duplicated and column-interleaved / fatal | `resources/naxwe/01-ereyada.md`; `resources/dhawaaq/01-hordhac.md`, `03-shibbanayaasha.md`, `04-shaqaallada.md`, `07-xuruufta-caalamiga.md`; `spec/orthography/` where present | Do not reproduce the damaged inventory or present the 23-symbol proposal as the SLS alphabet. Retain a concise distinction among alphabet, letter, and sound only where directly supported, and link to canonical dhawaaq/orthography resources. Remove all exercises and unrecoverable rows. | `repository-supported` for canonical links; `intentional-retained` only if the historical proposal is explicitly labeled; otherwise exclude |
| N14-R004 | 241-375 | Three-part word-class model followed by noun gender, definite articles, one plural pattern, tables, and exercises; page columns and duplicate text are extensively interleaved / fatal | `resources/naxwe/01-ereyada.md`, `02-sarfaha-magacyada.md`, `03-sarfaha-tifaftireyaasha.md`; `resources/sarfe/01-magacyada.md`; same-file repeated headings and examples | Retain the source's three broad classes—`magac`, `fal`, and `qurub`—as an explicitly source-specific model. Summarize noun diagnostics and representative intact forms, then link full gender, article, and plural treatment to canonical resources. Remove exercises and do not infer damaged table cells. | `intentional-retained` for the source taxonomy; `repository-supported` for canonical noun coverage; `unresolved` for damaged rows |
| N14-R005 | 376-459 | Verb classification by tense/person, partial tense and person paradigms, and exercises occupy crossed columns with missing cells / fatal | `resources/naxwe/07-sarfaha-falalka.md`, `08-hogatuska-baradigmaha-falalka.md`; `resources/sarfe/02-falalka.md`; dictionary evidence for `qofayn` | Retain only the intact claim that verbs are diagnosed here through `goorayn` and `qofayn`, with a small number of complete source examples. Delegate full paradigms and current terminology to canonical verb resources. Remove exercises and do not complete tables by analogy. | `repository-supported` for the overview; `unresolved` for table reconstruction |
| N14-R006 | 460-624 | Particle definition and the conjunctions `iyo`, `ama`, and `oo` contain useful examples but are crossed with exercise columns and the opening syntax chapter / fatal | `resources/naxwe/06-sarfaha-iskuxireyaasha.md`; same-file repeated conjunction explanations | Retain concise source-supported functions and only complete examples for `iyo`, `ama`, and `oo`; link to the canonical conjunction resource. Remove all exercises and isolate the syntax boundary without importing adjacent fragments. | `repository-supported`; `structural-only` for the boundary; `unresolved` for crossed examples |
| N14-R007 | 625-741 | Basic-sentence discussion, deletion test, sentence particles, examples, and the start of a five-type classification are duplicated and interleaved / fatal | `resources/naxwe/09-weer-fudud.md`; same-file clean repetitions at lines 659-740; `resources/naxwe/ereyfur.md` | Retain the recoverable source definition of `weedh saleed`, its deletion test, and its discussion of sentence particles as a source analysis. Preserve grammaticality marks on complete examples. Introduce the five-type taxonomy only as the source's model; do not generalize it as the canonical SLS classification. | `intentional-retained`; `repository-supported` for sentence-particle cross-links; `unresolved` for fragments |
| N14-R008 | 742-884 | Five basic-sentence types—`abyane`, `ma-abyane`, `farcan`, `tilmaameed`, and `lammaane`—are mixed with duplicated columns and several exercises / fatal | Same-file definitions and glossary fragments; `resources/naxwe/07-sarfaha-falalka.md`, `09-weer-fudud.md`; repository-wide searches show little independent support for `abyane`/`ma-abyane` in this grammatical sense | Retain the five-way classification only with clear source attribution and only where definitions/examples are intact. Do not normalize or silently map `abyane` and `ma-abyane` to modern categories without evidence. Remove exercises and incomplete examples. | `intentional-retained`; `unresolved` for terminology mapping and damaged examples |
| N14-R009 | 885-1,074 | Speech/writing distinction, two word-splitting tests, subject-pronoun contractions, `in`, `waxa`, `baa`, `waa`, and negative `ma` paradigms are densely interleaved and partly duplicated / fatal | `resources/qoraal/01-hadal-iyo-qoraal.md`, `02-eray-kooban-hadalka.md`, `03-kala-qoridda-adag.md`, `04-kala-qoridda-lama-qasban.md`; `resources/naxwe/04-sarfaha-magacuyaallada.md`, `09-weer-fudud.md` | Replace damaged duplicated exposition and paradigms with a concise overview and direct links to the manually verified orthography resources. Retain no row whose form-to-label alignment is uncertain. Remove the exercise material. | `repository-supported` for delegation; `unresolved` for damaged alignments |
| N14-R010 | 1,075-1,139 | `ma` plus copular material leads into a comma-only punctuation discussion and exercises; two page columns overlap / fatal | `resources/qoraal/05-astaamaynta.md`; `resources/naxwe/09-weer-fudud.md` | Retain only a short link to canonical punctuation and relevant copular/negative resources. Do not reconstruct the damaged paired table or reuse exercise sentences as authoritative examples. | `repository-supported`; `unresolved` for crossed rows |
| N14-R011 | 1,140-1,231 | Appendix opening on speech versus writing repeats earlier material and is crossed with a second page containing the two word-splitting principles and English comparisons / fatal | `resources/qoraal/01-hadal-iyo-qoraal.md`; same-file repetitions at lines 885-924 and 1,232-1,518 | Treat this as duplicated exposition. Preserve no page-order reconstruction; link to the canonical verified orthography chapter and carry forward only a concise source-supported principle if it adds unique value. | `repository-supported`; `unresolved` for interleaved prose |
| N14-R012 | 1,232-1,518 | Long conversational narrative, contracted-form inventory, expansion pairs, detailed analysis of ambiguous `bay`, and a rewritten formal version are duplicated across page columns / fatal | `resources/qoraal/01-hadal-iyo-qoraal.md`; `resources/qoraal/02-eray-kooban-hadalka.md`, especially its `bay` and paired-example sections | Do not reconstruct the narrative or its typography from the OCR. Retain a compact explanation of `koobnaan` and `fidsanaan` and selected verified `bay` contrasts by linking to the canonical orthography resource. Exclude damaged narrative filler and duplicate columns. | `repository-supported` for canonical treatment; `unresolved` for damaged source pairs |
| N14-R013 | 1,519-1,709 | Easy, difficult, and inappropriate expansion patterns contain useful categories but the paradigms and examples are heavily crossed and incomplete / fatal | `resources/qoraal/02-eray-kooban-hadalka.md`, `03-kala-qoridda-adag.md`, `04-kala-qoridda-lama-qasban.md`; `resources/naxwe/04-sarfaha-magacuyaallada.md` | Preserve the three-way distinction only as a short overview and route all paradigms to the canonical orthography files. Do not repair forms through analogy or translate the source's unresolved judgments into new normative rules. | `repository-supported` for the existing curated chapters; `unresolved` for source tables |
| N14-R014 | 1,710-1,866 | Exercise answer key, page references, answer tables, residual exercises, and the beginning of a multilingual glossary are interleaved / fatal | `resources/naxwe/README.md`; `resources/naxwe/ereyfur.md`; same-file exercise sections | Remove the complete answer-key block and all page/exercise apparatus. Treat the partial glossary opening as part of the unresolved glossary unit; do not recover definitions from answer fragments. | `structural-only`; `unresolved` for glossary fragments |
| N14-R015 | 1,867-2,095 | Somali/Arabic/English/Italian glossary has lost column alignment, duplicates many entries, merges definitions, and ends mid-column / fatal | `resources/naxwe/ereyfur.md`; relevant canonical grammar, phonology, and orthography resources; repository dictionaries and wordlists | Exclude the collapsed multilingual glossary from the topic file. Preserve source-specific terms only where they are used and defined in recoverable topical content. Do not infer Arabic, English, or Italian pairings; link to `ereyfur.md` for the maintained glossary. | `unresolved`; `repository-supported` for glossary delegation |

## Proposed SLS-native blueprint

The rewritten file should be titled **Naxwaha cusub: ereyeynta, weedhaynta,
iyo hadal-qoraalka** and contain:

1. a concise scope note and links to canonical SLS collections;
2. the distinction among codayn, ereyeyn, and weedhayn, without reproducing
   the damaged alternative alphabet;
3. the source's three broad word classes, clearly labeled as its analytical
   model;
4. a compact noun overview: gender, definite articles, and number;
5. a compact verb overview: tense (`goorayn`) and person (`qofayn`);
6. the conjunctions `iyo`, `ama`, and `oo` with complete source-supported
   examples;
7. the source's basic-sentence/deletion analysis and sentence particles;
8. the five source-defined basic-sentence types, with no unsupported modern
   remapping;
9. a short overview of speech versus writing, contraction (`koobnaan`), and
   expansion (`fidsanaan`) linked to the canonical orthography chapters;
10. a short punctuation link rather than a duplicate damaged chapter; and
11. a final table of canonical SLS references.

The rewrite must exclude covers, author and university text, the contents,
prefatory book-purpose material, page numbers, exercises, answer keys,
appendix labels, the conversational narrative used as a drill, the collapsed
multilingual glossary, duplicated columns, and fragments whose reading order
cannot be established.

No new linguistic example may be introduced during cleanup. Every retained
example, definition, and table row must map either to an intact source passage
or to an identified canonical repository resource. Source-specific terminology
must not be silently normalized.

## Audit approval

- **Audit approval:** approved by the maintainer on 2026-08-12 with the
  instruction, "go ahead."
- **Approved finding IDs:** N14-R001 through N14-R015
- **Deferred finding IDs:** unresolved readings within the approved findings
  remain excluded or delegated as recorded; approval does not authorize their
  reconstruction by inference.

## Cleanup result and review

The SLS-native cleanup was applied on 2026-08-12.

Applied:

- replaced the 2,095-line book/PDF-shaped OCR transcript with a 282-line
  topical reference;
- organized the retained content into six sections covering the source's
  three analytical levels, word classes, basic-sentence analysis,
  speech/writing, punctuation, and canonical SLS references;
- retained the source-specific three-class word model and five basic-sentence
  types with explicit labels preventing them from being mistaken for universal
  SLS classifications;
- retained only complete source examples and repository-supported forms;
- routed full noun, verb, conjunction, sentence, phonology, word-splitting,
  and punctuation treatment to the corresponding canonical resources; and
- added the repository-supported author form `Maxamed Xaaji Xuseen Raabi` to
  `resources/naxwe/00-sources.md`.

Excluded:

- duplicate cover and institutional material, contents, preface, pagination,
  exercises, answer keys, and appendix furniture;
- the author-specific 23-symbol alphabet proposal as a normative SLS alphabet;
- damaged paradigms and examples whose form-to-label relationships could not
  be recovered;
- the conversational drill narrative and duplicated page columns; and
- the collapsed Somali/Arabic/English/Italian glossary and all uncertain
  multilingual pairings.

Validation:

- `git diff --check`: passed;
- one H1, six H2 topic sections, and ten H3 subsections;
- all local Markdown links resolve;
- all correction-log rows have ten TSV fields;
- retained sentence examples were found in the original file, while the
  contraction/expansion forms are also present in the verified orthography
  resources; and
- the post-cleanup glossary link now targets the approved bilingual
  `ereyfur.md`; no grammar content changed; and
- final size: 282 lines; 1,676 words; 10,840 bytes.
- Current SHA-256:
  `1a2d41e81d8c05e96d2beec0f9d1052bdc818b52918b94a4a0da22f4fdc1f6e0`.

- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes
