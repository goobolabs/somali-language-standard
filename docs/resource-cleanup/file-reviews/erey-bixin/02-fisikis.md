# Audit record — Qaamuuska eray-bixinta ee Fisikis

- **Resource path:** `resources/erey-bixin/02-fisikis.md`
- **Collection / family:** erey-bixin / historical bilingual science glossary
- **Priority:** P2
- **Method:** repository-only, line-by-line glossary audit
- **Audit status:** approved; historical-glossary cleanup applied and awaiting
  maintainer cleanup review
- **Audit started:** 2026-08-19
- **File size at audit start:** 1,432 lines; 5,887 words; 38,813 bytes
- **Resource SHA-256 at audit start:**
  `e16e7f01e569b711aa923ff47361c81ab9ddd81b94e1646c358f43e3cd77c18c`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact historical English–Somali physics glossary, not an OCR
transcript, not a dictionary rewrite, and not an SLS-native grammar chapter.
It has one H1, all twenty-six `##` letter sections, and 1,352 `English — Somali`
pairs. Blank lines already precede every letter heading. There is no cover
matter, TOC, exercises, or `## OCR Page` marker.

This collection is a **historical bilingual glossary**. Keep printed term
pairs. Do not modernize 1987 scientific coinages. Keep the letter-section list
format from `resources/erey-bixin/README.md`. Fix uniquely supported OCR. Omit
unreadable Somali sides rather than reconstructing them. Do not copy terms
from cleaned `01-bayoolaji.md` or from `04-kimistari.md`.

Cleanup should add a one-line collection note linking `00-sources.md` (no
author/year), repair the uniquely supported OCR listed below, omit the two
unreadable rows named in EB02-R005, and drop the `(ada@)` debris. It must not
bulk-normalize tildes, capitalization, or cross-file variants, and it must not
reconstruct damaged Somali.

## Source and repository evidence

`resources/erey-bixin/00-sources.md` maps this file to *Qaamuuska eray-bixinta
ee Fisikis* (Xarunta Horumarinta Manaahijta, 1987). The collection README
already states the entry format, forbids per-file provenance blocks, and keeps
attribution in `00-sources.md`.

Cleaned `01-bayoolaji.md` is the model for opening, unique-OCR repairs, and
omitting a fully unreadable row. File `09` coverage notes omit unreadable OCR
rather than reconstructing it. Overlap with biology (40 heads) and chemistry
(204 heads) is a negative check: differing Somali sides stay as this source’s
pairs unless the damage is unique inside this file.

Repository comparison also included:

- same-file `Absolute * sugan`, `haydh`, `qotintogane`, `jabaqeed`, `Sallax`,
  `Qulqul`, `walaxeed`, `Hawl-fansaar`, `dulmadow`, `jiibeed`, `dalool`;
- dictionary `haydh`, `heerkul`, `qulqul-talantaaliya` (`q.t.`), `atam`,
  `dalool`, `fisikis`, `walax`, `dayuurad`, `xawaare`, `soohdin`, `dabayl`
  (`ld dabeyl`);
- cleaned `01-bayoolaji.md` and unaudited `04-kimistari.md` only as
  do-not-import checks.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-2 | reviewed | EB02-R001 |
