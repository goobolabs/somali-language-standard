---
id: "0011"
title: "Noun Morphology: Gender and Plurals"
status: Draft
since_version: 0.2.0
category: grammar
supersedes: null
---

# Noun Morphology: Gender and Plurals

> **Lifecycle:** this topic specification keeps its local spec-note status
> `Draft`. It is implemented by **SLS-0003 Somali Grammar Standard**, which is
> `Proposed` and inside its public comment period. Comments on this document
> are recorded in the
> [SLS-0003 review log](../../docs/standards/SLS-0003-review-log.md).

## Summary

Somali nouns participate in grammatical gender, definiteness, and number
contrasts. This specification standardizes the agreement tests and reviewed
patterns; it does not claim that one ending predicts every noun form.

## Rule

- **G11-R1.** A noun's grammatical gender MUST be determined from reviewed
  agreement behaviour, not inferred only from the natural sex of its referent
  or the shape of the isolated noun.
- **G11-R2.** A definite article MUST agree with the reviewed gender and
  phonological class of its noun. A masculine article MUST NOT replace a
  required feminine article, or conversely.
- **G11-R3.** A noun's plural form MUST be recorded in a reviewed lexical entry
  or paradigm for that noun. Implementations MUST NOT predict the plural form
  from the singular spelling or treat one plural suffix as universally
  productive.
- **G11-R4.** Mass and collective nouns MUST NOT be forced into ordinary
  singular-count interpretation when the intended reading is uncounted or
  collective. A contextual unit reading MAY license an omitted measure word;
  formal writing SHOULD state the measure or unit explicitly.
- **G11-R5.** Once a noun's reviewed plural form and ending class are known,
  agreement MUST follow the regular gender polarity of that plural class, even
  when the plural gender differs from the singular noun's gender. The plural
  form itself MUST NOT be predicted from the gender switch.

## Examples

| Somali | Gloss | English |
|---|---|---|
| Deeradu biyo bay cabtay. | `[G11-R1 +]` gazelle-DEF.F water FOC.3SG.F drank.F | Correct: agreement diagnoses the noun as feminine here. |
| \*Deeradu biyo buu cabbay. | `[G11-R1 −]` gazelle-DEF.F water FOC.3SG.M drank.M | Ungrammatical in the reviewed agreement pattern. |
| Ninka ayaan la hadlay. | `[G11-R2 +]` man-DEF.M FOC.1SG with spoke | Correct masculine article. |
| \*Ninta ayaan la hadlay. | `[G11-R2 −]` man-DEF.F FOC.1SG with spoke | Ungrammatical: the feminine article replaces the reviewed masculine form. |
| Naagta ayaan la hadlay. | `[G11-R2 +]` woman-DEF.F FOC.1SG with spoke | Correct feminine article. |
| \*Naagka ayaan la hadlay. | `[G11-R2 −]` woman-DEF.M FOC.1SG with spoke | Ungrammatical: the masculine article replaces the reviewed feminine form. |
| buug → buugag | `[G11-R3 +]` book → books | Correct reviewed plural. |
| \*buug → buugyo | `[G11-R3 −]` book → generated-plural | Not accepted here: the form is produced by an unsupported universal suffix rule. |
| Sonkor badan baan rabaa. | `[G11-R4 +]` sugar much FOC.1SG want | Correct mass-noun reading. |
| Hal sonkor. | `[G11-R4 +]` one sugar-unit | Common unit ellipsis: context supplies one bag, cup, packet, or other unit. |
| Hal koob oo sonkor ah. | `[G11-R4 +]` one cup of sugar COP | Preferred formal wording for one cup of sugar. |
| \*Hal sonkor. — intended: one unmeasured quantity of sugar | `[G11-R4 −]` one sugar | Incorrect only for direct counting of the substance without a recoverable unit. |
| Inankaas baa dheer; Inamadaas baa dhaadheer. | `[G11-R5 +]` boy-DEM.M FOC tall; boys-DEM.F FOC tall.PL | Correct: the demonstrative and modifier follow the reviewed plural pattern. |
| \*Inamakaas baa dhaadheer. | `[G11-R5 −]` boys-DEM.M FOC tall.PL | Ungrammatical here: the masculine demonstrative form is imposed on the reviewed feminine plural. |

## Edge Cases & Common Mistakes

- Plural formation is lexical: a dictionary must record the plural form for
  each noun rather than generate it from the singular. Once that form and its
  ending class are known, plural-gender polarity is regular and enforceable.
- Article allomorphy remains phonological as well as grammatical; a single
  isolated spelling is not a complete classifier.
- A collective form and an ordinary count plural may differ in meaning even
  when both refer to more than one entity.
- `Hal sonkor` is wrong for a literal mass reading but common when context
  supplies one unit. Formal prose should name the measure, as in
  `hal koob oo sonkor ah`.
- Unlisted or disputed plurals require lexical review before a validator treats
  them as errors.

## Related

- [SLS-0003 Grammar Evidence Map](../../docs/standards/SLS-0003-evidence-map.md)
- [Grammar resource: noun morphology](../../resources/naxwe/02-sarfaha-magacyada.md)
- [Reviewed noun paradigms](../../resources/sarfe/01-magacyada.md)
- [SLS-0003: Somali Grammar Standard](0018-somali-grammar-standard.md)
- [Parts of Speech](0010-parts-of-speech.md)
- [Pronouns](0013-pronouns.md)
- [Sentence Structure and Word Order](0014-sentence-structure-word-order.md)
