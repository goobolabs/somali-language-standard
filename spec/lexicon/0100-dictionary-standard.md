---
id: "0100"
sls_id: SLS-0100
title: Dictionary Standard
version: 0.1.0
standard_version: 0.1.0
status: Draft
category: lexicon
owner: maintainers
reviewers: []
dependencies:
  - SLS-0001
  - SLS-0002
  - SLS-0003
implements:
  - spec/lexicon/0100-dictionary-standard.md
publication_date: null
supersedes: null
superseded_by: null
revision_history:
  - version: 0.1.0
    date: 2026-09-04
    change: Initial evidence-mapped draft
---

> **Draft status:** This document defines the first reviewable dictionary-entry
> policy for the Milestone 4 pilot. It has not entered formal public comment,
> and no unreviewed pilot value is made normative by appearing in a source or
> review packet.

## Abstract

SLS-0100 defines how a curated Standard Somali dictionary entry is selected,
identified, structured, sourced, reviewed, corrected, and interpreted. It
connects the alphabet, orthography, and grammar standards to the machine-readable
lexicon schema without treating an OCR-derived dictionary line as an approved
record by itself.

## Purpose

The repository contains more than 48,000 dictionary headwords, but that source
surface mixes grammatical codes, homograph numbers, definitions, usage
examples, and cross-references in prose. It also lacks several fields required
by the SLS lexicon schema. This standard prevents bulk conversion from turning
source ambiguity, missing etymology, inferred plurals, or copied definitions
into apparently verified data.

## Scope

**In scope:** entries under `data/lexicon/core/` that claim Standard Somali
coverage; permanent entry identifiers; canonical headwords; the nine primary
word classes; sense separation; noun gender and plural recording; explicit
loanword uncertainty; optional lexical fields; source and licensing
provenance; record review; homographs; corrections; and schema conformance.

**Out of scope:** a complete theory of definitions, synonymy, antonymy,
loanword adaptation, morphology, or frequency. Those subjects belong to
SLS-0101 through SLS-0105. Regional-variety lexicons belong to future profiles
in the SLS-0700 block. This standard does not grant republication rights in an
upstream source and does not promote any particular pilot entry to reviewed
status.

## Definitions

- **Lexical entry** — one JSON object describing one reviewed lexeme or one
  draft candidate under `schemas/lexicon-entry.schema.json`.
- **Lexeme** — the lexical unit represented by an entry, including its word
  class and numbered meanings.
- **Canonical headword** — the spelling used in the entry's `word` field;
  source editorial marks such as homograph superscripts are not part of it.
- **Sense** — one numbered meaning of a lexeme. Sense numbers are local to one
  entry and begin at 1.
- **Homograph** — one of two or more distinct lexemes with the same canonical
  written headword.
- **Reviewed lexical fact** — a headword, class, gender, plural, meaning, or
  other value supported by named evidence and accepted by a recorded reviewer.
- **Reviewed absence of an ordinary plural** — a reviewer-approved finding that
  a noun has no applicable ordinary count plural for the recorded reading. It
  is encoded as JSON `null` and is not the same as an unknown plural.
- **Unresolved loanword status** — JSON `null` in `is_loanword`; it states that
  the entry has not been classified as either a loanword or a non-loanword.
- **Core lexicon** — the Standard Somali JSONL records under
  `data/lexicon/core/`.

## Normative Requirements

- **R1. Record identity and shape.** Every core lexical entry **MUST** validate
  against `schemas/lexicon-entry.schema.json` and **MUST** carry one permanent
  identifier of the form `sls:lex:NNNNNN`.
- **R2. Headword and variety.** A core entry **MUST** record a canonical
  headword conforming to SLS-0001 and SLS-0002 and **MUST** use `dialect: so`.
  A form known only from a regional variety **MUST NOT** be relabeled `so` merely
  to fit the core dataset.
- **R3. Primary word class.** `part_of_speech` **MUST** use one of the nine
  primary classes in SLS-0003 G10-R1. A source abbreviation **MUST NOT** be
  mapped automatically where its function is ambiguous or more specific than
  those nine classes.
- **R4. Definitions and senses.** Every entry **MUST** contain at least one
  sense with sequential numbering, an English definition, and a Somali gloss.
  The two texts **MUST** be reviewed for the same reading. Source wording whose
  reuse rights are unknown **MUST NOT** be copied into CC BY 4.0 data as though
  the project owned or could relicense it; a contributor-authored gloss or a
  compatible licensed quotation **MUST** be identified honestly in provenance.
- **R5. Noun morphology.** A noun's `gender` **MUST** be based on reviewed
  agreement evidence. Its `plural` **MUST** contain an attested lexical form or
  JSON `null` for a reviewed absence of an ordinary plural. A missing or
  uncertain plural **MUST NOT** be guessed from singular spelling, a productive
  suffix, or gender polarity.
