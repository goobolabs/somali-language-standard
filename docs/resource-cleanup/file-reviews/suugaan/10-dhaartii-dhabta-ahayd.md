# Audit record — Dhaartii Dhabta Ahayd

- **Resource path:** `resources/suugaan/10-dhaartii-dhabta-ahayd.md`
- **Collection / family:** suugaan / ururin taariikheed, khudbado, heeso,
  gabayo, geeraarro iyo murti
- **Priority:** P1
- **Method:** whole-file literary and documentary-content audit; direct
  comparison with the local source scan and its raw OCR conversion
- **Audit status:** approved; source-guided cleanup applied, awaiting maintainer review
- **Audit date:** 2026-08-14
- **File size at audit:** 2,690 lines; 13,439 words; 81,903 bytes
- **Resource SHA-256:**
  `30b0aad2a85b8fe9c6c491cc3e981ab36123a4aa533e1eadc983b558e6340811`
- **Resource-text changes during audit:** none

## Target output model

The cleaned file should remain a complete, source-faithful transcription of
Maxamed Nuur Xasan (Shareeco)'s 1986 compilation. It is not one short story.
It combines a presidential speech, a defence-minister statement, the author's
preface, a political and military-history essay, captioned monument and
portrait pages, and a second part presented as the performance
`Gaadiidka beeshoo lagu guurayaan ahay`, containing songs, a love dialogue,
gabay, geeraar, and other verse.

Cleanup may restore the source's two-part hierarchy, ten contents units,
individual literary titles, verse layout, attributions, paragraphs, lists,
and scan-proven words. It may remove verified page numbers, library marks,
isolated picture noise, and OCR symbols. It must not abridge the political
material, extract only the verse, rewrite historical claims as current SLS
facts, silently correct the author's dates or interpretations from outside
sources, modernize the author's voice, or present the work as neutral modern
history.

## Controlling source and metadata finding

The controlling source is the local image-only PDF:

`temp/soomaali/literature/Dhaartii dhabta ahayd.pdf`

It contains 84 sequential single-page images and no usable text layer. The two
raw OCR conversions at `temp/md/literature/Dhaartii dhabta ahayd.md` and
`temp/soomaali/md/literature/Dhaartii dhabta ahayd.md` are byte-identical and
serve only as navigation aids. Their shared SHA-256 is
`1aeb332b3803ca2632f2460231e01234631acd8bdf20ccd0c84c4f9c95603a0f`.
The page images control every correction.

The cover reads:

- `Dhaartii Dhabta Ahayd`;
- `Waxaa qoray: G/Sare Maxamed Nuur Xasan (Shareeco)`; and
- `12 Abriile 1986 Muqdisho`.

The source registry already gives the title and author but has no year. The
explicit cover date supports adding `1986` during approved cleanup. The
author is variously identified inside the source as `G/Sare`, `G/le Sare`,
and `Dr.`; those source-context forms should be retained where printed rather
than forced into one body-text title. The registry's plain personal name and
pen name remain appropriate.

## Source contents and page structure

The source's own contents page establishes this hierarchy:

| Contents item / source unit | Printed pages | PDF pages |
| --- | ---: | ---: |
| Khudbaddii ugu horreysay ee Ummadda Soomaaliyeed u jeediyey Madaxweynaha J.D.S., 23kii Okt. 1969 | 1-5 | 4-8 |
| Halku-dhegga Wasiirka Wasaaradda Gaashaandhigga S/Guud Maxamed Cali Samatar | 7 | 10 |
| Ararta Buugga | 9-11 | 12-14 |
| **Qaybta Hore ee Buugga** |  |  |
| Item 4 / body 1. Ciidanka X.D.S. | 14-20 | 17-23 |
| Item 5 / body 2. Wax qabadkii Ciidanka X.D.S. | 20-28 | 23-31 |
| Item 6 / body 4. Sheekh Abaadir Muuse Warwaajecle iyo halgankii uu soo maray | 29-30 | 32-33 |
| Item 7 / body 5. Halgankii Axmed Gurey | 30 | 33 |
| Item 8 / body 6. Halgankii dheeraa ee Sayid Maxamed Cabdulle Xasan | 32-35 | 35-38 |
| Item 9 / body 7. Dhalashadii Gobannimada | 35-36 | 38-39 |
| **Qaybta Labaad ee Buugga** |  |  |
| Item 10 / Riwaayadda `Gaadiidka beeshoo lagu guurayaan ahay` | 38-81 | 41-84 |

