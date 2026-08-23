# Audit record — Hal-Karaan (Hadraawi)

- **Resource path:** `resources/suugaan/14-hal-karaan-hadrawi.md`
- **Collection / family:** suugaan / maanso (diiwaan)
- **Priority:** P1
- **Method:** whole-file literary audit; comparison with local image PDF and raw OCR
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit date:** 2026-08-19
- **File size at audit start:** 9,119 lines
- **Resource SHA-256 at audit start:**
  `3de282438095b79834b15ca05c652dc41dbb2de9e595b33572cf08543ed82e29`
- **Resource-text changes during audit:** none (audit); phase-2 replacement (cleanup)

## Target output model

The cleaned file must remain a source-faithful edition of Maxamed Ibraahim
Warsame (Hadraawi)'s poetry collection *Hal-Karaan*: front matter (*Waa kuma
Hadraawi?*, *Mahadnaq*, *Gogoldhig*, *Ereyga curiyaha*, *Tilmaan*), four poem
parts (*Qaybta Kowaad*–*Afraad*), and closing *Xusuus* glossary. Cleanup may
repair scan-proven OCR, restore stanza and poem layout, and omit page furniture.
It must not abridge poems, modernize diction, or reorder the author's sequence.

## Controlling source

| Asset | Role |
| --- | --- |
| `temp/soomaali/literature/Hal-Karaan - Maxamed Ibraahim Warsama (Hadrawi).pdf` | Image-only scan, **246 pages**; controlling evidence |
| `temp/md/literature/Hal-Karaan - Maxamed Ibraahim Warsama (Hadrawi).md` | Raw OCR import; same damage patterns |

The resource retained three H2 sections but the *Maansooyinka* body (from line
96 onward) showed severe **two-column interleaving**, pipe/table debris, Latin
OCR fragments, and broken verse lines. Prose sections were partly readable but
still carried column splits and wrap breaks.

## Findings

| ID | Severity | Finding | Resolution |
| --- | --- | --- | --- |
| SUU14-R001 | low | H1 plus author-supplied front matter and poem parts | Retained; poem titles as `###` from PDF |
| SUU14-R002 | fatal | Column-merge corruption across *Maansooyinka* | **Resolved** via 8-batch PDF transcription |
| SUU14-R003 | high | Mechanical tokens: `00` for `oo`, pipe debris, Latin OCR | Superseded by PDF pass |
| SUU14-R004 | medium | Prose sections line-wrap and column splits | **Resolved** via PDF transcription |
| SUU14-R005 | low | Missing per-poem headings | **Restored** from PDF (`###` titles) |

## Approval state

- **Audit approval:** approved by the maintainer on 2026-08-19 with the instruction, "go ahead."
- **Finding IDs:** SUU14-R001 through SUU14-R005
- **Cleanup:** phase-2 PDF transcription (2026-08-19; 8 batches, pages 7–246)
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Cleanup result

Full source-faithful transcription from PDF page images (body pages 7–246).
Verse stanzas preserved; poem titles and part dividers restored; front matter
through *Tilmaan* (entries 1–14) and closing *Xusuus* glossary included. The
independently reviewed transcription batches were merged into one continuous file.

- **Post-cleanup:** 16,334 lines
- **Resource SHA-256 after cleanup:**
  `f1ff0748a81c360e5e81483ada3860c25005e953f1f47f8d3de1c0a06f4bd293`
- **PDF mapping:** 8 parts → `/tmp/halkaraan-part{1..8}.md`; PNGs at `/tmp/halkaraan-ocr/`
- **Status:** complete; cleanup approved 2026-08-23
