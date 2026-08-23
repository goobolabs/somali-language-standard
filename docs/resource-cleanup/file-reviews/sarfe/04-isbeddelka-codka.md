# Audit record — Isbeddelka codka

- **Resource path:** `resources/sarfe/04-isbeddelka-codka.md`
- **Collection / family:** sarfe / morphophonology
- **Priority:** P2
- **Method:** repository-only, line-by-line tabular audit against cleaned
  `naxwe/02`, `naxwe/07`, and `dhawaaq/03`
- **Audit status:** approved; SLS-native cleanup applied and awaiting
  maintainer cleanup review
- **Audit started:** 2026-08-19
- **File size at audit start:** 65 lines; 324 words; 1,981 bytes
- **Resource SHA-256 at audit start:**
  `a3dda395bf3554d3529e7a9226325276cbee82de9b67cac40b1ab5a41b6b0683`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact sound-change lookup for the paradigms already tabulated
in files `01` and `02`. It has one H1, six H2 sections, five valid Markdown
tables, and links to `naxwe/02`, `naxwe/07`, and `dhawaaq/03`.

Cleanup should keep the file tabular. It must align the feminine `*-t-*` table
with the four cleaned naxwe examples, including the already approved
`gashay` correction, restore prefix-verb `aqaan → iqiin`, and stop asserting a
`/k/ → /g/` rule that `dhawaaq/03` does not state. It must not invent a
complete phonological rule system from four examples.

## Source and repository evidence

The file names S-003 chapters 2 and 7. The controlling comparison is:

- cleaned `resources/naxwe/07-sarfaha-falalka.md` §7.1.1.3:
  `cun+tay → cuntay`; `qaad+tay → qaadday`; `badi+tay → badisay`;
  `gal+tay → gashay`;
- N07-R002, which already corrected `gal+tay → qashay` to `gashay` in naxwe
  because dictionary `gal³` is `(galay, gashay)` and `qashay` belongs to other
  roots;
- N07-R008: present `aqaan`, past `iqiin`;
- cleaned `naxwe/07` §7.3.2.1 for `kariyay` / `karisay`;
- cleaned `naxwe/02` §2.3.1 for plural phonology and L6 `áwr`, `díbi/dibí`,
  `mádax/madáx`;
- `resources/dhawaaq/03-shibbanayaasha.md` for the `/k/`–`/g/` voiced pair.

