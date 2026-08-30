# SLS-0003 Grammar Evidence Map

- **Status:** reviewed evidence map; approved 2026-08-23 as input to the
  initial SLS-0003 `Draft`, retained for its `Proposed` review, and supplemented
  by recorded maintainer native-speaker review on 2026-08-30; non-normative
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
7. **Review supplements** — a maintainer or public-comment judgment may refine
   the initial evidence boundary only when its wording, disposition, and
   resulting change are recorded in the SLS-0003 review log. Independent
   corroboration remains welcome.

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
use the maintained labels for interoperability. Maintainer native-speaker
review confirms `tifaftire` as a determiner/quantifier subcategory inside the
`tilmaame` modifier domain rather than a tenth primary class.

### G3 — Gender and agreement

Somali noun gender is grammatical and applies to inanimate nouns as well as
animate nouns. Agreement evidence, not natural sex alone, determines the
normative examples. Plural formation is lexical and must be recorded per noun;
once the reviewed plural form and its ending class are known, plural-gender
polarity follows the regular agreement pattern of that class.

### G4 — Paradigm boundary

Only reviewed forms and explicitly described classes are normative. Tools must
not generate a noun plural from a singular, or generate a verb form, pronoun
combination, or sound change by analogy outside a documented class.

### G5 — Focus and subject-clitic rules

The first Draft adopts only the word-order environments for *baa/ayaa/waa* and
subject clitics that have explicit positive/negative pairs. Source analyses
labeled as limited to one reading remain limited.

### G6 — Negation and questions

Orthographic surface forms governed by SLS-0002 are not re-segmented. SLS-0003
defines the grammatical environment of the reviewed negative and question
forms; it does not turn source-historical decompositions into spelling rules.
Recorded review further fixes the two nominal-predicate positions of `miyaa`,
the clause-initial `ma ... baa ... ah` alternative, and the inheritance of
definite-article allomorphy by the `-kee`/`-tee` interrogative series.

### G7 — Variation and uncertain forms

Dialect forms and explicitly unresolved resource forms are neither corrected
nor made errors. A negative example is normative only for the construction and
reading stated in its rule. This version targets Standard Somali
(`Aqoondhari` / `Soomaali Maxaa tiri`); identified Benaadir, Maay, and other
regional profiles return `not covered` pending dedicated SLS-0700 standards.

### G8 — Common mistakes

The common-mistakes file is a cross-reference and correction surface. It must
not introduce a new rule unsupported by topic evidence.

## Maintainer-review supplements — 2026-08-30

