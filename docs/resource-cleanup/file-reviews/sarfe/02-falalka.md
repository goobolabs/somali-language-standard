# Audit record — Falalka

- **Resource path:** `resources/sarfe/02-falalka.md`
- **Collection / family:** sarfe / verb paradigms
- **Priority:** P2
- **Method:** repository-only, line-by-line tabular audit against cleaned
  `naxwe/07` and `naxwe/08`
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 126 lines; 920 words; 4,280 bytes
- **Resource SHA-256 at audit start:**
  `47c15289585e6fc124186e6ffcf43a956d8d7737c1a19f1dfadd306d4da5a2cb`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact conjugation lookup, not a second verb chapter. It has
one H1, ten H2 sections, one H3, eleven valid Markdown tables, and working
links to `naxwe/07` and `naxwe/08`. The tables already reproduce the cleaned
person/tense/aspect/prefix paradigms with only compact differences (no
`waan/waad` particles; `3aad lab` rather than `3aad keli lab`).

Cleanup should keep every table and every cell except the two source-backed
repairs already made in naxwe: restore `(waa) cuni lahaa` in the mood table,
and replace the over-compressed final negation summary with the four-environment
summary from cleaned `naxwe/08`. It must not add missing person rows, expand
`ma cunayn(in)`, or reintroduce the pre-cleanup prefix forms `aqaanaa`/`aqiin`.

## Source and repository evidence

The file names S-003 chapters 7–8. The controlling comparison is:

- cleaned `resources/naxwe/07-sarfaha-falalka.md` §§7.1.2–7.2.1.3;
- cleaned `resources/naxwe/08-hogatuska-baradigmaha-falalka.md` §§8.1–8.3 and
  Gunaanad;
- N07-R005 (`cunlahaa` → `cuni lahaa`) and N07-R008 (prefix `aqaan`/`iqiin`);
- N08-R008 (separate Ebyoon / Amar / Talo / Dhimman negation).

The prefix-versus-suffix table in this file already uses `aqaan`, not
`aqaanaa`. File `04` still has the old forms; they are out of scope here.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-8 | reviewed | M02-R001 |
| 10-16 | reviewed | M02-R002 |
| 18-26 | reviewed | M02-R003 |
| 28-38 | reviewed | M02-R004 |
| 40-50 | reviewed | M02-R005 |
| 52-60 | reviewed | M02-R006 |
| 62-72 | reviewed | M02-R007 |
| 74-82 | reviewed | M02-R008 |
| 84-94 | reviewed | M02-R009 |
| 96-106 | reviewed | M02-R010 |
| 108-118 | reviewed | M02-R011 |
| 120-126 | reviewed | M02-R012 |
| whole file | reviewed | M02-R013 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| M02-R001 | 1-8 | Clean title, purpose, and links to both verb chapters; compact S-003 attribution / low | `resources/naxwe/07` and `08` both already point back here | Retain the opening, both links, and the source line. | `repository-supported`; `intentional-retained` |
| M02-R002 | 10-16 | Three-class imperative diagnostics are source-supported; class I lists `keen` where the noun chapter's identification list uses `cab`/`jiid` / low | Cleaned `naxwe/07` §7.2.1.1: class I `cun, qor, cab, jiid`; class II `kari`/`samee`; class III `xiro`/`raadso`; the next comparison table in both files uses `keen` | Retain `cun`, `qor`, and `keen` as this compact diagnostic set. Do not swap in extra identification verbs. | `intentional-retained`; `repository-supported` |
| M02-R003 | 18-26 | The five negative verb forms match `naxwe/08`, but the heading calls them subclasses of II and III while the table also contains class I / medium | Cleaned `naxwe/08` §8.1 table: I `cunin`, IIA `toosin`, IIB `caddaynin`, IIIA `dhaqanin`, IIIB `qabsanin`; N08-R002 labels these bare negative verb forms, not complete `ha` imperatives | Relabel the heading to include class I, matching `naxwe/08`. Retain all five rows. Keep the third column as the negative verb form, not a complete negative imperative. | `repository-supported`; `classification-correction`; `intentional-retained` |
| M02-R004 | 28-38 | Complete seven-person `cun` tense table matches the cleaned chapter except for omitted `waan/waad/wuu/way` particles / low | Cleaned `naxwe/07` §7.1.2 has the same 21 verb forms with declarative particles | Retain every cell. Do not add the particles; this is the compact lookup. | `repository-supported`; `intentional-retained` |
| M02-R005 | 40-50 | Five-column `tag` aspect table is an exact compact match of the cleaned 35-cell paradigm / low | Cleaned `naxwe/07` §7.1.3 | Retain all 35 cells. Add no present-simple column. | `repository-supported`; `intentional-retained` |
| M02-R006 | 52-60 | Class-comparison table matches the cleaned five-row `keen/kari/samee/xiro/raadso` display / low | Cleaned `naxwe/07` §7.2.1.1 | Retain all five rows. Do not collapse `kariyey`/`sameeyey` spelling. | `repository-supported`; `intentional-retained` |
| M02-R007 | 62-72 | Seven-row `cun` negative table matches cleaned `naxwe/08`, including source notation `ma cunayn(in)` / low | Cleaned `naxwe/08` §8.1; N08-R003 keeps `ma cunayn(in)` unresolved as to which expansion is canonical | Retain all seven rows and the source parenthesis. Do not choose `ma cunayn` or `ma cunaynin`. | `repository-supported`; `unresolved`; `intentional-retained` |
| M02-R008 | 74-82 | Mood table still has pre-cleanup `(waa) cunlahaa`, omitting the infinitive marker restored in naxwe / high | N07-R005 and cleaned `naxwe/07` §7.1.4: `(waa) cuni lahaa`; the negative table in this same file already has `cuni lahaa` / `ma cuni lahayn` | Correct only `(waa) cunlahaa` to `(waa) cuni lahaa`. Retain the other four mood rows. | `repository-supported`; `form-correction` |
| M02-R009 | 84-94 | Seven-person stative `adag` paradigm matches the joined forms in cleaned naxwe / low | Cleaned `naxwe/07` §7.2.1.2 and `naxwe/08` §8.2 affirmative cells | Retain all seven cells. Do not add the negative `adag` table from chapter 8. | `repository-supported`; `intentional-retained` |
| M02-R010 | 96-106 | 35-cell prefix-verb present table matches cleaned `naxwe/08` / low | Cleaned `naxwe/08` §8.3 Joogto caadaley: `aqaan`, `aal`, `iraahdaa`, `imaaddaa`, `ahay` and the remaining 30 cells | Retain every cell. Do not replace `aal` with root `ool` inside this person table. | `repository-supported`; `intentional-retained` |
| M02-R011 | 108-118 | Prefix-versus-suffix comparison already uses the corrected `aqaan` series, not `aqaanaa` / low | Cleaned `naxwe/07` §7.2.1.3; this table is the compact version of that comparison | Retain all seven rows and the morpheme hyphens. Do not reintroduce `aqaanaa`. | `repository-supported`; `intentional-retained` |
| M02-R012 | 120-126 | Final negation summary collapses four environments into three, places `ha` under Talo, and uses `aan…n` instead of the cleaned dependent form / high | Cleaned `naxwe/08` Gunaanad: Ebyoon `ma`; Amar `ha`; Talo `yaan / yuusan / yaanay`; Dhimman `uusan`; N08-R008 required that four-way split and forbade a universal `-in/-o` ending | Replace the three-row summary with the four cleaned environments. Describe endings as paradigm-dependent rather than one suffix rule. Do not add new example sentences. | `repository-supported`; `classification-correction` |
| M02-R013 | whole file | Dense but already SLS-native; two leftover pre-cleanup compressions still disagree with cleaned naxwe / medium | Eleven tables, no OCR debris; files `07` and `08` were cleaned 2026-08-12 | Preserve the ten-section sequence and all table cells except the two approved repairs. Add no missing mood or prefix-past tables. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Falalka — isrogrog iyo baaradigmayaal**
and keep this sequence:

