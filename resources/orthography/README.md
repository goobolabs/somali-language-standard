# Orthography — Somali writing, spelling, and punctuation

Curated Somali orthography reference material: principles of written Somali,
word-splitting rules (speech vs writing), and punctuation (*astaamaynta*). This
is a **reference and evidence base** for drafting normative orthography standards
(SLS-0002, SLS-0004) — not the standard itself.

Content here is **Somali only**. English and Italian glosses from the original
publications are omitted.

## Layout

Flat numbered files at the collection root — no subfolders:

```
00-sources.md              Collection inventory (title, author, year)

01-hadal-iyo-qoraal.md     Introduction; speech vs writing principles
02-eray-kooban-hadalka.md  Contracted forms in speech, expanded in writing
03-kala-qoridda-adag.md    Forms that must be written as separate words
04-kala-qoridda-lama-qasban.md  Forms that must not be split
05-astaamaynta.md          Punctuation marks and usage
06-xarafka-weyn.md         Capital letters (supplement; Nilsson §2.3)
```

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
live once in `00-sources.md`, not in every file.

## Conventions

- UTF-8, LF line endings; filenames in lowercase kebab-case with two-digit
  prefixes grouped by topic (like `naxwe/` and `suugaan/`).
- Somali text only; omit unreadable OCR fragments rather than preserving them.
- Alphabet inventory, vowel length, and codkac belong in `naxwe/01-ereyada.md`
  and `spec/orthography/0001-alphabet.md` — not duplicated here.
- Phonology (*codaynta*) is curated in `resources/phonology/`.

## Audit notes

Structural audit (2026-07-18):

- All **6** content files present (`01` … `06`) plus inventory (`00-sources.md`).
- Single `#` title per file; no page-marker or external archive path references in content.
- `05-astaamaynta.md`: all **12** planned punctuation marks present in four groups.
- Source OCR: **91** pages transcribed with `## OCR Page N` markers.
- **Capitalization** (SLS-0005): `06-xarafka-weyn.md` — interim Nilsson supplement;
  1974 Ministry book archived but not a capitalization primary (see `00-sources.md`).
- Alphabet/vowel-length material intentionally omitted (see `naxwe/01-ereyada.md`).

## Intended use

- Empirical evidence for SLS-0002 (spelling rules) and SLS-0004 (punctuation).
- Reference for spellcheckers, editors, and translation consistency tools.
- Seed material for orthography benchmarks downstream in `data/` and `benchmarks/`.
