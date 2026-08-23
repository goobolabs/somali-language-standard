# Audit record — Xeebtii Dahabka iyo Waayihii Saxardiid

- **Resource path:** `resources/suugaan/13-xeebtii-dahabka.md`
- **Collection / family:** suugaan / sheeko carruurta (Qaalib, 1977)
- **Priority:** P1
- **Method:** whole-file literary audit; PDF page-image transcription (phase 2)
- **Audit status:** complete
- **Audit date:** 2026-08-19
- **File size at audit start:** 873 lines (unreadable OCR import)
- **Resource SHA-256 at audit start:**
  `e5e25125899bcc5a2f17d5f249c57e76ea819a019dedc972cca1f4b5667d1a09`
- **Resource-text changes during audit:** none

## Target output model

Two school-reader stories in source order: **Xeebtii Dahabka iyo Waayihii Saxardiid**
(Saxardiid, Timir, golden shell) and **Jar-iskaxuurkii Yaraa** (Anwar, Koosiyaak,
Arctic adventure). One H1, two `##` story headings, readable paragraphs, «guillemet»
dialogue, no page furniture.

## Controlling source

| Asset | Role |
| --- | --- |
| `temp/soomaali/literature/Xeebtii Dahabka iyo Waayihii Saxardiid_lavorato.pdf` | Image scan, 30 pages |
| Page images `p-03.png`–`p-30.png` | Phase-2 transcription source |

## Findings (summary)

| ID | Severity | Resolution |
| --- | --- | --- |
| SUU13-R001 | low | Structure retained (H1 + two stories) |
| SUU13-R002 | fatal | Column-merge corruption → **resolved** via PDF transcription |
| SUU13-R003 | high | Mechanical OCR → superseded by full transcription |
| SUU13-R004 | medium | Page markers → omitted in body |
| SUU13-R005 | low | Printed pagination prefixes → normalized in transcription |

## Approval state

- **Audit approval:** 2026-08-19 ("go ahead")
- **Cleanup:** phase-1 (2026-08-19) + **phase-2 PDF transcription** (2026-08-19)
- **Cleanup approval:** pending
- **Complete:** no

## Cleanup result

Full source-faithful transcription from PDF pages 3–30 (printed pages 1–54). Both
stories read as continuous Somali prose. No `[illegible]` gaps reported.

- **Post-cleanup:** 279 lines
- **Resource SHA-256 after cleanup:**
  `5885a696be09a1340a1ae15802adb8171040000dcdfcf829bfe3d40a02f49a63`
- **PDF mapping:** Story 1 = images p-03–p-17; Story 2 = p-18–p-30
- **Status:** awaiting cleanup approval
