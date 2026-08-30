---
id: "0001"
sls_id: SLS-0001
title: Somali Alphabet Standard
version: 0.1.1
standard_version: 0.1.1
status: Proposed
category: foundation
owner: language-council
reviewers: []
dependencies: []
implements:
  - spec/orthography/0001-alphabet.md
publication_date: null
supersedes: null
superseded_by: null
revision_history:
  - version: 0.1.0
    date: 2026-07-10
    change: Initial draft — letter inventory, collation order, digraphs, glottal stop, vowel length, exclusions
  - version: 0.1.1
    date: 2026-08-30
    change: "Editorial: corrected the Scope cross-reference to R12; wrote the glottal stop as canonical U+02BC in prose; linked the public review log"
---

<!--
SLS-0001 is the root of the entire SLS dependency tree: every other standard
depends on it transitively. It defines the Somali Latin alphabet (Far Soomaali,
the official orthography adopted in 1972) as a fixed inventory with a canonical
ordering and a defined character set, so that all downstream standards, schemas,
and tooling share one unambiguous notion of "a Somali letter."

This document is simultaneously local spec note 0001 (ARCHITECTURE.md §5) and
global standard SLS-0001 (§30); it therefore carries both `id` and `sls_id`.
-->

## Abstract

This standard defines the Somali alphabet in its official Latin orthography
(*Far Soomaali*, adopted 21 October 1972): the complete inventory of letters,
their canonical ordering (collation), the digraphs, the representation of the
glottal stop, the marking of vowel length, and the Unicode character set used to
encode Somali text. It is the foundational standard on which Somali orthography
(SLS-0002), grammar (SLS-0003), and every lexical and terminological standard
depend.

## Purpose

Somali NLP tooling, dictionaries, and datasets today disagree on basic questions:
how many letters the alphabet has, how they sort, whether `dh` is one unit or
two, and which character encodes the glottal stop. These disagreements corrupt
sorting, deduplication, spellchecking, and tokenization. SLS-0001 fixes a single
canonical answer so that every SLS record — and every external system claiming
SLS conformance — encodes and orders Somali text identically.

## Scope

**In scope:** the letter inventory of standard written Somali; canonical
collation order; the three digraphs; the glottal stop; vowel-length notation;
letters excluded from native orthography; and the Unicode code points used.

**Out of scope:**

- **Phonology beyond letter identity** — full phonemic description, allophony,
  and syllable structure belong to a future Speech & Phonology standard
  (`SLS-0600` block).
- **Tone and stress** — Somali is a tonal-accent language, but tone is **not**
  marked in the standard orthography; its notation is out of scope here.
- **Spelling rules** — which letter to write in a given word (gemination,
  assimilation, vowel harmony spelling) is governed by SLS-0002.
- **Capitalization and punctuation rules** — governed by SLS-0005 and SLS-0004
  respectively; this standard fixes only the *case pairing* of digraphs (R12).
- **Non-Latin scripts** — Osmanya, Wadaad, and Borama writing are reserved for
  the Historical & Alternate Scripts block (`SLS-0800`).
- **Dialectal inventories** — Maay and other varieties are reserved for the
  Dialects block (`SLS-0700`).

## Definitions

- **Letter** — a grapheme that is a member of the Somali alphabet, either a
  single character or one of the three digraphs.
- **Digraph** — a letter written with two characters (`kh`, `dh`, `sh`) that
  represents a single sound and collates as a single unit.
- **Glottal stop (hamza)** — the consonant /ʔ/, written `ʼ`; see R6–R7.
- **Collation** — the canonical sort order of letters (R3).
- **Vowel length** — the short/long distinction, written by doubling the vowel
  letter (R8).
- **Standard Somali** — the `so` variety (BCP 47); the reference for this
  standard.

## Normative Requirements

### Inventory

- **R1.** Standard written Somali **MUST** use the Latin-based official
  orthography (*Far Soomaali*, 1972). It comprises **26 base letters — 21
  consonants and 5 vowels** — plus the glottal-stop sign (R6).
- **R2.** The 26 base letters are exactly:

  **Consonants (21):** `b t j x kh d r s sh dh c g f q k l m n w h y`
  **Vowels (5):** `a e i o u`

  No other base letters are part of the alphabet (see R9 for exclusions).

### Collation order

- **R3.** The canonical collation order **MUST** be the traditional Somali order
  (glottal stop first, then consonants, then vowels):

  ```
  ʼ  b  t  j  x  kh  d  r  s  sh  dh  c  g  f  q  k  l  m  n  w  h  y  a  e  i  o  u
  ```

- **R4.** Sorting **MUST** treat each digraph (`kh`, `dh`, `sh`) as a single
  collation element occupying its position above — not as its two component
  letters. For example, `sh` sorts as one unit after `s`, not among `s…h`
  sequences.

### Phonetic reference (informative anchor)

