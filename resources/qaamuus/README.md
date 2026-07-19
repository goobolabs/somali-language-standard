# Qaamuus

This directory contains the Somali language dictionary, meticulously organized
into individual Markdown files.

Each file corresponds to a letter or long vowel from the traditional Somali alphabet. The entries include the root headword (in bold), its part of speech, full definitions, and sometimes examples. 

## Layout

```
00-sources.md              Collection inventory (title, author, year)
00-abbreviations.md        Grammatical code key (Somali / Italian / English)
01-b.md … 31-uu.md         Dictionary entries by traditional Somali alphabet order
README.md                  Entry format and conventions
```

The `00-abbreviations.md` file contains the list of grammatical abbreviations (like `m.l` for masculine noun, `f.g1` for transitive verb, etc.) used throughout the dictionary entries.

The remaining files are sorted and numbered according to the traditional Somali alphabetical order, including consonants, short vowels, and long vowels:

```text
00-sources.md
00-abbreviations.md
01-b.md
02-t.md
03-j.md
04-x.md
05-kh.md
06-d.md
07-r.md
08-s.md
09-sh.md
10-dh.md
11-c.md
12-g.md
13-f.md
14-q.md
15-k.md
16-l.md
17-m.md
18-n.md
19-w.md
20-h.md
21-y.md
22-a.md
23-e.md
24-i.md
25-o.md
26-u.md
27-aa.md
28-ee.md
29-ii.md
30-oo.md
31-uu.md
```

## Audit notes

Structural audit (2026-07-18):

- All **33** expected files present (`00-sources.md`, `00-abbreviations.md`, `01-b` … `31-uu`).
- Headword count: **42,511** entries across letter files.
- Random sample: 50 entries × 32 letter files = **1,600** lines; no replacement-character (`�`) or severe pipe/garbage runs detected in sample.
- Cross-reference tokens (`ld`, `eeg`) present as expected; broken-target analysis deferred to `data/` pipeline.

Coverage vs `wordlists/`: every wordlist headword maps to a qaamuus entry except **31** heads (likely long-vowel-only variants covered under short-vowel letter files in qaamuus). Corrections belong downstream in `data/`, not here.
