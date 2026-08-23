---
id: "0002"
sls_id: SLS-0002
title: Somali Orthography Standard
version: 0.1.0
standard_version: 0.1.0
status: Proposed
category: foundation
owner: language-council
reviewers: []
dependencies:
  - SLS-0001
implements:
  - spec/orthography/0002-spelling-rules.md
publication_date: null
supersedes: null
superseded_by: null
revision_history:
  - version: 0.1.0
    date: 2026-08-23
    change: Initial evidence-mapped draft
---

> **Proposed status:** This document is accepted for formal public comment.
> Its scope is frozen for the comment period, but its requirements remain
> non-stable and may be refined through recorded review decisions.

## Abstract

This standard defines how standard written Somali represents word boundaries,
bound conjunction clitics, vowel and consonant length, supported
morphophonemic changes, compounds, unmarked tone, and adapted loanwords. It
builds on the alphabet and encoding requirements of SLS-0001 and separates
spelling decisions from punctuation, capitalization, grammar, and detailed
loanword governance.

## Purpose

Somali sources and current writing contain competing practices for expanding
speech contractions, attaching clitics, spelling compounds, and representing
some sound changes. A mechanical rule such as “write every analyzable element
separately” or “hyphenate every compound” contradicts other attested evidence.
SLS-0002 provides an implementable spelling layer while keeping unresolved
questions visible instead of converting uncertain source analyses into rules.

## Scope

**In scope:** orthographic word boundaries; the written attachment of the
conjunction clitics *-na* and *-se*; the separation of free preverbal negative
*ma*; preservation of reviewed fused forms; consonant gemination and supported
morphophonemic surface forms; treatment of reviewed compound spellings; the
ordinary non-marking of tone; and the spelling boundary for adapted loanwords.

**Out of scope:**

- alphabet inventory, collation, code points, and digraph identity, governed by
  SLS-0001;
- full grammatical paradigms and syntactic conditions, governed by SLS-0003;
- punctuation and hyphen rendering, governed by SLS-0004;
- capitalization, governed by SLS-0005;
- detailed loanword adoption and adaptation, governed by SLS-0103;
- phonetic, tone, and stress transcription, reserved for the SLS-0600 block;
- dialect-specific norms, reserved for the SLS-0700 block.

## Definitions

- **Orthographic word** — a written unit separated from adjacent independent
  units by whitespace, except where a reviewed rule attaches a clitic or where
  a reviewed lexical form is closed or hyphenated.
- **Independent word** — a grammatical unit that is not an affix or bound
  clitic and can occupy its documented syntactic position without a host.
- **Bound clitic** — a grammatical element that depends on a neighboring host
  and is written attached under a rule in this standard.
- **Host** — the orthographic word to which a bound clitic attaches.
- **Reviewed form** — a spelling explicitly approved in an SLS standard,
  canonical structured record, or recorded standards decision. Appearance in
  the descriptive `resources/` library alone does not make a form normative.
- **Surface form** — the written result after a supported morphological or
  morphophonemic change applies.
- **Compound** — a lexical unit formed from two or more bases. A compound may
  be closed, hyphenated, or spaced only when its reviewed spelling establishes
  that form.
- **Adapted loanword** — a borrowed word incorporated into Somali spelling and
  sound patterns, as distinct from an unadapted foreign name or quotation.

## Normative Requirements

### Dependency and character representation

- **R1.** Text conforming to SLS-0002 **MUST** satisfy the alphabet, character,
  digraph, glottal-stop, and encoding requirements of SLS-0001.
- **R2.** A reviewed long vowel **MUST** be written by doubling its vowel
  letter, and a reviewed geminate consonant **MUST** be written by doubling its
  consonant letter. This includes geminate *d*. A writer or tool **MUST NOT**
  infer length or gemination for an individual word without lexical or
  morphological evidence.

### Word boundaries and clitics

- **R3.** Independent words **MUST** be separated by whitespace. This rule
  **MUST NOT** be used to split an affix, a bound clitic governed below, or a
  reviewed closed compound.
