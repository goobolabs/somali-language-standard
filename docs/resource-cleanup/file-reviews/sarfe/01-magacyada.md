# Audit record — Magacyada

- **Resource path:** `resources/sarfe/01-magacyada.md`
- **Collection / family:** sarfe / noun paradigms
- **Priority:** P2
- **Method:** repository-only, line-by-line tabular audit against cleaned
  `naxwe/02`
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 74 lines; 468 words; 2,441 bytes
- **Resource SHA-256 at audit start:**
  `65b05435b47ce5c0f4085a796d40927c37d8e5422ec8ef9e35ac3d381a69d653`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact paradigm lookup, not a grammar chapter and not an OCR
transcript. It has one H1, seven H2 sections, seven valid Markdown tables, one
source line, and one local link to `naxwe/02`. It contains no cover matter,
exercises, scan debris, or damaged reading order.

Cleanup should keep every table and every listed example. It should remain a
tabular extract of S-003 chapter 2 via cleaned `naxwe/02`, not a second prose
grammar. It should qualify the animate-gender sentence the same way the cleaned
noun chapter does, restore the L2d lab→lab exception that the polarity table
currently omits, and add only the compact notes and links needed to stop this
file silently diverging from `naxwe/02`.

## Source and repository evidence

The file already names *Barashada Naxwaha Af Soomaaliga* (Puglielli & Mansuur,
1999), S-003 chapter 2. That is the same primary grammar now dated 1999 in
`resources/naxwe/00-sources.md`. The controlling comparison is the cleaned
noun chapter `resources/naxwe/02-sarfaha-magacyada.md`, especially §§2.2–2.3.1,
plus `resources/naxwe/03-sarfaha-tifaftireyaasha.md` for the compact determiner
sample.

Repository comparison also included:

- `resources/sarfe/00-sources.md` R5.7 rows for file `01`;
- `resources/sarfe/04-isbeddelka-codka.md` plural and L6 tables;
- `resources/naxwe/ereyfur.md` for `cayn`, `wadar`, `qodob`;
- dictionary and wordlist checks only as corroboration, not as authority to
  rewrite source wording.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-8 | reviewed | M01-R001 |