The numbering printed in the body differs from the contents after the first
two main chapters: the body visibly labels Sheekh Abaadir as `4`, Axmed Gurey
as `5`, Sayid Maxamed as `6`, and Dhalashadii Gobannimada as `7`. There is no
body unit numbered `3` at that point. Cleanup should preserve the body labels
and document the contents mismatch rather than silently renumbering the
author's work.

The remaining scan pages are meaningful source context:

| PDF pages | Source matter |
| ---: | --- |
| 1 | Cover with title, author, date, place, and later library markings |
| 2 | `Tusmada Buugga` |
| 3 | Portrait captioned `Madaxweynaha J.D.S. Jaalle Maxamed Siyaad Barre` |
| 9 | Portrait caption identifying Maxamed Cali Samatar and his offices |
| 11 | Poorly reproduced image/furniture between the statement and preface |
| 15 | Portrait captioned `G/Sare Dr. Maxamed Nuur Xasan (Shareeco)` |
| 16 | Author's note to readers before the main text |
| 18-19 | Captioned images: `Taalada S.Y.L.` and `Taallada Dhagaxtuur` |
| 25 and other prose pages | Documentary photographs embedded in the essay |
| 34 | `Taalada Axmed Gurey` |
| 37 | `Taalada Xaawo Cismaan (X. Taako)` |
| 40 | `Taalada Daljirka Dahsoon` |

The resource is therefore substantially complete in gross page order, but its
source hierarchy is almost entirely absent and its image-caption handling is
inconsistent.

## Current structural and textual condition

The Markdown has only one H1 and six H2 headings for an 81-printed-page book.
Two of those H2s are image captions, and two are fragments created from
damaged body text. Most actual chapters, the two main parts, songs, gabayo,
geeraarro, and attributions remain plain OCR prose.

The present `## Arar` at line 3 is false. The text immediately following it is
the author's introductory explanation printed on source page 1, followed by
the first presidential speech. The actual `ARAR` begins on PDF page 12 /
printed page 9 and continues through printed page 11. It presently has no
Markdown heading. This error shifts the apparent meaning of the first twelve
printed pages.

The source converter preserved most pages but left pervasive noise:

- `00` for `oo`, `800` for `soo`, and digits inside ordinary Somali words;
- false punctuation, copyright/trademark symbols, vertical rules, slashes,
  underscores, and isolated letters inherited from page backgrounds;
- `Clidanka` and other letter substitutions recurring hundreds of times;
- split and fused words across printed line endings;
- headings embedded inside paragraphs or misread as ordinary text;
- songs and poems flattened into visually noisy one-line fragments;
- printed page numbers and photograph noise entering the prose; and
- missing, damaged, or misassigned titles and attributions.

The file contains 103 lines matching high-risk symbols or obvious digit/OCR
patterns, 14 very short nonblank debris candidates, and only seven Markdown
headings. These are audit indicators, not automatic deletion or replacement
rules.

## Documentary and editorial scope

This source praises the Somali Democratic Republic's revolutionary government
and armed forces and makes political, religious, military, biographical, and
historical claims. It includes accounts of the 1969 coup, the 1964 and 1977
wars, Axmed Gurey, Sheekh Abaadir, Sayid Maxamed Cabdulle Xasan, the S.Y.L.,
Xaawo Taako, colonialism, socialism, and contemporary international conflicts.

For this resource-cleanup project, those statements are historical source
content—not verified repository claims and not present-day endorsement.
Cleanup should transcribe them faithfully and describe their source context in
the audit/provenance record. It should not use outside scholarship to rewrite
the text, even when a date, number, title, political characterization, or
causal claim appears disputable. A separate critical annotation project would
be needed to evaluate factual accuracy.

The second part also contains political praise poetry, military imagery,
romantic dialogue, and descriptions of war and death. These are substantive
literary content and cannot be removed merely because they are ideological,
dated, repetitive, or uncomfortable.

## Audit coverage and findings