- **R4.** When *-na* is the bound conjunction clitic connecting clauses or
  coordinated material, it **MUST** attach to its host: *Daahirna, adiguna,
  waana*. It **MUST NOT** be written as a separate conjunction in that
  function. This does not affect independent object-pronoun *na*.
- **R5.** When *-se* is the bound adversative conjunction clitic, it **MUST**
  attach to its host: *Xasanse, kumase, laakiinse*. It **MUST NOT** be written
  separately in that function.
- **R6.** Free preverbal negative *ma* **MUST** be separated from the following
  verb or verbal group: *ma keenin, ma tago, ma aha*. This requirement does not
  split reviewed fused interrogative or focus forms such as *miyaan*.
- **R7.** A reviewed fused form **MUST NOT** be mechanically split solely
  because its grammatical components can be identified. Forms such as *maxaa,
  maxay,* and *maxaad* remain single orthographic words when used in their
  reviewed interrogative functions.

### Morphophonemic spelling

- **R8.** When a reviewed morphological combination has a documented surface
  form, writers and tools **MUST** spell the surface form rather than an
  unattested concatenation of underlying pieces. Examples include *qaad + tay
  → qaadday*, *bad + ta → badda*, *bil + ta → bisha*, and *magac + o
  → magacyo*.
- **R9.** A tool **MUST NOT** generalize a morphophonemic change beyond the
  reviewed class or paradigm that supports it. Similar-looking forms are not
  sufficient evidence for automatic rewriting.

### Compounds

- **R10.** When a compound has a reviewed lexical spelling, writers and tools
  **MUST** preserve that spelling, whether closed or hyphenated. They **MUST
  NOT** apply a universal join, split, or hyphenation transformation to all
  compounds.
- **R11.** A newly formed or unresolved compound **MUST NOT** be presented as a
  standardized SLS spelling until its closed, hyphenated, or spaced form has a
  recorded review decision.

### Unmarked features and loanwords

- **R12.** Ordinary SLS orthography **MUST NOT** add tone or stress diacritics
  to distinguish words. Linguistic transcription **MAY** mark those features
  when clearly labeled and kept outside ordinary running-text fields.
- **R13.** An adapted loanword **MUST** use the alphabet and character rules of
  SLS-0001. Where SLS-0001 permits an unadapted foreign proper name, loanword,
  or quotation, it **MAY** retain foreign letters only when its unadapted status
  is explicit; writers **SHOULD** otherwise adapt it as directed by SLS-0001
  R11. Detailed adaptation mappings are deferred to SLS-0103.
- **R14.** A reviewed spelling containing the glottal-stop letter **MUST**
  preserve canonical U+02BC as required by SLS-0001. A tool **MUST NOT** insert
  or delete that letter solely from its position in a word.
- **R15.** Where a reviewed morpheme or compound boundary produces a sequence
  resembling `sh`, `dh`, or `kh`, writers and tools **MUST** preserve the
  reviewed surface spelling and boundary analysis. They **MUST NOT** invent an
  apostrophe, hyphen, or other separator solely to force a two-letter parse.
- **R16.** A reviewed spelling containing adjacent vowels or *y/w* **MUST** be
  preserved. A tool **MUST NOT** automatically convert between a vowel
  sequence and *y/w* without a lexical or morphological rule for that form.
- **R17.** Clitic attachment, contraction, or expansion not explicitly governed
  by R4–R7 **MUST NOT** be standardized by analogy alone. It requires a
  recorded lexical decision or a rule in SLS-0003.

## Recommendations

- Authors SHOULD consult the
  [`SLS-0002 evidence map`](../../docs/standards/SLS-0002-evidence-map.md)
  before proposing a new rule or example.
- Editors SHOULD prefer examples supported by more than one independent
  resource family when a rule is morphologically or dialectally sensitive.
- Tools SHOULD report an unresolved compound or contraction for review instead
  of silently normalizing it.
