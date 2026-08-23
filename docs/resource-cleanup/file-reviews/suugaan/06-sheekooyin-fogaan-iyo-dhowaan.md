# Audit record — Sheekooyin fogaan iyo dhowaan ba leh

- **Resource path:** `resources/suugaan/06-sheekooyin-fogaan-iyo-dhowaan.md`
- **Collection / family:** suugaan / sheekooyin
- **Priority:** P2
- **Method:** whole-file, line-by-line literary-content audit; direct comparison
  with the local source scan
- **Audit status:** approved; source-guided cleanup applied, awaiting maintainer review
- **Audit date:** 2026-08-13
- **File size at audit:** 1,608 lines; 11,984 words; 77,947 bytes
- **Resource SHA-256:**
  `ef36d0622dc8b3d2e513dcdbc11225052a8181ab152f6268dc9bc7ad8a96851a`
- **Resource-text changes during audit:** none

## Target output model

The cleaned file must remain a complete, source-faithful literary collection.
It must preserve the author's `Arar`, all twenty-one narratives, their order,
dialogue, verse, proverbs, repetition, names, historical vocabulary, and
endings. Cleanup may restore headings, paragraphs, dialogue and verse layout,
join words split only by printed line wrapping, and remove verified page
numbers and OCR debris. It must not summarize, modernize, sanitize, retell, or
silently complete damaged passages from narrative expectation.

This source predates the official 1972 Somali orthography and uses an older
Latin writing system, including `ch` where modern Somali commonly uses `x`.
Those forms are source text, not OCR errors. The cleanup must distinguish them
from genuine recognition damage such as `00` for `oo`, digits for letters,
symbol insertions, fused page numbers, and corrupted headings.

## Controlling source and metadata finding

The controlling source is the local image-only PDF:

`temp/soomaali/literature/Sheekooyin fogaan iyo dhowaan ba leh.pdf`

The scan contains 51 PDF pages. Its cover reads *Sheekooyin fogaan iyo dhowaan
ba leh* and credits `Mohamed Farah Abdillahi`; the active source registry gives
the standardized Somali form `Maxamed Faarax Cabdillaahi`. The scan itself is
undated. A secondary scholarly bibliography describes the work as circa 1967,
but that estimate should be recorded as secondary evidence rather than silently
presented as a date printed in the book.

The source layout is:

| PDF pages | Printed matter |
| ---: | --- |
| 1 | Cover |
| 2 | Blank or scan furniture |
| 3 | `Arar` |
| 4 | Contents |
| 5-49 | Literary text, printed pages 1-45 |
| 50-51 | `Raadguro`, printed pages 46-47 |

The `Raadguro` is a bibliography of external publications, not another
narrative. It belongs in this audit's provenance record and need not be copied
into the reader-facing literary resource. The cover, contents page, page
numbers, and publication furniture likewise should not interrupt the stories.
The author's substantive `Arar`, however, must remain.

## Structural inventory

The source contents page identifies twenty-one narrative units after the
`Arar`. The current Markdown has only five headings in total: one H1, one
`Hordhac` H2, and three story H2s. Eighteen story titles survive only as damaged
inline OCR or are absent entirely.

The source inventory is:

| No. | Source title | Printed pages | Current structural state |
| ---: | --- | ---: | --- |
| 1 | Tuugii Faraska Chaday | 1-2 | H2 present, but `Tuugii` is misread as `Muugii` |
| 2 | Wiilwaal iyo Wadaadodii Dhiiranaa | 3-4 | Damaged inline title |
| 3 | Gabadhii Rag Jereebtay | 5-7 | Inline title; OCR damaged |
| 4 | Dawacadii iyo Libaachii Daawaystey | 8 | Title absent; story text catastrophically corrupted |
| 5 | Naagtii Shaarubo Libaach Soo Goysey | 9-10 | Inline title; OCR damaged |
| 6 | Bohol Hadimo | 11-13 | Inline title; page number fused into prose |
| 7 | Dhiib iyo Jacayl | 14-17 | Title heavily corrupted as `DUZIF IYO gAvaYb` |
| 8 | Dhalin Ileeye iyo Gorayo Raran | 18 | Title heavily corrupted |
| 9 | Dawadii Shilayska | 19-21 | Title heavily corrupted; page number fused |
| 10 | Dagaalyahankii Wiilwaal | 22 | Title damaged inline |
| 11 | Wiilwaal iyo Inankuu Abtiga u Ahaa | 23 | Title damaged inline |
| 12 | Wiilwaal iyo Min-yarodiisii | 24-25 | Title damaged inline |
| 13 | Ninkii Hindisada Badnaa | 26-27 | H2 present; body remains damaged |
| 14 | Kabcalaf iyo Huuryo Ugaas | 28-35 | H2 misreads `Huuryo` as `Muuryo` |
| 15 | Sadechdii Ilma-boqor | 36-38 | Title damaged inline |
| 16 | Maahmaaho | 39 | Title damaged inline |
| 17 | Wiil Is Waalaba Waabiyaa Hela | 40-41 | Title damaged inline |
| 18 | Nin Reer Magaal Ah iyo Adhi Miyi | 42 | Title damaged inline |
| 19 | Aabow, Hal Hooyaday Ku Dheh | 42 | Title damaged inline |
| 20 | Shirsoore | 43-44 | Title heavily corrupted |
| 21 | Dani Waa Seeto | 45 | Title damaged inline |

