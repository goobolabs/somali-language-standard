# Audit record — Sarfe README

- **Resource path:** `resources/sarfe/README.md`
- **Collection / family:** sarfe / collection map and conventions
- **Priority:** P3
- **Method:** repository-only, line-by-line documentation audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 55 lines; 232 words; 1,823 bytes
- **Resource SHA-256 at audit start:**
  `78ec2bfc344acf698a1f58cb09e39493e8fcebe863250cc2fa0eb2650a96c6a6`
- **Resource-text changes during this audit:** none

## Target output model

This is a compact collection README, not a paradigm file. It has one H1, five
H2 headings, one fenced layout block, include/exclude lists, a three-row
boundary table, and local links to `naxwe/`. Its file map is complete.

Cleanup should keep the tabular-only charter and every mapped file. It should
identify `resources/` as descriptive evidence rather than a normative layer,
replace the inert layout fence with a linked table, and stop dating the
collection as finished on 2026-07-18 while M01–M04 still await cleanup.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-10 | reviewed | MRD-R001 |
| 12-21 | reviewed | MRD-R002 |
| 23-36 | reviewed | MRD-R003 |
| 38-44 | reviewed | MRD-R004 |
| 46-49 | reviewed | MRD-R005 |
| 51-55 | reviewed | MRD-R006 |
| whole file | reviewed | MRD-R007 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| MRD-R001 | 1-10 | Useful non-normative charter, but “Status (2026-07-18)” reads as finished curation and does not mention the later naxwe cleanup or this alignment audit / high | `docs/RESOURCES.md` and `docs/ARCHITECTURE.md` treat `resources/` as descriptive evidence and `spec/` as later norms; naxwe README cleanup made the same evidence/norm boundary explicit; M01–M04 are unaudited-cleanup as of this record | Retain the paradigm-table purpose and the statement that this collection does not define normative forms. Replace the dated status line with a descriptive-evidence note and a link to `docs/RESOURCES.md`. Do not hard-code a completion count. | `repository-supported`; `status-correction`; `scope-correction` |
| MRD-R002 | 12-21 | Layout lists every file correctly, but the code fence is non-navigable / low | All six collection files exist; cleaned `resources/naxwe/README.md` converted the same kind of fence into a linked table | Preserve all six entries. Convert the fence to a compact Markdown table with local links. | `repository-supported`; `navigation-update`; `intentional-retained` |
| MRD-R003 | 23-36 | Include/exclude charter matches the actual files and the naxwe-versus-morphology boundary / low | Content files are tables; pedagogical prose lives in `naxwe/02` and `07`–`08`; no Saeed/Green OCR is present | Retain every include and exclude bullet. | `repository-supported`; `intentional-retained` |
| MRD-R004 | 38-44 | Neighbour-boundary table is correct and already links `naxwe/` / low | `docs/RESOURCES.md` cross-collection table uses the same split | Retain the three rows. Add that morphology tables must stay aligned with the cleaned naxwe chapters rather than silently preserving pre-cleanup forms. | `repository-supported`; `scope-correction`; `intentional-retained` |
| MRD-R005 | 46-49 | Intended-use bullets are accurate and do not claim a finished `data/` morphology layer / low | `docs/RESOURCES.md` defers structured morphology to later `data/` work | Retain both bullets. | `repository-supported`; `intentional-retained` |
| MRD-R006 | 51-55 | Conventions are correct; “Somali-first” should not be read as banning the one English H1 gloss in file `04` / low | M04-R001 keeps `morphophonology` as a first-use gloss | Retain UTF-8, filenames, and table format. Scope Somali-first to content prose, allowing a first-use English gloss. | `repository-supported`; `scope-correction`; `intentional-retained` |
| MRD-R007 | whole file | Complete map, no OCR, no extra files; needs navigation and a non-stale status line / medium | Six collection files; file-review records now exist for this audit | Preserve one H1 and the charter/boundary/use/convention sections. Add links to the source registry and file-review folder without marking the collection complete. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned README should retain:

1. title and tabular-paradigm purpose;
2. descriptive-evidence versus normative-specification note;
3. a linked layout table of all six files;
4. the include/exclude charter;
5. neighbour boundaries, including naxwe alignment;
6. intended use; and
7. scoped Somali-first conventions.

No mapped file or charter topic should be removed. No transient approval count
or unverified completeness claim should be added.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** MRD-R001 through MRD-R007
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `78ec2bfc344acf698a1f58cb09e39493e8fcebe863250cc2fa0eb2650a96c6a6`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, five H2 headings, one fenced layout block, one
  boundary table, existing naxwe links, no OCR page markers.

## Cleanup result and review

### Applied cleanup

- Replaced the dated status line with a descriptive-evidence versus
  normative-specification note linked to `docs/RESOURCES.md` and `spec/`.
- Converted the layout fence into a linked table of all six files.
- Added that morphology tables must stay aligned with cleaned naxwe.
- Scoped Somali-first prose to allow a first-use English gloss.

### Deliberately retained

- The tabular-only charter and every include/exclude bullet.
- Intended-use bullets and the three neighbour collections.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, five H2 headings, one linked layout table, one boundary
  table.
- Cleaned file size: 56 lines; 284 words; 2,222 bytes.
- Cleaned SHA-256:
  `77ece648df124676c4e27c26535d8eba520b6c02757f430017e5ed11c76456da`.
