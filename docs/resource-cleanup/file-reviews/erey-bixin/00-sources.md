# Audit record — Sources: erey-bixin

- **Resource path:** `resources/erey-bixin/00-sources.md`
- **Collection / family:** erey-bixin / source registry
- **Priority:** P3
- **Method:** repository-only, row-by-row metadata audit
- **Audit status:** approved; registry cleanup applied and awaiting
  maintainer cleanup review
- **Audit started:** 2026-08-19
- **File size at audit start:** 50 lines; 284 words; 2,010 bytes
- **Resource SHA-256 at audit start:**
  `ab4bfa2db7e1e581e2a55aaad8393c48f692856f6fb4b74b054cfb82bfce36f2`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact collection registry, not a glossary. It has one H1,
four H2 headings, four valid Markdown tables (nine file rows), a 1987 series
publisher line, a Mansuur *Ururinta 2aad* paragraph under mathematics, and two
coverage-note bullets. Every mapped content file exists. File cells are
backticked paths, not links.

Cleanup should keep every existing source row, year, and title, keep
attribution here so content files stay clean, turn file references into local
links, correct the stale *Ururinta 2aad* duplicate-head claim against the
cleaned `05-xisaab.md` structure, and replace silent “inventory only” coverage
with extraction versus the 2026-08-19 cleanup-audit status. It must not add
authors, years, or books that are not already in this registry, and must not
rewrite the 2014 *Diiwaanka* compiler as the 1999 grammar author form.

## Source and repository evidence

Every table title matches the cleaned content-file H1. Sibling registries
(`dhawaaq/00-sources.md`, `qoraal/00-sources.md`,
`sarfe/00-sources.md`) keep English framing, link file cells, and
distinguish extraction from cleanup-audit status.

Repository comparison also included:

- cleaned `05-xisaab.md`: 1987 A–Z, `## Ururinta 2aad (Cilmiga)`,
  `## Sumadaha xisaabta`; cross-block `Deviation` kept in both lists;
- cleaned `06-wasaaradaha.md`: 1972 ministry glossary kept once; no Mansuur
  *Ururinta 1aad* block added;
- cleaned `09-farsamada-culuunta.md`: Italian-only and unreadable rows omitted;
  extract remains partial;
- cleaned naxwe/morphology registries: *Barashada Naxwaha* author cell is
  Abdalla Omar Mansur; that is a different book from the 2014 *Diiwaanka*.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-4 | reviewed | EB00S-R001 |
