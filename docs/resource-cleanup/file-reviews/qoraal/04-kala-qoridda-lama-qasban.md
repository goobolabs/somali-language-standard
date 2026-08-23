# Audit record — Meelaha aanay Habboonayn Kala Qoridda Labada Eray

- **Resource path:** `resources/qoraal/04-kala-qoridda-lama-qasban.md`
- **Collection / family:** qoraal / must-not-split clusters
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; SLS-native cleanup applied and awaiting
  maintainer cleanup review
- **Audit started:** 2026-08-19
- **File size at audit start:** 185 lines; 692 words; 4,397 bytes
- **Resource SHA-256 at audit start:**
  `9565daada60d2ed71e227d865137abee55de0eb7be54a8781dca47ad83a396bb`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact SLS chapter on two clusters the source says writing
should not treat as ordinary splits: negative *aan* plus a subject pronoun,
and the question word *Maxaa*. It has one H1, seven H2 sections, six H3
subsections, many fenced example blocks, and no cover matter, exercises, or
page markers.

Cleanup should keep it as the maintained “do not force a split” chapter. It
should restore the missing negative *aan* on the recommended fidsanaan
persons 2–8, correct the *n/m* readings `naxay` and `narkii`, repair `lagu
na` and `haw yari`, and add links to `01`–`03` and the naxwe negation
chapter. It must not invent three distinct dialect 2–4 forms, normalize
`aannan` / `aanan` / `aannannu`, import file `02`’s `uga`/`ugu` blocks, or
rewrite `Maxaa` into dictionary `maxaad` rows.

## Source and repository evidence

`resources/qoraal/00-sources.md` maps this file to *Habka Qoraalka* (1977)
Qaybta I §3. Cleaned `naxwe/14` §4.3 already points here and does not rebuild
the dialect tables.

Repository comparison also included:

- cleaned `resources/qoraal/01-hadal-iyo-qoraal.md` through `03-kala-qoridda-adag.md`;
- cleaned `02-eray-kooban-hadalka.md` §§6–8 (clitic clusters that stay joined;
  already linked here);
- `resources/naxwe/12-noocyada-weeraha.md` §12.3 (`baanan tegin`, `ma aan`);
- `resources/naxwe/ereyfur.md` `qurub diidmo`;
- dictionary `maxaa` / `maxay` (`Waa maxay?`; `maxaad` = `maxaa+aad`).

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-7 | reviewed | O04-R001 |
| 9-67 | reviewed | O04-R002 |
| 69-89 | reviewed | O04-R003 |
| 91-104 | reviewed | O04-R004 |
| 106-134 | reviewed | O04-R005 |
| 136-166 | reviewed | O04-R006 |
| 168-175 | reviewed | O04-R007 |
| 177-181 | reviewed | O04-R008 |
| 183-185 | reviewed | O04-R009 |
| whole file | reviewed | O04-R010 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| O04-R001 | 1-7 | Title matches the collection map; the opening correctly names two clusters (dialectal *aan* and an unresolved question split) / low | File map in `00-sources.md`; files `03` and `05` also use `neel` | Retain the title, the two-cluster scope, and `neel`. Add back-links to files `01`–`03`. Do not rewrite the opening into a new theory of writing. | `repository-supported`; `navigation-update`; `intentional-retained` |
| O04-R002 | 9-67 | Dialect 1 is the identity case `Waxa aan` / `Waxa aan`; dialects 2–4 are three identical `aannan` / `aan aan` copies, not three recoverable variants / high | Same-file recommended 1sg `aanan`; `naxwe/12` `baanan tegin` and `nin aanan aqoon`; no other resource lists four *aan* dialects | Retain all four headings as the source’s four-dialect claim. Keep dialect 1 and one `aannan` / `aan aan` pair. Do not invent distinguishing forms for dialects 3–4. Label the identical copies unresolved. | `unresolved`; `intentional-retained` |
| O04-R003 | 69-89 | Placement of *ma* before the verb versus before the subject pronoun is the syntactic point; `narkii` is the same *n/m* confusion as `naxay`; extra `na` before `ma` is not uniquely `lama` / medium | Same-file later `naxay` / dictionary `maxay`; parallel lines are `Cali ma tegin` without `na`; `naxwe/12` `ma aan tago` | Retain both placement blocks. Correct `narkii` to `markii`. Leave `Agaasimuhu na ma` unresolved. Do not delete `na` by analogy. | `repository-supported`; `form-correction`; `unresolved`; `intentional-retained` |
| O04-R004 | 91-104 | *ma* and clitic *aan* are the same two negatives the dialect section needs; `Gabadhe` has no unique repair / low | `ereyfur.md` `qurub diidmo`; `naxwe/12` §12.3 | Retain all eight example lines and `Gabadhe`. Link `naxwe/12` §12.3. Do not add `ha` / `yaan` from `naxwe/08`. | `repository-supported`; `unresolved`; `intentional-retained` |
| O04-R005 | 106-134 | The eight-person koobnaan is the recommended fused writing; fidsanaan keeps 1sg `Waxa aan` but drops negative *aan* on persons 2–8, which would read as positive pronouns / high | Same-file dialect 2 fidsanaan `Waxa aan aan`; same-file *Maxaa* fidsanaan keeps `Maxaa` on every line; koobnaan still has fused *aan* on every line; file `01` expansion must not change grammatical meaning | Retain the eight-line koobnaan, including `aannannu` and the two `aanay` lines. Keep 1sg fidsanaan as `Waxa aan tegin magaalada`. Restore `aan` before the pronoun on fidsanaan lines 2–8 (`Waxa aan aannu` … `Waxa aan ay`). Keep the optional `Waxa aan aan` sentence. Do not normalize `aannannu` or `aanan`. | `repository-supported`; `paradigm-correction`; `unresolved`; `intentional-retained` |
| O04-R006 | 136-166 | *Maxaa* plus eight pronouns is internally consistent and matches dictionary `maxaa+aad`; the intro says the second question-word is hard to split; `lagu na` is the same `lama`/`laguma` split / high | Dictionary `maxaa`; cleaned file `02` eight-person `aannu`/`aynu` order; O01-R002 / O03-R006 `lama`; same-file line 179 `hawl yari` | Retain both eight-line *Maxaa* blocks. Correct `lagu na kala saari karo` to `laguma kala saari karo`. Do not reduce 3sgf/3pl `Maxay` / `Maxaa ay` to one line. | `repository-supported`; `form-correction`; `intentional-retained` |
| O04-R007 | 168-175 | `naxay` in this question set is dictionary `maxay` (`Waa maxay?`), not the verb `nax`; the Q/A pair is a sameness test, not a person paradigm; `wax ba na aad` has no unique repair / high | `resources/qaamuus/17-m.md` `maxay`; `dhawaaq/01` `Waa maxay hadalka?`; file `02` unresolved `wax ba aad` | Correct `naxay` to `maxay`. Retain `Ma odhan karnaa?`, both Q/A lines, and `wax ba na aad`. Do not change `doonaysaa` / `doonaysid` or invent `waxbaad`. | `repository-supported`; `form-correction`; `unresolved`; `intentional-retained` |
| O04-R008 | 177-181 | The second question is `Waa maxay kani?`; `haw yari` is the same file’s `hawl yari`; the point that this `maxay` is not an ordinary contraction should stay / medium | Dictionary `maxay` sense (a) `Waa maxay?`; line 138 `hawl yari` | Correct `naxay` to `maxay` in the heading and body. Correct `haw yari` to `hawl yari`. Retain the claim that this item stays open for later writing decisions. Do not force a `Maxaa ay` expansion here. | `repository-supported`; `form-correction`; `intentional-retained` |
| O04-R009 | 183-185 | Accurate closing, but it does not point back to the splitting files or the clitic clusters that file `02` already parked here / low | Cleaned `02` §§6–8; collection map `01`–`03` | Retain the closing rule. Link files `02` and `03`. Do not move the `uga`/`ugu`/`kaga` blocks into this file. | `repository-supported`; `navigation-update`; `intentional-retained` |
| O04-R010 | whole file | Already SLS-native and example-led; dialect copies, a dropped negative, and *n/m* readings still disagree with the file’s own paradigms / medium | Seven H2 sections, six H3 subsections, no OCR page markers | Preserve the two-cluster sequence. Change only the approved form, paradigm, and link repairs. Keep `aannan`, `aanan`, `aannannu`, `Gabadhe`, and `wax ba na aad`. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Meelaha aanay Habboonayn Kala
Qoridda Labada Eray** and keep this sequence:

