---
id: "0018"
sls_id: SLS-0003
title: Somali Grammar Standard
version: 0.1.1
standard_version: 0.1.1
status: Proposed
category: foundation
owner: language-council
reviewers: []
dependencies:
  - SLS-0001
  - SLS-0002
implements:
  - spec/grammar/0010-parts-of-speech.md
  - spec/grammar/0011-noun-morphology-gender-plurals.md
  - spec/grammar/0012-verb-system-tense-aspect-mood.md
  - spec/grammar/0013-pronouns.md
  - spec/grammar/0014-sentence-structure-word-order.md
  - spec/grammar/0015-negation.md
  - spec/grammar/0016-question-formation.md
  - spec/grammar/0017-common-mistakes.md
  - spec/grammar/0018-somali-grammar-standard.md
publication_date: null
supersedes: null
superseded_by: null
revision_history:
  - version: 0.1.0
    date: 2026-08-23
    change: Initial evidence-mapped draft
  - version: 0.1.1
    date: 2026-08-30
    change: "Editorial: added a lifecycle banner to topic files 0010–0017; linked the public review log"
---

> **Proposed status:** This document and its implemented topic specifications
> are accepted for formal public comment. Their scope is frozen for the comment
> period, but their requirements remain non-stable and may be refined through
> recorded review decisions.

## Abstract

SLS-0003 defines a reviewed core of Somali word classes, noun and verb
morphology, pronouns, clause structure, negation, question formation, and
construction-specific error diagnostics. The standard is implemented as eight
focused topic specifications plus this lifecycle and compliance wrapper.

## Purpose

The maintained grammar resources preserve valuable analyses and examples from
Somali grammar books, but descriptive coverage alone does not establish a
normative rule. SLS-0003 selects a bounded, testable core, identifies the
construction to which each judgment applies, and prevents validators from
turning uncertain, dialectal, or unattested patterns into automatic errors.

## Scope

**In scope:** the nine maintained parts of speech; reviewed noun gender,
article, number, and plural behaviour; reviewed verb agreement and
tense/aspect/mood contrasts; independent and clitic pronouns; core predicate,
focus, verbal-group, preposition, and modifier-linking constructions;
construction-specific negation and questions; and diagnostics that directly
trace to those rules.

**Out of scope:** an exhaustive grammar of every Somali variety; unrestricted
generation of noun or verb paradigms; prosody and phonetic transcription;
historical reconstruction; universal discourse or focus analysis; exhaustive
valency and lexical selection; and dialect classification. These require
reviewed lexical data or future standards, including the SLS-0600 and SLS-0700
blocks.

## Definitions

- **Reviewed construction** — a grammatical pattern whose form, interpretation,
  and positive or negative evidence have been approved for the stated rule.
- **Normative example** — an example used to demonstrate conformance or
  non-conformance for one identified construction and interpretation.
- **Agreement** — a grammatical dependency in which person, number, or gender
  features determine a reviewed form.
- **Clitic** — a grammatically dependent form with a documented position or
  host in its construction.
- **Focus particle** — a form such as `baa` or `ayaa` that participates in the
  reviewed information-structure constructions described by this standard.
- **Paradigm** — the reviewed set of inflected forms for a lexeme or class.
- **Not covered** — the required result when a validator lacks an applicable
  reviewed rule; it is distinct from both conforming and non-conforming.
- **Topic specification** — one of the eight files numbered 0010 through 0017
  and listed in this standard's `implements` metadata.

## Normative Requirements

- **R1.** Text claiming full SLS-0003 conformance **MUST** satisfy SLS-0001 and
  SLS-0002 for its alphabet, encoding, and spelling. Grammar processing **MUST
  NOT** silently undo a spelling required by either dependency.
- **R2.** A full SLS-0003 implementation **MUST** implement every applicable
  **MUST** and **MUST NOT** requirement in topic specifications 0010–0017. A
  partial implementation **MUST** identify its implemented topics and **MUST
  NOT** claim full SLS-0003 conformance.
- **R3.** A normative grammar judgment **MUST** identify or inherit the reviewed
  construction and intended reading to which it applies. Dialectal, disputed,
  or source-uncertain material **MUST NOT** be reported as an error without a
  separately approved scope rule.
- **R4.** A grammar implementation **MUST** use reviewed paradigms and lexical
  classes. It **MUST NOT** manufacture an unreviewed plural, inflection,
  clitic combination, or morphophonemic result by analogy alone.
- **R5.** Agreement, clitic, focus, and word-order checks **MUST** use the
  grammatical features and construction named by the applicable topic rule.
  A checker **MUST NOT** diagnose those relations from spelling or linear
  position alone.
- **R6.** A negative example **MUST** be treated as non-conforming only for the
  rule, construction, and interpretation stated in its row. Its asterisk or
  diagnostic label **MUST NOT** be generalized to every possible context or
  variety.
- **R7.** A correction tool **MUST** identify the violated topic rule and
  relevant grammatical feature. It **MUST NOT** silently rewrite a form when
  more than one correction or interpretation remains possible.
- **R8.** English translations and compact glosses in the topic files are
  informative aids. Implementations **MUST NOT** derive a Somali grammar rule
  from the English wording when no Somali rule or example licenses it.

## Recommendations

- Implementers SHOULD begin with one topic profile and declare partial
  conformance until all eight profiles are covered.
- Editors SHOULD add a form to a paradigm only with a traceable source and a
  recorded review decision.
- Validators SHOULD return `not covered` for unknown lexical or dialectal
  patterns instead of guessing.
- Diagnostics SHOULD preserve the user's original sentence and propose a
  change separately.
