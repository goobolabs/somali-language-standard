# Audit record — Suugaan, Fasalka Koowaad (1976)

- **Resource path:** `resources/suugaan/15-dugsiga-fasalka-1aad-1976.md`
- **Collection / family:** suugaan / dugsiga sare (school textbook)
- **Priority:** P1
- **Method:** whole-file literary audit; comparison with local image PDF and raw OCR
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit date:** 2026-08-19
- **File size at audit start:** 4,107 lines
- **Resource SHA-256 at audit start:**
  `00a7d27e613c5fb2ccdde645b4e2680b57b6f5292ee7168a4679b8b0854be759`
- **Resource-text changes during audit:** none (audit); phase-2 replacement (cleanup)

## Target output model

The cleaned file must remain a source-faithful 1976 Ministry textbook: *Suugaan
— Dugsiga Sare, Fasalka Koowaad*. Structure follows the printed syllabus:
*Hordhac*, pedagogical *Gogoldhig* (*Suugaantu Waa Maxay?*), then anthology
sections — *Gabayo*, *Geeraar*, *Sheekooyin*, *Curisyo*, *Murti* / *Maahmaahyo*,
*Riwaayad*, and *Laylis* (exercises). Cleanup may repair OCR, restore verse and
prose layout, and omit page furniture and duplicate TOC debris. It must not
modernize diction, reorder anthology entries, or drop pedagogical framing.

## Controlling source

| Asset | Role |
| --- | --- |
| `temp/soomaali/literature/suugaan - Dugsida Sare - Fasalka Kowaad (1976).pdf` | Image scan, **100 pages**; controlling evidence |
| `temp/md/literature/suugaan - Dugsida Sare - Fasalka Kowaad (1976).md` | Raw OCR (4,356 lines); same damage patterns |

## Findings

| ID | Severity | Finding | Resolution |
| --- | --- | --- | --- |
| SUU15-R001 | low | H1 plus printed syllabus sections | **Retained**; section H2 boundaries restored from PDF |
| SUU15-R002 | fatal | Column-merge OCR corruption | **Resolved** via 4-batch PDF transcription |
| SUU15-R003 | high | Mechanical `00`, pipe debris, Latin OCR | Superseded by PDF pass |
| SUU15-R004 | medium | Duplicate TOC headings in body | **Omitted**; real sections from PDF |
| SUU15-R005 | low | Ministry imprint and pagination | **Omitted** from body |

## Approval state

- **Audit approval:** approved by the maintainer on 2026-08-19 with the instruction, "go ahead."
- **Finding IDs:** SUU15-R001 through SUU15-R005
- **Cleanup:** phase-2 PDF transcription (2026-08-19; 4 batches, pages 1–100)
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Cleanup result

Full source-faithful transcription from PDF page images (100 pages). Sections
read continuously from *Hordhac* through *Riwaayad: Dagaalkii Beder*. Verse,
prose commentary, exercises (*Laylis*), and vocabulary lists preserved. The
independently reviewed transcription batches were merged into one continuous file.

- **Post-cleanup:** 1,747 lines
- **Resource SHA-256 after cleanup:**
  `a639769c783d018a6d6a274db2934018742533c720ee15efad06a672767864fb`
- **PDF mapping:** 4 parts → `/tmp/suugaan15-part{1..4}.md`; PNGs at `/tmp/suugaan15-ocr/`
- **Status:** complete; cleanup approved 2026-08-23
