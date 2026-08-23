# Audit record — Farsamada iyo Culuunta

- **Resource path:** `resources/erey-bixin/09-farsamada-culuunta.md`
- **Collection / family:** erey-bixin / historical bilingual industrial glossary (partial)
- **Priority:** P1
- **Method:** repository-only, line-by-line glossary audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 468 lines; 2,122 words; 14,738 bytes
- **Resource SHA-256 at audit start:**
  `9c588e546e90779107ab777ad23eac03457866f0cdcc713cc897ba39e39a13a4`
- **Resource-text changes during this audit:** none

## Target output model

This file is a **partial** historical English–Somali industrial and urban-
planning glossary, not an OCR transcript, not a dictionary rewrite, and not
a reconstruction of Mansuur *Ururinta 3aad*. It has one H1, two domain
headings (`## Warshadaha iyo qalabka`, `## Qorshaynta magaalooyinka`), and
461 `left — right` pairs. Industrial heads are lowercase; urban-planning
heads are title case. Blank lines already precede both headings. There is
no cover matter, TOC, exercises, or `## OCR Page` marker.

This collection is a **historical bilingual glossary**. Keep printed
English–Somali pairs. Do not modernize 2014 coinages. Keep the two domain
headings. Fix uniquely supported OCR. Per `00-sources.md`, omit unreadable
OCR and Italian-only records rather than translating Italian into English
or splitting mashed lines into new heads. Do not invent missing industrial
terms. Do not copy terms from cleaned `01`–`08`. Do not expand this partial
extract from a missing full source.

Cleanup should add a one-line collection note linking `00-sources.md` (no
author/year/Mansuur), omit the Italian-only and unreadable rows in EB09-R005,
strip trailing debris in EB09-R006, and repair the uniquely supported OCR
listed below.

## Source and repository evidence

`resources/erey-bixin/00-sources.md` maps this file to Mansuur *Diiwaanka*
Ururinta 3aad (farsamada & culuunta, 2014) and already states that only
verified, readable English–Somali records are kept; unreadable OCR and
Italian-only records are omitted; a usable full source is required before
further curation.

The collection README already labels this file **partial**.

Repository comparison also included:

- same-file `kiimikaad`, `warshadeyn`, `tinnaar`, `qalab`, `degganaansho`,
  `ohmmeter — ohm-beeg`, `vapor pressure`, `sesame oil`, `fusion, melting`,
  `Dodge pulverizer`, `filter cake washing`, `laminar boundary layer`,
  `by-pass valve`;
- dictionary `qabooji`, `dhaqaale`, `dherer`, `warshad`, `gudaha`, `qalab`;
- cleaned `04-kimistari.md` `qaboojinta` and `06` `jaandiga` only as
  do-not-import / negative checks for `Jjaandi`.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-2 | reviewed | EB09-R001 |
