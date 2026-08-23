# Audit record — Maahmaahyada Soomaaliyeed

- **Resource path:** `resources/suugaan/01-maahmaahyada.md`
- **Collection / family:** suugaan / maahmaah (Kapchits proverb dictionary)
- **Priority:** P2
- **Method:** whole-file audit; comparison with local Kapchits PDF OCR at
  `temp/soomaali/terminology/Qaamuuska Maahmaahyada Soomaalyeed_Kapchits.pdf`
  and byte-identical OCR markdown in `temp/md/terminology/`
- **Audit status:** approved; phase-1 cleanup applied and awaiting maintainer cleanup review
- **Audit date:** 2026-08-19
- **File size at audit start:** 5,634 lines
- **Resource SHA-256 at audit start:**
  `8668abc073a27fa0ebd701e152c953e1f1095c49c58300e05e6f54b7be261c2d`
- **Resource-text changes during audit:** none

## Target output model

The cleaned file must preserve Georgi Kapchits's *Qaamuuska Maahmaahyada
Soomaalyeed* (1998) Somali body in three parts already merged in this resource:

1. **Qaybta 1aad** — letter sections `## B` … `## Uu` (one-phrase proverbs)
2. **Qaybta 2aad** — `## Maahmaahyo tix ka badan` (multi-line and dialogue proverbs)
3. **Qaybta 3aad** — `## Tiroley` (number clichés) with `### Saddexley`, `### Labaley`,
   `### Shanley`, `### Afarley`

Kapchits informant citation codes (`.P`, `.T`, `.Y`, `.V`, `.N`, cross-refs, page
keys) should be **stripped** per [`00-sources.md`](../../../../resources/suugaan/00-sources.md).
Foreign-language introduction and appendices stay out of the literary body. Cleanup
may repair OCR, split two-column scan collisions, join broken line wraps, and omit
unrecoverable fragments. It must not modernize proverb wording or invent missing text.

## Controlling source

| Asset | Role |
| --- | --- |
| `temp/soomaali/terminology/Qaamuuska Maahmaahyada Soomaalyeed_Kapchits.pdf` | Image PDF; controlling layout for two-column Somali glossary |
| `temp/md/terminology/Qaamuuska Maahmaahyada Soomaalyeed_Kapchits.md` | Raw OCR (4,564 lines); same damage patterns as the imported resource |

The resource is a prior curation pass over the Somali sections, but both the
resource and raw OCR retain **two-column page collisions**, partial citation
stripping, and line-wrap breaks. Full restoration of Qaybta 2aad and Tiroley
requires page-by-page work against the PDF.

## Structural inventory at audit start

| Section | Lines (approx.) | Condition |
| --- | ---: | --- |
| H1 + letter sections B–Uu | ~5,350 | Mixed: many readable one-line proverbs; citation debris; column pairs; wrap splits |
| `Maahmaahyo tix ka badan` | ~60 | Heavy column merge; dialogue lines broken; citation debris |
| `Tiroley` + number subsections | ~220 | Heavy column merge; partial entries; citation debris |

## Findings

| ID | Scope | Severity | Finding | Proposed action |
| --- | --- | --- | --- | --- |
| SUU01-R001 | whole file | low | H1, 30 letter sections, Qaybta 2/3 headings, and seven `###` tiroley subsections present in source order | Retain structure |
| SUU01-R002 | Qaybta 1aad | medium | Kapchits citation suffixes (`.P`, `.T`, `.Y`, `.V`, `.N`, `101 eeg`, cross-refs) on many lines | Strip mechanically where pattern-safe |
| SUU01-R003 | Qaybta 1aad | high | Two-column scan collisions: pipe-separated pairs and mid-line `.T`/`.Y` joins | Split into separate proverb lines when both sides are Somali |
| SUU01-R004 | Qaybta 1aad | medium | Line-wrap splits (e.g. `Been nin.` / `bixisaa`) | Join continuations before filtering |
| SUU01-R005 | Qaybta 1aad | medium | OCR debris lines (Latin junk, `Tecate`, `TISABAE`, `Bare eee…`) | Omit unreadable fragments |
| SUU01-R006 | Qaybta 1aad | low | Occasional mechanical letter damage (`waha`/`wah`, `dyul`, `00`) | Apply conservative substitutions |
| SUU01-R007 | Qaybta 2aad | fatal | Most dialogue/multi-line entries still merged or truncated; needs PDF page pass | Phase-2 source-guided repair; do not rewrite from expectation |
| SUU01-R008 | Qaybta 3aad | fatal | Tiroley entries heavily column-merged; opening lines fragmentary | Phase-2 PDF pass; preserve number cliché forms when verified |
| SUU01-R009 | metadata | low | No per-file provenance block (attribution in `00-sources.md` only) | No collection note in body |

## Approval state

- **Audit approval:** approved by the maintainer on 2026-08-19 with the instruction, "go ahead."
- **Finding IDs:** SUU01-R001 through SUU01-R009
- **Cleanup:** phase-1 applied on 2026-08-19
- **Cleanup approval:** pending
- **Complete:** no

## Cleanup result (phase 1)

The phase-1 mechanical cleanup applied SUU01-R002 through SUU01-R006 across the
full file. Qaybta 1aad letter sections are substantially
improved; Qaybta 2aad and Tiroley remain **partially corrupted** pending phase-2
PDF-guided work (SUU01-R007, SUU01-R008).

Applied changes include:

- joining line-wrap continuations (e.g. `Been nin bixisaa`);
- stripping Kapchits citation suffixes and most cross-ref debris from one-line proverbs;
- splitting pipe-separated and mid-line two-column collisions where both sides read as Somali;
- omitting isolated OCR garbage lines;
- conservative mechanical OCR substitutions (`waha`→`waxaa`, `dyul`→`duul`, etc.);
- deduplicating consecutive identical lines after repair.

Not attempted in phase 1:

- full reconstruction of `Maahmaahyo tix ka badan` dialogue blocks;
- full reconstruction of `Tiroley` number clichés;
- silent modernization of dialect or historical Kapchits forms.

- **Post-cleanup:** 5,552 lines; ~5,500 proverb/cliché entry lines
- **Resource SHA-256 after cleanup:**
  `1b317ad0283fcdcbed301a0f6d7845be10e894a696f1be1d7c33a333bd8d3e0a`
- **Wordlist parity:** n/a
- **Status:** phase-1 applied; awaiting maintainer cleanup review

## Recommended phase 2

Inspect the Kapchits PDF Somali pages for `Maahmaahyo tix ka badan` and
`Tiroley` sequentially; restore readable Markdown paragraphs and dialogue without
inventing text. Record substantive repairs in the provenance log.
