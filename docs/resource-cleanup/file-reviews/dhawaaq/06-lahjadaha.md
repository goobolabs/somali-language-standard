# Audit record — Lahjadaha

- **Resource path:** `resources/dhawaaq/06-lahjadaha.md`
- **Collection / family:** dhawaaq / dialect variation
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 129 lines; 703 words; 4,219 bytes
- **Resource SHA-256 at audit start:**
  `9f1edbfe4fd4944c21c5c8e80e31e9f5b886b1144dbff9f9e9563777996023dc`
- **Resource-text changes during this audit:** none

## Target output model

This file is already a compact SLS dialect-phonology chapter, not a full
dialectology collection. It has one H1, three H2 sections, four H3 types, and
several small tables.

Cleanup should keep the four structural-variation types and the `dialects/`
boundary. It should reframe booklet voice, repair uniquely supported OCR, and
strip the closing drill. It must not reconstruct damaged IPA rows, invent Maay
or Maxaa inventories, or upgrade this file into a dialect collection.

## Source and repository evidence

Mapped to *Codaynta* dialect-variation preview. `docs/RESOURCES.md` and
`spec/orthography/0001-alphabet.md` reserve dialect inventories for a future
SLS-0700 / `dialects/` collection. Dictionary `cudur`, `hufnaan`, `timaxiir`.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-17 | reviewed | PH06-R001 |
| 19-42 | reviewed | PH06-R002 |
| 44-119 | reviewed | PH06-R003 |
| 121-129 | reviewed | PH06-R004 |
| whole file | reviewed | PH06-R005 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| PH06-R001 | 1-17 | Useful class-versus-region split; `hufnaadaa` is not a unique verb from dictionary `hufnaan`; `curdur` conflicts with `cudur`; `dhegmodhihid` / `degmodhihid` mix in the next section / medium | Dictionary `hufnaan`, `cudur`; same file later `degmodhihidda` | Retain the two-cause opening. Correct `curdur` to `cudur`. Leave `hufnaadaa` unresolved. Do not normalize every `dhegmo`/`degmo` pair unless a unique same-file standard is approved; prefer the majority `degmo` only if cleanup is later scoped that narrowly. | `repository-supported`; `form-correction`; `unresolved`; `intentional-retained` |
| PH06-R002 | 19-42 | Regional-dialect discussion and English/Italian comparisons are this source's illustrations, not SLS dialect data; `jegoxiir` is not dictionary `timaxiir` / low | Dictionary `timaxiir` is hairdressing, not a unique repair; SLS-0700 is out of scope | Retain the English and Italian comparison rows as the source's typology. Leave `jegoxiir` unresolved. Do not add Somali regional wordlists. | `unresolved`; `intentional-retained` |
| PH06-R003 | 44-119 | Four variation types are the usable core; several IPA cells are unreadable (`felin`, `habradai`, `nagradai`, `dosfazr`, `rainst`, `/p/` pairs) / high | Same-file types 1–4; file `03` has no `/p/` phoneme; omit-unreadable-OCR | Retain all four type headings and every readable pair (`/dibin/` `/debin/`, `/hogol/` `/hagol/`, `/d/` `/t/`, `/g/` `/k/`). Leave damaged IPA cells unresolved; do not reconstruct them from modern dialectology. Keep `/p/` as this source's comparison, not as an SLS phoneme. Reframe `Buuggani` as this chapter. | `repository-supported`; `scope-correction`; `unresolved`; `intentional-retained` |
| PH06-R004 | 121-129 | Accurate `dialects/` boundary; the last paragraph is an exercise / medium | `00-sources.md` coverage note; SLS-native exercise exclusion | Retain the four-type limit and the `dialects/` pointer as a future collection, not a present folder. Omit the “isku dayi kartaa” drill. | `repository-supported`; `structural-only`; `intentional-retained` |
| PH06-R005 | whole file | Compact preview, not a dialect atlas; damaged rows must not be rebuilt / medium | Three H2 sections, four types, no OCR page markers | Preserve the four-type sequence. Change only the approved form, omission, and framing repairs. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Lahjadaha** and keep: class/region
opening; four variation types with readable examples; `dialects/` boundary.
No new dialect rows. Damaged IPA stays unresolved. The closing drill is
removed.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** PH06-R001 through PH06-R005
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `9f1edbfe4fd4944c21c5c8e80e31e9f5b886b1144dbff9f9e9563777996023dc`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, three H2 headings, four H3 type subsections,
  several small tables, no OCR page markers.

## Cleanup result and review

### Applied cleanup

- Reframed `Buuggani` / `Baaxadda buuggan` as this chapter.
- Corrected `curdur` to `cudur`.
- Omitted the closing drill.
- Kept `dialects/` as a future collection, not a present folder.

### Deliberately retained

- `hufnaadaa` and `jegoxiir`.
- Damaged IPA cells and `/p/` as this source's comparison.
- English and Italian typology illustrations.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, three H2 headings, four H3 type subsections.
- Pre-cleanup forms `Buuggani`, `curdur`, and the closing drill are absent.
- Cleaned file size: 126 lines; 694 words; 4,138 bytes.
- Cleaned SHA-256:
  `e7921e702bbf9d6c01fc56e78285c989bf151fa8358b5d18bd4ad743c8b437c5`.