| ID | Resource lines / source pages | Class / severity | Source evidence and finding | Proposed action |
| --- | --- | --- | --- | --- |
| SUU10-R001 | line 1 and registry; PDF 1-2 | identity, metadata, and contents / fatal | Cover inspection confirms Maxamed Nuur Xasan (Shareeco), Muqdisho, and 12 Abriile 1986. The registry year is blank. The source contents identifies two main parts and ten listed units, while the resource exposes almost none of them. | Retain one H1, add 1986 to the source registry, and reconstruct the source hierarchy from the contents and body. Do not insert library call numbers or scan ownership marks. |
| SUU10-R002 | lines 3-26; PDF 4 | false `Arar` boundary and opening note / fatal | The current H2 labels the opening title explanation as `Arar`, but the printed `ARAR` does not begin until PDF 12. The opening explains the book title and introduces Siyaad Barre's first speech. | Remove the false heading and retain this as the book's untitled opening note. Preserve the author's title explanation and transition into the speech; do not merge it with the later preface. |
| SUU10-R003 | lines 27-160; printed pp. 1-5 / PDF 4-8 | presidential speech / fatal | The 23 Oktoobar 1969 speech survives in sequence but has severe OCR substitutions, symbol fields, broken sentences, and damaged paragraphs. Its political claims and reference to Cabdirashiid Cali Sharmaarke are substantive documentary text. | Reconstruct the complete speech from all five pages under its source title. Preserve wording, date, names, political argument, closing slogans, and quotation status. Present it as source text without adding factual endorsement. |
| SUU10-R004 | lines 161-207; printed pp. 6-7 / PDF 9-10 | portrait caption and Maxamed Cali Samatar statement / fatal | A portrait/caption page precedes the statement. The statement title is plain damaged prose and the body is heavily corrupted. | Retain the meaningful portrait caption as source context without inventing a visual description; create the statement heading and restore its full prose and closing slogans from page 7. |
| SUU10-R005 | lines 208-333; printed pp. 9-13 / PDF 12-16 | actual `Arar`, author credit, portrait, and reader note / fatal | The true three-page `ARAR` discusses armies, the military oath, Islam, and the book title. It is followed by an author portrait caption and a one-page note asking readers to report errors. All are in the resource but lack correct boundaries and contain extensive OCR damage. | Restore `## Arar`, its full text and printed author credit; retain the meaningful author caption and complete reader note as distinct source units. Remove page numbers and image noise only. |
| SUU10-R006 | lines 334-630; printed pp. 14-20 / PDF 17-23 | Part One, chapter 1, and captioned monuments / fatal | `Qaybta Hore ee Buugga` and `1- Ciidanka Xoogga Dalka Soomaaliyeed` are buried in OCR. Captioned S.Y.L. and Dhagaxtuur monument pages interrupt but do not end the chapter. The chapter reaches the chapter-2 heading on printed page 20. | Restore the part and chapter headings and all prose in source order. Retain the two printed monument captions, omit uninterpretable picture pixels, and do not treat either caption as a prose chapter. |
| SUU10-R007 | lines 631-853; printed pp. 20-28 / PDF 23-31 | chapter 2, military activities and war claims / fatal | The source explicitly begins `2- Wax qabadkii Ciidanka Xoogga Dalka Soomaaliyeed iyo guulihii uu soo hoyay`. The resource lacks this heading. Lists of activities, military qualities, leadership, and 1964/1977 war claims are present but badly damaged. | Restore the chapter title, paragraphs, enumerated activities, names, quantities, and source claims. Verify character readings from the images; do not independently “correct” asserted history or numbers. |
| SUU10-R008 | lines 854-998; printed pp. 29-35 / PDF 32-38 | Sheekh Abaadir, Axmed Gurey, Sayid Maxamed, and monuments / fatal | Three body chapters are collapsed. `## Cabulle Xasan. -` is a false fragment of the complete Sayid title. Captioned Axmed Gurey and Xaawo Taako monument pages are obscured. The source/body numbering conflicts with the contents. | Restore source body headings 4, 5, and 6 exactly as printed, preserve full arguments and the numbered Daraawiish battle list, retain meaningful monument captions, and document rather than repair the numbering mismatch. |
| SUU10-R009 | lines 999-1058; printed pp. 35-37 / PDF 38-40 | Dhalashadii Gobannimada and Daljirka Dahsoon / fatal | Chapter 7 covers S.Y.L. struggle, Xaawo Taako, independence, and the 1969 takeover. It is followed by the captioned Daljirka Dahsoon page. No correct Markdown chapter heading exists. | Restore chapter 7 and all prose; retain the source caption for Daljirka Dahsoon without invented image prose. Preserve political language and dates as printed claims. |
| SUU10-R010 | lines 1059-1508; printed pp. 38-50 / PDF 41-53 | Part Two introduction and songs / fatal | `Qaybta Labaad` and the performance title are OCR prose. The section contains named songs beginning with `Hirashada Abriiley`, `Geed hoos qabooboo hurdo lagu gam'aan ahay`, question-and-answer verse on who represents a nation, `Runta aan u baymee`, `Haddii ay guracantiyo haddii nabad la gaaraba aniga gaar-hayaan ahay`, the romantic `Laamaha ubaxa iga hoo`, and another military-history song. Only `Hirashada Abriiley` is currently a heading. | Restore the part/performance hierarchy, each printed song title, prose introduction, stanza and refrain layout, speaker labels, numbering, and author attribution. Preserve each complete verse; do not convert songs into prose or omit repeated refrains. |
| SUU10-R011 | lines 1509-2663; printed pp. 51-81 / PDF 54-84 | gabay, geeraar, murti, speakers, and performance sequence / fatal | The remainder introduces a long gabay and many geeraar/murti turns attributed to figures such as Wadar Camey Barre, Askar, Foos, Ereg, Diidey, Tahliil, Xasan, and others. Almost all transitions and attributions are plain OCR. Verse is complete in broad order but pervasively damaged and flattened. | Reconstruct every source transition, named/role attribution, stanza, and verse line page by page. Use restrained lower-level headings only for explicit source divisions; do not invent poem titles where the source gives only a speaker or genre introduction. |
| SUU10-R012 | lines 2664-2690; printed p. 81 / PDF 84 | closing authorship and note / high | The final page attributes the collected gabay, geeraar, and murti to Maxamed Nuur Xasan (Shareeco), requests corrections from Somali scholars, reflects on writing and memory, and closes with his name/title. It is present but OCR-damaged and has no structural separation. | Restore the complete closing prose and author signature. Do not add `Dhammaad.` because the source does not print one. |

