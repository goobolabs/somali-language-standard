---
id: "0004"
sls_id: SLS-0004
title: Somali Punctuation Standard
version: 0.1.1
standard_version: 0.1.1
status: Proposed
category: foundation
owner: maintainers
reviewers: []
dependencies:
  - SLS-0002
implements:
  - spec/orthography/0004-punctuation.md
publication_date: null
supersedes: null
superseded_by: null
revision_history:
  - version: 0.1.0
    date: 2026-08-23
    change: Initial evidence-mapped draft
  - version: 0.1.1
    date: 2026-08-30
    change: "Editorial: repaired the long-dash edge-case sentence; linked the public review log"
---

> **Proposed status:** This document is accepted for formal public comment.
> Its scope is frozen for the comment period, but its requirements remain
> non-stable and may be refined through recorded review decisions.

## Abstract

This standard defines the supported functions of punctuation in ordinary
written Somali. It covers sentence-final marks, syntax-sensitive comma use,
colons, paired quotation marks, parentheses, the distinction between
punctuation and the glottal-stop letter, and the rendering boundary between a
hyphen and a long dash. It also defines an ordinary digital character and
spacing profile while keeping unsupported semicolon placement and additional
marks outside its compliance requirements.

## Purpose

Punctuation helps readers recover sentence structure and intended meaning, but
the SLS evidence library contains historical conventions, inconsistent glyphs,
and retained OCR variation. A rule based only on a spoken pause, a corpus
count, or conventions imported from another language would convert that
variation into unsupported policy. SLS-0004 provides a reviewable core while
keeping questions that need linguistic or technical evidence visible.

## Scope

**In scope:** punctuation for ordinary Somali running text; full stops,
question marks, exclamation marks, commas, colons, quotations, parentheses;
the distinction between punctuation and the glottal-stop letter; and the
rendering boundary among hyphen, long dash, and compound spelling.

**Recognized but not yet governed by complete rules:** semicolon, long-dash
placement beyond interruption, ellipsis, square and curly brackets, slash,
combined marks, dialogue turns, and punctuation in numeric or date formats.

**Out of scope:**

- alphabet identity and glottal-stop encoding, governed by SLS-0001;
- the lexical decision to write a compound closed, spaced, or hyphenated,
  governed by SLS-0002;
- grammatical analysis independent of punctuation, governed by SLS-0003;
- capitalization, governed by SLS-0005;
- document-house style, citation style, and typography beyond punctuation
  meaning, reserved for the SLS-0400 block;
- phonetic and prosodic transcription, reserved for the SLS-0600 block;
- machine formats for dates, times, decimals, and grouped numbers, reserved
  for their future data or style standards.

## Definitions

- **Punctuation mark** — a non-letter sign used to delimit or relate written
  material.
- **Glottal-stop letter** — the Somali consonant represented canonically by
  U+02BC MODIFIER LETTER APOSTROPHE (`ʼ`) under SLS-0001. It is not
  punctuation.
- **Sentence-final mark** — a full stop, question mark, or exclamation mark
  used to close an ordinary complete written utterance.
- **Core sentence element** — a subject, predicate, object, or another element
  whose direct grammatical connection must not be broken solely because a
  speaker might pause.
- **Inserted material** — supplementary material placed within a containing
  sentence and set off without changing the containing sentence's core
  relationship.
- **Direct quotation** — words presented as the words of the quoted speaker or
  source rather than as an indirect report.
- **Quotation pair** — visible opening and closing delimiters used for a
  quotation. This draft defines their function but does not select one
  canonical straight, curly, or guillemet profile.
- **Hyphen** — the short joining mark used inside a reviewed hyphenated form.
- **Long dash** — a sentence-level interruption or break mark, distinct in
  function from a compound hyphen.

## Normative Requirements

### Dependencies and character identity

- **R1.** Text conforming to SLS-0004 **MUST** satisfy SLS-0002 and its
  SLS-0001 dependency.
- **R2.** A punctuation process **MUST NOT** treat an in-word canonical U+02BC
  glottal-stop letter as a quotation mark or delete it as punctuation. Input
  aliases governed by SLS-0001 **MUST** be resolved before a destructive
  punctuation transformation is applied.

### Sentence-final marks

- **R3.** An ordinary complete statement that is neither a direct question nor
  an exclamation **MUST** end with a full stop when written as running prose.
  Headings, labels, list fragments, poetry lines, and structured data are not
  ordinary running prose under this requirement.
- **R4.** An independent direct question **MUST** end with a question mark.
- **R5.** A word or longer utterance presented as an exclamation **MUST** end
  with an exclamation mark. A writer or tool **MUST NOT** infer an exclamation
  mark merely from an interjection appearing inside a larger non-exclamatory
  sentence.

