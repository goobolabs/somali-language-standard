# Resources — curated evidence library

The Somali Language Standard (SLS) builds normative standards on a curated
**evidence library** in [`resources/`](../resources/). That directory holds
dictionaries, grammars, terminology glossaries, literature, and writing-system
reference material — preserved and attributed as **descriptive evidence**, not
as rules.

Live file map and entry formats: [`resources/README.md`](../resources/README.md).

**Last updated:** 2026-07-19

---

## Evidence vs normative layers

```text
resources/  →  curated source evidence (descriptive)
data/       →  structured records (later)
spec/       →  normative rules (later)
ai/         →  downstream material derived later
```

Do not hand-edit curated entries to “fix” the language. Corrections and review
status belong in downstream `data/` records. OCR artifacts in `resources/` are
preserved verbatim until structured downstream work.

Normative drafting in `spec/`, structured datasets in `data/`, and AI/benchmark
pipelines use `resources/` as input evidence. They follow the curated baseline
documented here and [`IMPLEMENTATION_PLAN.md`](../IMPLEMENTATION_PLAN.md)
Milestone 1b.

---

## Eight core collections

| Collection | Role | Content files | Status | Known limitations |
| --- | --- | ---: | --- | --- |
| `qaamuus/` | Monolingual dictionary (*Qaamuuska Af-Soomaaliga*) | 32 + abbreviations | review | Compiler, edition, republication rights incomplete |
| `wordlists/` | Bare headwords derived from `qaamuus/` | 26 | active | 31 heads without direct qaamuus match (for `data/` later) |
| `naxwe/` | Grammar and syntax reference | 18 + `ereyfur.tsv` | review | Supplements `13`–`17` carry OCR-fidelity caveat |
| `erey-bixin/` | Technical/administrative glossaries (EN→SO) | 9 (`01`–`09`) | active | `09-farsamada-culuunta.md` partial OCR |
| `suugaan/` | Proverbs, wisdom, stories, poetry, school literature | 24 | active | *Ismail Mire* deferred; variable OCR on some school texts |
| `orthography/` | Writing, word-splitting, punctuation | 6 | active | Capitalization interim supplement at `06` |
| `phonology/` | Sound system, IPA, prosody, dialect phonology | 8 | active | No audio/acoustic evidence; Orwin *Metrics* deferred |
| `morphology/` | Inflectional paradigm tables | 4 | active | Saeed/Green cross-check optional; publisher permission pending |

Each collection has `00-sources.md` (bibliographic inventory), optional
`00-<meta>.md`, numbered content files, and `README.md` (charter and format).

---

## Collection layout

Every curated collection is a **flat directory** — no content subfolders.

```text
00-sources.md              collection inventory (title, author, year)
00-<meta>.md               optional second 00-file (e.g. qaamuus/00-abbreviations.md)
01-<topic>.md … NN-<topic>.md   numbered content files
README.md                  charter, entry format, conventions
```

Conventions:

- UTF-8, LF line endings; lowercase kebab-case filenames.
- Two-digit numeric prefixes grouped by topic (as in `naxwe/`, `suugaan/`,
  `erey-bixin/`).
- One `#` title per content file; source attribution lives in `00-sources.md`.
- `**Ilaha:**` or equivalent source lines may appear in content files where a
  table or supplement needs inline traceability.

---

## Scope and boundaries

### `qaamuus/`

Monolingual dictionary evidence — headwords, grammatical codes, definitions,
cross-references (`ld`, `eeg`). Intended as seed evidence for a future
`data/lexicon/`. Does not include hand-corrected entries or new definitions.

### `wordlists/`

Bare headword lists derived from `qaamuus/` — one headword per line, no
definitions. For spellcheckers, autocomplete, tokenisers, and NLP baselines.

### `naxwe/`

Somali grammar and syntax, primarily from Puglielli & Mansuur, *Barashada
Naxwaha Af Soomaaliga* (1998), files `00`–`12`, plus five supplementary
OCR-recovered grammars (`13`–`17`). Detailed orthography and phonology live in
their own collections.

### `erey-bixin/`

Historical technical and administrative terminology glossaries (1972–2014):
`01-bayoolaji.md` … `09-farsamada-culuunta.md`. Proverbs and wisdom belong in
`suugaan/`, not here.

### `suugaan/`

