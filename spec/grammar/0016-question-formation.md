---
id: "0016"
title: Question Formation
status: Draft
since_version: 0.2.0
category: grammar
supersedes: null
---

# Question Formation

> **Lifecycle:** this topic specification keeps its local spec-note status
> `Draft`. It is implemented by **SLS-0003 Somali Grammar Standard**, which is
> `Proposed` and inside its public comment period. Comments on this document
> are recorded in the
> [SLS-0003 review log](../../docs/standards/SLS-0003-review-log.md).

## Summary

Somali distinguishes yes-or-no questions from questions that request a
constituent as their answer. Question particles interact with focus placement,
subject clitics, and interrogative words; they are not freely inserted into an
affirmative clause.

## Rule

- **G16-R1.** In the reviewed yes-or-no construction with `ma`, the
  interrogative particle and the focus particle MUST identify the same focused
  constituent or the licensed verbal focus. They MUST NOT be split across two
  independently focused constituents.
- **G16-R2.** `miyaa` and its agreement forms MUST occupy the position licensed
  by the focused constituent and predicate type; they MUST NOT be moved
  mechanically to clause-initial position.
- **G16-R3.** A constituent question MUST use a reviewed interrogative form and
  MUST keep that interrogative constituent adjacent to its associated focus
  marking where the construction requires adjacency.
- **G16-R4.** The interrogative modifiers `-kee` and `-tee` MUST follow the
  reviewed gender and phonological class of their head noun.
- **G16-R5.** An indirect question with a reviewed `halka` or `goorta`
  construction MUST be integrated as a subordinate constituent rather than
  punctuated or ordered as an independent direct question.

## Examples

| Somali | Gloss | English |
|---|---|---|
| Ma Axmed baa hadiyad keenay? | `[G16-R1 +]` Q Axmed FOC gift brought | Was it Axmed who brought a gift? |
| Axmed hadiyad ma keenay? | `[G16-R1 +]` Axmed gift Q brought | Did Axmed bring a gift? |
| \*Ma Axmed hadiyad buu keenay? | `[G16-R1 −]` Q Axmed gift FOC.3SG.M brought | Ungrammatical split focus in the reviewed construction. |
| Wiilkii miyaa ka tegaya? | `[G16-R2 +]` boy-DEF Q.FOC from leaving | Is the boy leaving? |
| Cali macallin miyaa? | `[G16-R2 +]` Cali teacher Q.FOC | Is Cali a teacher? |
| \*Miyaa Cali macallin? | `[G16-R2 −]` Q.FOC Cali teacher | Ungrammatical for the reviewed nominal-predicate reading. |
| Xaggee baa Axmed aaday? | `[G16-R3 +]` where FOC Axmed went | Where did Axmed go? |
| \*Xaggee Axmed baa aaday? | `[G16-R3 −]` where Axmed FOC went | Ungrammatical separation of the interrogative and its focus particle. |
| Macallinkee baa yimid? | `[G16-R4 +]` teacher-which.M FOC came | Which teacher came? |
| Naagtee baa timid? | `[G16-R4 +]` woman-which.F FOC came.F | Which woman came? |
| \*Naagkee baa timid? | `[G16-R4 −]` woman-which.M FOC came.F | Ungrammatical gender form in the reviewed pattern. |
| Weyddii halkuu tegayo. | `[G16-R5 +]` ask where.3SG.M go.PROG | Ask where he is going. |
| Weyddii: “Xaggee buu tegayaa?” | `[G16-R5 −]` ask: where FOC.3SG.M go.PROG Q | A direct quotation, not the reviewed integrated indirect-question construction. |

## Edge Cases & Common Mistakes

- `ma` can be interrogative or negative. Its construction must be identified
  before a checker assigns polarity.
- `miyaa` may appear initially in a licensed verbal-focus question; G16-R2
  prohibits only unlicensed mechanical movement.
- The resources document additional interrogatives such as `yaa`, `maxaa`,
  `immisa`, and `goormaa`. A tool must validate only the forms and
  constructions represented in its reviewed data.

## Related

- [SLS-0003 Grammar Evidence Map](../../docs/standards/SLS-0003-evidence-map.md)
- [Grammar resource: sentence types and questions](../../resources/naxwe/12-noocyada-weeraha.md)
- [Grammar resource: interrogative modifiers](../../resources/naxwe/03-sarfaha-tifaftireyaasha.md)
- [SLS-0003: Somali Grammar Standard](0018-somali-grammar-standard.md)
- [Sentence Structure and Word Order](0014-sentence-structure-word-order.md)
- [Negation](0015-negation.md)
- [Common Grammar Mistakes](0017-common-mistakes.md)
