# Dhawaaq — Somali sound system reference

Curated Somali phonology reference material: speech organs, segmental sounds
(consonants and vowels), suprasegmental features (stress, pitch, juncture), dialect
phonological variation, and IPA transcription charts. This collection is part of
the SLS [descriptive evidence library](../../docs/RESOURCES.md), not the
normative standard. Normative rules belong in the
[`spec/` layer](../../spec/0000-index.md). Planned speech/phonology specs
(SLS-0600 block) are not treated as drafted from this collection.

Content is **Somali-first**. IPA symbols, phonetic examples, and Somali example
words are retained as evidence. Running English or Italian translation of the
1977 prose is omitted; a first-use technical gloss and IPA chart labels are
allowed.

## Layout

| File | Role |
| --- | --- |
| [`00-sources.md`](00-sources.md) | Collection inventory (title, author, year) |
| [`01-hordhac.md`](01-hordhac.md) | Introduction; scope and transcription method |
| [`02-xubnaha-hadalka.md`](02-xubnaha-hadalka.md) | Speech organs and articulation |
| [`03-shibbanayaasha.md`](03-shibbanayaasha.md) | Consonants by manner class; geminates |
| [`04-shaqaallada.md`](04-shaqaallada.md) | Short, long, and paired vowels |
| [`05-codadka-sare.md`](05-codadka-sare.md) | Stress, pitch, and prosodic juncture |
| [`06-lahjadaha.md`](06-lahjadaha.md) | Dialect phonological variation |
| [`07-xuruufta-caalamiga.md`](07-xuruufta-caalamiga.md) | IPA charts and transcription tables |
| [`08-gariirka-iyo-glotis-furan.md`](08-gariirka-iyo-glotis-furan.md) | Supplement — phonation / spread glottis (Orwin) |
| [`README.md`](README.md) | Collection map and conventions |

## Content patterns

- **Principles** — readable Somali prose with `##` section headings.
- **Sound descriptions** — place and manner of articulation; three-position
  notation where the source uses it (`bilow`, `dhex`, `dhammaad`).
- **Examples** — Somali words with IPA or phonetic notation where verified in
  the source.

Each content file carries a single `#` title. Author, year, and source attribution
live once in [`00-sources.md`](00-sources.md), not in every file.

## Conventions

- UTF-8, LF line endings; filenames in lowercase kebab-case with two-digit
  prefixes (like [`qoraal/`](../qoraal/) and [`naxwe/`](../naxwe/)).
- Basic alphabet inventory and introductory codkac tables belong in
  [`naxwe/01-ereyada.md`](../naxwe/01-ereyada.md) — not duplicated here.
- Orthographic punctuation (*astaamaynta* in writing) belongs in
  [`qoraal/05-astaamaynta.md`](../qoraal/05-astaamaynta.md). Prosodic
  *astaamaynta* (juncture) is covered in
  [`05-codadka-sare.md`](05-codadka-sare.md).
- [`08-gariirka-iyo-glotis-furan.md`](08-gariirka-iyo-glotis-furan.md) is
  supplementary research summarized in Somali; primary phonology files remain
  the main authority.

## Current status

The 2026-08-19 cleanup audit and approved repairs are in
[`docs/resource-cleanup/file-reviews/dhawaaq/`](../../docs/resource-cleanup/file-reviews/dhawaaq/).
Source mapping lives in [`00-sources.md`](00-sources.md). Content files no longer
carry `## OCR Page N` markers.

## Intended use

- Empirical evidence for future dhawaaq/speech standards (SLS-0600 block).
- Reference for STT/TTS, phonetic transcription, and pronunciation tools.
- Seed material for phonology datasets downstream in `data/`.