- **R6. Cross-references.** A dictionary marker such as `ld` or `eeg` **MAY** be
  used as evidence for a candidate relationship, but **MUST NOT** be converted
  automatically into `synonyms`, `antonyms`, a preferred headword, or a merged
  sense. The relationship **MUST** first be classified and reviewed.
- **R7. Loanword status.** `is_loanword` **MUST** be `true`, `false`, or JSON
  `null`. `null` means unresolved and **MUST NOT** be interpreted as `false`.
  An entry set to `true` **MUST** include an evidence-backed `loan_origin`; an
  origin **MUST NOT** be inferred from spelling similarity alone.
- **R8. Optional fields.** `ipa`, `synonyms`, `antonyms`,
  `example_sentences`, and `frequency_rank` **MUST** be omitted unless their
  values come from an identified, applicable method or reviewed source. An
  omitted optional field **MUST NOT** be filled with generated or placeholder
  linguistic data in a release record.
- **R9. Provenance and licensing.** Metadata **MUST** identify the contributor,
  the precise lexical source or origin of the record, the review state, the
  applicable license, and the schema version. A generic reference to the whole
  evidence library **MUST NOT** replace a usable source pointer. Provenance
  **MUST** distinguish source facts from newly written definitions or
  translations when they have different origins.
- **R10. Review state.** A `draft` entry **MAY** preserve explicitly identified
  unresolved fields allowed by the schema. An entry marked `reviewed` or
  `verified` **MUST** name at least one reviewer in `metadata.reviewers`. Every
  entry included toward the Milestone 4 reviewed seed count **MUST** be
  `reviewed` or `verified`.
- **R11. Homographs.** Distinct lexemes sharing one written form **MUST** receive
  distinct `sls_id` values. Source homograph numbers **MUST NOT** be included in
  the canonical `word` value unless the character is independently part of the
  lexical spelling.
- **R12. Identifier permanence.** A published `sls:lex:` identifier **MUST NOT**
  be renumbered, reused for another lexeme, or reassigned to close a numerical
  gap. New identifiers **MUST** be allocated after the highest identifier
  already assigned.
- **R13. Uncertainty and correction.** When a required lexical decision is not
  supported, the candidate **MUST** stay out of the reviewed core dataset or use
  an explicit schema-supported unresolved value. A correction **MUST** retain
  the entry identifier, record updated provenance, and use `deprecated` rather
  than deletion when the original record must remain addressable.
- **R14. Dataset conformance.** Every committed core JSONL file **MUST** pass the
  repository validator, including schema, duplicate-ID, and reference checks.
  Passing structural validation **MUST NOT** be represented as proof that the
  linguistic content has been reviewed.

## Recommendations

- Curate small review batches that exercise different word classes and source
  problems before scaling extraction.
- Partition core JSONL files by the initial Somali letter or digraph, using
  stable lowercase filenames such as `b.jsonl`, `dh.jsonl`, and `sh.jsonl`.
- Keep source excerpts in the reviewer packet rather than copying them into the
  released dataset when a newly written gloss is sufficient.
- Prefer one clear definition per sense over preserving obsolete, circular, or
  encyclopedic source prose.

## Examples

These examples test record decisions, not the grammaticality of new Somali
sentences. Source-derived forms remain subject to entry-level review.

| Rule | Conforming example | Non-conforming example | Reason |
|---|---|---|---|
| R1 | `"sls_id":"sls:lex:000001"` in a schema-valid record | `"sls_id":"sls:lex:1"` | The second identifier is not permanent six-digit SLS form. |
| R2 | `"word":"mindi", "dialect":"so"` for a reviewed Standard Somali entry | Labeling an unreviewed regional form `"dialect":"so"` | Core scope cannot erase variety. |
| R3 | Source `m.dh` reviewed as `"part_of_speech":"magac"` | Mapping every source `qr` mechanically to `qodob` | The latter source class is not a guaranteed one-to-one primary-class mapping. |
| R4 | Two meanings represented as senses 1 and 2 with matching reviewed English and Somali text | Copying an upstream definition with unconfirmed rights and labeling it project-authored CC BY 4.0 | Sense structure does not erase authorship or rights. |
| R5 | `"gender":"feminine", "plural":"mindiyo"` after lexical review | Adding `"plural":"qalimo"` only because the form seems predictable | A plausible plural is still unreviewed evidence. |
| R5 | `"plural":null` after review finds no ordinary plural for the recorded mass reading | Using `null` merely because nobody checked | `null` records a reviewed absence, not missing work. |
| R6 | Holding `sonkor ld sokor` for relationship review | Automatically inserting `"synonyms":["sokor"]` | `ld` does not by itself settle canonicality or sense identity. |
| R7 | `"is_loanword":null` when etymology is unresolved | `"is_loanword":false` used as a default | Unknown is not evidence of non-borrowing. |
| R8 | Omitting `ipa` and `frequency_rank` where no method supplies them | Inventing an IPA string or rank to complete the object | Optional fields still require evidence. |
| R9 | A source pointer naming the collection file and headword, plus the origin of newly written glosses | `"source":"curated evidence library"` | The generic label is not independently auditable. |
| R10 | `review_status: reviewed` with a named maintainer reviewer | `review_status: reviewed` with no reviewer record | Review must be attributable and inspectable. |
| R11 | Two records with different IDs and the same canonical `"word":"qor"` | Canonical words `qor¹` and `qor²` copied from source numbering | Editorial homograph numbers are not letters in the headword. |
| R12 | Correcting the record while retaining `sls:lex:000042` | Reusing `sls:lex:000042` for another word | Published identifiers are permanent. |
| R13 | Keeping an unresolved candidate in its review packet | Guessing required facts and marking the entry reviewed | Unknown evidence is not an error to conceal. |
| R14 | `sls-validate check --root .` passes, followed by recorded lexical review | Claiming CI proves the definition is correct | Structural and linguistic review are separate gates. |

