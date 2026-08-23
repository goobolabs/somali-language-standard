# Audit record — Suugaanta dhallaanka

- **Resource path:** `resources/suugaan/21-suugaanta-dhallaanka.md`
- **Collection / family:** suugaan / carruur (1987 children's anthology; Aamina X. Aadan)
- **Priority:** P1
- **Method:** whole-file literary audit; comparison with local image PDF and raw OCR
- **Audit status:** complete
- **Audit date:** 2026-08-19
- **File size at audit start:** 993 lines
- **Resource SHA-256 at audit start:**
  `199539e5f9550ce20495bb7725975fdbefa46464ba001347fe581948bf114034`
- **Resource-text changes during audit:** none (audit); phase-2 replacement (cleanup)

## Target output model

The cleaned file must remain a source-faithful body of *Suugaanta dhallaanka*
(Waxbarashada iyo Barbaarinta, 1987):

1. **Tusmada buugga** — story and song tables
2. **Hordhac** — author introduction (acknowledgements retained in body as printed)
3. **14 sheekooyin** — each as `##` heading
4. **Heeso** — section with `###` per song/ rhyme
5. **Googaalaysi** — numbered riddles
6. **Diiqda** — closing colophon if printed

Cleanup may repair OCR and restore verse layout; it must not modernize diction.
Attribution stays in [`00-sources.md`](../../../../resources/suugaan/00-sources.md) only.

## Controlling source

| Asset | Role |
| --- | --- |
| `temp/soomaali/literature/Suugaanta dhallaanka.pdf` | Image scan, **94 pages**; controlling evidence |
| `temp/md/literature/Suugaanta dhallaanka.md` | Raw OCR (2,500 lines); same damage patterns |

The resource is a prior partial curation (2,500 → 993 lines). *Hordhac* and parts of
*Googaalaysi* read cleanly; folktales and song lyrics retain severe column-merge OCR,
pagination debris (`— 23 —`), and Latin/layout garbage.

## Findings

| ID | Scope | Severity | Finding | Proposed action |
| --- | --- | --- | --- | --- |
| SUU21-R001 | whole file | low | H1, tusmada, hordhac, 14 story `##`, heeso `###`, googaalaysi, diiqda | Retain structure from PDF |
| SUU21-R002 | stories | fatal | Folktales (*Dhaaratay* through *Isha Cali ka lulata*) heavily corrupted | Phase-2 PDF transcription |
| SUU21-R003 | heeso | high | Song lyrics mixed with OCR debris and layout tokens | Phase-2 PDF pass; preserve stanza breaks |
| SUU21-R004 | googaalaysi | medium | Riddle list partially readable; some lines garbled | Phase-2 PDF pass |
| SUU21-R005 | metadata | low | Author/acknowledgement block in *Hordhac* is source body | Retain as printed |

## Approval state

- **Audit approval:** approved by the maintainer on 2026-08-19 with the instruction, "go ahead."
- **Finding IDs:** SUU21-R001 through SUU21-R005
- **Cleanup:** phase-2 PDF transcription (2026-08-19; 3 batches, pages 1–94)
- **Cleanup approval:** pending
- **Complete:** no

## Cleanup result

Full source-faithful transcription from PDF page images (94 pages). Structure:
*Tusmada*, Gaarriye *ARAR* + author *HORDHACA*, 15 sheekooyin (including *Yoonis Tuug* as
its own story per PDF), 28 heeso with stanza layout, *Googaalaysi* riddles with *Furaha
Googaalaysiga*, and closing *Sheeko dhamaatay*.

The independently reviewed transcription batches were merged into one continuous file.

- **Post-cleanup:** 1,200 lines
- **Resource SHA-256 after cleanup:**
  `cf38455facbdfa41495d02c0a78a0adb97da44ee02c71f6a8e937d790a15425d`
- **Status:** awaiting cleanup approval
- **Spot-check (non-blocking):** printed book page 87 missing from scan — *Gorayo* verse after intro not recoverable; old *Diiqda* colophon replaced by PDF closing *Sheeko dhamaatay*
