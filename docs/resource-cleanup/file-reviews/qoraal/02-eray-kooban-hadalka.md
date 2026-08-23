# Audit record — Ereyada Afkeenna Koobma Marka Hadalka

- **Resource path:** `resources/qoraal/02-eray-kooban-hadalka.md`
- **Collection / family:** qoraal / contracted speech forms
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; SLS-native cleanup applied and awaiting
  maintainer cleanup review
- **Audit started:** 2026-08-19
- **File size at audit start:** 274 lines; 987 words; 5,482 bytes
- **Resource SHA-256 at audit start:**
  `e56c608990f3dada0685a634da0b3a5897b301a4b03445f87cba9c50ab321f72`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact contraction-expansion lookup, not an OCR transcript. It
has one H1, six H2 sections, eight H3 subsections, two valid Markdown tables,
many fenced example blocks, and no cover matter, exercises, or page markers.

Cleanup should keep it as the maintained list of speech contractions that
writing expands. It should repair the `lagama maarmaan` split already approved
in file `01`, remove the identity row that is not a contraction, restore the
two `aannu` expansions that the matching koobnaan lines and section 1 already
display, and add links to `01`, `03`, `04`, and naxwe 14. It must not invent
expansions for damaged table cells, move sections 6–8 into file `04`, or
collapse the two `dambee` or two `waxaan` readings.

## Source and repository evidence

`resources/qoraal/00-sources.md` maps this file to *Habka Qoraalka* (1977)
Qaybta I §1. Cleaned `naxwe/14` §4.2 already treats this file as the verified
contraction list and repeats eleven of its pairs.

Repository comparison also included:

- cleaned `resources/qoraal/01-hadal-iyo-qoraal.md` (O01-R002, O01-R007);
- `resources/qoraal/03-kala-qoridda-adag.md` and
  `04-kala-qoridda-lama-qasban.md`;
- dictionary `kolmo` (quotation mark), `kuma`, `lagama maarmaan`;
- same-file section 1 as the intact eight-person template for sections 2 and 4.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-5 | reviewed | O02-R001 |
| 7-42 | reviewed | O02-R002 |
| 44-58 | reviewed | O02-R003 |
| 60-70 | reviewed | O02-R004 |
| 72-100 | reviewed | O02-R005 |
| 102-128 | reviewed | O02-R006 |
| 130-156 | reviewed | O02-R007 |
| 158-184 | reviewed | O02-R008 |
| 186-212 | reviewed | O02-R009 |
| 214-270 | reviewed | O02-R010 |
| 272-274 | reviewed | O02-R011 |
| whole file | reviewed | O02-R012 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| O02-R001 | 1-5 | Title matches the collection map; the opening sentence repeats the file-01 formula, including the already rejected `laga ma naarmaan`; `labada kolmo` is the quotation-mark term / medium | O01-R002; cleaned `01-hadal-iyo-qoraal.md`; `ereyfur.md` and `qaamuus/15-k.md` `kolmo`; file `05` “laba kolmo” | Retain the title, `dhuuxi`, and `labada kolmo`. Correct `laga ma naarmaan` to `lagama maarmaan`. Add a back-link to file `01`. Do not rewrite the opening into a new theory of writing. | `repository-supported`; `form-correction`; `intentional-retained` |
| O02-R002 | 7-42 | Two-word table is the core lookup; `Waxaan`/`waxa aan` and `waxaan`/`wax aan` are two readings, as are the two `dambee` rows; `baa ay \| baa ay` is not a contraction; heading `Ereyyada` disagrees with the H1 `Ereyada` / high | Cleaned `naxwe/14` §4.2 repeats eleven of these pairs and omits the identity row; file `01` H1 and this file's H1 use `Ereyada` | Retain every genuine pair, including both `waxaan` readings and both `dambee` expansions. Delete only the identity row `baa ay \| baa ay`. Normalize the heading to `Ereyada`. Do not merge or re-case the remaining rows. | `repository-supported`; `structural-only`; `intentional-retained` |
| O02-R003 | 44-58 | Multi-word table is useful, but `diyaarihina`/`diyaarihiina` is not a multi-word expansion, and `waxbtayga`/`gibinihi`/`qoriyshoo` have no independent match that uniquely repairs them / high | No other resource contains `waxbtayga`, `gibinihi`, or `qoriyshoo`; `diyaarihiina` is still one token; `maroontay`/`ladnahayee`/`layma` are readable source pairs | Retain all eleven rows. Label `waxbtayga`, `gibinihi`, `qoriyshoo`, and `diyaarihina` unresolved. Do not invent `wax ba aad` replacements or split `diyaarihiina` by analogy. | `unresolved`; `intentional-retained` |
| O02-R004 | 60-70 | The five `bay` expansions match the source's own note that one contraction can expand several ways / low | Cleaned `naxwe/14` §4.2; table 1 `baa ay`; table 2 `baa ay ii` | Retain all five expansions. Do not reduce them to one reading. | `repository-supported`; `intentional-retained` |
| O02-R005 | 72-100 | Eight-person `Geed + fal` paradigm is internally consistent and supplies the template for later sections / low | Same-file rows 1sg `aan` through 3pl `ay beereen`; naxwe person labels | Retain both eight-line blocks. Add no extra person. | `repository-supported`; `intentional-retained` |
| O02-R006 | 102-128 | Koobnaan keeps `Geedaannu` then `Geedaynu`, but fidsanaan writes `Geed ayaa aynu` twice and drops `aannu` / high | Same-file koobnaan lines 108-109; section 1 fidsanaan `Geed aannu` then `Geed aynu` | Correct only the second fidsanaan line to `Geed ayaa aannu beeraynaa`. Keep the third line as `Geed ayaa aynu beeraynaa`. Do not add `ayaa` to section 1. | `repository-supported`; `paradigm-correction` |
| O02-R007 | 130-156 | Definite `Geedka + fal` eight-person pair matches section 1 / low | Same-file section 1 person order | Retain both blocks. | `repository-supported`; `intentional-retained` |
| O02-R008 | 158-184 | Same `aannu`/`aynu` copy error as section 2, now in the definite `ayaa` set / high | Koobnaan lines 164-165 `Geedkaannu` / `Geedkaynu`; O02-R006 | Correct only the second fidsanaan line to `Geedka ayaa aannu beeraynaa`. Keep the third as `aynu`. | `repository-supported`; `paradigm-correction` |
| O02-R009 | 186-212 | Possessive paradigm is usable; `Gurigayaga` versus 1sg `Gurigayga`, and 3pl `gashay` versus section 1 `beereen`, are source-internal mismatches without a unique repair / medium | Dictionary `gal` past `galay`/`gashay`; section 1 3pl is `beereen`; no other file lists this possessive set | Retain all sixteen lines. Do not change `Gurigayaga` to `Gurigayga` or `gashay` to `galeen` by analogy. | `unresolved`; `intentional-retained` |
| O02-R010 | 214-270 | Sections 6–8 repeat identical koobnaan and fidsanaan blocks; that is the source showing clitic clusters that stay joined, not a failed expansion table / medium | File `04` is the collection's “do not split” chapter and already uses identical koobnaan/fidsanaan blocks for `aan`; these sentences contain `uga`/`ugu`/`ula`/`kaga`/`kala` and `ka sii` | Retain all three paired blocks exactly. Add a one-line note that the remaining clusters stay joined and link file `04`. Do not move the sections or invent shorter koobnaan forms. | `repository-supported`; `scope-correction`; `intentional-retained` |
| O02-R011 | 272-274 | Accurate closing, but it does not point to the next two splitting files / low | Collection map: `03` hard splits, `04` must-not-split | Retain the closing rule. Link `03` and `04`. | `repository-supported`; `navigation-update`; `intentional-retained` |
| O02-R012 | whole file | Already SLS-native and table-led; a few copy errors and one identity row still disagree with the file's own paradigms / medium | Two tables, eight construction subsections, no OCR page markers | Preserve the six-section sequence and every genuine pair. Change only the approved form, paradigm, heading, and link repairs. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Ereyada Afkeenna Koobma Marka Hadalka**
and keep this sequence:

