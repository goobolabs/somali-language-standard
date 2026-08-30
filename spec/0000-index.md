# Specification Index

This is the index of SLS normative specification documents. Every document in
`spec/` is numbered within a reserved category block (see
[`docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md) §5) and carries a lifecycle
`status` in its front-matter. Documents that also hold a global `SLS-XXXX`
identifier are tracked in the standards catalog
([`standards/REGISTRY.md`](../standards/REGISTRY.md)).

Numbering blocks are append-only: a number, once used, is never reused or
renumbered.

A standard at `Proposed` has an open public-comment period with a tracking
issue and a review log; both are listed in
[`docs/standards/MILESTONE-2-READINESS.md`](../docs/standards/MILESTONE-2-READINESS.md).

## Category blocks

| Block | Category | Directory |
|---|---|---|
| `00xx` | Orthography (alphabet, spelling, capitalization, punctuation) | `spec/orthography/` |
| `01xx` | Grammar (parts of speech, morphology, syntax, …) | `spec/grammar/` |
| `03xx` | Style (registers: formal, academic, journalism, …) | `spec/style/` |
| `05xx` | Translation (EN↔SO guidelines, idioms, false friends, …) | `spec/translation/` |

## Documents

### Orthography (`00xx`)

| ID | Title | Standard | Status | File |
|---|---|---|---|---|
| 0001 | Somali Alphabet | SLS-0001 | Proposed | [`orthography/0001-alphabet.md`](orthography/0001-alphabet.md) |
| 0002 | Somali Orthography | SLS-0002 | Proposed | [`orthography/0002-spelling-rules.md`](orthography/0002-spelling-rules.md) |
| 0003 | Somali Capitalization | SLS-0005 | Proposed | [`orthography/0003-capitalization.md`](orthography/0003-capitalization.md) |
| 0004 | Somali Punctuation | SLS-0004 | Proposed | [`orthography/0004-punctuation.md`](orthography/0004-punctuation.md) |

### Grammar (`01xx`)

Topic files 0010–0017 retain their ordinary local `Draft` status while their
global numbered standard lifecycle is carried by the SLS-0003 wrapper (0018).

| ID | Title | Standard | Status | File |
|---|---|---|---|---|
| 0010 | Parts of Speech | SLS-0003 | Draft | [`grammar/0010-parts-of-speech.md`](grammar/0010-parts-of-speech.md) |
| 0011 | Noun Morphology: Gender and Plurals | SLS-0003 | Draft | [`grammar/0011-noun-morphology-gender-plurals.md`](grammar/0011-noun-morphology-gender-plurals.md) |
| 0012 | Verb System: Tense, Aspect, and Mood | SLS-0003 | Draft | [`grammar/0012-verb-system-tense-aspect-mood.md`](grammar/0012-verb-system-tense-aspect-mood.md) |
| 0013 | Pronouns | SLS-0003 | Draft | [`grammar/0013-pronouns.md`](grammar/0013-pronouns.md) |
| 0014 | Sentence Structure and Word Order | SLS-0003 | Draft | [`grammar/0014-sentence-structure-word-order.md`](grammar/0014-sentence-structure-word-order.md) |
| 0015 | Negation | SLS-0003 | Draft | [`grammar/0015-negation.md`](grammar/0015-negation.md) |
| 0016 | Question Formation | SLS-0003 | Draft | [`grammar/0016-question-formation.md`](grammar/0016-question-formation.md) |
| 0017 | Common Grammar Mistakes | SLS-0003 | Draft | [`grammar/0017-common-mistakes.md`](grammar/0017-common-mistakes.md) |
| 0018 | Somali Grammar Standard | SLS-0003 | Proposed | [`grammar/0018-somali-grammar-standard.md`](grammar/0018-somali-grammar-standard.md) |

### Style (`03xx`)

*Planned — see [`IMPLEMENTATION_PLAN.md`](../IMPLEMENTATION_PLAN.md).*

### Translation (`05xx`)

*Planned — see [`IMPLEMENTATION_PLAN.md`](../IMPLEMENTATION_PLAN.md) Phase 7.*

---

Reserved-but-unopened blocks (phonology `07xx`, dialects `08xx`, …) are described
in [`docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md) §21 and open only when their
workstream begins.
