# Audit record — Sheekooyin Soomaaliyeed

- **Resource path:** `resources/suugaan/04-sheekooyin-soomaaliyeed.md`
- **Collection / family:** suugaan / sheekooyin
- **Priority:** P1
- **Method:** whole-file, line-by-line literary-content audit; repository
  comparison plus source-location check
- **Audit status:** approved; source-guided cleanup applied and awaiting
  maintainer review
- **Audit started:** 2026-08-13
- **File size at audit start:** 1,393 lines; 12,244 words; 72,593 bytes
- **Resource SHA-256 at audit start:**
  `f26f6530ba5e8076beac64820ebac1f45ad8ddbbf5362a46a883dca5798ecfff`
- **Resource-text changes during audit:** none

## Target output model

The cleaned file must be a readable, source-faithful SLS literary resource,
not a topical rewrite or summary of the book. Unlike a grammar reference, a
story collection depends on the author's sequence, narration, dialogue,
repetition, wording, historical spelling, verse lines, and stated morals.
Those features must be preserved unless the source scan proves that the
current text is OCR damage.

The final file should contain the Somali `Arar` and all twenty-seven stories
in source order. It may repair headings, paragraphs, dialogue, verse layout,
line-wrap hyphenation, page-column collisions, and source-verified OCR errors.
It must not modernize the prose, silently replace unusual vocabulary, shorten
stories, rewrite culturally or historically marked content, introduce new
morals, or complete missing text from narrative expectation.

## Source and repository evidence

The active source registry identifies the work as *Sheekooyin Soomaaliyeed* by
Muuse Cumar Islaam, published in 1973. That attribution and year are supported
by two external catalogue records located during the audit:

- Roma Tre ArcAdiA, handle `2307/845`: 48-page PDF, Mogadishu, 1973, with the
  Ministry of Culture and Higher Education recorded as editor; and
- Indiana University pageturner `VAA3506`: a 51-image scanned-resource record.

The Roma Tre scan was acquired during cleanup from the public ArcAdiA record
and opened page by page. It is an image-only, 49-image PDF whose printed text
runs from PDF page 3 through PDF page 48. PDF pages 47-48 provide the source
contents list; they confirm the `Arar`, all twenty-seven stories, their order,
and the title of story 22, `Nin indhoole ah`. The scan is now the controlling
evidence for reading order, titles, punctuation, and damaged words.

Repository comparison included:

- the complete current resource and all same-file repetitions;
- `resources/suugaan/00-sources.md` and `resources/suugaan/README.md`;
- Git history for this path, which contains no earlier or cleaner version;
- exact-phrase and title searches across `resources/` and `source-evidence/`;
  and
- dictionary and wordlist resources as possible corroboration only, not as
  authority to rewrite source wording.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-24 | reviewed | SUU04-R001 |
| 25-106 | reviewed | SUU04-R002 |
| 107-195 | reviewed | SUU04-R003 |
| 196-298 | reviewed | SUU04-R004 |
| 299-404 | reviewed | SUU04-R005 |
| 405-650 | reviewed | SUU04-R006 |
| 651-684 | reviewed | SUU04-R007 |
| 685-798 | reviewed | SUU04-R008 |
| 799-959 | reviewed | SUU04-R009 |
| 960-1,043 | reviewed | SUU04-R010 |
| 1,044-1,101 | reviewed | SUU04-R011 |
| 1,102-1,155 | reviewed | SUU04-R012 |
| 1,156-1,200 | reviewed | SUU04-R013 |
| 1,201-1,326 | reviewed | SUU04-R014 |
| 1,327-1,393 | reviewed | SUU04-R015 |

## Structural inventory

The file contains one H1, an `Arar`, and a numbered sequence ending at story
27. The intended twenty-seven-story structure remains inferable, but the
Markdown exposes only eleven valid story-number headings and seven separate
story-title headings. Most other boundaries survive only as OCR-damaged inline
text.

This was the working title inventory at audit time. All titles in the table
were subsequently verified against the scan during cleanup:

