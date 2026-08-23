# Audit record — Sarfaha tirada

- **Resource path:** `resources/naxwe/05-sarfaha-tirada.md`
- **Collection / family:** naxwe / primary grammar
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; conservative SLS cleanup applied and awaiting review
- **Audit started:** 2026-08-12
- **File size at audit start:** 88 lines; 565 words; 3,059 bytes
- **Resource SHA-256 at audit start:**
  `ce957a438cc00a0e758f142d0dfc5f3df3b33a7d0239326fab0331e5d1895b78`
- **Resource-text changes during audit:** none

## Target output model

This file is already a short primary-grammar chapter rather than an OCR-shaped
transcript. It has one H1, two H2 sections, one H3 subsection, three valid
Markdown tables, and three bulleted measurement examples. It contains no front
matter, exercises, scan debris, false headings, damaged reading order, numbered
sentence series, or broken Markdown.

Cleanup should preserve the chapter's coverage of cardinal numerals, numeral–
noun order, counted-noun morphology, mass nouns and measure expressions,
approximate numerals, and ordinal numerals. It should correct the uniquely
supported `-eeya` suffix error, replace the internally contradictory
gender-only account of `-ood` with a morphology-based description, qualify the
mass-noun and ordinal-derivation generalizations, align the ordinal label with
the glossary without erasing the source term `jagaale`, and add navigation to
the maintained noun and supplementary numeral accounts.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-5 | reviewed | N05-R001 |
| 6-24 | reviewed | N05-R002 |
| 26-45 | reviewed | N05-R003 |
| 47-56 | reviewed | N05-R004 |
| 58-61 | reviewed | N05-R005 |
| 63-67 | reviewed | N05-R006 |
| 68-81 | reviewed | N05-R007 |
| 83-88 | reviewed | N05-R008 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| N05-R001 | 1-5 | Useful opening taxonomy, but `jagaale` is presented as the term parallel to `tiraale` while the chapter glossary records `tiraale jago`; calling every numeral a noun is the source's analysis rather than a repository-wide diagnostic / medium | `resources/naxwe/ereyfur.md` records `tiraale` and `tiraale jago`; `resources/qaamuus/02-t.md` defines `tiraale` and its `tiraale jago` subtype; `resources/qaamuus/00-abbreviations.md` independently records `Jagaale` | Retain the two-way taxonomy and the source term `jagaale`. Present `tiraale jago` as the glossary-aligned full label and attribute the noun analysis to this chapter rather than turning it into an exceptionless SLS rule. | `repository-supported`; `terminology-alignment`; `intentional-retained` |
| N05-R002 | 6-24 | Clean cardinal table with supported forms, but its inventory is visibly selective: it stops the unit sequence at 15 and omits 60 between 50 and 70 as well as later decades; the repository cannot show whether the 60 gap is source damage or deliberate selection / medium | Dictionary entries support the displayed forms and also independently support `lixdan`, `siddeetan`, and `sagaashan`; `malyan` is recorded as a variant of `malyuun`; chapter 17 distinguishes counting `kow` from attributive `hal` | Retain every displayed row, including `labo` and `hal malyan`. Explicitly label the table a representative inventory; do not silently reconstruct missing rows or claim completeness. Link the fuller `kow`/`hal` discussion. | `repository-supported`; `unresolved`; `intentional-retained` |
| N05-R003 | 26-45 | Cardinal-before-noun examples are useful, but the explanation incorrectly reduces counted-noun morphology to grammatical gender and then contradicts itself by calling `fardood` a feminine-noun pattern even though `faras` is masculine / high | `resources/qaamuus/13-f.md` marks `faras` masculine and gives plural `fardo`; `resources/naxwe/17-naxwaha-af-soomaaliga.md` records counted forms with both `-ood` and `-yood`; the same file supports `kow` versus `hal` | Retain both table columns, all six phrases, `lix fardood`, the `fardo + -ood` analysis, and the `kow`/`hal` distinction. Replace the gender-only rule with a description of the two counted-noun patterns illustrated here; state that gender alone does not predict the pattern, and link the broader noun and numeral discussions. | `repository-supported`; `classification-correction`; `interpretive-qualification` |
| N05-R004 | 47-56 | The count/mass distinction and all three measurement examples are sound, but “only” and “impossible” make the restriction broader than the ordinary unmeasured readings shown, and `walxaha tirsami kari maayo` has faulty plural agreement / medium | `resources/naxwe/ereyfur.md` records `tirsame`; `resources/qaamuus/02-t.md` defines it; `resources/qaamuus/17-m.md` defines `matirsame` with `biyo, sonkor, saliid`; `resources/sarfe/01-magacyada.md` and chapter 2 classify `sonkor` as mass/non-count | Retain the subsection, all three starred phrases, and all three measure expressions. Scope the restriction to ordinary direct, unmeasured mass-noun use and replace the faulty agreement with an impersonal formulation such as `walxaha si toos ah looma tirin karo`; add noun-class navigation. | `repository-supported`; `grammar-correction`; `interpretive-qualification` |
| N05-R005 | 58-61 | Approximate-numeral examples are supported, but the stated suffix `-eeya` conflicts with both displayed forms ending in `-eeye`; the prose then switches without explanation to plural/collective forms in `-eeyo`. The “only ten and after” limit is broader than the evidence displayed / high | Dictionary entries record `tobaneeye/tobaneeyo`, `labaatameeye/labaatameeyo`, and `sagaashameeye/sagaashameeyo`; no numeral suffix `-eeya` is established by this section's forms | Correct `-eeya` to singular `-eeye`, distinguish the related `-eeyo` forms, retain both examples, and scope the range statement to the decade series illustrated by the source rather than a universal ban on every other approximation strategy. | `repository-supported`; `form-correction`; `interpretive-qualification` |
| N05-R006 | 63-67 | Clear ordinal definition, but the heading uses `Tiro jagaale` while the glossary's maintained compound is `tiraale jago` / low | `resources/naxwe/ereyfur.md`; `resources/qaamuus/02-t.md`; chapter 17 uses the source-compatible phrase `tiro jagaale` | Normalize the heading to `Tiraale jago`, retain `jagaale` as an explicitly recorded short/source term, and preserve the definition without adding a new taxonomy. | `repository-supported`; `terminology-alignment` |
| N05-R007 | 68-81 | Twelve-row ordinal table is coherent; its forms, including variant `labo iyo tobanaad` and `kumaad`, should not be silently regularized / low | Dictionary entries support the simple ordinal forms; chapter 13 independently maps `kun` to `kumaad`; `labo` is a recorded variant of `laba` | Retain every table row exactly. Change only the column labels if needed to match `Tiraale jago`; do not normalize `labo` or replace `kumaad`. | `repository-supported`; `intentional-retained` |
| N05-R008 | 83-88 | The general `-aad` derivation and cardinal-before/ordinal-after contrast are useful, but “formed by attaching `-aad`” hides the surface adjustments visible in `kow → koowaad` and other rows / medium | Chapter 17 and dictionary ordinal entries support `-aad` as the general pattern; the current table itself shows non-mechanical surface forms | Retain all three examples and the ordering contrast. Describe `-aad` as the general derivational pattern with form adjustments, not as literal unchanged concatenation in every row; add no replacement examples. | `repository-supported`; `interpretive-qualification` |

