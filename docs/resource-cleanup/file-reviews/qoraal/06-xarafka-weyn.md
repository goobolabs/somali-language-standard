# Audit record — Xarafka weyn

- **Resource path:** `resources/qoraal/06-xarafka-weyn.md`
- **Collection / family:** qoraal / capitalization supplement
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 94 lines; 454 words; 2,842 bytes
- **Resource SHA-256 at audit start:**
  `6f55af16fe19fb1b7f9fe1fd8dd5b4644e77fb4ed1f728f7f04820c00cd4b966`
- **Resource-text changes during this audit:** none

## Target output model

This file is already a compact SLS supplement, not an OCR transcript of
*Habka Qoraalka*. It has one H1, seven H2 sections, three tables, and an
explicit 1977-gap note.

Cleanup should keep it as the interim capitalization chapter: sentence start,
proper nouns, days/months, ethnonyms/language names, seasons, and directions.
It should link the registry, keep first-use English glosses, and not claim that
SLS-0005 already exists as a spec file. It must not import extra Nilsson
sections, invent a Somali-primary capitalization source, or normalize `af
Soomaali`.

## Source and repository evidence

`resources/qoraal/00-sources.md` maps this file to Nilsson §2.3 (2024
preliminary) and records that the 1974 Ministry book is not a capitalization
primary. `spec/orthography/0001-alphabet.md` points capitalization to planned
SLS-0005; `spec/orthography/0003-capitalization.md` is not present.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-14 | reviewed | O06-R001 |
| 16-39 | reviewed | O06-R002 |
| 41-62 | reviewed | O06-R003 |
| 64-85 | reviewed | O06-R004 |
| 87-94 | reviewed | O06-R005 |
| whole file | reviewed | O06-R006 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| O06-R001 | 1-14 | Clear 1977-gap note and interim charter; first-use English glosses match the Nilsson supplement; `00-sources.md` is not linked / medium | Collection map; morphology README allows a first-use English gloss; SLS-0005 is planned only | Retain the title, the 1977-gap sentence, and `capital letters`. Link `00-sources.md`. Do not present this file as SLS-0005. | `repository-supported`; `navigation-update`; `intentional-retained` |
| O06-R002 | 16-39 | Sentence-initial and proper-noun rules are the usable core; 1972–1973 matches this file's literacy window rather than a unique 1972-only claim / low | Cleaned `naxwe/01` limits 1972 to official Latin-script adoption; file `01` does not cover capitals | Retain both sections and both tables. Do not collapse 1972–1973 to 1972. | `repository-supported`; `intentional-retained` |
| O06-R003 | 41-62 | Days, months, ethnonyms, and `af Soomaali` are this supplement's examples / low | No other orthography file lists these capitalization rows | Retain both tables and `af Soomaali`. Do not capitalize `af` by English analogy. | `repository-supported`; `intentional-retained` |
| O06-R004 | 64-85 | Seasons and directions are scoped as often-capitalized names with lowercase general use; `bariga` has an article the other three labels lack / low | Same-file season pair `Jiilaal` / `jiilaal` | Retain both sections. Leave `bariga` versus `bari` unresolved. Do not add compass-point rows. | `unresolved`; `intentional-retained` |
| O06-R005 | 87-94 | The three reminder bullets are accurate; the 1974 sampling date is the earlier structural audit / low | `00-sources.md` coverage notes | Retain all three bullets. Link the registry path. Do not invent a Somali-primary replacement source. | `repository-supported`; `intentional-retained` |
| O06-R006 | whole file | Already SLS-native and correctly marked a supplement / medium | Seven H2 sections, three tables, no OCR page markers | Preserve the seven-section sequence. Change only the approved links. Keep English glosses as first-use labels. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Xarafka weyn** and keep: 1977-gap
note; sentence start; proper nouns; days/months; ethnonyms/language names;
seasons; directions; reminders. Link `00-sources.md`. Do not add a spec file
or extra Nilsson matter.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** O06-R001 through O06-R006
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `6f55af16fe19fb1b7f9fe1fd8dd5b4644e77fb4ed1f728f7f04820c00cd4b966`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, seven H2 headings, three tables, no OCR page
  markers.

## Cleanup result and review

### Applied cleanup

- Linked the opening source pointer and the reminder bullet to
  `00-sources.md`.
- Did not present this file as drafted SLS-0005.

### Deliberately retained

- The 1977-gap note and the 1972–1973 literacy window.
- All tables, first-use English glosses, and `af Soomaali`.
- Season and direction pairs, including unresolved `bariga` versus `bari`.
- All three reminder bullets, including the 1974 Ministry sampling claim.
- No extra Nilsson sections.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, eight H2 headings, three tables, no OCR page markers.
- Cleaned file size: 94 lines; 453 words; 2,880 bytes.
- Cleaned SHA-256:
  `5491d4d6588546882adbe1d5622c2b0458d126e35a731179b2f9bd9a1dd4504f`.
