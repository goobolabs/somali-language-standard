# Audit record — Qaamuuska eray-bixinta ee Bayoolaji

- **Resource path:** `resources/erey-bixin/01-bayoolaji.md`
- **Collection / family:** erey-bixin / historical bilingual science glossary
- **Priority:** P2
- **Method:** repository-only, line-by-line glossary audit
- **Audit status:** approved; historical-glossary cleanup applied and awaiting
  maintainer cleanup review
- **Audit started:** 2026-08-19
- **File size at audit start:** 1,246 lines; 4,905 words; 31,103 bytes
- **Resource SHA-256 at audit start:**
  `4ae6375579b973531b002af4db17856bdf58738613ea6292a1d4ed88383c863f`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact historical English–Somali biology glossary, not an OCR
transcript, not a dictionary rewrite, and not an SLS-native grammar chapter.
It has one H1, twenty-three `##` letter sections, and 1,198 `English — Somali`
pairs. There is no cover matter, TOC, exercises, or `## OCR Page` marker.

This collection is a third cleanup pattern: **historical bilingual glossaries**.
Keep printed term pairs. Do not modernize 1987 scientific coinages. Keep the
existing letter-section list format from `resources/erey-bixin/README.md`. Fix
uniquely supported OCR. Omit unreadable Somali sides rather than reconstructing
them. Do not invent missing letter sections, and do not copy terms from
`04-kimistari.md` or later files.

Cleanup should add a blank line before each letter heading after `## A`, repair
the uniquely supported OCR listed below, omit only the unreadable `Gills` row,
and add a one-line collection note with a link to `00-sources.md` (no
author/year in the content file). It must not rewrite `Organada`, `Bayoloji`,
`Akson`, `ATP`/`DNA`, or `Midhe`.

## Source and repository evidence

`resources/erey-bixin/00-sources.md` maps this file to *Qaamuuska eray-bixinta
ee Bayoolaji* (Xarunta Horumarinta Manaahijta, 1987). The collection README
already states the entry format (`Antibody — Lid-jidh gale`), forbids per-file
provenance blocks, and keeps attribution in `00-sources.md`.

File `09` coverage notes omit unreadable OCR rather than reconstructing it.
That omit rule is the model for unreadable Somali sides here. It is not a
licence to import chemistry `K`/`Q`/`Y` heads.

Repository comparison also included:

- same-file `Root *` rows (`xidid`) versus `Kidid` / `Kidido`;
- same-file `Qanjidh` rows versus `Ganjidh`;
- same-file `Laf carjaweed` versus `Carjow`;
- cleaned `resources/dhawaaq/02-xubnaha-hadalka.md` `qalaanqulshaha`;
- dictionary `xidid`, `qanjidh`, `carjaw`, `qulaanqulshe`,
  `kimistariga-noolaha`, `ilig`, `qori`;
