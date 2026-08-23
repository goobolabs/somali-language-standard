# Audit record — Sheekooyin laysku soo ururshay

- **Resource path:** `resources/suugaan/05-sheekooyin-laysku-soo-ururshay.md`
- **Collection / family:** suugaan / sheekooyin
- **Priority:** P1
- **Method:** whole-file, line-by-line literary-content audit; direct comparison
  with the local source scan
- **Audit status:** approved; source-guided cleanup applied, awaiting maintainer review
- **Audit date:** 2026-08-13
- **File size at audit:** 1,313 lines; 12,103 words; 62,994 bytes
- **Resource SHA-256:**
  `2250389e3da33f255a0a4d71ad079d99b9966c78892eca4e29cea84bfe88f475`
- **Resource-text changes during audit:** none

## Target output model

The cleaned file must remain a source-faithful literary collection. It must
retain the complete narration, dialogue, repetition, songs and verse, names,
quantities, humour, culturally marked content, and endings of all five
narratives. Cleanup may reconstruct page order, headings, paragraphs, dialogue,
verse layout, and line-wrap joins; it may remove only source-confirmed page
numbers, illustrations, title-page furniture, and OCR debris. It must not
summarize, modernize, sanitize, or retell the narratives.

## Controlling source and metadata finding

The controlling source is the local 45-image PDF:

`temp/soomaali/literature/Sheekooyin laysku soo ururshay.pdf`

The PDF is image-only and contains three separately titled booklets bound into
one scanned volume. The cover and all three internal title pages identify
Maxamuud Maxamed Cilmi as author and Muqdisho **1979** as the publication place
and year. The active source registry currently records 1978; that date is
contradicted by the scan and should be corrected to 1979 during approved
cleanup.

The bound-volume inventory is:

| Booklet | Narrative | Printed pages | PDF location |
| --- | --- | ---: | --- |
| *Dhuruq iyo Dayo* | Dhuruq iyo Dayo | 1-30 | PDF pages 2-18 |
| *Inantii Eeyadda Ahayd iyo Sheekooyin Kale* | Inantii Eeyadda Ahayd | 1-8 | PDF pages 18-23 |
| same booklet | Cali Caddaalad | 9-19 | PDF pages 23-28 |
| same booklet | Dakhtar Samatar | 20-26 | PDF pages 29-32 |
| *Faadumo Yar iyo Cumar Lug-Yare* | Faadumo Yar iyo Cumar Lug-Yare | 1-25 | PDF pages 32-45 |

The current Markdown has only four headings: one collection H1 and three H2
headings. `Inantii Eeyadda Ahayd` and `Cali Caddaalad` survive as damaged inline
text rather than Markdown story boundaries. The title page of the second
booklet is merged into the final line of `Dhuruq iyo Dayo`, while author, place,
and year fragments from later title pages enter adjacent story prose.

## Audit coverage and findings

| ID | Original lines | Class / severity | Source evidence and finding | Approved-action proposal |
| --- | ---: | --- | --- | --- |
| SUU05-R001 | 1-1,313 | hierarchy and metadata / fatal | Direct scan inventory proves three booklets, five narratives, and the year 1979. The Markdown exposes only three story H2s, flattens the booklet hierarchy, and the registry gives 1978. | Use one collection H1, three booklet H2s, and five narrative H3s. Correct the source-registry year to 1979. Keep author and publication metadata in the source registry/audit rather than repeating title-page furniture in the literary text. |
| SUU05-R002 | 3-362 | `Dhuruq iyo Dayo` / fatal | Source printed pages 1-30. Nearly every illustrated spread is read across both pages: sentences and speaker turns interleave, drawing strokes become characters, page numbers remain, words split at print wraps, and the second booklet title is appended to the final sentence. The entire plot is present but its reading order is unsafe. | Split every scan spread into left and right printed pages before transcription. Reconstruct all narration and dialogue in source order, retain the complete repeated deceptions and ending, and remove only verified drawings/page furniture. |
| SUU05-R003 | 364-548 | `Inantii Eeyadda Ahayd` / fatal | Source printed pages 1-8. The booklet title and 1979 title-page matter are corrupted; the narrative heading is not Markdown. Opposing pages are interleaved throughout Xaabeeya, Shankaroon, the prince, and Guhaad episodes. Numerous letter substitutions and false symbols alter names and clauses. | Restore the booklet and narrative boundaries, then reconstruct all eight pages scan-first. Preserve the complete transformation, both princes, the Guhaad ending, source terminology, and culturally marked statements without editorial rewriting. |
| SUU05-R004 | 549-827 | `Cali Caddaalad` / fatal | Source printed pages 9-19. Its title is flattened inline. Prose, the multi-stanza song with repeated refrain, the suicide attempts, arrest, execution attempts, palace episode, religious dialogue, and ending are repeatedly crossed by the facing page and page furniture. | Restore the narrative heading; retain every stanza and refrain as verse, all dialogue, plot events, names, quantities, and the printed conclusion. Do not sanitize or infer lines from metre or plot expectation. |
| SUU05-R005 | 828-974 | `Dakhtar Samatar` / fatal | Source printed pages 20-26. The H2 survives, but each source spread is interleaved line by line. Title-page author/year fragments contaminate the conclusion. Names, royal titles, commands, medical deception, quoted alternatives, and paragraph order are damaged. | Reconstruct the complete story page by page, retaining the domestic-violence premise, Samatar's deception, all royal dialogue, the fire test, confession, reconciliation, and ending. Remove the following booklet's title-page debris only after the story endpoint is verified. |
| SUU05-R006 | 975-1,313 | `Faadumo Yar iyo Cumar Lug-Yare` / fatal | Source printed pages 1-25. The heading survives, but extensive illustrations are OCR'd as text and facing-page prose is merged. Dialogue, songs, action order, names, and the final ambush are damaged; 162 nonblank lines contain four or fewer characters across the file, largely from illustration debris. | Split all illustrated spreads before transcription. Preserve the complete sibling opening, waraabe sequence, travel episodes, Faadumo's stratagems, songs, Cumar's final refrain, death, and closing marriage statement. Remove drawings and page numbers, not narrative content. |