Title spelling in this inventory follows the scan's historical orthography.
Capitalization may be made consistent for Markdown headings, but spelling must
not be modernized merely to match present-day usage.

## Audit coverage and findings

| ID | Original lines | Class / severity | Source evidence and finding | Proposed action |
| --- | ---: | --- | --- | --- |
| SUU06-R001 | 1-1,608 | hierarchy, metadata, and historical qoraal / fatal | Direct scan inventory proves an `Arar`, 21 narratives, and two bibliography pages. The Markdown exposes only three stories as headings. The cover is undated and uses the author's romanized name. Historical `ch` and related spellings are systematic source features. | Use one collection H1, one H2 for the `Arar`, and one H2 for each of the 21 narratives. Keep source and date evidence in the registry/audit. Preserve historical orthography; correct only source-proven OCR errors. |
| SUU06-R002 | 3-25 | `Arar` / high | The complete introductory argument is present, but words are split or merged, punctuation and letters are damaged, and printed line endings remain. | Reconstruct readable paragraphs directly from PDF page 3. Preserve the author's full introduction and claims without replacing them with a new SLS preface. |
| SUU06-R003 | 27-290 | stories 1-4 / fatal | Three headings are missing or damaged. Story 1 begins with `Muugii` instead of source `Tuugii`. Printed pages 1-7 contain substitutions, fused lines, broken dialogue, and page debris. Story 4 on printed page 8 is almost wholly unreadable pseudo-text despite a legible scan. | Reconstruct all four stories page by page, including story 4 from the scan. Restore source titles, narration, dialogue, and endings; do not infer damaged wording from plot alone. |
| SUU06-R004 | 291-609 | stories 5-7 / fatal | Titles and bodies contain letter/digit substitutions, false symbols, broken paragraphs, fused page numbers, and damaged verse or dialogue. Story 7's title is no longer recognizable without the contents page. | Restore three H2s and the complete printed-page order for pages 9-17. Preserve every dialogue and verse line, repetition, historical form, and culturally marked event. |
| SUU06-R005 | 610-954 | stories 8-13 / fatal | Six consecutive titles are damaged inline except the H2 for story 13. OCR confuses letters, case, zeros, punctuation, and page numbers throughout printed pages 18-27. Proverbs, exchanges, and Wiilwaal material are structurally vulnerable. | Reconstruct all six units scan-first; verify names, quantities, speaker turns, proverbs, and endings. Remove only confirmed page furniture and OCR-only characters. |
| SUU06-R006 | 955-1,251 | `Kabcalaf iyo Huuryo Ugaas` / fatal | This eight-page narrative is the longest unit. Its Markdown H2 changes source `Huuryo` to `Muuryo`; the body has pervasive OCR substitutions, fused lines, broken dialogue, and page debris across printed pages 28-35. | Restore the source title and reconstruct the entire story from the scan. Preserve its full narration, exchanges, violence, names, repeated formulas, and ending without abridgment or sanitization. |
| SUU06-R007 | 1,252-1,608 | stories 15-21 and `Maahmaaho` / fatal | Seven titles are missing as headings and are heavily damaged inline. Dense OCR corruption affects prose, verse, proverbial material, punctuation, and page transitions across printed pages 36-45. | Restore all seven H2s and verify every passage directly against the scan. Preserve the complete `Maahmaaho` section, verse/proverb layout where printed, dialogue, names, humour, and final endings. |
| SUU06-R008 | source pages 46-47 | end matter / structural | The two final scan pages are headed `Raadguro` and list bibliographic references. They are not present in the current resource and are not part of the 21-story contents inventory. | Keep the bibliography out of the reader-facing story text, but document it here and use it as provenance evidence where relevant. Do not treat its omission as lost literary content. |

## Cross-file and technical findings

1. The PDF has no text layer. Automatic OCR can help locate passages but
   cannot establish authoritative wording.
2. Git history contains no earlier or cleaner version of this file.
3. The baseline has 43 nonblank lines of four or fewer characters, 46 lines
   containing likely `00`-for-`oo` errors, and 32 lines with obvious symbol
   debris. These counts locate risk; none is an automatic deletion rule.
