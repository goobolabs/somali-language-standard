# Sarfe — inflectional paradigms and morpheme evidence

Source-led **paradigm reference tables** for inflection, derivation, affixation,
and morphophonological alternation. This collection complements
[`naxwe/`](../naxwe/) (pedagogical school grammar); it does not define normative
forms.

The collection is part of the SLS [descriptive evidence
library](../../docs/RESOURCES.md), not the normative standard. Normative rules
belong in the [`spec/` layer](../../spec/0000-index.md). Saeed/Green remain
bibliographic cross-checks pending publisher permission.

## Layout

| File | Role |
| --- | --- |
| [`00-sources.md`](00-sources.md) | Source inventory and paradigm audit (R5.7) |
| [`01-magacyada.md`](01-magacyada.md) | Gender, number, plural patterns |
| [`02-falalka.md`](02-falalka.md) | Conjugation classes and tense/aspect/mood paradigms |
| [`03-dhismaha-ereyga.md`](03-dhismaha-ereyga.md) | Affixes, auxiliaries, nominalisation |
| [`04-isbeddelka-codka.md`](04-isbeddelka-codka.md) | Morphophonological alternations |
| [`README.md`](README.md) | Collection map and conventions |

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
| [`naxwe/`](../naxwe/) | Full grammar chapters — use for explanations; this collection is tables only and must stay aligned with the cleaned naxwe chapters |
| [`dhawaaq/`](../dhawaaq/) | Segment inventory — morphophonology tables cite phonology where relevant |
| [`erey-bixin/`](../erey-bixin/) | Technical terms — not inflection |

## Intended use

- Quick paradigm lookup for lemmatization and morphological analysis
- Evidence base for future structured morphology in `data/` (post Phase 9)

## Conventions

- UTF-8, LF; filenames `NN-topic.md`
- Content prose is Somali-first; a first-use English gloss is allowed
- One `#` title per file; tables in markdown