| 3-441 (`## Warshadaha iyo qalabka`) | reviewed | EB09-R003, EB09-R004, EB09-R005, EB09-R006, EB09-R007 |
| 443-468 (`## Qorshaynta magaalooyinka`) | reviewed | EB09-R003, EB09-R004 |
| whole file | reviewed | EB09-R002, EB09-R008 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| EB09-R001 | 1-2 | Source H1 is the right title; no collection framing or registry link / low | Cleaned `01`–`08`; attribution and “partial” status live in `00-sources.md` | Keep the printed H1. Add one Somali sentence that this is the industrial and urban-planning glossary in this collection, with a local link to `00-sources.md`. Do not put author, year, Mansuur, or “partial” in the content file. | `repository-supported`; `navigation-update`; `intentional-retained` |
| EB09-R002 | whole file | Compact two-block glossary already: industry then urban planning; blanks present; no A–Z / medium | README domain headings; `00-sources.md` partial extract | Keep both headings and the current within-section order. Keep industrial lowercase and urban title case. Do not A–Z sort. Do not invent missing *Ururinta 3aad* terms. | `structural-only`; `intentional-retained` |
| EB09-R003 | 22;61;163;165-166;184;192;194;226;322;326;344;366;440 | Unique English-head OCR on otherwise readable English–Somali rows / high | Same-file `catalytic cracking`, `coking`, `ohmmeter`, `vapor pressure`, `Dodge pulverizer`; standard heads carburetor, crystallization, feed, flow, extinguisher, Glover, enthalpy, potential, planning, resistance, strength, wettability | On rows that are kept: `area meter ,rotameter` → `area meter, rotameter`; `carburator` → `carburetor`; `entalpy` → `enthalpy`; `eristallization` → `crystallization`; `erystallizer` → `crystallizer`; `fead` → `feed`; `fiow` → `flow`; `estinguisher` → `extinguisher`; `Glower` → `Glover`; `potenzial` → `potential`; `planing` → `planning`; `resistence` → `resistance`; `strenght` → `strength`; `wattability` → `wettability`. Leave US `gage` / `center` as printed. Do not rewrite Italian heads (`combustione`, `craking`) into English. | `repository-supported`; `form-correction` |
| EB09-R004 | 76;104;152;180;214;246;250;253;255;286;290;300;321;382;428;450;463 | Unique Somali OCR against same-file majority or dictionary / high | Same-file `kiimikaad`, `warshadeyn`, `tinnaar`, `qalab`, `degganaansho`; dictionary `qabooji`, `dhaqaale`, `dherer`, `gudaha`, `qasid` | `kimikaad` → `kiimikaad`; `gaboojinta` → `qaboojinta`; `dhagaale` → `dhaqaale`; `warshed` → `warshad`; `tinaar` / `tinnar` / `finnaar` → `tinnaar`; `Jjanjeer` → `janjeer`; `galab` → `qalab`; `guadaha` → `gudaha`; `isku-qgasid` → `isku-qasid`; `Jjajab` → `jajab`; `hawlead` → `hawleed`; `dexalaysan` → `daxalaysan`; `deggenaansha` → `degganaansho`; `dherir` → `dherer`; `tobograafiyeed` → `topograafiyeed`. Do not rewrite `Jjaandi` from chemistry or `06` `jaandi`. | `repository-supported`; `form-correction`; `intentional-retained` |
| EB09-R005 | 23;34;55-56;78;83;90;92;96;105-107;114;118;129;142;150;153;168;191;198;209;212;215;217;267;287;297;331;339;356;359;361-362;371;388;395;398;405-406;421;432-433;435;441 | Italian-only records, unreadable OCR, or OCR-duplicates of a kept English–Somali row / high | `00-sources.md` file-09 omit rule; `01`–`08` omit-mash; same-file `rotameter — wareeg-beeg`, `constituent`, `pulverizer`, `filter cake washing`, `fusion, melting`, `ohmmeter`, `sesame oil`, `vapor pressure`, `turbulence`, `laminar boundary layer`, `by-pass valve` | Omit the Italian-only, unreadable, and OCR-duplicate rows listed in the blueprint, including `combustione furnace` and `craking — craking`. Do not translate Italian heads into English. Do not split mashed lines into new heads. | `omit-unreadable-ocr`; `unresolved` |
| EB09-R006 | 227;242;392 | Readable rows with trailing OCR debris / medium | Physics dropped `(ada@)` and kept the term | Keep `gram-molecular weight — garaam-molekiyuul` (drop trailing `i`); `horizontal crusher — shiiddo jiifeed` (drop trailing `b`); `suction head — joogga soo-nuugidda` (drop trailing `b`; `Jjoogga` → `joogga`). | `repository-supported`; `form-correction` |
| EB09-R007 | remaining Italianate loans; `Jjaandi`; US spelling | Remaining 2014 coinages and mixed `by pass` variants on Somali sides are this extract’s printed forms / low | Cleaned chemistry `Aluuminiyam` and `06` `jaandi` not copied here | Retain `Jjaandi`, `gage`, `center of gravity`, `by-pass` as a Somali variant beside `ka-bootin`, `wirqoobid`, `histogaram`, `kutalgal`, `sheat — qob`. Do not import `01`–`08` Somali. Do not fill gaps from a missing full *Diiwaanka*. | `intentional-retained`; `unresolved` |
| EB09-R008 | whole file | Two-block list format; Italian leftovers are heavier than in `06` / medium | 461 pairs; 2 H2s; partial extract | Keep one concept per line, English first. Apply only the approved omits, debris strip, and unique OCR. Add no new terms. | `structural-only`; `unresolved`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Farsamada iyo Culuunta** and keep
this sequence:

