# Audit record — Qoraal README

- **Resource path:** `resources/qoraal/README.md`
- **Collection / family:** qoraal / collection map and conventions
- **Priority:** P3
- **Method:** repository-only, line-by-line documentation audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 67 lines; 360 words; 2,787 bytes
- **Resource SHA-256 at audit start:**
  `b822fbace26a5d29fc8ffe8044c226c9f0d8f3fa14fec5959d78d1776823069d`
- **Resource-text changes during this audit:** none

## Target output model

This is a compact collection README, not a punctuation file. It has one H1,
five H2 headings, one fenced layout block, and an intended-use list. Its file
map is complete.

Cleanup should keep the evidence-base charter and every mapped file. It should
identify `resources/` as descriptive evidence rather than a normative layer,
replace the inert layout fence with a linked table, scope “Somali only” so
file `06` may keep first-use English glosses, and replace the 2026-07-18 note
that still talks about `## OCR Page N` markers in content files that no longer
have them.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-9 | reviewed | ORD-R001 |
| 11-24 | reviewed | ORD-R002 |
| 26-40 | reviewed | ORD-R003 |
| 42-49 | reviewed | ORD-R004 |
| 51-61 | reviewed | ORD-R005 |
| 63-67 | reviewed | ORD-R006 |
| whole file | reviewed | ORD-R007 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| ORD-R001 | 1-9 | Useful non-normative charter for SLS-0002/0004 evidence; it does not say `resources/` versus `spec/` as clearly as the cleaned morphology README / medium | `docs/RESOURCES.md`; `docs/ARCHITECTURE.md`; SLS-0002/0004/0005 planned | Retain the reference-and-evidence purpose. Add the descriptive-versus-normative boundary and a `docs/RESOURCES.md` link. Do not claim those specs are drafted. | `repository-supported`; `scope-correction`; `intentional-retained` |
| ORD-R002 | 11-24 | Layout lists every file correctly, but the code fence is non-navigable / low | All seven collection files exist; cleaned morphology README converted the same fence into a linked table | Preserve all seven entries. Convert the fence to a compact Markdown table with local links. | `repository-supported`; `navigation-update`; `intentional-retained` |
| ORD-R003 | 26-40 | Content-pattern examples still match files `01` and `05` / low | Cleaned `01` keeps `Lana iman` / `La ma iman`; file `05` still has one H3 per mark | Retain the three pattern bullets and the `Lana iman` pair. Do not replace it with a new example. | `repository-supported`; `intentional-retained` |
| ORD-R004 | 42-49 | Conventions are correct; “Somali only” would ban the Nilsson first-use glosses in file `06` / medium | O06-R001 keeps `capital letters`; alphabet/codkac stay in `naxwe/01` and `spec/orthography/0001-alphabet.md` | Retain UTF-8, filenames, omit-unreadable-OCR, and the alphabet/phonology exclusions. Scope Somali-only to files `01`–`05`, allowing first-use English glosses in `06`. | `repository-supported`; `scope-correction`; `intentional-retained` |
| ORD-R005 | 51-61 | The 2026-07-18 structural note is stale: content files no longer carry `## OCR Page N` markers, and `01`–`04` have since been cleaned / high | Current `01`–`05` have no OCR page headings; file-review folder now exists | Replace the dated OCR-page-marker inventory with a short current-status note linking `00-sources.md` and the file-review folder. Retain that file `06` is the Nilsson supplement. Do not hard-code a completion count. | `repository-supported`; `status-correction` |
| ORD-R006 | 63-67 | Intended-use bullets are accurate and do not claim finished `data/` orthography layers / low | `docs/RESOURCES.md` defers structured records | Retain all three bullets. | `repository-supported`; `intentional-retained` |
| ORD-R007 | whole file | Complete map, no extra files; needs navigation and a non-stale status line / medium | Seven collection files; this audit covers the remaining four | Preserve one H1 and the charter/pattern/convention/use sections. Add no mapped file. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned README should retain:

1. title and evidence-base purpose;
2. descriptive-evidence versus normative-specification note;
3. a linked layout table of all seven files;
4. the three content patterns, including `Lana iman`;
5. scoped Somali-only conventions; and
6. intended use, without a 2026-07-18 OCR-page claim.

No mapped file or charter topic should be removed. The collection should not
be marked complete.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** ORD-R001 through ORD-R007
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `b822fbace26a5d29fc8ffe8044c226c9f0d8f3fa14fec5959d78d1776823069d`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, five H2 headings, one fenced layout block, no
  OCR page markers in this README.

## Cleanup result and review

### Applied cleanup

- Added the descriptive-evidence versus normative-specification note with
  links to `docs/RESOURCES.md` and `spec/`.
- Converted the layout fence into a linked table of all seven files.
- Scoped Somali-only to files `01`–`05`, allowing a first-use English gloss
  in file `06`.
- Replaced the 2026-07-18 `## OCR Page N` inventory with a current-status
  note linking `00-sources.md` and the file-review folder.
- Did not claim SLS-0002, SLS-0004, or SLS-0005 are drafted.

### Deliberately retained

- The `Lana iman` / `La ma iman` pattern example.
- UTF-8, filenames, omit-unreadable-OCR, and the alphabet/phonology
  exclusions.
- All three intended-use bullets.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, five H2 headings, one linked layout table.
- Cleaned file size: 67 lines; 367 words; 3,140 bytes.
- Cleaned SHA-256:
  `3d55872cae5fc8ad5b52d48f67aa4a1f5e02ca8cd5d580c80e5cc5359127aca5`.
