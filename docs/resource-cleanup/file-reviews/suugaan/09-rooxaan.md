# Audit record — Rooxaan

- **Resource path:** `resources/suugaan/09-rooxaan.md`
- **Collection / family:** suugaan / sheeko-curis bulsheed
- **Priority:** P2
- **Method:** whole-file literary-content audit; direct comparison with the
  local source scan and its raw OCR conversion
- **Audit status:** approved; source-guided cleanup applied, awaiting maintainer review
- **Audit date:** 2026-08-14
- **File size at audit:** 67 lines; 3,787 words; 24,913 bytes
- **Resource SHA-256:**
  `d1b2ebc5868e8b3c53104c3bb3d070faeadea7fd5bdd33f3103a84ee3c7f2d82`
- **Resource-text changes during audit:** none

## Target output model

The cleaned file should remain a complete, source-faithful edition of Jaalle
Shire J. Axmed's single continuous story. It follows the inquisitive Guhaad
from Gellinsoor to Muqdisho: his unanswered question about trees, experience
at Qur'aan school, dismissal from a café after money is stolen, appeal to
Shiikh Muxsin, and eventual exposure of the staged `rooxaan` performance.

Cleanup may repair scan-proven OCR errors, restore readable paragraph and
dialogue layout, format the opening verse, and remove verified page and
publication furniture. It must not shorten the story, turn it into a modern
grammar lesson, explain its moral inside the authorial text, sanitize its
portrayals of religious fraud, schooling, marriage, or violence, invent
chapters, or silently modernize clearly printed 1973 forms.

## Controlling source and metadata finding

The controlling source is the local image-only PDF:

`temp/soomaali/literature/Rooxaan.pdf`

It contains 18 sequential single-page images and no usable text layer. The two
raw OCR conversions at `temp/md/literature/Rooxaan.md` and
`temp/soomaali/md/literature/Rooxaan.md` are byte-identical navigation aids;
the page images control all corrections.

The scan's structure is:

| PDF pages | Source matter |
| ---: | --- |
| 1 | Illustrated front cover, title, and ministry imprint |
| 2 | Title page: `Rooxaan`, `Waxa qoray: Jaalle shire J. Axmed`, `Xamar 1973` |
| 3 | Four-line verse attributed to Qamaan Bulxan |
| 4-16 | Complete continuous story, printed pages 3-15 |
| 17 | National Printing Press colophon |
| 18 | Back cover and printed price |

The source registry already records `Rooxaan`, Jaalle Shire J. Axmed, and
1973. Those fields agree with the title page. Capitalizing `shire` as `Shire`
in the registry is a metadata presentation choice and does not require adding
the title page to the literary body.

## Source contents and structure

This is one story, not a story collection and not a chaptered work. No
author-supplied internal title or numbered division occurs between the opening
verse and `Dhammaad.` The cleaned Markdown therefore needs one H1, the verse,
and the complete narrative in source order. Editorial section headings would
misrepresent the source and should not be added merely to divide a long text.

The narrative is complete in the current resource:

1. Guhaad asks his parents and Shiikh Saalax why trees are fixed in the soil.
2. He leaves Gellinsoor for Muqdisho and stays with Caasha and Geelle.
3. He attends a Qur'aan school, is badly beaten, and leaves it.
4. He works in Nuur's café, where 50 shillings are stolen during his absence.
5. Caasha directs him to Shiikh Muxsin in Boondheere.
6. Guhaad pays the sheikh and buys the requested incense and perfumes.
7. After two supposed spirit sessions, he returns with a `karbuuno`, exposes
   the strings, boards, tins, voices, and Nadiifo's participation, and recalls
   the closing proverb about light driving out darkness.

No narrative page, scene, conclusion, or `Dhammaad.` is missing. The earlier
mechanical conversion removed scan wrappers and joined printed line wraps, but
it did not yet perform a dependable character-level transcription.

## Current structural and textual condition

The resource has one correct H1 followed by 66 lines of text. Its 58 prose
paragraphs preserve the overall page sequence, but 11 lines exceed 1,000
characters. Long descriptive and dialogue passages are therefore difficult to
review, cite, and maintain even where the words are accurate.

The opening quotation is semantically part of the source, but its Markdown
does not preserve verse line breaks reliably: consecutive blockquote lines
render as one wrapped paragraph unless hard breaks are supplied. It also
retains the OCR reading `Sara Kacaaniaaye`; the scan clearly prints
`Sara Kacaantaaye`. The other printed lines and the Qamaan Bulxan attribution
must be verified directly rather than regularized from modern expectations.

The body contains a mixture of three different things that must be kept
separate during cleanup:

- clear OCR damage, including `00` for `oo`, stray hesitation symbols, broken
  punctuation, letter substitutions, and questionable word joins;
