# Audit record — Eraybixinta Af Soomaaliga ee Hawlaha Wasaaradaha

- **Resource path:** `resources/erey-bixin/06-wasaaradaha.md`
- **Collection / family:** erey-bixin / historical bilingual administrative glossary
- **Priority:** P2
- **Method:** repository-only, line-by-line glossary audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 4,401 lines; 20,702 words; 134,631 bytes
- **Resource SHA-256 at audit start:**
  `715f1aacf95718b0cd740d80c43895957df51eb3e514c63a3d29c5cec0a68002`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact historical bilingual ministry glossary, not an OCR
transcript, not a dictionary rewrite, and not an A–Z science list. It has one
H1, seventeen `## Wasaaradda …` domain headings, and 4,349 `left — right`
pairs. Blank lines already precede every heading. There is no cover matter,
TOC, exercises, or `## OCR Page` marker.

This collection is a **historical bilingual glossary**. Keep printed term
pairs. Do not modernize 1972 coinages. Keep the domain-heading list format
named in `resources/erey-bixin/README.md`. Fix uniquely supported OCR. Omit
mashed or unreadable rows and editor/page-number apparatus rather than
splitting mashed lines into new heads. Do not invent missing ministries
beyond the `Gaadiidka` heading already present as a mis-parsed row. Do not
copy terms from cleaned `01`–`05`. Do not add a Mansuur *Ururinta 1aad*
block: `00-sources.md` already keeps that overlap once in this file.

Cleanup should add a one-line collection note linking `00-sources.md` (no
author/year), restore `## Wasaaradda Gaadiidka`, repair the uniquely
supported OCR listed below, omit the apparatus and mashed rows in EB06-R005,
and apply the bounded split-head repairs in EB06-R006.

## Source and repository evidence

`resources/erey-bixin/00-sources.md` maps this file to *Eraybixinta Af
Soomaaliga ee Hawlaha Wasaaradaha* (1972). Ministry/government terms from
Mansuur *Ururinta 1aad* overlap almost entirely and are kept once here.

The collection README already names domain headings (`## Wasaaradda …`) as
the section model for this file, as opposed to `## A` … `## Z`.

Repository comparison also included:

- same-file `Wasaaradda`, `iyo`, `Dheerayn`, `qoraal`, `Nolol-aqoon`,
  `Copper — Maar`, `Input`, `Transactions`, `Bacteria — Jeermi`, `Weevils`,
  `Petition`;
- dictionary `wasaarad`, `qoraal`, `matxaf`, `bakteeriya`;
- cleaned `01-bayoolaji.md` `Bacteria — Bakteeriya` only as a do-not-import
  check;
