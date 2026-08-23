# Audit record — Sources: qoraal

- **Resource path:** `resources/qoraal/00-sources.md`
- **Collection / family:** qoraal / source registry
- **Priority:** P3
- **Method:** repository-only, row-by-row metadata audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 42 lines; 259 words; 1,828 bytes
- **Resource SHA-256 at audit start:**
  `afbdeb4ac2742ca76981ca973f5bb527d610b85d38249d29125028e49637edb1`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact collection registry, not a punctuation chapter. It has
one H1, four H2 headings, three valid Markdown tables, and four coverage-note
bullets. Every mapped content file exists.

Cleanup should keep *Habka Qoraalka* as the only primary excerpted source for
files `01`–`05`, keep Nilsson as the capitalization supplement, and keep the
dhawaaq/alphabet exclusions. It should turn file references into local
links and replace the stale “manually verified” completion wording with the
August 2026 cleanup-audit status. It must not add a Somali-primary
capitalization book or mark SLS-0005 as drafted.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-3 | reviewed | O00S-R001 |
| 5-12 | reviewed | O00S-R002 |
| 14-23 | reviewed | O00S-R003 |
| 25-29 | reviewed | O00S-R004 |
| 31-42 | reviewed | O00S-R005 |
| whole file | reviewed | O00S-R006 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| O00S-R001 | 1-3 | Clear English registry title and attribution-once purpose / low | Sibling `00-sources.md` files; cleaned morphology registry kept English | Retain the title and one-sentence purpose. Do not translate only this registry. | `repository-supported`; `intentional-retained` |
| O00S-R002 | 5-12 | Primary row correctly names Raabi 1977 and the Lafoole publisher; the file range is not linked / low | Cleaned `naxwe/00-sources.md` file 14 author Maxamed Xaaji Xuseen Raabi; files `01`–`05` exist | Retain title, author, year, and publisher. Link `01`–`05`. Do not add a year-range or a second primary book. | `repository-supported`; `navigation-update`; `intentional-retained` |
| O00S-R003 | 14-23 | File map matches the six content files and the 1977 section labels; paths are inert / low | Cleaned files `01`–`04`; this audit of `05`–`06` | Retain all six rows. Convert file cells to local links. Do not retitle Nilsson as a 1977 section. | `repository-supported`; `navigation-update`; `intentional-retained` |
| O00S-R004 | 25-29 | Nilsson 2024 preliminary row is the correct interim capitalization source / low | File `06` opening; `spec/orthography/0003-capitalization.md` is absent | Retain the supplement row. Link `06-xarafka-weyn.md`. Do not upgrade Nilsson to primary. | `repository-supported`; `intentional-retained` |
| O00S-R005 | 31-42 | Coverage notes are right on phonology and alphabet boundaries, but “manually verified” reads as finished curation while `01`–`04` are at cleanup review and `05`–`06` are in this audit / high | Orthography file-reviews; 1974 Ministry sampling note in file `06`; `naxwe/01` alphabet overlap | Retain the phonology and alphabet exclusions and the 1974 sampling claim. Replace completion wording with extraction versus August 2026 cleanup-audit status and a link to the file-review folder. Do not add an inferred Somali capitalization source. | `repository-supported`; `status-correction`; `intentional-retained` |
| O00S-R006 | whole file | Valid registry with no extra bibliography / medium | Three tables, all mapped files present | Preserve the four-section sequence. Add no new source row. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned registry should retain:

1. title and attribution-once purpose;
2. linked Raabi 1977 primary row for `01`–`05`;
3. linked six-row file map;
4. linked Nilsson supplement row; and
5. coverage notes that distinguish scan verification from this cleanup audit.

No new book should be added. SLS-0005 should stay planned.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** O00S-R001 through O00S-R006
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `afbdeb4ac2742ca76981ca973f5bb527d610b85d38249d29125028e49637edb1`
  (unchanged).
- Resource diff during this audit: none.

## Cleanup result and review

### Applied cleanup

- Linked files `01`–`05` in the primary row and converted every file-map cell
  to a local link.
- Linked `06-xarafka-weyn.md` in the Nilsson supplement row.
- Replaced “manually verified / finished” coverage wording with extraction
  versus the 2026-08-19 cleanup-audit status and a link to
  `docs/resource-cleanup/file-reviews/qoraal/`.
- Recorded that planned spec `SLS-0005` is not yet drafted.

### Deliberately retained

- Raabi 1977 as the only primary for files `01`–`05`.
- Nilsson as a capitalization supplement, not a primary.
- Phonology and alphabet exclusions.
- The 1974 Ministry sampling claim.

### Cleanup validation

- `git diff --check`: passed.
- Three tables remain; no new source row was added.
- Cleaned file size: 44 lines; 269 words; 2,316 bytes.
- Cleaned SHA-256:
  `be2c1baf4994a23707a25d4953bf8d9fe0813e1e34b95d43d9baef03e8d5d167`.
