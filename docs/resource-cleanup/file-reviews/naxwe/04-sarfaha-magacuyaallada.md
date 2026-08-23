# Audit record — Sarfaha magacuyaallada

- **Resource path:** `resources/naxwe/04-sarfaha-magacuyaallada.md`
- **Collection / family:** naxwe / primary grammar
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; conservative SLS cleanup applied and awaiting review
- **Audit started:** 2026-08-12
- **File size at audit start:** 173 lines; 1,036 words; 6,417 bytes
- **Resource SHA-256 at audit start:**
  `fed5d327f0051294e102dcd1f147c825582b302d605df82f9c382ee1294b8d4c`
- **Resource-text changes during audit:** none

## Target output model

This file is already a compact primary-grammar chapter, not an OCR-shaped
book transcript. It has one H1, three H2 sections, five H3 subsections, two
valid three-column tables, and 26 numbered example lines. It contains no front
matter, exercises, scan debris, false headings, fenced fragments, or damaged
reading order.

Cleanup should retain its complete coverage of independent and reduced
personal pronouns, inclusive/exclusive first-person plural, object forms,
`la`, reflexive/reciprocal `is`, independent possessives, demonstratives,
indefinite pronouns, and interrogative pronouns. It should make the single
uniquely supported paradigm correction, repair one uniquely supported word
error, scope several source analyses, preserve three forms whose replacements
are not uniquely established, and add links to the determiner, clitic syntax,
preposition/clitic-combination, and question resources.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-9 | reviewed | N04-R001 |
| 10-35 | reviewed | N04-R002 |
| 37-53 | reviewed | N04-R003 |
| 55-70 | reviewed | N04-R004 |
| 72-84 | reviewed | N04-R005 |
| 86-109 | reviewed | N04-R006 |
| 111-125 | reviewed | N04-R007 |
| 127-136 | reviewed | N04-R008 |
| 138-147 | reviewed | N04-R009 |
| 149-158 | reviewed | N04-R010 |
| 160-173 | reviewed | N04-R011 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| N04-R001 | 1-9 | Clean title and basic independent/reduced distinction, but the opening sounds exhaustive before section 4.3 introduces pronominal uses of determiners / low | `resources/naxwe/ereyfur.md` supports `magacuyaal` and `magacuyaal ebyoon`; `resources/qaamuus/17-m.md` supports the independent/reduced contrast; same-file section 4.3 | Retain the title and contrast. Call it the chapter's primary personal-pronoun distinction and preview the later determiner-derived pronominal group; add navigation without creating a new paradigm. | `repository-supported`; `structural-only` |
| N04-R002 | 10-35 | Ten clean personal-pronoun examples and coherent first/second/third-person explanation / low | `resources/qaamuus/14-q.md` entry `qof²`; dictionary entries for `annaga`, `innaga`, and `idinka`; `resources/naxwe/13-aasaaska-naxwaha.md`; `resources/naxwe/17-naxwaha-af-soomaaliga.md` | Retain all ten example lines, the three participant roles, and every pronoun form. Add no replacement examples. | `repository-supported` |
| N04-R003 | 37-53 | Useful source analysis of independent pronouns as noun-like and of their gender; the wording applies the masculine/generalization to all pronoun types, and `magacuyaal sooca` differs from glossary term `magacuyaal soocan` / medium | `resources/qaamuus/17-m.md` says independent pronouns can bear an article; `resources/naxwe/ereyfur.md` records `magacuyaal soocan` and `magacuyaal mideeya`; the displayed forms support the personal-pronoun scope | Retain the noun-like distribution, subject/object examples, gender contrast, and inclusive/exclusive meanings. Scope the gender statement to the independent personal-pronoun series and normalize `magacuyaal sooca` to glossary-supported `magacuyaal soocan`. | `repository-supported`; `terminology-normalization`; `intentional-retained` |
| N04-R004 | 55-70 | The clitic table has a uniquely supported word error (`ragtoodu`) and a substantive inclusive-row defect: `innu` is unattested elsewhere and the subject cell repeats exclusive `aannu`; `ina` is internally treated as inclusive but its dictionary prose conflicts with its own inclusive-looking example / high | Repository search supports `rugta`, not `ragta`, in this syntactic sense; `resources/naxwe/14-naxwaha-cusub.md` and `15-naxwaha-sifayneed.md` list `aannu` exclusive and `aynu` inclusive; `resources/qaamuus/22-a.md` records `aynnu` as inclusive; `resources/qaamuus/24-i.md` entry `ina` has a conflicting gloss and example | Correct `ragtoodu` to `rugtoodu`. In the inclusive row, replace parenthetical `innu` with `aynu` and subject `aannu` with `aynu`. Retain object `ina` but explicitly note the internal dictionary conflict; do not infer another object form. Preserve every other row unchanged. | `repository-supported`; `unresolved` for `ina` metadata |
| N04-R005 | 72-84 | The zero third-person object analysis is independently supported, but absence of an overt object clitic is described as if it alone always identifies a third-person object / medium | `resources/naxwe/09-weer-fudud.md` §9.3 explicitly analyzes third-person object as zero and supplies the same `Waan arkay` pattern; `resources/naxwe/06-sarfaha-iskuxireyaasha.md` supplies the overt object-clitic combinations | Retain the zero paradigm, example, and four inline sentences. State that a third-person object reading depends on the verb and discourse context, and link the detailed clitic-position and combination accounts. | `repository-supported`; `intentional-retained` |
| N04-R006 | 86-109 | `la` and `is` topics and examples are valuable; “aan la ogeyn ama la garanayo” is internally contradictory, while plural `is` can be reflexive or reciprocal depending on interpretation rather than form alone / high | `resources/naxwe/17-naxwaha-af-soomaaliga.md` describes `la/loo` as marking an unmentioned actor; `resources/naxwe/15-naxwaha-sifayneed.md` uses `la` for the source's hidden voice; `resources/qaamuus/24-i.md` entry `is²` supports both reflexive and reciprocal functions | Retain examples 11-16. Replace the contradictory `la` wording with “aan la magacaabin ama aan si gaar ah loo aqoonsan.” Present examples 13-15 as the reflexive readings intended here while noting that plural `is` can depend on context; retain example 16 as reciprocal. | `repository-supported`; `interpretive-qualification` |
| N04-R007 | 111-125 | Clean transition to determiner-derived pronouns and two useful possessive examples; `Guryahaan` occurs nowhere else in the repository and a unique spelling correction cannot be established from an exact parallel / medium | `resources/naxwe/03-sarfaha-tifaftireyaasha.md` supports possessive and determiner structure; dictionary entries support `kayga`, `keenna`, and plural possessive forms; exact-form search finds `Guryahaan` only here | Retain both numbered examples exactly and label `Guryahaan` an unresolved source form. Do not silently change it to inferred `Guryahan`; link the determiner chapter. | `repository-supported` in part; `unresolved` |
| N04-R008 | 127-136 | Complete eight-row independent-possessive paradigm; all rows have repository support, including variant spellings `kayaga/tayada` and the plural forms / low | Dictionary entries `kayga`, `kaaga`, `kiisa`, `keeda`, `kaayaga/kayaga`, `taayada/tayada`, `keenna`, `teenna`, `kiinna`, `tiinna`, `kooda`, `tooda`, and corresponding `kuw-` forms | Retain the complete table unchanged. Identify `kayaga/tayada` as recorded variants only if a note is useful; do not silently lengthen them to `kaayaga/taayada`. | `repository-supported`; `intentional-retained` |
| N04-R009 | 138-147 | Demonstrative and `kii/tii` pronominal uses are clean and independently supported / low | Dictionary entries `kan²`, `tan²`, `kii¹`, and `tii¹`; `resources/naxwe/03-sarfaha-tifaftireyaasha.md` | Retain both numbered examples and both inline `kii/tii` examples. Link the full determiner treatment; no new forms are required. | `repository-supported` |
| N04-R010 | 149-158 | Indefinite-pronoun category and first three forms are supported; singular/plural `kuwe` occurs only here, while the dictionary records both `kuwo` and `kuwoo`, so a replacement is not unique / high | Dictionary entries `koo²`, `tu`, `too¹`, `kuwo`, and `kuwoo`; repository-wide exact-form search finds pronominal `kuwe` only here; `resources/naxwe/17-naxwaha-af-soomaaliga.md` lists indefinite `kuwo` | Retain the category and examples 5-8. Mark `kuwe` as unresolved and do not guess between `kuwo` and `kuwoo`; connect this classification to the clarification in chapter 3. | `repository-supported` in part; `unresolved` |
| N04-R011 | 160-173 | Interrogative inventory and both examples are supported; “only in interrogative sentences” is useful classification wording but full focus and indirect-question syntax lies outside this compact morphology chapter / low | Dictionary entries `yaa¹`, `kee¹`, `tee¹`, `kuwee`, `kuma`, and `tuma`; `resources/naxwe/12-noocyada-weeraha.md` §§12.2.3-12.2.3.4 | Retain the full inventory, both examples, and functional conclusion. Link the question chapter and frame the restriction as their interrogative use, without adding a syntax paradigm. | `repository-supported`; `structural-only` |