- Corpus counts SHOULD be treated as attestation evidence, not as automatic
  proof of correctness; duplicated sources and retained OCR variation can
  distort frequency.
- Input normalization SHOULD preserve the distinction between the glottal-stop
  letter and punctuation as required by SLS-0001.

## Examples

| Somali | Analysis | Status in this draft |
| --- | --- | --- |
| `Daahirna shaah buu cabbay.` | conjunction clitic *-na* attached to host | correct under R4 |
| `Daahir na shaah buu cabbay.` | *-na* incorrectly separated in conjunction function | incorrect under R4 |
| `Xasanse wuu shaqaynayaa.` | adversative *-se* attached | correct under R5 |
| `Cali hadiyad ma keenin.` | free preverbal negative *ma* | correct under R6 |
| `maxaad` | reviewed fused form, analyzed as *maxaa + aad* | correct under R7 |
| `qaadday` | reviewed *qaad + tay* surface form | correct under R2 and R8 |
| `qaad + tay → qaaday` | single *d* substituted in this specific derivation; *qaaday* is valid elsewhere | incorrect for this derivation |
| `badda` | reviewed *bad + ta* surface form | correct under R8 |
| `afmiinshaar` | reviewed closed dictionary form | preserve under R10 |
| `xeer-ilaaliye` | reviewed hyphenated dictionary form | preserve under R10 |
| `baabuur` | long *aa* written by doubling | correct under R2 |

### Per-rule example coverage

| Rule | Positive example | Negative example |
| --- | --- | --- |
| R1 | `goʼaan` preserves the SLS-0001 character profile. | `go'aan` is emitted as canonical output with U+0027. |
| R2 | `baabuur`, `qaadday` preserve reviewed length and gemination. | `babuur`, `qaad + tay → qaaday` undouble those specific reviewed forms. |
| R3 | `Cali wuu yimid.` separates independent words. | `Caliwuuyimid.` mechanically joins the same independent words. |
| R4 | `Daahirna shaah buu cabbay.` attaches conjunction clitic *-na*. | `Daahir na shaah buu cabbay.` separates that bound conjunction clitic. |
| R5 | `Xasanse wuu shaqaynayaa.` attaches adversative *-se*. | `Xasan se wuu shaqaynayaa.` separates that bound adversative clitic. |
| R6 | `Cali hadiyad ma keenin.` separates free negative *ma*. | `Cali hadiyad makeenin.` joins the free negative to the verb. |
| R7 | `maxaad` preserves a reviewed fused interrogative form. | `maxa aad` is produced solely by mechanical decomposition. |
| R8 | `bad + ta → badda` uses the reviewed surface form. | `bad + ta → badta` emits an unattested raw concatenation. |
| R9 | A rule applies `bad + ta → badda` only to its reviewed class. | A similar-looking unreviewed form is rewritten by the same rule automatically. |
| R10 | `afmiinshaar` and `xeer-ilaaliye` retain their reviewed spellings. | `af miinshaar` and `xeerilaaliye` are produced by one universal compound rewrite. |
| R11 | A new compound is returned as `not standardized`. | A new compound is labelled SLS-standard without a recorded decision. |
| R12 | Ordinary text writes an unmarked reviewed form such as `inan`. | Ordinary text adds an acute accent solely to mark tone: `inán`. |
| R13 | `baabuur` uses SLS-0001 characters as an adapted loanword. | `pizza` is labelled adapted Somali while retaining excluded `p` and `z`. |
| R14 | `goʼaan` preserves its reviewed U+02BC letter. | `goaan` is produced by deleting the letter because of its position. |
| R15 | A reviewed `s+h` boundary is stored unchanged with boundary metadata. | An apostrophe is inserted solely to force the `s+h` parse. |
| R16 | Reviewed `Soomaali` is preserved. | A tool rewrites it as `Sowmaali` by an automatic vowel-to-*w* rule. |
| R17 | An unreviewed contraction is returned as `not standardized`. | It is joined or split merely because a reviewed form looks similar. |

