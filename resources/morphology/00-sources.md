# Sources — morphology

Collection inventory for morphological paradigm evidence.

## Primary source (curated)

| ID | Source | Authors | Year | Rights | Status |
| --- | --- | --- | --- | --- | --- |
| S-003 | *Barashada Naxwaha Af Soomaaliga* | Mansur; Puglielli | 1998 | HAAN grammar (in repo) | **primary** — paradigm tables in `01`–`04` |

Content files extract **tabular paradigms** from S-003 chapters already curated
in [`naxwe/`](../naxwe/). Pedagogical prose remains in `naxwe/` only.

## Bibliographic cross-checks (not excerpted)

| ID | Source | Publisher | Rights | Status |
| --- | --- | --- | --- | --- |
| S-301 | *Somali* (LOALL v.10) | John Benjamins | unconfirmed | permission not requested |
| S-303 | *Somali Grammar* | De Gruyter Mouton | unconfirmed | permission not requested |
| S-302 | *Somali Reference Grammar* | Dunwoody | unconfirmed | cross-check only |
| S-304 | *Referenzgrammatik des Somali* | Niemeyer | unconfirmed | bibliographic only |

See [`docs/RESOURCES.md`](../../docs/RESOURCES.md) — `morphology/` section.

## File map

| File | Topic | Primary source |
| --- | --- | --- |
| `01-magacyada.md` | Gender, number, plurals | S-003 ch. 2 → `naxwe/02` |
| `02-falalka.md` | Verbal paradigms | S-003 ch. 7–8 → `naxwe/07`, `08` |
| `03-dhismaha-ereyga.md` | Derivation and affixes | S-003 ch. 7 → `naxwe/07` |
| `04-isbeddelka-codka.md` | Morphophonology | S-003 ch. 2, 7 + `phonology/03` |

## R5.7 — Paradigm audit

Every table set traces to S-003 via the corresponding `naxwe/` chapter.

| File | Table set | `naxwe/` source | S-003 section |
| --- | --- | --- | --- |
| `01` | Cayn / waafaqid | `02` §2.2 | 2.2 |
| `01` | Gender polarity | `02` §2.3 | 2.3 |
| `01` | Plural patterns L1–L6, F | `02` §2.3.1 | 2.3.1 |
| `01` | Non-pluralising nouns | `02` §2.3 | 2.3 |
| `02` | Isrogrog I–III | `07` §7.2.1.1 | 7.2.1.1 |
| `02` | Subclasses IIA/IIB, IIIA/IIIB | `08` §8.1 | 8.1 |
| `02` | Tense *cun* | `07` §7.1.2 | 7.1.2 |
| `02` | Aspect *tag* | `07` §7.1.3 | 7.1.3 |
| `02` | Class comparison | `07` §7.2.1.1 | 7.2.1.1 |
| `02` | Negation *cun* | `08` §8.1 | 8.1 |
| `02` | Moods (3aad) | `07` §7.1.4 | 7.1.4 |
| `02` | Stative *adag* | `07` §7.2.1.2; `08` §8.2 | 7.2.1.2, 8.2 |
| `02` | Prefix verbs | `07` §7.2.1.3; `08` §8.3 | 7.2.1.3, 8.3 |
| `02` | Negation system | `08` gunaanad | 8 |
| `03` | Verbal affixes | `07` §7.3.2 | 7.3.2 |
| `03` | I→II causative | `07` §7.3.2.1 | 7.3.2.1 |
| `03` | Nominalisation | `07` §7.3.3 | 7.3.3 |
| `03` | Auxiliaries | `07` §7.2.2 | 7.2.2 |
| `03` | Transitivity | `07` §7.3.1 | 7.3.1 |
| `04` | 3rd fem *-t-* | `07` §7.1.1.3 | 7.1.1.3 |
| `04` | Causative *-y-* | `07` §7.3.2.1 | 7.3.2.1 |
| `04` | Plural phonology | `02` §2.3.1 | 2.3.1 |
| `04` | Tone-shift plurals | `02` §2.3.1 L6 | 2.3.1 |
| `04` | Prefix vowel change | `07` §7.2.1.3 | 7.2.1.3 |

## Coverage notes

- Phase 5 curation completed 2026-07-18 from S-003 (D-016).
- Saeed/Green paradigm matrices may supplement or cross-check later if
  publisher permission is obtained — do not OCR full volumes without clearance.
- Kirk 1905 (S-105) remains archive-only per Phase 3C.

## Phase 1 provenance status (2026-08-09)

- The existing stable ID `S-003` is retained for all four derived paradigm files.
- No exact S-003 PDF was found despite the earlier “in repo” note, so edition,
  pages, source checksum, and derivation cannot yet be reproduced (M-116).
- Files `01`–`04` remain `blocked`, not authenticated derivatives, until S-003
  and the cited upstream sections are registered and verified.
- Machine-readable records:
  [`sources.tsv`](../../data/provenance/sources.tsv) and
  [`resource-manifest.tsv`](../../data/provenance/resource-manifest.tsv).