1. purpose and links to `naxwe/07` and `naxwe/08`;
2. three conjugation-class diagnostics;
3. class I / IIA / IIB / IIIA / IIIB negative verb forms;
4. `cun` tense, `tag` aspect, and class-comparison tables;
5. `cun` negatives, with `ma cunayn(in)` retained;
6. five-mood compact table with `cuni lahaa`;
7. stative `adag`;
8. prefix-verb present paradigm and prefix/suffix comparison; and
9. four-environment negation summary.

No person row should be added. Prefix-past and stative-negative tables stay in
`naxwe/08`. File `04` remains the place to correct `aqaanaa`/`aqiin`.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** M02-R001 through M02-R013
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `47c15289585e6fc124186e6ffcf43a956d8d7737c1a19f1dfadd306d4da5a2cb`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, ten H2 headings, one H3, eleven valid tables, two
  local naxwe links, no OCR page markers.

## Cleanup result and review

### Applied cleanup

- Relabeled the negative-verb table to include class I and identified the
  third column as the bare negative verb form, not a complete `ha` imperative.
- Corrected `(waa) cunlahaa` to `(waa) cuni lahaa`.
- Replaced the three-row negation summary with the four cleaned environments
  (Ebyoon `ma`, Amar `ha`, Talo `yaan / yuusan / yaanay`, Dhimman `uusan`) and
  a paradigm-dependent ending note.

### Deliberately retained

- All person/tense/aspect/prefix cells, including `ma cunayn(in)`.
- Compact forms without `waan/waad` particles.
- Prefix-present `aqaan` series; `aqaanaa` was not reintroduced.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, ten H2 headings, one H3, eleven valid tables.
- Pre-cleanup forms `cunlahaa` and `ha…n` under Talo are absent.
- Cleaned file size: 132 lines; 947 words; 4,452 bytes.
- Cleaned SHA-256:
  `617a27e2ebe4b93a5d4a7452951f41b124d0f623b2fd47c579a30131b2ae1cac`.