## Cross-file and technical findings

1. The source scan is sequential and single-page; no facing-page split or page
   reordering is needed.
2. Git history contains only the initial imported resource and no cleaner
   committed transcription.
3. The current file is 607 words shorter than the raw OCR. Much of that
   difference is converter wrappers and debris, but completeness must be
   decided page by page rather than from word counts.
4. The scan quality varies sharply. Some pages are crisp text; portraits and
   monument photographs generate symbol-only OCR; several verse pages have
   weak contrast. A fresh OCR result may assist but cannot control a reading.
5. `00`/`800`, `Clidanka`, names, dates, ranks, military terms, and poem-initial
   alliteration require contextual correction, not blind global replacement.
   A valid digit or unusual source form can occur in the same pages.
6. The source's meaningful image captions should survive in text. The cleaned
   Markdown should not describe visual details that the source itself does not
   caption, and symbol fields emitted from images should be removed.
7. Verse lineation is literary evidence. Printed line order, refrain returns,
   enumerated stanzas, dialogue labels, and explicit attributions must be
   preserved even where punctuation is inconsistent.
8. Current repository spelling can help recognize an OCR substitution but
   cannot override a clear historical/source spelling, political term, name,
   or poetic form.
9. The classification under `suugaan` remains defensible because more than
   half the printed book is an authored performance and verse collection. Its
   documentary first part should remain with it so the resource represents the
   complete source, not a decontextualized poetry extract.

## Proposed source-faithful structure

