# Resources — Somali Language Standard source data

The linguistic evidence library for the Somali Language Standard (SLS). It
holds dictionaries, grammar references, literature, writing-system material,
and historical terminology glossaries. This directory is part of the SLS
[descriptive evidence library](../docs/RESOURCES.md), not the normative
standard. Normative rules belong in the [`spec/` layer](../spec/0000-index.md).
Structured records belong later in `data/`.

Collections are not all edited the same way:

- **SLS-native topic reference** — [`naxwe/`](naxwe/),
  [`sarfe/`](sarfe/), [`qoraal/`](qoraal/),
  [`dhawaaq/`](dhawaaq/): extract useful evidence and organize it by topic.
  These are not page-by-page book transcriptions.
- **Source-faithful literary** — [`suugaan/`](suugaan/): keep author sequence,
  dialogue, and historical spelling; fix OCR, do not modernize.
- **Historical bilingual glossaries** — [`erey-bixin/`](erey-bixin/): keep
  printed English–Somali pairs; fix uniquely supported OCR; omit unreadable or
  Italian-only rows rather than reconstructing them.
- **Dictionary collections** — [`qaamuus/`](qaamuus/) and
  [`madax-ereyo/`](madax-ereyo/) remain as listed here and are not yet in this
  cleanup pass.

## Layout

Every collection is a flat directory. Charter and file map live in that
collection’s `README.md`; attribution lives in `00-sources.md`.

| Collection | Role |
| --- | --- |
| [`qaamuus/`](qaamuus/) | Monolingual Somali dictionary (full entries with definitions) |
| [`madax-ereyo/`](madax-ereyo/) | Headwords only — no definitions, no grammatical codes |
| [`naxwe/`](naxwe/) | Somali grammar reference (SLS synthesis of *Aasaaska Naxwaha Af Soomaaliga* and *Barashada Naxwaha Af Soomaaliga*) |
| [`erey-bixin/`](erey-bixin/) | Historical bilingual technical glossaries (English — Somali) |
| [`suugaan/`](suugaan/) | Literature, proverbs, wisdom, poetry |
| [`qoraal/`](qoraal/) | Writing, spelling, and punctuation reference |
| [`dhawaaq/`](dhawaaq/) | Sound system and phonetic reference |
| [`sarfe/`](sarfe/) | Inflectional paradigm reference tables |

### `qaamuus/`

| File or range | Role |
| --- | --- |
| [`00-sources.md`](qaamuus/00-sources.md) | Collection inventory (title, author, year) |
| [`00-abbreviations.md`](qaamuus/00-abbreviations.md) | Key to grammatical codes used in entries |
| [`01-b.md`](qaamuus/01-b.md) … [`31-uu.md`](qaamuus/31-uu.md) | 31 letter files in traditional Somali alphabetical order (B, T, J, X, Kh, D, R, S, Sh, Dh, C, G, F, Q, K, L, M, N, W, H, Y, A, E, I, O, U, AA, EE, II, OO, UU) |
| [`README.md`](qaamuus/README.md) | Entry format and conventions |

### `madax-ereyo/`

| File or range | Role |
| --- | --- |
| [`00-sources.md`](madax-ereyo/00-sources.md) | Collection inventory (title, author, year) |
| [`01-b.md`](madax-ereyo/01-b.md) … [`26-u.md`](madax-ereyo/26-u.md) | 26 letter files in the same traditional order (short vowels only; long-vowel variants are in `qaamuus/`) |
| [`README.md`](madax-ereyo/README.md) | Format and intended uses |

### `naxwe/`

| File or range | Role |
| --- | --- |
| [`00-sources.md`](naxwe/00-sources.md) | Collection inventory (title, author, year) |
| [`00-luqadda-iyo-fekerka.md`](naxwe/00-luqadda-iyo-fekerka.md) … [`12-noocyada-weeraha.md`](naxwe/12-noocyada-weeraha.md) | Core grammar chapters |
| [`13-aasaaska-naxwaha.md`](naxwe/13-aasaaska-naxwaha.md) … [`17-naxwaha-af-soomaaliga.md`](naxwe/17-naxwaha-af-soomaaliga.md) | Supplementary source grammars |
| [`ereyfur.md`](naxwe/ereyfur.md) | Somali / English grammar terms with Somali explanations |
| [`README.md`](naxwe/README.md) | Chapter map and conventions |

