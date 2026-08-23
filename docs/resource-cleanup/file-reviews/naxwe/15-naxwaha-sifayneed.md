# Audit record — Naxwaha Sifayneed ee Af Soomaaliga

- **Resource path:** `resources/naxwe/15-naxwaha-sifayneed.md`
- **Collection / family:** naxwe / supplementary grammar
- **Priority:** P0
- **Method:** repository-only, line-by-line audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-12
- **File size at audit start:** 5,191 lines; 42,543 words; 242,495 bytes

## Scope and rules

Every line will be inspected before this audit is submitted for approval. This
record uses only evidence available in the repository. It does not use source
scans, external sources, or source-verification claims.

For every candidate, search the same file and relevant repository resources,
record competing readings, and use one of these labels:
`repository-supported`, `structural-only`, `unresolved`, or
`intentional-retained`.

## Audit progress

| Lines | Status | Notes |
| --- | --- | --- |
| 1-180 | reviewed | Findings N15-R001 through N15-R005 recorded below. |
| 181-380 | reviewed | Findings N15-R006 through N15-R010 recorded below. |
| 381-620 | reviewed | Findings N15-R011 through N15-R014 recorded below. |
| 621-900 | reviewed | Findings N15-R015 and N15-R016 recorded below. |
| 901-1,200 | reviewed | Finding N15-R017 recorded below. |
| 1,201-1,300 | reviewed | Finding N15-R018 recorded below. |
| 1,301-1,400 | reviewed | Finding N15-R019 recorded below. |
| 1,401-1,500 | reviewed | Finding N15-R020 recorded below. |
| 1,501-1,600 | reviewed | Finding N15-R021 recorded below. |
| 1,601-1,700 | reviewed | Finding N15-R022 recorded below. |
| 1,701-1,800 | reviewed | Finding N15-R023 recorded below. |
| 1,801-1,900 | reviewed | Finding N15-R024 recorded below. |
| 1,901-2,000 | reviewed | Finding N15-R025 recorded below. |
| 2,001-2,100 | reviewed | Finding N15-R026 recorded below. |
| 2,101-2,200 | reviewed | Finding N15-R027 recorded below. |
| 2,201-2,300 | reviewed | Finding N15-R028 recorded below. |
| 2,301-2,400 | reviewed | Finding N15-R029 recorded below. |
| 2,401-2,500 | reviewed | Finding N15-R030 recorded below. |
| 2,501-2,600 | reviewed | Finding N15-R031 recorded below. |
| 2,601-2,700 | reviewed | Finding N15-R032 recorded below. |
| 2,701-2,800 | reviewed | Finding N15-R033 recorded below. |
| 2,801-2,900 | reviewed | Finding N15-R034 recorded below. |
| 2,901-3,000 | reviewed | Finding N15-R035 recorded below. |
| 3,001-3,100 | reviewed | Finding N15-R036 recorded below. |
| 3,101-3,200 | reviewed | Finding N15-R037 recorded below. |
| 3,201-3,300 | reviewed | Finding N15-R038 recorded below. |
| 3,301-3,400 | reviewed | Finding N15-R039 recorded below. |
| 3,401-3,500 | reviewed | Finding N15-R040 recorded below. |
| 3,501-3,600 | reviewed | Finding N15-R041 recorded below. |
| 3,601-3,700 | reviewed | Finding N15-R042 recorded below. |
| 3,701-3,800 | reviewed | Finding N15-R043 recorded below. |
| 3,801-3,900 | reviewed | Finding N15-R044 recorded below. |
| 3,901-4,000 | reviewed | Finding N15-R045 recorded below. |
| 4,001-4,100 | reviewed | Finding N15-R046 recorded below. |
| 4,101-4,200 | reviewed | Finding N15-R047 recorded below. |
| 4,201-4,300 | reviewed | Finding N15-R048 recorded below. |
| 4,301-4,400 | reviewed | Finding N15-R049 recorded below. |
| 4,401-4,500 | reviewed | Finding N15-R050 recorded below. |
| 4,501-4,600 | reviewed | Finding N15-R051 recorded below. |
| 4,601-4,700 | reviewed | Finding N15-R052 recorded below. |
| 4,701-4,800 | reviewed | Finding N15-R053 recorded below. |
| 4,801-4,900 | reviewed | Finding N15-R054 recorded below. |
| 4,901-5,000 | reviewed | Finding N15-R055 recorded below. |
| 5,001-5,100 | reviewed | Finding N15-R056 recorded below. |
| 5,101-5,191 | reviewed | Finding N15-R057 recorded below. |

