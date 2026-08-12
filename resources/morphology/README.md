# Morphology — inflectional paradigms and morpheme evidence

Source-led **paradigm reference tables** for inflection, derivation, affixation,
and morphophonological alternation. This collection complements
[`naxwe/`](../naxwe/) (pedagogical school grammar); it does not define normative
forms.

**Status (2026-07-18):** R5.4–R5.7 curated from S-003 (Puglielli & Mansuur 1999)
as a tabular reference layer. Saeed/Green remain bibliographic cross-checks
pending publisher permission.

## Layout

```
00-sources.md           Source inventory and paradigm audit (R5.7)
01-magacyada.md         Gender, number, plural patterns
02-falalka.md           Conjugation classes and tense/aspect/mood paradigms
03-dhismaha-ereyga.md   Affixes, auxiliaries, nominalisation
04-isbeddelka-codka.md  Morphophonological alternations
README.md               This file
```

## Charter

### Include

- Paradigm tables with Somali example forms
- Morpheme boundaries and labels where applicable

### Exclude

- Long pedagogical prose duplicated from [`naxwe/02`–`08`](../naxwe/)
- Syntax and sentence patterns (`naxwe/09`–`12`)
- Normative conjugation rules or guessed roots
- Full OCR of copyrighted reference books
- English exposition copied from academic grammars

## Boundaries

| Neighbour | Rule |
| --- | --- |
| `naxwe/` | Full grammar chapters — use for explanations; this collection is tables only |
| `phonology/` | Segment inventory — morphophonology tables cite phonology where relevant |
| `erey-bixin/` | Technical terms — not inflection |

## Intended use

- Quick paradigm lookup for lemmatization and morphological analysis
- Evidence base for future structured morphology in `data/` (post Phase 9)

## Conventions

- UTF-8, LF; filenames `NN-topic.md`
- Somali-first
- One `#` title per file; tables in markdown
