# Audit record — Hal Ka Haleel

- **Resource path:** `resources/suugaan/07-hal-ka-haleel.md`
- **Collection / family:** suugaan / sooyaal, faaqidaad, iyo maanso
- **Priority:** P1
- **Method:** whole-file, line-by-line literary-content audit; direct comparison
  with the local source scan
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit date:** 2026-08-14
- **File size at audit:** 6,413 lines; 64,253 words; 414,794 bytes
- **Resource SHA-256:**
  `c6c134e47441c6520a218eeda59944aca4d4c56118e42419810408d987f9277e`
- **Resource-text changes during audit:** none

## Target output model

The cleaned file must remain a source-faithful edition of Maxamed Baashe X.
Xasan's biographical and critical study of Maxamed Ibraahim Warsame
(Hadraawi). It is not merely a collection of poems. It combines biography,
oral testimony, historical narrative, literary criticism, quoted verse, eight
extended poem studies, references, selected verse extracts, an author
biography, and a photographic appendix.

Cleanup must retain the author's complete substantive Somali prose, direct
quotations, analytical claims, political and historical discussion, verse
lines, poem order, dates, names, notes, dedication, preface, acknowledgements,
references, author biography, and meaningful photo captions. It may restore
page order, hierarchy, paragraphs, quotations, verse layout, footnotes, and
line-wrap joins. It must not abridge the book, rewrite its criticism, update
historical claims, modernize Hadraawi's verse, or silently import alternative
versions of poems from another edition.

## Controlling source and metadata

The controlling source is the local image-only PDF:

`temp/soomaali/literature/Hal Ka Haleel.pdf`

The scan contains 155 images and no text layer. Most images are two-page
spreads, covering a 304-page printed book plus the cover and back cover. The
cover and title page identify:

- title: *Hal Ka Haleel*;
- subtitle: *Sooyaalka Hadraawi iyo Suugaantiisa*;
- author: Maxamed Baashe X. Xasan;
- place and year: London, 2004; and
- publisher: Bashe Publications.

The copyright page identifies the first edition as May 2004 and gives ISBN
`0-9547674-0-3`. The active source registry's author and year are therefore
source-supported; no metadata correction is proposed. Publisher, ISBN,
addresses, emails, and obsolete website details belong in provenance rather
than in the reader-facing literary text.

The scan is readable enough for a full reconstruction, but each two-page image
must first be split into left and right printed pages. The present OCR often
reads horizontally across both pages, interleaving unrelated prose and verse.
Automatic OCR may assist navigation but cannot establish authoritative wording
or line order.

## Source contents inventory

The printed `Tusmo` gives the following major structure:

| Part | Unit | Printed page |
| --- | --- | ---: |
| Qaybta Koowaad | Hibeyn | 5 |
| Qaybta Koowaad | Hor-dhac | 6 |
| Qaybta Koowaad | Hanaqaadkii Hadraawi | 11 |
| Qaybta Koowaad | Sidii uu magaca `Hadraawi` ku baxay | 14 |
| Qaybta Koowaad | Dhalashadii hal-abuur-nimadiisa | 15 |
| Qaybta Koowaad | Xadhiggii iyo Noloshii Xabsiga | 26 |
| Qaybta Koowaad | Maansada Hadraawi iyo 1970-naadkii | 41 |
| Qaybta Koowaad | Faaqidaadda maansadiisa meelaha qaar | 66 |
| Qaybta Koowaad | Tis-qaadka Maansadiisa Rubucii hore ee 1980-da | 75 |
| Qaybta Koowaad | Maansadiisa Rubucii 2aad 1980-1990 | 101 |
| Qaybta Koowaad | Shakhsiyadda Hadraawi | 161 |
| Qaybta Koowaad | Muuqaalka Hadraawi horraantii 1990-naadka | 167 |
| Qaybta Labaad | Maansadiisa Run ku sii durugga ah | 169 |
| Qaybta Labaad | Daba-Huwan | 172 |
| Qaybta Labaad | Badbaado | 204 |
| Qaybta Labaad | Hal iyo Halkiis | 212 |
| Qaybta Labaad | Suubban | 226 |
| Qaybta Labaad | Af-ku-Siran | 239 |
| Qaybta Labaad | Adduunyadu waa Besteed | 260 |
| Qaybta Labaad | Rag Siyaasi wada Noqoy | 273 |
| Qaybta Labaad | Anuun baa Hooyadaa ah | 284 |
| End matter | Ilaha xigashada buugga | 290 |
| End matter | Dheegag-maanseed xulasho ah | 291 |
| End matter | Iftiimin ku saabsan qoraaga | 296 |
| End matter | Lifaaq Sawirro ah | 298-304 |

