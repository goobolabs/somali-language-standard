---
id: "0015"
title: Negation
status: Draft
since_version: 0.2.0
category: grammar
supersedes: null
---

# Negation

> **Lifecycle:** this topic specification keeps its local spec-note status
> `Draft`. It is implemented by **SLS-0003 Somali Grammar Standard**, which is
> `Proposed` and inside its public comment period. Comments on this document
> are recorded in the
> [SLS-0003 review log](../../docs/standards/SLS-0003-review-log.md).

## Summary

Somali negation is expressed through construction-specific negative particles
and verb forms. This specification distinguishes free `ma`, focused negative
forms in `-aan`, nominal-predicate `ma aha`, and prohibitive `ha`.

## Rule

- **G15-R1.** In the reviewed free-negative verbal construction, `ma` MUST
  precede the verbal group and the verb MUST use its licensed negative form.
- **G15-R2.** A negated nominal predicate using `ah` MUST be expressed with the
  reviewed form `ma aha`; affirmative `waa` MUST NOT be retained as though it
  alone carried negation.
- **G15-R3.** In the reviewed focused negative construction, the negative
  focus form in `-aan` MUST agree with the subject and MUST be followed by the
  licensed negative verb form.
- **G15-R4.** A negative command MUST use prohibitive `ha` with the reviewed
  negative form; it MUST NOT combine `ha` with an affirmative imperative as a
  mechanically prefixed command.
- **G15-R5.** Interrogative `ma` and negative `ma` MUST NOT be doubled in one
  construction to express a negative yes-or-no question; the reviewed fused
  negative interrogative construction MUST be used.

## Examples

| Somali | Gloss | English |
|---|---|---|
| Cali hadiyad ma keenin. | `[G15-R1 +]` Cali gift NEG bring.NEG.PST | Cali did not bring a gift. |
| \*Cali hadiyad ma keenay. | `[G15-R1 −]` Cali gift NEG brought.AFF | Ungrammatical: the affirmative verb form follows negative *ma*. |
| Muuse macallin ma aha. | `[G15-R2 +]` Muuse teacher NEG be.NEG | Muuse is not a teacher. |
| \*Muuse waa macallin ma ah. | `[G15-R2 −]` Muuse DECL teacher NEG be | Ungrammatical mixture of the affirmative and negative predicate constructions. |
| Cali hadiyad buusan keenin. | `[G15-R3 +]` Cali gift FOC.3SG.M.NEG bring.NEG.PST | Cali did not bring a gift. |
| \*Cali hadiyad baan keenin. | `[G15-R3 −]` Cali gift FOC.1SG.NEG bring.NEG.PST | Ungrammatical subject agreement. |
| Waxba ha keenin. | `[G15-R4 +]` anything PROH bring.NEG | Do not bring anything. |
| \*Waxba ha keen. | `[G15-R4 −]` anything PROH bring.IMP | Ungrammatical: prohibitive *ha* is followed by an affirmative imperative. |
| Ma Axmed baan hilibka cunin? | `[G15-R5 +]` Q Axmed FOC.NEG meat-DEF eat.NEG.PST | Was it not Axmed who ate the meat? |
| \*Cali hadiyad ma ma keenin? | `[G15-R5 −]` Cali gift Q NEG bring.NEG.PST | Ungrammatical doubling of interrogative and negative *ma*. |

## Edge Cases & Common Mistakes

- Negative forms vary with tense, mood, person, and clause type; this
  specification does not license generating a full negative paradigm by
  suffix substitution.
- A written `ma` must be interpreted from its construction before it is
  labelled interrogative or negative.
- Prohibitive `ha` is not the only `ha`, and the two are distinguished by the
  person and form of the verb that follows.
  [Verb System](0012-verb-system-tense-aspect-mood.md) G12-R4 licenses the
  third-person directive `Isagu ha qoro!` — `ha` plus an affirmative
  third-person form, an instruction about someone else. G15-R4 governs the
  second-person negative command only: `ha` plus the licensed negative form, as
  in `Waxba ha keenin`. A checker that treats every `ha` plus affirmative form
  as a G15-R4 violation will report the licensed directive as an error
  (maintainer review, 2026-08-30).
- Negative existential and possession constructions require dedicated
  evidence and are outside this core draft unless covered by a cited rule.

## Related

- [SLS-0003 Grammar Evidence Map](../../docs/standards/SLS-0003-evidence-map.md)
- [Grammar resource: sentence types and negation](../../resources/naxwe/12-noocyada-weeraha.md)
- [SLS-0003: Somali Grammar Standard](0018-somali-grammar-standard.md)
- [Verb System: Tense, Aspect, and Mood](0012-verb-system-tense-aspect-mood.md)
- [Sentence Structure and Word Order](0014-sentence-structure-word-order.md)
- [Question Formation](0016-question-formation.md)
