# Sources — madax-ereyo

Collection inventory for the bare Somali headword baseline. Attribution is kept
here once, so content files stay clean.

## Derivation

| Attribute | Value |
| --- | --- |
| Source collection | [`resources/qaamuus/`](../qaamuus/) (*Qaamuuska Af-Soomaaliga*) |
| Extraction | Headwords only — definitions, codes, and examples omitted |
| Files | [`01-b.md`](01-b.md) … [`26-u.md`](26-u.md) (short-vowel letter split; long-vowel entries remain in [`qaamuus/27-aa.md`](../qaamuus/27-aa.md) … [`31-uu.md`](../qaamuus/31-uu.md)) |

## File map

| Files | Content |
| --- | --- |
| [`01-b.md`](01-b.md) … [`26-u.md`](26-u.md) | Bare headwords by traditional Somali alphabet order |

## Coverage notes

- **47,502** bare headwords (recount 2026-08-19 after cleanup: lines matching
  `- ` in [`01-b.md`](01-b.md) … [`26-u.md`](26-u.md)). This is not a full scan
  verification pass.
- Paired letter files match [`qaamuus/`](../qaamuus/) headwords one-for-one for
  [`01-b`](../qaamuus/01-b.md) … [`26-u`](../qaamuus/26-u.md).
- Full dictionary provenance and the **48,119**-entry qaamuus total (including
  long-vowel letter files) are in
  [`qaamuus/00-sources.md`](../qaamuus/00-sources.md).
- The 2026-08-19 cleanup audit and applied repairs for letter files are in
  [`docs/resource-cleanup/file-reviews/madax-ereyo/`](../../docs/resource-cleanup/file-reviews/madax-ereyo/).
  Cleanup approval was granted on 2026-08-23, and the registry files are
  complete for the cleanup pass.

## Intended use

Spellcheckers, autocomplete, tokenizers, and linguistic analysis baseline — not
a standalone lexical authority separate from [`qaamuus/`](../qaamuus/).
