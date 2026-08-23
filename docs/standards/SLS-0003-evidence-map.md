# SLS-0003 Grammar Evidence Map

- **Status:** reviewed evidence map; approved 2026-08-23 as input to the
  initial SLS-0003 `Draft` and retained for its `Proposed` review;
  non-normative
- **Prepared:** 2026-08-23
- **Target standard:** SLS-0003 Somali Grammar Standard
- **Normative dependencies:** SLS-0001 and SLS-0002

This document maps the maintained grammar evidence to the eight topic files
implemented by SLS-0003. It is a drafting and review record, not a grammar
standard. The founding maintainer's instruction to complete Milestone 2
sponsors the bounded decisions below under the interim-Council provision in
`GOVERNANCE.md`.

## Evidence policy

1. **Maintained grammar core** —
   [`resources/naxwe/00-luqadda-iyo-fekerka.md`](../../resources/naxwe/00-luqadda-iyo-fekerka.md)
   through
   [`resources/naxwe/12-noocyada-weeraha.md`](../../resources/naxwe/12-noocyada-weeraha.md)
   form the primary descriptive synthesis.
2. **Source-based supplements** — `naxwe/13`–`17` add classifications and
   analyses but do not silently override the core.
3. **Paradigm evidence** —
   [`resources/sarfe/`](../../resources/sarfe/) provides compact reviewed noun
   and verb tables. A listed paradigm supports the listed forms, not an
   unrestricted productive algorithm.
4. **Terminology** —
   [`resources/naxwe/ereyfur.md`](../../resources/naxwe/ereyfur.md) supplies
   Somali terms and English glosses; topic chapters supply the actual analyses.
5. **Source limitations** — the resource layer is an SLS editorial synthesis
   of scanned books. Explicitly uncertain forms, source-only analyses, damaged
   tables, and dialect variation remain non-normative unless separately
   resolved.
6. **Example policy** — positive and negative examples in the first Draft are
   taken from, or narrowly adapted from, reviewed source pairs. English glosses
   are editorial aids and do not add a linguistic rule.

## Implemented topic set

