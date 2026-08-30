# SLS-0003 Reviewer Packet

**For the linguist or native-speaker reviewer.** Everything SLS-0003 asserts
about Somali grammar is on this page: 42 numbered rules across eight topic
specifications, each with the positive and negative example the standard
relies on. You do not need to read the repository, run anything, or use git.

- **Standard:** SLS-0003 Somali Grammar Standard (`Proposed`)
- **Comment venue:** [issue #11](https://github.com/goobolabs/somali-language-standard/issues/11)
- **Review log:** [`SLS-0003-review-log.md`](SLS-0003-review-log.md)
- **How review works:** [`docs/REVIEWERS.md`](../REVIEWERS.md)
- **Evidence behind each rule:** [`SLS-0003-evidence-map.md`](SLS-0003-evidence-map.md)

## What we are asking

For each rule, one of four verdicts:

| Verdict | Meaning |
| --- | --- |
| **OK** | The rule is right and the examples are natural Somali. |
| **Narrow** | Right in the example, but stated too broadly — it would flag correct Somali. |
| **Wrong** | The rule or its example is not correct Somali. |
| **Unsure** | Genuinely variable, dialectal, or you would want a second opinion. |

“Narrow” is the most valuable verdict here. This standard drives spellcheckers
and grammar checkers: a rule stated too broadly turns correct Somali into a
reported error for every user of every tool that adopts it.

You do not have to do all 42. A verdict on the [priority questions](#priority-questions)
alone is a complete, citable review.

**Per-rule progress:** 20 of 42 rules have maintainer native-speaker verdicts:
G10-R1 through G10-R4, G11-R1 through G11-R5, G12-R1 through G12-R5, and
G13-R1 through G13-R6 were approved on 2026-08-30 as MR-13 through MR-32.

## Conventions used below

- `*` marks a form the standard treats as **non-conforming for the stated
  reading only** — never as “always wrong” (SLS-0003 R6).
- Glosses are informative. No Somali rule may be derived from the English
  wording alone (SLS-0003 R8).
- `not covered` is a valid result: the standard deliberately refuses to judge
  what its evidence does not support.

## 0010 — Parts of Speech

| Rule | Requirement | Conforming | Non-conforming (`*` = stated reading only) | Verdict |
| --- | --- | --- | --- | --- |
| **G10-R1** | An analysis MUST use the nine primary classes `magac`, `magacuyaal`, `fal`, `tilmaame`, `falkaab`, `meeleeye`, `xiriiriye`, `qodob`, and `yaab` when one fits. `Tifaftire` MAY name the determiner/quantifier grouping inside the `tilmaame` modifier domain but is not a tenth primary class. | qor — fal | qor — magac | **OK — MR-13** |
| **G10-R2** | A noun (`magac`) MUST be identified by its nominal function and available noun behaviour, such as reference, article marking, number, or participation in a noun phrase. | ninka | ninka — fal | **OK — MR-14** |
| **G10-R3** | An item whose class changes with syntactic use MUST be classified in its attested context. | qalinkan<br>Kani waa buug. | kan — magacuyaal mar kasta | **OK — MR-15** |
| **G10-R4** | Linkers, conjunctions, and prepositions MUST be distinguished by their grammatical relation: a linker connects constituents within a construction, a conjunction coordinates or subordinates units, and a preposition introduces the relation selected by its construction. | dukaanka dharka ee Cali<br>Mire baa ku fariistay kursiga. | ku — xiriiriye | **OK — MR-16** |

## 0011 — Noun Morphology: Gender and Plurals

| Rule | Requirement | Conforming | Non-conforming (`*` = stated reading only) | Verdict |
| --- | --- | --- | --- | --- |
| **G11-R1** | A noun's grammatical gender MUST be determined from reviewed agreement behaviour, not inferred only from the natural sex of its referent or the shape of the isolated noun. | Deeradu biyo bay cabtay. | \*Deeradu biyo buu cabbay. | **OK — MR-17** |
| **G11-R2** | A definite article MUST agree with the reviewed gender and phonological class of its noun. | Ninka ayaan la hadlay.<br>Naagta ayaan la hadlay. | \*Ninta ayaan la hadlay.<br>\*Naagka ayaan la hadlay. | **OK — MR-18** |
| **G11-R3** | A noun's plural form MUST be recorded in a reviewed lexical entry or paradigm for that noun; it MUST NOT be predicted from the singular spelling. | buug → buugag | \*buug → buugyo | **OK — MR-19** |
| **G11-R4** | A mass or collective MUST NOT be forced into a singular-count reading when the intended reading is uncounted or collective; contextual unit ellipsis MAY be accepted, while formal writing SHOULD name the measure. | Sonkor badan baan rabaa.<br>Hal sonkor. — one contextual unit<br>Hal koob oo sonkor ah. | \*Hal sonkor. — intended: one unmeasured quantity of sugar | **OK — MR-20** |
| **G11-R5** | Once a reviewed plural form and ending class are known, agreement MUST follow that class's regular gender polarity; the plural form itself MUST NOT be predicted from the gender switch. | Inankaas baa dheer; Inamadaas baa dhaadheer. | \*Inamakaas baa dhaadheer. | **OK — MR-21** |

## 0012 — Verb System: Tense, Aspect, and Mood

| Rule | Requirement | Conforming | Non-conforming (`*` = stated reading only) | Verdict |
| --- | --- | --- | --- | --- |
| **G12-R1** | A finite verb MUST use the reviewed person, number, and gender agreement required by its construction; reviewed reduced-agreement environments MUST NOT be diagnosed from surface number alone. | Cali baa keenay.<br>Asli baa keentay.<br>Nimankii baa yimid. | \*Asli baa keenay. | **OK — MR-22** |
| **G12-R2** | A tense form MUST be selected from a reviewed conjugation and MUST preserve the temporal interpretation licensed by that form and its context. | keen — tagto: keenay | keen — tagto: keenaa | **OK — MR-23** |
| **G12-R3** | A progressive or habitual construction MUST NOT be substituted silently for a simple completed-event form when the aspectual contrast changes the asserted meaning. | Nuur shalay ayuu cunay mallaay.<br>Nuur shalay buu cunayay mallaay. | cunay → cunayay, isla macne | **OK — MR-24** |
| **G12-R4** | An imperative form MUST use the reviewed second-person paradigm; a non-second-person command meaning MUST use an independently licensed construction. | Qor!<br>Qora!<br>Isagu ha qoro! | \*Isagu qor! | **OK — MR-25** |
| **G12-R5** | A generator or checker MUST NOT infer an unattested stem or ending merely by extending the most common conjugation pattern to an irregular verb. | imow → yimid | imow + -ay → imoway | **OK — MR-26** |

## 0013 — Pronouns

| Rule | Requirement | Conforming | Non-conforming (`*` = stated reading only) | Verdict |
| --- | --- | --- | --- | --- |
| **G13-R1** | An independent third-person singular pronoun MUST agree with the reviewed gender of its antecedent in a context that requires that contrast. | Faadumo ayaan la hadlay. = Iyada ayaan la hadlay. | Faadumo ayaan la hadlay. = Isaga ayaan la hadlay. | **OK — MR-27** |
| **G13-R2** | First-person plural `annaga` MUST be used for an exclusive reading and `innaga` for an inclusive reading when the speaker explicitly distinguishes whether the addressee is included. | Annaga ayaa baxayna.<br>Innaga ayaa baxayna. | \*Annaga ayaa baxayna. — intended: the addressee is included | **OK — MR-28** |
| **G13-R3** | A subject clitic in a reviewed focus construction MUST agree with the subject in person, number, and gender. | Adigu moos baad cuntay. | \*Adigu moos baa cuntay. | **OK — MR-29** |
| **G13-R4** | A first- or second-person object MUST use its required clitic. An overt object noun or independent third-person pronoun MUST be preserved when it supplies plurality or prevents ambiguity; without an explicit object noun, first/second-person form, or independent object pronoun, a recoverable singular third-person object MAY be zero. | Cali baa adiga kuu soo ordayay.<br>Cali baa iyaga u yeeray.<br>Cali baa u yeeray. | \*Cali baa adiga u soo ordayay. | **OK — MR-30** |
| **G13-R5** | `is` MUST be interpreted as reflexive or reciprocal according to the clause participants and context; `la` MUST NOT replace it when the intended meaning is reflexive or reciprocal. | Ninku wuu is dhaawacay.<br>Carruurtii way is arkeen. | Ninku waa la dhaawacay. | **OK — MR-31** |
| **G13-R6** | Impersonal `la` MUST be used only where the human participant is left general or unspecified; it MUST NOT replace an explicitly identified subject without changing the construction. | Tuuggii waa la qabtay. | Cali baa la qabtay. | **OK — MR-32** |

## 0014 — Sentence Structure and Word Order

| Rule | Requirement | Conforming | Non-conforming (`*` = stated reading only) | Verdict |
| --- | --- | --- | --- | --- |
| **G14-R1** | A complete independent construction MUST supply the predicate and required participants licensed by that predicate, except where a reviewed imperative or context-dependent omission licenses a shorter form. | Kaaley!<br>Macallinkaa buug i siiyay. | \*Macallinkaa buug i… |  |
| **G14-R2** | A verb that selects a prepositional relation MUST retain the required preposition and associated clitic material. | Mire baa ku fariistay kursiga. | \*Mire baa fariistay kursiga. |  |
| **G14-R3** | In the reviewed `baa` or `ayaa` focus construction, the focus particle MUST follow its focused constituent and precede the verbal group. | Wiilkii baa yimid. | \*Wiilkii yimid baa. |  |
| **G14-R4** | In the reviewed subject–focused-object–verb order, the verbal group MUST contain the agreeing subject clitic; bare `baa` MUST NOT replace that clitic. | Wiilkii moos buu cunay. | \*Wiilkii moos baa cunay. |  |
| **G14-R5** | In an affirmative declarative `waa` construction, `waa` and its subject-clitic realization MUST occur at the beginning of the verbal group rather than between a verb and its selected complement. | Cali Axmed wuu dilay. | \*Cali wuu Axmed dilay. |  |
| **G14-R6** | When two direct post-nominal modifiers require the linker `ee`, the linker MUST be retained between the modifier units. | dukaanka dharka ee Cali | \*dukaanka dharka Cali |  |

## 0015 — Negation

| Rule | Requirement | Conforming | Non-conforming (`*` = stated reading only) | Verdict |
| --- | --- | --- | --- | --- |
| **G15-R1** | In the reviewed free-negative verbal construction, `ma` MUST precede the verbal group and the verb MUST use its licensed negative form. | Cali hadiyad ma keenin. | \*Cali hadiyad ma keenay. |  |
| **G15-R2** | A negated nominal predicate using `ah` MUST be expressed with the reviewed form `ma aha`; affirmative `waa` MUST NOT be retained as though it alone carried negation. | Muuse macallin ma aha. | \*Muuse waa macallin ma ah. |  |
| **G15-R3** | In the reviewed focused negative construction, the negative focus form in `-aan` MUST agree with the subject and MUST be followed by the licensed negative verb form. | Cali hadiyad buusan keenin. | \*Cali hadiyad baan keenin. |  |
| **G15-R4** | A negative command MUST use prohibitive `ha` with the reviewed negative form; it MUST NOT combine `ha` with an affirmative imperative as a mechanically prefixed command. | Waxba ha keenin. | \*Waxba ha keen. |  |
| **G15-R5** | Interrogative `ma` and negative `ma` MUST NOT be doubled in one construction to express a negative yes-or-no question; the reviewed fused negative interrogative construction MUST be used. | Ma Axmed baan hilibka cunin? | \*Cali hadiyad ma ma keenin? |  |

## 0016 — Question Formation

| Rule | Requirement | Conforming | Non-conforming (`*` = stated reading only) | Verdict |
| --- | --- | --- | --- | --- |
| **G16-R1** | In the reviewed yes-or-no construction with `ma`, the interrogative particle and the focus particle MUST identify the same focused constituent or the licensed verbal focus. | Ma Axmed baa hadiyad keenay?<br>Axmed hadiyad ma keenay? | \*Ma Axmed hadiyad buu keenay? |  |
| **G16-R2** | `miyaa` MUST occupy the position licensed by the focus and predicate type. In a nominal-predicate question, it MAY follow the subject before a predicate with `ah`, or follow the nominal predicate, but MUST NOT begin the clause; clause-initial questioning uses the reviewed `ma ... baa ... ah` construction. | Wiilkii miyaa ka tegaya?<br>Cali miyaa macallin ah?<br>Cali macallin miyaa?<br>Ma Cali baa macallin ah? | \*Miyaa Cali macallin? |  |
| **G16-R3** | A constituent question MUST use a reviewed interrogative form and MUST keep that interrogative constituent adjacent to its associated focus marking where the construction requires adjacency. | Xaggee baa Axmed aaday? | \*Xaggee Axmed baa aaday? |  |
| **G16-R4** | The `-kee`/`-tee` interrogative series MUST select exactly the same gender class and consonant allomorph as the head noun's definite article. | Macallinkee baa yimid?<br>Naagtee baa timid?<br>magaalada → magaaladee<br>meesha → meeshee | \*Naagkee baa timid? |  |
| **G16-R5** | An indirect question with a reviewed `halka` or `goorta` construction MUST be integrated as a subordinate constituent rather than punctuated or ordered as an independent direct question. | Weyddii halkuu tegayo. | Weyddii: “Xaggee buu tegayaa?” |  |

## 0017 — Common Grammar Mistakes

| Rule | Requirement | Conforming | Non-conforming (`*` = stated reading only) | Verdict |
| --- | --- | --- | --- | --- |
| **G17-R1** | An agreement diagnostic MUST enforce G11-R1, G12-R1, and G13-R1 in reviewed constructions; it MUST NOT diagnose an unreviewed reduced-agreement environment. | Deeradu biyo bay cabtay.<br>Asli baa keentay.<br>Faadumo ayaan la hadlay. = Iyada ayaan la hadlay.<br>Nimankii baa yimid. | \*Deeradu biyo buu cabbay.<br>\*Asli baa keenay.<br>Faadumo ayaan la hadlay. = Isaga ayaan la hadlay. |  |
| **G17-R2** | A focus diagnostic MUST enforce G13-R3, G14-R3, and G14-R4, including the required subject clitic and its person, number, and gender features. | Wiilkii moos buu cunay.<br>Adigu moos baad cuntay. | \*Wiilkii moos baa cunay.<br>\*Adigu moos baa cuntay. |  |
| **G17-R3** | A checker applying the preposition diagnostic MUST enforce G14-R2 only for predicates and relations in its reviewed lexical data. | Mire baa ku fariistay kursiga. | \*Mire baa fariistay kursiga. |  |
| **G17-R4** | A checker applying the verbal-group diagnostic MUST enforce G14-R5 without treating every word order not shown there as an error. | Cali Axmed wuu dilay. | \*Cali wuu Axmed dilay. |  |
| **G17-R5** | A checker applying the modifier-linking diagnostic MUST enforce G14-R6 in the documented two-modifier construction. | dukaanka dharka ee Cali | \*dukaanka dharka Cali |  |
| **G17-R6** | A checker applying the negative-question diagnostic MUST enforce G15-R5 and MUST distinguish interrogative `ma` from negative `ma` before reporting duplication. | Ma Axmed baan hilibka cunin? | \*Axmed ma ma imanayo? |  |
## Priority questions

These twelve are where a native-speaker judgment actually changes the
standard. Each names the rule it would move.

**Q1 — Two different `ha` (G12-R4, G15-R4).** *(Answered 2026-08-30 — MR-2 in the [review log](SLS-0003-review-log.md); a second opinion is still welcome.)* G15-R4 requires prohibitive `ha`
with a negative form: `Waxba ha keenin.` is conforming, `*Waxba ha keen.` is
not. G12-R4 separately licenses the third-person directive `Isagu ha qoro!`,
where `ha` is followed by an affirmative form. Is that contrast stable enough
for a checker to apply, and is G15-R4 worded so that it cannot flag a licensed
`ha qoro`-type directive?

**Q2 — Plural gender polarity (G11-R5).** *(Answered 2026-08-30 — MR-5; a
second opinion is still welcome.)* `Inankaas baa dheer; Inamadaas baa
dhaadheer.` Is gender switching in the plural regular enough to enforce as a
rule, or must it be recorded per lexeme in the dictionary instead?

**Q3 — Inclusive vs exclusive `annaga` / `innaga` (G13-R2).** *(Answered 2026-08-30 — MR-1; a second opinion is still welcome.)* The current rows
describe the contrast rather than showing it. Please supply a minimal Somali
sentence pair where the choice changes the meaning — and say whether the
contrast is consistently observed in modern written Somali or often
neutralized.

**Q4 — Bare `baa` in object focus (G14-R4).** *(Answered 2026-08-30 — MR-3; a second opinion is still welcome.)* `Wiilkii moos buu cunay.` is
conforming; `*Wiilkii moos baa cunay.` is not. Is bare `baa` genuinely
ungrammatical here for all speakers, or acceptable in some registers,
dialects, or speech?

**Q5 — Interrupting the verbal group (G14-R5).** *(Answered 2026-08-30 — MR-4; a second opinion is still welcome.)* `Cali Axmed wuu dilay.`
versus `*Cali wuu Axmed dilay.` Is the second always non-conforming, or is
there a topicalized or emphatic reading that licenses it?

**Q6 — `miyaa` position (G16-R2).** *(Answered 2026-08-30 — MR-6; a second
opinion is still welcome.)* `*Miyaa Cali macallin?` is marked non-conforming
for the nominal-predicate reading. Is clause-initial `miyaa` ever acceptable
there?

**Q7 — `-kee` / `-tee` agreement (G16-R4).** *(Answered 2026-08-30 — MR-7; a
second opinion is still welcome.)* Do the interrogative modifiers follow
exactly the gender and phonological classes of the definite article, or are
there noun classes where they diverge?

**Q8 — `*hal sonkor` (G11-R4).** *(Answered 2026-08-30 — MR-8; a second opinion
is still welcome.)* Is this ungrammatical, or acceptable under a unit reading
(“one packet of sugar”)? If the latter, the rule needs the intended reading
stated more tightly.

**Q9 — Zero third-person object (G13-R4).** *(Answered 2026-08-30 — MR-9; a
second opinion is still welcome.)* The standard requires first- and
second-person object clitics but allows a zero third-person object. Is the
boundary between required and zero stated well enough to implement?

**Q10 — The nine word classes (G10-R1).** *(Answered 2026-08-30 — MR-10; a
second opinion is still welcome.)* Are `magac`, `magacuyaal`, `fal`,
`tilmaame`, `falkaab`, `meeleeye`, `xiriiriye`, `qodob`, `yaab` the right
primary inventory for a standard, and does treating `tifaftire` as a grouping
inside the modifier domain match how Somali grammar is taught?

**Q11 — Diagnostic coverage (0017).** *(Answered 2026-08-30 — MR-11; a second
opinion is still welcome.)* The six diagnostics cover noun gender, focus,
prepositions, verbal group, modifier linking, and negative questions. Nothing
covers verb agreement (G12-R1) or pronoun agreement (G13-R1, G13-R3). Is that
caution correct, or should those be diagnosed too?

**Q12 — Dialect reach.** *(Answered 2026-08-30 — MR-12; a second opinion is
still welcome.)* The rules are stated for standard written Somali. Is there
anything here that would misfire on Maay, Benaadir, or other regional writing
in a way the “not covered” escape hatch does not already handle?

## How to answer

Reply in [issue #11](https://github.com/goobolabs/somali-language-standard/issues/11),
one line per item:

```
G14-R4: Narrow — bare baa occurs in speech and in some newspaper prose; enforce only for edited formal text.
Q3: annaga/innaga — pair: "Annagu waan tagaynaa" (you stay) vs "Innagu waan tagaynaa" (you come). Still observed in writing.
```

Somali, English, or both. Partial answers are useful; there is no requirement
to finish the table. Every response is copied into the
[review log](SLS-0003-review-log.md) with a disposition and a written
resolution, under your name or a handle, whichever you prefer.

## What a review commits you to

Nothing beyond the response itself. No standard's progress waits on you, and
there is no role to accept: SLS is maintainer-reviewed, and an outside response
is recorded on its own merits (`SLS-0000` R17). If you would like to be named as
an independent reviewer for Somali grammar, that is welcome and recorded as such
— see [`docs/REVIEWERS.md`](../REVIEWERS.md).
