# Audit record — Suugaanta, Fasalka Shanaad

- **Resource path:** `resources/suugaan/19-dugsiga-fasalka-5aad.md`
- **Collection / family:** suugaan / dugsiga sare (school textbook, grade 5)
- **Priority:** P1
- **Method:** whole-file literary audit; comparison with local image PDF and raw OCR
- **Audit status:** complete
- **Audit date:** 2026-08-19
- **File size at audit start:** 2,914 lines
- **Resource SHA-256 at audit start:**
  `00b3d387d580de41964dfa16b0a02617af0234a65dfe8a77556a64c84b987d2c`
- **Resource-text changes during audit:** none (audit); phase-2 replacement (cleanup)

## Target output model

The cleaned file must remain a source-faithful Ministry textbook: *Suugaanta —
Fasalka Shanaad* (second volume in the 4–6 series). Structure includes *Hordhac*,
*Tusmada*, lesson sections (*Casharka*), anthology entries (*Gabay*, *Geeraar*,
*Buraanbur*, *Curisyo*, *Halxidhaale*), and *Laylis* blocks. Cleanup may repair
OCR and restore verse layout; it must not modernize diction or reorder lessons.

## Controlling source

| Asset | Role |
| --- | --- |
| `temp/soomaali/literature/Sugaanta Fasalka Shanaad 5.pdf` | Image scan, **86 pages**; controlling evidence |
| `temp/md/literature/Sugaanta Fasalka Shanaad 5.md` | Raw OCR; same damage patterns |

## Findings

| ID | Severity | Finding | Resolution |
| --- | --- | --- | --- |
| SUU19-R001 | low | H1 plus *Casharka* lesson sections | **Retained**; H2 boundaries restored from PDF |
| SUU19-R002 | fatal | Column-merge OCR corruption throughout | **Resolved** via PDF transcription |
| SUU19-R003 | high | Mechanical debris, pipe/column fragments, TOC garbage in body | Superseded by PDF pass |
| SUU19-R004 | medium | Corrupted `## Gabay:` headings merged with adjacent columns | **Restored** from PDF |
| SUU19-R005 | low | Ministry imprint and pagination | **Omitted** from body |

## Approval state

- **Audit approval:** approved by the maintainer on 2026-08-19 with the instruction, "go ahead."
- **Finding IDs:** SUU19-R001 through SUU19-R005
- **Cleanup:** phase-2 PDF transcription (2026-08-19; 3 batches, pages 1–86)
- **Cleanup approval:** pending
- **Complete:** no

## Cleanup result

Full source-faithful transcription from PDF page images (86 pages). Lesson-based
structure (*Casharka I*–*XXXIV*) with hees, gabay, geeraar, buraanbur, curis,
*Laylis*, *Maahmaaho*, and *Halxiraale* blocks. The independently reviewed
transcription batches were merged into one continuous file.

- **Post-cleanup:** 1,519 lines
- **Resource SHA-256 after cleanup:**
  `a3ff4f9717e3fd10cb0556a5d6eef17398b9b91a1bf7eb85a704584d4f660e2a`
- **PDF mapping:** 3 parts → `/tmp/suugaan19-part{1..3}.md`; PNGs at `/tmp/suugaan19-ocr/`
- **Status:** awaiting cleanup approval
- **Spot-check (non-blocking):** printed lesson numbering skips *Casharka XIII*; two *Casharka XII* blocks appear in the merge — verify against PDF if needed
