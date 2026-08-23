# Audit record — Meelaha ay Adag Tahay Kala Qoridda

- **Resource path:** `resources/qoraal/03-kala-qoridda-adag.md`
- **Collection / family:** qoraal / hard splits (adjective + copula *ah*)
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 140 lines; 614 words; 3,664 bytes
- **Resource SHA-256 at audit start:**
  `0ba92bad8be57fcea787dc77e61e4bdee022910492f82cd9771cfa73a882ed59`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact SLS chapter on the hard case of speech contraction:
adjective + copula *ah*, where a sound change makes the written split look
wrong. It has one H1, eight H2 sections, no H3 subsections, many fenced
example blocks, and no cover matter, exercises, or page markers.

Cleanup should keep it as the maintained account of that clash and of the
source's decision to write the two words apart. It should restore `nahay` in
the expansion sets that the same file already recommends, correct the Type II
row that expands `dahay` as `ahay`, repair the `lama` split, omit the
unreadable closing sentence, and add links to `01`, `02`, `04`, and the
related copula and *t→d* chapters. It must not normalize `nadaw` / `madaw` /
`madoobahay` to dictionary `madow`, add missing persons to the opening
copula sample, rewrite the *n*-final rule into a *w*-final rule, or import
naxwe article tables.

## Source and repository evidence

`resources/qoraal/00-sources.md` maps this file to *Habka Qoraalka* (1977)
Qaybta I §2. Cleaned `naxwe/14` §4.3 already points here and does not rebuild
the source's mixed tables.

Repository comparison also included:

- cleaned `resources/qoraal/01-hadal-iyo-qoraal.md` and `02-eray-kooban-hadalka.md`;
- `resources/qoraal/04-kala-qoridda-lama-qasban.md`;
- `resources/naxwe/07-sarfaha-falalka.md` and `08-hogatuska-baradigmaha-falalka.md`
  for the copula set `ahay` / `tahay` / `yahay` / `nahay` / `tihiin` / `yihiin`;
- `resources/naxwe/03-sarfaha-tifaftireyaasha.md` and
  `resources/sarfe/04-isbeddelka-codka.md` for *t→d*;
- dictionary `madow`, `ladan`; no independent `nadaw` or `madaw` entry.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-5 | reviewed | O03-R001 |
| 7-20 | reviewed | O03-R002 |
| 22-51 | reviewed | O03-R003 |
| 53-60 | reviewed | O03-R004 |
| 62-80 | reviewed | O03-R005 |
| 82-91 | reviewed | O03-R006 |
| 93-108 | reviewed | O03-R007 |
| 110-137 | reviewed | O03-R008 |
| 139-140 | reviewed | O03-R009 |
| whole file | reviewed | O03-R010 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| O03-R001 | 1-5 | Title matches the collection map; the opening correctly scopes the chapter to copula *ah* on some adjectives; `yareyn` and `neel` are this source's terms / low | File map in `00-sources.md`; files `04` and `05` also use `neel`; file `01` already links here | Retain the title, `yareyn`, and the *ah* + tilmaame scope. Add a back-link to files `01` and `02`. Do not rewrite the opening into a new theory of writing. | `repository-supported`; `navigation-update`; `intentional-retained` |
| O03-R002 | 7-20 | Independent copula sample is usable; this source's `tilmaame` is the writing class from file `01`, not ereyfur's demonstrative; the six lines omit 1sg `ahay` and 1pl exclusive / medium | Cleaned `naxwe/08` lists `ahay`, `tahay`, `yahay`, `tahay`, `nahay`, `tihiin`, `yihiin`; file `01` keeps the five writing classes including `tilmaame` | Retain all six lines and the label `tilmaame`. Do not add `waxa aan ahay`. Link `naxwe/08` for the full copula set. Do not rename `tilmaame` to `sifo`. | `repository-supported`; `intentional-retained` |
| O03-R003 | 22-51 | Koobnaan is a mixed six-token speech sample; fidsanaan is two six-line sets that write `tahay` twice and omit `nahay`, against the same file's later writing decision / high | Same-file Go'aanka lines 115-127: `ahay`, `nahay`, `tahay`, `tihiin`, `yahay`, `yihiin`; koobnaan already has `nadawnahay`; `naxwe/07`–`08` include `nahay` | Retain the six-line koobnaan sample, including `madoobahay`. Replace each fidsanaan six-line set with the matching Go'aanka order. Do not expand koobnaan, merge the two adjectives, or normalize `nadaw` / `madaw` / `madoobahay` to `madow`. | `repository-supported`; `paradigm-correction`; `intentional-retained` |
| O03-R004 | 53-60 | The easy-split pair is the control case: no sound change, so the words already stand apart / low | Same-file hard section contrasts `ladanahay`; dictionary `ladan` is healthy/well | Retain both arrows. Leave `Madaw` versus `nadaw` in the unresolved spelling cluster. Do not add extra persons. | `repository-supported`; `unresolved`; `intentional-retained` |
| O03-R005 | 62-80 | The two writing strategies are the chapter's core; Type II maps `nadaw dahay` to `nadaw ahay`, but the same file derives `dahay` from `tahay` / high | Lines 104-105: `madaw + tahay → madawdahay`; Go'aanka writes `nadaw tahay` for that person; 1sg is `ahay` | Retain Type I and the four Type II rows. Correct only the first Type II expansion to `waa nadaw tahay`. Do not invent `dihiin` on the `tihiin` row. | `repository-supported`; `paradigm-correction` |
| O03-R006 | 82-91 | The rule-clash question is needed; `la isku na qoro` is the same `lama` split already fixed in file `01`; `lagama eray` and `sanceysan` have no unique repair / medium | O01-R002 `la na huraan` → `lama huraan`; file `01` two splitting rules; no dictionary `sanceysan` | Retain the two numbered rules. Correct `la isku na qoro` to `lama isku qoro`. Link file `01`. Leave `lagama eray` and `sanceysan` unresolved. Do not rewrite the question. | `repository-supported`; `form-correction`; `unresolved`; `intentional-retained` |
| O03-R007 | 93-108 | The *t→d* examples support the clash, but the prose says the stem ends in *n* while `caw` and `madaw` end in *w* / medium | `naxwe/03` *t→d* after a vowel or `d, c, x, h, y, '`; `sarfe/04` uses different *-t-* verb examples; dictionary `madow` does not uniquely license rewriting these stems | Retain `cawda` / `cawdaa` / `cawday` and `madawdahay` / `madawdihiin`. Do not change the *n*-final sentence into a *w*-final rule. Link `naxwe/03` and `sarfe/04` without importing their tables. | `unresolved`; `intentional-retained`; `repository-supported` |
| O03-R008 | 110-137 | The recommended writing is the file's decision and the intact six-person template; the short `ladan` reprise drops `nahay` / medium | Lines 122-127 already list `waa ladan nahay`; `naxwe/08` 1pl is `nahay` | Retain the recommendation to split tilmaame + *ah*. Insert `waa ladan nahay` in the short reprise so it matches the six-line set above. Do not add a seventh 3sg-feminine line. | `repository-supported`; `paradigm-correction`; `intentional-retained` |
| O03-R009 | 139-140 | The closing paragraph is not uniquely readable (`dhisnihii`, `wayn u ma`, `siddii`); the collection omits unreadable OCR rather than reconstructing it / high | Orthography README: omit unreadable OCR fragments; line 112 already states the writing decision | Omit the damaged closing paragraph. Do not invent a replacement theory. Add links to file `04` and to `naxwe/14` §4.3. | `structural-only`; `unresolved` |
| O03-R010 | whole file | Already SLS-native and example-led; a few copy errors still disagree with the file's own writing decision / medium | Eight H2 sections, no OCR page markers | Preserve the eight-section sequence and both adjectives. Change only the approved form, paradigm, omission, and link repairs. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Meelaha ay Adag Tahay Kala Qoridda**
and keep this sequence:

1. opening, with links to files `01` and `02`;
2. independent copula sample, unexpanded;
3. mixed koobnaan sample plus fidsanaan aligned to the Go'aanka six-line set;
4. easy split versus hard split, with Type II `dahay` → `tahay`;
5. the two-rule clash, with `lama isku qoro` and a link to file `01`;
6. the *t→d* examples, linked to `naxwe/03` and `sarfe/04`;
7. the writing decision, with `nahay` restored in the short `ladan` reprise;
   and
8. links to file `04` and `naxwe/14` §4.3, without a reconstructed closing.

`nadaw`, `madaw`, `Madaw`, and `madoobahay` stay unresolved. `caw + ta`,
`lagama eray`, and `sanceysan` stay. No person is added to the opening copula
sample.

## Approval gate

- **Audit approval:** 2026-08-19 (“go ahead”)
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `0ba92bad8be57fcea787dc77e61e4bdee022910492f82cd9771cfa73a882ed59`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, eight H2 headings, no H3 subsections, no OCR
  page markers.

## Cleanup result and review

### Applied cleanup

- Linked the opening to files `01` and `02`.
- Linked the independent copula sample to `naxwe/08` without adding persons.
- Aligned both fidsanaan six-line sets to the Go'aanka order, restoring
  `nahay`.
- Corrected Type II `nadaw dahay → waa nadaw tahay`.
- Corrected `la isku na qoro` to `lama isku qoro` and linked file `01`.
- Linked *t→d* to `naxwe/03` and `sarfe/04` without importing tables.
- Inserted `waa ladan nahay` in the short reprise.
- Omitted the unreadable closing paragraph and linked file `04` plus
  `naxwe/14` §4.3.

### Deliberately retained

- The six-line opening copula sample and the mixed koobnaan sample,
  including `madoobahay`.
- `nadaw` / `madaw` / `Madaw` / `madoobahay`; not rewritten to `madow`.
- `caw + ta`, the *n*-final rule sentence, `lagama eray`, and `sanceysan`.
- The source label `tilmaame`.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, eight H2 headings, no H3 subsections.
- Pre-cleanup forms `isku na qoro`, duplicate fidsanaan `tahay`, Type II
  `dahay → ahay`, and the damaged closing are absent.
- Cleaned file size: 149 lines; 650 words; 4,397 bytes.
- Cleaned SHA-256:
  `a21557a8775ab928255eabcde88f64321065de83298f1eb15df42a6e5723556d`.
