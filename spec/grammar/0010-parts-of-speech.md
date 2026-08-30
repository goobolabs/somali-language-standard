---
id: "0010"
title: Parts of Speech
status: Draft
since_version: 0.2.0
category: grammar
supersedes: null
---

# Parts of Speech

> **Lifecycle:** this topic specification keeps its local spec-note status
> `Draft`. It is implemented by **SLS-0003 Somali Grammar Standard**, which is
> `Proposed` and inside its public comment period. Comments on this document
> are recorded in the
> [SLS-0003 review log](../../docs/standards/SLS-0003-review-log.md).

## Summary

This specification defines the working word-class inventory used by the Somali
Language Standard (SLS). Classification depends on a word's form, meaning, and
grammatical function in context; spelling alone is not sufficient.

## Rule

- **G10-R1.** An analysis conforming to this specification MUST use the nine
  primary classes `magac`, `magacuyaal`, `fal`, `tilmaame`, `falkaab`,
  `meeleeye`, `xiriiriye`, `qodob`, and `yaab` when one of those classes fits
  the item being described. `Tifaftire` MAY name the maintained determiner
  grouping within the modifier domain, but it does not replace `tilmaame` in
  this nine-class inventory.
- **G10-R2.** A noun (`magac`) MUST be identified by its nominal function and
  available noun behaviour, such as reference, article marking, number, or
  participation in a noun phrase. Not every noun is required to display every
  test.
- **G10-R3.** An item whose class changes with syntactic use MUST be classified
  in its attested context. A demonstrative accompanying a noun and an
  independent demonstrative pronoun MUST NOT be assigned one class merely
  because their forms are related.
- **G10-R4.** Linkers, conjunctions, and prepositions MUST be distinguished by
  their grammatical relation: a linker connects constituents within a
  construction, a conjunction coordinates or subordinates units, and a
  preposition introduces the relation selected by its construction.

## Examples

| Somali | Gloss | English |
|---|---|---|
| qor — fal | `[G10-R1 +]` qor = verb | Correct: *qor* is classified as a verb here. |
| qor — magac | `[G10-R1 −]` qor = noun | Incorrect for the imperative use: the item is not a noun here. |
| ninka | `[G10-R2 +]` man-DEF.M | Correct: an article-marked noun. |
| ninka — fal | `[G10-R2 −]` man-DEF.M = verb | Incorrect: the nominal form is classified as a verb. |
| qalinkan | `[G10-R3 +]` pen-DEM | Correct: *kan* accompanies the noun as a demonstrative modifier. |
| Kani waa buug. | `[G10-R3 +]` this.M COP book | Correct: *kani* stands independently as a pronoun. |
| kan — magacuyaal mar kasta | `[G10-R3 −]` this = pronoun always | Incorrect: the context has been ignored. |
| dukaanka dharka ee Cali | `[G10-R4 +]` shop-DEF clothes LINK Cali | Correct: *ee* links the modifiers in the noun phrase. |
| Mire baa ku fariistay kursiga. | `[G10-R4 +]` Mire FOC PREP sat chair-DEF | Correct: *ku* marks the selected relation. |
| ku — xiriiriye | `[G10-R4 −]` PREP = conjunction | Incorrect here: *ku* is not coordinating or subordinating units. |

## Edge Cases & Common Mistakes

- A dictionary headword does not by itself establish every syntactic use of that form.
- Subclasses may be defined in a topic specification, but they do not replace
  the nine primary labels in general inventories.
- The examples classify the displayed use, not every possible use of the same written form.

## Related

- [SLS-0003 Grammar Evidence Map](../../docs/standards/SLS-0003-evidence-map.md)
- [Grammar resource: parts of speech](../../resources/naxwe/13-aasaaska-naxwaha.md)
- [SLS-0003: Somali Grammar Standard](0018-somali-grammar-standard.md)
- [Noun Morphology: Gender and Plurals](0011-noun-morphology-gender-plurals.md)
- [Pronouns](0013-pronouns.md)
- [Sentence Structure and Word Order](0014-sentence-structure-word-order.md)
