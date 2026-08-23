# Audit record — Qaamuuska eray-bixinta ee Xisaab

- **Resource path:** `resources/erey-bixin/05-xisaab.md`
- **Collection / family:** erey-bixin / historical bilingual science glossary
- **Priority:** P2
- **Method:** repository-only, line-by-line glossary audit
- **Audit status:** approved; historical-glossary cleanup applied and awaiting
  maintainer cleanup review
- **Audit started:** 2026-08-19
- **File size at audit start:** 1,381 lines; 6,723 words; 42,125 bytes
- **Resource SHA-256 at audit start:**
  `bd5291c38faef015473a7bb3e81c08262c094aa93c009922191824e6a16aa39a`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact historical English–Somali mathematics glossary plus two
appended Mansuur blocks, not an OCR transcript and not a dictionary rewrite.
It has one H1, twenty-five letter sections (A–I, K–Z), `## Ururinta 2aad
(Cilmiga)`, `## Sumadaha xisaabta`, and 1,322 `English — Somali` pairs. Almost
every letter heading after `## A` lacks a preceding blank line. There is no
cover matter, TOC, exercises, or `## OCR Page` marker.

This collection is a **historical bilingual glossary**. Keep printed term
pairs. Do not modernize 1987 or 2014 coinages. Keep the letter-section list
format and the two appended blocks named in `resources/erey-bixin/README.md`.
Fix uniquely supported OCR. Omit mashed or unreadable rows rather than
splitting them into new heads. Do not invent a `J` section, and do not copy
terms from cleaned `01`–`04`.

Cleanup should add a one-line collection note linking `00-sources.md` (no
author/year), insert a blank line before each letter heading after `## A`,
repair the uniquely supported OCR listed below, omit the mashed and
OCR-duplicate rows in EB05-R005, and apply the symbol-table / Ururinta
punctuation repairs in EB05-R006.

## Source and repository evidence

`resources/erey-bixin/00-sources.md` maps the A–Z glossary to *Qaamuuska
eray-bixinta ee Xisaab* (1987) and the appended science terms plus symbol table
to Mansuur *Diiwaanka* Ururinta 2aad (2014). Duplicate English heads are not
to be repeated.

The collection README already uses `Angle, right — Xagal qumman` as the entry
format example, taken from this file.

Repository comparison also included:

- same-file `Xagal`, `aljebraad`, `isugeynta`, `iqtiyaari`, `Qaybin`,
  `isle'eg`, `Triangle, right angled — Saddexagal qumman`, `Non-trivial
  solution` under `N`, `Anticlockwise`, `Angle, vertex of an`;
