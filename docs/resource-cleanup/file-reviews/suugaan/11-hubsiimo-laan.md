# Audit record — Hubsiimo La'aan

- **Resource path:** `resources/suugaan/11-hubsiimo-laan.md`
- **Collection / family:** suugaan / qiso jacayl
- **Priority:** P1
- **Method:** whole-file literary audit; direct comparison with the local
  source scan and its raw OCR conversion
- **Audit status:** approved; second cleanup completed, awaiting maintainer approval
- **Audit date:** 2026-08-14
- **File size at audit:** 2,086 lines; 23,765 words; 136,200 bytes
- **Resource SHA-256:**
  `34f5f8e7a27bc169f9cffea95ae3675f58fd8071dd81214a6bd7dab4d2b82fe8`
- **Resource-text changes during audit:** none

## Target output model

The cleaned file should be a complete, source-faithful transcription of
Maxamed Guuleed Aadan (Seyga)'s romantic novella `Hubsiimo La'aan`. The source
labels it `Qiso Jacayl`, identifies it as number 6 in `Taxanaha Seyga`, and
divides its 102-page narrative into ten explicitly headed parts.

Cleanup may restore the dedication, authorial `Hordhac`, ten printed part
headings, continuous page order, paragraphs, dialogue turns, and
scan-supported words. It may omit publishing addresses, copyright boilerplate,
printed page numbers, scan furniture, unrelated book advertisements, and the
back-cover repetition of an internal passage. It must not abridge the story,
rewrite its social values, modernize its literary voice without evidence, add
a synopsis or moral, or treat the opening hospital scene and the later
chronological narrative as separate stories.

## Controlling source and metadata finding

The controlling source is the local image-only PDF:

`temp/soomaali/literature/Hubsiimo la_aan.pdf`

It contains 57 scanned images. Most images are two-page spreads, with printed
story pages 1-102 occupying PDF pages 4-55. The two OCR conversions at
`temp/md/literature/Hubsiimo la_aan.md` and
`temp/soomaali/md/literature/Hubsiimo la_aan.md` are byte-identical and serve
only as navigation aids. Their shared SHA-256 is
`8c0145b3e3b7692269a0eb49dfd8bcb5c0a562030f190674a51a3e8fb91cc266`.
The page images control every correction.

The source establishes:

- title: `Hubsiimo La'aan`;
- genre label: `Qiso Jacayl`;
- author: `Maxamed Guuleed Aadan (Seyga)`;
- series: `Taxanaha Seyga 6`; and
- place: Muqdisho, Soomaaliya.

The registry currently gives the year `1968`, but the scanned publication page
visibly begins both date fields with `198` and loses their final digits at the
cropped right edge. The scan therefore does not support `1968`, but it also
does not support inventing a complete 1980s year. Approved cleanup should
remove or qualify the unsupported registry year unless a complete primary
copy is found.

## Source structure

The scan has no contents page, but the body itself prints all ten divisions:

| Source division | Printed start | PDF page / side | Narrative focus at the boundary |
| --- | ---: | --- | --- |
| Qaybta Koowaad | 1 | 4 right | American hospital frame: Dr. Johnson explains Faatma's illness to Xaajiyo Binti and Xasan |
| Qaybta Labaad | 4 | 6 left | Saadiq goes to meet Bashiir and first encounters Faatma |
| Qaybtii Saddexaad | 21 | 14 right | Xaajiyo Binti travels to Saudi Arabia and visits her brother |
| Qaybtii Afraad | 30 | 19 left | Faatma prepares for another meeting with Saadiq |
| Qaybta Shanaad | 42 | 25 left | Xaajiyo Binti returns from Jidda |
| Qaybta Lixaad | 51 | 29 right | Cali Waraabe's role and intentions enter the plot |
| Qaybtii Toddobaad | 64 | 36 left | Bashiir learns of Saadiq's detention and intervenes |
| Qaybta Sideedaad (8) | 73 | 40 right | Xaajiyo Binti receives a telephone call concerning Cali |
| Qaybtii Sagaalaad | 81 | 44 right | Saadiq returns from the regional work journey and learns of Faatma's nikax |
| Qaybta Tobnaad | 94 | 51 left | Faatma reports her decision to Saadiq; the final illness sequence follows |