## Findings

Findings will be added in line order. Each finding must include exact lines,
issue class, severity, repository searches, proposed action, evidence label,
and unresolved alternatives.

| ID | Lines | Class / severity | Repository evidence searched | Proposed action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| N15-R001 | 3-27 | Invalid Markdown hierarchy and metadata noise / high | Same file lines 1-29; `resources/naxwe/00-sources.md`; repository-wide title search. | Convert the opening material into a single front-matter block after approval. Retain the current wording except for separately supported corrections; remove the stray trailing `|` on line 7 and leading `j` on line 24 only if approved as debris. | `structural-only` for hierarchy; `unresolved` for wording. |
| N15-R002 | 31-93 | Corrupt table of contents: broken numbering, inserted symbols, damaged words, and lost hierarchy / fatal | Same-file occurrences of `Qaybaha Hawleed` (line 327), `Kooxaynta Magaca` (line 1280), `Salkordhin` (line 1737), `Codbeddelidda` (line 3009), and later chapter headings; related grammar files. | Rebuild the contents block only from section titles and numbering that can be supported inside the repository. Keep any title or page-number reading without unique internal support as an explicit unresolved item. | `unresolved` with possible `structural-only` entries. |
| N15-R003 | 95-108 | Interleaved and heavily damaged acknowledgements / author-note material / fatal | Same file; `resources/naxwe/00-sources.md`; repository-wide searches for distinctive names and phrases. No complete internal parallel was found in the initial search. | Preserve the span as unresolved during this pass. Do not reconstruct prose or names from inference. Remove only isolated non-language debris if it is separately approved. | `unresolved` |
| N15-R004 | 110-165 | Heavily corrupted introductory prose, with word damage, false breaks, and incomplete phrases / fatal | Same-file heading and phrase searches; related naxwe files; dictionary and wordlist collections. No complete repository parallel was found in the initial search. | Record word-level candidates individually in later audit entries. Do not rewrite this prose as a paragraph until repository evidence supports each approved correction or structural change. | `unresolved` |
| N15-R005 | 167-180 | Chapter-boundary collision: the chapter-1 heading and its prose are merged with preceding damaged material / fatal | Same file lines 167-180 and later chapter-1 material; related naxwe resources. | Separate the visible chapter boundary from prior material as an approved structural repair only. Audit the chapter text line by line before proposing word edits. | `structural-only` for boundary; `unresolved` for wording. |
| N15-R006 | 181-217 | Continued chapter-1 prose and alphabet-list explanation are interleaved with earlier material, false prefixes, and damaged list labels / fatal | Same file lines 181-238; `resources/naxwe/14-naxwaha-cusub.md` lines 170-192, which discusses the same 23-letter alphabet but is not a textual parallel; phonology and orthography collections. | Treat this span as unresolved. Do not use a related work to reconstruct sentences or list labels. Retain only a separately approved structural boundary. | `unresolved` |
| N15-R007 | 218-238 | Alphabet subsection contains broken words, inserted characters, and false line boundaries / high | Same file lines 218-238 and later alphabet references; related naxwe, orthography, and phonology files. | Audit individual tokens for repository-supported corrections; otherwise preserve the wording as unresolved. Do not change the proposed alphabet or its examples through normalization. | `unresolved` |
| N15-R008 | 239-280 | Collapsed phonetic-reference table: headings, row labels, symbols, examples, and prose have lost their columns and reading order / fatal | Same file lines 239-280; `resources/dhawaaq/07-xuruufta-caalamiga.md`; phonology tables using IPA. These confirm the topic but do not provide a line-for-line parallel. | Define a table blueprint during audit, but do not reconstruct rows or symbols without unique repository support. | `unresolved` with possible `structural-only` repair |
| N15-R009 | 283-326 | Chapter-2 opening and its numbered classification list contain noise prefixes, broken words, missing list content, and invalid list structure / fatal | Same file lines 283-326; later occurrences of chapter-2 terminology; `resources/naxwe/01-ereyada.md` and related grammar resources. | Record each recoverable list label separately if internal evidence is unique; otherwise retain an explicit unresolved span. No inferred prose rewrite. | `unresolved` |
| N15-R010 | 327-380 | Functional-class section is merged with unrelated text and damaged examples; list, prose, and table-like material have no reliable reading order / fatal | Same file lines 327-380; later same-file terminology; `resources/naxwe/ereyfur.md`; related grammar files. | Preserve as an unresolved structural-reconstruction unit pending a complete internal-evidence inventory. Do not make local word corrections that imply a reading order. | `unresolved` |
| N15-R011 | 381-404 | Tail of the functional-class section and its examples are broken, reordered, and incomplete / fatal | Same file lines 381-404; earlier functional-class terminology; related grammar resources. | Keep this example block unresolved. It must not be repaired from semantic expectation alone. | `unresolved` |
| N15-R012 | 405-466 | Affix section contains missing list items, false prefixes, word damage, and interleaved explanatory prose / fatal | Same file lines 405-466; later same-file uses of `Lifaaq`; `resources/naxwe/01-ereyada.md`; terminology glossary. | Audit each list item and example separately before proposing a repair. Until then, retain the unit as unresolved and do not normalize linguistic terminology. | `unresolved` |
| N15-R013 | 468-534 | Boundary between parts of speech and the noun chapter is collapsed; examples, headings, and prose occupy mixed columns / fatal | Same file lines 468-534; same-file occurrences of `Qaybaha Hadalka` and `Magaca`; `resources/naxwe/ereyfur.md`; related grammar files. | Establish only approved major-section boundaries. Do not reconstruct the intervening explanatory text or examples without unique repository support. | `structural-only` for boundaries; `unresolved` for content |
| N15-R014 | 535-620 | Opening noun and gender material is severely column-interleaved, including tone notation and tabular examples / fatal | Same file lines 535-620; `resources/naxwe/02-sarfaha-magacyada.md`; `resources/qaamuus/11-c.md` and related entries. These support terminology, not a textual reconstruction. | Preserve notation, examples, and prose as unresolved until each item has internal evidence. Do not replace forms using modernized grammar material. | `unresolved` |
| N15-R015 | 621-859 | Continued gender and determiner material contains collapsed tables, split and reordered prose, damaged linguistic notation, and merged examples / fatal | Same file lines 621-859; `resources/naxwe/02-sarfaha-magacyada.md`; same-file qodob and wadar references; relevant dictionary entries. | Keep prose, examples, and notation unresolved. Use internal resources only to log potential token candidates, never to import a replacement explanation or table. | `unresolved` |
| N15-R016 | 860-900 | The heading `Aee See` is unreadable and the opening pluralisation material contains pervasive column interleaving and broken example rows / fatal | Same file lines 860-900; later same-file pluralisation and cunsurguurid material; `resources/naxwe/02-sarfaha-magacyada.md`. | Do not rename the heading or rebuild its examples without unique internal support. Record the apparent section boundary and retain the content as unresolved. | `structural-only` for boundary; `unresolved` for content |
| N15-R017 | 901-1,200 | Pluralisation and gender-change material is comprehensively corrupted: explanatory prose, classifications, tables, examples, and section boundaries are interleaved or incomplete / fatal | Same file lines 901-1,200; later same-file plural and cunsurguurid anchors; `resources/naxwe/02-sarfaha-magacyada.md`; related dictionary and wordlist entries. The related resources provide terminology and isolated forms, not a replacement for this text. | Keep this entire reconstruction unit unresolved. Record only uniquely supported token candidates in subsequent detailed findings; do not assemble a new table or prose sequence from other works. | `unresolved` |
| N15-R018 | 1,201-1,300 | A plural-suffix table is collapsed into fragments (1,208-1,279), followed by a partially intact but still interleaved noun-classification section (1,280-1,300) / fatal | Same file lines 1,201-1,300; later same-file references to noun classes and plurals; `resources/naxwe/02-sarfaha-magacyada.md`; dictionary entries for individual forms. | Treat the table as unresolved; do not infer cells or labels. Retain the visible `Kooxaynta Magaca` boundary as a proposed structural-only repair, while auditing its prose and examples line by line. | `unresolved` with a `structural-only` boundary candidate |
| N15-R019 | 1,301-1,400 | Noun classification and pronoun material is partly legible but interleaved with earlier text, damaged examples, and lost table cells / fatal | Same file lines 1,301-1,400; `resources/naxwe/02-sarfaha-magacyada.md`; `resources/naxwe/04-sarfaha-magacuyaallada.md`; relevant dictionary entries. These support terminology and isolated forms only. | Preserve readable, possibly intentional forms. Record each damaged example/table cell as unresolved unless the exact reading is uniquely supported inside the repository; do not import equivalent examples from another grammar. | `unresolved` |
| N15-R020 | 1,401-1,500 | Demonstrative, personal, subject, object, and reflexive pronoun material is extensively interleaved; multiple paradigms have lost labels and cells / fatal | Same file lines 1,401-1,500; `resources/naxwe/04-sarfaha-magacuyaallada.md`; dictionary entries for pronoun forms. These corroborate individual terms but not the original paradigm order. | Keep all damaged paradigms and examples unresolved. Preserve only clear section starts as structural candidates; do not fill missing forms from related grammars or dictionaries. | `unresolved` with possible `structural-only` boundaries |
| N15-R021 | 1,501-1,600 | Possessive and locative-pronoun sections contain interleaved prose, missing paradigm cells, OCR damage, and partly merged example series / fatal | Same file lines 1,501-1,600; later same-file locative-particle material; `resources/naxwe/04-sarfaha-magacuyaallada.md`; relevant dictionary entries. | Leave damaged paradigms and examples unresolved. Individual clear expressions may become repository-supported candidates only when the same reading has an unambiguous internal match. | `unresolved` |
| N15-R022 | 1,601-1,700 | Compound locative-pronoun paradigms and their example sentences have severe OCR corruption, duplicated fragments, and broken alignment between forms and analyses / fatal | Same file lines 1,601-1,700; later same-file locative-particle discussion; related grammar resources. | Retain these paradigms and examples as unresolved. Do not regularize forms or recreate alignments from similar material elsewhere in the repository. | `unresolved` |
| N15-R023 | 1,701-1,800 | The verb chapter begins inside residual locative-pronoun fragments; its overview and root-extension material are interleaved with damaged tables and examples / fatal | Same file lines 1,701-1,800; later same-file verb-section anchors; `resources/naxwe/07-sarfaha-falalka.md` and `08-hogatuska-baradigmaha-falalka.md`. | Record the verb and root-extension headings as structural candidates only. Keep prose, terminology, examples, and paradigm fragments unresolved unless exact internal matches are found. | `unresolved` with possible `structural-only` boundaries |
| N15-R024 | 1,801-1,900 | Root-extension and person-marking discussion has merged columns, incomplete paradigms, and damaged linguistic notation / fatal | Same file lines 1,801-1,900; later same-file person-marking material; related verb resources. | Do not reconstruct paradigms from related grammars. Retain the unit as unresolved while preserving individually clear, unmodified text. | `unresolved` |
| N15-R025 | 1,901-2,000 | Person and number sections combine corrupted paradigms, broken tables, and overlapping explanatory prose / fatal | Same file lines 1,901-2,000; related verb resources; same-file occurrences of person and number headings. | Keep the affected paradigms unresolved. A structural repair may separate clear heading boundaries only after audit approval. | `unresolved` with possible `structural-only` boundaries |
| N15-R026 | 2,001-2,100 | Number and simple-tense material is partially legible but has interleaved example sets, dropped cells, and broken prose order / fatal | Same file lines 2,001-2,100; later same-file tense sections; related verb resources. | Do not normalize or infer tense examples. Record only exact, repository-supported token corrections in later detailed findings. | `unresolved` |
| N15-R027 | 2,101-2,200 | Compound and complex-tense classifications, tables, and examples are mixed together and include OCR-damaged labels and forms / fatal | Same file lines 2,101-2,200; later same-file tense anchors; related verb resources. | Preserve classification boundaries as possible structural candidates; leave the tables, forms, and prose unresolved without unique internal support. | `unresolved` with possible `structural-only` boundaries |
| N15-R028 | 2,201-2,300 | Tense summary, conditional examples, and interrogative material are interleaved with broken table fragments and OCR-damaged tokens / fatal | Same file lines 2,201-2,300; later same-file question and status sections; related verb resources. | Keep examples and classifications unresolved. Do not replace corrupted forms with semantically expected ones without an exact internal match. | `unresolved` |
| N15-R029 | 2,301-2,400 | Negative-interrogative material has merged question sets, incomplete contrast tables, and damaged forms / fatal | Same file lines 2,301-2,400; later same-file negative and question material; related verb resources. | Leave the form-to-analysis relationships unresolved. Preserve unusual readable forms unless the repository uniquely proves OCR damage. | `unresolved` |
| N15-R030 | 2,401-2,500 | Interrogative, affirmative, and negative examples contain overlapping columns, dropped cells, and broken prose / fatal | Same file lines 2,401-2,500; later same-file affirmative and negative subsections; related verb resources. | Do not rebuild examples or tables from regularity assumptions. Record only individual, uniquely repository-supported corrections for future approval. | `unresolved` |
| N15-R031 | 2,501-2,600 | Negative paradigms and double-negative discussion are structurally interleaved, with corrupted labels, forms, and example alignment / fatal | Same file lines 2,501-2,600; same-file negative and person-marking sections; related verb resources. | Retain paradigms and prose as unresolved. A later cleanup may separate clear subsection boundaries only if approved. | `unresolved` with possible `structural-only` boundaries |
| N15-R032 | 2,601-2,700 | Mood material (double negative, imperative, declarative, and subjunctive) is merged across columns and includes incomplete paradigms and examples / fatal | Same file lines 2,601-2,700; later same-file mood and aspect material; related verb resources. | Preserve the readable section labels as structural candidates; keep all incomplete tables, forms, and prose unresolved. | `unresolved` with possible `structural-only` boundaries |
| N15-R033 | 2,701-2,800 | Subjunctive/inchoative and aspect material has damaged paradigm rows, broken prose, and interleaved explanatory fragments / fatal | Same file lines 2,701-2,800; later same-file aspect material; related verb resources. | Leave paradigms and prose unresolved; preserve only unambiguous heading boundaries as possible structural repairs. | `unresolved` with possible `structural-only` boundaries |
| N15-R034 | 2,801-2,900 | Aspect, voice, and verb-classification material is mixed with the beginning of the particle section, creating unreliable boundaries and damaged examples / fatal | Same file lines 2,801-2,900; later same-file particle sections; related grammar resources. | Audit boundaries separately and leave examples/prose unresolved. Do not move text based only on thematic expectation. | `unresolved` with possible `structural-only` boundaries |
| N15-R035 | 2,901-3,000 | Particle subsections are more legible but still contain damaged examples, stray prefixes, and local reading-order failures / high | Same file lines 2,901-3,000; earlier same-file particle references; related grammar resources. | Retain readable forms and flag only uniquely supported word corrections. Keep damaged examples and uncertain order unresolved. | `unresolved` |
| N15-R036 | 3,001-3,100 | Sound-change section contains collapsed analytical tables, malformed symbols, and interleaved column text / fatal | Same file lines 3,001-3,100; `resources/dhawaaq/` and related grammar material. | Do not recreate phonological tables or normalize notation without exact internal support. | `unresolved` |
| N15-R037 | 3,101-3,200 | Sound-change material continues with corrupted rule notation and examples, then overlaps the start of word-formation content / fatal | Same file lines 3,101-3,200; later same-file word-formation material; related morphology resources. | Keep the transition as a possible structural boundary; retain rules, examples, and prose as unresolved. | `unresolved` with possible `structural-only` boundaries |
| N15-R038 | 3,201-3,300 | Word-formation and affixation material mixes explanatory prose with collapsed pluralisation tables, damaged references, and interleaved examples / fatal | Same file lines 3,201-3,300; later same-file affixation material; related morphology and grammar resources. | Preserve content as unresolved unless a specific token has one unambiguous repository-supported reading. Do not rebuild tables or citations from related works. | `unresolved` |
| N15-R039 | 3,301-3,400 | Inflectional and derivational affix lists are partly legible but have merged columns, damaged labels, and unreliable example alignment / fatal | Same file lines 3,301-3,400; later same-file derivation material; related morphology resources. | Preserve lists and forms as unresolved unless an individual correction has exact repository support. Do not complete list entries by analogy. | `unresolved` |
| N15-R040 | 3,401-3,500 | Derivational-affix material includes damaged classification tables, overlapping explanatory prose, and corrupted examples / fatal | Same file lines 3,401-3,500; later same-file derivation material; related morphology resources. | Preserve individual readable forms but leave damaged tables, examples, and relationships unresolved without unique repository evidence. | `unresolved` |
| N15-R041 | 3,501-3,600 | Derivational-affix discussion and root-derived-noun examples contain collapsed table columns and detached labels, especially across lines 3,528-3,584 / fatal | Same file lines 3,501-3,600; later same-file derivation material; related morphology resources. | Do not infer pairings or rebuild prose. Preserve the material as an unresolved reconstruction unit. | `unresolved` |
| N15-R042 | 3,601-3,700 | The opening of `Jibaarid` (compounding) retains a partial topic sequence but interleaves examples, prose fragments, and broken rows / fatal | Same file lines 3,601-3,700; later same-file compounding material; related morphology resources. | Do not reconstruct definitions or examples from thematic expectation. Preserve the span as unresolved. | `unresolved` |
| N15-R043 | 3,701-3,800 | Compound classifications and examples are crossed with one another, including dense collapsed tabular material around lines 3,761-3,779 / fatal | Same file lines 3,701-3,800; later same-file compounding material; related morphology resources. | Neither the original column layout nor every example association is recoverable; retain the unit as unresolved. | `unresolved` |
| N15-R044 | 3,801-3,900 | M+F and Q+Q compound descriptions mix explanatory prose, examples, and locative constructions in an indeterminate order / fatal | Same file lines 3,801-3,900; later same-file locative material; related grammar resources. | Do not normalize the content beyond any separately approved structural boundary. Preserve wording and examples as unresolved. | `unresolved` |
| N15-R045 | 3,901-4,000 | MQM/MQF compound examples remain interleaved, and the transition into `1.4 Goldooxid` is affected by table and column collapse / fatal | Same file lines 3,901-4,000; later same-file word-formation material; related morphology resources. | The visible topic transition may support a future structural boundary, but its wording and sequence remain unresolved. | `unresolved` with possible `structural-only` boundary |
| N15-R046 | 4,001-4,100 | The acronym section begins relatively legibly but then becomes crossed with the following external-growth material. Its examples and category labels contain local OCR damage and uncertain reading order / fatal | Same file lines 4,001-4,100; later same-file word-formation material; repository terminology searches. | Preserve the apparent topic boundary only as a structural candidate. Do not normalize acronym forms, expansions, or explanatory prose without unique internal support. | `unresolved` with possible `structural-only` boundary |
| N15-R047 | 4,101-4,200 | External-growth, borrowing, translation, and sound-change discussion is column-interleaved, with damaged terminology and detached example lists / fatal | Same file lines 4,101-4,200; related morphology, phonology, and terminology resources. | Do not reassemble the prose or examples from related works. Retain the span as unresolved. | `unresolved` |
| N15-R048 | 4,201-4,300 | The start of dialect-unification material overlaps preceding text, and the explanation of dialect formation is crossed with another page column / fatal | Same file lines 4,201-4,300; later same-file dialect headings; related grammar resources. | Retain the visible dialect topic and subsection start as potential structural boundaries, but leave prose, examples, and order unresolved. | `unresolved` with possible `structural-only` boundaries |
| N15-R049 | 4,301-4,400 | Dialect classifications, grammatical-level headings, and explanatory material have lost their original column order, despite some individually legible list entries / fatal | Same file lines 4,301-4,400; later same-file dialect material; repository searches for listed terminology. | Do not complete the classification or recreate the explanatory sequence. Preserve readable entries and mark the unit unresolved. | `unresolved` |
| N15-R050 | 4,401-4,500 | Spoken-versus-written dialect examples culminate in a severely collapsed analytical table (lines 4,403-4,445), where rules, examples, and columns no longer align / fatal | Same file lines 4,401-4,500; later same-file dialect material; related grammar resources. | Keep the table and its analyses unresolved. A future cleanup may isolate a clear heading boundary but must not infer table cells or associations. | `unresolved` with possible `structural-only` boundary |
| N15-R051 | 4,501-4,600 | The dialect-level sound analysis begins with crossed prose and an extensively collapsed phonetic-classification table (lines 4,530-4,554) / fatal | Same file lines 4,501-4,600; repository phonology resources and terminology lists. | Do not recover table structure, symbols, or classifications by analogy. Preserve the material as unresolved. | `unresolved` |
| N15-R052 | 4,601-4,700 | Phonological comparison, source-word discussion, and dialect examples are interleaved with broken columns and malformed notation / fatal | Same file lines 4,601-4,700; related phonology resources; same-file dialect headings. | Preserve any clearly readable text unchanged; leave arguments, examples, and notation unresolved without exact internal support. | `unresolved` |
| N15-R053 | 4,701-4,800 | The grammatical-level dialect section begins amid residual crossed columns. Its paradigms and explanatory prose lose reliable order and some example pairings / fatal | Same file lines 4,701-4,800; later same-file negative-particle material; related grammar resources. | Keep the visible section boundary as a possible structural candidate, but do not reconstruct its paradigms or prose. | `unresolved` with possible `structural-only` boundary |
| N15-R054 | 4,801-4,900 | Negative-particle and dialect examples continue through damaged alignments, duplicated material, and overprinted page fragments / fatal | Same file lines 4,801-4,900; same-file grammar material; related verb resources. | Do not regularize forms or complete comparisons. Retain the span as unresolved. | `unresolved` |
| N15-R055 | 4,901-5,000 | The apparent glossary begins at line 4,895, but its Somali-English entries include OCR debris, broken labels, and uncertain pairings; it follows residual damaged dialect prose / fatal | Same file lines 4,901-5,000; repository terminology/glossary files and related grammar resources. | Do not normalize or fill glossary pairs from other glossaries. Retain entries as unresolved unless each exact pairing has unique internal support. | `unresolved` |
| N15-R056 | 5,001-5,100 | The glossary continues with malformed terms and pairings, then runs directly into a badly interleaved bibliography whose author, title, year, and publication fields have lost their original associations / fatal | Same file lines 5,001-5,100; repository terminology resources; same-file bibliography material. | Preserve the glossary and bibliography as separate unresolved reconstruction units. Do not correct translation pairs or bibliographic citations from outside sources. | `unresolved` with possible `structural-only` boundary |
| N15-R057 | 5,101-5,191 | The closing bibliography is extensively column-interleaved and truncated; individual author names, dates, titles, and publication details cannot be reliably re-associated / fatal | Same file lines 5,101-5,191; repository-wide searches for distinctive citations and author names. | Do not reconstruct or normalize citations without exact internal support. Preserve the final bibliography span as unresolved. | `unresolved` |

