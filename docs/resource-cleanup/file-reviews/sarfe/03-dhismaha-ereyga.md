# Audit record — Dhismaha ereyga

- **Resource path:** `resources/sarfe/03-dhismaha-ereyga.md`
- **Collection / family:** sarfe / derivation and affixes
- **Priority:** P2
- **Method:** repository-only, line-by-line tabular audit against cleaned
  `naxwe/07` §§7.2–7.3
- **Audit status:** approved; SLS-native cleanup applied and awaiting
  maintainer cleanup review
- **Audit started:** 2026-08-19
- **File size at audit start:** 85 lines; 490 words; 2,368 bytes
- **Resource SHA-256 at audit start:**
  `f3d3b62a6dd16a341e33d5a34fce9a3beb6bdcec090695aec093621f5d7d7b8b`
- **Resource-text changes during this audit:** none

## Target output model

This file is a compact derivational lookup, not a second morphology essay. It
has one H1, six H2 sections, three H3 subsections, eight valid Markdown tables,
and a working link to `naxwe/07`. Several tables already encode corrections
that naxwe had to restore later: `-oob`/`-ood` make verbs from nouns, and
`@ (eber)` is the source's zero-suffix notation.

Cleanup should keep every affix row and every listed example. It should remove
only the one misleading auxiliary example that does not contain the claimed
verb, keep `la'` without inventing a sentence, and leave source spellings such
as `sameyn` untouched. It must not import the unresolved feminine agent forms
`abuurto`, `karise`, and `barate` from `naxwe/07`.

## Source and repository evidence

The file names S-003 chapter 7. The controlling comparison is cleaned
`resources/naxwe/07-sarfaha-falalka.md` §§7.2.2–7.3.3, plus N07-R009 through
N07-R014.

Dictionary corroboration: `seexo` records masculine past `-xday`, so
`Cali wuu seexday` is a valid masculine counterpart of naxwe's feminine
`Shamso way seexatay`. It is not an OCR error.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-7 | reviewed | M03-R001 |
| 9-20 | reviewed | M03-R002 |
| 22-30 | reviewed | M03-R003 |
| 32-38 | reviewed | M03-R004 |
| 40-55 | reviewed | M03-R005 |
| 57-75 | reviewed | M03-R006 |
| 77-85 | reviewed | M03-R007 |
| whole file | reviewed | M03-R008 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| M03-R001 | 1-7 | Clean title, purpose, and scoped link to `naxwe/07` §§7.2–7.3 / low | Cleaned `naxwe/07` already points back to this file at §7.3.2.1 | Retain the opening and source line. | `repository-supported`; `intentional-retained` |
| M03-R002 | 9-20 | Eight-row affix table matches the cleaned inventory; `-oob` and `-ood` already have the correct noun→verb direction that naxwe had to restore / low | Cleaned `naxwe/07` §7.3.2.3; N07-R013 reversed the original naxwe descriptions; this table already shows `biyo → biyoob` and `dhaxan → dhaxmood` | Retain all eight rows and glosses. Do not expand `fur → furmay` into the full naxwe sentence pair. | `repository-supported`; `intentional-retained` |
| M03-R003 | 22-30 | Five-row I→II causative table is an exact match of the cleaned derivation display / low | Cleaned `naxwe/07` §7.3.2.1 | Retain all five rows. Add no extra causative pairs. | `repository-supported`; `intentional-retained` |
| M03-R004 | 32-38 | Three surface forms of `-an` match the cleaned distribution, including `baratay` / low | Cleaned `naxwe/07` §7.3.2.2: `-an` / `-o` / `-at`, with `bar+at+tay → baratay` | Retain the three rows. Do not add the `dhaq`/`beer` sentence pairs. | `repository-supported`; `intentional-retained` |
| M03-R005 | 40-55 | Three deverbal-noun classes, zero-suffix notation `@ (eber)`, genders, and `sameyn` are source-supported compact extracts / low | Cleaned `naxwe/07` §7.3.3; N07-R014 keeps `abuurto`, `karise`, and `barate` unresolved and forbids building a feminine `-te` rule from them | Retain both tables, `@ (eber)`, `sameyn`, and the listed examples. Do not import the unresolved feminine agents. | `repository-supported`; `intentional-retained`; `unresolved` kept out of this file |
| M03-R006 | 57-75 | Auxiliary group A matches naxwe; group B's `rab` cell cites `Waan baxayaa`, a sentence that does not contain `rab`, while `la'` has no example / high | Cleaned `naxwe/07` §7.2.2: `kar` *Waan akhrin karaa*; `waay` *Wuu ordi waayay*; independent use illustrated with `gaar` *Dayaaraddii … bay gaartay*; `la'` listed with no compact example; `rab` has no independent sentence in that section | Retain `kar`, `waay`, and `gaar` examples. Replace the `rab` sentence with a dash rather than inventing a `rab` clause. Keep `la'` as an empty example. Do not add new auxiliary sentences. | `repository-supported`; `example-correction`; `intentional-retained` |
| M03-R007 | 77-85 | Compact transitive/intransitive table uses valid lookup examples; closing sentence restates the cleaned naxwe valency/causative relation / low | Dictionary `seexo` `(-xday, -xatay)`; cleaned `naxwe/07` §7.3.1–7.3.2.1; naxwe 06 also uses `seexday` | Retain both rows, `Cali waraaq buu qoray`, `Cali wuu seexday`, and the closing `-in` note. Do not replace them with the longer naxwe numbered examples. | `repository-supported`; `intentional-retained` |
| M03-R008 | whole file | Already SLS-native and mostly aligned with post-cleanup naxwe; one false auxiliary example is the only unique defect / low | Eight tables, no OCR debris | Preserve the six-section sequence and every affix row. Change only the approved `rab` cell. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Dhismaha ereyga — lifaaqyo iyo farcamo**
and keep this sequence:

1. purpose and link to `naxwe/07`;
2. eight-row verbal-affix inventory;
3. I→II causative table;
4. `-an` surface forms;
5. deverbal nouns by type and by conjugation class;
6. auxiliary groups A and B, with no invented `rab` sentence; and
7. compact transitive/intransitive contrast.

No new affix, noun, or sentence should be introduced. Unresolved naxwe feminine
agent forms stay out of this lookup.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** M03-R001 through M03-R008
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Audit validation

- Resource SHA-256 after audit:
  `f3d3b62a6dd16a341e33d5a34fce9a3beb6bdcec090695aec093621f5d7d7b8b`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, six H2 headings, three H3 subsections, eight
  valid tables, one local naxwe link, no OCR page markers.

## Cleanup result and review

### Applied cleanup

- Replaced the `rab` cell `*Waan baxayaa* (madaxbannaan)` with a dash. No
  substitute `rab` sentence was invented.

### Deliberately retained

- All eight affix rows, I→II pairs, `-an` forms, deverbal-noun tables
  including `@ (eber)` and `sameyn`, and the `kar`/`waay`/`gaar`/`la'` cells.
- Transitivity examples `Cali waraaq buu qoray` and `Cali wuu seexday`.
- Unresolved naxwe feminine agents were not imported.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, six H2 headings, three H3 subsections, eight valid tables.
- Cleaned file size: 85 lines; 488 words; 2,342 bytes.
- Cleaned SHA-256:
  `f53dbd7974734cff13541243a978a63245fb73fe19cb90d6c36b941066b63d0f`.
