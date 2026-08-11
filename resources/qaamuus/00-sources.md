# Sources — qaamuus

Collection inventory for the monolingual Somali dictionary library. Attribution
is kept here once, so content files stay clean.

## Primary source

| Attribute | Value |
| --- | --- |
| Title | *Qaamuuska Af-Soomaaliga* |
| Compiler / editor | Annarita Puglielli; Cabdalla Cumar Mansuur |
| Year / edition | 2012, first edition |
| Publisher / place | RomaTrE-Press, Roma |
| ISBN | 978-88-97524-02-1 |
| Verified source | `SRC-QAA-002`, SHA-256 `28658fd204f9156ed02fb83654366f0c97faf9139682d4a1597a29b2a3ebdbaa` |
| Format | Letter-split curation in `00-abbreviations.md`, `01-b.md` … `31-uu.md` |

## File map

| Files | Content |
| --- | --- |
| `00-abbreviations.md` | Grammatical code key (Somali / Italian / English) |
| `01-b.md` … `31-uu.md` | Dictionary entries by traditional Somali alphabet order |

## Coverage notes

- **42,511** headword entries (structural audit 2026-07-18).
- Bare headword baseline derived from this collection lives in `resources/wordlists/`.
- Republication and derivative-work rights remain unresolved (M-003, M-104).
- Correct OCR/conversion defects here only from the exact authenticated source
  page and through the recorded transcription review. Linguistic normalization,
  preferred forms, and new definitions belong downstream in `data/`.

## Phase 3 source-identity review (2026-08-11)

A Phase 1 archive search matched `SRC-QAA-001` to this collection by title
string alone ("Qaamuuska Af-Soomaaliga" is reused by several distinct
Somali dictionaries). Phase 3 gold-sample comparison found this PDF is
**Yaasiin C. Keenadiid's 1976 dictionary** — a real, correctly-catalogued
scan (see the `sources.tsv` record), but not the source of this collection:
none of several sampled headwords appear in the current text, and the
current text's entry format (bold headword, grammatical-code abbreviations,
`ld` cross-references, numbered senses) does not match Keenadiid's plain
`headword (-suffix) — definition; synonyms` style.

- The actual source is now verified as Annarita Puglielli & Cabdalla Cumar
  Mansuur, *Qaamuuska Af-Soomaaliga* (RomaTrE-Press, Roma, 2012), registered
  as `SRC-QAA-002`. The downloaded public PDF has SHA-256
  `28658fd204f9156ed02fb83654366f0c97faf9139682d4a1597a29b2a3ebdbaa`.
  Its title/colophon and three sampled entries (`baraarujin`, `islaamid`, and
  `sagal`) were directly checked against this collection. M-105 is closed.
- `SRC-QAA-001` remains correctly unlinked as the distinct 1976 Keenadiid
  dictionary. `qaamuus/` correction work is still blocked by rights and full
  page-mapping requirements, not by source identity.
