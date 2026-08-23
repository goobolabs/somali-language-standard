# Qoraal — Somali writing, spelling, and punctuation

Curated Somali orthography reference material: principles of written Somali,
word-splitting rules (speech vs writing), and punctuation (*astaamaynta*). This
collection is part of the SLS [descriptive evidence
library](../../docs/RESOURCES.md), not the normative standard. Normative rules
belong in the [`spec/` layer](../../spec/0000-index.md). Planned specs include
SLS-0002 (spelling), SLS-0004 (punctuation), and SLS-0005 (capitalization); none
of those three is treated as drafted from this collection.

Content in files [`01`](01-hadal-iyo-qoraal.md)–[`05`](05-astaamaynta.md) is
**Somali only**. File [`06-xarafka-weyn.md`](06-xarafka-weyn.md) allows a
first-use English gloss.

## Layout

| File | Role |
| --- | --- |
| [`00-sources.md`](00-sources.md) | Collection inventory (title, author, year) |
| [`01-hadal-iyo-qoraal.md`](01-hadal-iyo-qoraal.md) | Introduction; speech vs writing principles |
| [`02-eray-kooban-hadalka.md`](02-eray-kooban-hadalka.md) | Contracted forms in speech, expanded in writing |
| [`03-kala-qoridda-adag.md`](03-kala-qoridda-adag.md) | Forms that must be written as separate words |
| [`04-kala-qoridda-lama-qasban.md`](04-kala-qoridda-lama-qasban.md) | Forms that must not be split |
| [`05-astaamaynta.md`](05-astaamaynta.md) | Punctuation marks and usage |
| [`06-xarafka-weyn.md`](06-xarafka-weyn.md) | Capital letters (supplement; Nilsson §2.3) |
| [`README.md`](README.md) | Collection map and conventions |

## Content patterns

- **Principles** — readable Somali prose with `##` section headings.
- **Word-splitting rules** — rule statement plus paired examples:

```
hadal: Lana iman
qoraal: La ma iman
```

- **Punctuation** — one `###` subsection per mark: Somali name, symbol, usage,
  and verified Somali example sentences.

Each content file carries a single `#` title. Author, year, and source attribution
live once in [`00-sources.md`](00-sources.md), not in every file.

## Conventions

- UTF-8, LF line endings; filenames in lowercase kebab-case with two-digit
  prefixes grouped by topic (like `naxwe/` and `suugaan/`).
- Somali text only in files `01`–`05`; omit unreadable OCR fragments rather than
  preserving them. A first-use English gloss is allowed in file `06`.
- Alphabet inventory, vowel length, and codkac belong in
  [`naxwe/01-ereyada.md`](../naxwe/01-ereyada.md) and
  [`spec/orthography/0001-alphabet.md`](../../spec/orthography/0001-alphabet.md)
  — not duplicated here.
- Phonology (*codaynta*) is curated in [`resources/dhawaaq/`](../dhawaaq/).

## Current status

The 2026-08-19 cleanup audit and approved repairs are in
[`docs/resource-cleanup/file-reviews/qoraal/`](../../docs/resource-cleanup/file-reviews/qoraal/).
Source mapping lives in [`00-sources.md`](00-sources.md). Content files no longer
carry `## OCR Page N` markers.

## Intended use

- Empirical evidence for SLS-0002 (spelling rules) and SLS-0004 (punctuation).
- Reference for spellcheckers, editors, and translation consistency tools.
- Seed material for orthography benchmarks downstream in `data/` and `benchmarks/`.