- dictionary `isugeyn`, `dhammaan`, `mayl-badeed`;
- cleaned `01`–`04` only as do-not-import checks.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-2 | reviewed | EB05-R001 |
| 3-125 (`## A`) | reviewed | EB05-R003, EB05-R004, EB05-R005 |
| 126-545 (`## B`–`## H`) | reviewed | EB05-R003, EB05-R005 |
| 546-1054 (`## I`–`## Z`) | reviewed | EB05-R002, EB05-R003, EB05-R005 |
| 1056-1344 (`## Ururinta 2aad`) | reviewed | EB05-R006, EB05-R007 |
| 1346-1381 (`## Sumadaha xisaabta`) | reviewed | EB05-R006 |
| whole file | reviewed | EB05-R002, EB05-R008 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| EB05-R001 | 1-2 | Source H1 is the right title; no collection framing or registry link / low | Cleaned `01`–`04`; attribution lives in `00-sources.md` | Keep the printed H1. Add one Somali sentence that this is the mathematics glossary in this collection, with a local link to `00-sources.md`. Do not put author, year, or Mansuur in the content file. | `repository-supported`; `navigation-update`; `intentional-retained` |
| EB05-R002 | whole file | Compact three-block glossary already: A–Z (no `J`; `I` has only Identity and Improper fraction), then Ururinta 2aad, then the symbol table. Letter headings after `A` lack a blank line. One duplicate English head / medium | README maps Ururinta + `## Sumadaha xisaabta`; chemistry restored `I` only because 107 I-heads were present; this file has no J-heads | Insert a blank line before each letter heading from `## B` through `## Z`. Keep both appended blocks. Do not invent `J` or extra I-terms. Do not re-sort Ururinta. | `structural-only`; `intentional-retained` |
| EB05-R003 | 7;30;35;44;65;96;120-122;134;137 | English-head OCR on otherwise readable 1987 rows / high | Same-file `Alternate angles`, `Arc of a circle`, `Axis — Dhidib`; standard heads Absorptive, Algorithm, Ambiguous, Between, Deviation | `Abgorptive` → `Absorptive`; `Algorithn` → `Algorithm`; `Ambigugua` → `Ambiguous`; `Angle at centte "` → `Angle at centre`; `Angles, alleruate` → `Angles, alternate`; `Are of a curve` → `Arc of a curve`; `harmenic` → `harmonic`; `Axia of revolution` → `Axis of revolution`; `Axie of synmetry` → `Axis of symmetry`; `Batween` → `Between`; `Beviation` → `Deviation`. | `repository-supported`; `form-correction` |
| EB05-R004 | 15;19-20;28;31;42;61;82;90 | Unique Somali OCR against same-file majority forms / high | Same-file `Xagal fiiqan`, `isugeynta`, `aljebraad`, `iqtiyaari`, `Saddexagal qumman`; README `Xagal qumman`; dictionary `isugeyn` | `Xagel fiigan` → `Xagal fiiqan`; `Taugeyn` → `Isugeyn`; `Jeugeynta` → `Isugeynta`; `aijebraad` → `aljebraad`; `Xagio` → `Xaglo`; `Tyo` → `Iyo`; `quoman` → `qumman`; `Xdaaab` → `Xisaab`; `igtiyaari` → `iqtiyaari`. Also `Null sec` → `Null set`. | `repository-supported`; `form-correction` |
| EB05-R005 | 43;77;484;541;557;639-640;647;814;879;990 | Mashed, unreadable, or OCR-duplicate rows / high | Same-file `Angle, vertex of an`, `Anticlockwise`, `Non-trivial solution` under `N`; geography omit-mash rule | Omit `Angle — vertex of an Gees uagleed`; `Anticlock wise…`; `General Guad General equation…`; `Hyperboloid Labosaabeke c. Hypotenuse…`; `Kumber — murakab ah`; `Negative direction Jiho taban Negative sign…`; `Neighborhood Derisnimo…`; `Number, perfect Tire — buuxda`; `Reota of an equation…`; `Simplified Fududaysan…`; and the `T`-section `Non-trivial solution` duplicate. Do not split mashed lines into new heads. | `omit-unreadable-ocr`; `unresolved` |
| EB05-R006 | 1228-1229;1349-1351;1358;1361;1368;1372; Ururinta `Isle,eg` | Unique punctuation and polarity OCR in the appended blocks / high | Same-file 1987 `isle'eg`, `Qaybin`, `Negative — Taban`; dictionary `dhammaan` | In Ururinta: `Isle,eg` / `isle,eg` → `Isle'eg` / `isle'eg`; `Integer positive — Abyoone taban` → `Abyoone togan`; `Integer negative — Abyoone togan` → `Abyoone taban`. In the symbol table: `substract` → `subtract`; `Qybin` → `Qaybin`; `le,eg` → `le'eg`; `Dhamaan` → `Dhammaan`; `Ugu dhywaan nla mid ah` → `Ugu dhawaan la mid ah`. | `repository-supported`; `form-correction` |
| EB05-R007 | Ururinta and 1987 coinages | Second-source science heads and remaining damaged Somali are this file’s printed forms / low | README: duplicate English heads dropped; cleaned chemistry `Kimistari` not copied here | Retain Ururinta as a separate block. Leave `Keal axis`, `Nautical — mile May!`, `Gdd`, `Pi (Π) — Bay`, `Element (∈) — Aalad`, and remaining quote debris on Angle rows unresolved rather than reconstructed. Do not import `01`–`04` Somali. | `intentional-retained`; `unresolved` |
| EB05-R008 | whole file | Three-block list format; OCR is as heavy as physics, plus mashed lines / medium | 1,322 pairs; one duplicate English head; no `J` | Keep one concept per line, English first, then Ururinta, then symbols. Apply only the approved spacing, unique OCR, polarity/punctuation repairs, and omits. Add no new terms. | `structural-only`; `unresolved`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Qaamuuska eray-bixinta ee Xisaab**
and keep this sequence:

