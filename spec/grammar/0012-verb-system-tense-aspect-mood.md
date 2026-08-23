---
id: "0012"
title: "Verb System: Tense, Aspect, and Mood"
status: Draft
since_version: 0.2.0
category: grammar
supersedes: null
---

# Verb System: Tense, Aspect, and Mood

## Summary

Somali verb forms express agreement and contrasts of tense, aspect, mood,
polarity, and clause type. This specification fixes core conformance principles
while leaving complete lexical paradigms to reviewed morphological data.

## Rule

- **G12-R1.** A finite verb MUST use the reviewed person, number, and gender
  agreement required by its construction.
- **G12-R2.** A tense form MUST be selected from a reviewed conjugation and
  MUST preserve the temporal interpretation licensed by that form and its
  context.
- **G12-R3.** A progressive or habitual construction MUST NOT be substituted
  silently for a simple completed-event form when the aspectual contrast
  changes the asserted meaning.
- **G12-R4.** An imperative form MUST use the reviewed second-person paradigm;
  a non-second-person command meaning MUST use an independently licensed
  construction.
- **G12-R5.** A generator or checker MUST NOT infer an unattested stem or
  ending merely by extending the most common conjugation pattern to an
  irregular verb.

## Examples

| Somali | Gloss | English |
|---|---|---|
| Cali baa keenay. | `[G12-R1 +]` Cali FOC brought.3SG.M | Correct masculine singular agreement. |
| Asli baa keentay. | `[G12-R1 +]` Asli FOC brought.3SG.F | Correct feminine singular agreement. |
| \*Asli baa keenay. | `[G12-R1 −]` Asli FOC brought.3SG.M | Ungrammatical in the reviewed feminine-agreement reading. |
| keen — tagto: keenay | `[G12-R2 +]` bring — past: brought | Correct tense form from the reviewed paradigm. |
| keen — tagto: keenaa | `[G12-R2 −]` bring — past: bring.HAB | Incorrect: the habitual form is labelled as the simple past. |
| Nuur shalay ayuu cunay mallaay. | `[G12-R3 +]` Nuur yesterday FOC.3SG.M ate fish | Correct simple-past presentation. |
| Nuur shalay buu cunayay mallaay. | `[G12-R3 +]` Nuur yesterday FOC.3SG.M eat.PST.PROG fish | Correct past-progressive presentation. |
| cunay → cunayay, isla macne | `[G12-R3 −]` simple-past → past-progressive, same meaning | Incorrect: a tool silently treats the reviewed aspect forms as equivalent. |
| Qor! | `[G12-R4 +]` write.IMP.2SG | Correct singular command. |
| Qora! | `[G12-R4 +]` write.IMP.2PL | Correct plural command. |
| Isagu ha qoro! | `[G12-R4 +]` he OPT write.3SG.M | Correct licensed third-person directive construction. |
| \*Isagu qor! | `[G12-R4 −]` he write.IMP.2SG | Incorrect as a third-person imperative analysis. |
| imow → yimid | `[G12-R5 +]` come.IMP → came.3SG.M | Correct reviewed irregular pairing. |
| imow + -ay → imoway | `[G12-R5 −]` come.STEM + past → generated form | Non-conforming automatic regularization; it is not licensed by the reviewed paradigm. |

## Edge Cases & Common Mistakes

- Tense and aspect readings can depend on particles, clause type, and discourse
  context in addition to the verb ending.
- Negative paradigms are specified with negation in
  [Negation](0015-negation.md); they must not be reconstructed from affirmative
  forms without evidence.
- A form absent from this summary is not automatically wrong. It is outside
  automatic conformance until its paradigm is reviewed.

## Related

- [SLS-0003 Grammar Evidence Map](../../docs/standards/SLS-0003-evidence-map.md)
- [Grammar resource: verb morphology](../../resources/naxwe/07-sarfaha-falalka.md)
- [Reviewed verb paradigms](../../resources/sarfe/02-falalka.md)
- [SLS-0003: Somali Grammar Standard](0018-somali-grammar-standard.md)
- [Pronouns](0013-pronouns.md)
- [Negation](0015-negation.md)
- [Question Formation](0016-question-formation.md)