| 3-201 (`## A`) | reviewed | EB02-R003, EB02-R004, EB02-R005, EB02-R007 |
| 202-437 (`## B`–`## C`) | reviewed | EB02-R004, EB02-R005, EB02-R006 |
| 438-700 (`## D`–`## F`) | reviewed | EB02-R004, EB02-R005 |
| 701-1022 (`## G`–`## N`) | reviewed | EB02-R006 |
| 1023-1268 (`## O`–`## S`) | reviewed | EB02-R004, EB02-R007 |
| 1269-1432 (`## T`–`## Z`) | reviewed | EB02-R004 |
| whole file | reviewed | EB02-R002, EB02-R008 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| EB02-R001 | 1-2 | Source H1 is the right title; no collection framing or registry link / low | Cleaned `01-bayoolaji.md`; collection README keeps attribution in `00-sources.md` | Keep the printed H1. Add one Somali sentence that this is the physics glossary in this collection, with a local link to `00-sources.md`. Do not put author, year, or publisher in the content file. | `repository-supported`; `navigation-update`; `intentional-retained` |
| EB02-R002 | whole file | Compact glossary already: 1 H1, all 26 letter sections, 1,352 unique English heads, internal sort, no missing em dashes, blank line before every `##` / low | Biology lacked `K`/`Q`/`Y` and blank lines; this file does not | Preserve the letter map and spacing. Add no sections. Do not copy biology or chemistry terms. | `structural-only`; `intentional-retained` |
| EB02-R003 | 6-9;112-113;131;149-151 | English-head OCR on otherwise readable rows / high | Standard physics heads Absorptivity, Aberration, Anomalous, Anisotropy, Aperture; same-file `Absorption` / `Xagal dalool` | `Abaorptivity` → `Absorptivity`; `Aberation` → `Aberration` (three rows); `Anamolous` → `Anomalous` (two rows); `Anistropy` → `Anisotropy`; `Aperature` → `Aperture` (three rows). | `repository-supported`; `form-correction` |
| EB02-R004 | 5;7-9;11;15;43;48;80;82;91-92;136;138;141;149;155;174;222;230-234;238;244;284;286;1026;1077;1086;1400;1402 | Unique Somali OCR against same-file majority forms / high | Same-file `sugan`, `Haydh`, `jabaqeed`, `qotintogane`, `dalool`, `Qulqul`, `Sallax`, `walaxeed`, `Hawl-fansaar`, `dulmadow`, `jiibeed`; dictionary `heerkul`, `haydh`, `atam`, `fisikis`, `walax`, `dayuurad`, `xawaare`, `soohdin`, `q.t.` | `Q:T.`/`Q,T.` → `Q.T.`; `Quigul` → `Qulqul`; `augan` → `sugan`; `Yeerkul` → `Heerkul`; `Raydh`/`Yaydh` → `Haydh`; drop the colon in `Haydh: kubbadeed` and `Nuugid: Jabageed`; `Jabageed`/`fabageed` → `jabaqeed`; `Dayyurad` → `Dayuurad`; `Xavaarg -hawo` → `Xawaare hawo`; `Atag` → `Atam`; `gotintogane`/`qetintogane` → `qotintogane`; `Dalooi` → `Dalool`; `muugqata` → `muuqata`; `41ibeed` → `jiibeed`; `Kaglis` → `Xaglis`; `haba`/`aba` → `Laba` on the biconcave/bifocal rows; `meadow` → `madow`; `soohd{n` → `soohdin`; `Gotintabane` → `Qotintabane`; `qotin tebane` → `qotin tabane`; `Waltax` → `Walax`; `Fiaika` → `Fisikis`; `Ssllax` → `Sallax`; drop `!` after `Dabeyl`; `Havl` → `Hawl`. Leave `talantaaliyr` unresolved. | `repository-supported`; `form-correction`; `unresolved` |
| EB02-R005 | 68;274;632 | Fully unreadable Somali / medium | Cleaned biology omitted `Gills — Redhsdhazie`; file `09` omit-unreadable rule | Omit `Carbon cycle — Meerte Sersttsa` and `Eyepiece — @bol-Lleed`. On aerodynamics, drop `(ada@)` and keep `Eeroodaynaamik`. Do not invent replacements. Do not import biology/chemistry `Meerto kaarboon`. | `omit-unreadable-ocr`; `unresolved` |
| EB02-R006 | many `~` / `@` | Tilde compounds and leftover `@` are OCR punctuation, not Somali letters / low | Same-file `Kalareeba` versus `Kala~reebe`; chemistry `Wiriqlaawe` | Do not bulk-replace tildes. Leave `G@rguur` unresolved. Apply only the `(ada@)` drop in EB02-R005. | `unresolved`; `intentional-retained` |
| EB02-R007 | cross-file overlaps | Many heads differ from biology/chemistry on purpose / low | 40 biology overlaps, 204 chemistry overlaps, 131 chemistry disagreements | Retain this source’s coinages (`qotintogane` concatenated, `Anteena`, `Huur`, `Isku god`, and similar). Do not harmonize to `01` or `04`. | `intentional-retained` |
| EB02-R008 | whole file | Already in list format, but OCR is much heavier than biology / medium | 1,352 pairs; no duplicate English heads | Keep one concept per line, English first. Apply only the approved unique repairs and two omits. Add no new terms. Leave remaining damaged Somali unresolved, including `AlotarobL`, abundance `felitaan`/`Welitaan`/`Yelitaan`, aerial `Eer*` variants, `G@rguur`, and truncated X-ray rows. | `structural-only`; `unresolved`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Qaamuuska eray-bixinta ee Fisikis**
and keep this sequence:

1. one-line collection purpose plus a link to `00-sources.md`;
2. `## A` through `## Z` as now, with the existing blank line before each
   heading;
3. one `English — Somali` line per concept, in the current within-section
   sort, minus the two omitted rows.

No new term should be invented. Unreadable Somali is omitted or left
unresolved, not reconstructed. Source coinages stay as printed.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** EB02-R001 through EB02-R008
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Audit validation

- Resource SHA-256 after audit:
  `e16e7f01e569b711aa923ff47361c81ab9ddd81b94e1646c358f43e3cd77c18c`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, twenty-six H2 letter headings, 1,352 em-dash
  pairs, zero duplicate English heads, zero section-letter mismatches, blank
  line before every letter heading.

## Cleanup result and review

### Applied cleanup

- Added a one-line collection note linking `00-sources.md`.
- Corrected English heads `Absorptivity`, `Aberration`, `Anomalous`,
  `Anisotropy`, and `Aperture`.
- Applied the unique Somali OCR repairs in EB02-R004, including `sugan`,
  `Heerkul`, `Haydh`, `jabaqeed`, `Qulqul`/`Q.T.`, `Dayuurad`, `Xawaare hawo`,
  `Atam`, `qotintogane`/`Qotintabane`, `Dalool`, `jiibeed`, `Laba`/`Xaglis`,
  `madow`, `soohdin`, `Walax`, `Fisikis`, `Sallax`, and `Hawl`.
- Omitted `Carbon cycle — Meerte Sersttsa` and `Eyepiece — @bol-Lleed`; dropped
  `(ada@)` from aerodynamics.

### Deliberately retained

- `talantaaliyr`; concatenated `qotintogane`; `Anteena`; `Huur`.
- Tilde compounds; `G@rguur`; abundance `felitaan`/`Welitaan`/`Yelitaan`;
  aerial `Eer*` variants; `AlotarobL`; `Cathode ray Oscilloscope` `gotin-tabane`.
- No biology or chemistry Somali imported.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, twenty-six H2 letter headings, 1,350 em-dash pairs,
  zero duplicate English heads, blank line before every letter heading.
- Pre-cleanup forms `Abaorptivity`, `Aberation`, `Anamolous`, `Aperature`,
  `augan`, `Yeerkul`, `Quigul`, `Atag`, `Fiaika`, `Waltax`, `Sersttsa`, and
  `@bol-Lleed` are absent.
- Cleaned file size: 1,432 lines; 5,885 words; 38,831 bytes.
- Cleaned SHA-256:
  `72270293c521e3b4af5cf0c2e8477e8393944d7edd995f8b38011f0b1add3167`.