| 10-18 | reviewed | M01-R002 |
| 20-26 | reviewed | M01-R003 |
| 28-34 | reviewed | M01-R004 |
| 36-48 | reviewed | M01-R005 |
| 50-57 | reviewed | M01-R006 |
| 59-65 | reviewed | M01-R007 |
| 67-74 | reviewed | M01-R008 |
| whole file | reviewed | M01-R009 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| M01-R001 | 1-8 | Clean title, compact purpose, working naxwe link, and 1999 source line; the file correctly presents itself as a lookup table rather than a chapter / low | `resources/naxwe/02-sarfaha-magacyada.md` already points back here; `resources/naxwe/00-sources.md` and `docs/RESOURCES.md` date S-003 to 1999 | Retain the title, purpose, source line, and naxwe link. Do not expand this opening into pedagogical prose. | `repository-supported`; `intentional-retained` |
| M01-R002 | 10-18 | Agreement table matches the cleaned noun chapter, but the following sentence still equates animate gender with natural sex more categorically than that chapter now allows / medium | Cleaned `naxwe/02` §2.2: natural sex often guides relevant animate nouns, while grammatical gender is established by agreement (`-ku/-gu`, `buu`, `yahay` vs `-tu/-du`, `bay`, `tahay`); N02-R004 already required that qualification | Retain the complete three-column table and both inanimate-agreement sentences. Reword only the animate sentence so natural sex is a semantic guide, not an identity with grammatical gender. Do not add new agreement examples. | `repository-supported`; `scope-correction`; `intentional-retained` |
| M01-R003 | 20-26 | Useful three-row polarity table, but it treats monosyllabic masculine nouns as the only lab→lab class, omitting the L2d vowel-loss class that the same source keeps masculine / high | Cleaned `naxwe/02`: examples 12–14 match the three displayed rows; §2.3.1 A.II.d says `jilib → jilbo` and that this class does not change gender; the next table in this file already lists L2d | Retain all three rows and examples. Add a compact note that L2d (`jilib → jilbo`) also stays masculine. Do not invent a fourth polarity row or new nouns. | `repository-supported`; `classification-correction` |
| M01-R004 | 28-34 | Compact codkac/ending diagnostics are source-supported, but the table presents them as exceptionless identification tests / medium | Cleaned `naxwe/02` §2.2.1 scopes masculine pre-final stress, feminine final stress, heavy/light monosyllables, and `-e/-o` endings to the represented classes; N02-R005 forbids converting those observations into universal rules | Retain the three diagnostic rows. Scope them as the source's common tests, not a complete gender algorithm. Link `naxwe/02` §2.2.1 and `dhawaaq/05-codadka-sare.md`. Add no accented pairs here. | `repository-supported`; `scope-correction`; `intentional-retained` |
| M01-R005 | 36-48 | L1–L6 pattern table matches the cleaned masculine classes and keeps the supported examples; L2d, L3, L4, and L5 do not show the gender outcome that `naxwe/02` states for those classes / medium | Cleaned `naxwe/02` §2.3.1: L1 and L2d stay masculine; other `-o` masculines, `-yaal`, `-aan`, and L6 become feminine; `buugag`, `doofaarro`, `magacyo`, `gabayo`, `jilbo`, `tukayaal`, `dhagxaan`, `macallimiin`, and `awr` are the maintained compact examples | Retain all nine rows and every example. Add only compact gender-outcome notes already stated in `naxwe/02`. Do not import extra lexical rows such as `qamuun` or unresolved accent pairs. | `repository-supported`; `classification-correction`; `intentional-retained` |
| M01-R006 | 50-57 | Feminine `-o`, vowel-loss, `-y-`, and `-oyin` examples match the source, but the table does not say these plurals become masculine / low | Cleaned `naxwe/02` §2.3.1 B: feminine plurals take `-o`/`-oyin` and become masculine (`kab → kabo`, `gacan → gacmo`, `mindi → mindiyo`, `hooyo → hooyooyin`) | Retain all four rows. Add the source's lab polarity note. Do not add `dheg`, `tummaati`, or `ri'` from the longer naxwe table. | `repository-supported`; `intentional-retained` |
| M01-R007 | 59-65 | Non-plural noun classes and examples are supported; `jacayl` already uses the maintained spelling / low | Cleaned `naxwe/02` §Magacyada aan wadar yeelan; N00-R002 corrected `jaceyl` to `jacayl`; dictionary and this file agree on `sonkor`, `biyo`, `caano`, `dad`, `geel`, `carruur`, `run`, `jacayl`, `murugo` | Retain the three classes and all nine examples. Do not expand the lists from the longer naxwe inventory. | `repository-supported`; `intentional-retained` |
| M01-R008 | 67-74 | Compact determiner sample is useful, but it can look like the complete article system / low | Cleaned `naxwe/02` routes the full allomorph inventory to `naxwe/03`; the displayed forms `ninka`, `naagta`, `buugga`, `ninkan`, `naagtaas`, and `buuggaas` are ordinary examples, not the complete paradigm | Retain both rows and the closing agreement sentence. Label them as examples and link `naxwe/03`. Do not add allomorph tables. | `repository-supported`; `scope-correction`; `intentional-retained` |
| M01-R009 | whole file | Already SLS-native and free of OCR debris; it still needs the post-cleanup naxwe qualifications so the two noun resources do not disagree / medium | Seven tables, one H1, seven H2 headings, one local link, no page markers; `naxwe/02` was cleaned 2026-08-12 and still points here | Preserve the seven-section sequence and every table cell. Apply only the approved qualifications, L2d note, and links. Add no new paradigm, noun, or reconstructed accent row. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Magacyada — cayn, tirada, iyo wadar**
and keep this compact sequence:

1. purpose, 1999 source line, and link to `naxwe/02`;
2. gender-agreement table, with the cleaned naxwe qualification on animate
   nouns;
3. gender-polarity table plus the L2d lab→lab note;
4. scoped codkac/ending diagnostics;
5. masculine L1–L6 patterns with source gender outcomes;
6. feminine plural patterns with the lab polarity note;
7. nouns without ordinary plurals; and
8. a compact determiner sample linking to `naxwe/03`.

No table row or listed example should be removed. No new noun, sentence, or
accented pair should be introduced. Unresolved naxwe forms (`qamuun`, `golo`,
`gato`, `Cárab`, `órgi`) must not be imported here.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** M01-R001 through M01-R009
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `65b05435b47ce5c0f4085a796d40927c37d8e5422ec8ef9e35ac3d381a69d653`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, seven H2 headings, seven valid tables, one source
  line, one local naxwe link, no OCR page markers.

## Cleanup result and review

### Applied cleanup

- Qualified the animate-gender sentence so natural sex guides, rather than
  equals, grammatical gender.
- Added the L2d lab→lab note after the polarity table.
- Scoped the codkac diagnostics and linked `naxwe/02` §2.2.1 and
  `dhawaaq/05`.
- Added compact gender-outcome notes for L1–L6 and for the feminine classes.
- Labeled the determiner sample as examples and linked `naxwe/03`.

### Deliberately retained

- All seven tables and every listed example.
- Compact lookup format; no new nouns, sentences, or accented pairs.
- Unresolved naxwe forms (`qamuun`, `golo`, `gato`, `Cárab`, `órgi`) were not
  imported.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, seven H2 headings, seven valid tables.
- Cleaned file size: 88 lines; 526 words; 3,013 bytes.
- Cleaned SHA-256:
  `ab05b1cfea0be16e5a962c715263291eccd51f6d096f5c56154aaab9ed0c915c`.
