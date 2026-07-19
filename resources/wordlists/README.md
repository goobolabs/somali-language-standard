# Wordlists

A collection of Somali headwords extracted from the dictionary, formatted as simple markdown lists. 

These lists contain only the bare root words (without definitions, parts of speech, or grammar explanations), making them ideal for spellcheckers, autocomplete systems, machine learning, and general linguistic analysis.

## Layout

```
00-sources.md              Collection inventory (title, author, year)
01-b.md … 26-u.md          Headwords by traditional Somali alphabet order
README.md                  Format and intended uses
```

The files are split by starting letter and numbered according to the traditional Somali alphabetical order:

```text
00-sources.md
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
```

## Audit notes

Structural audit (2026-07-18):

- All **27** expected files present (`00-sources.md`, `01-b` … `26-u`; long-vowel
  variants intentionally omitted — covered by `qaamuus/` entries).
- Headword count: **42,542** bare heads.
- Coverage vs `qaamuus/`: **42,511** heads match qaamuus entries; **31** wordlist-only
  heads lack a direct qaamuus match (review during `data/` normalization).
- No content edits performed — OCR artifacts preserved verbatim per collection policy.
