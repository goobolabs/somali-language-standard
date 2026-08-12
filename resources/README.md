# Resources — Somali Language Standard source data

The canonical collection of linguistic source materials for the Somali Language Standard (SLS) project.
This directory functions as the digital linguistic library and evidence base—containing dictionaries, grammar references, literature, and terminology resources. Source material is selected, cleaned, and reorganized into topic-focused SLS resources that form the empirical foundation for the wider ecosystem (`spec/`, `data/`, `ai/`).

## Layout

```
qaamuus/                 Monolingual Somali dictionary (full entries with definitions)
  00-sources.md          collection inventory (title, author, year)
  00-abbreviations.md    key to all grammatical codes used in entries
  01-b.md … 31-uu.md    entries split by letter in traditional Somali alphabetical
                         order (B, T, J, X, Kh, D, R, S, Sh, Dh, C, G, F, Q, K,
                         L, M, N, W, H, Y, A, E, I, O, U, AA, EE, II, OO, UU)
                         31 files total
  README.md              entry format and conventions

wordlists/               Headwords only — no definitions, no grammatical codes
  00-sources.md          collection inventory (title, author, year)
  01-b.md … 26-u.md     headwords split by letter, same traditional order
                         26 files (short vowels only; long-vowel variants are
                         covered by the qaamuus/ entries)
  README.md              format and intended uses

naxwe/                   Somali grammar reference (SLS synthesis of Aasaaska
                         Naxwaha Af Soomaaliga and Barashada Naxwaha Af
                         Soomaaliga)
  00-sources.md          collection inventory (title, author, year)
  00-luqadda-iyo-fekerka.md … 12-noocyada-weeraha.md
                         13 chapter files covering phonology, word formation,
                         noun/pronoun/numeral/verb morphology, and syntax
  13-aasaaska-naxwaha.md … 17-naxwaha-af-soomaaliga.md
                         supplementary Somali grammars (1973–school texts)
  ereyfur.md             Somali / English grammar terms + Somali explanations
  README.md              chapter map and conventions

erey-bixin/              Bilingual technical glossaries (English — Somali)
  00-sources.md          collection inventory (title, author, year)
  01-bayoolaji.md        school-science glossary — biology
  02-fisikis.md          school-science glossary — physics
  03-juqraafi.md         school-science glossary — geography
  04-kimistari.md        school-science glossary — chemistry
  05-xisaab.md           mathematics glossary
  06-wasaaradaha.md      ministry / government terminology
  07-barbaarinta-jirka.md physical-education terminology
  08-magacyada-dhirta.md  Somali plant names (scientific — Somali)
  09-farsamada-culuunta.md industrial and urban-planning terminology
  README.md              entry format and conventions

suugaan/                 Literature, proverbs, wisdom, poetry
  00-sources.md          collection inventory (title, author, year)
  01-maahmaahyada.md     proverb collection
  02-murtida.md          wisdom / idioms
  03-xikmad-soomaali.md  Xikmad Soomaali
  04-13-*.md             short stories and narratives
  14-hal-karaan-hadrawi.md  poetry (gabay, maanso)
  15-19-dugsiga-*.md     school suugaan textbooks
  20-suugaanta-carruurta.md  children's literature (merged)
  21-suugaanta-dhallaanka.md
  22-24-*.md             literary reference / overviews / maanso terminology
  README.md              entry format and conventions

orthography/             Writing, spelling, and punctuation reference
  00-sources.md          collection inventory (title, author, year)
  01-hadal-iyo-qoraal.md introduction; speech vs writing
  02-eray-kooban-hadalka.md  contracted speech forms
  03-kala-qoridda-adag.md    forms requiring word separation
  04-kala-qoridda-lama-qasban.md  forms that must stay joined
  05-astaamaynta.md      punctuation marks and usage
  06-xarafka-weyn.md     capitalization (supplement)
  README.md              chapter map and conventions

phonology/               Sound system and phonetic reference
  00-sources.md          collection inventory (title, author, year)
  01-hordhac.md          introduction; transcription method
  02-xubnaha-hadalka.md  speech organs
  03-shibbanayaasha.md   consonants
  04-shaqaallada.md      vowels
  05-codadka-sare.md     stress, pitch, juncture
  06-lahjadaha.md        dialect phonological variation
  07-xuruufta-caalamiga.md  IPA charts
  08-gariirka-iyo-spread-glottis.md  supplement — phonation
  README.md              chapter map and conventions

morphology/              Inflectional paradigm reference tables
  00-sources.md          collection inventory (title, author, year)
  01-magacyada.md        gender, number, plural patterns
  02-falalka.md          conjugation classes and verbal paradigms
  03-dhismaha-ereyga.md  affixes, derivation, nominalisation
  04-isbeddelka-codka.md morphophonological alternations
  README.md              charter and conventions
```

## Entry format (qaamuus/)

Each line in a `qaamuus/` file is one entry: headword, grammatical code(s), then
definition(s), numbered when there are several senses.

```
Aabbe m.l (-bayaal, m.l/m.dh) 1. Nin ubad dhalay. 2. (u.j) Aabbow!; wiilkaygiyow! ld aabbo.
```

- Grammatical codes (`m.dh`, `f.g1`, `mu.dhm.y`, …) are decoded in
  `qaamuus/00-abbreviations.md` (Somali / Italian / English).
- `ld` = *la mid* (same as …), `eeg` = see — cross-references to other entries.
- Superscript digits (`aa¹`, `aa²`) number homonyms and are intentional.

## Wordlist format (wordlists/)

Each line is a single bare headword — no definitions, no codes, no punctuation.
Suitable for spellcheckers, autocomplete, tokenisers, and NLP/ML training data.

## Conventions

- UTF-8, LF line endings.
- Files named in lowercase kebab-case.
- Resource files contain usable topic content, not page-by-page book or PDF
  transcriptions.
- Exclude cover text, author biographies, acknowledgements, contents pages,
  page numbers, running headers, exercises, and unrelated bibliography blocks.
- Reorganize retained content under an SLS-native topic hierarchy. Cross-link
  canonical resources instead of duplicating long explanations or paradigms.
- Remove OCR artifacts during reviewed cleanup. Record substantive changes and
  provenance in `data/` and the relevant file-review record.