- Reviewers SHOULD consult the
  [`SLS-0003 evidence map`](../../docs/standards/SLS-0003-evidence-map.md)
  before expanding the standard's scope.

## Examples

| Somali | Analysis | Status in this draft |
| --- | --- | --- |
| `goʼaan` | `[R1 +]` canonical glottal-stop spelling retained | conforms to the dependency requirement |
| `go'aan` | `[R1 −]` ASCII apostrophe substituted | does not conform to SLS-0001 through R1 |
| `Adigu moos baad cuntay.` | `[R2 +]` topic G13-R3 implemented | conforming subject-clitic agreement |
| `Adigu moos baa cuntay.` | `[R2 −]` applicable topic rule omitted | cannot support full conformance |
| `qaab goboleed — not covered` | `[R3 +]` variation withheld from diagnosis | correct when no approved scope rule exists |
| `qaab goboleed — qalad` | `[R3 −]` unlabeled regional form declared wrong | prohibited without a scope rule |
| `buug → buugag` | `[R4 +]` reviewed plural | conforming reviewed paradigm use |
| `buug → buugyo` | `[R4 −]` unsupported generated plural | prohibited automatic generation |
| `Deeradu biyo bay cabtay.` | `[R5 +]` feminine agreement evidence applied | conforming construction-aware check |
| `deero = feminine because animate` | `[R5 −]` feature inferred from meaning alone | insufficient agreement analysis |
| `Cali wuu Axmed dilay.` | `[R6 +]` non-conforming under G14-R5's stated reading | bounded diagnostic |
| `every occurrence of this order is wrong` | `[R6 −]` judgment generalized beyond G14-R5 | prohibited generalization |
| `Wiilkii moos baa cunay. — G14-R4: subject clitic missing` | `[R7 +]` rule and feature named | conforming diagnostic |
| `Wiilkii moos baa cunay. → fixed automatically` | `[R7 −]` silent rewrite without recorded diagnosis | non-conforming correction behaviour |
| `Annaga` = “we, excluding the addressee” | `[R8 +]` English explains the Somali distinction | informative aid |
| English “we” ⇒ `annaga` in every context | `[R8 −]` Somali rule inferred from English ambiguity | prohibited derivation |

## Edge Cases

- **Grammatical sentence, wrong intended reading.** Some negative rows contain a
  form that can be grammatical under another interpretation. The stated
  reading, not the string alone, determines the diagnostic.
- **Third-person zero object.** The absence of a visible object clitic can be
  licensed for a reviewed third-person interpretation. It is not automatically
  a missing-object error.
- **Inclusive and exclusive plural.** A validator can enforce the contrast only
  when the intended inclusion of the addressee is available.
- **Focus variation.** The Draft covers only the environments supported by
  explicit source pairs. It does not claim a universal distribution for
  `baa`, `ayaa`, or every clitic realization.
- **Unknown forms.** Absence from the Draft means `not covered`, not necessarily
  non-conforming.
- **Source quotation.** Faithful quotation may preserve historical, regional,
  or source-specific grammar when clearly marked; it does not create a new
  normative rule.

## Compliance Requirements

Because SLS-0003 is at `Proposed`, this checklist is provisional and does not
support a `Stable` compliance claim.

| # | Requirement | Traces to | Level |
| --- | --- | --- | --- |
| C1 | Alphabet and spelling dependencies are preserved | R1 | MUST |
| C2 | Every applicable topic requirement is implemented, or the implementation is labelled partial | R2 | MUST |
| C3 | Judgments remain bounded to reviewed constructions and scope | R3, R6 | MUST |
| C4 | Unreviewed paradigms and forms are not generated by analogy | R4 | MUST |
| C5 | Agreement, clitic, focus, and word-order checks use grammatical features | R5 | MUST |
| C6 | Diagnostics name the applicable rule and do not silently rewrite ambiguity | R7 | MUST |
| C7 | English glosses remain informative rather than normative | R8 | MUST |
| C8 | Parts-of-speech profile 0010 is implemented | R2, G10-R1–G10-R4 | MUST |
| C9 | Noun and verb morphology profiles 0011–0012 are implemented | R2, G11-R1–G12-R5 | MUST |
| C10 | Pronoun and clause-structure profiles 0013–0014 are implemented | R2, G13-R1–G14-R6 | MUST |
| C11 | Negation and question profiles 0015–0016 are implemented | R2, G15-R1–G16-R5 | MUST |
| C12 | Common-mistakes profile 0017 introduces no independent rules | R2, G17-R1–G17-R6 | MUST |

## References

- SLS-0000, *SLS Standards Process Standard*.
- SLS-0001, *Somali Alphabet Standard*.
- SLS-0002, *Somali Orthography Standard*.
- [`SLS-0003 Grammar Evidence Map`](../../docs/standards/SLS-0003-evidence-map.md).
- Guddiga Af Soomaaliga, *Aasaaska Naxwaha Af Soomaaliga* (1973), and Abdalla
  Omar Mansur and Annarita Puglielli, *Barashada Naxwaha Af Soomaaliga* (1999),
  curated in [`resources/naxwe/`](../../resources/naxwe/).
- Reviewed paradigms curated in
  [`resources/sarfe/`](../../resources/sarfe/).
- [`SLS-0003 review log`](../../docs/standards/SLS-0003-review-log.md) — the public-comment record for this standard.

## Revision History

| Version | Date | Change |
| --- | --- | --- |
| 0.1.0 | 2026-08-23 | Initial evidence-mapped draft |
| 0.1.1 | 2026-08-30 | Editorial: added a lifecycle banner to topic files 0010–0017; linked the public review log |
