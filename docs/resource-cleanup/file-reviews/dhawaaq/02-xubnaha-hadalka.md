# Audit record — Xubnaha hadalka

- **Resource path:** `resources/dhawaaq/02-xubnaha-hadalka.md`
- **Collection / family:** dhawaaq / speech organs
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 135 lines; 736 words; 4,741 bytes
- **Resource SHA-256 at audit start:**
  `3bda89e89a5d14cb1115d40771610b27438e0463087159b8ae6e1b3b8c1372ce`
- **Resource-text changes during this audit:** none

## Target output model

This file is already a compact SLS speech-organ chapter, not an OCR transcript.
It has one H1, eight H2 sections, two numbered lists, and no cover matter or
page markers.

Cleanup should keep initiation, diaphragm, phonation, articulation, the three
cavities, the organ list, and the consonant/vowel diagnostics. It should repair
uniquely supported splits, qualify the 52/47/15 counts as this source's figures
rather than an SLS phoneme inventory, and link files `01`, `03`, `04`, and `05`.
It must not invent a replacement for the non-adding totals or normalize
source terms `jeojin`, `neel`, or `qalaanqulshaha`.

## Source and repository evidence

Mapped to *Codaynta* Qaybta III. File `01` already names the three speech
tasks. File `07` repeats cavity and organ terms. Cleaned `naxwe/01` keeps the
SLS alphabet count separate from phonological description.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-28 | reviewed | PH02-R001 |
| 30-64 | reviewed | PH02-R002 |
| 66-80 | reviewed | PH02-R003 |
| 82-99 | reviewed | PH02-R004 |
| 101-135 | reviewed | PH02-R005 |
| whole file | reviewed | PH02-R006 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| PH02-R001 | 1-28 | Usable initiation and diaphragm account; `nuxur` is dictionary-supported as essence/nutrient, not a unique name for oxygen; carbon-dioxide gloss is a first-use label / low | Dictionary `nuxur¹` / `nuxur²`; file `01` initiation task | Retain both sections and the first-use *carbon dioxide* / *vacuum* labels. Leave `nuxurta` as this source's airflow term. Do not substitute modern anatomical names. | `repository-supported`; `unresolved`; `intentional-retained` |
| PH02-R002 | 30-64 | Phonation and articulation are the maintained organ account; `duunta` in the passive-articulator list conflicts with the same file's `dhuunta` / high | Same file lines 71, 79, 107; dictionary `dhuun²` (throat); `kowaad` is an accepted variant of `koowaad` | Correct `duunta` to `dhuunta`. Retain `qalaanqulshaha`, `jeojinta`, `cillidhinta`, voiced/voiceless first-use glosses, and `kowaad`. Do not add extra organs. | `repository-supported`; `form-correction`; `intentional-retained` |
| PH02-R003 | 66-80 | The three cavities match later files / low | File `03` sankabax/dhuun; file `07` cavity table | Retain all three numbered cavities and the oral/nasal/pharyngeal labels. Link `03` and `07` only if useful; do not copy their charts. | `repository-supported`; `intentional-retained` |
| PH02-R004 | 82-99 | Source sound-count paragraph is useful as this book's inventory, but `Laba iyo afartan iyo toddoba` disagrees with the same line's `(47)`, and 47+15 does not equal 52; `shibane` is a split of same-file `shibbane` / high | Same-file parentheticals 52 / 47 / 15; cleaned `naxwe/01` 21 consonants + 5 vowels is an alphabet count, not this chapter's phonetic total; file `03` prose says 22 consonants | Present 52, 47, and 15 as this source's figures, not as the SLS phoneme inventory. Correct `Laba iyo afartan iyo toddoba` to `afartan iyo toddoba` to match `(47)`. Correct both `shibane` to `shibbane`. Do not invent a reconciliation of 47+15 versus 52. Link `naxwe/01` for the alphabet count and files `03`–`05` for the two sound classes. | `repository-supported`; `form-correction`; `scope-correction`; `unresolved`; `intentional-retained` |
| PH02-R005 | 101-135 | The 15-item organ list and the consonant/vowel diagnostics match the chapter's purpose / low | File `01` three tasks; file `03` opening “saddexda arrimood”; file `04` syllabicity | Retain the complete organ list and both diagnostic lists. Link `03` and `04`. Keep `allanaynta` / *syllabication*. | `repository-supported`; `navigation-update`; `intentional-retained` |
| PH02-R006 | whole file | Already SLS-native; a few splits and an unreconciled count remain / medium | Eight H2 sections, no OCR page markers | Preserve the eight-section sequence. Change only the approved form, scope, and link repairs. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Xubnaha hadalka** and keep:
initiation; diaphragm; phonation; articulation; three cavities; source counts
(qualified); organ list; consonant/vowel diagnostics. Links go to `01`, `03`,
`04`, `05`, and `naxwe/01`. No phoneme chart is added here.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** PH02-R001 through PH02-R006
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `3bda89e89a5d14cb1115d40771610b27438e0463087159b8ae6e1b3b8c1372ce`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, eight H2 headings, two numbered lists, no OCR
  page markers.

## Cleanup result and review

### Applied cleanup

- Corrected `duunta` to `dhuunta` and both `shibane` to `shibbane`.
- Corrected `Laba iyo afartan iyo toddoba` to `afartan iyo toddoba` to match
  `(47)`.
- Presented 52 / 47 / 15 as this source's figures, not the SLS alphabet, and
  linked `naxwe/01`, `03`, `04`, and `05`.
- Reframed `buuggu tilmaamo` as `cutubkani tilmaamo`.

### Deliberately retained

- `nuxurta`, `kowaad`, `qalaanqulshaha`, `jeojinta`, and `cillidhinta`.
- The unreconciled 47+15 versus 52 totals.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, eight H2 headings, two numbered lists.
- Pre-cleanup forms `duunta`, `shibane`, and `Laba iyo afartan` are absent.
- Cleaned file size: 144 lines; 771 words; 5,239 bytes.
- Cleaned SHA-256:
  `66bb4a14a77a274d8f5420c804cf183e9fc1de9e919f54b9644aafb4ee4a6003`.
