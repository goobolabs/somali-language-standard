# Audit record — Resources README

- **Resource path:** `resources/README.md`
- **Collection / family:** resources / top-level evidence-library map
- **Priority:** P3
- **Method:** repository-only, line-by-line documentation audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 124 lines; 667 words; 6,444 bytes
- **Resource SHA-256 at audit start:**
  `72fda9fe3d1dcc1e9a3f39cdcf327cbf136c7d4ceae7f02c0bf237a14bf233fc`
- **Resource-text changes during this audit:** none

## Target output model

This is the live map of the eight `resources/` collections, not a grammar
chapter and not `docs/RESOURCES.md`. It has one H1, four H2 headings, one
nested layout fence covering all eight collections, a qaamuus entry example,
a wordlist format note, and a shared-conventions list. Every named collection
directory exists. File cells are inert. The opening and conventions treat
every collection as topic-reorganized SLS synthesis, which is false for
`suugaan/` and `erey-bixin/`.

Cleanup should keep all eight collections, the qaamuus example, and the
wordlist one-headword-per-line rule. It should identify `resources/` as
descriptive evidence rather than a normative layer, convert the fence into
linked collection tables, and scope the three content philosophies already
used in this cleanup: SLS-native topic reference (`naxwe`, `morphology`,
`orthography`, `phonology`); source-faithful literary (`suugaan`); historical
bilingual glossary (`erey-bixin`). Dictionary collections (`qaamuus`,
`wordlists`) stay as listed and remain unaudited. It must not mark any
collection complete, copy stale OCR percentages from `docs/RESOURCES.md`, or
expand this audit into `qaamuus/`, `madax-ereyo/`, or remaining `suugaan/`
files.

## Source and repository evidence

Filesystem counts: `qaamuus/` 31 letter files plus abbreviations; `madax-ereyo/`
26 letter files; `naxwe/` files `00`–`17` plus `ereyfur.md`; `erey-bixin/`
`01`–`09`; `suugaan/` 24 content files; `qoraal/` `01`–`06`;
`dhawaaq/` `01`–`08`; `sarfe/` `01`–`04`. Cleaned collection READMEs
already use linked tables and `docs/RESOURCES.md` / `spec/` links.

Repository comparison also included:

- cleaned `naxwe/README.md`: files `00`–`12` are the core; `13`–`17`
  supplement; not a single “phonology-to-syntax” 13-file lump;
- cleaned `erey-bixin/README.md` and `00-sources.md`: `09` partial;
  Ururinta 2aad is a separate block in `05`;
- suugaan file-reviews `04`–`11`: source-faithful literary, not
  SLS-native topic rewrite;
- `docs/RESOURCES.md` (2026-07-19): still useful for the evidence-versus-norm
  layer, but its `naxwe/13`–`17` OCR percentages and `erey-bixin/09`
  Italian-unresolved row are stale relative to this cleanup.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-5 | reviewed | RES-R001 |