1. one-line collection purpose plus a link to `00-sources.md`;
2. `## Warshadaha iyo qalabka`;
3. `## Qorshaynta magaalooyinka`.

Rows to omit (do not translate or split):

- Italian gloss or Italian-only head: `rotametro`; `barometrico`;
  `by pass — by pass`; `scambiatore refrigerante`; `flocculante`;
  `combustione furnace`; `condensato`; `nucleo`; `costante`;
  `craking — craking`; `defiinizione`; `disincrostazione`; `diffusore`;
  `polvere`; `diffusivita' turbolenta`; `valutazione`; `torcia`;
  `sedimentazione libera`; `calibro`; `aria umida Ei`; `ohmmetro`;
  `carrucola`; `reattivita'`; `aria satura E`; `fenditura`; `sensore`;
  `pendenza`; `vapore acqueo`; `tecnologia`; `diffusione termica`;
  `calibrare`; `comprimere`; `turbolenza`; `valore, costo`;
  `vapor — vapore`; `biancastro`;
- OCR-duplicates of a kept English–Somali row: `by pass valve`;
  `component, costituent`; `Dodge pulverizer`; `filtre cake washing`;
  `fusion, meelting`; `laminar-boundary layer`; `sesame 0il`;
  `vapore pressure`;
- unreadable: `friction loss, friction head — em em me…`.

Expected remaining pairs: 416 (461 minus 45 omits).

No new term should be invented. The partial extract is not completed from
other science files or from Italian.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** EB09-R001 through EB09-R008
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `9c588e546e90779107ab777ad23eac03457866f0cdcc713cc897ba39e39a13a4`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, two domain H2s, 461 em-dash pairs, blank line
  before every heading, mixed Italian leftovers in the industry block, a
  cleaner urban-planning block, no `## OCR Page` marker.

## Cleanup result and review

### Applied cleanup

- Added a one-line collection note linking `00-sources.md`.
- Corrected English heads `area meter, rotameter`, `carburetor`, `enthalpy`,
  `crystallization`, `crystallizer`, `feed tray`, `flow reactor`,
  `fire extinguisher`, `Glover tower`, `potential energy`,
  `process planning`, `resistance furnace`, `shear strength`, and
  `wettability`.
- Corrected Somali `kiimikaad`, `qaboojinta`, `dhaqaale`, `warshad`,
  `tinnaar` (from `tinaar` / `tinnar` / `finnaar`), `janjeer`, `qalab`,
  `gudaha`, `isku-qasid`, `jajab`, `hawleed`, `daxalaysan`,
  `degganaansho`, `dherer`, and `topograafiyeed`.
- Stripped trailing OCR debris on `gram-molecular weight`,
  `horizontal crusher`, and `suction head` (`Jjoogga` → `joogga`).
- Omitted 45 Italian-only, unreadable, or OCR-duplicate rows (461 → 416).

### Deliberately retained

- Both domain headings; industrial lowercase vs urban title case; current
  within-section order; the partial extract was not expanded.
- `Jjaandi`, US `gage` / `center of gravity`, `by-pass` beside `ka-bootin`,
  `wirqoobid`, `histogaram`, `kutalgal`, `sheat — qob`.
- No imported `01`–`08` Somali; Italian heads were not rewritten into
  English.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, two domain H2s, 416 em-dash pairs, blank line before
  every heading.
- Line-count check: 468 + 2 opening − 45 omitted = 425.
- Entry-count check: 461 − 45 = 416.
- Pre-cleanup forms `carburator`, `eristallization`, `Glower`, `gaboojinta`,
  `combustione`, `craking`, `rotametro`, `sesame 0il`, and `Jjoogga` are
  absent.
- Cleaned file size: 425 lines; 1,924 words; 13,537 bytes.
- Cleaned SHA-256:
  `b0964cb0cc5c1aa1237866482e38f22950ad304ce0060219ec2a2e035f2f98f0`.
