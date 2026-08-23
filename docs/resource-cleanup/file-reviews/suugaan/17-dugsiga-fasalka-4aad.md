# Audit record — Suugaanta, Fasalka Afraad

- **Resource path:** `resources/suugaan/17-dugsiga-fasalka-4aad.md`
- **Collection / family:** suugaan / dugsiga sare (school textbook)
- **Priority:** P1
- **Method:** whole-file literary audit; comparison with local image PDF and raw OCR
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit date:** 2026-08-19
- **File size at audit start:** 2,417 lines
- **Resource SHA-256 at audit start:**
  `57444131eed5c491184ff2c233dbdad86698bc1d76772405d4436076d550315a`
- **Resource-text changes during audit:** none (audit); phase-2 replacement (cleanup)

## Target output model

The cleaned file must remain a source-faithful Ministry textbook: *Suugaanta —
Fasalka Afraad*. Structure includes *Hordhac*, *Tusmada buugga*, anthology
sections (*Geeraar*, *Gabay*, *Buraanbur*, *Curisyo*, *Halxidhaale*), and
pedagogical *Laylis* blocks. Cleanup may repair OCR, restore verse and prose
layout, and omit page furniture. It must not modernize diction or reorder
anthology entries.

## Controlling source

| Asset | Role |
| --- | --- |
| `temp/soomaali/literature/Sugaanta- Fasalka Afraad 4.pdf` | Image scan, **73 pages**; controlling evidence |
| `temp/md/literature/Sugaanta- Fasalka Afraad 4.md` | Raw OCR; same damage patterns |

## Findings

| ID | Severity | Finding | Resolution |
| --- | --- | --- | --- |
| SUU17-R001 | low | H1 plus printed syllabus sections | **Retained**; section H2 boundaries restored from PDF |
| SUU17-R002 | fatal | Column-merge OCR corruption throughout (e.g. Hordhac lines 28–60, garbled `## Gabay:` headings) | **Resolved** via PDF transcription |
| SUU17-R003 | high | Mechanical `00`, pipe debris, Latin garbage strings (`keyed ueany ey NpoosisBnp`) | Superseded by PDF pass |
| SUU17-R004 | medium | Corrupted section headings from column splits (`## Gabay: gan o uu tiriyey…`) | **Restored** from PDF |
| SUU17-R005 | low | Ministry imprint and pagination | **Omitted** from body |

## Approval state

- **Audit approval:** approved by the maintainer on 2026-08-19 with the instruction, "go ahead."
- **Finding IDs:** SUU17-R001 through SUU17-R005
- **Cleanup:** phase-2 PDF transcription (2026-08-19; 3 batches, pages 1–73)
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Cleanup result

Full source-faithful transcription from PDF page images (73 pages). Includes
pedagogical framing (*Laylis*, *Maahmaaho*, *Halxidhaale*) and anthology sections
(*Hees*, *Gabay*, *Geeraar*, *Buraanbur*, *Sheeko*, *Curis*) as printed. The
independently reviewed transcription batches were merged into one continuous file.

- **Post-cleanup:** 1,444 lines
- **Resource SHA-256 after cleanup:**
  `fa62852bccd3539083f06d288ab858cadb15a26ac52fe0d66f8ca20b1168a185`
- **PDF mapping:** 3 parts → `/tmp/suugaan17-part{1..3}.md`; PNGs at `/tmp/suugaan17-ocr/`
- **Status:** complete; cleanup approved 2026-08-23