| 7-92 | reviewed | RES-R002 |
| 94-106 | reviewed | RES-R003 |
| 108-111 | reviewed | RES-R004 |
| 113-124 | reviewed | RES-R005 |
| whole file | reviewed | RES-R006 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| RES-R001 | 1-5 | Useful library purpose, but “cleaned and reorganized into topic-focused SLS resources” over-applies naxwe method to every collection; no `docs/RESOURCES.md` / `spec/` link / high | `docs/RESOURCES.md` evidence-versus-norm; cleaned naxwe/erey-bixin/suugaan READMEs | Retain the evidence-library purpose. Add the descriptive-versus-normative boundary with links to `docs/RESOURCES.md` and `spec/`. State the three content philosophies; do not call `suugaan/` or `erey-bixin/` topic-reorganized SLS synthesis. | `repository-supported`; `scope-correction`; `intentional-retained` |
| RES-R002 | 7-92 | Layout lists all eight collections and is a useful live map, but the fence is non-navigable; naxwe `00`–`12` is mis-summarized; `erey-bixin/09` is not marked partial; suugaan uses fake globs (`04-13-*.md`) / high | Filesystem counts above; cleaned naxwe and erey-bixin READMEs | Keep all eight collections and their files. Convert the fence to linked tables (collection `README.md` / `00-sources.md`, plus range endpoints rather than every qaamuus letter file). Describe naxwe `00`–`12` as the core and `13`–`17` as supplements. Label `erey-bixin/09` partial. Replace suugaan globs with real filenames or a compact `04`–`13` story range. Do not add Ismail Mire or other deferred works. | `repository-supported`; `navigation-update`; `status-correction`; `intentional-retained` |
| RES-R003 | 94-106 | Qaamuus one-line entry example and code notes still match the unaudited dictionary files / low | `qaamuus/00-abbreviations.md` exists; this audit does not open letter files | Retain the example, `ld` / `eeg`, homonym superscripts, and the abbreviations link (make it a local link). Do not rewrite dictionary format during this cleanup. | `repository-supported`; `intentional-retained` |
| RES-R004 | 108-111 | Wordlist one-headword-per-line description is correct / low | 26 letter files exist; long vowels live in qaamuus | Retain the wordlist format note. Link `madax-ereyo/`. | `repository-supported`; `navigation-update`; `intentional-retained` |
| RES-R005 | 113-124 | Shared exclusions (cover, bios, TOC, page numbers, exercises) are right, but “topic-focused / SLS-native hierarchy” and “not page-by-page transcriptions” would ban source-faithful suugaan and historical erey-bixin lists / high | Erey-bixin and suugaan file-reviews; `data/provenance/correction-log.tsv` | Retain UTF-8, kebab-case, and the shared exclusions. Scope SLS-native topic hierarchy to `naxwe/`, `sarfe/`, `qoraal/`, and `dhawaaq/`. Keep suugaan source-faithful (fix OCR, do not modernize). Keep erey-bixin printed pairs (unique OCR only; Italian-only/unreadable omitted). Point provenance to `data/provenance/correction-log.tsv` and `docs/resource-cleanup/file-reviews/`. | `repository-supported`; `scope-correction`; `intentional-retained` |
| RES-R006 | whole file | Complete eight-collection map; needs navigation, philosophy scoping, and a non-complete status line / medium | Tracker: naxwe/sarfe/qoraal/dhawaaq/erey-bixin at cleanup review; suugaan paused; qaamuus/wordlists unaudited | Preserve one H1 and the map / qaamuus / wordlist / convention sections. Add a short current-status note linking the tracker. Do not hard-code a completion count. Do not edit `docs/RESOURCES.md` from this cleanup. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned top-level README should retain:

1. title and evidence-library purpose;
2. descriptive-evidence versus normative-specification note;
3. the three content philosophies;
4. a linked live map of all eight collections;
5. the qaamuus entry example and wordlist format;
6. shared exclusions, scoped by collection type; and
7. a current-status pointer to the tracker, without a completeness claim.

No collection should be added or removed. Qaamuus, wordlists, and remaining
suugaan files should not be cleaned from this README.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** RES-R001 through RES-R006
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `72fda9fe3d1dcc1e9a3f39cdcf327cbf136c7d4ceae7f02c0bf237a14bf233fc`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, four H2 headings, one nested eight-collection
  fence, qaamuus example present, all eight collection directories present.

## Cleanup result and review

### Applied cleanup

- Added the descriptive-evidence versus normative-specification note with
  links to `docs/RESOURCES.md` and `spec/`.
- Stated the three content philosophies plus unaudited dictionary collections.
- Converted the layout fence into linked tables for all eight collections.
- Described naxwe `00`–`12` as the core and `13`–`17` as supplements.
- Labeled `erey-bixin/09` partial; replaced suugaan fake globs with real
  filenames or range endpoints.
- Linked the qaamuus abbreviations file and `madax-ereyo/`.
- Scoped SLS-native hierarchy, source-faithful suugaan, and historical
  erey-bixin conventions; pointed provenance to `correction-log.tsv` and the
  file-review folder.
- Added a current-status note linking the tracker.

### Deliberately retained

- All eight collections; the qaamuus `Aabbe` example; `ld` / `eeg` /
  homonym superscripts; the wordlist one-headword rule.
- Shared exclusions (cover, bios, TOC, page numbers, exercises,
  bibliography).
- No Ismail Mire row; no completeness claim; `docs/RESOURCES.md` not edited.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, linked overview table, eight collection subsections,
  qaamuus example, wordlist note, conventions, current status.
- 97 Markdown links; zero missing targets.
- Pre-cleanup wording `04-13-*.md`, `reorganized into topic-focused SLS
  resources`, and unscoped “not page-by-page transcriptions” is absent.
- Cleaned file size: 190 lines; 1,156 words; 10,708 bytes.
- Cleaned SHA-256:
  `fd9fb5ff6c65339fd5dcf8d563facaee4b788597036bb8a3c27d20269a6c02ff`.
