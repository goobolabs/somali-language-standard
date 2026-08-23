# Audit record — Sources: dhawaaq

- **Resource path:** `resources/dhawaaq/00-sources.md`
- **Collection / family:** dhawaaq / source registry
- **Priority:** P3
- **Method:** repository-only, row-by-row metadata audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 59 lines; 362 words; 2,554 bytes
- **Resource SHA-256 at audit start:**
  `63dffe482d3ffd5319fa1260e0ed2777c7a89006e42acad102f3e4d2adc47181`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact collection registry, not a phoneme chapter. It has one
H1, five H2 headings, three valid Markdown tables, four coverage-note bullets,
and a 2026-07-18 structural-audit table. Every mapped content file exists.

Cleanup should keep *Codaynta Af Soomaaliga* (1977) as the only primary for
files `01`–`07`, keep Orwin as the phonation supplement, and keep the
audio/dialect/`Metrics` exclusions. It should turn file references into local
links and replace the stale “manually verified” / 2026-07-18 completion
wording with extraction versus August 2026 cleanup-audit status. It must not
add a second primary book or mark SLS-0600 as drafted.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-4 | reviewed | PH00S-R001 |
| 6-12 | reviewed | PH00S-R002 |
| 14-22 | reviewed | PH00S-R003 |
| 24-35 | reviewed | PH00S-R004 |
| 37-59 | reviewed | PH00S-R005 |
| whole file | reviewed | PH00S-R006 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| PH00S-R001 | 1-4 | Clear English registry title and attribution-once purpose / low | Sibling `00-sources.md` files; cleaned sarfe/orthography registries kept English | Retain the title and one-sentence purpose. Do not translate only this registry. | `repository-supported`; `intentional-retained` |
| PH00S-R002 | 6-12 | Primary row correctly names Raabi 1977 and the Lafoole publisher; the file range is not linked / low | Cleaned `qoraal/00-sources.md` same author/publisher; files `01`–`07` exist | Retain title, author, year, and publisher. Link `01`–`07`. Do not add a second primary book. | `repository-supported`; `navigation-update`; `intentional-retained` |
| PH00S-R003 | 14-22 | Orwin row is the correct interim phonation source and already says supplementary-only / low | File `08` opening; `00-sources.md` Metrics deferral | Retain the supplement row. Link `08-gariirka-iyo-glotis-furan.md`. Do not upgrade Orwin to primary. | `repository-supported`; `intentional-retained` |
| PH00S-R004 | 24-35 | File map matches all eight content files; paths are inert / low | All eight files exist; this audit of `01`–`08` | Retain all eight rows. Convert file cells to local links. Do not retitle Orwin as a 1977 section. | `repository-supported`; `navigation-update`; `intentional-retained` |
| PH00S-R005 | 37-59 | Coverage notes are right on audio, Metrics, and `dialects/`, but “manually verified” plus the 2026-07-18 pass table read as finished curation while this cleanup audit is still open / high | Orthography O00S-R005; no audio files in this collection; `dialects/` folder is not present | Retain the audio, Metrics, and dialect-lexicon exclusions. Replace completion wording and the dated pass table with extraction versus August 2026 cleanup-audit status and a link to `docs/resource-cleanup/file-reviews/dhawaaq/`. Do not claim SLS-0600 is drafted. | `repository-supported`; `status-correction`; `intentional-retained` |
| PH00S-R006 | whole file | Valid registry with no extra bibliography / medium | Three tables, all mapped files present | Preserve the primary / supplement / map / coverage sequence. Add no new source row. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned registry should retain:

1. title and attribution-once purpose;
2. linked Raabi 1977 primary row for `01`–`07`;
3. linked Orwin supplement row;
4. linked eight-row file map; and
5. coverage notes that distinguish scan extraction from this cleanup audit.

No new book should be added. SLS-0600 should stay planned.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** PH00S-R001 through PH00S-R006
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `63dffe482d3ffd5319fa1260e0ed2777c7a89006e42acad102f3e4d2adc47181`
  (unchanged).
- Resource diff during this audit: none.

## Cleanup result and review

### Applied cleanup

- Linked files `01`–`07` in the primary row and converted every file-map cell
  to a local link.
- Linked `08-gariirka-iyo-glotis-furan.md` in the Orwin supplement row.
- Replaced “manually verified” wording and the 2026-07-18 pass table with
  extraction versus the 2026-08-19 cleanup-audit status.
- Recorded that planned spec `SLS-0600` is not yet drafted.

### Deliberately retained

- Raabi 1977 as the only primary for files `01`–`07`.
- Orwin as a phonation supplement, not a primary.
- Audio, Metrics, and dialect-lexicon exclusions.

### Cleanup validation

- `git diff --check`: passed.
- Three tables remain; no new source row was added.
- Cleaned file size: 49 lines; 272 words; 2,478 bytes.
- Cleaned SHA-256:
  `7348a313fa9266f72141e24833ad4b72b37bec7aefd372a083566bd6ad5127fb`.
