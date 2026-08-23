# Madax-ereyo — bare Somali headwords

Bare headwords extracted from the monolingual dictionary, one per line. This
collection is part of the SLS [descriptive evidence library](../../docs/RESOURCES.md),
not the normative standard. Normative rules belong in the
[`spec/` layer](../../spec/0000-index.md).

These lists contain only root headwords (no definitions, parts of speech, or
grammar codes), making them suitable for spellcheckers, autocomplete, tokenisers,
and general linguistic analysis.

**Cleanup scope:** keep headwords aligned with [`qaamuus/`](../qaamuus/); fix
OCR orphan heads and registry navigation only. Substantive lexical changes
belong in [`qaamuus/`](../qaamuus/) and
[`data/provenance/correction-log.tsv`](../../data/provenance/correction-log.tsv).

## Layout

| File or range | Role |
| --- | --- |
| [`00-sources.md`](00-sources.md) | Collection inventory and qaamuus derivation |
| [`01-b.md`](01-b.md) … [`26-u.md`](26-u.md) | Headwords by traditional Somali alphabet order |
| [`README.md`](README.md) | Collection map and conventions |

Twenty-eight Markdown files in this directory: the registry file above, 26 letter
files, and this README. Long-vowel headwords (`aa`, `ee`, `ii`, `oo`, `uu`) live
only in [`qaamuus/27-aa.md`](../qaamuus/27-aa.md) … [`31-uu.md`](../qaamuus/31-uu.md).

## Entry format

One headword per line:

```
- baabuur
- baabuurta
```

Each content file opens with a one-line collection note linking
[`00-sources.md`](00-sources.md). Author, year, and dictionary provenance stay in
the qaamuus registry, not in every letter file.

## Conventions

- UTF-8, LF line endings; filenames match the qaamuus letter split (`01-b` …
  `26-u`).
- Headwords must stay in parity with the paired [`qaamuus/`](../qaamuus/) letter
  file after cleanup.
- Do not add definitions or grammatical codes here.

## Current status

The 2026-08-19 cleanup audit and applied repairs for letter files `01-b` …
`26-u` are in
[`docs/resource-cleanup/file-reviews/madax-ereyo/`](../../docs/resource-cleanup/file-reviews/madax-ereyo/).
Source mapping lives in [`00-sources.md`](00-sources.md). Post-cleanup parity
with qaamuus is verified for all 26 letter pairs. This collection is not marked
complete.

## Intended use

- Spellcheckers, autocomplete, and tokenizer baselines.
- Lightweight headword lookup without loading full dictionary entries.
- Upstream input for downstream structured records in `data/`.