- source-era or authorial forms such as `Muqdishow`, `bogol`, `shago`, and
  `jirey`, which are not automatically errors merely because current standard
  spelling may differ; and
- ordinary source prose whose punctuation and paragraph boundaries can be
  made readable without changing its words or voice.

The story also contains dated and sometimes disturbing content: corporal
punishment, illnesses and spirit-treatment claims, manipulation by religious
figures, polygyny, divorce threats, and descriptions of wives' submission.
These are elements of the 1973 literary work. Cleanup should preserve them as
source content without presenting them as current factual, religious, medical,
or social guidance.

## Audit coverage and findings

| ID | Resource lines / source pages | Class / severity | Source evidence and finding | Proposed action |
| --- | --- | --- | --- | --- |
| SUU09-R001 | lines 1-7; PDF 1-3 | identity, metadata, and epigraph / high | The title and registry metadata agree with the scan. The current resource appropriately omits ministry and title-page furniture but retains the authorial Qamaan Bulxan verse. Its second line is misread as `Sara Kacaaniaaye`, and its Markdown collapses the intended four-line form. | Keep one H1; retain all four verse lines and attribution; correct the second line to the scan-proven `Sara Kacaantaaye`; use explicit Markdown verse breaks. Keep cover, ministry, place/year, and author-credit furniture in metadata/provenance rather than inserting it into the story. |
| SUU09-R002 | lines 9-29; printed pp. 3-6 | Guhaad, Gellinsoor, and Shiikh Saalax / fatal | Guhaad's appearance, curiosity, parents, recurring tree question, and visit to Shiikh Saalax survive in order. The current text includes damaged punctuation and suspicious forms in the sheikh's hesitant answer, while other historical spellings may be genuine. | Reconstruct the opening page by page, preserving characterization, repetition, religious vocabulary, named books, dialogue, hesitation, and the unanswered question. Correct only readings supported by the image; do not rewrite the satire or modernize source forms wholesale. |
| SUU09-R003 | lines 31-33; printed pp. 6-8 | journey, Muqdisho, and Qur'aan school / fatal | Guhaad's hidden journey, reception by Caasha and Geelle, impressions of Muqdisho, year at school, punishment, injury, and withdrawal are all present. Several apparent recognition errors occur inside long flattened paragraphs. | Preserve the complete journey and school account. Repair word joins, letter substitutions, speaker punctuation, and paragraph boundaries from the pages. Do not omit or soften the punishment, religious setting, or the narrator's descriptions. |
| SUU09-R004 | lines 35-39; printed pp. 8-10 | café work, theft, and dismissal / fatal | The three-shilling work, theft of 50 shillings, police report, Nuur's response, dismissal, and conversation with Caasha survive. Forms including `shago`/`shagayn` require source verification and must not be automatically converted merely because `shaqo` is standard today. | Retain the full causal sequence, sums, dialogue, and Guhaad's reaction. Correct only scan-proven OCR errors; separate dialogue and narrative into readable paragraphs without altering who says what. |
| SUU09-R005 | lines 41-51; printed pp. 10-13 | Shiikh Muxsin and his household / fatal | The long descriptions of Muxsin, Cawrala, Nadiifo, their household, previous marriages, coercion, and his public reputation are intact. Dense paragraphs contain ambiguous forms (`bogol`, `naagod`, `nogoto`, `catoodo`, and others) that may combine printed historical spelling with OCR mistakes. | Verify every ambiguous word against the scan and repeated source usage. Preserve names, places, money, family relations, social criticism, threats, and uncomfortable characterization. Do not condense these descriptions or add editorial judgment to the body. |
| SUU09-R006 | lines 53-59; printed pp. 13-15 | preparations and staged spirit sessions / fatal | The purchase of 70 shillings' worth of materials, Guhaad's dream, first interrupted session, second session, and false claim that the money lies beneath his pillow all survive. The source's sensory vocabulary and deliberate repetition are central to the deception. | Retain the entire sequence and its repetitions. Restore paragraph and dialogue boundaries; verify specialized terms such as `cuud`, `jaawe`, `catar`, and the hut descriptions directly from the scan. |
| SUU09-R007 | lines 61-67; printed pp. 15 | exposure and conclusion / fatal | Guhaad returns with a `karbuuno`, illuminates the hut, and sees Muxsin, Nadiifo, strings, boards, and tins producing the supposed spirits. The closing proverb and `Dhammaad.` are present and complete. | Preserve the reveal without expansion or moral paraphrase. Verify `karbuuno` and every mechanical detail from the page, retain the proverb exactly as printed, and keep `Dhammaad.` as the final source line. |
| SUU09-R008 | PDF 17-18 | printer and back-cover furniture / structural | The pages after `Dhammaad.` contain the national-printer colophon and back-cover/price matter, not additional story text. Their omission from the resource is correct. | Keep them outside the literary body and record their existence only as provenance. |