The story ends on printed page 102 with news of Faatma's death, Xaajiyo
Binti's final words invoking `Hubsiimo la'aaneey`, and the decorative closing
word `DHAMMAAD`. The right side of PDF page 55 and PDF page 56 advertise other books.
PDF page 57 is back-cover copy that repeats a passage from Part Nine; none of
those three items continues the novella.

## Current structural and textual condition

The Markdown has one H1 and no usable section headings. The body begins with
printed page 1, but the facing dedication and `Hordhac` have been inserted
line-by-line into the hospital scene. `Qaybta Koowaad` itself is absent. The
same converter error affects nearly the entire book: left and right printed
pages from each spread are read in alternating fragments instead of normal
page order.

Consequences include:

- narration from one printed page entering dialogue from the facing page;
- sentences, words, and speaker turns broken at arbitrary horizontal points;
- part headings lost, misspelled, or embedded inside unrelated prose;
- pages apparently jumping forward and backward inside the same paragraph;
- hundreds of false digits, symbols, and letter substitutions;
- printed page numbers and scan marks entering the narrative; and
- advertisements and back-cover text being mistaken for the ending.

The current text contains 442 lines matching high-risk digit, symbol, or OCR
patterns and 49 very short nonblank debris candidates. These are audit
indicators only; no global replacement or automatic deletion is safe.

## Literary and editorial scope

The plot concerns Saadiq and Faatma's relationship, opposition from Xaajiyo
Binti, class and marriage expectations, a competing nikax, illness, medical
treatment abroad, and death. It contains coercive family pressure, physical
punishment, detention, gendered and class-based judgments, and historical
representations of mental and physical illness. These are substantive parts
of the source and must be retained as authored narrative, not endorsed as
modern SLS guidance and not removed because they are dated or difficult.

Names, place names, monetary amounts, travel details, institutional names,
dialogue, religious expressions, and the source's narrative chronology must
be checked against the scan. Modern repository spelling may help identify an
OCR error, but the scan—not an editor's preferred spelling—controls a clearly
printed historical form.

## Audit coverage and findings

| ID | Resource lines / source pages | Class / severity | Source evidence and finding | Proposed action |
| --- | --- | --- | --- | --- |
| SUU11-R001 | title, registry, PDF 1-4 | identity and metadata / fatal | The source confirms title, genre, author, series number, and place. The registry's `1968` conflicts with the cropped publication page, which visibly begins `198…`. | Retain the canonical title and author; record genre/series in provenance; remove or qualify the unsupported year without guessing the missing digit. |
| SUU11-R002 | whole file; PDF 4-55 | spread-order corruption / fatal | Every narrative PDF page is a facing-page spread, but the OCR was read across both pages. The current Markdown therefore alternates unrelated left- and right-page fragments. | Reconstruct the story in printed page order by treating every page half independently. No current paragraph boundary may be trusted without scan comparison. |
| SUU11-R003 | lines 1-79; front matter and pp. 1-3 | foreword and Part One / fatal | The dedication and `Hordhac` are mixed into the opening hospital scene, while `Qaybta Koowaad` is missing. The opening is a deliberate frame that anticipates the later story. | Restore the dedication and full `Hordhac` as authorial front matter, then Part One pp. 1-3 in continuous order. Preserve Dr. Johnson, Xaajiyo Binti, Xasan, and the transition back into the history. |
| SUU11-R004 | lines 80-approximately 400; pp. 4-20 | Part Two / fatal | The printed `Qaybta Labaad` survives only as damaged inline text. Saadiq's meeting with Bashiir, first encounters with Faatma, the wedding, and subsequent meetings are interleaved across spreads. | Restore the heading and every scene, dialogue turn, paragraph, and printed page transition without summarizing. |
| SUU11-R005 | approximately lines 400-588; pp. 21-29 | Part Three / fatal | `Qaybtii Saddexaad` is absent from Markdown. Xaajiyo Binti's Saudi visit and the concurrent Saadiq/Faatma narrative are merged together by spread OCR. | Restore the source heading and place each printed page in sequence; retain travel, family, health, and relationship details. |
| SUU11-R006 | approximately lines 589-837; pp. 30-41 | Part Four / fatal | The heading `Qaybtii Afraad` was not recognized. The romantic meeting, money, hotel, vehicle, and letter/account sequence is badly fragmented. | Restore the full division and all source dialogue, amounts, settings, and actions from pp. 30-41. |
| SUU11-R007 | lines 838-974; pp. 42-50 | Part Five / fatal | `QBYBTA SHANAAD` is embedded inside a facing-page sentence. Xaajiyo Binti's return, household conflict, and search for Saadiq are out of order. | Restore `Qaybta Shanaad`, then reconstruct pp. 42-50 page by page. |
| SUU11-R008 | lines 975-1294; pp. 51-63 | Part Six / fatal | The Part Six heading is attached to the end of page 50. Cali Waraabe's plot, the ship/shipment material, Xaajiyo Binti's actions, and Saadiq's detention are interleaved. | Restore `Qaybta Lixaad` and the complete source sequence, retaining names, accusations, official interactions, and all dialogue. |
| SUU11-R009 | lines 1295-1485; pp. 64-72 | Part Seven / fatal | `QAYBTII TODOBAAD` survives but shares a line with facing-page prose. Bashiir's intervention, police-station events, and Saadiq's work assignment are fragmented. | Restore the heading and pp. 64-72 in normal page order with clear speaker and paragraph boundaries. |
| SUU11-R010 | lines 1486-approximately 1616; pp. 73-80 | Part Eight / fatal | The source prints `Qaybta Sideedaad (8)`, but the current form is corrupted and inline. The telephone, journey, return, and marriage arrangements are mixed. | Restore the printed heading—including `(8)`—and the complete division without silently recasting the source's nikax terminology or social claims. |
| SUU11-R011 | approximately lines 1617-1882; pp. 81-93 | Part Nine / fatal | The `Qaybtii Sagaalaad` heading is absent. Saadiq's return, discovery of Faatma's nikax, confrontations, and the renewed Saadiq/Faatma contact are interleaved. | Restore the division, all conversations, and the source's emotional and social language through p. 93. |
| SUU11-R012 | lines 1883-2048; pp. 94-102 | Part Ten and ending / fatal | `QAYBRA DOBNALD` is the damaged Part Ten heading. The broken OCR obscures Faatma's decision, illness, treatment attempts, Saadiq's response, death report, Xaajiyo Binti's closing words, and the decorative `DHAMMAAD`. | Restore `Qaybta Tobnaad`, all final scenes, the exact closing statement, and `Dhammaad`. |
| SUU11-R013 | lines 2049-2086; PDF 55-57 | unrelated advertisements and duplicate cover copy / high | After story p. 102, the resource includes OCR from advertisements for `Filan Waa` and `Iima Qalantid`, artwork noise, and a back-cover excerpt repeating Part Nine. | Remove these non-narrative advertising and duplicate-cover fields from the reader-facing resource; document their exclusion rather than treating them as lost story text. |

## Cross-file and technical findings

1. The two raw OCR files are identical; neither is an independent witness.
2. Git history contains only the initial imported resource and no cleaner
   committed transcription.
3. The PDF's 90-degree rotation metadata does not change the literary order;
   the central problem is the two-page-spread layout.
4. Printed pages 1-102 are complete in the scan. The page halves are generally
   legible, although contrast, shadows, cropped edges, fingers, and marks vary.
5. A fresh OCR pass on separately cropped halves may accelerate cleanup, but
   each result still requires visual confirmation.
6. Common errors include `g` for `q`, `c` for `e`, `wexay` for `waxay`, digits
   for letters, fused words, dropped apostrophes, and false punctuation. These
   patterns are not safe global replacements because valid source instances
   occur nearby.
7. The author uses dialogue-heavy prose and inconsistent spacing. Cleanup may
   regularize Markdown paragraph and quotation spacing when the speaker order
   is clear, but must not rewrite utterances into newly composed Somali.
8. The classification under `suugaan` is correct: the source explicitly calls
   itself a `Qiso Jacayl` and presents one continuous ten-part literary work.

## Proposed source-faithful structure

```text
# Hubsiimo La'aan

