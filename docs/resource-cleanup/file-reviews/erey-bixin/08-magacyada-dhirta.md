# Audit record — Magacyada dhirta Soomaaliyeed

- **Resource path:** `resources/erey-bixin/08-magacyada-dhirta.md`
- **Collection / family:** erey-bixin / historical bilingual plant-name glossary
- **Priority:** P2
- **Method:** repository-only, line-by-line glossary audit
- **Audit status:** approved; historical-glossary cleanup applied and awaiting
  maintainer cleanup review
- **Audit started:** 2026-08-19
- **File size at audit start:** 352 lines; 1,148 words; 7,993 bytes
- **Resource SHA-256 at audit start:**
  `f684455f4c487f7d72fa51f05680af2120a7061cf1acc823e772e75e96ea6cdb`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact historical scientific–Somali plant-name list, not an
OCR transcript, not a flora rewrite, and not an A–Z of Latin names. It has
one H1, all twenty-six `##` letter sections grouped by **Somali** initial,
and 273 `Genus epithet — Vernacular` pairs. Blank lines already precede
every heading. Author citations are already omitted. There is no cover
matter, TOC, exercises, or `## OCR Page` marker.

This collection is a **historical bilingual glossary**. Keep printed term
pairs. Do not modernize 1985 vernaculars or 1985 taxonomy (do not replace
`Acacia` with `Vachellia`, or `Sansevieria` with `Dracaena`). Keep the
Somali-initial letter-section format named in `resources/erey-bixin/README.md`.
Fix uniquely supported OCR. Omit mashed or unreadable rows rather than
splitting them. Do not invent missing taxa. Do not copy terms from cleaned
`01`–`07`. Do not drop a binomial just because another vernacular for the
same species already exists.

Cleanup should add a one-line collection note linking `00-sources.md` (no
author/year), move `## F` so the five F-initial rows are not left under
`E`, and repair the uniquely supported Latin (and one Somali) OCR listed
below. No rows are proposed for omission.

## Source and repository evidence

`resources/erey-bixin/00-sources.md` maps this file to *Magacyada dhirta
Soomaaliyeed* (English scientific — Somali, 1985). The collection README
already requires: left side = accepted scientific binomial with author
citations omitted; sections grouped by Somali initial letter.

Repository comparison also included:

- same-file `Enneapogon schimperanus` (twice), `Commiphora erlangerana`
  (twice), `Caws-dameer`, `Boswellia` / `Yegaar` / `Gegar`, `Balanites`,
  `Ayax-makare`;
- dictionary `yicib`, `caday` (support only; not a reason to drop printed
  variants `Yeheb`, `Allan`, `Ellan`);
