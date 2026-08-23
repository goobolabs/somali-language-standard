# Suugaan — Somali literature, proverbs, and wisdom

Curated Somali literary and cultural source material: proverbs (*maahmaahyo*),
wisdom (*murti*), stories (*sheeko*), poetry (*gabay*, *maanso*), school
literature, and children's texts. This collection is part of the SLS
[descriptive evidence library](../../docs/RESOURCES.md), not the normative
standard. Normative rules belong in the [`spec/` layer](../../spec/0000-index.md).

This is a **reference and evidence base** — not a normative literary canon.

Content here is **Somali only**. Foreign-language passages from the original
publications (English, Italian, Norwegian, Russian, …) are omitted rather than
copied or translated.

**Cleanup scope:** preserve source-faithful Somali text; fix OCR and unreadable
fragments where uniquely supported; improve registry navigation. Do not modernize
literary Somali or rewrite stories. Substantive corrections belong in
[`data/provenance/correction-log.tsv`](../../data/provenance/correction-log.tsv).

## Layout

| File | Role |
| --- | --- |
| [`00-sources.md`](00-sources.md) | Collection inventory (title, author, year) |
| [`01-maahmaahyada.md`](01-maahmaahyada.md) | Proverb collection (Kapchits; merged letter sections) |
| [`02-murtida.md`](02-murtida.md) | Wisdom and idiomatic expressions (Hadi) |
| [`03-xikmad-soomaali.md`](03-xikmad-soomaali.md) | *Xikmad Soomaali* (wisdom tales) |
| [`04-sheekooyin-soomaaliyeed.md`](04-sheekooyin-soomaaliyeed.md) … [`13-xeebtii-dahabka.md`](13-xeebtii-dahabka.md) | Stories (*sheeko*) |
| [`14-hal-karaan-hadrawi.md`](14-hal-karaan-hadrawi.md) | Poetry (gabay, maanso) |
| [`15-dugsiga-fasalka-1aad-1976.md`](15-dugsiga-fasalka-1aad-1976.md) … [`19-dugsiga-fasalka-5aad.md`](19-dugsiga-fasalka-5aad.md) | School textbooks (*dugsiga*) |
| [`20-suugaanta-carruurta.md`](20-suugaanta-carruurta.md), [`21-suugaanta-dhallaanka.md`](21-suugaanta-dhallaanka.md) | Children's literature |
| [`22-suugaanta-soomaaliyeed.md`](22-suugaanta-soomaaliyeed.md) … [`24-ereybixinta-maansada.md`](24-ereybixinta-maansada.md) | Literary reference (*tixraac*) |
| [`README.md`](README.md) | Collection map and conventions |

Twenty-six Markdown files in this directory: the registry file above, 24 content
files, and this README.

## Content patterns

- **Proverbs / wisdom / xikmad** — one entry per line or short block:

```
Abaal nin gala waa la arkaa, abaal sow la ma arko.
```

- **`01-maahmaahyada.md`** also includes multi-line proverbs, dialogue forms
  under `## Maahmaahyo tix ka badan`, and number clichés under `## Tiroley`.

- **Stories / poetry / textbooks** — cleaned Somali prose or verse with `##`
  headings for story titles, chapters, or poem names.

Each content file carries a single `#` title and its body. Author, year, and
source attribution live once in [`00-sources.md`](00-sources.md), not in every
file.

## Conventions

- UTF-8, LF line endings; filenames in lowercase kebab-case with two-digit
  prefixes grouped by genre (like [`naxwe/`](../naxwe/) and
  [`qaamuus/`](../qaamuus/)).
- Somali text only; keep it readable. Omit unreadable OCR fragments rather than
  preserving them.
- Kapchits proverbs, Hadi wisdom, and Xikmad are kept as **separate** evidence
  files (no merged master list beyond the letter-section merge in
  [`01-maahmaahyada.md`](01-maahmaahyada.md)).
- Proverbs and wisdom belong here — not in [`erey-bixin/`](../erey-bixin/)
  (technical terminology).

## Current status

Files [`01-maahmaahyada.md`](01-maahmaahyada.md) through
[`24-ereybixinta-maansada.md`](24-ereybixinta-maansada.md), together with this
README and [`00-sources.md`](00-sources.md), completed the audit and cleanup
process. Cleanup approval was granted on 2026-08-23. Audit records are in
[`docs/resource-cleanup/file-reviews/suugaan/`](../../docs/resource-cleanup/file-reviews/suugaan/).
The collection is complete for this cleanup pass; source limitations and later
evidence-based corrections remain open.

## Intended use

- Cultural and idiomatic evidence for high-context Somali comprehension.
- Seed material for proverb / idiom datasets downstream in `data/`.
- Reference for translating non-literal Somali (*maahmaah*, *murti*, literary
  register).