| No. | Recoverable title | Current structural state |
| ---: | --- | --- |
| 1 | Shimbir iyo Nin | Number heading valid; title damaged inline |
| 2 | Nin Cirweyn | Number and title headings valid |
| 3 | Abees iyo Dabagaalle | Number and title damaged inline |
| 4 | Libaax iyo Saddex Dibi | Number and title headings valid |
| 5 | Wan iyo Orgi | Number and title merged with prior column |
| 6 | Dhagar Haween | Number and title damaged inline |
| 7 | Laba Saaxiib Ahaan Jirey | Number valid; title heading has OCR damage |
| 8 | Bisadda | Number and title damaged inline |
| 9 | Laba Kala Daran | Number and title damaged inline |
| 10 | Faarax iyo Faadumo | Number valid; title damaged inline |
| 11 | Gacmaha | Number and title damaged inline |
| 12 | Libaax | Number valid; title inline |
| 13 | Cigaal Shiidaad | Number and title damaged inline |
| 14 | Kooradadab iyo Geenyadiisii | Number and title damaged inline |
| 15 | Walaalo Isjecel | Number damaged inline; title heading partly damaged |
| 16 | Saddex Cabdi | Number and title headings valid |
| 17 | Raage Ugaas | Number and title damaged inline |
| 18 | Nin Socdaalay | Number inline; title heading valid |
| 19 | Bakayle iyo Digiiran | Number and title damaged inline |
| 20 | Riyaha | Number valid; title damaged inline |
| 21 | Garaad Axmed iyo Shariif | Number and title damaged inline |
| 22 | Nin Indhoole Ah | Number and title verified on source page 38 |
| 23 | Nin Fuley Ah | Number valid; title damaged inline |
| 24 | Qool iyo Hangool | Number valid; title inline |
| 25 | Diin iyo Dawaco | Number and title damaged inline |
| 26 | Siciid Saaweel iyo Muun Xaad | Number valid; title inline |
| 27 | Wiil iyo Wiil Waal | Number damaged inline; title heading damaged |

The spellings in this table are now source-verified readings. Their former
structural-state descriptions record the OCR baseline rather than the cleaned
file.

## Findings