These decisions supplement rather than replace the source families above. The
reviewer's complete wording and each written disposition are preserved in the
[SLS-0003 review log](SLS-0003-review-log.md#comments).

| Decision | Rules | Evidence added | Result |
| --- | --- | --- | --- |
| MR-5 / Q2 | G11-R3, G11-R5 | Native-speaker judgment that plural formation is lexically unpredictable, while gender polarity is regular after the plural form and ending are known | Record each noun's plural; enforce the reviewed plural class's gender polarity |
| MR-6 / Q6 | G16-R2 | Native-speaker judgment on nominal-predicate `miyaa` placement | License post-subject and predicate-final `miyaa`; reject it clause-initially for this reading; license clause-initial `ma ... baa ... ah` |
| MR-7 / Q7 | G16-R4 | Native-speaker judgment that interrogative and definite suffixes share gender classes and consonant changes without exception | Reuse definite-article allomorphy, including `-da → -dee` and `-sha → -shee` |
| MR-8 / Q8 | G11-R4 | Native-speaker distinction between direct mass counting, contextual unit ellipsis, and formal measure phrases | Reject the direct mass reading; accept contextual unit ellipsis; prefer an explicit formal measure |
| MR-9 / Q9 | G13-R4 | Native-speaker boundary between overt `iyaga` and a recoverable singular third-person zero object | Preserve an explicit object noun or independent pronoun when it supplies plurality or prevents ambiguity; license zero only without explicit object-noun, first/second-person, or independent-pronoun cues |
| MR-10 / Q10 | G10-R1 | Native-speaker confirmation of the nine primary classes and traditional treatment of `tifaftire` | Retain the nine-class inventory and keep `tifaftire` within the modifier domain |
| MR-11 / Q11 | G12-R1, G13-R1, G13-R3, G17-R1, G17-R2 | Native-speaker judgment that agreement errors are diagnosable, with reduced agreement under plural subject focus and in relative clauses | Expand agreement/focus diagnostics; accept `Nimankii baa yimid`; return `not covered` for unreviewed relative-clause patterns |
| MR-12 / Q12 | SLS-0003 R3, R6 | Native-speaker identification of Benaadir and Maay false-positive risk | Declare the Standard-Somali target; return `not covered` for identified regional profiles and defer them to SLS-0700 |
| MR-13 | G10-R1 | Native-speaker per-rule approval of the nine primary classes, the `tifaftire` subcategory, and the `qor` classification contrast | Retain the rule and examples unchanged |
| MR-14 | G10-R2 | Native-speaker per-rule approval of nominal-function and noun-behaviour diagnostics, including `ninka` | Retain the rule and examples unchanged |
| MR-15 | G10-R3 | Native-speaker per-rule approval of context-sensitive classification, including modifier and independent-pronoun uses of `kan` | Retain the rule and examples unchanged |
| MR-16 | G10-R4 | Native-speaker per-rule approval of the linker, conjunction, and preposition distinction, including prepositional `ku` | Retain the rule and examples unchanged |
| MR-17 | G11-R1 | Native-speaker per-rule approval of grammatical gender determined through syntactic agreement, including feminine `deero` | Retain the rule and examples unchanged |
| MR-18 | G11-R2 | Native-speaker per-rule approval of definite-article gender concord and phonological assimilation | Retain the rule and examples unchanged |
| MR-19 | G11-R3 | Native-speaker per-rule approval of explicit lexical storage for plural forms that cannot be safely generated from the singular | Retain the rule and examples unchanged |
| MR-20 | G11-R4 | Native-speaker per-rule approval of the mass, contextual-unit, and explicit-measure distinction for `sonkor` | Retain the rule and examples unchanged |
| MR-21 | G11-R5 | Native-speaker per-rule approval of plural gender polarity, including `inankaas`, `inamadaas`, and the rejected `inamakaas` | Retain the rule and examples unchanged |
| MR-22 | G12-R1 | Native-speaker per-rule approval of person, gender, and number agreement together with reviewed reduced agreement under plural `baa` focus | Retain the rule and examples unchanged |
| MR-23 | G12-R2 | Native-speaker per-rule approval of the `keenay` past and `keenaa` present-habitual tense contrast | Retain the rule and examples unchanged |
| MR-24 | G12-R3 | Native-speaker per-rule approval of the completed-event and past-progressive contrast between `cunay` and `cunayay` | Retain the rule and examples unchanged |
| MR-25 | G12-R4 | Native-speaker per-rule approval of second-person imperatives and the third-person `ha` directive construction | Retain the rule and examples unchanged |
| MR-26 | G12-R5 | Native-speaker per-rule approval of `imow → yimid` and rejection of regularized `imoway` | Retain the rule and examples unchanged |
| MR-27 | G13-R1 | Native-speaker per-rule approval of independent-pronoun gender agreement for `Faadumo`, `iyada`, and `isaga` | Retain the rule and examples unchanged |
| MR-28 | G13-R2 | Native-speaker per-rule approval of the `annaga`/`innaga` contrast and its casual-neutralization boundary | Retain the rule, examples, and edge case unchanged |
| MR-29 | G13-R3 | Native-speaker per-rule approval of second-person subject-clitic agreement in `Adigu moos baad cuntay` | Retain the rule and examples unchanged |
| MR-30 | G13-R4 | Native-speaker per-rule approval of required first/second-person object clitics and the reviewed overt/zero third-person contrast | Retain the rule and examples unchanged |
| MR-31 | G13-R5 | Native-speaker per-rule approval of reflexive and reciprocal `is` and the meaning change caused by replacement with `la` | Retain the rule and examples unchanged |
| MR-32 | G13-R6 | Native-speaker per-rule approval of impersonal `la` for an unspecified agent and rejection of an explicit-agent reading | Retain the rule and examples unchanged |
| MR-33 | G14-R1 | Native-speaker per-rule approval of predicate completeness and the complete imperative `Kaaley!` | Retain the rule and examples unchanged |
| MR-34 | G14-R2 | Native-speaker per-rule approval of selected preposition `ku` with `fariistay` | Retain the rule and examples unchanged |
| MR-35 | G14-R3 | Native-speaker per-rule approval of focus-particle placement after the focused constituent and before the verbal group | Retain the rule and examples unchanged |
| MR-36 | G14-R4 | Native-speaker per-rule approval of the required subject clitic in object focus and the meaning change caused by bare `baa` | Retain the rule and examples unchanged |
| MR-37 | G14-R5 | Native-speaker per-rule approval of clitic–verb adjacency in the reviewed `waa` construction | Retain the rule and examples unchanged |
| MR-38 | G14-R6 | Native-speaker per-rule approval of linker `ee` between two post-nominal modifiers | Retain the rule and examples unchanged |
| MR-39 | G15-R1 | Native-speaker per-rule approval of preverbal `ma` with the licensed negative verb form | Retain the rule and examples unchanged |
| MR-40 | G15-R2 | Native-speaker per-rule approval of negative nominal-predicate `ma aha` and rejection of retained affirmative `waa` | Retain the rule and examples unchanged |

## Scope resolutions

| Question | Milestone 2 resolution |
| --- | --- |
| Does SLS-0003 standardize every paradigm in the books? | No. It standardizes reviewed forms/classes and prohibits unsupported generation. |
| Are dialect differences errors? | No. They require scope labels and future SLS-0700 treatment. |
| Does word order alone always identify subject and object? | No. Predicate meaning, case, agreement, focus particles, clitics, and order work together. |
| Are all omitted third-person objects absent? | No. `Cali baa u yeeray.` permits a recoverable singular third-person zero object; `Cali baa iyaga u yeeray.` overtly identifies a plural object. |
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
publication as part of Milestone 2. Public comment began in
[pull request #7](https://github.com/goobolabs/somali-language-standard/pull/7)
on 2026-08-23. Under SLS-0000 0.2.0, the maintainer records lifecycle decisions
and the correction channel remains open at every stage; independent linguistic
and technical review is welcome but is not a gate.