Files `00`–`12` are the maintained core. Files `13`–`17` supplement that core
and do not silently override it or `spec/`.

### `erey-bixin/`

| File | Role |
| --- | --- |
| [`00-sources.md`](erey-bixin/00-sources.md) | Collection inventory (title, author/compiler, year) |
| [`01-bayoolaji.md`](erey-bixin/01-bayoolaji.md) | School-science glossary — biology |
| [`02-fisikis.md`](erey-bixin/02-fisikis.md) | School-science glossary — physics |
| [`03-juqraafi.md`](erey-bixin/03-juqraafi.md) | School-science glossary — geography |
| [`04-kimistari.md`](erey-bixin/04-kimistari.md) | School-science glossary — chemistry |
| [`05-xisaab.md`](erey-bixin/05-xisaab.md) | Mathematics glossary, plus separate Ururinta 2aad science terms and symbol table |
| [`06-wasaaradaha.md`](erey-bixin/06-wasaaradaha.md) | Ministry / government terminology |
| [`07-barbaarinta-jirka.md`](erey-bixin/07-barbaarinta-jirka.md) | Physical-education terminology |
| [`08-magacyada-dhirta.md`](erey-bixin/08-magacyada-dhirta.md) | Somali plant names (printed 1985 scientific — Somali) |
| [`09-farsamada-culuunta.md`](erey-bixin/09-farsamada-culuunta.md) | Industrial and urban-planning terminology (partial) |
| [`README.md`](erey-bixin/README.md) | Entry format and conventions |

### `suugaan/`

| File or range | Role |
| --- | --- |
| [`00-sources.md`](suugaan/00-sources.md) | Collection inventory (title, author, year) |
| [`01-maahmaahyada.md`](suugaan/01-maahmaahyada.md) | Proverb collection |
| [`02-murtida.md`](suugaan/02-murtida.md) | Wisdom / idioms |
| [`03-xikmad-soomaali.md`](suugaan/03-xikmad-soomaali.md) | Xikmad Soomaali |
| [`04-sheekooyin-soomaaliyeed.md`](suugaan/04-sheekooyin-soomaaliyeed.md) … [`13-xeebtii-dahabka.md`](suugaan/13-xeebtii-dahabka.md) | Short stories and narratives |
| [`14-hal-karaan-hadrawi.md`](suugaan/14-hal-karaan-hadrawi.md) | Poetry (*gabay*, *maanso*) |
| [`15-dugsiga-fasalka-1aad-1976.md`](suugaan/15-dugsiga-fasalka-1aad-1976.md) … [`19-dugsiga-fasalka-5aad.md`](suugaan/19-dugsiga-fasalka-5aad.md) | School suugaan textbooks |
| [`20-suugaanta-carruurta.md`](suugaan/20-suugaanta-carruurta.md) | Children's literature (merged) |
| [`21-suugaanta-dhallaanka.md`](suugaan/21-suugaanta-dhallaanka.md) | Children's literature |
| [`22-suugaanta-soomaaliyeed.md`](suugaan/22-suugaanta-soomaaliyeed.md) | Literary reference / overview |
| [`23-ina-cabdille-xasan.md`](suugaan/23-ina-cabdille-xasan.md) | Literary reference / overview |
| [`24-ereybixinta-maansada.md`](suugaan/24-ereybixinta-maansada.md) | Maanso terminology |
| [`README.md`](suugaan/README.md) | Entry format and conventions |

### `qoraal/`

| File | Role |
| --- | --- |
| [`00-sources.md`](qoraal/00-sources.md) | Collection inventory (title, author, year) |
| [`01-hadal-iyo-qoraal.md`](qoraal/01-hadal-iyo-qoraal.md) | Introduction; speech vs writing |
| [`02-eray-kooban-hadalka.md`](qoraal/02-eray-kooban-hadalka.md) | Contracted speech forms |
| [`03-kala-qoridda-adag.md`](qoraal/03-kala-qoridda-adag.md) | Forms requiring word separation |
| [`04-kala-qoridda-lama-qasban.md`](qoraal/04-kala-qoridda-lama-qasban.md) | Forms that must stay joined |
| [`05-astaamaynta.md`](qoraal/05-astaamaynta.md) | Punctuation marks and usage |
| [`06-xarafka-weyn.md`](qoraal/06-xarafka-weyn.md) | Capitalization (supplement) |
| [`README.md`](qoraal/README.md) | Chapter map and conventions |