## Proposed SLS-native blueprint

The cleaned chapter should retain its existing sequence:

1. cardinal and ordinal terminology;
2. the representative cardinal-number table;
3. cardinal position before the counted noun;
4. the two counted-noun patterns illustrated by the source, including `-ood`;
5. mass nouns and measure expressions;
6. approximate numerals in `-eeye/-eeyo`; and
7. ordinal formation and position after the noun.

All rows of the three tables and all three bulleted measurement examples should
remain. The cleanup must not silently complete the selective cardinal table,
replace dictionary-supported variants such as `labo` or `malyan`, infer a new
ordinal in place of `kumaad`, or turn the chapter's short descriptions into
exceptionless SLS rules.

Recommended links are `02-sarfaha-magacyada.md`,
`17-naxwaha-af-soomaaliga.md`, and `../sarfe/01-magacyada.md`.

## Audit approval

- **Audit approval:** approved by the maintainer on 2026-08-12 with the
  instruction, "go ahead."
- **Approved finding IDs:** N05-R001 through N05-R008
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Cleanup result and review

### Applied cleanup

- Aligned the full ordinal term with glossary-supported `tiraale jago` while
  retaining `jagaale` as a supported alternative/source term.
- Labeled the cardinal table as representative and retained every displayed
  value rather than reconstructing its unresolved gaps.
- Replaced the contradictory feminine-only account of `-ood` with the two
  counted-noun patterns actually illustrated, including masculine
  `faras → fardood`, and retained the `kow`/`hal` distinction.
- Scoped the mass-noun restriction to ordinary direct, unmeasured use and
  repaired the plural-agreement error in its explanation.
- Corrected approximate-numeral suffix `-eeya` to `-eeye`, distinguished the
  related `-eeyo` forms, and qualified the range statement.
- Presented `-aad` as the general ordinal pattern with possible surface-form
  adjustments and added three resolving links to noun and numeral resources.

### Deliberately retained

- Every data row in all three tables, including `labo`, `hal malyan`,
  `labo iyo tobanaad`, and `kumaad`.
- All six counted-noun phrases, `lix fardood`, both approximate-numeral
  examples, and all three ordinal examples.
- All three bulleted measurement examples and all three starred mass-noun
  phrases.
- The original topic sequence; no missing cardinal or ordinal form was
  inferred or inserted.

### Validation

- `git diff --check`: passed.
- Original/current table-data comparison: no differences; only the two
  approved descriptive header rows changed.
- Original/current bulleted measurement examples: no differences.
- Structure: one H1, two H2, one H3; three Markdown tables with 36 total
  rows; three bulleted examples.
- Local-link resolution: 3 occurrences checked, 0 missing.
- Provenance TSV validation: every row has 10 tab-separated fields.
- Supported correction: 0 occurrences of suffix `-eeya`; `-eeye` and the
  related `-eeyo` explanation are present.
- Retained forms: `labo`, `malyan`, `kumaad`, `lix fardood`, and all measure
  expressions remain.
- Cleaned file size: 102 lines; 668 words; 3,921 bytes.
- Cleaned SHA-256:
  `75c497325d3b07307f7ecaa7407a765278fad3874db84f70bc7c72fff71f943e`.