## Edge Cases

- **Homographic free and bound forms.** Object-pronoun *na* is an independent
  grammatical element in examples such as *Idinkaa na siiyay buuggan*; R4
  governs only conjunction clitic *-na*.
- **Expanded analyses.** A historical source may show an expansion such as
  *waxa aan* beside *waxaan*. This draft does not yet standardize the complete
  contraction inventory; R7 prevents blind splitting while the remaining
  forms are reviewed individually.
- **Negative and interrogative paradigms.** This draft records supported
  surface spellings but leaves the full paradigms and their syntactic
  conditions to SLS-0003.
- **Compounds.** The evidence library contains closed, hyphenated, and spaced
  variants. R10 preserves reviewed forms; it does not supply a productive rule
  for new compounds.
- **Digraphs across boundaries.** A true `s+h`, `d+h`, or `k+h` sequence across
  a morpheme or compound boundary can resemble a digraph. R15 preserves the
  reviewed spelling and metadata without inventing a separator.
- **Vowel sequences and semivowels.** The choice between adjacent vowels and
  *y/w* is lexical or morphological where analyses compete. R16 prohibits a
  mechanical conversion.
- **Source variation.** Historical, regional, or author-specific spelling is
  not automatically an error. It must not be normalized without a scope label
  and evidence.

## Compliance Requirements

Because this standard is at `Proposed`, this checklist is provisional and does
not support a `Stable` compliance claim.

| # | Requirement | Traces to | Level |
| --- | --- | --- | --- |
| C1 | Text satisfies the alphabet and encoding dependency | R1 | MUST |
| C2 | Reviewed vowel length and consonant gemination are doubled | R2 | MUST |
| C3 | Independent words are separated without splitting governed bound forms | R3 | MUST |
| C4 | Conjunction clitics *-na/-se* attach to their hosts | R4, R5 | MUST |
| C5 | Free preverbal negative *ma* is separated | R6 | MUST |
| C6 | Reviewed fused forms are not mechanically split | R7 | MUST |
| C7 | Reviewed morphophonemic surface forms are preserved without overgeneralization | R8, R9 | MUST |
| C8 | Reviewed compound spelling is preserved; unresolved compounds are not standardized | R10, R11 | MUST |
| C9 | Ordinary text does not add tone or stress marks | R12 | MUST |
| C10 | Adapted loanwords follow SLS-0001 characters | R13 | MUST |
| C11 | Reviewed glottal-stop spellings preserve U+02BC without position-based insertion or deletion | R14 | MUST |
| C12 | Boundary-like digraph sequences and vowel/*y/w* spellings are preserved without invented transformations | R15, R16 | MUST |
| C13 | Unreviewed clitic and contraction patterns are not standardized by analogy | R17 | MUST |

## References

- SLS-0000, *SLS Standards Process Standard*.
- SLS-0001, *Somali Alphabet Standard*.
- [`SLS-0002 Orthography Evidence Map`](../../docs/standards/SLS-0002-evidence-map.md).
- Maxamed Xaaji Xuseen Raabi, *Habka Qoraalka* (1977), curated in
  [`resources/qoraal/`](../../resources/qoraal/).
- Guddiga Af Soomaaliga, *Aasaaska Naxwaha Af Soomaaliga* (1973), and Abdalla
  Omar Mansur and Annarita Puglielli, *Barashada Naxwaha Af Soomaaliga* (1999),
  curated in [`resources/naxwe/`](../../resources/naxwe/).
- Morphological paradigms curated in
  [`resources/sarfe/`](../../resources/sarfe/).
- Maxamed Xaaji Xuseen Raabi, *Codaynta Af Soomaaliga* (1977), curated as
  supporting phonological evidence in
  [`resources/dhawaaq/`](../../resources/dhawaaq/).

## Revision History

| Version | Date | Change |
| --- | --- | --- |
| 0.1.0 | 2026-08-23 | Initial evidence-mapped draft |
