# Audit record — Barbaarinta Jirka — Qaamuuska Ereybixinta

- **Resource path:** `resources/erey-bixin/07-barbaarinta-jirka.md`
- **Collection / family:** erey-bixin / historical bilingual physical-education glossary
- **Priority:** P2
- **Method:** repository-only, line-by-line glossary audit
- **Audit status:** approved; historical-glossary cleanup applied and awaiting
  maintainer cleanup review
- **Audit started:** 2026-08-19
- **File size at audit start:** 643 lines; 3,251 words; 19,340 bytes
- **Resource SHA-256 at audit start:**
  `2cd63a9cd7904fe62176ee026fa8b3bfd24da1adc39b79006090511e34f2b193`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact historical English–Somali physical-education glossary,
not an OCR transcript, not a dictionary rewrite, and not an A–Z science list.
It has one H1, seven domain headings (`## Ereybixin guud`, four `## Kubbadda
…` sports, `## Qaalmorogadka`, `## Gawaan & saroon (ciyaaraha fudud)`), and
621 `English — Somali` pairs. Blank lines already precede every heading.
Somali equivalents are lowercase throughout. There is no cover matter, TOC,
exercises, or `## OCR Page` marker.

This collection is a **historical bilingual glossary**. Keep printed term
pairs. Do not modernize 1984 coinages. Keep the domain-heading list format
named in `resources/erey-bixin/README.md`. Fix uniquely supported OCR. Omit
mashed or unreadable rows rather than splitting them into new heads. Do not
invent extra sport sections. Do not copy terms from cleaned `06-wasaaradaha.md`
(its Cayaaraha block uses different coinages, and some of those English heads
are themselves split).

Cleanup should add a one-line collection note linking `00-sources.md` (no
author/year) and repair the uniquely supported OCR listed below. No rows are
proposed for omission.

## Source and repository evidence

`resources/erey-bixin/00-sources.md` maps this file to *Barbaarinta Jirka —
Qaamuuska Ereybixinta* (1984). The collection README already names domain
headings (`## Kubbadda …`) as the section model for this file.

Repository comparison also included:

- same-file `Sportswoman`, `Women's Competitions`, `Front-line player`,
  `Back zone`, `xarriigda`, `kubbadda`, `xukumidda`, `qof-qof`, `difaace`,
  `xadka`, `madax`, `saxan tuurka`, `shakhsi`, `garoonka`, `ka saarid`,
  `gool`, `dhiibasho`, `Gymnastic apparatus`;
- dictionary `sharci`, `madax`, `saxan`, `noqosho`, `ciyaartoy` (used only
  as support, not as a rewrite target);
- cleaned `06-wasaaradaha.md` Cayaaraha terms (`Jimicsi`, `Gool haye`,
  `Dusmo`, `Shebeg`, volleyball mapped to `kubbadda gacanta`) only as
  do-not-import checks.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-2 | reviewed | EB07-R001 |