| 6-18 | reviewed | EB00S-R002 |
| 19-22 | reviewed | EB00S-R003 |
| 24-41 | reviewed | EB00S-R004 |
| 43-50 | reviewed | EB00S-R005 |
| whole file | reviewed | EB00S-R006 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| EB00S-R001 | 1-4 | Clear English registry title and attribution-once purpose / low | Sibling `00-sources.md` files; cleaned sarfe/qoraal/phonology registries kept English | Retain the title and one-sentence purpose. Do not translate only this registry. Do not move author/year into content files. | `repository-supported`; `intentional-retained` |
| EB00S-R002 | 6-18 | 1987 science table titles, years, and Lafoole-era publisher match cleaned files `01`–`05`; paths are inert; no per-book author is present / low | Cleaned H1s `Qaamuuska eray-bixinta ee …`; files exist | Retain all five rows, 1987, and the series publisher. Link each file cell. Do not invent authors for the 1987 series. | `repository-supported`; `navigation-update`; `intentional-retained`; `unresolved` |
| EB00S-R003 | 19-22 | *Ururinta 2aad* is correctly attached to `05-xisaab.md`, but “Duplicate English heads are not repeated” is false for the cleaned file / high | Cleaned `05`: Ururinta and Sumadaha are separate blocks; `Deviation` exists in both the 1987 list and Ururinta; EB05 retained both | Keep compiler `Cabdalla Cumar Mansuur`, 2014, Akademiye-Goboleedka / Centro Studi Somali. State that Ururinta 2aad science terms and the symbol table are appended as separate blocks, not folded into A–Z. Do not claim duplicate English heads were dropped. Do not rewrite the compiler as Abdalla Omar Mansur. | `repository-supported`; `status-correction`; `intentional-retained` |
| EB00S-R004 | 24-41 | Administrative, plant, and industrial rows match cleaned H1s and years; paths are inert; 1972/1984/1985 authors are absent / low | Cleaned `06`–`09` H1s; `09` remains partial | Retain all four rows and years. Link each file cell. Keep the 09 source as Mansuur *Diiwaanka* Ururinta 3aad (2014). Do not invent missing authors. Do not add a second industrial source. | `repository-supported`; `navigation-update`; `intentional-retained`; `unresolved` |
| EB00S-R005 | 43-50 | Coverage notes are still right on Ururinta 1aad overlap and the 09 Italian-only omit rule, but they do not record that files `01`–`09` now have an applied 2026-08-19 cleanup audit / high | EB01–EB09 cleanup records; sibling registries link `docs/resource-cleanup/file-reviews/…` | Retain both coverage bullets. Add that files `01`–`09` received the 2026-08-19 cleanup audit (awaiting cleanup review) and that this is not full scan verification. Link the erey-bixin file-review folder. Keep the 09 need for a usable full source. Do not mark the collection complete. | `repository-supported`; `status-correction`; `intentional-retained` |
| EB00S-R006 | whole file | Valid four-section registry with no extra bibliography / medium | Four tables; nine mapped files present | Preserve the science / admin / plants / industrial sequence. Add no new source row and no inferred author or year. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned registry should retain:

1. title and attribution-once purpose;
2. linked 1987 science table plus the series publisher;
3. a corrected *Ururinta 2aad* note that the science terms and symbol table
   are separate blocks in `05-xisaab.md`;
4. linked 1972 / 1984 / 1985 / 2014 rows;
5. coverage notes that keep the Ururinta 1aad once-only rule and the 09
   Italian-only omit rule, plus extraction versus this cleanup audit.

No new book should be added. Unresolved authors stay unresolved. The 2014
compiler spelling in this registry stays `Cabdalla Cumar Mansuur`.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** EB00S-R001 through EB00S-R006
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Audit validation

- Resource SHA-256 after audit:
  `ab4bfa2db7e1e581e2a55aaad8393c48f692856f6fb4b74b054cfb82bfce36f2`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, four H2s, four tables, nine file rows, all
  mapped files present, no Markdown links.

## Cleanup result and review

### Applied cleanup

- Converted all nine file-table cells, plus the inline `05`, `06`, and `09`
  mentions, to local links.
- Restated *Ururinta 2aad* science terms and the symbol table as separate
  blocks in `05-xisaab.md`; duplicate English heads across those blocks are
  kept.
- Added the 2026-08-19 cleanup-audit status, a link to
  `docs/resource-cleanup/file-reviews/erey-bixin/`, and the scan-verification
  limitation.

### Deliberately retained

- English attribution-once title and purpose.
- All nine source rows, years, titles, and the 1987 series publisher.
- Compiler `Cabdalla Cumar Mansuur` for the 2014 *Diiwaanka*.
- Ururinta 1aad once-only rule and the 09 Italian-only omit rule.
- Unresolved authors for the 1987 series and for 1972 / 1984 / 1985.
- No new source row.

### Cleanup validation

- `git diff --check`: passed.
- Four tables remain; nine linked file rows; no new source row.
- Pre-cleanup wording `Duplicate English heads are not repeated` is absent.
- Cleaned file size: 57 lines; 329 words; 2,701 bytes.
- Cleaned SHA-256:
  `52b53d1d8111b17759b0d561f4427449cb38b9d94e2f5e899bb4c7c66060a839`.