| ID | Lines | Class / severity | Evidence searched | Proposed action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| SUU04-R001 | 1-24 | The `Arar` is present but severely OCR-damaged: words are split or merged, letters are substituted, printed line endings are retained, and non-text glyphs enter the prose / fatal | Same-file title and vocabulary searches; source registry; repository-wide searches for distinctive introductory phrases; external catalogue abstract confirms the general subject but not the exact wording | Retain the complete Somali `Arar`. Reconstruct its paragraphs and wording only against the scan. Remove only source-confirmed page furniture; do not replace it with a new SLS introduction or paraphrase the author's claims. | `unresolved`; `structural-only` for the visible `Arar` boundary |
| SUU04-R002 | 25-106 | Stories 1-3 have damaged titles, false prefixes, line-wrap hyphens, symbol insertions, broken paragraphs, and a collapsed speaker exchange in story 3 / fatal | Same-file story boundaries and repeated character names; repository-wide title searches; no parallel text found | Restore three source-verified story headings. Rebuild prose paragraphs from page order and render story 3's speaker turns as dialogue without changing their wording. Do not infer damaged words from plot context. | `unresolved`; `structural-only` for confirmed boundaries and dialogue shape |
| SUU04-R003 | 107-195 | Stories 4-5 are crossed by illustration/caption or adjacent-column fragments; story 5's number and title occur inside the tail of story 4; prose and the visible `Tusaale` boundary are damaged / fatal | Same-file narrative continuity and story numbering; exact-title searches; no clean repository parallel | Use the scan to separate story 4, its source moral or `Tusaale`, and story 5. Preserve the moral if it is source text. Remove only verified illustration captions and page furniture; do not silently discard repeated-looking prose. | `unresolved`; `structural-only` for scan-confirmed boundaries |
| SUU04-R004 | 196-298 | Stories 6-7 contain pervasive OCR substitutions, spurious line-leading characters, column fragments, broken sentences, and an embedded page transition; story 6's sequence remains broadly recoverable but individual readings are unsafe / fatal | Same-file plot continuity; repository title and phrase searches; dictionary searches provide isolated words only | Restore story headings and paragraphs page by page. Preserve the full narrative and dialogue. Correct individual tokens only from the scan; dictionary regularity alone is insufficient. | `unresolved` |
| SUU04-R005 | 299-404 | Stories 8-9 have damaged number/title lines, mixed columns, lost paragraph order, and stray illustration/page symbols; story 9's food and milk sequence is partly interleaved / fatal | Same-file sequence and character references; repository-wide title searches; no textual parallel | Re-establish the two story units and printed paragraph order from the scan. Retain every recoverable narrative event and the concluding moral; exclude only source-confirmed non-content. | `unresolved`; `structural-only` for story boundaries |
| SUU04-R006 | 405-650 | Story 10, `Faarax iyo Faadumo`, is the longest and most damaged unit: title corruption, multiple page-column collisions, false marginal prefixes, broken dialogue, damaged refrain-like speech, and interleaving through its conclusion / fatal | Same-file names and repeated phrases; repository-wide searches for Faarax-bari-cadde, Faadumo, Fiqifarey, and distinctive passages; no complete repository parallel | Treat the entire story as a single scan-led reconstruction unit. Preserve its plot, repeated formulas, direct speech, names, and ending. Do not summarize, sanitize, or join fragments by narrative guesswork. Mark any scan-illegible span rather than inventing prose. | `unresolved` |
| SUU04-R007 | 651-684 | Story 11, `Gacmaha`, has a damaged story number and title; the verse exchange between `MIDIG` and `BIDIX` has line contamination from adjacent prose, especially lines 676-680 / fatal | Same-file repeated verse line and speaker labels; poetry and terminology resources confirm verse formatting conventions but are not textual parallels | Verify every verse line and its order against the scan. Format the two speaker sections as verse while preserving capitalization only as needed for readable speaker labels. Do not regularize metre or supply missing words. | `unresolved`; `structural-only` for source-confirmed verse layout |
| SUU04-R008 | 685-798 | Stories 12-14 contain damaged prose, missing paragraph breaks, false prefixes, column collisions, and corrupted number/title lines for stories 13-14; story 14 includes an identity-name contrast that must remain exact / fatal | Same-file names and story sequence; repository-wide searches for `Cigaal Shiidaad`, `Kooradadab`, and `Dambas Jiif`; no complete parallel | Restore three headings and source paragraph order. Preserve names, direct speech, and source-specific spellings after scan verification. Remove marginal/page debris only where the scan proves it is not content. | `unresolved` |
| SUU04-R009 | 799-959 | Stories 15-16 have page-column material inserted into prose, damaged title spelling, false prefixes, long merged lines, and extensive corruption in story 16's evidentiary dialogue and inheritance judgment / fatal | Same-file character and plot continuity; repository-wide searches for titles and distinctive expressions; no clean parallel | Rebuild both stories against the scan. Preserve the story 16 reasoning sequence and judgments as source content rather than rewriting them into modern explanatory prose. No wording should be normalized merely because it is culturally or lexically unusual. | `unresolved` |
| SUU04-R010 | 960-1,043 | Stories 17-19 have damaged number/title lines, dense column interleaving, digit-for-letter substitutions, false page fragments, and broken coded messages in stories 17-18; story 19's battle sequence is compressed and partly lost / fatal | Same-file names and message structure; title/phrase searches across resources; no earlier Git version or complete parallel | Verify the coded messages, quantities, title spellings, and story endings directly from the scan. Preserve the messages exactly because their wording drives the stories' meanings. Do not reconstruct them from expected answers. | `unresolved` |
| SUU04-R011 | 1,044-1,101 | Stories 20-21 have damaged titles, word substitutions, inserted column fragments, and a three-line `masafo` whose lines are contaminated by adjacent prose / fatal | Same-file story sequence and speaker names; searches for the poem's distinctive phrases; no complete repository match | Restore the two headings and verify all prose and verse lines from the scan. Format the `masafo` as verse; retain its historical/religious wording and the story's response without editorial rewriting. | `unresolved`; `structural-only` for verse formatting |
| SUU04-R012 | 1,102-1,155 | Story 22 begins without a surviving number or title and is among the most visually contaminated spans; illustration or second-column glyphs occur on nearly every line, although the narrative about repeated filial abandonment is discernible / fatal | Same-file sequence between stories 21 and 23; repository-wide exact-phrase and theme searches; no matching title or clean parallel found | Recover the number, exact title, paragraphs, and wording only from the scan. Do not assign an inferred title such as `Waalid caasi`; the current evidence supports the topic but not the source title. | `unresolved` |
| SUU04-R013 | 1,156-1,200 | Story 23 has a valid number heading but a corrupted inline title, false marginal prefixes, broken words, and scattered page/illustration debris / high | Same-file plot continuity; repository-wide title and phrase searches | Verify the title and prose against the scan, restore normal paragraphs, and remove only confirmed non-text material. Preserve the source's fear dialogue and conclusion. | `unresolved` |
| SUU04-R014 | 1,201-1,326 | Stories 24-25 contain corrupted titles, broken dialogue and enumerated animal speeches, column fragments, symbols, and damaged paragraph order; story 24's five-item speech sequence and story 25's repeated `caadadeyda` exchange require exact wording / fatal | Same-file numbering, speaker sequence, and repeated formula; repository-wide title searches; no complete parallel | Reconstruct both stories scan-first. Format story 24's numbered speeches as a list only if the source visibly numbers them. Preserve the repeated formula and all speaker associations; do not regularize animal names or supply damaged phrases by analogy. | `unresolved`; `structural-only` for source-confirmed list shape |
| SUU04-R015 | 1,327-1,393 | Stories 26-27 have damaged inline titles, letter substitutions, broken place names, false prefixes, and corrupt final dialogue; story 27's title and five-part wordplay depend on exact lexical forms / fatal | Same-file place/name repetitions; repository searches for `Siciid Saaweel`, `Muun Xaad`, `Wiil Waal`, and the final lexical sequence; no full parallel | Verify both stories, place names, names, questions, and answers from the scan. Preserve the wordplay and historical wording exactly. Keep the collection ending after story 27 without adding a new conclusion. | `unresolved` |