- `resources/erey-bixin/04-kimistari.md` letter map only, as a negative check
  (do not import).

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-2 | reviewed | EB01-R001 |
| 3-155 (`## A`) | reviewed | EB01-R003, EB01-R005, EB01-R007 |
| 156-380 (`## B`–`## D`) | reviewed | EB01-R005, EB01-R007 |
| 381-541 (`## E`–`## F`) | reviewed | EB01-R003, EB01-R006, EB01-R007 |
| 542-709 (`## G`–`## L`) | reviewed | EB01-R003, EB01-R004, EB01-R005, EB01-R006 |
| 710-973 (`## M`–`## R`) | reviewed | EB01-R003 |
| 974-1246 (`## S`–`## Z`) | reviewed | EB01-R005, EB01-R006 |
| whole file | reviewed | EB01-R002, EB01-R008 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| EB01-R001 | 1-2 | Source H1 is the right title; the file has no collection framing and no link to the registry / low | Collection README keeps attribution in `00-sources.md` and forbids per-file provenance blocks; cleaned sarfe/phonology openings add a local registry link without restating author/year | Keep the printed H1. Add one Somali sentence that this is the biology glossary in this collection, with a local link to `00-sources.md`. Do not put author, year, or publisher in the content file. | `repository-supported`; `navigation-update`; `intentional-retained` |
| EB01-R002 | whole file | Compact glossary already: 1 H1, 23 letter sections, 1,198 unique English heads, internal A–Z sort, no missing em dashes, no section-letter mismatches. Almost every `##` after `A` lacks a preceding blank line. Headings `K`, `Q`, and `Y` are absent / medium | `04-kimistari.md` has `K`/`Q`/`Y`; this file has no Kidney/Karyotype/Yeast heads to place there | Insert a blank line before each letter heading from `## B` through `## Z`. Do not invent `K`/`Q`/`Y` sections. Do not copy chemistry or later-file terms. | `structural-only`; `intentional-retained` |
| EB01-R003 | 28, 30, 500, 679 | `Kidido` / `Kidid` conflict with the same file’s `Root *` rows, which already use `xidid` / high | Same-file lines 968–972; dictionary `xidid` / `xididdo` | Correct `Kidido` → `Xididdo` and `Kidid` → `Xidid` in those four rows. Leave other `xidid` uses as printed. | `repository-supported`; `form-correction` |
| EB01-R004 | 675–676 | `Quiaanquishe` and `Carjow` are OCR, not printed coinages. `Locomotion — Dhaqdhaqaaq quiaanquishe` (697) is a different problem / high | Same-file `Laf carjaweed`; dictionary `carjaw`, `qulaanqulshe`; cleaned phonology `qalaanqulshe` | On larynx rows only: `Carjow` → `Carjaw`. Replace `Quiaanquishe` with dictionary `qulaanqulshe` (not phonology `qalaanqulshe`, unless the maintainer prefers that form). Do not turn locomotion into a larynx gloss; leave line 697 unresolved. | `repository-supported`; `form-correction`; `unresolved` |
| EB01-R005 | 59, 68, 180, 565, 571, 1114, 1232 | Unique OCR on otherwise readable rows / high | Dictionary `kimistariga-noolaha`, `qanjidh`, `ilig`, `qori`; same-file `Qanjidh` cluster; English biology heads Amnion / Ammonification | `Ammion` → `Amnion`; `Amonification` → `Ammonification` (leave Somali `Ammoeniyeyn` unresolved); `Kimigtariga noolaha` → `Kimistariga noolaha`; `Ganjidh` → `Qanjidh` on the two gland rows; `Tooth — Ilk` → `Ilig`; `Gan gqori` → `Gan qori`. | `repository-supported`; `form-correction`; `unresolved` |
| EB01-R006 | 151, 523, 541, 563, 567, 660–661, 1113 | Several Somali sides are unreadable or only weakly guessable / medium | File `09` omit-unreadable rule; no unique same-file repair for these | Omit the whole `Gills — Redhsdhazie` row. Leave `Avoodda dib u dhsidda`, `Faraqgie` / `Faragie` / `Faraqle`, `Bhogox`, `Gilisarect`, `Snuq`, `Koliuun sabeed`, and `Xoqgado, gquman` unresolved. Do not invent replacements. | `unresolved`; `omit-unreadable-ocr` |
| EB01-R007 | 11, 41, 44, 74, 139, 184, 364, 457, 536 | Source scientific spellings and acronym identity rows / low | Title `Bayoolaji` versus entry `Bayoloji`; dictionary would modernize several of these; orthography cleanup removed a junk identity row of a different kind | Retain `Akson`, `ATP — ATP`, `DNA — DNA`, `Biology — Bayoloji`, `Organada`, `Midhe` (versus `Midho` on line 41), and mixed English in Somali equivalents. Do not modernize coinages. | `intentional-retained` |
| EB01-R008 | whole file | Already in the collection list format; remaining work is spacing, a registry link, unique OCR, and one omit / medium | 1,198 pairs; no duplicate English heads; README entry format | Keep one concept per line, English first. Apply only the approved repairs. Add no new terms. | `structural-only`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Qaamuuska eray-bixinta ee Bayoolaji**
and keep this sequence:

1. one-line collection purpose plus a link to `00-sources.md`;
2. `## A` through `## Z` as now, with a blank line before each heading, and
   with no invented `K`/`Q`/`Y`;
3. one `English — Somali` line per concept, in the current within-section
   sort.

No new term should be invented. Unreadable Somali is omitted or left
unresolved, not reconstructed. Source coinages stay as printed.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** EB01-R001 through EB01-R008
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Audit validation

- Resource SHA-256 after audit:
  `4ae6375579b973531b002af4db17856bdf58738613ea6292a1d4ed88383c863f`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, twenty-three H2 letter headings, 1,198 em-dash
  pairs, zero duplicate English heads, zero section-letter mismatches, missing
  headings `K`, `Q`, `Y`.

## Cleanup result and review

### Applied cleanup

- Added a one-line collection note linking `00-sources.md`.
- Inserted a blank line before each letter heading from `## B` through `## Z`.
- Corrected `Kidido`/`Kidid` to `Xididdo`/`Xidid` on the four root rows.
- On larynx rows only: `Carjow` → `Carjaw`; `Quiaanquishe` → `Qulaanqulshe`.
- Corrected `Ammion`, `Amonification`, `Kimigtariga`, `Ganjidh`, `Ilk`, and
  `gqori`.
- Omitted the unreadable `Gills — Redhsdhazie` row.

### Deliberately retained

- `Akson`, `ATP — ATP`, `DNA — DNA`, `Biology — Bayoloji`, `Organada`, and
  `Midhe`.
- Somali `Ammoeniyeyn`; locomotion `quiaanquishe`.
- Unreadable or weakly supported Somali sides: `Avoodda dib u dhsidda`,
  `Faraqgie` / `Faragie` / `Faraqle`, `Bhogox`, `Gilisarect`, `Snuq`,
  `Koliuun sabeed`, and `Xoqgado, gquman`.
- Missing `K`/`Q`/`Y` headings; no imported chemistry terms.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, twenty-three H2 letter headings, 1,197 em-dash pairs,
  zero duplicate English heads, blank line before every letter heading, no
  `K`/`Q`/`Y`.
- Pre-cleanup forms `Kidido`, `Kidid`, `Quiaanquishe`, `Carjow`, `Ammion`,
  `Amonification`, `Kimigtariga`, `Ganjidh`, `Ilk`, `gqori`, and
  `Redhsdhazie` are absent.
- Cleaned file size: 1,269 lines; 4,909 words; 31,191 bytes.
- Cleaned SHA-256:
  `f3e15784685de37aba3911a430f0e6b35d9012dfe984445671d76322ac341df5`.