| 3-198 (`## Ereybixin guud`) | reviewed | EB07-R003, EB07-R004, EB07-R007 |
| 200-285 (`## Kubbadda koleyga`) | reviewed | EB07-R004 |
| 287-346 (`## Kubbadda cagta`) | reviewed | EB07-R004, EB07-R007 |
| 348-405 (`## Qaalmorogadka`) | reviewed | EB07-R003, EB07-R007 |
| 407-463 (`## Kubbadda gacanta`) | reviewed | EB07-R004 |
| 465-559 (`## Gawaan & saroon`) | reviewed | EB07-R003, EB07-R005, EB07-R007 |
| 561-643 (`## Kubbadda laliska`) | reviewed | EB07-R003, EB07-R004 |
| whole file | reviewed | EB07-R002, EB07-R006, EB07-R008 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| EB07-R001 | 1-2 | Source H1 is the right title; no collection framing or registry link / low | Cleaned `01`–`06`; attribution lives in `00-sources.md` | Keep the printed H1, including its emdash. Add one Somali sentence that this is the physical-education glossary in this collection, with a local link to `00-sources.md`. Do not put author or year in the content file. | `repository-supported`; `navigation-update`; `intentional-retained` |
| EB07-R002 | whole file | Compact seven-block domain glossary already; blanks present; 31 cross-section duplicate English heads / medium | README domain-heading model; ministry file kept cross-ministry duplicate heads | Keep all seven headings and the current within-section order. Do not A–Z sort. Do not drop `Field of play` and other cross-sport duplicates. Do not invent boxing, tennis, or other missing sport sections. | `structural-only`; `intentional-retained` |
| EB07-R003 | 115;122;353;487;599 | Unique English-head OCR on otherwise readable rows / high | Same-file `Sportswoman`, `Women's Competitions`, `Front-line player`, `Back zone`, `Gymnastic apparatus` | `Spinned shot` → `Spin shot`; `Sportman` → `Sportsman`; `Artistic gymnastic` → `Artistic gymnastics`; `Man's Competitions` → `Men's Competitions`; `Pack-line player` → `Back-line player`. | `repository-supported`; `form-correction` |
| EB07-R004 | 52;70;108;149;180;205;210;289;315;328;433;475;564;567;581;599;600;622 | Unique Somali OCR against same-file majority forms / high | Same-file `gool`, `qof-qof`, `sharciga` / `sharci jebin`, `ka saarid`, `difaace`, `xukumidda kubbadda`, `xadka`, `garoonka`, `madax`, `saxan tuurka`, `dhiibasho`, `xarriigda`, `shakhsi`; dictionary `noqosho` | `golal` → `goolal`; `gof-qof` → `qof-qof`; `sharc` → `sharci`; `ke saarid` → `ka saarid`; `daaface` → `difaace`; `xukumaada kubadda` → `xukumidda kubbadda` (both Ball-control rows); `xedke` → `xadka`; `garoonke` → `garoonka`; `soo nogosho` / `soo nogotay` → `soo noqosho` / `soo noqotay`; `wadax` → `madax`; `gaxan tuure` → `saxan tuure`; volleyball `xarriiqda` → `xarriigda` (four rows); `dhiibisho` → `dhiibasho`; `shaqsi` → `shakhsi`. Do not rewrite majority `xarriigda` from dictionary `xarriiq`. | `repository-supported`; `form-correction` |
| EB07-R005 | 519-524; 558-559 | No mashed multi-entry rows; distance identities and quoted commands are printed pairs / low | `01`–`06` omit-mash rule; nothing here matches that pattern | Omit no rows. Keep `100m, 200m, 400m — 100m, 200m, 400m` and the other identity distance rows. Keep `"On your marks"` and `"Set"`. | `intentional-retained` |
| EB07-R006 | whole file | Somali sides are lowercase; other erey-bixin files title-case Somali / low | Cleaned `06` title-cases Somali; this source does not | Keep lowercase Somali. Do not title-case to match `01`–`06`. No split-head joins are proposed. | `structural-only`; `intentional-retained` |
| EB07-R007 | headings; 1984 coinages; remaining damaged Somali | Source headings and remaining doubtful Somali are this file’s printed forms / low | Cleaned `06` `Jimicsi`, `Dusmo`, `Shebeg`, and volleyball→`kubbadda gacanta` are not this source | Retain `Qaalmorogadka`, `Gawaan & saroon`, `koleyga`, `laliska`, `hobsayd`, `haardal`, `baloog`, `ciyaaryahan` on the basketball player row, mixed `shabaga`/`shabagga`, `dheeli tir`, `keli-ke-bixida`, `dhul-ke-kac`, `birte`, `goolkeyaga`. Leave `dayac`, `farriin`, `tiir` (Shower), `aflagaado`, `jairid`, `furaya`, `decalka` unresolved. Do not import `06` Somali. | `intentional-retained`; `unresolved` |
| EB07-R008 | whole file | Domain-heading list format; OCR is light compared with `06` / medium | 621 pairs; 7 H2s; 31 cross-section duplicate English heads | Keep one concept per line, English first. Apply only the approved unique OCR. Add no new terms. | `structural-only`; `unresolved`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Barbaarinta Jirka — Qaamuuska
Ereybixinta** and keep this sequence:

1. one-line collection purpose plus a link to `00-sources.md`;
2. `## Ereybixin guud`;
3. `## Kubbadda koleyga`;
4. `## Kubbadda cagta`;
5. `## Qaalmorogadka`;
6. `## Kubbadda gacanta`;
7. `## Gawaan & saroon (ciyaaraha fudud)`;
8. `## Kubbadda laliska`.

No new term should be invented. Source coinages stay as printed. No mashed
row is omitted.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** EB07-R001 through EB07-R008
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Audit validation

- Resource SHA-256 after audit:
  `2cd63a9cd7904fe62176ee026fa8b3bfd24da1adc39b79006090511e34f2b193`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, seven domain H2s, 621 em-dash pairs, blank line
  before every heading, zero starred apparatus lines, zero non-emdash content
  lines, 31 cross-section duplicate English heads, no `## OCR Page` marker.

## Cleanup result and review

### Applied cleanup

- Added a one-line collection note linking `00-sources.md`.
- Corrected English heads `Spin shot`, `Sportsman`, `Artistic gymnastics`,
  `Men's Competitions`, and `Back-line player`.
- Applied the unique Somali OCR repairs in EB07-R004, including `goolal`,
  `qof-qof`, `sharci`, `ka saarid`, `difaace`, `xukumidda kubbadda`, `xadka`,
  `garoonka`, `noqosho`/`noqotay`, `madax`, `saxan tuure`, volleyball
  `xarriigda`, `dhiibasho`, and `shakhsi`.

### Deliberately retained

- All seven domain headings; lowercase Somali; cross-sport duplicate English
  heads; identity distance rows and quoted start commands.
- `Qaalmorogadka`, `koleyga`, `laliska`, `hobsayd`, `haardal`, `baloog`,
  mixed `shabaga`/`shabagga`, `dheeli tir`, `keli-ke-bixida`, `birte`.
- Unresolved Somali: `dayac`, `farriin`, `tiir` (Shower), `aflagaado`,
  `jairid`, `furaya`, `decalka`.
- No omitted rows; no imported `06` Somali; majority `xarriigda` not rewritten
  from dictionary `xarriiq`.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, seven domain H2s, 621 em-dash pairs, blank line before
  every heading.
- Line-count check: 643 + 2 opening = 645.
- Entry-count check: 621 (unchanged).
- Pre-cleanup forms `Spinned`, `Sportman`, `Pack-line`, `golal`, `gof-qof`,
  `xukumaada`, `xarriiqda`, `dhiibisho`, `gaxan`, and `shaqsi` are absent.
- Cleaned file size: 645 lines; 3,259 words; 19,433 bytes.
- Cleaned SHA-256:
  `d71e0f146f962307128e4c7b1384ec6141ed863b22f04b91903f1ec132010c7b`.
