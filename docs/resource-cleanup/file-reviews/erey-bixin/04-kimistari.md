# Audit record — Qaamuuska eray-bixinta ee Kimistari

- **Resource path:** `resources/erey-bixin/04-kimistari.md`
- **Collection / family:** erey-bixin / historical bilingual science glossary
- **Priority:** P1
- **Method:** repository-only, line-by-line glossary audit
- **Audit status:** approved; historical-glossary cleanup applied and awaiting
  maintainer cleanup review
- **Audit started:** 2026-08-19
- **File size at audit start:** 1,759 lines; 7,142 words; 48,371 bytes
- **Resource SHA-256 at audit start:**
  `92cc3e51dd757c52c13a1e02b66160e6c2bcbcfeefb61fd92e2d3946baec2b7e`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact historical English–Somali chemistry glossary, not an OCR
transcript, not a dictionary rewrite, and not an SLS-native grammar chapter.
It has one H1, twenty-four `##` letter sections, and 1,686 `English — Somali`
pairs. Blank lines already precede every letter heading. There is no cover
matter, TOC, exercises, or `## OCR Page` marker.

This collection is a **historical bilingual glossary**. Keep printed term
pairs. Do not modernize 1987 scientific coinages. Keep the letter-section list
format. Fix uniquely supported OCR. Omit unreadable Somali rather than
reconstructing it. Do not copy terms from cleaned `01`–`03`, and do not invent
a `J` section that has no entries.

Cleanup should add a one-line collection note linking `00-sources.md` (no
author/year), insert the missing `## I` heading before the 107 I-heads now
listed under `H`, and repair the uniquely supported OCR listed below. It must
not re-sort within letters, must not move `Ortho-`/`Para-` rows out of `B` and
`X`, and must not bulk-normalize `Oorgaanik` / `orgaanik` coinages.

## Source and repository evidence

`resources/erey-bixin/00-sources.md` maps this file to *Qaamuuska eray-bixinta
ee Kimistari* (Xarunta Horumarinta Manaahijta, 1987). The collection README
already states the entry format and keeps attribution in `00-sources.md`.

Cleaned `01`–`03` are the model for opening and unique-OCR repairs. Biology
already uses `Kimistariga noolaha` for Biochemistry; that row here matches and
must not be rewritten. Geography’s `Aluminium — Jaandi` must not be imported.

Repository comparison also included:

- same-file `Kaadmiyam`, `Saabaanka Kib`, `Hubsashada`/`Hubsasho`, `orgaanik
  ma ahe`, `Atam`;
- dictionary `qori`, `kimistariga-noolaha`, `kimistariga-orgaanig-ma-ahe`;
- cleaned physics `Atam` and `Gan qori` only as corroboration, not as
  authority to copy other physics terms.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-2 | reviewed | EB04-R001 |
