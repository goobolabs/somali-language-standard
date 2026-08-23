# Audit record — Dhawaaq README

- **Resource path:** `resources/dhawaaq/README.md`
- **Collection / family:** dhawaaq / collection map and conventions
- **Priority:** P3
- **Method:** repository-only, line-by-line documentation audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 64 lines; 338 words; 2,756 bytes
- **Resource SHA-256 at audit start:**
  `ecbbe8c68dd92cfd30e6f41d0b3124905649e99480a76da3e65a8e24c3b5244e`
- **Resource-text changes during this audit:** none

## Target output model

This is a compact collection README, not a phoneme file. It has one H1, five
H2 headings, one fenced layout block, and an intended-use list. Its file map
is complete.

Cleanup should keep the evidence-base charter and every mapped file. It should
identify `resources/` as descriptive evidence rather than a normative layer,
replace the inert layout fence with a linked table, scope “Somali-first” so
first-use English technical glosses and IPA labels may stay, and replace the
2026-07-18 verification note that reads as finished curation.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-11 | reviewed | PHRD-R001 |
| 13-28 | reviewed | PHRD-R002 |
| 30-39 | reviewed | PHRD-R003 |
| 41-51 | reviewed | PHRD-R004 |
| 53-64 | reviewed | PHRD-R005 |
| whole file | reviewed | PHRD-R006 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| PHRD-R001 | 1-11 | Useful non-normative charter for future speech/phonology work; it does not say `resources/` versus `spec/` as clearly as the cleaned sarfe/orthography READMEs; “English and Italian explanatory glosses … omitted” would ban the first-use labels already in files `01`–`08` / medium | `docs/RESOURCES.md`; cleaned orthography README ORD-R001/R004; SLS-0600 is planned only | Retain the reference-and-evidence purpose. Add the descriptive-versus-normative boundary and a `docs/RESOURCES.md` link. Do not claim SLS-0600 is drafted. Scope omitted English to running translation of the 1977 prose, allowing first-use technical glosses and IPA chart labels. | `repository-supported`; `scope-correction`; `intentional-retained` |
| PHRD-R002 | 13-28 | Layout lists every file correctly, but the code fence is non-navigable / low | All ten collection files exist; cleaned sarfe/orthography READMEs converted the same fence into a linked table | Preserve all ten entries (eight content files plus registry and README). Convert the fence to a compact Markdown table with local links. | `repository-supported`; `navigation-update`; `intentional-retained` |
| PHRD-R003 | 30-39 | Content-pattern bullets still match files `01`–`05` / low | File `03` still uses `bilow` / `dhex` / `dhammaad`; files carry one H1 | Retain the three pattern bullets and the three-position notation. Do not replace them with a new example. | `repository-supported`; `intentional-retained` |
| PHRD-R004 | 41-51 | Conventions correctly split alphabet, punctuation, and prosody; file `08` role is accurate / low | Cleaned `naxwe/01`; cleaned `qoraal/05`; this audit of file `08` | Retain UTF-8, filenames, the `naxwe/01` alphabet exclusion, the punctuation-versus-juncture split, and the supplement-only role of file `08`. Convert those paths to Markdown links. | `repository-supported`; `navigation-update`; `intentional-retained` |
| PHRD-R005 | 53-64 | Intended-use bullets are accurate; the 2026-07-18 “Collection verified” note reads as finished curation and cites 155 OCR pages that content files no longer surface / high | Current `01`–`08` have no OCR page headings; file-review folder will exist after this audit | Retain all three intended-use bullets. Replace the dated verification block with a short current-status note linking `00-sources.md` and the file-review folder. Do not hard-code a completion count. | `repository-supported`; `status-correction`; `intentional-retained` |
| PHRD-R006 | whole file | Complete map, no extra files; needs navigation and a non-stale status line / medium | Ten collection files; this audit covers all of them | Preserve one H1 and the charter/pattern/convention/use sections. Add no mapped file. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned README should retain:

1. title and evidence-base purpose;
2. descriptive-evidence versus normative-specification note;
3. a linked layout table of all ten files;
4. the three content patterns, including three-position notation;
5. alphabet / punctuation / supplement conventions; and
6. intended use, without a 2026-07-18 completion claim.

No mapped file or charter topic should be removed. The collection should not
be marked complete.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** PHRD-R001 through PHRD-R006
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `ecbbe8c68dd92cfd30e6f41d0b3124905649e99480a76da3e65a8e24c3b5244e`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, five H2 headings, one fenced layout block, no
  OCR page markers in this README.

## Cleanup result and review

### Applied cleanup

- Added the descriptive-evidence versus normative-specification note with
  links to `docs/RESOURCES.md` and `spec/`.
- Converted the layout fence into a linked table of all ten files.
- Scoped omitted English to running translation of the 1977 prose, allowing
  first-use technical glosses and IPA chart labels.
- Converted convention paths to Markdown links.
- Replaced the 2026-07-18 verification block with a current-status note.
- Did not claim SLS-0600 is drafted.

### Deliberately retained

- The three content-pattern bullets, including `bilow` / `dhex` / `dhammaad`.
- Alphabet / punctuation / supplement conventions.
- All three intended-use bullets.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, five H2 headings, one linked layout table.
- Cleaned file size: 67 lines; 391 words; 3,426 bytes.
- Cleaned SHA-256:
  `e1aa90ef7a1a72b036b263263c2056dece011e28cb4eaea57941ea58a9d4b4e3`.
