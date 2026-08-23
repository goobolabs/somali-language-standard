# Audit record — Shaqaallada

- **Resource path:** `resources/dhawaaq/04-shaqaallada.md`
- **Collection / family:** dhawaaq / vowels
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; SLS-native cleanup applied and awaiting
  maintainer cleanup review
- **Audit started:** 2026-08-19
- **File size at audit start:** 209 lines; 1,252 words; 7,454 bytes
- **Resource SHA-256 at audit start:**
  `9c831abf360260cf40e19c83fa742c6e1276442759b8fa2e91766389f0b64aae`
- **Resource-text changes during this audit:** none

## Target output model

This file is already a compact SLS vowel chapter, not an OCR transcript. It has
one H1, five H2 sections, fifteen H3 phoneme subsections, one summary table,
and no page markers.

Cleanup should keep five descriptive parameters, short vowels including source
`/ə/`, five long vowels, five diphthongs, and the closing transcription note.
It should distinguish this phonetic set from the five-letter SLS alphabet,
repair uniquely supported OCR, and link `naxwe/01` §1.2.1. It must not drop
`/ə/`, invent lip-position labels for the collapsed triad, or respell
diphthong examples into modern orthography.

## Source and repository evidence

Mapped to *Codaynta* vowel chapters. Cleaned `naxwe/01` §1.2.1 already points
here for vowel quality and keeps five short / five long letters. Dictionary
`samee` supports `senceyay` as likely `sameeyay`. File `07` repeats `nuhinsan`.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-23 | reviewed | PH04-R001 |
| 25-46 | reviewed | PH04-R002 |
| 48-92 | reviewed | PH04-R003 |
| 94-175 | reviewed | PH04-R004 |
| 177-209 | reviewed | PH04-R005 |
| whole file | reviewed | PH04-R006 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| PH04-R001 | 1-23 | Useful vowel-versus-consonant opening; `senceyay` is not a repository verb; `nuhinsan` also occurs in file `07` and is not uniquely `muhiimsan` / medium | Dictionary `samee` (`-meeyay`); file `07` `ugu nuhinsan`; cleaned naxwe does not use `nuhinsan` | Retain the two diagnostics, *nidab* / *color*, and the sound-wave sentence. Correct `senceyay` to `sameeyay`. Leave `nuhinsan` unresolved. Link file `03` only as the consonant counterpart, not to copy its table. | `repository-supported`; `form-correction`; `unresolved`; `intentional-retained` |
| PH04-R002 | 25-46 | Five parameters are the source's vowel-description grid; lip-position line 42 repeats `urursan` and does not uniquely yield rounded/spread/neutral; `badbaad` is not an independent place term / medium | Same-file later headings use hore / dhexe / danbe; no other phonology file lists `badbaad` | Retain all five numbered parameters. Leave the collapsed lip triad and `badbaad` unresolved. Do not invent replacement labels. | `unresolved`; `intentional-retained` |
| PH04-R003 | 48-92 | Short-vowel set includes `/ə/` as omissible; that is this source's extra quality, not a sixth SLS letter / medium | Cleaned `naxwe/01` five short letters; this file already says `/ə/` `waa laga naafin karaa` | Retain all six short subsections, including `/ə/`. Present `/ə/` as this source's extra quality, not as an SLS alphabet letter. Keep the example words. Link `naxwe/01` §1.2.1. | `repository-supported`; `scope-correction`; `intentional-retained` |
| PH04-R004 | 94-175 | Long vowels and diphthongs match the five-letter length system; some diphthong examples mix orthography and IPA (`shay` `/shai/`, *koo* `/kou/`) / medium | `naxwe/01` doubled long vowels; file `03` `/j/` `/w/` as glides; no unique repair for *dei*, *koo*, *gow* | Retain all five long vowels and all five diphthongs. Leave mixed-notation examples unresolved. Do not respell them into `ee`/`oo` by English analogy. | `repository-supported`; `unresolved`; `intentional-retained` |
| PH04-R005 | 177-209 | Summary table matches the subsections; the `/y/`-final spelling note is this source's transcription remark, not SLS-0002 / low | Same-file diphthong rows; orthography collection is spelling, not this chapter | Retain the 15-row table and the `/y/` note as source transcription practice. Do not turn `shaiyo` into a spelling rule. | `intentional-retained` |
| PH04-R006 | whole file | Already SLS-native; source extras (`/ə/`, diphthongs) must stay scoped / medium | Five H2 sections, fifteen H3 subsections, one table, no OCR page markers | Preserve the short / long / diphthong sequence. Change only the approved form, scope, and link repairs. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Shaqaallada** and keep: opening
diagnostics; five parameters; short vowels including `/ə/`; long vowels;
diphthongs; summary table. Link `naxwe/01` and file `03`. Do not add or drop
a vowel symbol.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** PH04-R001 through PH04-R006
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Audit validation

- Resource SHA-256 after audit:
  `9c831abf360260cf40e19c83fa742c6e1276442759b8fa2e91766389f0b64aae`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, five H2 headings, fifteen H3 phoneme subsections,
  one table, no OCR page markers.

## Cleanup result and review

### Applied cleanup

- Corrected `senceyay` to `sameeyay`.
- Linked file `03` as the consonant counterpart.
- Scoped `/ə/` as this source's extra quality, not an SLS alphabet letter, and
  linked `naxwe/01` §1.2.1.

### Deliberately retained

- `nuhinsan`.
- The collapsed lip-position triad and `badbaad`.
- Mixed diphthong notation (`shay` `/shai/`, *koo* `/kou/`).
- All six short vowels including `/ə/`.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, five H2 headings, fifteen H3 phoneme subsections, one
  table.
- Pre-cleanup form `senceyay` is absent.
- Cleaned file size: 215 lines; 1,282 words; 7,726 bytes.
- Cleaned SHA-256:
  `47570153fe5e8029c4f4cb3b78a058aa380e114f7cfcc187acaf4609817a9187`.
