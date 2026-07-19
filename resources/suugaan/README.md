# Suugaan — Somali literature, proverbs, and wisdom

Curated Somali literary and cultural source material: proverbs (*maahmaahyo*),
wisdom (*murti*), stories (*sheeko*), poetry (*gabay*, *maanso*), school
literature, and children's texts. This is a **reference and evidence base** — not
a normative literary canon.

Content here is **Somali only**. Foreign-language passages from the original
publications (English, Italian, Norwegian, Russian, …) are omitted rather than
copied or translated.

## Layout

Flat numbered files at the collection root — no subfolders:

```
00-sources.md              Collection inventory (title, author, year)

01-maahmaahyada.md         Proverb collection (Kapchits; merged letter sections)
02-murtida.md              Wisdom and idiomatic expressions (Hadi)
03-xikmad-soomaali.md      Xikmad Soomaali (wisdom tales)

04-sheekooyin-soomaaliyeed.md
05-sheekooyin-laysku-soo-ururshay.md
06-sheekooyin-fogaan-iyo-dhowaan.md
07-hal-ka-haleel.md
08-qiso-kalgacal.md
09-rooxaan.md
10-dhaartii-dhabta-ahayd.md
11-hubsiimo-laan.md
12-bisaddii-bubaysta.md
13-xeebtii-dahabka.md

14-hal-karaan-hadrawi.md   Poetry (gabay, maanso)

15-dugsiga-fasalka-1aad-1976.md
16-dugsiga-fasalka-1aad-1983.md
17-dugsiga-fasalka-4aad.md
18-dugsiga-fasalka-4aad-buugga.md
19-dugsiga-fasalka-5aad.md

20-suugaanta-carruurta.md  Children's literature (merged qaybta I–III)
21-suugaanta-dhallaanka.md

22-suugaanta-soomaaliyeed.md   Literary reference / overview
23-ina-cabdille-xasan.md       Literary reference / overview
24-maanso-terminology.md       Maanso terminology (Chart A)
```

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
source attribution live once in `00-sources.md`, not in every file.

## Conventions

- UTF-8, LF line endings; filenames in lowercase kebab-case with two-digit
  prefixes grouped by genre (like `naxwe/` and `qaamuus/`).
- Somali text only; keep it readable. Omit unreadable OCR fragments rather than
  preserving them.
- Kapchits proverbs, Hadi wisdom, and Xikmad are kept as **separate** evidence
  files (no merged master list beyond the letter-section merge in
  `01-maahmaahyada.md`).
- Proverbs and wisdom belong here — not in `erey-bixin/` (technical terminology).

## Intended use

- Cultural and idiomatic evidence for high-context Somali comprehension.
- Seed material for proverb / idiom datasets downstream in `data/`.
- Reference for translating non-literal Somali (*maahmaah*, *murti*, literary
  register).
