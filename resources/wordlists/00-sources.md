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