- **R5.** Each letter's reference sound is as follows. (The IPA column is
  informative; the letter identities and ordering in R2–R3 are normative.)

  | Letter | Code point(s) | IPA | Notes |
  |---|---|---|---|
  | `ʼ` | U+02BC (see R6) | /ʔ/ | glottal stop (hamza) |
  | `b` | U+0062 | /b/ | |
  | `t` | U+0074 | /t/ | dental |
  | `j` | U+006A | /dʒ/ | |
  | `x` | U+0078 | /ħ/ | voiceless pharyngeal fricative |
  | `kh` | U+006B U+0068 | /χ/ | mainly Arabic loanwords |
  | `d` | U+0064 | /d/ | dental |
  | `r` | U+0072 | /r/ | trill |
  | `s` | U+0073 | /s/ | |
  | `sh` | U+0073 U+0068 | /ʃ/ | |
  | `dh` | U+0064 U+0068 | /ɖ/ | retroflex |
  | `c` | U+0063 | /ʕ/ | voiced pharyngeal fricative (Arabic ʿayn) |
  | `g` | U+0067 | /ɡ/ | |
  | `f` | U+0066 | /f/ | |
  | `q` | U+0071 | /q/ | uvular (often realized [ɢ]) |
  | `k` | U+006B | /k/ | |
  | `l` | U+006C | /l/ | |
  | `m` | U+006D | /m/ | |
  | `n` | U+006E | /n/ | |
  | `w` | U+0077 | /w/ | |
  | `h` | U+0068 | /h/ | |
  | `y` | U+0079 | /j/ | |
  | `a` | U+0061 | /a/ | |
  | `e` | U+0065 | /e/ | |
  | `i` | U+0069 | /i/ | |
  | `o` | U+006F | /o/ | |
  | `u` | U+0075 | /u/ | |

### Glottal stop

- **R6.** The canonical character for the glottal stop **MUST** be **U+02BC
  MODIFIER LETTER APOSTROPHE (`ʼ`)** in normative SLS data and released text.
  This is a letter (Unicode general category `Lm`), so it does not break word
  tokenization the way a punctuation apostrophe does.

- **R7.** Tools **SHOULD** accept U+0027 (`'`) and U+2019 (`’`) on input as
  aliases for the glottal stop and normalize them to the canonical U+02BC on
  ingest. Released SLS records **MUST NOT** contain U+0027 or U+2019 as a glottal
  stop.

### Vowel length

- **R8.** Vowel length **MUST** be written by doubling the vowel letter: short
  `a e i o u` versus long `aa ee ii oo uu`. Diacritics (macron, circumflex,
  acute) **MUST NOT** be used to mark length. Consonant gemination is likewise
  written by doubling the consonant (its spelling conditions are governed by
  SLS-0002, not here).

### Digraph integrity

- **R9.** The digraphs `kh`, `dh`, `sh` **MUST** be encoded as sequences of their
  two component ASCII letters. Precomposed or single-code-point substitutes
  **MUST NOT** be used.
- **R10.** A digraph is a single *letter* for the purposes of collation (R4),
  counting, and hyphenation, even though it is two *characters*.

### Excluded letters

- **R11.** The Latin letters `p`, `v`, `z`, and `ñ` are **not** part of the
  Somali alphabet and **MUST NOT** appear in native Somali words. Where they
  occur in unadapted foreign proper nouns or unassimilated loanwords, writers
  **SHOULD** adapt them to Somali phonology (commonly `p → b`, `v → f`,
  `z → s`, `ñ → ny`). SLS-0002 and SLS-0103 (Loanword Standard) govern specific
  adaptations.

### Case pairing of digraphs

- **R12.** When a digraph is capitalized, **only its first character** is
  uppercased: `Sh`, `Dh`, `Kh` — never `SH`, `DH`, `KH` in ordinary running
  text. (When and where capitalization applies is governed by SLS-0005; this
  requirement fixes only the *form* a capitalized digraph takes.)

## Recommendations

- Datasets SHOULD store Somali text in Unicode Normalization Form C (NFC),
  though for the ASCII-plus-U+02BC repertoire of this standard NFC and NFD are
  equivalent.
- Sorting implementations SHOULD encode R3–R4 as an explicit collation table
  rather than relying on a locale's default Latin collation, which will sort
  digraphs incorrectly.
- Tokenizers SHOULD treat U+02BC as a word-internal letter, not a boundary.

## Examples

| Somali | Note | Status |
|---|---|---|
| `baabuur` | doubled `aa` = long vowel (R8) | ✅ |
| `bābūr` | macron used for length | ❌ (R8) |
| `shaqo` | `sh` digraph, one letter (R9–R10) | ✅ |
| `dhagax` | `dh` retroflex; final `x` = /ħ/ | ✅ |
| `suʼaal` | glottal stop stored as canonical U+02BC (R6) | ✅ |
| `caano` | initial `c` = /ʕ/ (not English /k/) | ✅ |
| `pizza` | contains `p`, `z` — not native letters (R11) | ❌ |
| `Sheeko` | capitalized digraph `Sh`, first char only (R12) | ✅ |
| `SHeeko` | both chars of digraph uppercased | ❌ (R12) |

### Per-rule example coverage