1. opening, with `lagama maarmaan` and a link to file `01`;
2. two-word contraction table, minus the identity row;
3. multi-word table, with unresolved rows kept visible;
4. the five `bay` expansions;
5. eight-person `Geed` / `Geedka` / `Gurigayga` paradigms, with `aannu`
   restored in the two `ayaa` fidsanaan blocks;
6. three clitic-cluster sections, linked to file `04`; and
7. a closing that links files `03` and `04`.

No new contraction pair should be invented. Both `waxaan` readings and both
`dambee` expansions stay. `waxbtayga`, `gibinihi`, `qoriyshoo`, `diyaarihina`,
`Gurigayaga`, and 3pl `gashay` stay unresolved.

## Approval gate

- **Audit approval:** 2026-08-19 (“go ahead”)
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Audit validation

- Resource SHA-256 after audit:
  `e56c608990f3dada0685a634da0b3a5897b301a4b03445f87cba9c50ab321f72`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, six H2 headings, eight H3 subsections, two valid
  tables, no OCR page markers.

## Cleanup result and review

### Applied cleanup

- Corrected `laga ma naarmaan` to `lagama maarmaan` and linked file `01` plus
  `naxwe/14` §4.2.
- Normalized both table headings to `Ereyada` and deleted the identity row
  `baa ay | baa ay`.
- Restored `aannu` on the second fidsanaan line of the two `ayaa` paradigms:
  `Geed ayaa aannu beeraynaa` and `Geedka ayaa aannu beeraynaa`.
- Noted that sections 6–8 stay joined and linked file `04`.
- Linked the closing to files `03` and `04`.

### Deliberately retained

- Both `Waxaan`/`Waxa aan` and `waxaan`/`wax aan`, and both `dambee` expansions.
- All five `bay` expansions.
- Unresolved pairs `gibinihi`, `qoriyshoo`, `diyaarihina`, and `waxbtayga`.
- The `Gurigayaga` extra vowel and 3pl `gashay`.
- Identical koobnaan/fidsanaan blocks in sections 6–8.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, six H2 headings, eight H3 subsections, two valid tables.
- Pre-cleanup forms `naarmaan`, `Ereyyada`, and the identity row are absent.
- Duplicate `Geed ayaa aynu` / `Geedka ayaa aynu` as the 1pl exclusive line is
  absent; the inclusive `aynu` lines remain.
- Cleaned file size: 279 lines; 1,038 words; 6,121 bytes.
- Cleaned SHA-256:
  `57ad153ef73c051a4b4b7811d5c6d5ca79ba34417d952be9dfd508f94d4f83a0`.