## Proposed SLS-native blueprint

The cleaned chapter should retain its title and existing sequence:

1. independent personal pronouns and participant roles;
2. inclusive and exclusive first-person plural;
3. reduced subject and object pronouns;
4. third-person zero object marking;
5. impersonal `la` and reflexive/reciprocal `is`;
6. independent possessive, demonstrative, indefinite, and interrogative
   pronouns; and
7. the final distributional distinction between determiners and pronouns.

All 26 numbered example lines should remain exactly as written, including the
source-only `Guryahaan` and `Kuwe` examples. The independent-possessive table
should remain unchanged. The reduced-pronoun table should receive only the
approved inclusive-row correction `innu/aannu` → `aynu/aynu`; `ina` must
remain, with the conflicting repository metadata noted rather than resolved by
inference.

Recommended links are `03-sarfaha-tifaftireyaasha.md`,
`06-sarfaha-iskuxireyaasha.md`, `09-weer-fudud.md`, and
`12-noocyada-weeraha.md`.

## Audit approval

- **Audit approval:** approved by the maintainer on 2026-08-12 with the
  instruction, "go ahead."
- **Approved finding IDs:** N04-R001 through N04-R011
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Cleanup result and review

### Applied cleanup

- Corrected `ragtoodu` to `rugtoodu` and changed only the approved inclusive
  table row from `innu/aannu` to `aynu/aynu`; object form `ina` remains.
