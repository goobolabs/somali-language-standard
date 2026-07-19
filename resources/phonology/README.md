# Phonology — Somali sound system reference

Curated Somali phonology reference material: speech organs, segmental sounds
(consonants and vowels), suprasegmental features (stress, pitch, juncture), dialect
phonological variation, and IPA transcription charts. This is a **reference and
evidence base** for future speech/phonology standards and STT/TTS work — not the
standard itself.

Content is **Somali-first**. IPA symbols, phonetic examples, and Somali example
words are retained as evidence. English and Italian explanatory glosses from the
original publication are omitted.

## Layout

Flat numbered files at the collection root — no subfolders:

```
00-sources.md              Collection inventory (title, author, year)

01-hordhac.md              Introduction; scope and transcription method
02-xubnaha-hadalka.md      Speech organs and articulation
03-shibbanayaasha.md       Consonants by manner class; geminates
04-shaqaallada.md          Short, long, and paired vowels
05-codadka-sare.md         Stress, pitch, and prosodic juncture
06-lahjadaha.md            Dialect phonological variation
07-xuruufta-caalamiga.md   IPA charts and transcription tables
08-gariirka-iyo-spread-glottis.md  Supplement — phonation / spread glottis (Orwin)
```

## Content patterns

- **Principles** — readable Somali prose with `##` section headings.
- **Sound descriptions** — place and manner of articulation; three-position
  notation where the source uses it (`bilow`, `dhex`, `dhammaad`).
- **Examples** — Somali words with IPA or phonetic notation where verified in
  the source.

Each content file carries a single `#` title. Author, year, and source attribution
live once in `00-sources.md`, not in every file.

## Conventions

- UTF-8, LF line endings; filenames in lowercase kebab-case with two-digit
  prefixes (like `orthography/` and `naxwe/`).
- Basic alphabet inventory and introductory codkac tables belong in
  `naxwe/01-ereyada.md` — not duplicated here.
- Orthographic punctuation (*astaamaynta* in writing) belongs in
  `orthography/05-astaamaynta.md`. Prosodic *astaamaynta* (juncture) is covered
  in `05-codadka-sare.md`.
- `08-gariirka-iyo-spread-glottis.md` is supplementary research summarized
  in Somali; primary phonology files remain the main authority.

## Intended use

- Empirical evidence for future phonology/speech standards (SLS-0600 block).
- Reference for STT/TTS, phonetic transcription, and pronunciation tools.
- Seed material for phonology datasets downstream in `data/`.

## Audit

Collection verified 2026-07-18: seven curated files from *Codaynta Af
Soomaaliga* (1977 OCR recovery, 155 pages). Supplement `08` added 2026-07-18
(Orwin phonation). See `00-sources.md` for file map, coverage gaps, and
structural audit.
