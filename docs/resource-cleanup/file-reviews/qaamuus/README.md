# Audit record — Qaamuus README

- **Resource path:** `resources/qaamuus/README.md`
- **Collection / family:** qaamuus / collection map and conventions
- **Priority:** P3
- **Method:** repository-only, line-by-line documentation audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 66 lines; 252 words; 1,968 bytes
- **Resource SHA-256 at audit start:**
  `b81f030f5810058d8a6bbe450fb29e884cdf380473e369bc4478ba48a83775b4`
- **Resource-text changes during this audit:** none

## Target output model

This is a compact collection README, not a letter file. It has one H1, two H2
headings (`Layout`, `Audit notes`), two fenced blocks (layout summary and full
file list), and no entry-format section of its own.

Cleanup should keep the dictionary-evidence charter and every mapped file. It
should identify `resources/` as descriptive evidence rather than a normative
layer, replace inert layout fences with a linked table, add or link the entry-format
conventions now documented in the cleaned top-level
[`resources/README.md`](../../../../resources/README.md), scope cleanup to registry
navigation and OCR/unreadable fragments (not definition rewrites), and replace
the stale 2026-07-18 “structural audit” block. It must not mark the collection
complete.

## Source and repository evidence

All **34** Markdown files exist in `resources/qaamuus/` (`00-sources.md`,
`00-abbreviations.md`, `31` letter files, and this README).

Repository comparison:

- Cleaned phonology, morphology, orthography, and erey-bixin READMEs add
  evidence-versus-norm notes, linked layout tables, and current-status lines
  pointing at `docs/resource-cleanup/file-reviews/<collection>/`.
- Top-level [`resources/README.md`](../../../../resources/README.md) now documents
  qaamuus entry format (`Aabbe m.l …`, `ld`, `eeg`, homonym superscripts) and
  states qaamuus/wordlists are entering cleanup; this collection README does not
  yet link there.
- Recount at this audit: **48,138** entries in letter files; README audit notes
  still cite **42,511** and a 1,600-line sample with no replacement characters.
- [`madax-ereyo/00-sources.md`](../../../../resources/madax-ereyo/00-sources.md)
  records derivation from this collection; README audit notes mention wordlists
  but do not link.

Sample entry structure in `01-b.md` remains:

```text
- **ba'¹** m.l 1. Hoog iyo dhibaato … 2. (e.d) …
- **baa** qr.dd (nax.) … ld aa¹, ayaa, yaa³.
```

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-6 | reviewed | QARD-R001 |
| 8-15 | reviewed | QARD-R002 |
| 17-55 | reviewed | QARD-R003 |
| 57-66 | reviewed | QARD-R004 |
| whole file | reviewed | QARD-R005 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| QARD-R001 | 1-6 | Useful directory description but lacks the descriptive-evidence versus normative-spec boundary and the dictionary cleanup scope now stated in `docs/RESOURCES.md` and top-level README / medium | `docs/RESOURCES.md` qaamuus section; cleaned phonology README PHRD-R001 pattern | Retain the opening purpose (letter-split monolingual dictionary). Add evidence-versus-norm note and dictionary cleanup scope: preserve headwords, codes, definitions, and cross-refs; fix OCR/unreadable fragments; route substantive corrections to `data/provenance/correction-log.tsv`. Link `docs/RESOURCES.md` and top-level README entry-format section. Do not claim SLS lexicon norm status. | `repository-supported`; `scope-correction`; `navigation-update`; `intentional-retained` |
| QARD-R002 | 8-15 | Layout fence lists four roles correctly but paths are non-navigable / low | All four targets exist; sibling READMEs use linked tables | Preserve all four layout rows. Convert the fence to a compact Markdown table with local links. Mention **34** files total (32 content/registry targets plus this README). | `repository-supported`; `navigation-update`; `intentional-retained` |
| QARD-R003 | 17-55 | Full alphabet file list is accurate and should stay, but the duplicate fenced list is redundant once a linked table exists / low | Traditional Somali order matches `01-b` … `31-uu` filenames | Retain the complete B … UU sequence. After cleanup, either fold into the linked layout table or keep one alphabet block — not both identical fences. Do not rename letter files. | `repository-supported`; `intentional-retained`; `structural-only` |
| QARD-R004 | 57-66 | 2026-07-18 structural audit reads as finished verification; headword count **42,511** is stale; sample size and wordlists parity note lack links / high | Recount 2026-08-19: **48,138** entries; wordlists **47,524** heads; zero `�` lines in letter files at recount time | Replace the dated block with current status: structural file set complete, 2026-08-19 cleanup audit started on registry files, letter files unaudited, full scan not claimed. Refresh or remove hard-coded **42,511** (defer exact wordlists orphan count to paired wordlists audit). Link [`00-sources.md`](../../../../resources/qaamuus/00-sources.md), [`madax-ereyo/00-sources.md`](../../../../resources/madax-ereyo/00-sources.md), and [`docs/resource-cleanup/file-reviews/qaamuus/`](./). Retain the downstream-`data/` posture for broken `ld`/`eeg` targets. | `repository-supported`; `status-correction`; `navigation-update`; `intentional-retained` |
| QARD-R005 | whole file | Complete map, no entry-format section, needs navigation and non-stale status / medium | 34 collection files; abbreviations and sources audited same date | Preserve one H1 and layout/status sections. Add no mapped file. Do not mark complete. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned README should retain:

1. title and dictionary-evidence purpose;
2. descriptive-evidence versus normative-spec note and dictionary cleanup scope;
3. a linked layout table covering `00-sources.md`, `00-abbreviations.md`,
   `01-b` … `31-uu`, and this README;
4. entry-format conventions by link to top-level README (or one short local
   example plus link to `00-abbreviations.md`);
5. intended use (lexicon seed, cross-collection lookup, wordlists upstream);
6. current-status note without a 2026-07-18 completion claim.

No letter file should be removed. The collection should not be marked complete.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** QARD-R001 through QARD-R005
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `b81f030f5810058d8a6bbe450fb29e884cdf380473e369bc4478ba48a83775b4`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, two H2 headings, two fenced blocks, no Markdown
  links, no entry-format section.

## Cleanup result and review

### Applied cleanup

- Added descriptive-evidence versus normative-spec boundary and dictionary
  cleanup scope with links to `docs/RESOURCES.md`, `spec/`, and
  `correction-log.tsv`.
- Replaced inert layout fences with a linked table covering all registry and
  letter files.
- Added local entry-format section with `Aabbe` example and link to
  `00-abbreviations.md` and top-level README.
- Replaced the 2026-07-18 structural-audit block with current status, file-review
  link, wordlists derivation link, and letter files marked unaudited.

### Registry refresh (2026-08-19, post letter-file cleanup)

- Updated current-status section: letter files `01-b` … `31-uu` cleanup-applied.
  Maintainer approval was granted on 2026-08-23; the collection is complete
  for this cleanup pass.

### Deliberately retained

- Traditional Somali letter order B … UU in the layout table.
- Downstream-`data/` posture for broken `ld` / `eeg` targets.
- All 34 mapped files; no letter file removed or renamed.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, four H2 headings, one linked layout table, entry-format
  section, current-status section.
- Cleaned file size: 65 lines; 376 words; 2,970 bytes.
- Cleaned SHA-256:
  `678425bff703ffda2c38949378f81bb0a681de7f4d5542bcd6cc265c3de685a6`.