- Normalized `magacuyaal sooca` to glossary-supported `magacuyaal soocan`.
- Scoped the noun-like and gender descriptions to independent personal
  pronouns, qualified the third-person zero-object interpretation, and clarified
  the roles of `la` and context-sensitive `is`.
- Retained `Guryahaan` and `Kuwe` exactly as source forms with explicit
  unresolved notes; no inferred spelling replacement was made.
- Added six resolving links to the determiner, clitic-combination, simple
  sentence, and question resources.

### Deliberately retained

- All 26 numbered example lines, with no textual differences from the
  pre-cleanup resource.
- Every row of the independent-possessive table.
- Every row of the reduced-pronoun table except the single approved inclusive
  row correction.
- The unresolved object form `ina`, all possessive variants, and the original
  topic sequence.

### Validation

- `git diff --check`: passed.
- Original/current numbered-example comparison: no differences.
- Original/current table comparison: exactly one changed row,
  `1aad wadar mideeya (innu) | aannu | ina` →
  `1aad wadar mideeya (aynu) | aynu | ina`.
- Structure: one H1, three H2, five H3; 26 numbered example lines; two
  three-column tables.
- Local-link resolution: 6 occurrences checked, 0 missing.
- Provenance TSV validation: every row has 10 tab-separated fields.
- Supported corrections: 0 occurrences of `ragtoodu` or standalone `innu`;
  `rugtoodu` and the corrected inclusive row each occur once.
- Unresolved-source preservation: the examples containing `Guryahaan`,
  `Kuwe`, and the table cell `ina` all remain.
- Cleaned file size: 209 lines; 1,250 words; 8,318 bytes.
- Cleaned SHA-256:
  `e33a1ecb31b76b3830228b91f00dbf88ab84025d739b668a14be9f8a154d10fc`.
