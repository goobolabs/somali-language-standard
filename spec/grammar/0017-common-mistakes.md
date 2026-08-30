---
id: "0017"
title: Common Grammar Mistakes
status: Draft
since_version: 0.2.0
category: grammar
supersedes: null
---

# Common Grammar Mistakes

> **Lifecycle:** this topic specification keeps its local spec-note status
> `Draft`. It is implemented by **SLS-0003 Somali Grammar Standard**, which is
> `Proposed` and inside its public comment period. Comments on this document
> are recorded in the
> [SLS-0003 review log](../../docs/standards/SLS-0003-review-log.md).

## Summary

This specification provides a compact diagnostic profile for recurring errors
already governed by SLS grammar rules. It introduces no independent grammar
analysis: every diagnostic refers to a rule in another SLS-0003 topic
specification.

## Rule

- **G17-R1.** A checker applying the agreement diagnostic MUST enforce
  [G11-R1](0011-noun-morphology-gender-plurals.md),
  [G12-R1](0012-verb-system-tense-aspect-mood.md), and
  [G13-R1](0013-pronouns.md) in their reviewed constructions. It MUST NOT infer
  agreement from the referent alone or diagnose a construction whose reduced
  agreement pattern has not been reviewed.
- **G17-R2.** A checker applying the focus diagnostic MUST enforce
  [G13-R3](0013-pronouns.md),
  [G14-R3](0014-sentence-structure-word-order.md) and
  [G14-R4](0014-sentence-structure-word-order.md), including the required
  subject clitic and its person, number, and gender features in the documented
  focus environments.
- **G17-R3.** A checker applying the preposition diagnostic MUST enforce
  [G14-R2](0014-sentence-structure-word-order.md) only for predicates and
  relations in its reviewed lexical data.
- **G17-R4.** A checker applying the verbal-group diagnostic MUST enforce
  [G14-R5](0014-sentence-structure-word-order.md) without treating every word
  order not shown there as an error.
- **G17-R5.** A checker applying the modifier-linking diagnostic MUST enforce
  [G14-R6](0014-sentence-structure-word-order.md) in the documented
  two-modifier construction.
- **G17-R6.** A checker applying the negative-question diagnostic MUST enforce
  [G15-R5](0015-negation.md) and MUST distinguish interrogative `ma` from
  negative `ma` before reporting duplication.

## Examples

| Somali | Gloss | English |
|---|---|---|
| Deeradu biyo bay cabtay. | `[G17-R1 +]` gazelle-DEF.F water FOC.3SG.F drank.F | Correct agreement. |
| \*Deeradu biyo buu cabbay. | `[G17-R1 −]` gazelle-DEF.F water FOC.3SG.M drank.M | Diagnose gender agreement, not natural sex. |
| Asli baa keentay. | `[G17-R1 +]` Asli FOC brought.3SG.F | Correct feminine verb agreement. |
| \*Asli baa keenay. | `[G17-R1 −]` Asli FOC brought.3SG.M | Diagnose the masculine verb form in the reviewed feminine-subject construction. |
| Faadumo ayaan la hadlay. = Iyada ayaan la hadlay. | `[G17-R1 +]` Faadumo FOC.1SG with spoke = she FOC.1SG with spoke | Correct independent-pronoun agreement. |
| Faadumo ayaan la hadlay. = Isaga ayaan la hadlay. | `[G17-R1 −]` Faadumo FOC.1SG with spoke = he FOC.1SG with spoke | Diagnose the pronoun mismatch for the intended reference to Faadumo. |
| Nimankii baa yimid. | `[G17-R1 +]` men-DEF FOC came.3SG | Correct reduced agreement with a focused plural subject. |
| Wiilkii moos buu cunay. | `[G17-R2 +]` boy-DEF banana FOC.3SG.M ate | Correct object focus with subject clitic. |
| \*Wiilkii moos baa cunay. | `[G17-R2 −]` boy-DEF banana FOC ate | Diagnose the missing subject clitic. |
| Adigu moos baad cuntay. | `[G17-R2 +]` you.SG banana FOC.2SG ate | Correct second-person subject-clitic agreement. |
| \*Adigu moos baa cuntay. | `[G17-R2 −]` you.SG banana FOC ate | Diagnose bare `baa` where the reviewed construction requires `baad`. |
| Mire baa ku fariistay kursiga. | `[G17-R3 +]` Mire FOC PREP sat chair-DEF | Correct selected preposition. |
| \*Mire baa fariistay kursiga. | `[G17-R3 −]` Mire FOC sat chair-DEF | Diagnose omission only because this lexical construction is reviewed. |
| Cali Axmed wuu dilay. | `[G17-R4 +]` Cali Axmed 3SG.M killed | Correct reviewed verbal group. |
| \*Cali wuu Axmed dilay. | `[G17-R4 −]` Cali 3SG.M Axmed killed | Diagnose interruption of this verbal group. |
| dukaanka dharka ee Cali | `[G17-R5 +]` shop-DEF clothes LINK Cali | Correct modifier linking. |
| \*dukaanka dharka Cali | `[G17-R5 −]` shop-DEF clothes Cali | Diagnose the missing linker in this construction. |
| Ma Axmed baan hilibka cunin? | `[G17-R6 +]` Q Axmed FOC.NEG meat-DEF eat.NEG.PST | Correct reviewed negative question. |
| \*Axmed ma ma imanayo? | `[G17-R6 −]` Axmed Q NEG come.NEG.PROG | Diagnose doubled interrogative and negative *ma*. |

## Edge Cases & Common Mistakes

- An asterisk marks non-conformance only for the interpretation and
  construction identified by the cited rule.
- Variation, an unattested lexical item, or an unreviewed paradigm is not
  automatically an error. A validator should return “not covered” when its
  evidence is insufficient.
- `Nimankii baa yimid` MUST NOT be flagged merely because its focused plural
  subject occurs with reduced singular agreement. Subject relative clauses can
  also neutralize gender or number marking; without a reviewed relative-clause
  profile, an apparent mismatch is `not covered`.
- Error messages should name the violated rule and the relevant feature rather
  than silently rewriting the user's text.

## Related

- [SLS-0003 Grammar Evidence Map](../../docs/standards/SLS-0003-evidence-map.md)
- [SLS-0003: Somali Grammar Standard](0018-somali-grammar-standard.md)
- [Noun Morphology: Gender and Plurals](0011-noun-morphology-gender-plurals.md)
- [Sentence Structure and Word Order](0014-sentence-structure-word-order.md)
- [Negation](0015-negation.md)
- [Question Formation](0016-question-formation.md)