| Rule | Positive example | Negative example |
|---|---|---|
| R1 | `Soomaali` uses the official Latin-based system. | A native Somali record written in another script cannot claim R1 conformance. |
| R2 | `shaqo` contains only listed base letters. | `pizza` contains excluded base letters when presented as a native Somali word. |
| R3 | `ʼ, b, t, j, x, kh` follows the opening canonical order. | `b, j, t` does not follow the canonical order. |
| R4 | A sort key treats `sh` as one element after `s`. | A sort key separates `sh` into independent `s` and `h` elements. |
| R5 | `caano`: `c` has the reference value /ʕ/. | Reading Somali `c` as English /k/ contradicts the reference identity. |
| R6 | `suʼaal` stores U+02BC internally. | `su'aal` stores U+0027 as the released glottal-stop letter. |
| R7 | Input `su'aal` is accepted and normalized to `suʼaal`. | An alias is emitted unchanged as the released canonical spelling. |
| R8 | `baabuur` writes long `aa` and `uu` by doubling. | `bābūr` uses macrons to encode vowel length. |
| R9 | `sh` is stored as U+0073 U+0068. | `š` substitutes a single non-inventory code point for `sh`. |
| R10 | A counter reports the initial `sh` in `shaqo` as one letter. | A hyphenator splits the initial digraph as `s-haqo`. |
| R11 | `baabuur` uses only Somali letters as an adapted word. | `pizza` is labelled a native Somali word without adaptation or a foreign-use label. |
| R12 | `Sheeko` uses the capitalized form `Sh`. | `SHeeko` capitalizes both digraph characters in ordinary text. |

**Collation example.** The words `sac`, `shan`, `sonkor` sort as
`sac → sonkor → shan`, because `sh` (R4) sorts as a unit *after* all plain-`s`
words, not between them.

## Edge Cases

- **`sh` vs. `s`+`h` across a morpheme boundary.** A genuine `s`+`h` sequence
  (rare, e.g. at a compound seam) is still two letters; the digraph reading is
  the default and any true sequence must be resolvable from morphology (SLS-0003)
  — flagged here as a known ambiguity for SLS-0002 to address in spelling rules.
- **Word-initial glottal stop.** Vowel-initial words phonetically begin with a
  glottal onset; the standard orthography does **not** write an initial `ʼ`
  (e.g. `af`, not `ʼaf`). The `ʼ` is written only where contrastive intervocalic
  or coda glottal stops occur (e.g. `suʼaal`).
- **`kh` in native words.** `kh` occurs almost exclusively in Arabic loanwords;
  its presence is a useful loanword signal but does not change its status as a
  full alphabet letter (R2).
- **Uppercase-only contexts** (headlines, acronyms): where surrounding text is
  all-caps, a digraph may appear as `SH`; this is a rendering/style concern for
  SLS-0005, and does not license `SH` in ordinary running text (R12).

## Compliance Requirements

| # | Requirement | Traces to | Level |
|---|---|---|---|
| C1 | Text uses only the 26 base letters + glottal stop; no excluded letters in native words | R1, R2, R11 | MUST |
| C2 | Collation orders letters per R3 with digraphs as single units | R3, R4 | MUST |
| C3 | Digraphs encoded as two ASCII characters, never precomposed | R9 | MUST |
| C4 | Digraphs counted/collated/hyphenated as single letters | R4, R10 | MUST |
| C5 | Glottal stop stored as canonical U+02BC in released records | R6, R7 | MUST |
| C6 | Vowel length written by doubling; no length diacritics | R8 | MUST |
| C7 | Capitalized digraphs uppercase only the first character | R12 | MUST |
| C8 | Input aliases for glottal stop normalized to canonical form | R7 | SHOULD |

## References

- `docs/ARCHITECTURE.md` §5, §8, §30 — spec-layer and standards-framework rules.
- SLS-0002 Somali Orthography Standard *(`Proposed`; depends on this standard)*.
- SLS-0003 Somali Grammar Standard *(`Proposed`; depends on this standard and SLS-0002)*.
- SLS-0004 Somali Punctuation Standard *(`Proposed`; depends indirectly on this standard)*.
- SLS-0005 Somali Capitalization Standard *(`Proposed`; depends indirectly on this standard)*.
- SLS-0103 Loanword Standard *(planned)* — foreign-letter adaptation.
- BCP 47 — language tags (`so` for standard Somali).
- The Unicode Standard — code point references (Basic Latin; U+02BC).
- Republic of Somalia, official adoption of the Latin script for Somali, 1972.
- [`SLS-0001 review log`](../../docs/standards/SLS-0001-review-log.md) — the public-comment record for this standard.

## Revision History

| Version | Date | Change |
|---|---|---|
| 0.1.0 | 2026-07-10 | Initial draft — inventory, collation, digraphs, glottal stop, vowel length, exclusions |
| 0.1.1 | 2026-08-30 | Editorial: corrected the Scope cross-reference to R12; wrote the glottal stop as canonical U+02BC in prose; linked the public review log |
