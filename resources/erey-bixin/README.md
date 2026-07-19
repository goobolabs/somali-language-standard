# Erey-bixin — Somali technical terminology glossaries

Curated bilingual (English–Somali) technical glossaries drawn from authoritative
historical *ereybixin* publications. This is a **reference and evidence base** for
Domain Editors — not a normative terminology standard. Governed records belong
downstream in `data/terminology/`.

## Layout

Flat numbered files at the collection root — no subfolders:

```
00-sources.md              Collection inventory (title, author/compiler, year)
01-bayoolaji.md            School-science glossary — biology (1987)
02-fisikis.md              School-science glossary — physics (1987)
03-juqraafi.md             School-science glossary — geography (1987)
04-kimistari.md            School-science glossary — chemistry (1987)
05-xisaab.md               School-science glossary — mathematics (1987), with
                           Mansuur Ururinta 2aad science terms folded in
06-wasaaradaha.md          Ministry / government terminology (1972)
07-barbaarinta-jirka.md    Physical-education terminology (1984)
08-magacyada-dhirta.md     Plant names (English scientific — Somali vernacular)
09-farsamada-culuunta.md   Industrial and urban-planning terminology (partial)
```

## Entry format

Each line is one concept, English term first and Somali equivalent(s) second:

```
Antibody — Lid-jidh gale
Angle, right — Xagal qumman
```

- Left: English term — scientific name (genus + epithet) for plants; otherwise the
  English source-language term as printed.
- Right: Somali equivalent(s); comma-separated variants are kept as printed.
- Plant names (`08-magacyada-dhirta.md`): left side is the accepted scientific
  binomial (author citations omitted); sections stay grouped by Somali initial
  letter.
- Sections use `##` headings by letter (`## A` … `## Z`) or by domain
  (`## Wasaaradda …`, `## Kubbadda …`) depending on the source.

## Conventions

- UTF-8, LF line endings; filenames in lowercase kebab-case with two-digit
  prefixes (like `naxwe/` and `suugaan/`).
- Content files carry a single `#` title and the term sections — no per-file
  provenance blocks. All source attribution lives once in `00-sources.md`.
- The 1987 mathematics glossary (`05-xisaab.md`) absorbs the distinct science terms
  from the Mansuur *Diiwaanka* (Ururinta 2aad); duplicate English heads are
  dropped, and the symbol table is appended as `## Sumadaha xisaabta`.
- Proverbs (*maahmaahyada*) and wisdom (*murtida*) are not terminology; they live
  in `resources/suugaan/`.

## Intended use

- Seed vocabulary for Domain Editors promoting terms into `data/terminology/`.
- Evidence for translation consistency and technical-term benchmarks.
- Citable historical grounding for AI / RAG systems.