This file still contains the pre-cleanup naxwe errors. Files `01` and `02`
do not.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-8 | reviewed | M04-R001 |
| 10-20 | reviewed | M04-R002 |
| 22-29 | reviewed | M04-R003 |
| 31-38 | reviewed | M04-R004 |
| 40-49 | reviewed | M04-R005 |
| 51-58 | reviewed | M04-R006 |
| 60-65 | reviewed | M04-R007 |
| whole file | reviewed | M04-R008 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| M04-R001 | 1-8 | Clean purpose and links; English `morphophonology` in the H1 is a first-use gloss on an otherwise Somali title / low | Collection README requires Somali-first prose and allows tables; naxwe chapters commonly gloss technical terms once | Retain the title, links, and 1999 source line. Keep the English gloss rather than deleting or expanding it. | `intentional-retained`; `repository-supported` |
| M04-R002 | 10-20 | Feminine `*-t-*` table mislabels `cuntay` as `t → d` and still has the rejected mixed-root form `gal+tay → qashay` / fatal | N07-R002 and cleaned `naxwe/07` §7.1.1.3 give four examples, not a two-row `t → d` rule; dictionary `gal³` `(galay, gashay)`; `qashay` is not the past of `gal` | Rebuild the table from the four naxwe examples only. Correct `qashay` to `gashay`. Do not label `cuntay` as `t → d`. Do not add further environments by analogy. | `repository-supported`; `paradigm-correction`; `classification-correction` |
| M04-R003 | 22-29 | Causative `*-y-*` and `t → s` pair matches the cleaned naxwe account / low | Cleaned `naxwe/07` §7.3.2.1: `kari + ay → kariyay`; `kari+t+ay → karisay` | Retain both rows. Do not collapse `kariyay` with table spelling `kariyey` from file `02`. | `repository-supported`; `intentional-retained` |
| M04-R004 | 31-38 | Four plural-phonology rows match files `01` and cleaned `naxwe/02` / low | `doofaarro`, `magacyo`, `gacmo`/`jilbo`, `hooyooyin` | Retain all four rows. Add no extra plural class. | `repository-supported`; `intentional-retained` |
| M04-R005 | 40-49 | L6 tone-shift table keeps the three independently supported accent rows and correctly omits the two naxwe source-only pairs / low | Cleaned `naxwe/02` §2.3.1.5; N02-R012 leaves `Cárab/Caráb` and `órgi/orgí` unresolved | Retain `áwr`, `díbi/dibí`, and `mádax/madáx` with their accents. Do not import the unresolved pairs. | `repository-supported`; `intentional-retained` |
| M04-R006 | 51-58 | Prefix-vowel table still uses pre-cleanup `aqaanaa` / `aqiin`; `imaaddaa` / `imid` are correct / high | N07-R008 and cleaned `naxwe/07` §7.2.1.3: `aqaan → iqiin`; cleaned `naxwe/08` past table: 1sg `imid`; file `02` already has present `aqaan` | Correct only `aqaanaa` → `aqaan` and `aqiin` → `iqiin`. Retain `imaaddaa` / `imid` and the `a` → `i` statement. | `repository-supported`; `paradigm-correction` |
| M04-R007 | 60-65 | Closing pointer overstates a morphophonological rule `/k/ → /g/ xarig dambe` that `dhawaaq/03` does not make / medium | `dhawaaq/03` describes `/k/` as voiceless and `/g/` as voiced at the same place of articulation; it does not state a following-context mutation rule; `dhawaaq/08` treats spread glottis, not this alternation | Keep the links to `dhawaaq/03` and `dhawaaq/08`. Describe them as the voiced/voiceless pair and phonation notes, not as a verified `/k/ → /g/` rule. Do not invent an example. | `repository-supported`; `scope-correction` |
| M04-R008 | whole file | Compact and otherwise well structured, but it is the morphology file that still carries the two naxwe pre-cleanup errors / high | Five tables, no OCR debris; `naxwe/07` cleanup already removed `qashay` and `aqaanaa` | Preserve the six-section sequence. Change only the approved `*-t-*` table, prefix-vowel cells, and phonology pointer wording. | `structural-only`; `repository-supported` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Isbeddelka codka — morphophonology**
and keep this sequence:

1. purpose and links to `naxwe/02`, `naxwe/07`, and `dhawaaq/03`;
2. four source examples of 3rd-feminine `*-t-*`, including `gashay`;
3. causative `*-y-*` / `t → s`;
4. plural phonology;
5. L6 tone-shift masculines that become feminine;
6. prefix-verb `a` → `i` with `aqaan → iqiin`; and
7. a scoped pointer to the phonology voiced/voiceless pair.

No new sound-change rule or example should be introduced. Unresolved naxwe
accent rows stay out of the L6 table.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** M04-R001 through M04-R008
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Audit validation

- Resource SHA-256 after audit:
  `a3dda395bf3554d3529e7a9226325276cbee82de9b67cac40b1ab5a41b6b0683`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, six H2 headings, five valid tables, three local
  links, no OCR page markers.

## Cleanup result and review

### Applied cleanup

- Rebuilt the 3rd-feminine `*-t-*` table from the four cleaned naxwe examples
  and corrected `qashay` to `gashay`. `cuntay` is no longer labeled `t → d`.
- Corrected prefix-vowel `aqaanaa` / `aqiin` to `aqaan` / `iqiin`.
- Replaced the unverified `/k/ → /g/` rule with a scoped pointer to the
  phonology voiced/voiceless pair and spread-glottis note.

### Deliberately retained

- Causative `kariyay` / `karisay`, plural-phonology rows, and L6 accents
  `áwr`, `díbi/dibí`, `mádax/madáx`.
- English first-use gloss `morphophonology` in the H1.
- Unresolved naxwe accent pairs were not imported.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, six H2 headings, five valid tables.
- Pre-cleanup forms `qashay`, `aqaanaa`, and `aqiin` are absent.
- Cleaned file size: 65 lines; 293 words; 1,869 bytes.
- Cleaned SHA-256:
  `3d2d2906aeb6b89c02be121d72d1bc3ef8e7b9997e413fe1ae7a7ab5d5587bc7`.
