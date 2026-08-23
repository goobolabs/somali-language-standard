# Qaamuus — monolingual Somali dictionary

Monolingual dictionary evidence: headwords, grammatical codes, definitions, and
cross-references (`ld`, `eeg`). This collection is part of the SLS
[descriptive evidence library](../../docs/RESOURCES.md), not the normative
standard. Normative rules belong in the [`spec/` layer](../../spec/0000-index.md).
It is intended as seed evidence for a future `data/lexicon/`.

**Cleanup scope:** preserve headwords, codes, definitions, and cross-references;
fix OCR and unreadable fragments where uniquely supported; improve registry
navigation. Do not rewrite definitions or modernize Somali in letter files.
Substantive corrections belong in
[`data/provenance/correction-log.tsv`](../../data/provenance/correction-log.tsv).

Each letter file corresponds to a consonant, short vowel, or long vowel from the
traditional Somali alphabet. Entries include the root headword (in bold), its
part of speech, numbered senses, and sometimes examples.

## Layout

| File or range | Role |
| --- | --- |
| [`00-sources.md`](00-sources.md) | Collection inventory (title, compiler, year) |
| [`00-abbreviations.md`](00-abbreviations.md) | Key to grammatical and domain codes |
| [`01-b.md`](01-b.md) … [`31-uu.md`](31-uu.md) | 31 letter files in traditional Somali alphabetical order (B, T, J, X, Kh, D, R, S, Sh, Dh, C, G, F, Q, K, L, M, N, W, H, Y, A, E, I, O, U, AA, EE, II, OO, UU) |
| [`README.md`](README.md) | Collection map and conventions |

Thirty-four Markdown files in this directory: the two registry files above, 31
letter files, and this README.

## Entry format

Each line is one entry: headword, grammatical code(s), then definition(s).

```
Aabbe m.l (-bayaal, m.l/m.dh) 1. Nin ubad dhalay. 2. (u.j) Aabbow!; wiilkaygiyow! ld aabbo.
```

- Codes (`m.dh`, `f.g1`, `mu.dhm.y`, …) are decoded in
  [`00-abbreviations.md`](00-abbreviations.md).
- `ld` = *la mid* (same as …), `eeg` = see — cross-references to other entries.
- Superscript digits (`aa¹`, `aa²`) number homonyms and are intentional.

Full collection conventions and the bare-headword derivative
[`madax-ereyo/`](../madax-ereyo/) are documented in
[`resources/README.md`](../README.md).

## Current status

The 2026-08-19 cleanup audit and approved registry repairs are in
[`docs/resource-cleanup/file-reviews/qaamuus/`](../../docs/resource-cleanup/file-reviews/qaamuus/).
Source mapping lives in [`00-sources.md`](00-sources.md). Letter files
[`01-b.md`](01-b.md) … [`31-uu.md`](31-uu.md) received the 2026-08-19 cleanup
audit and applied repairs (awaiting cleanup review). Broken `ld` / `eeg` target
analysis remains deferred to the `data/` pipeline. This collection is not
marked complete.

Bare headwords extracted from this dictionary live in
[`madax-ereyo/`](../madax-ereyo/) — see
[`madax-ereyo/00-sources.md`](../madax-ereyo/00-sources.md).

## Intended use

- Lexicon seed and cross-collection headword lookup.
- Upstream source for [`madax-ereyo/`](../madax-ereyo/) (spellcheckers, autocomplete,
  tokenisers, NLP baselines).
- Reference material for downstream structured records in `data/`.
