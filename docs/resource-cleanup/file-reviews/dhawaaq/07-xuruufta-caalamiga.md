# Audit record — Xuruufta caalamiga ah

- **Resource path:** `resources/dhawaaq/07-xuruufta-caalamiga.md`
- **Collection / family:** dhawaaq / IPA and terminology
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; SLS-native cleanup applied and awaiting
  maintainer cleanup review
- **Audit started:** 2026-08-19
- **File size at audit start:** 115 lines; 844 words; 4,926 bytes
- **Resource SHA-256 at audit start:**
  `04579e6bd65ea8bde25cb1a430a26203bd0be9516234117e7f1d9a6750e09f05`
- **Resource-text changes during this audit:** none

## Target output model

This file is already a compact SLS IPA/terminology appendix, not a reprint of
the 1977 plates. It has one H1, three H2 sections, six H3 glossary groups, and
three tables.

Cleanup should keep the terminology glossary and the source's IPA-versus-Somali
writing note. It should reframe booklet voice, link files `02`–`05` and
`naxwe/01`, and treat the plate-number list as a source index without
inventing figures. It must not rewrite `/c/ = /ʤ/` into modern IPA, add missing
drawings, or duplicate file `03`'s phoneme chart.

## Source and repository evidence

Mapped to *Codaynta* IPA appendix. File `03` uses `/ʤ/` and `/ħ/` in a
different pairing than this file's `/c/ = /ʤ/` and `/x/ = /ħ/`. SLS-0001 is
letter identity, not IPA. File `02` supplies cavity terms.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-19 | reviewed | PH07-R001 |
| 21-75 | reviewed | PH07-R002 |
| 76-107 | reviewed | PH07-R003 |
| 109-115 | reviewed | PH07-R004 |
| whole file | reviewed | PH07-R005 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| PH07-R001 | 1-19 | Useful IPA-chart charter; `Buuggu` / `buugga asalka ah` is booklet voice; English manner/place lists are first-use IPA labels, not omitted glosses of running prose / low | File `03` closing place list; morphology README allows first-use English glosses | Reframe as this collection's IPA appendix. Retain the English place/manner/vowel labels as IPA chart terms. Do not add drawings. | `repository-supported`; `scope-correction`; `intentional-retained` |
| PH07-R002 | 21-75 | Terminology tables are the usable glossary; `/ʕ/` appears here but not as a file `03` row; unaspirated row repeats `/d/`; `nuhinsan` recurs from file `04` / medium | File `03` inventory omits `/ʕ/`; file `05` pitch names match this table | Retain every glossary row. Leave `/ʕ/`, the repeated `/d/`, and `nuhinsan` unresolved. Do not add a `/ʕ/` subsection to file `03` from this mention. | `unresolved`; `intentional-retained` |
| PH07-R003 | 76-107 | Plate-number list and tusayaasha index have no images in this collection / medium | SLS-native exclusions cover book apparatus; no `source-evidence` figures are linked from this file | Keep a short statement that the 1977 plates are not reproduced here. Collapse or omit the unused figure-number list rather than inventing captions. Retain the eight-item tusayaasha map as a chapter index linking `03`–`05` where those topics exist. | `structural-only`; `navigation-update`; `intentional-retained` |
| PH07-R004 | 109-115 | IPA-versus-orthography closing is useful, but `/c/ = /ʤ/` and `/x/ = /ħ/` are this file's equations and conflict with standard IPA and with file `03`'s example pairing / high | SLS letters: `c` ≠ `j`, `x` = source `/ħ/` in file `03` *xoolo*; file `03` `/ʤ/` examples are *dh*-words | Retain the closing contrast that IPA is phonetic description and Somali orthography is the writing system. Leave the concrete `/c/ = /ʤ/` equation unresolved; do not silently modernize it. Link `naxwe/01` and SLS-0001 for letter identity. | `repository-supported`; `unresolved`; `intentional-retained` |
| PH07-R005 | whole file | Compact appendix; glossary should stay, plates should not be rebuilt / medium | Three H2 sections, six glossary groups, no OCR page markers | Preserve terminology tables and the IPA/orthography note. Change only the approved framing, plate-list, and link repairs. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Xuruufta caalamiga ah** and keep:
IPA charter; terminology glossary; a short no-plates note plus a linked topic
index; IPA-versus-writing closing. No drawings are invented. File `03`'s
inventory is not overwritten.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** PH07-R001 through PH07-R005
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Audit validation

- Resource SHA-256 after audit:
  `04579e6bd65ea8bde25cb1a430a26203bd0be9516234117e7f1d9a6750e09f05`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, three H2 headings, six H3 glossary groups, three
  tables, no OCR page markers.

## Cleanup result and review

### Applied cleanup

- Reframed booklet voice as this collection's IPA appendix.
- Replaced the unused plate-number list with a no-plates note and a linked
  topic index to files `03`–`05`.
- Linked letter identity to `naxwe/01` and SLS-0001 without rewriting
  `/c/ = /ʤ/`.

### Deliberately retained

- Terminology glossary rows, including `/ʕ/` and `nuhinsan`.
- The source equation `/c/ = /ʤ/`.
- English IPA place/manner labels.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, three H2 headings, six H3 glossary groups; plate-number
  table omitted.
- Cleaned file size: 100 lines; 713 words; 4,738 bytes.
- Cleaned SHA-256:
  `4842378c75bb480b02ea146a2d007ba1a14713f5eacada8beb223ac35f1d1b26`.
