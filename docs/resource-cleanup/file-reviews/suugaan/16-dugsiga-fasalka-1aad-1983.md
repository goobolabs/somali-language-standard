# Audit record — Suugaan, Fasalka Koowaad (1983)

- **Resource path:** `resources/suugaan/16-dugsiga-fasalka-1aad-1983.md`
- **Collection / family:** suugaan / dugsiga sare (school textbook)
- **Priority:** P1
- **Method:** whole-file literary audit; comparison with local image PDF and raw OCR
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit date:** 2026-08-19
- **File size at audit start:** 3,280 lines
- **Resource SHA-256 at audit start:**
  `d21053ad8f8b04a3770a63ff98e924e8b7d3b9f65696a6fff0a9114e9ed43e9e`
- **Resource-text changes during audit:** none (audit); phase-2 replacement (cleanup)

## Target output model

The cleaned file must remain a source-faithful 1983 Ministry textbook: *Suugaan
— Fasalka Koowaad* (revised edition). Structure parallels the 1976 syllabus:
*Hordhac*, *Gogoldhig*, anthology sections (*Gabayo*, *Geeraar*, *Sheekooyin*,
*Curisyo*, *Murti* / *Maahmaahyo*, *Riwaayad*, *Laylis*). Cleanup may repair OCR,
restore verse and prose layout, and omit page furniture. It must not modernize
diction or reorder anthology entries.

## Controlling source

| Asset | Role |
| --- | --- |
| `temp/soomaali/literature/Suugaan - Fasalka koowaad 1 - Dugsiga Sare (1983)64pag.pdf` | Image scan, **71 pages**; controlling evidence |
| `temp/md/literature/Suugaan - Fasalka koowaad 1 - Dugsiga Sare (1983)64pag.md` | Raw OCR; same damage patterns |

## Findings

| ID | Severity | Finding | Resolution |
| --- | --- | --- | --- |
| SUU16-R001 | low | H1 plus printed syllabus sections | **Retained**; section H2 boundaries restored from PDF |
| SUU16-R002 | fatal | Column-merge OCR corruption | **Resolved** via 3-batch PDF transcription |
| SUU16-R003 | high | Mechanical `00`, pipe debris, column fragments | Superseded by PDF pass |
| SUU16-R004 | medium | Corrupted `## Gabay:` headings from column splits | **Restored** from PDF |
| SUU16-R005 | low | Ministry imprint and pagination | **Omitted** from body |

## Approval state

- **Audit approval:** approved by the maintainer on 2026-08-19 with the instruction, "go ahead."
- **Finding IDs:** SUU16-R001 through SUU16-R005
- **Cleanup:** phase-2 PDF transcription (2026-08-19; 3 batches, pages 1–71)
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Cleanup result

Full source-faithful transcription from PDF page images (71 pages). Includes
pedagogical framing (*Laylis*, *Weydiimo*, *Sharrax*) as printed. Sections read
continuously from *Hordhac* through *Riwaayad: Dagaalkii Beder* (Aragtida I–V).
The independently reviewed transcription batches were merged into one continuous file.

- **Post-cleanup:** 1,719 lines
- **Resource SHA-256 after cleanup:**
  `10e7a504a39358f60616667d2a5e57568412cb0523a6afa5911fdcf0ae3b4c3a`
- **PDF mapping:** 3 parts → `/tmp/suugaan16-part{1..3}.md`; PNGs at `/tmp/suugaan16-ocr/`
- **Status:** complete; cleanup approved 2026-08-23