## Cross-file and collection findings

1. The source registry's author and 1973 year are supported; no metadata
   correction is proposed during this audit.
2. The current file is byte-identical to the only version in Git history, so
   Git cannot recover the damaged readings.
3. No local source scan or full textual parallel was found. Related dictionary,
   grammar, and literature files may corroborate isolated words but cannot
   establish this book's sentence wording or narrative order.
4. The file contains very long lines up to 327 characters, but line length
   alone is not a deletion criterion: several long lines contain two columns
   merged together and must be separated against the scan.
5. Exact nonempty-line comparison found no substantive duplicated paragraphs.
   Apparent repetitions must therefore be checked as possible storytelling
   formulas rather than removed automatically.
6. Culturally marked material—including violence, cannibalism, gendered
   statements, disability terms, religious language, and historical social
   judgments—is source content. Cleanup does not imply SLS endorsement, but it
   must not silently sanitize or modernize the stories.

## Proposed source-faithful blueprint

The cleaned file should use this hierarchy:

1. `# Sheekooyin Soomaaliyeed`;
2. `## Arar`, retaining the source introduction;
3. one H2 for each source-numbered story in the form
   `## Sheekada 1aad — Shimbir iyo Nin`, after number and title verification;
4. ordinary prose paragraphs in source order;
5. direct speech separated into readable paragraphs without changing words;
6. verse retained line for line, with speaker labels for story 11 and a verse
   block for story 21;
7. numbered or speaker lists only when the scan confirms that presentation;
   and
8. source morals or `Tusaale` passages retained within their stories, with a
   subordinate heading only where the printed source supplies one.

The cleanup must:

- inspect the scan page by page and map every page to the current OCR span;
- recover all 27 story boundaries and verify every title;
- reconstruct reading order before attempting word-level corrections;
- remove page numbers, running furniture, illustration fragments, and scan
  symbols only after identifying them on the page;
- rejoin words split solely by printed line wrapping;
- preserve genuine source hyphens, apostrophes, punctuation choices, names,
  dialectal forms, historical spellings, repetition, and oral-story formulas;
- record every substantive textual correction with page evidence in the
  provenance log; and
- leave an explicit unresolved marker or omit only the affected fragment when
  the scan itself cannot support a reading.

The cleanup must not:

- convert the stories into summaries or an SLS-authored retelling;
- import prose from similar folktales or other editions without attribution;
- infer story 22's title from its moral;
- alter plot, dialogue, verse, speaker attribution, quantities, proper names,
  or culturally marked wording for stylistic consistency; or
- treat dictionaries, modern spelling frequency, or grammatical expectation
  as sufficient evidence to overwrite the source.

## Audit approval

- **Audit approval:** approved by the maintainer on 2026-08-13 with the
  instruction, "go ahead."
- **Approved finding IDs:** SUU04-R001 through SUU04-R015
- **Resolved during cleanup:** word-level readings, verse lines, speaker
  associations, page-column order, and story 22's title were checked directly
  against the source scan.

## Cleanup result and review

- **Cleanup:** applied. Source pages 3-46 were checked directly, and source
  contents pages 47-48 were used to confirm the complete inventory. The `Arar`
  and all twenty-seven stories now have source-verified headings, reading
  order, paragraphs, line-wrap joins, dialogue, verse, lists, quantities,
  names, place names, and conclusions. Page furniture, illustration fragments,
  column collisions, and OCR-only symbols were removed. No story was
  summarized or silently replaced with an automatic OCR transcription.
- **Content retained:** story 4's printed `Tusaale`; story 10's five parts and
  refrain; story 11's two speakers and every geeraar line; story 14's
  `Kooradadab`/`Dambas Jiif` contrast; story 16's complete evidentiary and
  inheritance judgment; stories 17-18's coded messages; story 19's battle
  sequence; story 21's three-line masafo; story 24's five animal speeches; and
  story 27's five-part lexical wordplay.
- **Cleanup resource size:** 478 lines; 10,408 words; 66,288 bytes
- **Cleanup SHA-256:**
  `5073341cefae85d071c17c461ba389da956d1556c490c21e5462c1bece822eee`
- **Cleanup approval:** pending
- **Complete:** no