`Mahadnaq` is a substantive internal heading in the printed preface although it
is not separately listed in the contents. It must also be restored.

## Current structural condition

The Markdown exposes only four headings: one H1, `Hibeyn`, `Maansada Run Ku
Sii Durugga Ah`, `Anuunbaa Hooyadaa Ah`, and a single merged heading for
`Ilaha ... Dheegag-Maanseedyo`. All other printed boundaries are flattened
inside prose or joined to the opposite page. This loses both the two-part book
architecture and the distinction between biography, literary analysis, poem
introductions, complete poems, references, and appendices.

The current file begins correctly with the substantive dedication rather than
title-page furniture. However, its first spread already demonstrates the
controlling defect: printed pages 6 and 7 are read across each other, so the
left-page `Hor-dhac` and right-page continuation alternate on the same lines.
The same collision recurs throughout the biography, criticism, and poems.

## Audit coverage and findings

| ID | Original lines | Class / severity | Source evidence and finding | Proposed action |
| --- | ---: | --- | --- | --- |
| SUU07-R001 | 1-6,413 | hierarchy, metadata, and page order / fatal | Direct cover, title-page, contents, and whole-scan inspection confirms a 304-page, two-part study with 25 listed content/end-matter units plus `Mahadnaq`. The Markdown has only four headings. Most PDF images contain facing pages, and current OCR repeatedly interleaves them. | Reconstruct printed-page order before word-level cleanup. Restore the two-part hierarchy and every source heading. Keep source-supported metadata in the registry/audit and remove running furniture from the literary body. |
| SUU07-R002 | 3-122 | dedication, preface, and acknowledgements / fatal | `Hibeyn`, `Hor-dhac`, and `Mahadnaq` are substantive authorial text, but printed pages 6-10 are crossed with their facing pages. Names, roles, the explanation of the book's title, and acknowledgements are joined into unrelated sentences. | Retain all three units completely. Split pages, restore paragraphs and lists, and preserve the author's account of scope and method. Do not replace the preface with an SLS summary. |
| SUU07-R003 | 96-764 | early biography through imprisonment / fatal | `Hanaqaadkii Hadraawi`, the origin of the name Hadraawi, the birth of his creativity, and the Qansax-dheere imprisonment section are mostly unheaded. Prose, direct testimony, dates, names, quoted verse, and facing-page sections collide. | Restore the four printed headings and every subordinate source heading. Reconstruct prose and quotations page by page; preserve biographical claims as attributed source content rather than independently rewriting them. |
| SUU07-R004 | 765-1,474 | 1970s poetry and critical assessment / fatal | The `Maansada Hadraawi iyo 1970-naadkii` and `Faaqidaadda ...` sections contain long prose analysis interwoven with verse quotations from facing pages. Speaker/poem attribution, stanza order, dates, and critical judgments are unsafe in the OCR sequence. | Separate analysis from quoted verse, retain all poem labels and dates, and verify each stanza line against the scan. Repository parallels may corroborate verse but may not replace this edition's selection or wording. |
| SUU07-R005 | 1,475-2,019 | early 1980s survey / fatal | `Tis-qaadka Maansadiisa Rubucii hore ee 1980-da` is flattened and crossed page by page. Discussions of the `Deelley`, poems, political circumstances, prose commentary, and quoted lines merge. | Restore the section hierarchy, prose sequence, poem titles, dates, verse blocks, and attributions directly from printed pages 75-100. Preserve historical and political wording without editorial normalization. |
| SUU07-R006 | 2,020-3,479 | later 1980s, personality, and early-1990s portrait / fatal | `Maansadiisa Rubucii 2aad 1980-1990`, `Shakhsiyadda Hadraawi`, and `Muuqaalka ...` lack reliable boundaries. Dense criticism and verse are interleaved across spreads; some headings are unreadable symbol strings. | Rebuild all three source units in printed order. Preserve the full analysis, quotations, poem sequence, and transition into the second part. Do not collapse repeated-looking verse without scan confirmation. |
| SUU07-R007 | 3,480-4,264 | second-part introduction and `Daba-Huwan` / fatal | The manually exposed `Maansada Run Ku Sii Durugga Ah` heading does not solve the underlying crossed-page order. `Daba-Huwan` begins inline and runs through more than thirty printed pages of commentary and verse with headings, stanzas, and prose repeatedly mixed. | Restore `Qaybta Labaad`, its introduction, and `Daba-Huwan` as separate units. Verify every prose paragraph and verse line from printed pages 169-203, retaining the complete critical discussion and poem. |
| SUU07-R008 | 4,265-4,747 | `Badbaado` and `Hal iyo Halkiis` / fatal | Both titles survive only as inline capitals. Their introductions, dates, explication, refrains, and verse lines are merged with facing-page commentary. | Restore two H3 units and reconstruct printed pages 204-225 in order. Preserve repeated formulas, stanza divisions, and all analytical prose. |
| SUU07-R009 | 4,748-5,752 | `Suubban`, `Af-ku-Siran`, and `Adduunyadu waa Besteed` / fatal | Three long poem studies occupy printed pages 226-272. Current OCR loses boundaries and alternates columns/pages, producing prose inside stanzas and stanzas inside prose; title spellings are inconsistent. | Restore three source titles and rebuild each introduction, commentary sequence, and complete poem. Use the scan as controlling evidence for capitalization, hyphenation, refrain, and lineation. |
| SUU07-R010 | 5,753-6,190 | `Rag Siyaasi wada Noqoy` and `Anuun baa Hooyadaa ah` / fatal | The first title is inline and the second has an H2 placed amid an interleaved spread. Discussions of fatherhood and motherhood, dates, prose, and long verse passages cross their facing pages. | Restore two H3 units and reconstruct printed pages 273-289. Preserve the author's related argument across both poems and the exact verse order through the end of `Anuun baa Hooyadaa ah`. |
| SUU07-R011 | 6,191-6,413 | references, extracts, author biography, and photo appendix / fatal | `Ilaha` and `Dheegag-maanseedyo` are wrongly merged into one heading. Selected extracts on pages 291-295 are interleaved by page; the author biography on pages 296-297 is crossed; photo-page scan noise obscures captions on pages 298-304. | Restore four end-matter units. Keep the reference list, every selected extract and attribution, the complete author biography, and meaningful photo captions. Omit image pixels/OCR noise and blank photo-page space; do not fabricate descriptions not printed as captions. |