## Cross-file and technical findings

1. The scan is already sequential and single-page; no facing-page split or
   narrative reordering is required.
2. Git history contains only the initial resource import and no cleaner
   committed transcription to recover.
3. Both local raw OCR files have the same SHA-256,
   `8c6f1a89fa412c93720c78ebd107e0fabe75e281552c7b45d4b09e62a1b5c596`.
   Their agreement is not independent evidence because they are duplicate
   conversions of the same scan.
4. The current resource has two obvious `00`-for-`oo` candidates and numerous
   less mechanically detectable substitutions. Global search-and-replace is
   unsafe: each occurrence needs its printed context.
5. Page-wrap hyphens have mostly been removed. Each joined word must still be
   checked because an apparently valid Somali form can result from a wrong
   join.
6. The source uses guillemets and embeds many exchanges inside prose. Cleanup
   may normalize Markdown spacing around the printed dialogue but must retain
   wording, speaker order, repetition, and expressive pauses.
7. Other SLS resources can corroborate modern lexical forms, but the PDF—not a
   later repository preference—controls this historical literary
   transcription. Any intentional modernization would be a separate editorial
   edition and is outside this cleanup.

## Proposed source-faithful structure

```text
# Rooxaan

[four-line Qamaan Bulxan verse]

[complete continuous story in readable source paragraphs]

Dhammaad.
```

No chapter headings, synopsis, lesson notes, glossary, or newly written moral
should be inserted into the literary body.

## Cleanup gates

Cleanup must:

- inspect PDF pages 3-16 directly and preserve every substantive source line;
- retain the complete narrative, all descriptions and conversations, the
  Qamaan Bulxan attribution, final proverb, and `Dhammaad.`;
- repair only scan-supported OCR substitutions, word joins, punctuation, and
  broken line/page transitions;
- give the opening verse true Markdown line breaks and reconstruct readable
  prose and dialogue paragraphs without inventing sections;
- preserve clearly printed historical spelling and authorial voice, recording
  any exceptional modernization separately rather than applying it silently;
- keep ministry, title-page, printer, page-number, price, and back-cover
  furniture out of the literary body; and
- record the epigraph correction and substantive transcription repairs in the
  provenance log.

Cleanup must not:

- summarize, abridge, or rewrite the story in newly generated Somali;
- use present-day SLS preferences to overwrite legible 1973 literary forms;
- add explanations about folklore, religion, education, marriage, or the
  story's moral as if they were written by the author;
- remove difficult social content merely because it is dated;
- create chapters or topical headings absent from the source; or
- mark cleanup approved or complete without maintainer review.

## Approval state

- **Audit approval:** approved by the maintainer on 2026-08-14 with the
  instruction, "go ahead."
- **Finding IDs:** SUU09-R001 through SUU09-R008
- **Cleanup:** applied on 2026-08-14
- **Cleanup approval:** pending
- **Complete:** no

## Cleanup result

The approved cleanup retained the complete continuous story and corrected it
against the source scan page by page. No narrative scene, character
description, conversation, historical social material, closing proverb, or
`Dhammaad.` was removed, summarized, or newly rewritten.

The cleaned resource contains 183 lines, 3,792 words, and 25,005 bytes. Its
SHA-256 is
`f7823cab527afb5ab88997ae397ebe5365b1c54f467ab90d8fab313e323ed1e5`.

Applied changes include:

- correction of the epigraph's OCR-damaged `Sara Kacaaniaaye` to the printed
  `Sara Kacaantaaye`, with its two couplets and Qamaan Bulxan attribution
  given explicit Markdown verse layout;
- scan-supported repairs including `agal` to `aqal`, `qiig` to `qiiq`,
  `tusbixdisa` to `tusbixiisa`, `gofna` to `qofna`, `shago`/`shagayn` to
  `shaqo`/`shaqayn`, `bogol` to `boqol`, `nogoto` to `noqoto`, `catoodo` to
  `caroodo`, `cegin` to `eegin`, and the damaged dream phrase to the printed
  `hoosas dad u eg`;
- restoration of readable paragraph and speaker boundaries while retaining
  one H1 and adding no invented chapter divisions;
- preservation of printed historical or source-specific forms where the scan
  supports them, including `Muqdishow`, `jirey`, `aabuuray`, `naagod`,
  `miterkuubbo`, `khalfado`, `maqalka`, and `gasada danbe`;
- preservation of all monetary amounts, religious vocabulary, satire,
  corporal punishment, household descriptions, coercive speech, staged
  spirit-session mechanics, and the final proverb; and
- continued exclusion of cover, ministry, title-page, page-number, printer,
  price, and back-cover furniture from the literary body.

The cleanup remains awaiting maintainer approval and is not marked complete.