- cleaned `01-bayoolaji.md` as a do-not-import check (no overlapping plant
  binomials there).

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-2 | reviewed | EB08-R001 |
| 3-53 (`## A`) | reviewed | EB08-R003, EB08-R006 |
| 55-111 (`## B`–`## C`) | reviewed | EB08-R003, EB08-R004 |
| 113-150 (`## D`–`## F`) | reviewed | EB08-R002 |
| 152-204 (`## G`–`## J`) | reviewed | EB08-R003, EB08-R006 |
| 206-269 (`## K`–`## Q`) | reviewed | EB08-R003, EB08-R007 |
| 271-352 (`## R`–`## Z`) | reviewed | EB08-R006, EB08-R007 |
| whole file | reviewed | EB08-R002, EB08-R005, EB08-R008 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| EB08-R001 | 1-2 | Source H1 is the right title; no collection framing or registry link / low | Cleaned `01`–`07`; attribution lives in `00-sources.md` | Keep the printed H1. Add one Somali sentence that this is the plant-name glossary in this collection, with a local link to `00-sources.md`. Do not put author or year in the content file. | `repository-supported`; `navigation-update`; `intentional-retained` |
| EB08-R002 | 138-144; whole file | Compact Somali-initial A–Z glossary already; all 26 letters present; blanks present. Five F-initial vernaculars sit under `## E` / medium | README: sections grouped by Somali initial; chemistry restored `## I` because I-heads sat under `H` | Move `## F` to immediately after `Lawsonia inermis — Ellan`, so `Faradheere` through `Fifiole` fall under F. Do not re-sort within F. Keep all 26 headings. Do not A–Z by Latin name. Do not drop the 40 binomials that have more than one vernacular. | `structural-only`; `repository-supported`; `intentional-retained` |
| EB08-R003 | 27;31;35;53;69;174;250;258-259 | Unique Latin-epithet OCR on otherwise readable 1985 rows / high | Same-file `schimperanus` twice and `erlangerana` twice; standard 1985 spellings Dichanthium, fastigiata, phillipsiae, aegyptiaca, frereana, schimperianus, erlangeriana | `phillipwiae` → `phillipsiae`; `famstigiata` → `fastigiata`; `Dicanthium` → `Dichanthium`; both `schimperanus` → `schimperianus`; `aegyptica` → `aegyptiaca`; `freereana` → `frereana`; both `erlangerana` → `erlangeriana`. Do not replace `Acacia`, `Sansevieria`, or `Boswellia carteri`. | `repository-supported`; `form-correction` |
| EB08-R004 | 106 | Unique Somali OCR against a same-file majority form / low | Same-file `Caws-dameer` | `Caws-damer` → `Caws-dameer`. Do not merge `Aan-ole`/`Aan-orle`, `Abac`/`Abak`/`Agac`, `Jeeb`/`Yeheb`/`Yicib`, or `Allan`/`Ellan`/`Cillaan`. | `repository-supported`; `form-correction`; `intentional-retained` |
| EB08-R005 | whole file | No mashed multi-entry rows; `spp.` / `sp.` are genus-level heads, not author citations / low | README already omits author citations; `01`–`07` omit-mash rule | Omit no rows. Keep `Justicia spp.`, `Loranthus spp.`, and `Cassia sp.`. | `intentional-retained` |
| EB08-R006 | vernacular variants | Multiple Somali names per species, including Italianate spellings, are this source’s printed forms / low | Dictionary `yicib` and `caday` do not license dropping printed variants | Retain all distinct vernacular lines, including `Eddi-scebel`, `Banane`, `Venino`, `Verdei`, `Zibleola`, and the yeheb/`yicib` cluster. | `intentional-retained` |
| EB08-R007 | 1985 taxonomy; remaining doubtful Latin | 1985-era genera and remaining weakly supported epithets stay as printed / low | Cleaned biology has no plant binomials to import | Retain `Acacia`, `Sansevieria`, `Boswellia carteri`, `Notonia`, `crenato-lobata`, `zizyphispina`, `Adenium somalensis`, `Jatropha palmatifolia`, `Pieris scabra`. Do not import `01`–`07` Somali. | `intentional-retained`; `unresolved` |
| EB08-R008 | whole file | Somali-initial list format; OCR is light / medium | 273 pairs; 191 distinct binomials; 26 H2s; 5 section mismatches | Keep one vernacular per line, scientific name first. Apply only the approved heading move and unique OCR. Add no new taxa. | `structural-only`; `unresolved`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Magacyada dhirta Soomaaliyeed**
and keep this sequence:

1. one-line collection purpose plus a link to `00-sources.md`;
2. `## A` through `## Z` as now, with `## F` sitting before the F-initial
   vernaculars;
3. one `Genus epithet — Vernacular` line per printed name, in the current
   within-section order.

No new taxon should be invented. 1985 vernaculars and 1985 taxonomy stay as
printed. No mashed row is omitted.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** EB08-R001 through EB08-R008
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Audit validation

- Resource SHA-256 after audit:
  `f684455f4c487f7d72fa51f05680af2120a7061cf1acc823e772e75e96ea6cdb`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, twenty-six Somali-initial H2s, 273 em-dash
  pairs, blank line before every heading, zero author citations, zero exact
  duplicate lines, 40 binomials with more than one vernacular, five F-initial
  rows currently under `E`, no `## OCR Page` marker.

## Cleanup result and review

### Applied cleanup

- Added a one-line collection note linking `00-sources.md`.
- Moved `## F` to after `Ellan`, so `Faradheere` through `Fifiole` sit under
  F (zero remaining section-letter mismatches).
- Corrected Latin epithets `phillipsiae`, `fastigiata`, `Dichanthium`,
  `schimperianus` (two rows), `aegyptiaca`, `frereana` (two rows), and
  `erlangeriana` (two rows).
- Corrected `Caws-damer` → `Caws-dameer`.

### Deliberately retained

- All 26 Somali-initial headings; 40 binomials with more than one vernacular;
  `spp.` / `sp.` rows.
- 1985 taxonomy: `Acacia`, `Sansevieria`, `Boswellia carteri`, `Notonia`,
  `Adenium somalensis`.
- Vernacular variants including `Aan-ole`/`Aan-orle`, `Yeheb`/`Yicib`,
  `Eddi-scebel`, `Banane`, `Venino`.
- No omitted rows; no imported `01`–`07` terms.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, twenty-six Somali-initial H2s, 273 em-dash pairs, blank
  line before every heading, zero section-letter mismatches.
- Line-count check: 352 + 2 opening = 354.
- Entry-count check: 273 (unchanged).
- Pre-cleanup forms `phillipwiae`, `famstigiata`, `Dicanthium`,
  `schimperanus`, `aegyptica`, `freereana`, `erlangerana`, and `Caws-damer`
  are absent.
- Cleaned file size: 354 lines; 1,156 words; 8,085 bytes.
- Cleaned SHA-256:
  `70891b65c2c67b57eb4fe2c4d8b2b1c6c37de45830670579528a08ed4754dacd`.
