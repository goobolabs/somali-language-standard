# Audit record — Sources: qaamuus

- **Resource path:** `resources/qaamuus/00-sources.md`
- **Collection / family:** qaamuus / source registry
- **Priority:** P3
- **Method:** repository-only, row-by-row metadata audit
- **Audit status:** approved; registry cleanup applied and awaiting
  maintainer cleanup review
- **Audit started:** 2026-08-19
- **File size at audit start:** 27 lines; 159 words; 1,006 bytes
- **Resource SHA-256 at audit start:**
  `f8cc0fee3ed8340a3b8a9460667aee12defe95cb0866b886038d147678fbcc11`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact collection registry, not a dictionary letter file. It has
one H1, three H2 headings, two valid Markdown tables, and four coverage-note
bullets. Every mapped content file exists (34 Markdown files in the directory:
`00-abbreviations.md`, `01-b.md` … `31-uu.md`, plus this registry and
`README.md`).

Cleanup should keep *Qaamuuska Af-Soomaaliga* as the primary title, keep
compiler/year as unresolved metadata queue items, link file-map cells and the
wordlists derivation note, refresh the headword count with an explicit counting
rule, and replace the silent July 2026 structural-audit posture with the current
2026-08-19 cleanup-audit status. It must not invent a compiler, edition, or
publisher, and must not claim republication rights.

## Source and repository evidence

Sibling registries (`madax-ereyo/00-sources.md`, cleaned sarfe/phonology
`00-sources.md`) link file cells, record derivation from upstream collections,
and distinguish extraction or structural counts from cleanup-audit status.

Repository recount at this audit (lines matching `- **` in `01-b.md` …
`31-uu.md`): **48,138** dictionary entries. The registry and collection README
still cite **42,511** from the 2026-07-18 structural audit — stale by **5,627**
entries. Wordlists currently hold **47,524** bare heads (same counting rule on
`- ` list items), versus the documented **42,542**.

`docs/RESOURCES.md` scopes `qaamuus/` as monolingual dictionary evidence for a
future `data/lexicon/`, with no hand-corrected definitions. The cleaned top-level
[`resources/README.md`](../../../../resources/README.md) documents entry format,
abbreviations, `ld`/`eeg`, and homonym superscripts, and states that qaamuus and
wordlists are now entering this cleanup pass.

Metadata queue references M-001, M-003, and M-015 appear only in this registry;
no other in-repo file resolves them yet.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-4 | reviewed | QA00S-R001 |
| 6-13 | reviewed | QA00S-R002 |
| 15-20 | reviewed | QA00S-R003 |
| 22-27 | reviewed | QA00S-R004 |
| whole file | reviewed | QA00S-R005 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| QA00S-R001 | 1-4 | Clear English registry title and attribution-once purpose matching sibling `00-sources.md` files / low | Cleaned morphology, phonology, erey-bixin registries keep English framing | Retain the title and one-sentence purpose. Do not translate only this registry. Do not move compiler/year into letter files. | `repository-supported`; `intentional-retained` |
| QA00S-R002 | 6-13 | Primary source row correctly titles *Qaamuuska Af-Soomaaliga* and defers compiler, year, and rights to M-001, M-003, M-015; format cell lists the letter split accurately / low | `docs/RESOURCES.md` dictionary limitation table; no competing compiler name in repo | Retain title, unresolved compiler/year cells, and the `01-b` … `31-uu` format note. Do not invent compiler, edition, publisher, or rights. Keep metadata queue IDs until resolved elsewhere. | `repository-supported`; `intentional-retained`; `unresolved` |
| QA00S-R003 | 15-20 | File map is complete but paths are inert / low | All 32 mapped targets exist; cleaned sibling registries link file cells | Retain both rows (`00-abbreviations.md` and `01-b` … `31-uu.md`). Convert cells to local Markdown links. Do not add files. | `repository-supported`; `navigation-update`; `intentional-retained` |
| QA00S-R004 | 22-27 | Coverage notes correctly point wordlists derivation and the `data/` correction posture, but the **42,511** headword count is stale and there is no cleanup-audit status / high | Recount 2026-08-19: **48,138** qaamuus entries; wordlists **47,524** heads; top-level README entry-format section | Retain wordlists derivation and the “structured corrections belong downstream in `data/`” rule. Replace **42,511** with the recount and state the counting rule (`- **` lines in letter files). Note that full scan verification is not part of this registry pass. Link [`madax-ereyo/00-sources.md`](../../../../resources/madax-ereyo/00-sources.md) and [`docs/resource-cleanup/file-reviews/qaamuus/`](./). Do not mark the collection complete. | `repository-supported`; `status-correction`; `navigation-update`; `intentional-retained` |
| QA00S-R005 | whole file | Valid three-section registry with no extra bibliography / medium | Two tables; 32 mapped content targets present | Preserve primary-source / file-map / coverage-note sequence. Add no new source row and no inferred metadata. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned registry should retain:

1. title and attribution-once purpose;
2. primary *Qaamuuska Af-Soomaaliga* row with unresolved M-001 / M-003 / M-015
   fields;
3. linked file map for `00-abbreviations.md` and `01-b` … `31-uu.md`;
4. coverage notes with refreshed entry count, wordlists derivation link,
   downstream-`data/` correction rule, and current cleanup-audit status.

No compiler, edition, or rights claim should be added. Unresolved metadata stays
unresolved.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** QA00S-R001 through QA00S-R005
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Audit validation

- Resource SHA-256 after audit:
  `f8cc0fee3ed8340a3b8a9460667aee12defe95cb0866b886038d147678fbcc11`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, three H2s, two tables, four coverage bullets, no
  Markdown links.

## Cleanup result and review

### Applied cleanup

- Linked the format cell and both file-map rows.
- Replaced **42,511** with **48,138** and documented the `- **` counting rule
  plus the no-full-scan limitation.
- Linked [`madax-ereyo/00-sources.md`](../../../../resources/madax-ereyo/00-sources.md)
  and [`docs/resource-cleanup/file-reviews/qaamuus/`](./).
- Scoped normative lexical edits to `data/`; noted letter files are unaudited.

### Registry refresh (2026-08-19, post letter-file cleanup)

- Updated entry count to **48,119** after letter-file cleanup.
- Marked letter files `01-b` … `31-uu` as cleanup-applied (awaiting review).

### Deliberately retained

- English attribution-once title and purpose.
- Unresolved compiler/year and M-001 / M-003 / M-015 metadata queue IDs.
- Primary title *Qaamuuska Af-Soomaaliga* and letter-split format description.
- No new source row or inferred metadata.

### Cleanup validation

- `git diff --check`: passed.
- Two linked tables; refreshed entry count; letter files marked unaudited.
- Cleaned file size: 35 lines; 198 words; 1,533 bytes.
- Cleaned SHA-256:
  `1c62311cf14d0e1519f7d4c7ed62471e7878d9e3b809b3a25d7de6c6301f0225`.
