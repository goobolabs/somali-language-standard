# Audit record — Sources: sarfe

- **Resource path:** `resources/sarfe/00-sources.md`
- **Collection / family:** sarfe / source registry
- **Priority:** P3
- **Method:** repository-only, row-by-row metadata audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 70 lines; 598 words; 3,179 bytes
- **Resource SHA-256 at audit start:**
  `5b4f21e3ef77b43f828c5814475ce57d0b2e0b9d67b97c527f931e5e5d6a843a`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact collection registry, not a paradigm chapter. It has one
H1, four H2 headings, four valid Markdown tables, three coverage-note bullets,
and one Markdown link to `docs/RESOURCES.md`. Every mapped content file exists.

Cleanup should keep S-003 as the only excerpted source, keep the bibliographic
cross-checks unexcerpted, and keep the R5.7 table-to-naxwe map. It should use
the cleaned naxwe author/year form, turn file references into local links, and
replace the stale Phase 5 note with the post-naxwe-cleanup status: the tables
were extracted in July 2026 and still need the alignment repairs recorded in
M01–M04.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-3 | reviewed | M00S-R001 |
| 5-12 | reviewed | M00S-R002 |
| 14-23 | reviewed | M00S-R003 |
| 25-33 | reviewed | M00S-R004 |
| 35-63 | reviewed | M00S-R005 |
| 65-70 | reviewed | M00S-R006 |
| whole file | reviewed | M00S-R007 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| M00S-R001 | 1-3 | Clear English registry title and purpose matching sibling `00-sources.md` files / low | `resources/naxwe/00-sources.md` and other collection registries keep attribution here so content files stay clean | Retain the title and one-sentence purpose. Do not translate only this registry. | `repository-supported`; `intentional-retained` |
| M00S-R002 | 5-12 | Primary row correctly dates S-003 to 1999, but authors are abbreviated `Mansur; Puglielli` and the rights cell is informal / medium | Cleaned `naxwe/00-sources.md`: *Barashada Naxwaha Af Soomaaliga — A Somali School Grammar*, Abdalla Omar Mansur; Annarita Puglielli, 1999; N00S later confirmed 1999 from the PDF title page | Retain S-003, 1999, primary status, and the naxwe-extraction note. Expand the author cell to the cleaned full names. Keep rights as a compact in-repo limitation rather than a new licence claim. | `repository-supported`; `metadata-clarification`; `intentional-retained` |
| M00S-R003 | 14-23 | Four bibliographic cross-checks are correctly marked unexcerpted; permission status is still unconfirmed / low | `docs/RESOURCES.md` morphology section; the file already forbids OCR of those volumes | Retain all four rows and the RESOURCES link. Do not add Kirk or other archive-only works to this table. | `repository-supported`; `intentional-retained` |
| M00S-R004 | 25-33 | File map is complete and accurate, but the naxwe paths are not clickable / low | Files `01`–`04` exist; `04` also cites `dhawaaq/03` | Retain the four rows. Convert naxwe (and phonology, for file `04`) targets to local links. Do not add files. | `repository-supported`; `navigation-update` |
| M00S-R005 | 35-63 | R5.7 map remains a useful table-to-section index; paths are unlinked / low | M01–M04 confirm the listed table sets still exist in the content files | Retain every R5.7 row. Link the naxwe chapter files. Do not add or delete table sets during registry cleanup. | `repository-supported`; `intentional-retained`; `navigation-update` |
| M00S-R006 | 65-70 | Coverage note still reads as completed Phase 5 curation from 2026-07-18, although naxwe 02/07/08 were cleaned later and M01–M04 found leftover pre-cleanup forms / high | Naxwe cleanup 2026-08-12; this audit 2026-08-19 found `cunlahaa`, `qashay`, and `aqaanaa`/`aqiin` still in morphology | Replace the “completed” wording with extraction date plus the current alignment-audit status. Link the file-review folder rather than hard-coding a transient approval count. Retain the Saeed/Green permission warning and the Kirk archive-only note. | `repository-supported`; `status-correction` |
| M00S-R007 | whole file | Valid registry with no extra bibliography, but most internal paths are inert and the status note is stale / medium | Four tables, all mapped files present | Preserve the four-section sequence. Add no new source row and no inferred permission. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned registry should retain:

1. title and attribution-once purpose;
2. S-003 primary row with full cleaned author names and 1999;
3. four unexcerpted bibliographic cross-checks;
4. linked file map;
5. linked R5.7 table; and
6. coverage notes that distinguish July 2026 extraction from the August 2026
   naxwe cleanup and this alignment audit.

No new book should be added. No cross-check should be marked excerpted.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** M00S-R001 through M00S-R007
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `5b4f21e3ef77b43f828c5814475ce57d0b2e0b9d67b97c527f931e5e5d6a843a`
  (unchanged).
- Resource diff during this audit: none.

## Cleanup result and review

### Applied cleanup

- Expanded the S-003 author cell to Abdalla Omar Mansur; Annarita Puglielli.
- Linked the file map and R5.7 naxwe/phonology paths.
- Replaced the stale Phase 5 completion note with extraction date, naxwe
  cleanup date, and a link to the morphology file-review folder.

### Deliberately retained

- S-003 as the only excerpted source, 1999, and all four unexcerpted
  cross-checks.
- Every R5.7 table set.
- The Saeed/Green permission warning and the Kirk archive-only note.

### Cleanup validation

- `git diff --check`: passed.
- Four tables remain; no new source row was added.
- Cleaned file size: 73 lines; 621 words; 4,669 bytes.
- Cleaned SHA-256:
  `c31d9598e83405e8b2a9abc9e416ef91551604246e190587f329577cf0e72a12`.
