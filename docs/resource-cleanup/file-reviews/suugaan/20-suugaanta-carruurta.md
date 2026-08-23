# Audit record — Suugaanta Carruurta

- **Resource path:** `resources/suugaan/20-suugaanta-carruurta.md`
- **Collection / family:** suugaan / carruur (1976 anthology: Idaajaa folktales + Dahabo children's stories + Russian tale translation)
- **Priority:** P1
- **Method:** whole-file literary audit; comparison with local image PDF and raw OCR
- **Audit status:** complete
- **Audit date:** 2026-08-19
- **File size at audit start:** 1,696 lines
- **Resource SHA-256 at audit start:**
  `d38535c67ecf798f0c2893096044be17126792eab9e9144a62a1e946314ef136`
- **Resource-text changes during audit:** none

## Target output model

The cleaned file must remain a source-faithful body of *Suugaanta Carruurta*
(Wasaaradda Hiddaha iyo Tacliinta Sare, 1976):

1. **Qaybta I** — *Sheekooyin hidde* (32 folktales narrated by Hooyo Xaawo; Idaajaa)
2. **Qaybta II** — *Sheeko carruureed Soomaaliyeed* (six stories; Dahabo Faarax Xasan)
3. **Qaybta III** — *Sheeko xariir Ruush ah* (Russian tale translation; Dahabo Faarax Xasan)

Each story keeps its `###` heading and `---` separators. Frame narration (*Annagoo …
hooyo Xaawo sheekada … iigu sheekaysay*) in Part I must be preserved where present.
Cleanup may repair OCR, restore paragraph and dialogue layout, and omit page
furniture. It must not modernize diction, reorder stories, or invent missing text.
Attribution stays in [`00-sources.md`](../../../../resources/suugaan/00-sources.md) only.

## Controlling source

| Asset | Role |
| --- | --- |
| `temp/soomaali/literature/Suugaanta Carruurta (Dahabo Faarax Xasan _ Idaajaa).pdf` | Image scan, **249 pages**; controlling evidence |
| `temp/md/literature/Suugaanta Carruurta (Dahabo Faarax Xasan _ Idaajaa).md` | Raw OCR (11,171 lines); page-per-line garbage, unusable without PDF |

The resource is a **prior partial curation** over the raw OCR import (11,171 → 1,696
lines). Early Part I folktales (roughly stories 1–15) read cleanly; the remainder
of Part I, all of Part II, and Part III retain severe scan/OCR damage. Git history
shows only import plus selective condensation, not a full PDF pass.

## Structural inventory at audit start

| Section | Lines (approx.) | Stories | Condition |
| --- | ---: | ---: | --- |
| H1 + *Qaybaha buugga* metadata | 1–7 | — | Metadata block in body (should not remain) |
| Qaybta I + *Tusmada* + folktales | 9–763 | 32 | **Mixed:** stories 1–15 largely readable; 16–32 heavily corrupted |
| Qaybta II + *Tusmada* + stories | 765–1509 | 6 | **Heavy OCR damage** throughout |
| Qaybta III (*Shimbir Garab la'a*) | 1511–1696 | 1 | **Duplicate/truncated** vs Part II homonym; closing garbage |

### Per-story damage heuristic (OCR-debris line count)

| Part | Clean / light (≤10% debris lines) | Mixed (11–25%) | Heavy (>25%) |
| --- | --- | --- | --- |
| I (32 stories) | 1–15, 17, 21, 25–26 | 16, 18–20, 22–24, 27–29, 32 | 30–31 |
| II (6 stories) | — | Catir, Abeeso, Takara | Shimbir, Walaalo Istecel, Qayb Libaax |
| III | — | — | entire section |

## Findings

| ID | Scope | Severity | Finding | Proposed action |
| --- | --- | --- | --- | --- |
| SUU20-R001 | whole file | low | H1, three part headings, two *Tusmada* tables, 39 story `###` headings in source order | Retain structure after cleanup |
| SUU20-R002 | metadata | low | Lines 3–7 *Qaybaha buugga* author/part block duplicated from catalogue | Remove from body; attribution in `00-sources.md` only |
| SUU20-R003 | Part I | medium | Stories 1–15 (*Foolxun iyo Qurxoon* … *Nimaan kibir jebin*) largely source-readable with occasional token damage | Retain text; spot-check against PDF; mechanical fixes only where pattern-safe |
| SUU20-R004 | Part I | high | Stories 16–32 from *Raage Ugaas* through *Wiilweel* show column-merge, Latin debris, `00`/`@` tokens, broken words | Phase-2 PDF transcription per story or page batch |
| SUU20-R005 | Part II | fatal | All six Dahabo stories (*Catir Caana ku Nuuge* … *Takara Faw Faw*) unreadable in long stretches; pagination markers (`= 34 =`, `= 62 =`) | Phase-2 PDF transcription (largest block after Part I tail) |
| SUU20-R006 | Part II | medium | *Abeeso iyo Sagaaro* retains stage-direction markup (`Dabagaalle:"…"`) mixed with OCR debris | Restore dialogue layout from PDF; do not flatten to plain prose |
| SUU20-R007 | Part II vs III | high | *Shimbir Garab la'a* in Part II (lines 1012–1234) and Part III (1513–1696) are ~89% text-similar; Part III ends truncated | PDF pass must confirm whether Part III is a distinct Russian translation or a duplicate export; deduplicate or restore missing Part III only from PDF |
| SUU20-R008 | Part III | fatal | Closing lines (≈1680–1695) are garbage fragments; story incomplete | Replace from PDF through proper ending |
| SUU20-R009 | raw OCR | fatal | 11,171-line OCR markdown is page-image garbage; cannot guide repair | Ignore for content; use PDF images only |
| SUU20-R010 | whole file | medium | Mechanical tokens throughout damaged zones: `00`→`oo`, `@`, `= N =` page keys, pipe/column fragments | Apply during PDF pass; omit unrecoverable lines |
| SUU20-R011 | Part I frame | low | Hooyo Xaawo frame openings present on early folktales | Preserve in cleaned Part I |

## Recommended cleanup approach

Given **249 pages** and mixed condition:

1. **Retain** verified-clean Part I stories (SUU20-R003) after PDF spot-check.
2. **Phase-2 PDF transcription** for SUU20-R004, SUU20-R005, SUU20-R007, SUU20-R008
   in batches (suggested: Part I tail, Part II stories 1–3, Part II stories 4–6 +
   Part III resolution).
3. **Strip** SUU20-R002 metadata block from body.
4. Merge the independently reviewed transcription batches into one continuous file.

Estimated effort: comparable to files 17–19 (multi-batch vision transcription).

## Approval state

- **Audit approval:** approved by the maintainer on 2026-08-19 with the instruction, "go ahead."
- **Finding IDs:** SUU20-R001 through SUU20-R011
- **Cleanup:** phase-2 PDF transcription (2026-08-19; 8 batches, pages 1–249)
- **Cleanup approval:** pending
- **Complete:** no

## Cleanup result

Full source-faithful transcription from PDF page images (249 pages). Three-part
anthology structure restored:

| Part | Content | Stories |
| --- | --- | ---: |
| Qaybta I | *Sheekooyin hidde* (Idaajaa / Hooyo Xaawo frame) | 32 folktales |
| Qaybta II | *Sheeko carruureed Soomaaliyeed* (Dahabo) | 6 stories |
| Qaybta III | *Sheeko xariir Ruush ah* (Dahabo translation) | 6 Russian tales |

SUU20-R002: *Qaybaha buugga* metadata and Part III translator imprint stripped from body.
SUU20-R007 resolved: Part III tales (*Shimbir-Kaah*, *Faarax iyo Suuban*, *Ceebla
Caqlibadan*, *Kariir*, *Camalow*, *Faaduma-Xay*) are distinct from Part II *Shimbir
Garabl'i*.

The independently reviewed transcription batches were merged into one continuous
file after page-image comparison and boundary review.

- **Post-cleanup:** 2,305 lines
- **Resource SHA-256 after cleanup:**
  `b5cce85007add63864d32758ae456ac21ee0f3f46f50b43d87ced654766e3dc0`
- **Status:** awaiting cleanup approval