## Cross-file and technical findings

1. The current resource is derived from the same OCR family as the temporary
   conversion and has no cleaner version in Git history.
2. The file contains 145 running-header occurrences involving `Hal Ka Haleel`,
   568 lines with likely `00`-for-`oo` substitutions, 315 lines with raw symbol
   debris, 75 nonblank lines of four or fewer characters, and 33 lines longer
   than 160 characters. These are risk indicators, not automatic deletion
   rules.
3. Printed page numbers and running titles are frequently fused with prose or
   verse. Removal must follow visual page verification.
4. Poetry lineation is semantic content. Stanzas, refrains, indentation,
   parentheses identifying poem sources, and prose/verse boundaries must be
   reconstructed before spelling repair.
5. `resources/suugaan/14-hal-karaan-hadrawi.md` and other Hadraawi resources may
   contain parallel lines. They can corroborate an illegible reading, but the
   *Hal Ka Haleel* scan controls which lines are quoted, their local ordering,
   and Maxamed Baashe's surrounding commentary.
6. Political, religious, historical, biographical, and evaluative statements
   are attributed book content. Cleanup preserves them without converting them
   into current SLS factual assertions or endorsements.
7. Names written with initials, source-era spellings, quotations, and poem
   titles must not be silently standardized merely for consistency.

