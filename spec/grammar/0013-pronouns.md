---
id: "0013"
title: Pronouns
status: Draft
since_version: 0.2.0
category: grammar
supersedes: null
---

# Pronouns

## Summary

This specification defines core distinctions among independent pronouns,
subject clitics, object clitics, reflexive and reciprocal `is`, and impersonal
`la`. Pronoun choice must preserve person, number, gender, and inclusive or
exclusive meaning where those distinctions are expressed.

## Rule

- **G13-R1.** An independent third-person singular pronoun MUST agree with the
  reviewed gender of its antecedent in a context that requires that contrast.
- **G13-R2.** First-person plural `annaga` MUST be used for an exclusive reading
  and `innaga` for an inclusive reading when the speaker explicitly
  distinguishes whether the addressee is included.
- **G13-R3.** A subject clitic in a reviewed focus construction MUST agree with
  the subject in person, number, and gender.
- **G13-R4.** A first- or second-person object licensed by the verb or
  preposition MUST be represented by the required object clitic; it MUST NOT
  be omitted as though it had the ordinary zero form available to many
  third-person objects.
- **G13-R5.** `is` MUST be interpreted as reflexive or reciprocal according to
  the clause participants and context; `la` MUST NOT replace it when the
  intended meaning is reflexive or reciprocal.
- **G13-R6.** Impersonal `la` MUST be used only where the human participant is
  left general or unspecified; it MUST NOT replace an explicitly identified
  subject without changing the construction.

## Examples

| Somali | Gloss | English |
|---|---|---|
| Faadumo ayaan la hadlay. = Iyada ayaan la hadlay. | `[G13-R1 +]` Faadumo FOC.1SG with spoke = she FOC.1SG with spoke | Correct feminine-pronoun substitution. |
| Faadumo ayaan la hadlay. = Isaga ayaan la hadlay. | `[G13-R1 −]` Faadumo FOC.1SG with spoke = he FOC.1SG with spoke | Incorrect for the intended reference to Faadumo. |
| annaga — dhegeystaha kuma jiro | `[G13-R2 +]` we.EXCL — addressee excluded | Correct exclusive reading. |
| innaga — dhegeystuhu wuu ku jiraa | `[G13-R2 +]` we.INCL — addressee included | Correct inclusive reading. |
| annaga — akhris mideeya | `[G13-R2 −]` we.EXCL — inclusive reading | Incorrect when the intended meaning explicitly includes the addressee. |
| Adigu moos baad cuntay. | `[G13-R3 +]` you.SG banana FOC.2SG ate | Correct subject-clitic agreement. |
| \*Adigu moos baa cuntay. | `[G13-R3 −]` you.SG banana FOC.3SG ate | Ungrammatical: the second-person subject clitic is missing. |
| Cali baa adiga kuu soo ordayay. | `[G13-R4 +]` Cali FOC you to.2SG toward ran.PROG | Correct second-person object clitic with *u*. |
| \*Cali baa adiga u soo ordayay. | `[G13-R4 −]` Cali FOC you to toward ran.PROG | Ungrammatical in the reviewed construction: the object clitic is omitted. |
| Ninku wuu is dhaawacay. | `[G13-R5 +]` man-DEF 3SG.M REFL injured | The man injured himself. |
| Carruurtii way is arkeen. | `[G13-R5 +]` children-DEF 3PL RECIP saw | The children saw one another. |
| Ninku waa la dhaawacay. | `[G13-R5 −]` man-DEF IMPERS injured | Grammatical with a passive-like reading, but incorrect for the intended reflexive reading. |
| Tuuggii waa la qabtay. | `[G13-R6 +]` thief-DEF DECL IMPERS caught | The thief was caught; the catcher is not identified. |
| Cali baa la qabtay. | `[G13-R6 −]` Cali FOC IMPERS caught | Grammatical with Cali as the person caught, but incorrect if Cali is intended as the explicit catcher. |

## Edge Cases & Common Mistakes

- Pronouns and clitics do not occupy identical syntactic positions and are not
  freely interchangeable.
- Inclusive and exclusive first-person plurals may be neutralized in some
  usage; a tool must diagnose the distinction only when the intended reading
  is known.
- The interpretation of `is` can be ambiguous with plural subjects. Context
  determines reflexive versus reciprocal meaning.

## Related

- [SLS-0003 Grammar Evidence Map](../../docs/standards/SLS-0003-evidence-map.md)
- [Grammar resource: pronouns](../../resources/naxwe/04-sarfaha-magacuyaallada.md)
- [SLS-0003: Somali Grammar Standard](0018-somali-grammar-standard.md)
- [Parts of Speech](0010-parts-of-speech.md)
- [Sentence Structure and Word Order](0014-sentence-structure-word-order.md)
- [Negation](0015-negation.md)