### Commas and colons

- **R6.** In a simple inline series of three or more parallel elements, commas
  **MUST** separate successive non-final elements. This draft neither requires
  nor prohibits a comma immediately before the final conjunction; that choice
  **MUST NOT** change the intended grouping.
- **R7.** Introductory or genuinely inserted material **MAY** be delimited by
  commas when the commas clarify its boundary. When a writer uses two commas
  to enclose inserted material, both delimiters **MUST** be present.
- **R8.** A comma **MUST NOT** separate tightly connected core sentence
  elements solely to represent a spoken pause. In particular, an ordinary
  subject and its predicate **MUST NOT** be separated without an independently
  supported inserted boundary.
- **R9.** A colon **MAY** introduce a following list, explanation, or
  emphasized completion when that material expands the preceding text. It
  **MUST NOT** be inserted between unrelated material merely to represent a
  pause.

### Quotations and paired marks

- **R10.** When direct speech or directly quoted text is marked inline with
  quotation marks, the opening and closing delimiters **MUST** form a visible,
  unambiguous pair. This requirement does not select a canonical quotation
  glyph profile.
- **R11.** A quotation nested inside another inline quotation **MUST** use a
  distinct delimiter pair or an equivalently unambiguous structured mechanism
  so that the two quotation levels can be recovered.
- **R12.** Parentheses used for supplementary material **MUST** occur as a
  balanced opening and closing pair.

### Hyphen and dash boundary

- **R13.** When SLS-0002 or a reviewed lexical record establishes a
  hyphenated spelling, the joining mark **MUST** remain internal to that form
  and **MUST NOT** be replaced by a sentence-level long dash. SLS-0004 **MUST
  NOT** independently decide that an unresolved compound requires a hyphen.

### Digital character and spacing profile

- **R14.** Ordinary SLS punctuation output **MUST** use the characters in the
  following table for the listed functions. A source-faithful field **MAY**
  preserve a different historical glyph when that status is explicit.

  | Function | Character | Code point |
  | --- | --- | --- |
  | full stop | `.` | U+002E |
  | comma | `,` | U+002C |
  | semicolon, when retained | `;` | U+003B |
  | colon | `:` | U+003A |
  | question mark | `?` | U+003F |
  | exclamation mark | `!` | U+0021 |
  | opening / closing parenthesis | `(` / `)` | U+0028 / U+0029 |
  | compound hyphen | `-` | U+002D |
  | sentence-level long dash | `—` | U+2014 |

- **R15.** Ordinary running text **MUST NOT** insert whitespace immediately
  before `. , ; : ? ! )` or a closing quotation mark, or immediately after `(`
  or an opening quotation mark. Where prose continues on the same line, one
  space **MUST** separate the completed punctuation unit from the next word;
  a closing quotation mark or parenthesis **MAY** immediately follow a mark
  that belongs inside it. A compound hyphen **MUST NOT** have adjacent spaces;
  a sentence-level U+2014 long dash **MUST** have one space on each side.
  Labeled source-faithful text is exempt from normalization.
- **R16.** Inline quotation marks **MUST** use one internally consistent paired
  profile: curly double/single (`“ ”`, `‘ ’`), straight double/single (`" "`,
  `' '`), or double/single guillemets (`« »`, `‹ ›`). Straight U+0027 **MUST
  NOT** be classified as quotation punctuation until SLS-0001 glottal-stop
  normalization has resolved its lexical use.
- **R17.** A sentence-final mark that belongs to quoted or parenthetical
  material **MUST** occur inside its closing delimiter. A mark that belongs to
  the containing sentence **MUST** occur outside. When a quotation ending the
  containing sentence already ends in `.`, `?`, or `!`, a second full stop
  **MUST NOT** be added solely for the containing sentence.

## Recommendations

- Authors SHOULD consult the
  [`SLS-0004 evidence map`](../../docs/standards/SLS-0004-evidence-map.md)
  before proposing a rule or canonical character profile.
- A document SHOULD use one internally consistent quotation profile unless it
  is reproducing source text or demonstrating variation.
- Editors SHOULD choose commas by grammatical structure and intended grouping,
  not by every audible pause.
- Tools SHOULD report an ambiguous apostrophe-shaped input, unmatched paired
  mark, or uncertain dash for review instead of silently normalizing it.
- Historical and literary punctuation SHOULD be preserved when source fidelity
  is the purpose and labeled accordingly.
- Corpus counts SHOULD be treated as attestation evidence, not as automatic
  proof of a punctuation rule.