```text
# Dhaartii Dhabta Ahayd

[untitled opening note]

## Khudbaddii ugu horreysay ... 23kii Oktoobar 1969
[complete speech]

## Halku-dhegga Wasiirka Wasaaradda Gaashaandhigga ...
[complete statement]

## Arar
[complete preface and authorial follow-up]

## Qaybta Hore ee Buugga
### 1. Ciidanka Xoogga Dalka Soomaaliyeed
### 2. Wax qabadkii Ciidanka Xoogga Dalka Soomaaliyeed ...
### 4. Sheekh Abaadir Muuse Warwaajecle ...
### 5. Halgankii Axmed Gurey
### 6. Halgankii dheeraa ee Sayid Maxamed Cabdulle Xasan
### 7. Dhalashadii Gobannimada

## Qaybta Labaad ee Buugga
### Gaadiidka beeshoo lagu guurayaan ahay
#### [explicit song / gabay / geeraar / murti divisions]

[closing author note and signature]
```

Captioned portrait and monument text should appear at its source position in a
restrained caption style rather than as sibling chapters.

## Cleanup gates

Cleanup must:

- inspect all 84 PDF pages directly and preserve every substantive speech,
  essay, list, song, gabay, geeraar, murti passage, transition, attribution,
  refrain, closing note, and meaningful image caption;
- correct the false `Arar` boundary and restore the source's two main parts,
  body chapter numbers, performance title, and explicit literary divisions;
- add the cover-supported year 1986 to the source registry;
- repair OCR substitutions, line-wrap joins, page transitions, and symbol
  noise only from scan evidence;
- preserve authorial and historical spelling where legible, including source
  inconsistencies that are not OCR mistakes;
- retain political, religious, military, romantic, and violent content as
  historical source material without present-day endorsement;
- record structural reconstruction, registry year, caption handling, major
  transcription ranges, and any intentional unresolved readings in the
  provenance log; and
- keep library markings, page numbers, uninterpretable image pixels, and
  converter debris out of the reader-facing body.

Cleanup must not:

- reduce the book to its poems or remove documentary prose as “not suugaan”;
- summarize an 81-page source into newly generated prose;
- silently correct historical claims, dates, political titles, ranks, or
  numbers using outside knowledge;
- rewrite ideological language into neutral modern commentary;
- modernize poems and songs at the expense of meter, alliteration, rhyme, or
  source diction;
- invent titles for verses introduced only by genre or speaker;
- invent visual descriptions for photographs; or
- mark cleanup approved or complete without maintainer review.

## Approval state

- **Audit approval:** approved by the maintainer on 2026-08-14 with the
  instruction, "go ahead."
- **Finding IDs:** SUU10-R001 through SUU10-R012
- **Cleanup:** applied on 2026-08-14
- **Cleanup approval:** pending
- **Complete:** no

## Cleanup result

The approved cleanup retained the complete documentary and literary work. It
did not abridge the presidential speech, ministerial statement, preface,
military-history chapters, songs, gabay, geeraarro, murti, political claims,
romantic dialogue, closing reflection, or author attribution.

The cleaned resource contains 2,056 lines, 11,274 words, and 81,617 bytes. Its
SHA-256 is
`415dc2d28cd497a1c18575ac1b250f86c00c1f5ff74da7f6fb558fad67fc24b8`.

Applied changes include:

- correction of the false opening `Arar` boundary and restoration of the true
  three-page `Arar` after the Maxamed Cali Samatar statement;
- restoration of the source's opening note, two main parts, six printed Part
  One body chapters, performance title, six song divisions, gabay, and all
  explicit geeraar or murti speaker divisions;
- preservation of the source's absent chapter number 3: the body proceeds
  from chapter 2 to chapter 4 and was not silently renumbered;
- scan-supported repair of high-confidence OCR substitutions, broken dates,
  names, headings, lists, paragraphs, page joins, and repeated zero-for-letter
  errors;
- restoration of readable verse lineation throughout Part Two with author and
  performer attributions retained;
- retention of the meaningful captions for Siyaad Barre, Maxamed Cali
  Samatar, Maxamed Nuur Xasan, S.Y.L., Dhagaxtuur, Axmed Gurey, Xaawo
  Cismaan, and Daljirka Dahsoon, while symbol-only picture OCR was removed;
- addition of the cover-supported publication year `1986` to the suugaan
  source registry; and
- removal only of printed page numbers, library/scan marks, uninterpretable
  image pixels, and converter debris.

The work's ideological and historical statements remain source content. They
were neither independently endorsed nor rewritten as current SLS historical
claims. Cleanup approval and completion remain for the maintainer.
