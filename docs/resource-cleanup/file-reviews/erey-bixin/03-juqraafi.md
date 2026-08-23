# Audit record — Qaamuuska eray-bixinta ee Juqraafi

- **Resource path:** `resources/erey-bixin/03-juqraafi.md`
- **Collection / family:** erey-bixin / historical bilingual science glossary
- **Priority:** P2
- **Method:** repository-only, line-by-line glossary audit
- **Audit status:** approved; historical-glossary cleanup applied and awaiting
  maintainer cleanup review
- **Audit started:** 2026-08-19
- **File size at audit start:** 922 lines; 3,934 words; 23,898 bytes
- **Resource SHA-256 at audit start:**
  `c5b1e8c93660020e92ffdf4ce8f519a20fac473e0e0a6b7ce1d9b931dd6d5f64`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact historical English–Somali geography glossary, not an OCR
transcript, not a dictionary rewrite, and not an SLS-native grammar chapter.
It has one H1, all twenty-six `##` letter sections, and 843 `English — Somali`
pairs. Blank lines already precede every letter heading. There is no cover
matter, TOC, exercises, or `## OCR Page` marker.

This collection is a **historical bilingual glossary**. Keep printed term
pairs. Do not modernize 1987 scientific coinages. Keep the letter-section list
format. Fix uniquely supported OCR. Omit unreadable or mashed rows rather than
reconstructing them. Do not copy terms from cleaned `01-bayoolaji.md` or
`02-fisikis.md`, or from `04-kimistari.md`. Do not move stray-S rows into other
letter sections.

Cleanup should add a one-line collection note linking `00-sources.md` (no
author/year), repair the uniquely supported OCR listed below, strip trailing
scan debris from otherwise readable rows, and omit the mashed or fully
unreadable rows in EB03-R005. It must not bulk-normalize `Brojakshan` /
`Dabeyl` variants or invent missing English heads.

## Source and repository evidence

`resources/erey-bixin/00-sources.md` maps this file to *Qaamuuska eray-bixinta
ee Juqraafi* (Xarunta Horumarinta Manaahijta, 1987). The collection README
already states the entry format and keeps attribution in `00-sources.md`.

Cleaned `01-bayoolaji.md` and `02-fisikis.md` are the model for opening,
unique-OCR repairs, and omitting unreadable rows. Same-file duplicates that
differ only by OCR (`Coseismicline`, `Courter-trades`, `Paraseiene`, mashed
`Wel`) are omitted rather than “fixed” into a second identical head.

Repository comparison also included:

- same-file `heerkul`, `Aqoon`, `Marin`, `qaaradeed`, `Dhaqdhaqaaq`, `Qorrax`,
  `Joog ekeeye`, `Huur tuse`, `Pelagic deposits`, `Well — Ceel`;
- dictionary `heerkul`, `dhidib-dhuleed`, `oomane`, `filiqsanaan`, `gegi`
  (`ld gego`), `juqraafi`, `qorrax`, `xeerka-*`;
- cleaned physics `Dabeyl` / `Dhidib` / `Huur` only as do-not-harmonize
  checks.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-2 | reviewed | EB03-R001 |
