# Audit record — Suugaanta Soomaaliyeed (Hadraawi)

- **Resource path:** `resources/suugaan/22-suugaanta-soomaaliyeed.md`
- **Collection / family:** suugaan / tixraac (1992 Norwegian school anthology; 11 Hadraawi poems + Somali biography)
- **Priority:** P2
- **Method:** whole-file literary audit; comparison with local image PDF and raw OCR
- **Audit status:** complete
- **Audit date:** 2026-08-19
- **File size at audit start:** 1,227 lines
- **Resource SHA-256 at audit start:**
  `c5c39e3d783d82ceeda618ae829d50c7401a2b7f4ed219f24e75912da15ebefe`
- **Resource-text changes during audit:** none

## Target output model

The cleaned file must remain a source-faithful Somali body of *Suugaanta
Soomaaliyeed* (Den Norske Somaliakomiteen / Sekretariatet for Grunnskolen,
1992):

1. **Taariikh nololeedka** — Somali author biography (printed pp. 6–7)
2. **Eleven poems** — each as `##` with one line per printed verse and closing
   place/date signature where printed

| Poem | Dateline in resource |
| --- | --- |
| Baaq | (Hargeysa, 1972) |
| Hooyo | (Lafoole, Afgooye, 8/02/1973) |
| Hablaha Geeska Afrika | (Qansax-dheere, 1977) |
| Arraxmaan | (Muqdisho, 1972) |
| Beled-Weyn | (Muqdisho, 1970) |
| Daryeel | (Qansax-dheere, 1977) |
| Hud-Hud | (Berbera, 1970) |
| Haatuf | (Burco, 1971) |
| Daalo | (Muqdisho, 1973) |
| Xamareey Ma Nabadbaa? | (Qansax-dheer, 1975) |
| Tog-Dheer | (Muqdisho, 1972) |

Cleanup may repair scan-proven OCR, restore stanza breaks, and omit non-Somali
front matter (Norwegian *Forord*, *Presentasjon*, ISBN/copyright pages,
illustration-only spreads). It must not abridge poems, modernize diction, or
reorder the printed sequence. Attribution stays in
[`00-sources.md`](../../../../resources/suugaan/00-sources.md) only.

## Controlling source

| Asset | Role |
| --- | --- |
| `temp/soomaali/literature/Suugaanta Soomaaliyeed.pdf` | Image scan, **48 pages**; controlling evidence |
| `temp/md/literature/Suugaanta Soomaaliyeed.md` | Raw OCR (1,625 lines); navigation aid only |

Both OCR paths are byte-identical (SHA-256
`124933523fbe5b5a9a4defed0748fff1628fc30c20ddb00c7dd138372bad4b6e`).

The resource is a prior curation (1,625 → 1,227 lines): page markers, Norwegian
text, and most scan debris were removed; poem titles became `##` headings; verse
line breaks are largely preserved. Unlike the *Hal-Karaan* import, there is no
fatal two-column interleaving across whole poems, but word-level OCR damage and
one biography truncation remain.

## Findings

| ID | Scope | Severity | Finding | Proposed action |
| --- | --- | --- | --- | --- |
| SUU22-R001 | whole file | low | H1, `## Taariikh nololeedka`, eleven poem `##` sections, datelines match PDF *Innholdsliste* | Retain structure |
| SUU22-R002 | bio | medium | Closing sentence after S.N.M. membership (PDF p. 7) is absent: *wuxuu halkaas ka galay halgan siyaasadeed iyo mid sugaaneed oo weji waddani ah…* | Restore from PDF |
| SUU22-R003 | metadata | low | Norwegian foreword (pp. 5–6), Norwegian author presentation (pp. 8–9), ISBN/copyright (p. 3) correctly omitted | Keep out of body |
| SUU22-R004 | bio + verse | medium | Residual OCR tokens, e.g. `shqeeyey` (bio), `hooggtyo`, `sityo`, `dalxtis tyo`, inconsistent `xasuustaa` / `xusuustaa`, `weeyee`/`weeyoo` clusters | Phase-2 PDF transcription or PDF-verified token fixes |
| SUU22-R005 | poems | medium | Scattered word-level damage across all eleven poems; stanza layout mostly intact (not column-fatal like SUU14-R002) | Phase-2 PDF pass recommended (48 pp.; est. 2–3 batches) |
| SUU22-R006 | datelines | low | Place-name spelling varies (`Qansax-Dheere`, `Qansax-dheere`, `Qansax-dheer`) | Normalize only where PDF proves one form |
| SUU22-R007 | furniture | low | Raw OCR retains illustration debris on poem openings (e.g. p. 13); resource already strips most | Omit scan/illustration lines in cleanup |

## Approval state

- **Audit approval:** approved by the maintainer on 2026-08-19 with the instruction, "go ahead."
- **Finding IDs:** SUU22-R001 through SUU22-R007
- **Cleanup:** PDF-guided pass (2026-08-19; pages 6–7 bio, 10–48 poems via `/tmp/suugaan22-ocr/`)
- **Cleanup approval:** pending
- **Complete:** no

## Cleanup result

Phase-2 PDF-guided transcription and correction from the 48-page scan restored
the full Somali biography (SUU22-R002, Hay'adaha Cadaadinta paragraph, closing
SNM sentence), corrected scan-proven OCR
across all eleven poems (SUU22-R004, SUU22-R005), and aligned datelines to printed
*(Xamar, …)* where the PDF differs from prior *(Muqdisho, …)* readings.

- **Post-cleanup:** 1,226 lines
- **Resource SHA-256 after cleanup:**
  `ea6c43d0e880fdcf5d58ce0ba8a10cff0831cad863546aa061413102fa0a1636`
- **PNG mapping:** `/tmp/suugaan22-ocr/p-{01..48}.png` (200 DPI)
- **Status:** awaiting cleanup approval
- **Spot-check (non-blocking):** Norwegian foreword and author presentation (pp. 5–9) omitted per Somali-only policy; some `weeyee`/`weyee` spellings remain where the scan uses both forms in different poems

## Cleanup notes (for approved pass)

Estimated effort: **lower than files 14 or 20** — single-volume 48-page school
heft with continuous verse layout. Suggested approach: page-image transcription
from PDF pp. 6–7 (bio) and pp. 10–47 (poems); skip Norwegian-only pages per
Somali-only policy. No merge script exists yet.