| Local ID | Topic | Principal evidence | Initial normative boundary |
| --- | --- | --- | --- |
| 0010 | Parts of speech | [`naxwe/13` §2](../../resources/naxwe/13-aasaaska-naxwaha.md#2-sagaalka-qaybood-ee-hadalka), `naxwe/01`–`07` | Use the nine maintained labels; classify by form, meaning, and sentence function rather than spelling alone. |
| 0011 | Noun morphology, gender, plurals | [`naxwe/02`](../../resources/naxwe/02-sarfaha-magacyada.md), [`naxwe/03`](../../resources/naxwe/03-sarfaha-tifaftireyaasha.md), [`sarfe/01`](../../resources/sarfe/01-magacyada.md) | Gender is grammatical and agreement-visible; count, mass, collective, and reviewed plural classes stay distinct. |
| 0012 | Verb system: tense, aspect, mood | [`naxwe/07`](../../resources/naxwe/07-sarfaha-falalka.md), [`naxwe/08`](../../resources/naxwe/08-hogatuska-baradigmaha-falalka.md), [`sarfe/02`](../../resources/sarfe/02-falalka.md) | Preserve reviewed person/number/gender agreement, tense/aspect contrasts, and mood paradigms without generating unattested forms. |
| 0013 | Pronouns | [`naxwe/04`](../../resources/naxwe/04-sarfaha-magacuyaallada.md), `naxwe/09` | Distinguish independent and clitic pronouns, inclusive/exclusive first-person plural, visible object clitics, impersonal *la*, and reflexive/reciprocal *is*. |
| 0014 | Sentence structure and word order | [`naxwe/09`](../../resources/naxwe/09-weer-fudud.md), [`naxwe/10`](../../resources/naxwe/10-dhismaha-oraah-magaceedyada.md), [`naxwe/11`](../../resources/naxwe/11-weerta-adag.md) | Require a predicate, honor predicate valency, and preserve the reviewed *baa/ayaa/waa*, subject-clitic, and noun-phrase linker constraints. |
| 0015 | Negation | [`naxwe/12` §12.3–12.4](../../resources/naxwe/12-noocyada-weeraha.md#123-weer-diidmo), [`sarfe/02`](../../resources/sarfe/02-falalka.md#diidmada--fal-cun) | Distinguish free verbal *ma*, focused *-aan*, nominal *ma aha*, negative imperative *ha*, and negative-question combinations. |
| 0016 | Question formation | [`naxwe/12` §12.2](../../resources/naxwe/12-noocyada-weeraha.md#122-weer-weyddiimeed), [`naxwe/03` §3.3](../../resources/naxwe/03-sarfaha-tifaftireyaasha.md#33-weyddiimaha) | Cover reviewed yes/no focus constructions, *miyaa*, wh-forms, and *-kee/-tee* without claiming every dialectal question paradigm. |
| 0017 | Common mistakes | Source-marked negative pairs in `naxwe/02`, `03`, `09`, `10`, and `12` | Consolidate only construction-specific errors already supported by a positive/negative contrast. |

## Cross-topic decisions

### G1 — Normative selection versus descriptive coverage

The eight specs define a reviewed core, not every grammatical construction in
the resource library. A descriptive form is not normative merely because it
appears in `resources/`.

### G2 — Word class

The first Draft retains the nine-class school-grammar inventory. Alternative
linguistic taxonomies may be documented, but implementations of this version
use the maintained labels for interoperability.

### G3 — Gender and agreement

Somali noun gender is grammatical and applies to inanimate nouns as well as
animate nouns. Agreement evidence, not natural sex alone, determines the
normative examples. Reviewed plural gender can differ from singular gender.

### G4 — Paradigm boundary

Only reviewed forms and explicitly described classes are normative. Tools must
not generate a noun plural, verb form, pronoun combination, or sound change by
analogy outside a documented class.

### G5 — Focus and subject-clitic rules

The first Draft adopts only the word-order environments for *baa/ayaa/waa* and
subject clitics that have explicit positive/negative pairs. Source analyses
labeled as limited to one reading remain limited.

### G6 — Negation and questions

Orthographic surface forms governed by SLS-0002 are not re-segmented. SLS-0003
defines the grammatical environment of the reviewed negative and question
forms; it does not turn source-historical decompositions into spelling rules.

### G7 — Variation and uncertain forms

Dialect forms and explicitly unresolved resource forms are neither corrected
nor made errors. A negative example is normative only for the construction and
reading stated in its rule.

### G8 — Common mistakes

The common-mistakes file is a cross-reference and correction surface. It must
not introduce a new rule unsupported by topic evidence.

## Scope resolutions

| Question | Milestone 2 resolution |
| --- | --- |
| Does SLS-0003 standardize every paradigm in the books? | No. It standardizes reviewed forms/classes and prohibits unsupported generation. |
| Are dialect differences errors? | No. They require scope labels and future SLS-0700 treatment. |
| Does word order alone always identify subject and object? | No. Predicate meaning, case, agreement, focus particles, clitics, and order work together. |
| Are all omitted third-person objects absent? | No. The reviewed analysis permits a zero third-person object interpreted from context. |
| Are *baa* and *ayaa* universally interchangeable? | The maintained focus constructions treat their function together; no regional or universal distribution rule is invented. |
| Does one negative suffix cover every tense and verb class? | No. The reviewed paradigm determines the negative form. |
| Does one question strategy cover every question? | No. The Draft distinguishes yes/no, content, selection, and indirect patterns only where reviewed. |
| Can a source-marked error be generalized to every possible reading? | No. Each judgment is construction- and reading-specific. |

## Drafting gates — satisfied 2026-08-23

- every topic file has the lightweight spec front matter and required sections;
- every normative rule has at least one positive and one negative three-column
  example row;
- all example judgments trace to the mapped resource family;
- source-only and uncertain forms are excluded from normative examples;
- the wrapper carries the formal SLS-0003 lifecycle and compliance surface;
- dependencies point to SLS-0001 and SLS-0002 without creating a cycle;
- the founding maintainer sponsored the planned-to-`Draft` transition.

## Lifecycle handoff

The approved evidence boundary is implemented by
[`spec/grammar/0010-parts-of-speech.md`](../../spec/grammar/0010-parts-of-speech.md)
through
[`spec/grammar/0017-common-mistakes.md`](../../spec/grammar/0017-common-mistakes.md)
and the
[formal SLS-0003 wrapper](../../spec/grammar/0018-somali-grammar-standard.md).
The founding maintainer accepted that completed Draft for `Proposed`
publication as part of Milestone 2. Public comment begins when the proposal
branch is published; the transition does not replace the required public
linguistic and technical review.