## Examples

| Somali | Analysis | Status in this draft |
| --- | --- | --- |
| `Cali waxa uu tegay Xamar.` | ordinary complete statement | correct under R3 |
| `Ma Cali baa tegay?` | independent direct question | correct under R4 |
| `Tulaay!` | one-word exclamation | correct under R5 |
| `Jaamac, Faarax, Caasha, iyo Cali ayaa tegay.` | parallel series; comma before final conjunction retained from the source | correct under R6 |
| `Jaamac, Faarax, Caasha iyo Cali ayaa tegay.` | same series without a final serial comma | also permitted by R6 if grouping is clear |
| `Markii aan ka imid Hargeysa, waxa aan ku degay saaxiibkay Guuleed.` | introductory material delimited | permitted under R7 |
| `Cali, ayaa yimid.` | comma splits subject from predicate without inserted material | incorrect under R8 |
| `Xubnaha guddigu waa kuwan: Cumar, Maxamed, iyo Cabdi.` | colon introduces a list | permitted under R9 |
| `Waxa uu yidhi, “Waan imanaynaa.”` | visibly paired direct quotation | pairing conforms to R10 and final placement to R17 |
| `Abaartu (tii ka hore ee kale) waa cadawga dadka.` | source-attested balanced supplementary material | correct under R12 |
| `xeer-ilaaliye` | reviewed hyphenated lexical form | preserve under R13 |
| `xeer—ilaaliye` | long dash substituted for compound hyphen | incorrect under R13 |

### Per-rule example coverage

| Rule | Positive example | Negative example |
| --- | --- | --- |
| R1 | `goʼaan.` preserves the inherited alphabet and spelling. | `go'aan.` is emitted canonically with unresolved U+0027. |
| R2 | Punctuation cleanup leaves in-word `goʼaan` unchanged. | Cleanup deletes `ʼ` as though it were quotation punctuation. |
| R3 | `Cali waxa uu tegay Xamar.` ends a statement with a full stop. | `Cali waxa uu tegay Xamar` omits the mark in ordinary running prose. |
| R4 | `Ma Cali baa tegay?` ends a direct question with `?`. | `Ma Cali baa tegay.` ends that direct question with a full stop. |
| R5 | `Tulaay!` marks the presented exclamation. | `Tulaay.` presents the same exclamation with a full stop. |
| R6 | `Jaamac, Faarax iyo Caasha` separates successive non-final items. | `Jaamac Faarax iyo Caasha` leaves the first two series items unseparated. |
| R7 | `Cali, sidaan filayay, wuu yimid.` balances inserted-material commas. | `Cali, sidaan filayay wuu yimid.` supplies only the opening delimiter. |
| R8 | `Cali ayaa yimid.` keeps subject and predicate connected. | `Cali, ayaa yimid.` inserts a comma solely for a spoken pause. |
| R9 | `Qalabku waa kan: qalin iyo buug.` uses a colon for an expansion. | `Cali: ayaa yimid.` inserts a colon as an unrelated pause. |
| R10 | `Waxa uu yidhi, “Waan imanaynaa.”` has a recoverable pair. | `Waxa uu yidhi, “Waan imanaynaa.` lacks a closing delimiter. |
| R11 | `“Wuxuu yidhi ‘haa’.”` uses a distinct nested pair. | `“Wuxuu yidhi “haa”.”` reuses an ambiguous pair at both levels. |
| R12 | `Abaartu (tii hore) way dhammaatay.` balances parentheses. | `Abaartu (tii hore way dhammaatay.` omits the closing parenthesis. |
| R13 | `xeer-ilaaliye` preserves the reviewed internal hyphen. | `xeer—ilaaliye` substitutes a sentence-level long dash. |
| R14 | `Cali — Faarax` uses U+2014 for the long dash. | `Cali -- Faarax` substitutes two hyphen-minus characters. |
| R15 | `Cali, Faarax`; `xeer-ilaaliye`; `Cali — Faarax` follow the spacing profile. | `Cali ,Faarax`; `xeer - ilaaliye`; `Cali—Faarax` violate it. |
| R16 | `“hadal”` and `«hadal»` each use a consistent accepted pair. | `“hadal»` mixes unrelated opening and closing profiles. |
| R17 | `Waxay tidhi, “Ma timid?”` places the question mark inside its quote. | `Waxay tidhi, “Ma timid”?` places the quoted question mark outside. |

## Edge Cases

- **Question or exclamation inside a quotation.** Whether a final mark belongs
  inside or outside the closing quotation depends on what material it governs;
  R17 applies the semantic-scope decision.
