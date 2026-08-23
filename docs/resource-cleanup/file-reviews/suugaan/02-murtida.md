# Audit record — Qaamuuska Murtida

- **Resource path:** `resources/suugaan/02-murtida.md`
- **Collection / family:** suugaan / murti glossary
- **Priority:** P2
- **Method:** whole-file glossary audit; repository-only
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit date:** 2026-08-19
- **File size at audit start:** 221 lines
- **Resource SHA-256 at audit start:**
  `115ff2bc4184292848928c13666ccf3ee3c5390de13c9739c39bc7c6a0a5140f`
- **Resource-text changes during audit:** none

## Target output model

The cleaned file should remain Maxamed Xuseen Hadi's *Qaamuuska Murtida* (1996)
glossary: H1, one `## Ereyada iyo macnahooda` section, and `Headword — definition`
lines in source order. Cleanup may repair obvious OCR damage in headwords or
definitions. It must not reorder entries, merge duplicates silently, or add
per-file provenance blocks.

## Findings

| ID | Severity | Finding | Proposed action |
| --- | --- | --- | --- |
| SUU02-R001 | low | H1 and single glossary section structure intact | Retain |
| SUU02-R002 | medium | Line 90: `daf ka slin` OCR → `daf ka silin` | Fix definition gloss |
| SUU02-R003 | medium | Line 102: headword `Soqjna` OCR → `Soofna` | Fix headword spelling |
| SUU02-R004 | low | Historical or dialect gloss forms elsewhere unverified without scan | Retain unless scan proves error |

## Approval state

- **Audit approval:** approved by the maintainer on 2026-08-19 with the instruction, "go ahead."
- **Finding IDs:** SUU02-R001 through SUU02-R004
- **Cleanup:** applied on 2026-08-19
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Cleanup result

Two scan-supported OCR repairs (SUU02-R002, SUU02-R003) were applied. No entries
were added, removed, or reordered.

- **Post-cleanup:** 221 lines
- **Resource SHA-256 after cleanup:**
  `66f1dd6bad0158bc3f8b4666990145e00b14a8597c7775f46122854ef63ccda0`
- **Wordlist parity:** n/a
- **Status:** complete; cleanup approved 2026-08-23