4. Many printed page numbers are fused to the first or last prose line rather
   than isolated. Each must be separated against the page image before removal.
5. The source scan is sufficiently legible to support a full reconstruction,
   including the one-page fourth story that the current OCR does not preserve
   intelligibly.
6. The collection's violence, death, gendered statements, historical social
   judgments, religious language, and animal treatment are source content.
   Source-faithful cleanup preserves them without implying SLS endorsement.
7. Related dictionaries and modern spelling resources may corroborate a
   reading, but they must not overwrite this work's older writing system or
   sentence wording.

## Proposed source-faithful structure

```text
# Sheekooyin fogaan iyo dhowaan ba leh

## Arar
## Tuugii Faraska Chaday
## Wiilwaal iyo Wadaadodii Dhiiranaa
## Gabadhii Rag Jereebtay
## Dawacadii iyo Libaachii Daawaystey
## Naagtii Shaarubo Libaach Soo Goysey
## Bohol Hadimo
## Dhiib iyo Jacayl
## Dhalin Ileeye iyo Gorayo Raran
## Dawadii Shilayska
## Dagaalyahankii Wiilwaal
## Wiilwaal iyo Inankuu Abtiga u Ahaa
## Wiilwaal iyo Min-yarodiisii
## Ninkii Hindisada Badnaa
## Kabcalaf iyo Huuryo Ugaas
## Sadechdii Ilma-boqor
## Maahmaaho
## Wiil Is Waalaba Waabiyaa Hela
## Nin Reer Magaal Ah iyo Adhi Miyi
## Aabow, Hal Hooyaday Ku Dheh
## Shirsoore
## Dani Waa Seeto
```

`Arar` follows the source label and should replace the editorial `Hordhac`
heading. Story headings retain source spellings, while unnecessary terminal
punctuation and all-capitals may be normalized as presentation rather than
textual change.

## Cleanup gates

Cleanup must:

- inspect PDF pages 3-49 directly and map each printed page to its OCR span;
- retain the complete `Arar` and all 21 stories in the source order;
- verify every title, paragraph, speaker turn, verse line, proverb, name,
  quantity, and ending against the scan;
- restore story 4 from the source rather than attempting to repair its
  meaningless OCR token by token;
- preserve the pre-1972 writing system and historical vocabulary;
- remove only source-confirmed page numbers, cover/contents furniture, and OCR
  debris; and
- record substantive reconstructions and metadata decisions in the provenance
  log.

Cleanup must not:

- modernize `ch` to `x` or otherwise regularize historical spelling without a
  separate approved editorial policy;
- use the damaged OCR line order as authority where the page image differs;
- summarize long stories, collapse repeated formulas, or omit difficult verse;
- invent missing words from metre, grammar, or narrative expectation;
- copy the `Raadguro` into the literary body as if it were a story; or
- mark cleanup approved or complete without maintainer review.

## Approval state

- **Audit approval:** approved by the maintainer on 2026-08-13 with the
  instruction, "go ahead."
- **Approved finding IDs:** SUU06-R001 through SUU06-R008
- **Cleanup:** applied on 2026-08-13
- **Cleanup approval:** pending
- **Complete:** no

## Cleanup result

The approved cleanup restored the source hierarchy as one collection H1, the
author's `Arar`, and twenty-one narrative H2s in the printed contents order.
All title readings were checked against the contents page. The completely
corrupted fourth narrative, `Dawacadii iyo Libaachii Daawaystay`, was
reconstructed from printed page 8; the other narratives retained their full
OCR-transcribed bodies while headings, source-proven substitutions, page
markers, digit-for-letter errors, and symbol debris were repaired.

The book's pre-1972 writing system—including `ch`, `wacha`, `wuchu`, historical
name forms, and related spellings—was deliberately retained. The cover,
contents furniture, printed page numbers, and the two-page `Raadguro` were not
inserted into the reader-facing literary body. The source registry now records
`c. 1967` explicitly as a secondary bibliographic estimate because the scan
itself has no printed date.

- **Cleaned file size:** 1,503 lines; 11,859 words; 77,517 bytes
- **Cleaned resource SHA-256:**
  `de766e242218d7a3962a09b3ddcde3affa0c35357d46e2470e05e7925f8efeae`
- **Structural validation:** one H1, twenty-two H2s (`Arar` plus 21 stories),
  no detected printed-page markers, no `00` OCR substitutions, no lines of four
  or fewer characters, and no detected raw `@`, brace, pipe, backslash, or
  angle-bracket debris
- **Validation state:** cleanup applied; maintainer cleanup review remains
  pending