### `dhawaaq/`

| File | Role |
| --- | --- |
| [`00-sources.md`](dhawaaq/00-sources.md) | Collection inventory (title, author, year) |
| [`01-hordhac.md`](dhawaaq/01-hordhac.md) | Introduction; transcription method |
| [`02-xubnaha-hadalka.md`](dhawaaq/02-xubnaha-hadalka.md) | Speech organs |
| [`03-shibbanayaasha.md`](dhawaaq/03-shibbanayaasha.md) | Consonants |
| [`04-shaqaallada.md`](dhawaaq/04-shaqaallada.md) | Vowels |
| [`05-codadka-sare.md`](dhawaaq/05-codadka-sare.md) | Stress, pitch, juncture |
| [`06-lahjadaha.md`](dhawaaq/06-lahjadaha.md) | Dialect phonological variation |
| [`07-xuruufta-caalamiga.md`](dhawaaq/07-xuruufta-caalamiga.md) | IPA charts |
| [`08-gariirka-iyo-glotis-furan.md`](dhawaaq/08-gariirka-iyo-glotis-furan.md) | Supplement — phonation |
| [`README.md`](dhawaaq/README.md) | Chapter map and conventions |

### `sarfe/`

| File | Role |
| --- | --- |
| [`00-sources.md`](sarfe/00-sources.md) | Collection inventory (title, author, year) |
| [`01-magacyada.md`](sarfe/01-magacyada.md) | Gender, number, plural patterns |
| [`02-falalka.md`](sarfe/02-falalka.md) | Conjugation classes and verbal paradigms |
| [`03-dhismaha-ereyga.md`](sarfe/03-dhismaha-ereyga.md) | Affixes, derivation, nominalisation |
| [`04-isbeddelka-codka.md`](sarfe/04-isbeddelka-codka.md) | Morphophonological alternations |
| [`README.md`](sarfe/README.md) | Charter and conventions |

## Entry format (`qaamuus/`)

Each line in a [`qaamuus/`](qaamuus/) file is one entry: headword, grammatical
code(s), then definition(s), numbered when there are several senses.

```
Aabbe m.l (-bayaal, m.l/m.dh) 1. Nin ubad dhalay. 2. (u.j) Aabbow!; wiilkaygiyow! ld aabbo.
```

- Grammatical codes (`m.dh`, `f.g1`, `mu.dhm.y`, …) are decoded in
  [`qaamuus/00-abbreviations.md`](qaamuus/00-abbreviations.md) (Somali / Italian /
  English).
- `ld` = *la mid* (same as …), `eeg` = see — cross-references to other entries.
- Superscript digits (`aa¹`, `aa²`) number homonyms and are intentional.

## Wordlist format (`madax-ereyo/`)

Each line in [`madax-ereyo/`](madax-ereyo/) is a single bare headword — no
definitions, no codes, no punctuation. Suitable for spellcheckers, autocomplete,
tokenisers, and NLP/ML training data.

## Conventions

- UTF-8, LF line endings; filenames in lowercase kebab-case.
- Shared exclusions: cover text, author biographies, acknowledgements, contents
  pages, page numbers, running headers, exercises, and unrelated bibliography
  blocks.
- [`naxwe/`](naxwe/), [`sarfe/`](sarfe/),
  [`qoraal/`](qoraal/), and [`dhawaaq/`](dhawaaq/) are
  topic-focused SLS references. Cross-link canonical files instead of
  duplicating long explanations or paradigms.
- [`suugaan/`](suugaan/) keeps source sequence and historical spelling. Fix OCR;
  do not modernize.
- [`erey-bixin/`](erey-bixin/) keeps printed term pairs. Fix uniquely supported
  OCR; omit unreadable or Italian-only rows as recorded in
  [`erey-bixin/00-sources.md`](erey-bixin/00-sources.md).
- Substantive cleanup is recorded in
  [`data/provenance/correction-log.tsv`](../data/provenance/correction-log.tsv)
  and the relevant record under
  [`docs/resource-cleanup/file-reviews/`](../docs/resource-cleanup/file-reviews/).

## Current status

Cleanup progress for this directory is in
[`RESOURCE_CLEANUP_TRACKER.md`](../RESOURCE_CLEANUP_TRACKER.md). No collection
is marked complete from this file. `docs/RESOURCES.md` remains the
evidence-versus-norm overview; it is not updated by this cleanup.
