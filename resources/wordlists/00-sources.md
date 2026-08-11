# Sources — wordlists

Collection inventory for the bare Somali headword baseline. Attribution is kept
here once, so content files stay clean.

## Derivation

| Attribute | Value |
| --- | --- |
| Source collection | `resources/qaamuus/` (*Qaamuuska Af-Soomaaliga*) |
| Extraction | Headwords only — definitions, codes, and examples omitted |
| Files | `01-b.md` … `26-u.md` (short-vowel letter split; long-vowel variants in qaamuus letter files) |

## Coverage notes

- **42,542** bare headwords (structural audit 2026-07-18).
- **42,511** map directly to `qaamuus/` entries; **31** heads lack a direct qaamuus
  match (normalization deferred to `data/`).
- Full dictionary provenance: see [`qaamuus/00-sources.md`](../qaamuus/00-sources.md).

## Intended use

Spellcheckers, autocomplete, tokenizers, and linguistic analysis baseline — not
a standalone lexical authority separate from `qaamuus/`.

## Phase 1 provenance status (2026-08-09)

- `01-b.md`–`26-u.md` are derived relationships; they are not an independent
  source work.
- They remain `blocked` until the exact dictionary scan is registered, the
  dictionary transcription is authenticated, and the derivation is reproduced.
- File-level hashes and relationships are recorded in
  [`data/provenance/resource-manifest.tsv`](../../data/provenance/resource-manifest.tsv).
- The 31 unmatched heads remain tracked as M-118 in
  [`METADATA_ISSUES.md`](../../docs/resource-cleanup/METADATA_ISSUES.md).

## Phase 3 source-identity review (2026-08-11)

- `SRC-QAA-001` (Yaasiin C. Keenadiid's 1976 dictionary) remains unlinked;
  it is a different book. The actual source is now verified as `SRC-QAA-002`:
  Puglielli & Mansuur, *Qaamuuska Af-Soomaaliga* (RomaTrE-Press, 2012).
  See `qaamuus/00-sources.md` for the direct page/style comparison and hash.
- This derived collection remains **blocked** for regeneration or source-based
  edits until the source's rights and the dictionary's full page mapping are
  resolved.