- file `09` Italian-only omit as a negative check (that rule is for the
  partial *Ururinta 3aad* industrial extract, not this 1972 glossary).

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-2 | reviewed | EB06-R001 |
| 3-317 (`## Wasaaradda beeraha`) | reviewed | EB06-R003, EB06-R005 |
| 319-602 (`## Wasaaradda boostada iyo isgaarsiinta`) | reviewed | EB06-R003, EB06-R005, EB06-R007 |
| 604-773 (`## Wasaaradda xoolaha`) | reviewed | EB06-R003 |
| 775-906 (`## Wasaaradda Dibadda`) | reviewed | EB06-R006 |
| 908-1128 (`## Wasaaradda sancada`) | reviewed | EB06-R003, EB06-R006 |
| 1130-2018 (`## Wasaaradda Dhismaha Guud`, including the Gaadiidka block) | reviewed | EB06-R002, EB06-R003, EB06-R005, EB06-R006 |
| 2020-2499 (`## Wasaaradda Gaashaandhigga iyo Booliska`) | reviewed | EB06-R005, EB06-R007 |
| 2501-2623 (`## Wasaaradda Garsoorka iyo Diinta`) | reviewed | EB06-R005, EB06-R007 |
| 2625-2760 (`## Wasaaradda Ganacsiga`) | reviewed | EB06-R007 |
| 2762-3108 (`## Wasaaradda Gudaha`) | reviewed | EB06-R005, EB06-R006 |
| 3110-3370 (`## Wasaaradda Caafimaadka`) | reviewed | EB06-R003, EB06-R007 |
| 3372-3558 (`## Wasaaradda Cayaaraha iyo Shaqaalaha`) | reviewed | EB06-R005 |
| 3560-3820 (`## Wasaaradda Qorshaha iyo Midaynta`) | reviewed | EB06-R006, EB06-R007 |
| 3822-4035 (`## Wasaaradda Lacagta`) | reviewed | EB06-R003, EB06-R005, EB06-R006 |
| 4037-4128 (`## Wasaaradda Macdanta iyo Biyaha`) | reviewed | EB06-R007 |
| 4130-4198 (`## Wasaaradda Waxbarashada iyo Barbaarinta`) | reviewed | EB06-R004, EB06-R006 |
| 4200-4401 (`## Wasaaradda Wararka yo Hanuuninta Dadweynaha`) | reviewed | EB06-R002, EB06-R003 |
| whole file | reviewed | EB06-R002, EB06-R008 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| EB06-R001 | 1-2 | Source H1 is the right title; no collection framing or registry link / low | Cleaned `01`–`05`; attribution lives in `00-sources.md` | Keep the printed H1. Add one Somali sentence that this is the ministry-work glossary in this collection, with a local link to `00-sources.md`. Do not put author, year, or Mansuur in the content file. | `repository-supported`; `navigation-update`; `intentional-retained` |
| EB06-R002 | 1804; 4200; whole file | Compact domain glossary already: 17 ministry H2s, blanks present, no A–Z. Line 1804 is a ministry title parsed as a term pair. Last heading uses `yo` / medium | Same-file `## Wasaaradda …` and `iyo` in six other headings; chemistry restored `## I` because 107 I-heads sat under `H`; dictionary `wasaarad`; README domain-heading model | Convert `Wasaraadda — Gaadiidka` to `## Wasaaradda Gaadiidka` (214 following pairs). Repair `Wararka yo` → `Wararka iyo`. Keep mixed heading capitalisation as printed (`beeraha` vs `Dibadda`). Do not invent other ministries, A–Z sections, or an *Ururinta 1aad* block. Do not drop cross-ministry duplicate English heads. | `structural-only`; `repository-supported`; `intentional-retained` |
| EB06-R003 | 7;12;49;315;343;350;409;414;1021;1822;3037;3237;3289;3846;4374 | Unique English-head OCR on otherwise readable rows / high | Same-file `Copper — Maar`, `Input`, `Weevils`, `Petition`; standard heads Aphids, tick, bulky, bearing, domicile, copper, Paris, lading, secondment, larvicide, suffrage | `ADMINISTRATION` → `Administration`; `Ahids` → `Aphids`; `Cattle tike` → `Cattle tick`; `Wood Borders` → `Wood Borers`; `Bulwy` → `Bulky`; `Capital baring` → `Capital bearing`; `reign` → `foreign` in the air-mail dispatch head; `Domicille` → `Domicile`; `In put` → `Input`; `cooper welding` → `copper welding`; `Secendment (? Leech?)` → `Secondment`; `Plaster of pairs` → `Plaster of Paris`; `Larvacidem` → `Larvicide`; `Bill of landing` → `Bill of lading`; `Sufferage` → `Suffrage`. | `repository-supported`; `form-correction` |
| EB06-R004 | 2836;4143;4167;303 | Unique Somali OCR against same-file or dictionary majority / high | Same-file `Nolol-aqoon`, `Gal qoraal`, `Dheerayn` (three times); dictionary `qoraal`, `matxaf` | `Nalolwarran` → `Nololwarran`; `qoeaal` → `qoraal`; `matfax` → `matxaf`; `Dherayn` → `Dheerayn`. Do not rewrite `Bagteria` from biology `Bakteeriya`. | `repository-supported`; `form-correction` |
| EB06-R005 | 236;306;379;388-389;401;418;498-499;505;517;550;570;577;580;586-587;1802-1803;2101;2495-2499;2534;3101;3507;3848 | Editor apparatus, mashed multi-entry lines, or unreadable fragments / high | File `09` page-number exclusion; `01`–`05` omit-mash rule; same-file `Rice — Bariid, bariis`, `Weevils`, `Vote — book` under Lacagta | Omit the eight starred page/arrow notes (`*angle dozer eliminato…` through `*antipersonale…`). Omit the mashed or fragmentary rows listed in the blueprint below. Do not split them into new heads. Do not realign the rest of the postal section. | `omit-unreadable-ocr`; `structural-only`; `unresolved` |
| EB06-R006 | 799;855;1108-1109;1117;1220;1689;1809-1810;2800;3586;3701;3736;3852-3853;3893;3952;4135 | English compound split at the emdash, with the rest of the English sitting on the Somali side / medium | Same-file `To Switch off`, `Stainless steel welding`, `Input`, `Cashier` | Join only the listed compounds (`Charge d'Affaires`, `National Day`, `To excite` / `To feed` / `To sharpen`, `Cam shaft`, `Stainless steel`, `Air cooling`, `Air-cooled arc welder` with `are` → `arc`, `Cash book`, `Capital Outlay`, `Capital/Ordinary/Development Budget`, `Maritime Traffic`, `Art and Craft`). Leave other `X — fragment` rows unresolved. | `repository-supported`; `form-correction`; `unresolved` |
| EB06-R007 | Italian sections; 1972 coinages; remaining postal shift | Italian heads, source coinages, and remaining damaged pairs are this file’s printed forms / low | File `09` omits Italian-only *Ururinta 3aad* records; that rule does not apply here; cleaned biology `Bakteeriya` not copied | Retain Italian heads in Gaashaandhigga, Garsoorka, and Gudaha (`Esplosivo`, `Giudice`, `Ufficio`, `Viticoltura`, `Bilancio`, `Indennità`). Retain `Bagteria`, `Kombyuutar`, `Nolol-aqoon`, `Ifilo`, `Cypher`, `Base Meriod (?)`, `Transection`, `Keal`-style debris, and the remaining postal misalignment. Do not import `01`–`05` Somali. | `intentional-retained`; `unresolved` |
| EB06-R008 | whole file | Domain-heading list format; OCR is uneven (postal worst; agriculture cleaner) / medium | 4,349 pairs; 17 H2s; 214 pairs after the Gaadiidka row; 305 cross-ministry duplicate English heads | Keep one concept per line, source-language term first (English or printed Italian). Apply only the approved heading restore, unique OCR, split-head joins, and omits. Add no new terms. | `structural-only`; `unresolved`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Eraybixinta Af Soomaaliga ee Hawlaha
Wasaaradaha** and keep this sequence:

1. one-line collection purpose plus a link to `00-sources.md`;
2. the current ministry headings, with `## Wasaaradda Gaadiidka` inserted
   where `Wasaraadda — Gaadiidka` now sits, and `iyo` in the Wararka heading;
3. one `left — right` line per concept, in the current within-section order.

Mashed rows to omit (do not split or realign):

- agriculture: `Rice — Weevil …`; `Vote — Book Buugga baaqi tuska`;
- postal fragments and multi-entry mashes at 379, 388–389, 401, 418, 498–499,
  505, 517, 550, 570, 577, 580, 586–587;
- Dhismaha notes at 1802–1803;
- Gaashaandhigga notes at 2101 and 2495–2499;
- `Infermità — Waalli lngiuria Cay`;
- `Vote — book partitionis entrata uscita …`;
- `Spooning sticky-rests in hand – — scooping …`;
- `Budget classification Laòa gurka — miisaaniyadda`.

No new term should be invented. Source coinages stay as printed.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** EB06-R001 through EB06-R008
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `715f1aacf95718b0cd740d80c43895957df51eb3e514c63a3d29c5cec0a68002`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, seventeen domain H2s, 4,349 em-dash pairs, blank
  line before every heading, eight starred apparatus lines, `Wasaraadda —
  Gaadiidka` sitting as a pair with 214 following transport/welding rows, no
  `## OCR Page` marker.

## Cleanup result and review

### Applied cleanup

- Added a one-line collection note linking `00-sources.md`.
- Converted `Wasaraadda — Gaadiidka` to `## Wasaaradda Gaadiidka` (214 pairs).
- Repaired `Wararka yo` → `Wararka iyo`.
- Corrected English heads `Administration`, `Aphids`, `Cattle tick`,
  `Wood Borers`, `Bulky`, `Capital bearing`, `foreign` (air-mail dispatch),
  `Domicile`, `Input`, `copper welding`, `Secondment`, `Plaster of Paris`,
  `Larvicide`, `Bill of lading`, and `Suffrage`.
- Corrected Somali `Nololwarran`, `qoraal`, `matxaf`, and `Dheerayn`.
- Joined the listed split English compounds, including `Charge d'Affaires`,
  `National Day`, `To excite` / `To feed` / `To sharpen`, `Cam shaft`,
  `Stainless steel`, `Air-cooled arc welder`, `Air cooling`, `Cash book`,
  budget/outlay compounds, `Maritime Traffic`, and `Art and Craft`.
- Omitted twenty-nine mashed, fragmentary, or starred apparatus rows.

### Deliberately retained

- Mixed heading capitalisation (`beeraha` vs `Dibadda`); no extra ministries;
  no A–Z; no *Ururinta 1aad* block.
- Italian heads (`Esplosivo`, `Giudice`, `Ufficio`, `Viticoltura`, `Bilancio`,
  `Indennità`).
- `Bagteria`, `Kombyuutar`, `Nolol-aqoon`, `Ifilo`, `Cypher`,
  `Base Meriod (?)`, `Transection`, and remaining postal misalignment.
- Cross-ministry duplicate English heads. No imported `01`–`05` Somali.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, eighteen domain H2s (Gaadiidka restored), 4,319 em-dash
  pairs, blank line before and after every heading, no starred apparatus.
- Line-count check: 4,401 + 2 opening + 2 Gaadiidka blanks − 29 omits = 4,376.
- Entry-count check: 4,349 − 29 omits − 1 converted heading = 4,319.
- Pre-cleanup forms `Ahids`, `Cattle tike`, `Wood Borders`, `ADMINISTRATION`,
  `Bulwy`, `Domicille`, `Secendment`, `Plaster of pairs`, `Nalolwarran`,
  `qoeaal`, `matfax`, `Wasaraadda`, and `Wararka yo` are absent.
- Cleaned file size: 4,376 lines; 20,366 words; 132,388 bytes.
- Cleaned SHA-256:
  `24393dab563a094ad0938b3facabcd11691ff03d892be649a9903a0ad1a064b2`.