| 3-150 (`## A`) | reviewed | EB04-R005 |
| 151-424 (`## B`–`## C`) | reviewed | EB04-R002, EB04-R004 |
| 425-642 (`## D`–`## G`) | reviewed | EB04-R005 |
| 643-845 (`## H` including I-heads) | reviewed | EB04-R003, EB04-R004 |
| 846-1759 (`## K`–`## Z`) | reviewed | EB04-R002, EB04-R004, EB04-R005 |
| whole file | reviewed | EB04-R002, EB04-R006 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| EB04-R001 | 1-2 | Source H1 is the right title; no collection framing or registry link / low | Cleaned `01`–`03`; attribution lives in `00-sources.md` | Keep the printed H1. Add one Somali sentence that this is the chemistry glossary in this collection, with a local link to `00-sources.md`. Do not put author, year, or publisher in the content file. | `repository-supported`; `navigation-update`; `intentional-retained` |
| EB04-R002 | whole file | Compact glossary: 1 H1, 24 letter sections, 1,686 unique English heads, blank line before every `##`. `I` heading is missing (107 I-heads sit under `H`). No `J` entries exist. `Ortho-`/`Para-` rows sit under `B` and `X`. Within-letter order is not strict A–Z / medium | Physics/geography have `## I`; this file has I-heads from `Ideal gas` to `Isotope`; no `Joule`/`Junction` rows | Preserve all 1,686 pairs. Insert `## I` as EB04-R003. Do not invent a `J` section. Do not move `Para-bromobenzaldehyde` / `Ortho-bromobenzyl cyanide` out of `B`, or `Meta-`/`Ortho-`/`Para-xylene` out of `X`. Do not re-sort. | `structural-only`; `intentional-retained` |
| EB04-R003 | 737-738 | `Hypothesis` is the last H-head; `Ideal gas` begins 107 I-heads with no heading / high | Same-file `## K` follows `Isotope`; cleaned `02`/`03` use `## I` | Insert `## I` with the usual blank line before and after, immediately after `Hypothesis — Afeef`. Do not add I-terms. | `structural-only`; `repository-supported` |
| EB04-R004 | 227;252;647;729;774;1729 | Unique OCR against same-file majority forms / high | Same-file `Hubsashada`/`Hubsasho`, `Kaadmiyam`×5, `Saabaanka Kib`, `orgaanik ma ahe` on the following inorganic rows; dictionary `qori`; `Hypo-` cluster | `Hubaashada` → `Hubsashada`; `Kaadmiyan` → `Kaadmiyam`; `Saabadnka` → `Saabaanka`; `Hybo` → `Hypo`; `Inorganic — Organik ma ahe` → `Orgaanik ma ahe`; `Alkohool gori` → `Alkohool qori`. Do not change `Oorgaanik` on the organic rows. | `repository-supported`; `form-correction`; `intentional-retained` |
| EB04-R005 | cross-file and source coinages | Chemistry coinages and dual heads are this source’s printed forms / low | Biology `Isotope — Isku nooc` vs `Aysotoob`; geography `Aluminium — Jaandi` vs `Aluuminiyam`; `Chemistry — Kimistari` matches the H1 | Retain `Aluuminiyam`, `Aysotoob`, `Kimistari`, `Oorgaanik`, `Wolfram=Tungsten`, `Heptavalent`/`Hexavalent` both `Lix kaaftoonle`, `Xeerka bool`, `Henary`, `Hegeoos`, `Faiqid`, `Haakniyam`, and `Ysterbiyam`. Do not import geography or physics Somali. | `intentional-retained`; `unresolved` |
| EB04-R006 | whole file | Already in list format; OCR is lighter than physics/geography / medium | 1,686 pairs; no duplicate English heads; no `@`/`~` debris rows | Keep one concept per line, English first. Apply only the opening, the `## I` insert, and the unique OCR repairs. Add no new terms. Omit no rows. | `structural-only`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Qaamuuska eray-bixinta ee Kimistari**
and keep this sequence:

1. one-line collection purpose plus a link to `00-sources.md`;
2. `## A` through `## Z` as now, plus the restored `## I` after `H`, with no
   invented `J`;
3. one `English — Somali` line per concept, in the current within-section
   order, including prefixed bromo and xylene rows in their present sections.

No new term should be invented. Source coinages stay as printed.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** EB04-R001 through EB04-R006
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Audit validation

- Resource SHA-256 after audit:
  `92cc3e51dd757c52c13a1e02b66160e6c2bcbcfeefb61fd92e2d3946baec2b7e`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, twenty-four H2 letter headings (A–H, K–Z), 1,686
  em-dash pairs, zero duplicate English heads, 107 I-heads currently under
  `H`, no `J` entries.

## Cleanup result and review

### Applied cleanup

- Added a one-line collection note linking `00-sources.md`.
- Inserted `## I` after `Hypothesis — Afeef` (107 I-heads).
- Corrected `Hubaashada`, `Kaadmiyan`, `Saabadnka`, `Hybo`, `Organik ma ahe`,
  and `gori`.

### Deliberately retained

- `Aluuminiyam`, `Aysotoob`, `Kimistari`, `Oorgaanik`, `Wolfram=Tungsten`.
- Prefixed bromo rows in `B` and xylene rows in `X`.
- No `J` section; no omitted rows; no imported geography/physics Somali.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, twenty-five H2 letter headings (A–I, K–Z), 1,686 em-dash
  pairs, zero duplicate English heads, 107 I-heads under `## I`, no `J`.
- Pre-cleanup forms `Hubaashada`, `Kaadmiyan`, `Saabadnka`, `Hybo —`,
  `Organik ma ahe`, and `Alkohool gori` are absent.
- Cleaned file size: 1,764 lines; 7,151 words; 48,462 bytes.
- Cleaned SHA-256:
  `bf07bc120e4bc16c50c24988bdbfa8111c716b7f0109083ad8334e6f054048cd`.
