---
id: "0014"
title: Sentence Structure and Word Order
status: Draft
since_version: 0.2.0
category: grammar
supersedes: null
---

# Sentence Structure and Word Order

## Summary

Somali clause structure is organized by predicate requirements, focus
constructions, subject clitics, and the verbal group. This specification
defines a narrow core supported by reviewed grammar resources; it does not
reduce all Somali clauses to one fixed surface order.

## Rule

- **G14-R1.** A complete independent construction MUST supply the predicate and
  required participants licensed by that predicate, except where a reviewed
  imperative or context-dependent omission licenses a shorter form.
- **G14-R2.** A verb that selects a prepositional relation MUST retain the
  required preposition and associated clitic material.
- **G14-R3.** In the reviewed `baa` or `ayaa` focus construction, the focus
  particle MUST follow its focused constituent and precede the verbal group.
- **G14-R4.** In the reviewed subject–focused-object–verb order, the verbal
  group MUST contain the agreeing subject clitic; bare `baa` MUST NOT replace
  that clitic. Other constituent orders require their separately documented
  rule.
- **G14-R5.** In an affirmative declarative `waa` construction, `waa` and its
  subject-clitic realization MUST occur at the beginning of the verbal group
  rather than between a verb and its selected complement.
- **G14-R6.** When two direct post-nominal modifiers require the linker `ee`,
  the linker MUST be retained between the modifier units.

## Examples

| Somali | Gloss | English |
|---|---|---|
| Kaaley! | `[G14-R1 +]` come.IMP.2SG | Correct complete imperative. |
| Macallinkaa buug i siiyay. | `[G14-R1 +]` teacher-DEF.FOC book me gave | Correct: the predicate completes the construction. |
| \*Macallinkaa buug i… | `[G14-R1 −]` teacher-DEF.FOC book me… | Incomplete: the required predicate has not been supplied. |
| Mire baa ku fariistay kursiga. | `[G14-R2 +]` Mire FOC PREP sat chair-DEF | Correct selected preposition. |
| \*Mire baa fariistay kursiga. | `[G14-R2 −]` Mire FOC sat chair-DEF | Ungrammatical in the reviewed construction: *ku* is omitted. |
| Wiilkii baa yimid. | `[G14-R3 +]` boy-DEF FOC came | Correct subject focus. |
| \*Wiilkii yimid baa. | `[G14-R3 −]` boy-DEF came FOC | Ungrammatical placement of the focus particle. |
| Wiilkii moos buu cunay. | `[G14-R4 +]` boy-DEF banana FOC.3SG.M ate | Correct object focus with a subject clitic. |
| \*Wiilkii moos baa cunay. | `[G14-R4 −]` boy-DEF banana FOC ate | Ungrammatical in this object-focus construction. |
| Cali Axmed wuu dilay. | `[G14-R5 +]` Cali Axmed 3SG.M killed | Cali killed Axmed. |
| \*Cali wuu Axmed dilay. | `[G14-R5 −]` Cali 3SG.M Axmed killed | Ungrammatical interruption of the reviewed verb group. |
| dukaanka dharka ee Cali | `[G14-R6 +]` shop-DEF clothes LINK Cali | Cali's clothing shop. |
| \*dukaanka dharka Cali | `[G14-R6 −]` shop-DEF clothes Cali | Incorrect here: the required linker is omitted. |

## Edge Cases & Common Mistakes

- Topic, focus, and information structure permit more than one surface order;
  this document standardizes only the displayed constructions.
- `baa`, `ayaa`, and their clitic realizations require construction-specific
  analysis. A checker must not replace one mechanically without identifying
  the focused constituent.
- The linker `ee` has uses beyond the noun-phrase pattern shown here; those
  uses require their own reviewed rules.

## Related

- [SLS-0003 Grammar Evidence Map](../../docs/standards/SLS-0003-evidence-map.md)
- [Grammar resource: simple clauses](../../resources/naxwe/09-weer-fudud.md)
- [Grammar resource: noun phrases](../../resources/naxwe/10-dhismaha-oraah-magaceedyada.md)
- [SLS-0003: Somali Grammar Standard](0018-somali-grammar-standard.md)
- [Pronouns](0013-pronouns.md)
- [Negation](0015-negation.md)
- [Question Formation](0016-question-formation.md)
