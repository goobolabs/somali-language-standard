# Erey-bixin — Somali technical terminology glossaries

Curated bilingual (English–Somali) technical glossaries drawn from historical
*ereybixin* publications. This collection is part of the SLS [descriptive
evidence library](../../docs/RESOURCES.md), not the normative standard.
Normative rules belong in the [`spec/` layer](../../spec/0000-index.md).
Governed terminology records belong later in `data/terminology/`; that layer
is not treated as already present from this collection.

This is a **historical bilingual glossary** collection — printed English–Somali
pairs, not a dictionary rewrite.

## Layout

Flat numbered files at the collection root — no subfolders:

| File | Role |
| --- | --- |
| [`00-sources.md`](00-sources.md) | Collection inventory (title, author/compiler, year) |
| [`01-bayoolaji.md`](01-bayoolaji.md) | School-science glossary — biology (1987) |
| [`02-fisikis.md`](02-fisikis.md) | School-science glossary — physics (1987) |
| [`03-juqraafi.md`](03-juqraafi.md) | School-science glossary — geography (1987) |
| [`04-kimistari.md`](04-kimistari.md) | School-science glossary — chemistry (1987) |
| [`05-xisaab.md`](05-xisaab.md) | School-science glossary — mathematics (1987), plus separate Ururinta 2aad science terms and symbol table |
| [`06-wasaaradaha.md`](06-wasaaradaha.md) | Ministry / government terminology (1972) |
| [`07-barbaarinta-jirka.md`](07-barbaarinta-jirka.md) | Physical-education terminology (1984) |
| [`08-magacyada-dhirta.md`](08-magacyada-dhirta.md) | Plant names (printed 1985 scientific — Somali vernacular) |
| [`09-farsamada-culuunta.md`](09-farsamada-culuunta.md) | Industrial and urban-planning terminology (partial) |
| [`README.md`](README.md) | Collection map and conventions |

## Entry format

Each line is one concept, English term first and Somali equivalent(s) second:

```
Antibody — Lid-jidh gale
Angle, right — Xagal qumman
```

- Left: English term — scientific name (genus + epithet) for plants; otherwise the
  English source-language term as printed.
- Right: Somali equivalent(s); comma-separated variants are kept as printed.
- Plant names ([`08-magacyada-dhirta.md`](08-magacyada-dhirta.md)): left side is
  the printed 1985 scientific name (author citations omitted), not a modern
  “accepted” binomial; sections stay grouped by Somali initial letter.
- Sections use `##` headings by letter (`## A` … `## Z`) or by domain
  (`## Wasaaradda …`, `## Kubbadda …`) depending on the source.
  [`05-xisaab.md`](05-xisaab.md) also has `## Ururinta 2aad (Cilmiga)` and
  `## Sumadaha xisaabta`.

## Conventions

- UTF-8, LF line endings; filenames in lowercase kebab-case with two-digit
  prefixes (like [`naxwe/`](../naxwe/) and [`suugaan/`](../suugaan/)).
- Content files carry a single `#` title, a one-line collection note linking
  [`00-sources.md`](00-sources.md), and the term sections. Author, year, and
  source attribution live once in that registry, not in content files.
- Printed coinages stay as printed. Unreadable OCR and Italian-only records
  are omitted as recorded in [`00-sources.md`](00-sources.md).
- The 1987 mathematics glossary ([`05-xisaab.md`](05-xisaab.md)) keeps the
  distinct science terms from the Mansuur *Diiwaanka* (Ururinta 2aad) as
  `## Ururinta 2aad (Cilmiga)` and the symbol table as `## Sumadaha xisaabta`,
  separate from the 1987 A–Z list; duplicate English heads across those blocks
  are kept.
- Proverbs (*maahmaahyada*) and wisdom (*murtida*) are not terminology; they live
  in [`resources/suugaan/`](../suugaan/).

## Current status

The 2026-08-19 cleanup audit and approved repairs are in
[`docs/resource-cleanup/file-reviews/erey-bixin/`](../../docs/resource-cleanup/file-reviews/erey-bixin/).
Source mapping lives in [`00-sources.md`](00-sources.md). That audit is not full
scan verification; cleanup review is still open.

## Intended use

- Seed vocabulary for Domain Editors promoting terms later into `data/terminology/`.
- Evidence for translation consistency and technical-term benchmarks.
- Citable historical grounding for AI / RAG systems.