## Proposed source-faithful hierarchy

```text
# Hal Ka Haleel

## Qaybta Koowaad — Sooyaalka Hadraawi iyo Suugaantiisa
### Hibeyn
### Hor-dhac
### Mahadnaq
### Hanaqaadkii Hadraawi
### Sidii uu magaca “Hadraawi” ku baxay
### Dhalashadii hal-abuur-nimadiisa
### Xadhiggii iyo Noloshii Xabsiga
### Maansada Hadraawi iyo 1970-naadkii
### Faaqidaadda maansadiisa meelaha qaar
### Tis-qaadka Maansadiisa Rubucii hore ee 1980-da
### Maansadiisa Rubucii 2aad 1980-1990
### Shakhsiyadda Hadraawi
### Muuqaalka Hadraawi horraantii 1990-naadka

## Qaybta Labaad — Maansada Run ku sii Durugga ah
### Maansadiisa Run ku sii durugga ah
### Daba-Huwan
### Badbaado
### Hal iyo Halkiis
### Suubban
### Af-ku-Siran
### Adduunyadu waa Besteed
### Rag Siyaasi wada Noqoy
### Anuun baa Hooyadaa ah

## Tixraac iyo lifaaqyo
### Ilaha laga soo xigtey buuggan
### Dheegag-maanseedyo xulasho ah
### Iftiimin ku saabsan qoraaga buugga
### Lifaaq Sawirro ah
```

Additional subordinate headings printed inside these units should be retained
at H4 rather than flattened. The title-page subtitle may be recorded in a short
source note or collection index; it need not become invented introductory
prose.

## Cleanup gates

Cleanup must:

- split and inspect all two-page images in left-to-right printed-page order;
- retain all substantive content from printed pages 5-304;
- restore both parts, all contents-listed units, `Mahadnaq`, and printed
  subordinate headings;
- separate prose, direct testimony, verse quotations, full poems, references,
  lists, footnotes, author biography, and captions according to the scan;
- verify titles, dates, names, stanza lines, refrains, and attributions directly
  against the source;
- remove only verified cover/title/copyright/contents furniture, running
  headers, page numbers, blank image space, and OCR-only symbols; and
- record each large reconstruction range in the provenance log.

Cleanup must not:

- turn the book into a short Hadraawi biography or selected-poems summary;
- omit the author's preface, acknowledgements, criticism, references, or
  end-matter biography as “non-literary”;
- merge Maxamed Baashe's commentary with Hadraawi's verse;
- infer missing lines from metre, rhyme, politics, biography, or another poem
  edition without direct source support;
- modernize historical quotations, names, or politically marked wording; or
- mark cleanup approved or complete without maintainer review.

## Approval state

- **Audit approval:** approved by the maintainer on 2026-08-14
- **Finding IDs:** SUU07-R001 through SUU07-R011
- **Cleanup:** applied on 2026-08-14
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Cleanup result

The approved reconstruction was applied from the controlling scan. Each
two-page image was separated into left and right printed pages, the body region
was extracted without the running-title/footer strips, and printed pages 5-297
were reassembled in page order. The meaningful captions from printed pages
298-304 were retained as the photographic appendix.

The cleaned resource now contains 11,673 lines, 61,659 words, and 406,359
bytes. Its SHA-256 is
`d70057b60a499539a034fc2614ae279d78fc37ce28faf94af06b8f5f93bfc5d1`.

Applied changes include:

- the two book parts and the end-matter division;
- all 26 substantive H3 units, including `Mahadnaq`;
- the eleven verse divisions and eleven explanation divisions inside `Hal iyo
  Halkiis`;
- complete source-ordered biography, testimony, historical and political
  discussion, literary criticism, verse, poem studies, references, selected
  extracts, author biography, and meaningful photo captions;
- separation of formerly interleaved facing pages; and
- removal of cover/publication furniture, running headers, printed page
  numbers, recurring subtitle footers, blank photo space, and OCR-only symbols.

No literary section was summarized or replaced with new editorial prose. The
source registry already had the scan-supported author and 2004 date, so no
registry change was necessary. Cleanup was approved by the maintainer on
2026-08-23 and is complete.