[dedication]

## Hordhac
[complete authorial foreword and attribution]

## Qaybta Koowaad
[printed pp. 1-3]

## Qaybta Labaad
[printed pp. 4-20]

## Qaybtii Saddexaad
[printed pp. 21-29]

## Qaybtii Afraad
[printed pp. 30-41]

## Qaybta Shanaad
[printed pp. 42-50]

## Qaybta Lixaad
[printed pp. 51-63]

## Qaybtii Toddobaad
[printed pp. 64-72]

## Qaybta Sideedaad (8)
[printed pp. 73-80]

## Qaybtii Sagaalaad
[printed pp. 81-93]

## Qaybta Tobnaad
[printed pp. 94-102 and Dhammaad]
```

## Cleanup gates

Cleanup must:

- inspect all 57 PDF images and reconstruct printed pages 1-102 from separate
  page halves;
- retain the dedication, complete `Hordhac`, all ten printed parts, every
  narrative scene, dialogue turn, name, place, amount, and final statement;
- restore the exact printed part headings and preserve the opening
  hospital-frame chronology;
- repair OCR substitutions, page joins, word joins, and punctuation only from
  scan evidence;
- preserve clearly printed historical and authorial forms instead of silently
  applying present-day SLS preferences;
- remove the unsupported `1968` registry claim unless stronger primary
  evidence establishes it;
- exclude printed page numbers, scan marks, unrelated advertisements, artwork
  noise, and duplicate back-cover copy from the reader-facing body;
- record the spread reconstruction, metadata uncertainty, structural repair,
  advertising exclusion, major transcription ranges, and unresolved readings
  in provenance; and
- leave cleanup approval and completion pending for maintainer review.

Cleanup must not:

- summarize or abridge the 102-page narrative;
- omit coercive, violent, class-based, gendered, medical, or religious content
  because it is dated or difficult;
- rewrite the story as modern relationship or medical guidance;
- invent a full publication year from the cropped `198…` fields;
- insert topical subheadings, explanations, a glossary, or a moral not printed
  by the author;
- silently modernize literary diction or normalize every unusual spelling;
  or
- mark cleanup approved or complete without maintainer review.

## Approval state

- **Audit approval:** approved by maintainer instruction `go ahead` on 2026-08-14
- **Finding IDs:** SUU11-R001 through SUU11-R013
- **Cleanup:** completed on 2026-08-23; awaiting maintainer approval
- **Cleanup approval:** pending
- **Complete:** no

## Cleanup result

The approved pass rebuilt the resource from separately cropped printed pages
1-102 instead of retaining the converter's across-spread reading order. It
restored the dedication, `Hordhac`, author attribution, all ten printed part
headings, normal narrative chronology, and the source ending through
`Dhammaad`. Three page crops whose OCR had absorbed a facing-page strip were
checked separately, and the closing pages were read directly against the
scan. Printed page numbers, scan-only debris, unrelated advertisements, and
the duplicate back-cover extract were excluded.

The source registry's unsupported `1968` was changed to an unresolved em dash.
The cropped publication page shows only `198…`; no missing final digit was
invented. Historical diction, plot content, social claims, and the complete
ten-part narrative remain in scope. Because the source is a low-resolution,
image-only scan with shadows, curved gutters, faded type, and damaged edges,
some uncertain character-level readings remain visible for maintainer cleanup
review rather than being silently rewritten from editorial preference.

The resulting resource is 1,321 lines, 21,441 words, and 130,874 bytes. Its
SHA-256 is
`66f4a554b50d9af96034da9a6be3c3d7d0bc97f4110acd49c7f2f1cfffafb276`.

## Independent source recheck — 2026-08-22

The cleanup result above is retained as a historical account of the attempted
rebuild, not as a successful verification. A new comparison against the local
57-image scan found severe OCR corruption throughout the current 1,321-line
resource. The defects include fragments from facing pages, digits and symbols
inside words, broken line joins, omitted text, and prose that cannot be read as
continuous Somali. Clear examples occur at resource lines 47–145, 291–337,
441–463, and especially 549–591. Printed pages 38–39 (PDF page 23) independently
confirm that the damaged text at lines 573–591 has not been transcribed
faithfully.

The resource hash still matches the recorded attempted-cleanup hash, so this is
not an accidental later regression: the earlier pass itself was incomplete.
Cleanup is therefore reopened in the tracker. Approval and completion remain
blocked until all printed pages 1–102 are retranscribed or checked page by page
against separately cropped page halves. No speculative corrections were made
to the literary text during this recheck.

## Second reconstruction pass — 2026-08-22

The failed 1,321-line rebuild was replaced with a new page-order reconstruction
made from 102 separately cropped printed pages. The crops were processed as
individual pages rather than facing-page spreads, and text boxes intruding from
the opposite page were filtered at the page-column boundary. This removes the
fatal cross-page alternation found by the independent recheck.

The new pass also:

- retains the dedication, complete `Hordhac`, author attribution, and the ten
  printed part headings in their source order;
- directly retranscribes printed pages 1–3 and 101–102 from the page images;
- checks and repairs the high-risk scan marks found on printed pages 5, 15, 24,
  36, 54, 62, 69, 71, 72, 77, 78, 81, 89, 90, 92, and 94;
- retains the narrative through the final death report and `Dhammaad`; and
- continues to omit printed page numbers, unrelated advertisements, and the
  duplicate back-cover extract.

This is a material structural improvement, but it is not yet a verified final
transcription. The image-only source still leaves ordinary letter substitutions,
line-wrap joins, punctuation damage, and several uncertain words in the body.
Those readings must be checked page by page before cleanup approval. The tracker
therefore leaves Cleanup, Cleanup approval, and Complete unchecked.

### Page-image proofread — Part Two

Printed pages 4–20 (`Qaybta Labaad`) were subsequently compared line by line
with their seventeen separate page images and retranscribed as one continuous
division. This pass restores Saadiq's journey and encounter at the house, his
meeting with Bashiir, the wedding, his first conversations with Faatma,
Xaajiyo Binti's travel instructions, and the page-20 arrangement for their next
meeting. It also removes the remaining OCR-derived substitutions and damaged
joins in this division. Part Two passes the local mixed-case, embedded-digit,
symbol-artifact, and heading checks.

This page range is now source-guided and structurally reviewed, but Parts Three
through Ten still require the same character-level page-image proofread. The
overall cleanup state therefore remains open.

### Page-image proofread — Part Three

Printed pages 21–29 (`Qaybtii Saddexaad`) were next compared line by line with
their nine separate page images and retranscribed continuously. This pass
restores Xaajiyo Binti's arrival in Sucuudi Arabiya, her conversation with her
brother about Faatma's prospective marriage, Faatma's telephone call and
private meeting with Saadiq, their agreement, and Faatma's closing confrontation
with Asli. The source's `tel. 009888`, `Jaaw xabiibi`, names, dialogue, and
narrative sequence were retained. Context-supported letter repairs were checked
against repository forms, including `shaxan`, `tabaabushaysto`, and `xisaab
xidhkii sannadka`.

Part Three passes the local mixed-case, embedded-digit, symbol-artifact, and
heading checks. Parts Four through Ten still require character-level page-image
proofreading, so the overall cleanup state remains open.

### Page-image proofread — Part Four

Printed pages 30–41 and the continuation above the Part Five heading on page 42
were compared line by line with the separate page images and retranscribed as
one continuous division. This restores the nighttime meeting, cinema visit,
Saadiq's telephone invitation, Faatma's drive and shopping trip, the hotel meal,
and the complete envelope scene. The page-42 continuation is retained in Part
Four because the printed `Qaybta Shanaad` heading occurs below it.

The source's venues, vehicle references, `25 ilaa 40 km`, and the final
`200,000 sh. so.` amount were retained. Part Four passes the local mixed-case,
embedded-digit, symbol-artifact, quote-balance, and heading checks.

### Page-image proofread — Part Five

Printed pages 42–50 and the continuation above the Part Six heading on page 51
were compared line by line with the separate page images and retranscribed as
one continuous division. The page-51 continuation completes Xaajiyo Binti's
telephone message before the printed `Qaybta Lixaad` heading and therefore
remains in Part Five.

This pass restores Xaajiyo Binti's return from Jidda, her questioning of Asli,
the search for Faatma, the household confrontation, the search of Faatma's
xusuus-qor, the meeting with Cali Waraabe, and the disguised telephone call to
Saadiq's household. The source's coercion and physical violence were retained
as substantive narrative content. Part Five passes the local mixed-case,
embedded-digit, symbol-artifact, quote-balance, and heading checks.

### Page-image proofread — Part Six

Printed pages 51–63 (`Qaybta Lixaad`) were compared line by line with their
thirteen separate page images and retranscribed continuously. The division
ends at the bottom of page 63; `Qaybtii Toddobaad` begins at the top of page
64, so no cross-heading continuation is involved.

This pass restores Cali Waraabe's background, the account of his earlier
theft and imprisonment, Xaajiyo Binti's arrangement with Cali and Ciise,
Saadiq's arrival and arrest, and the complete police-station questioning by
Xidigle Haashim. The source's references to Shaneemo Banaadir, Shaneemo
Ceelgaab, Saldhigga Afar Iridood, the Bangiga Ganacsiga iyo Keydka
Soomaaliyeed, and the false-witness finding were retained. Part Six passes the
local mixed-case, embedded-digit, symbol-artifact, quote-balance, and heading
checks. Parts Seven through Ten still require character-level page-image
proofreading, so the overall cleanup state remains open.

The current reconstruction is 1,105 lines, 19,392 words, and 125,570 bytes. Its
SHA-256 is
`973de66e7d693eca095235c5044ba902abc9237a034c37d2531b3f27880bc476`.

### Page-image proofread — Part Seven

Printed pages 64–72 and the continuation above the Part Eight heading on page
73 were compared line by line with the separate page images and retranscribed
continuously. The continuation on page 73 completes the account of the damaged
`Linea-Bus` and therefore belongs to Part Seven.

This pass restores Bashiir and Faatma's visit to Saadiq, the police interviews
of Xaajiya Binti, Asli, Cali Waraabe, and Ciise Ma-qubeyste, the exposure of
the false accusation, Saadiq's release and work assignment, Bashiir's
representation of Saadiq, and Xaajiya Binti's shipping loss. The source's
police procedure, detention, false testimony, place references, and unusual
literary forms were retained. Part Seven passes the local mixed-case,
embedded-digit, symbol-artifact, quote-balance, and heading checks.

### Page-image proofread — Part Eight

Printed pages 73–80 (`Qaybta Sideedaad (8)`) were compared line by line with
their eight separate page images. The division begins below its heading on
page 73 and ends at the bottom of page 80; Part Nine begins at the top of page
81.

This pass restores the telephone call from Jidda, Xaajiya Binti's journey,
Yuusuf and Cali's meeting, the marriage negotiation, Cali's gift and travel to
Muqdisho, and the preparations for the nikax. The printed `(8)`, Jidda and
Muqdisho settings, household arrangements, and source social claims were
retained. Part Eight passes the local mixed-case, embedded-digit,
symbol-artifact, quote-balance, and heading checks.

### Page-image proofread — Part Nine

Printed pages 81–93 and the continuation above the Part Ten heading on page 94
were compared line by line with the separate page images and retranscribed as
one division. The page-94 continuation contains Faatma's final response to
Saadiq and precedes the printed `Qaybta Tobnaad` heading.

This pass restores Saadiq's return, discovery of the proposed nikax, entry
into the gathering, production of the existing nikax paper, the resulting
confrontations in both households, and Saadiq and Faatma's final meeting. The
source's religious, familial, class-based, coercive, and violent content was
retained as authored narrative. Part Nine passes the local mixed-case,
embedded-digit, symbol-artifact, quote-balance, and heading checks.

### Page-image proofread — Part Ten and ending

Printed pages 94–102 (`Qaybta Tobnaad`) were compared line by line with their
nine separate page images and retranscribed continuously. This restores the
celebration after the separation, Faatma's illness, successive treatment
attempts, Xaajiya Binti's appeal to Saadiq, Saadiq's account of his later
marriage and child, the death report, and Xaajiya Binti's final words.

The decorative box on page 102 spells `DHAMMAAD`. The earlier audit reading
`Hammar` was a visual misidentification and has been corrected rather than
retained as a place name. Part Ten passes the local mixed-case, embedded-digit,
symbol-artifact, quote-balance, heading, and ending checks.

## Final cleanup validation — 2026-08-23

All printed narrative pages 1–102 have now received the second, direct
page-image proofread. The result retains the dedication, complete `Hordhac`,
author attribution, opening hospital frame, all ten printed divisions, all
narrative sequences, and the closing `Dhammaad`. Printed page numbers, scan
marks, advertisements, and duplicate back-cover text remain intentionally
excluded.

The completed resource has:

- one H1 and eleven H2 headings (`Hordhac` plus ten divisions);
- 431 opening and 431 closing Somali quotation marks;
- no control characters, OCR-only symbol substitutions, embedded-digit words,
  stray heading marks, or ASCII quotation marks; and
- 1,079 lines, 20,071 words, and 131,177 bytes.

Its SHA-256 is
`928e31d654feab3e0d4838a20bc7b86ea0a1750c188e6f6261cd2aef3ec2455a`.

Cleanup is complete and ready for maintainer review. Cleanup approval and the
overall Complete state remain intentionally unchecked until that review.
