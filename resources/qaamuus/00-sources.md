# Sources — qaamuus

Collection inventory for the monolingual Somali dictionary library. Attribution
is kept here once, so content files stay clean.

## Primary source

| Attribute | Value |
| --- | --- |
| Title | *Qaamuuska Af-Soomaaliga* |
| Compiler / editor | unknown (M-015) |
| Year / edition | unknown (M-001) |
| Format | Letter-split curation in `00-abbreviations.md`, `01-b.md` … `31-uu.md` |

## File map

| Files | Content |
| --- | --- |
| `00-abbreviations.md` | Grammatical code key (Somali / Italian / English) |
| `01-b.md` … `31-uu.md` | Dictionary entries by traditional Somali alphabet order |

## Coverage notes

- **42,511** headword entries (structural audit 2026-07-18).
- Bare headword baseline derived from this collection lives in `resources/wordlists/`.
- Edition, publisher, and rights confirmation pending (M-001, M-003, M-015).
- Correct OCR/conversion defects here only from the exact authenticated source
  page and through the recorded transcription review. Linguistic normalization,
  preferred forms, and new definitions belong downstream in `data/`.

## Phase 1 provenance status (2026-08-09)

- Stable source ID: `SRC-QAA-001` for `00-abbreviations.md` and `01-b.md`–`31-uu.md`.
- The current title is provisional metadata from this inventory; no exact scan
  was present to verify its title page, edition, compiler, publisher, pages, or
  checksum.
- All source-dependent files remain `blocked`; see
  [`data/provenance/resource-manifest.tsv`](../../data/provenance/resource-manifest.tsv).
- Open evidence issues: M-001, M-003, M-015, M-100–M-104 in
  [`METADATA_ISSUES.md`](../../docs/resource-cleanup/METADATA_ISSUES.md).