- **Quotation profiles.** Straight quotes, curly quotation marks, and
  guillemets occur in the evidence library. R16 accepts all three paired
  profiles and requires internal consistency rather than choosing one winner.
- **Glottal stop and straight apostrophe.** U+0027 can represent legacy input
  for the Somali glottal stop or act as punctuation. Tools must apply SLS-0001
  normalization with lexical context before punctuation replacement.
- **Serial comma.** R6 supports both reviewed examples above. It does not make
  the English-language “Oxford comma” label or policy part of Somali grammar.
- **Semicolon.** The principal source names and describes the semicolon but
  supplies no distinct positive Somali examples. No compliance rule is adopted
  in this draft.
- **Long dash.** Interruption use is attested, but the exact Unicode character
  varies across sources; R14–R15 define U+2014 with one surrounding space on
  each side for ordinary output. Other source glyphs require a source-faithful
  label.
- **Ellipsis and repeated marks.** Literature contains ellipses and combinations
  such as `?!`, but the primary source does not define their complete
  functions. They are not normalized by this draft.
- **Block quotations and dialogue.** Structured block quotations may use
  document markup instead of inline quotation pairs. Canonical dialogue-turn
  layout is assigned to SLS-0407.
- **Parentheses and sentence-final marks.** This draft requires balanced
  parentheses; R17 assigns a final mark by semantic scope.
- **Source fidelity.** Historical spacing or quotation practice is not
  automatically an error when a field is explicitly preserving a source.

## Compliance Requirements

Because this standard is at `Proposed`, this checklist is provisional and does
not support a `Stable` compliance claim.

| # | Requirement | Traces to | Level |
| --- | --- | --- | --- |
| C1 | Text satisfies SLS-0002 and the inherited alphabet dependency | R1 | MUST |
| C2 | Punctuation processing preserves the canonical glottal-stop letter | R2 | MUST |
| C3 | Ordinary statements, direct questions, and exclamations use their supported final marks | R3–R5 | MUST |
| C4 | Simple parallel series separate successive non-final elements | R6 | MUST |
| C5 | Paired commas used around inserted material are balanced | R7 | MUST |
| C6 | Commas do not split core sentence elements merely by pause | R8 | MUST |
| C7 | A colon is not used merely as an unrelated pause marker | R9 | MUST |
| C8 | Inline direct and nested quotations have recoverable delimiter pairs | R10, R11 | MUST |
| C9 | Parentheses used for supplementary material are balanced | R12 | MUST |
| C10 | A reviewed compound hyphen is preserved and not replaced by a long dash | R13 | MUST |
| C11 | Fixed punctuation functions use the ordinary code-point profile unless source fidelity is explicit | R14 | MUST |
| C12 | Ordinary punctuation, hyphen, and long-dash spacing follows the digital profile | R15 | MUST |
| C13 | Inline quotations use a consistent accepted pair and preserve glottal-stop identity | R16 | MUST |
| C14 | Final marks are placed according to quoted, parenthetical, or containing-sentence scope | R17 | MUST |

## References

- SLS-0000, *SLS Standards Process Standard*.
- SLS-0001, *Somali Alphabet Standard*.
- SLS-0002, *Somali Orthography Standard*.
- [`SLS-0004 Punctuation Evidence Map`](../../docs/standards/SLS-0004-evidence-map.md).
- Maxamed Xaaji Xuseen Raabi, *Habka Qoraalka* (1977), punctuation section
  curated in
  [`resources/qoraal/05-astaamaynta.md`](../../resources/qoraal/05-astaamaynta.md).
- Grammar examples curated in
  [`resources/naxwe/14-naxwaha-cusub.md`](../../resources/naxwe/14-naxwaha-cusub.md#5-astaamaynta).
- Punctuation terminology curated in
  [`resources/naxwe/ereyfur.md`](../../resources/naxwe/ereyfur.md).
- Supporting prosodic evidence curated in
  [`resources/dhawaaq/05-codadka-sare.md`](../../resources/dhawaaq/05-codadka-sare.md).
- [The Unicode Standard, Chapter 6: Writing Systems and
  Punctuation](https://www.unicode.org/versions/Unicode17.0.0/core-spec/chapter-6/)
  — character identities and distinctions among hyphens, dashes, and
  quotation marks.
- [`SLS-0004 review log`](../../docs/standards/SLS-0004-review-log.md) — the public-comment record for this standard.

## Revision History

| Version | Date | Change |
| --- | --- | --- |
| 0.1.0 | 2026-08-23 | Initial evidence-mapped draft |
| 0.1.1 | 2026-08-30 | Editorial: repaired the long-dash edge-case sentence; linked the public review log |