| 3-87 (`## A`) | reviewed | EB03-R003, EB03-R004, EB03-R007 |
| 88-267 (`## B`–`## C`) | reviewed | EB03-R004, EB03-R005, EB03-R006 |
| 268-508 (`## D`–`## H`) | reviewed | EB03-R004, EB03-R005, EB03-R006 |
| 509-744 (`## I`–`## R`) | reviewed | EB03-R005 |
| 745-922 (`## S`–`## Z`) | reviewed | EB03-R005, EB03-R006 |
| whole file | reviewed | EB03-R002, EB03-R008 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| EB03-R001 | 1-2 | Source H1 is the right title; no collection framing or registry link / low | Cleaned `01` and `02`; attribution lives in `00-sources.md` | Keep the printed H1. Add one Somali sentence that this is the geography glossary in this collection, with a local link to `00-sources.md`. Do not put author, year, or publisher in the content file. | `repository-supported`; `navigation-update`; `intentional-retained` |
| EB03-R002 | whole file | Compact glossary already: 1 H1, all 26 letters, 843 unique English heads, internal sort, blank line before every `##` / low | Physics file had the same complete letter map | Preserve the letter map and spacing. Add no sections. Do not move `SGeocentric` or `SHarmattaan` into `G`/`H`. | `structural-only`; `intentional-retained` |
| EB03-R003 | 9;74;147 | English-head OCR on otherwise readable rows / high | Standard heads Abyssal, Asthenosphere, Calcification | `Abysal` → `Abyssal`; `Astenosphere` → `Asthenosphere`; `Caleification` → `Calcification`. | `repository-supported`; `form-correction` |
| EB03-R004 | 10-11;28;33;64;71;83;140;224;228;304;329 | Unique Somali OCR against same-file majority forms / high | Same-file `heerkul`, `Aqoon yahan`, `Marin webi`, `qaaradeed`, `Dhaqdhaqaaqyo`, `Qorrax madoobaad`; dictionary `dhidib-dhuleed`, `oomane`, `filiqsanaan`, `xeerka-*` | `heerkui` → `heerkul`; `Dhaqdhaqaag` → `Dhaqdhaqaaq`; `Agoon` → `Aqoon`; `Marinks cirka` → `Marin cirka`; `gorraxeed` → `qorraxeed`; `Ooomane` → `Oomane`; `Dhidid` → `Dhidib`; `Keerka` → `Xeerka`; `qaaratieed`/`qaatadeed` → `qaaradeed`; `Filigsanaan` → `Filiqsanaan`; `Dhagdhaqaaga` → `Dhaqdhaqaaqa`. Keep `Gego dayuuradeed` (dictionary `gego`). | `repository-supported`; `form-correction`; `intentional-retained` |
| EB03-R005 | 113;125;234;244;248;300;309;654;765-766;892 | Unreadable, mashed, or OCR-duplicate rows / high | Same-file `Coseismic line`, `Counter-trades`, `Paraselene`, `Pelagic deposits`, `Well — Ceel`; biology/physics omit-unreadable rule | Omit `Bey “`, `Bos`, `Copse`, `Coseismicline`, `Courter-trades`, `Diet — Holding land…`, `Dogmatic`, `Paraseiene`, `Sees`, `Sef`, and `Wel — w= Pelagic deposits…`. Do not split mashed lines into new heads. Do not import replacements. | `omit-unreadable-ocr`; `unresolved` |
| EB03-R006 | 124;230;463;507;772 | Trailing scan debris on otherwise readable rows / medium | Same-file `Joog ekeeye`, `Huur`, `Bora` identity | `Bora “ — Bora` → `Bora — Bora`; `Contour 2 — Joogekeeye a` → `Contour — Joog ekeeye`; drop the curly quote on `Grit` (leave `eect` unresolved); `Hygroscope` → `Huur tuse`; `Serra` → `Silsilad buureed`. | `repository-supported`; `form-correction`; `unresolved` |
| EB03-R007 | cross-file and source coinages | Many forms are this source’s spellings, not biology/physics errors / low | Physics `Dabeyl` vs this file `Dabayl`; biology `Xerophyte — Geed abaareed` vs `Dhir abaareed`; `Atmosphere — Atmosphere` | Retain `Gego`, `Juqraafi`, `Atmosphere — Atmosphere`, mixed `Dabayl`/`Dabeyl`, `Brojakshan` variants, `SGeocentric`, `SHarmattaan`, and `Anti trades — Xageashi`. Do not harmonize to files `01`–`02`. | `intentional-retained` |
| EB03-R008 | whole file | Already in list format, with a denser scan-debris tail than biology / medium | 843 pairs; no duplicate English heads today | Keep one concept per line, English first. Apply only the approved unique repairs, debris strips, and omits. Add no new terms. Leave remaining damaged Somali unresolved. | `structural-only`; `unresolved`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Qaamuuska eray-bixinta ee Juqraafi**
and keep this sequence:

1. one-line collection purpose plus a link to `00-sources.md`;
2. `## A` through `## Z` as now, with the existing blank line before each
   heading;
3. one `English — Somali` line per concept, in the current within-section
   sort, minus the omitted debris and OCR-duplicate rows.

No new term should be invented. Unreadable Somali is omitted or left
unresolved, not reconstructed. Source coinages stay as printed.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** EB03-R001 through EB03-R008
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Audit validation

- Resource SHA-256 after audit:
  `c5b1e8c93660020e92ffdf4ce8f519a20fac473e0e0a6b7ce1d9b931dd6d5f64`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, twenty-six H2 letter headings, 843 em-dash
  pairs, zero duplicate English heads, zero section-letter mismatches, blank
  line before every letter heading.

## Cleanup result and review

### Applied cleanup

- Added a one-line collection note linking `00-sources.md`.
- Corrected English heads `Abyssal`, `Asthenosphere`, and `Calcification`.
- Applied the unique Somali OCR repairs in EB03-R004.
- Stripped trailing debris on `Bora`, `Contour`, `Grit`, `Hygroscope`, and
  `Serra`.
- Omitted eleven mashed, unreadable, or OCR-duplicate rows.

### Deliberately retained

- `Gego dayuuradeed`, `Juqraafi`, `Atmosphere — Atmosphere`.
- Mixed `Dabayl`/`Dabeyl`, `Brojakshan` variants, `SGeocentric`,
  `SHarmattaan`, `Anti trades — Xageashi`.
- `Grit` Somali `eect`; `Coseismic line`, `Counter-trades`, `Paraselene`,
  `Well — Ceel`, and `Pelagic deposits` kept once.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, twenty-six H2 letter headings, 832 em-dash pairs, zero
  duplicate English heads, blank line before every letter heading.
- Pre-cleanup forms `Abysal`, `Astenosphere`, `Caleification`, `heerkui`,
  `Agoon`, `Marinks`, `Ooomane`, `Dhidid`, `Keerka`, `Coseismicline`,
  `Courter-trades`, `Paraseiene`, and `Wel — w=` are absent.
- Cleaned file size: 913 lines; 3,876 words; 23,639 bytes.
- Cleaned SHA-256:
  `a14b483f2891b83b27e893fa38273f85f4de76a85c5be2bdb96254d077b82ac8`.