## Edge Cases

- A noun can have several source-listed plurals or plural readings with
  different gender behaviour. SLS-0100 does not choose among them by frequency
  or spelling; the entry stays in review until its scope is clear.
- A mass or collective noun may support a contextual unit or count reading.
  JSON `null` applies only to the reviewed reading represented by the entry; it
  is not a universal claim that no plural use can ever occur.
- A reviewed entry may retain `is_loanword: null` when the reviewer explicitly
  records that the available evidence does not settle etymological status.
  This is reviewed uncertainty, not an omitted review.
- An upstream source can number homographs differently from another source.
  SLS entry identity is carried by `sls_id`, not by importing either numbering
  scheme into the word.
- A cross-reference-only source entry can become a separate SLS entry if its
  status and relationship are reviewed. It is not silently discarded or merged.
- A source can provide a grammatical label but no evidence adequate for an SLS
  class, gender, plural, etymology, or variety decision. Presence in the source
  is evidence for the candidate, not automatic approval of every required
  field.
- Schema-valid fixtures under `tools/validators/tests/fixtures/` test software
  behaviour. They are not dictionary evidence and do not count toward the seed
  lexicon.

## Compliance Requirements

| # | Requirement | Traces to | Level |
|---|---|---|---|
| C1 | Every entry validates and has one permanent six-digit lexicon ID | R1 | MUST |
| C2 | Core headword and variety conform to the three foundation dependencies | R2 | MUST |
| C3 | Primary word class is reviewed and uses the SLS-0003 inventory | R3 | MUST |
| C4 | Senses contain aligned reviewed English and Somali definition text with honest rights provenance | R4, R9 | MUST |
| C5 | Noun gender and plural value are reviewed rather than generated | R5 | MUST |
| C6 | Dictionary cross-references are classified before becoming structured lexical relations | R6 | MUST |
| C7 | Loanword status and any origin are explicit and evidence-backed | R7 | MUST |
| C8 | Optional fields are evidence-backed or omitted | R8 | MUST |
| C9 | Metadata gives precise, auditable provenance and the correct review state | R9, R10 | MUST |
| C10 | Homographs have distinct IDs without editorial superscripts in canonical headwords | R11 | MUST |
| C11 | Published entry identifiers remain permanent through correction or deprecation | R12, R13 | MUST |
| C12 | Core data passes repository validation without confusing validation with linguistic review | R14 | MUST |

## References

- [SLS-0001 Somali Alphabet Standard](../orthography/0001-alphabet.md)
- [SLS-0002 Somali Orthography Standard](../orthography/0002-spelling-rules.md)
- [SLS-0003 Somali Grammar Standard](../grammar/0018-somali-grammar-standard.md)
- [`schemas/lexicon-entry.schema.json`](../../schemas/lexicon-entry.schema.json)
- [`resources/qaamuus/`](../../resources/qaamuus/) — principal dictionary evidence
- [`resources/madax-ereyo/`](../../resources/madax-ereyo/) — derived headword cross-check
- [`resources/sarfe/`](../../resources/sarfe/) — reviewed morphology tables
- [SLS-0100 evidence map](../../docs/standards/SLS-0100-evidence-map.md)
- [SLS-0100 review log](../../docs/standards/SLS-0100-review-log.md)
- [Milestone 4 tracking issue](https://github.com/goobolabs/somali-language-standard/issues/24)

## Revision History

| Version | Date | Change |
|---|---|---|
| 0.1.0 | 2026-09-04 | Initial evidence-mapped draft |