1. opening, with links to files `01`–`03`;
2. four dialect headings, with dialect 1 as identity and 2–4 left identical;
3. placement of *ma*, with `markii` restored;
4. *ma* plus clitic *aan*, linked to `naxwe/12`;
5. eight-person recommended writing, with *aan* restored on fidsanaan
   persons 2–8;
6. *Maxaa* eight-person pair, with `laguma` and `maxay`; and
7. a closing that links `02` and `03` and does not import clitic tables.

`aannannu`, `Gabadhe`, `Agaasimuhu na ma`, `Ma odhan karnaa?`, and `wax ba
na aad` stay unresolved. File `02`’s `uga`/`ugu`/`kaga` sentences stay in
file `02`.

## Approval gate

- **Audit approval:** 2026-08-19 (“go ahead”)
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Audit validation

- Resource SHA-256 after audit:
  `9565daada60d2ed71e227d865137abee55de0eb7be54a8781dca47ad83a396bb`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, seven H2 headings, six H3 subsections, no OCR
  page markers.

## Cleanup result and review

### Applied cleanup

- Linked the opening to files `01`–`03`.
- Corrected `narkii` to `markii`.
- Linked *ma* / *-aan* to `naxwe/12` §12.3.
- Restored negative `aan` on fidsanaan persons 2–8; kept 1sg as
  `Waxa aan tegin magaalada`.
- Corrected `lagu na` to `laguma`, `naxay` to `maxay`, and `haw yari` to
  `hawl yari`.
- Linked the closing to files `02` and `03` without importing `uga`/`ugu`
  blocks.

### Deliberately retained

- All four dialect headings; dialects 2–4 remain identical.
- `aanan`, `aannan`, and `aannannu`.
- `Gabadhe`, `Agaasimuhu na ma`, `Ma odhan karnaa?`, and `wax ba na aad`.
- Both `aanay` / `Maxay` 3sgf and 3pl lines.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, seven H2 headings, six H3 subsections.
- Pre-cleanup forms `naxay`, `narkii`, `lagu na`, `haw yari`, and
  pronoun-only fidsanaan lines 2–8 are absent.
- Cleaned file size: 191 lines; 751 words; 5,075 bytes.
- Cleaned SHA-256:
  `0ff53cc3b52a62ad38ea486df5be10aa621fb33105a09fbcf179e1ba75451b4c`.