## Cross-file and technical findings

1. The current resource is byte-identical to its only Git version, so history
   provides no cleaner text.
2. The PDF has no text layer. Automatic OCR may assist navigation but cannot be
   treated as textual authority.
3. The file contains 519 lines with likely scan/drawing/column debris, 162
   nonblank lines of four or fewer characters, several page-number lines, and
   only four Markdown headings for the required hierarchy.
4. The controlling reconstruction unit is the **printed page**, not the PDF
   image: most PDF images contain a two-page spread. Each spread must be split
   before reading order is established.
5. Illustrations are not required in the Markdown resource, but their OCR
   traces must be distinguished from nearby captions or narrative text before
   removal.
6. Violence, death, attempted suicide, animal transformation, religious
   language, disability-related wording, and gendered or social judgments are
   source content. Cleanup must preserve them without implying SLS endorsement.

## Proposed source-faithful structure

```text
# Sheekooyin laysku soo ururshay

## Dhuruq iyo Dayo
### Dhuruq iyo Dayo

## Inantii Eeyadda Ahayd iyo Sheekooyin Kale
### Inantii Eeyadda Ahayd
### Cali Caddaalad
### Dakhtar Samatar

## Faadumo Yar iyo Cumar Lug-Yare
### Faadumo Yar iyo Cumar Lug-Yare
```

Repeated booklet/narrative titles are intentional because the scan presents
`Dhuruq iyo Dayo` and `Faadumo Yar iyo Cumar Lug-Yare` as both booklet titles
and their sole narrative titles. If the maintainer prefers a flatter reader
presentation, the five narratives may instead be H2s, but the three-booklet
provenance must still be recorded explicitly.

## Cleanup gates

Cleanup must:

- process the source in printed-page order across all 45 PDF images;
- verify every paragraph and speaker turn directly against the scan;
- preserve songs and repeated lines in their printed order;
- keep all five complete narrative endings;
- remove only scan-confirmed illustrations, page numbers, and title furniture;
- record substantive reconstruction ranges in the provenance log; and
- update `resources/suugaan/00-sources.md` from 1978 to source-proven 1979.

Cleanup must not:

- treat the current OCR line order as source order;
- collapse the five narratives into three;
- summarize long illustrated sequences;
- invent missing words from narrative expectation;
- modernize names, dialogue, historical spelling, or culturally marked prose;
  or
- mark cleanup approved or complete without maintainer review.

## Cleanup result

The approved cleanup was applied on 2026-08-13. The 1,313-line unsafe OCR
extract was replaced by a 454-line, 8,675-word source-guided transcription.
The file now has the verified three-booklet/five-narrative hierarchy, continuous
printed-page order, clean prose paragraphs, three preserved verse passages, and
all five endings. Scan-confirmed illustrations, page numbers, binding/title-page
furniture, and OCR symbols were removed; the narrative content, dialogue,
names, quantities, violent or culturally marked material, and historical
wording were retained.

During cleanup, split-page verification corrected the audit's preliminary
boundary between two adjacent narratives: `Cali Caddaalad` occupies printed
pages 9-19, and `Dakhtar Samatar` pages 20-26. The source registry year was
corrected from 1978 to the title-page-supported 1979.

- **Cleaned file size:** 454 lines; 8,675 words; 54,496 bytes
- **Cleaned resource SHA-256:**
  `2b63e8b7164f4c63ffd1f5aee9f212286e3cc33ce6cbe2d0786572b12db6e2b7`
- **Applied finding IDs:** SUU05-R001 through SUU05-R006
- **Validation state:** structural and debris checks passed; maintainer cleanup
  review remains pending

## Approval state

- **Audit approval:** approved by the maintainer on 2026-08-13 with the
  instruction, "go ahead complete."
- **Approved finding IDs:** SUU05-R001 through SUU05-R006
- **Cleanup:** applied on 2026-08-13
- **Cleanup approval:** pending
- **Complete:** no