1. one-line collection purpose plus a link to `00-sources.md`;
2. `## A` through `## Z` as now (no invented `J`), with a blank line before
   each heading;
3. `## Ururinta 2aad (Cilmiga)`;
4. `## Sumadaha xisaabta`.

No new term should be invented. Mashed rows are omitted, not split. Source
coinages stay as printed.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** EB05-R001 through EB05-R008
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Audit validation

- Resource SHA-256 after audit:
  `bd5291c38faef015473a7bb3e81c08262c094aa93c009922191824e6a16aa39a`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, twenty-five letter headings, two appended H2
  blocks, 1,322 em-dash pairs, one duplicate English head (`Non-trivial
  solution`), no `J` entries, blank line missing before letter headings B–Z.

## Cleanup result and review

### Applied cleanup

- Added a one-line collection note linking `00-sources.md`.
- Inserted a blank line before each letter heading from `## B` through `## Z`.
- Corrected English heads `Absorptive`, `Algorithm`, `Ambiguous`,
  `Angle at centre`, `Angles, alternate`, `Arc of a curve`, `harmonic`,
  `Axis of revolution`, `Axis of symmetry`, `Between`, and `Deviation`
  (from `Beviation`).
- Applied the unique Somali OCR repairs in EB05-R004, including
  `Xagal fiiqan`, `Isugeyn`, `Isugeynta`, `aljebraad`, `Xaglo`, `Iyo`,
  `qumman`, `Xisaab dabbagqaad`, `iqtiyaari`, and `Null set`.
- Omitted eleven mashed, unreadable, or OCR-duplicate rows, including the
  `T`-section `Non-trivial solution` (the `N`-section copy is kept).
- In Ururinta: `Isle,eg`/`isle,eg` → `Isle'eg`/`isle'eg`; polarity of
  `Integer positive`/`Integer negative` swapped to `Abyoone togan` /
  `Abyoone taban`.
- In the symbol table: `subtract`, `Qaybin`, `le'eg`, `Dhammaan`,
  `Ugu dhawaan la mid ah`.

### Deliberately retained

- No `J` section; Ururinta kept as a separate unsorted block; symbol table
  kept as a third block.
- After `Beviation` → `Deviation`, the 1987 row `Deviation — Leexsanaan`
  and the Ururinta row `Deviation — Isbedel, doorsoome` both remain.
  README’s “drop duplicate English heads” applies when folding Mansuur into
  the 1987 list; Ururinta is a separate block, so both pairs stay.
- `Keal axis`, `Nautical — mile May!`, `Gdd`, `Pi (Π) — Bay`,
  `Element (∈) — Aalad`, `Right angle — Xagel qunsaa`, and remaining
  Angle-row quote debris.
- No imported `01`–`04` Somali.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, twenty-five letter headings (A–I, K–Z) plus Ururinta
  and Sumadaha, 1,311 em-dash pairs, one remaining duplicate English head
  (`Deviation`, cross-block), blank line before every letter heading, no
  `J`.
- Line-count check: 1,381 + 2 opening + 24 blanks − 11 omits = 1,396.
- Entry-count check: 1,322 − 11 = 1,311.
- Pre-cleanup forms `Abgorptive`, `Algorithn`, `Ambigugua`, `Beviation`,
  `Xagel fiigan`, `Taugeyn`, `Jeugeynta`, `aijebraad`, `Null sec`,
  `Isle,eg`, `substract`, `Qybin`, and `Ugu dhywaan` are absent.
- Cleaned file size: 1,396 lines; 6,651 words; 41,701 bytes.
- Cleaned SHA-256:
  `b0b11736492c506ec8b6a3fc98697839635df7811738cca06902539c661add5f`.
