# Audit record — Bisaddii Bubaysta

- **Resource path:** `resources/suugaan/12-bisaddii-bubaysta.md`
- **Collection / family:** suugaan / sheeko (Henty translation)
- **Priority:** P1
- **Method:** whole-file literary audit; comparison with local image PDF and raw OCR
- **Audit status:** complete
- **Audit date:** 2026-08-19
- **File size at audit start:** 3,699 lines
- **Resource SHA-256 at audit start:**
  `7356c0865b4799310b06ae7c29490b80fc0493828eadaa5c5291d4d26f58a6da`
- **Resource-text changes during audit:** none

## Target output model

The cleaned file must remain a complete, source-faithful Somali translation of G. A.
Henty's *The Cat of Bubastes* (Xasan Aw Daahir Qaalib). It is one continuous
historical novel set in ancient Egypt and Rebu: Amuba, Jethro, the Rebu king,
Egyptian invasion, and the resolution with Miisa and Ruut. Cleanup may repair
scan-proven OCR, restore paragraph and dialogue layout, and omit page furniture.
It must not abridge, rewrite, or modernize the 1970s school-edition Somali.

## Controlling source

| Asset | Role |
| --- | --- |
| `temp/soomaali/literature/Bisaddii Bubaysta.pdf` | Image-only scan, **123 pages**; controlling evidence |
| `temp/md/literature/Bisaddii Bubaysta.md` | Raw OCR (4,844 lines); same damage patterns |

The resource is a stripped subset of the raw OCR import. Both retain severe
**column-merge collisions**, line-wrap breaks mid-word, Latin OCR debris, and
missing paragraph structure. Git history contains only the initial import.

## Findings

| ID | Severity | Finding | Proposed action |
| --- | --- | --- | --- |
| SUU12-R001 | low | Single H1; no author-supplied chapter divisions | Retain one H1; do not invent chapters |
| SUU12-R002 | fatal | Widespread two-column and line-wrap OCR damage from opening through closing (`Sidaasaa leebkij bisaddii Bubaysta…`) | Phase-2: page-by-page PDF reconstruction |
| SUU12-R003 | high | Mechanical tokens: `00` for `oo` (~176), `Maagaalo`, `wuxu`/`waxa` fragments, `@`/`™` debris | Phase-1 mechanical pass |
| SUU12-R004 | medium | Isolated garbage lines (`See vases…`, `ome: =sgies`) | Omit in phase 1 |
| SUU12-R005 | low | Ministry/title-page furniture correctly absent from body | Keep out of literary text |

## Approval state

- **Audit approval:** approved by the maintainer on 2026-08-19 with the instruction, "go ahead."
- **Finding IDs:** SUU12-R001 through SUU12-R005
- **Cleanup:** phase-1 (2026-08-19) + **phase-2 PDF transcription** (2026-08-19)
- **Cleanup approval:** pending
- **Complete:** no

## Cleanup result (phase 1)

The phase-1 mechanical cleanup applied SUU12-R003 and SUU12-R004 only.
Narrative remained largely unreadable pending SUU12-R002.

- **After phase-1:** 3,693 lines
- **Resource SHA-256 after phase-1:**
  `1badbd6286a2b2015906db6c938061a7fc16645138cf2b76739f996a4e483d31`

## Cleanup result (phase 2)

Full source-faithful transcription from PDF page images p-03–p-123 (printed
pages 1–121). Single H1; Roman-numeral section markers omitted; continuous
novel prose through closing line about *bisaddii Bubaysta*. Names preserved
(Amuba, Jitro/Jethro, Rebus, Kasbian, Miisa, Ruut, Jebron, Totmes, Ameres).

- **Post-cleanup:** 734 lines
- **Resource SHA-256 after cleanup:**
  `d98daf413141589265b8f4afbe2b59360a52005eed2af14b67061c56e4056906`
- **PDF mapping:** body = images `p-003.png` → `p-123.png` (covers p-01/p-02 omitted)
- **Status:** awaiting cleanup approval