## Proposed structural blueprint

The audit establishes that a future cleanup must be conservative:

- Preserve unresolved prose, examples, tables, glossary entries, and bibliography
  spans unless an approved finding supplies a unique repository-supported reading.
- Isolate only approved, clearly visible major-topic boundaries as structural-only
  repairs; do not use topic familiarity to reorder text.
- Do not recreate table cells, paradigm alignments, glossary pairs, or citations
  where the original columns or associations are lost.
- Make no wording correction, deletion, or regularization until the maintainer
  approves its specific finding ID.

## Audit approval

- **Audit approval:** approved by the maintainer on 2026-08-12 with the
  instruction, "now go ahead and fix."
- **Approved finding IDs:** N15-R001 through N15-R057, limited to each
  finding's recorded proposed action and evidence label.
- **Deferred finding IDs:** unresolved content within N15-R001 through
  N15-R057 remains deferred; approval did not convert unresolved readings into
  supported corrections.

## Cleanup result and review

Cleanup was applied on 2026-08-12. It implements the approved structural-only
parts of the audit while preserving all unresolved source content.

Applied changes:

- converted the duplicated title-page material into one metadata block and
  removed the two specifically approved debris characters on original lines 7
  and 24;
- separated repository-confirmed chapter boundaries for chapters 1 through 9;
- restored chapter 6 and chapter 8 titles from the same-file contents and
  matching body context;
