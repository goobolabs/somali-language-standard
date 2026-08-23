# Audit record — Buugga Suugaanta, Fasalka Afraad

- **Resource path:** `resources/suugaan/18-dugsiga-fasalka-4aad-buugga.md`
- **Collection / family:** suugaan / dugsiga sare (gabay anthology)
- **Priority:** P1
- **Method:** whole-file literary audit; comparison with local image PDF and raw OCR
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit date:** 2026-08-19
- **File size at audit start:** 2,367 lines
- **Resource SHA-256 at audit start:**
  `2736cc24c8c2ea74ed40a05122ade90139c20373e218d9e2c30e818609893aac`
- **Resource-text changes during audit:** none (audit); phase-2 replacement (cleanup)

## Target output model

The cleaned file must remain a source-faithful Ministry anthology: *Buugga
Suugaanta — Dugsiga Sare, Fasalka Afraad*. Structure is lesson-based (*Casharka
I*–*XXIV*), each framing gabay/geeraar entries with biographical notes,
*Laylis*, and glossaries. Cleanup may repair OCR, restore verse layout, and omit
page furniture. It must not modernize diction or reorder lessons.

## Controlling source

| Asset | Role |
| --- | --- |
| `temp/soomaali/literature/Buugga Suugaanta Dugsiga Sare Fasalka Afraad_lavorato.pdf` | Image scan, **65 pages**; controlling evidence |
| `temp/md/literature/Buugga Suugaanta Dugsiga Sare Fasalka Afraad_lavorato.md` | Raw OCR; same damage patterns |

## Findings

| ID | Severity | Finding | Resolution |
| --- | --- | --- | --- |
| SUU18-R001 | low | H1 plus *Casharka* lesson sections | **Retained**; H2 boundaries restored from PDF |
| SUU18-R002 | fatal | Column-merge OCR corruption throughout (e.g. lines 32–50, garbled `## Gabay:` headings) | **Resolved** via PDF transcription |
| SUU18-R003 | high | Mechanical debris, pipe/column fragments, Latin garbage | Superseded by PDF pass |
| SUU18-R004 | medium | Corrupted gabay headings merged with adjacent column text | **Restored** from PDF |
| SUU18-R005 | low | Ministry imprint and pagination | **Omitted** from body |

## Approval state

- **Audit approval:** approved by the maintainer on 2026-08-19 with the instruction, "go ahead."
- **Finding IDs:** SUU18-R001 through SUU18-R005
- **Cleanup:** phase-2 PDF transcription (2026-08-19; 3 batches, pages 1–65)
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Cleanup result

Full source-faithful transcription from PDF page images (65 pages). Lesson-based
structure (*Casharka I*–*XXIV*) with gabay/geeraar entries, biographical notes,
*Laylis*, glossaries, and closing *Murtida* section. The independently reviewed
transcription batches were merged into one continuous file.

- **Post-cleanup:** 2,087 lines
- **Resource SHA-256 after cleanup:**
  `6e8b52cc5fa1184d96be1b595f38475e9beffca80974347df9ed1d3d67e5f634`
- **PDF mapping:** 3 parts → `/tmp/suugaan18-part{1..3}.md`; PNGs at `/tmp/suugaan18-ocr/`
- **Status:** complete; cleanup approved 2026-08-23
