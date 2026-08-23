# Audit record — Abbreviations: qaamuus

- **Resource path:** `resources/qaamuus/00-abbreviations.md`
- **Collection / family:** qaamuus / grammatical and domain code key
- **Priority:** P2
- **Method:** repository-only, table-row audit with entry-tag cross-check
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 79 lines; 696 words; 3,273 bytes
- **Resource SHA-256 at audit start:**
  `3a22f7f8d2a99e63e476c552a1dbf1cf42fc5f805597fd3d2b12695f5537216f`
- **Resource-text changes during this audit:** none

## Target output model

This file is the trilingual decode key for dictionary entry codes, not a letter
file. It has one H1, one four-column Markdown table (**74** abbreviation rows),
and one closing Somali note on compound codes (`m.dh`, `f.mg1`, …).

Cleanup should preserve every decodable row and the compound-code note. It should
repair only clearly broken table cells where columns are merged or empty without
changing the underlying abbreviations used in entries. It must not normalize
entry-level domain tags in letter files during registry cleanup, and must not
delete duplicate technology rows (`tegno.`, `to.`) if both appear in source
entries.

## Source and repository evidence

Sample entries in `01-b.md` use parenthetical domain tags (`(nax.)`, `(daaw.)`,
`(dii.)`), bare compound grammatical codes (`m.l`, `f.mg1`, `qr.dd`), and
cross-reference tokens (`ld`, `eeg`) defined in usage rather than in this table.

Repository cross-check:

- Top-frequency domain tags in letter files (`daaw.`, `fiis.`, `xis.`, `c.nafl.`,
  `baay.`, `kiim.`, `dii.`, `juqr.`) map to table keys, usually with the entry
  tag carrying a trailing period inside parentheses while the table key may omit
  it (`xis` vs `(xis.)`).
- `(n.k.h.a.)` appears in religious honorific contexts (e.g. `17-m.md` *mawliid*);
  it is not a domain abbreviation and should not be forced into this table.
- Rare entry tags **`(baai.)`** (1×, `01-b.md` *bakteeriafayj*) and
  **`(c.daaw.)`** (1×, `01-b.md` *beeryaro²*) look like source/OCR variants of
  `baay.` and `daaw.`; letter-file correction deferred to the `01-b` audit, not
  silent registry rewrites.
- Table row **`maan. maanso`** merges two abbreviations into one key with empty
  Italian/English columns and garbled gloss text in the Somali column.
- Table row **`wr`** leaves the English column empty; Italian and English gloss
  text appear merged in the Italian cell (`frase sentence`).

Cleaned top-level [`resources/README.md`](../../../../resources/README.md) already
links this file as the decode target for qaamuus entry format.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1 | reviewed | QA00A-R001 |
| 3-77 | reviewed | QA00A-R002 |
| 49 | reviewed | QA00A-R003 |
| 74 | reviewed | QA00A-R004 |
| 66-67 | reviewed | QA00A-R005 |
| 79 | reviewed | QA00A-R006 |
| whole file | reviewed | QA00A-R007 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| QA00A-R001 | 1 | Single H1 with no purpose line; sibling collection meta files sometimes add one sentence / low | `00-sources.md` carries attribution; top-level README links here for code decode | Retain `# Abbreviations`. Optionally add one English sentence that this table decodes codes used in `01-b` … `31-uu.md`. Do not move the full entry-format example here — link [`README.md`](../../../../resources/qaamuus/README.md) or top-level README instead. | `repository-supported`; `navigation-update`; `intentional-retained` |
| QA00A-R002 | 3-77 | Core code inventory is usable and matches entry usage for high-frequency tags; four-column layout is consistent except known broken rows / medium | Entry samples in `01-b.md`, `17-m.md`; naxwe cleanup cites this file for `tiraale` / `jagaale` alignment | Retain all 74 rows and the Somali / Italian / English columns. Repair only the broken rows identified in QA00A-R003 and QA00A-R004. Do not rename keys that appear in live entries unless a letter-file audit approves the matching entry change. | `repository-supported`; `intentional-retained` |
| QA00A-R003 | 49 | **`maan. maanso`** row is malformed: one key cell, garbled Somali/Italian merge, empty Italian and English columns / medium | Row reads `maan. maanso \| poesia/poetico poetry/poetic \| \|` | Split or restate as source-faithful rows only if the printed dictionary distinguishes `maan.` and `maanso`; otherwise retain the combined key but repair column alignment and separate Italian (`poesia/poetico`) from English (`poetry/poetic`). Do not invent a third gloss. | `repository-supported`; `table-repair`; `intentional-retained`; `unresolved` |
| QA00A-R004 | 74 | **`wr`** row missing English column; gloss `frase sentence` is split across columns / low | Row reads `wr \| weer \| frase sentence \|` | Move `sentence` into the English column and keep `frase` in Italian. Retain abbreviation `wr` and Somali `weer`. | `repository-supported`; `table-repair`; `intentional-retained` |
| QA00A-R005 | 66-67 | **`tegno.`** and **`to.`** both gloss technology with duplicate Italian `tecnologia` / low | Both keys present once each in the table | Retain both rows as printed variants. Do not collapse to one key during registry cleanup. | `repository-supported`; `intentional-retained` |
| QA00A-R006 | 79 | Closing Somali compound-code note correctly explains chained codes (`m.dh`, `f.mg1`, `m.l.u.kh`, `.iwm`) / low | Entry lines throughout `01-b.md` use chained codes exactly in this pattern | Retain the note verbatim except for obvious OCR repair if ever supported. Link from collection README during README cleanup. | `repository-supported`; `intentional-retained` |
| QA00A-R007 | whole file | Valid single-table key with no navigation links and no cleanup-audit status / medium | Referenced from top-level README and future letter-file audits | Preserve one H1, one table, one closing note. Add a link to [`docs/resource-cleanup/file-reviews/qaamuus/`](./) in cleanup only. Do not add codes absent from source evidence. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned abbreviations file should retain:

1. title (plus optional one-line purpose);
2. the full four-column table with repaired **`maan. maanso`** and **`wr`**
   cells;
3. both technology rows **`tegno.`** and **`to.`**;
4. the Somali compound-code closing note.

Entry-level tag variants (`xis.` vs `xis`, `baai.`, `c.daaw.`) belong in letter-file
audits, not silent table expansion without source proof.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** QA00A-R001 through QA00A-R007
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `3a22f7f8d2a99e63e476c552a1dbf1cf42fc5f805597fd3d2b12695f5537216f`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, one table (74 body rows), one closing paragraph, no
  links.

## Cleanup result and review

### Applied cleanup

- Added a two-sentence English purpose with links to letter files, README, and
  top-level entry-format section.
- Repaired **`maan. maanso`** column alignment (`maanso` / `poesia/poetico` /
  `poetry/poetic`).
- Repaired **`wr`** row (`frase` / `sentence`).
- Retained **`tegno.`** and **`to.`** as separate technology rows.
- Retained the Somali compound-code closing note verbatim.

### Deliberately retained

- All 74 abbreviation keys and the four-column layout.
- No entry-level tag changes (`baai.`, `c.daaw.`) — deferred to letter-file
  audits.
- No new codes added without source evidence.

### Cleanup validation

- `git diff --check`: passed.
- One purpose paragraph, one table (74 body rows), one closing note.
- Cleaned file size: 85 lines; 727 words; 3,562 bytes.
- Cleaned SHA-256:
  `92ae979c806b56d32b792c3e0e145dd4fa430d307e19003eff5290e781f5e107`.