- restored the chapter 9 title from repeated same-file references to
  `midaynta lahjadaha` and its numbered subsections;
- normalized confirmed subsection headings to a subordinate Markdown level;
- demoted the false `B Bobqb` heading without changing its text; and
- separated the confirmed `A. Ereybixin` and `B. Raadraac` appendix boundaries.

Deferred without change:

- all uncertain prose order, damaged words, paradigms, examples, and table
  cells;
- all uncertain glossary pairs; and
- all damaged bibliography associations and incomplete citations.

Validation:

- `git diff --check`: passed;
- one H1 remains, with confirmed chapters and appendices at H2 and confirmed
  subsections at H3;
- post-cleanup size: 5,263 lines; 42,590 words; 242,747 bytes; and
- substantive debris and title-boundary changes are recorded in
  `data/provenance/correction-log.tsv`.

- **Cleanup approval:** approved by the maintainer on 2026-08-12
- **Complete:** superseded by the post-approval SLS-native rewrite below

## Post-approval SLS-native rewrite

After the first cleanup approval, the maintainer changed the required output
model: resource files must contain usable SLS topic content rather than preserve
the organization or wording of a source book or PDF. This instruction
supersedes the conservative structural blueprint above for the final form of
this file.

The rewritten resource:

- removes title pages, author and institutional material, contents pages,
  acknowledgements, introduction, pagination, scan furniture, glossary
  duplication, and the damaged bibliography;
- reorganizes usable linguistic material into nine SLS topics: word structure,
  word classes, nouns and pronouns, verbs, particles, morphophonology, word
  formation, dialect variation, and related SLS resources;
- replaces damaged or duplicated book exposition with concise Somali reference
  prose and canonical repository links;
- retains the distinctive useful concepts `salkordhin`, `salguurin`,
  `lifaagayn`, `jibaarid`, `goldooxid`, external word growth, and dialect-level
  analysis; and
- does not carry forward unrecoverable tables, bibliography associations, or
  OCR fragments merely to preserve source-page order.

Rewrite validation:

- `git diff --check`: passed;
- one H1, nine H2 topic sections, and fifteen H3 subsections;
- 381 lines; 2,090 words; 12,655 bytes;
- all local Markdown link targets exist; and
- the rewrite is recorded as `N15-SLS001` in the correction log.

- **SLS rewrite approval:** approved 2026-08-23
- **Final complete:** yes