Literary and cultural evidence — *maahmaah*, *murti*, sheeko, *gabay*,
school suugaan, children's literature, and literary reference. Foreign-language
passages from original publications are omitted, not translated.

### `orthography/`

Writing, word-splitting, and punctuation from *Habka Qoraalka* (1977), files
`01`–`05`. Capitalization evidence is an interim supplement in
`06-xarafka-weyn.md` (Nilsson §2.3). Alphabet inventory overlap with
`naxwe/01-ereyada.md` is intentional and not duplicated here.

### `phonology/`

Sound system reference from *Codaynta Af Soomaaliga* (1977), files `01`–`07`.
`08-gariirka-iyo-spread-glottis.md` is a Somali-first supplement from Orwin
(phonation research) and does not displace *Codaynta* as primary authority.
Orthographic punctuation lives in `orthography/05-astaamaynta.md`; prosodic
*astaamaynta* lives in `phonology/05-codadka-sare.md`.

### `morphology/`

Tabular paradigm reference extracted from the 1998 HAAN grammar (via curated
`naxwe/` chapters): gender, number, plurals, conjugation, derivation,
morphophonology. Complements pedagogical `naxwe/`; does not prescribe norms.

### Cross-collection boundaries

| Boundary | Resolved by |
| --- | --- |
| Alphabet/vowel inventory | `naxwe/01-ereyada.md` |
| Punctuation vs prosodic *astaamaynta* | `orthography/05-astaamaynta.md` vs `phonology/05-codadka-sare.md` |
| Technical terms vs proverbs/wisdom | `erey-bixin/` vs `suugaan/` |
| Full dictionary vs headword list | `qaamuus/` vs `wordlists/` |
| Grammar prose vs paradigm tables | `naxwe/` vs `morphology/` |

---

## Source attribution and traceability

- Every collection maintains `00-sources.md` with title, author, year, and a
  file map linking content files to source works.
- Content derived from another collection (e.g. `morphology/` from `naxwe/`,
  `wordlists/` from `qaamuus/`) records that derivation.
- OCR-recovered material states its fidelity posture in `00-sources.md` and
  collection READMEs. Unreadable fragments are omitted, not silently repaired.
- Missing licence or republication rights are recorded as **limitations**, not
  hidden gaps.

Representative open limitations:

| Area | Limitation |
| --- | --- |
| Dictionary | Compiler, edition, republication rights incomplete |
| `naxwe/13`–`17` | 0.9–6.3% raw OCR noise per file; review-status supplements |
| `erey-bixin/09` | Partial scan; Italian-source redistribution terms unresolved |
| Capitalization | Interim Nilsson supplement; no Somali-primary source located |
| Phonology | No audio/acoustic evidence |
| Morphology | Saeed/Green academic cross-check not excerpted without permission |

---

## Current baseline

All eight collections are curated, attributed, and documented. Each is
**accepted with limitations** — scope, source inventory, and known gaps are
recorded in collection `README.md` files and `00-sources.md`.

This baseline is descriptive evidence only. It does not prescribe rules; normative
work in `spec/` builds on it in a separate step.

### Remaining open items

| Area | Status |
| --- | --- |
| Dictionary compiler, edition, republication rights | open |
| `naxwe/13`–`17` OCR supplements | accepted with fidelity caveat |
| `erey-bixin/09` partial scan | accepted with omission policy |
| Capitalization primary source | interim supplement in `orthography/06` |
| Phonology audio/acoustic evidence | not yet sourced |
| Morphology academic cross-check | bibliographic only; no excerpts without permission |

Orthography spec drafting (Implementation Phase 2) depends on this baseline and
is tracked under Milestone 1b in [`ROADMAP.md`](../ROADMAP.md).

---

## Phase naming disambiguation

Three different “Phase 2” labels appear in this repository:

| Name | Document | Meaning |
| --- | --- | --- |
| Resources baseline | This document | Curated evidence library — accepted with limitations |
| Implementation Phase 2 | [`IMPLEMENTATION_PLAN.md`](../IMPLEMENTATION_PLAN.md) | Draft SLS-0002/0004/0005 orthography specs |
| Architecture Phase 2 | [`ARCHITECTURE.md`](ARCHITECTURE.md) §20 | “v1.0 Standard” milestone (~6 months) |
