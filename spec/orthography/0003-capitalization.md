---
id: "0003"
sls_id: SLS-0005
title: Somali Capitalization Standard
version: 0.1.0
standard_version: 0.1.0
status: Proposed
category: foundation
owner: language-council
reviewers: []
dependencies:
  - SLS-0002
implements:
  - spec/orthography/0003-capitalization.md
publication_date: null
supersedes: null
superseded_by: null
revision_history:
  - version: 0.1.0
    date: 2026-08-23
    change: Initial evidence-mapped draft
---

> **Proposed status and evidence limitation:** This document is accepted for
> formal public comment. Its direct capitalization evidence is an interim
> supplementary source rather than a Somali-primary capitalization chapter.
> Its scope is frozen for the comment period, but its requirements remain
> non-stable and may be refined through recorded review decisions.

## Abstract

This standard defines a conservative capitalization core for ordinary written
Somali: independent sentence beginnings, reviewed proper names, weekdays,
months, the ethnonym and language name *Soomaali*, lowercase common uses of
season and direction terms, and inherited digraph case. It explicitly avoids
creating unsupported title-case, institutional-name, acronym, or display-case
rules.

## Purpose

Somali uses the Latin script and therefore has letter case, but the current
evidence baseline lacks a comprehensive Somali-primary capitalization source.
Unrestricted analogy with English would capitalize categories for which the
repository has no Somali rule. SLS-0005 establishes only the supported core
and gives later style and institutional standards ownership of the remaining
questions.

## Scope

**In scope:** independent sentence beginnings; personal, geographic, and other
reviewed proper names; weekdays and months; *Soomaali* as ethnonym and language
name; the construction *af Soomaali*; common season and direction terms; and
capitalized Somali digraphs.

**Out of scope:** heading and publication-title case; full official names of
institutions; honorific and office-title conventions; acronym formation;
all-capital emphasis; trademarks, usernames, and product casing; detailed
foreign-name adaptation; and source-specific display typography.

## Definitions

- **Capital letter** — the uppercase member of a Somali letter's case pair as
  defined by SLS-0001.
- **Cased letter** — a letter with uppercase and lowercase forms.
- **Independent sentence** — a complete sentence that is not grammatically
  embedded as an incomplete continuation of preceding text.
- **Proper name** — a reviewed name that uniquely identifies a person, place,
  organization, work, or other particular referent.
- **Proper-name component** — a word established as part of a multiword proper
  name, rather than a nearby generic description.
- **Common use** — use of a word for a general class, direction, season, or
  description rather than as a reviewed proper name.
- **Source-faithful casing** — casing intentionally retained because a field
  reproduces a source, registered name, trademark, or username rather than
  asserting ordinary SLS normalization.

## Normative Requirements

### Dependencies and letter forms

- **R1.** Text conforming to SLS-0005 **MUST** satisfy SLS-0002 and its
  SLS-0001 dependency.
- **R2.** When a Somali digraph is capitalized in ordinary text, only its first
  character **MUST** be uppercase: *Sh, Dh,* and *Kh*. A tool **MUST NOT**
  produce *SH, DH,* or *KH* merely to capitalize one digraph, except in an
  explicitly all-capital or source-faithful field.

### Sentence beginnings

- **R3.** The first cased letter of an independent sentence **MUST** be
  uppercase. The same requirement applies when an independent sentence begins
  inside a direct quotation.
- **R4.** A comma, semicolon, colon, hyphen, or long dash **MUST NOT** by itself
  cause the following word to be capitalized. Capitalization follows only when
  the following material independently satisfies a rule in this standard.

### Proper and common names

- **R5.** The first cased letter of a reviewed personal, geographic, or other
  proper name **MUST** be uppercase.
- **R6.** In a multiword proper name, every word established as a proper-name
  component **MUST** begin with a capital letter. This requirement **MUST NOT**
  be extended into a universal title-case rule for particles, connectors,
  generic descriptions, headings, or publication titles.
- **R7.** A common noun or common descriptive use **MUST NOT** be capitalized
  solely because the same spelling can occur in a proper name. Sentence-start
  and other independently applicable rules still apply.

### Conventional named classes

- **R8.** Somali weekday and month names **MUST** begin with a capital letter
  when used as calendar names: *Isniin, Talaado, Arbaco, Khamiis, Jimce,
  Sabti, Axad; Jannaayo, Abriil,* and *Maajo*.
- **R9.** *Soomaali* **MUST** begin with a capital letter when it is the
  ethnonym or the name of the language.
- **R10.** In the ordinary language-name construction *af Soomaali*, generic
  *af* **MUST** remain lowercase unless it begins a sentence or belongs to a
  separately reviewed full proper title.
- **R11.** Ordinary common uses of season and direction terms **MUST** be
  lowercase: *jiilaal, gu, dayr, xagaa, koonfur, waqooyi, bari,* and
  *galbeed*. They **MAY** begin with a capital when R3 applies or when the term
  is an established component of a reviewed proper name.

### Preserved external casing

- **R12.** A source-faithful or registered-casing field **MAY** preserve casing
  not generated by this standard, but its status **MUST** be explicit. Such a
  preserved form **MUST NOT** be presented as evidence for a general Somali
  capitalization rule without separate review.

## Recommendations

- Authors SHOULD consult the
  [`SLS-0005 evidence map`](../../docs/standards/SLS-0005-evidence-map.md)
  before proposing a broader capitalization category.
- Editors SHOULD classify a word as proper or common before changing its case.
- Tools SHOULD avoid automatic English-style title casing for Somali text.
- Official names, acronyms, and publication titles SHOULD retain their
  reviewed or source-faithful casing until the owning standard defines them.
- A Somali-primary capitalization source SHOULD trigger a new evidence review
  before SLS-0005 advances beyond public comment.

## Examples

| Somali | Analysis | Status in this draft |
| --- | --- | --- |
| `Cali wuu yimid.` | independent sentence and personal name | correct under R3 and R5 |
| `cali wuu yimid.` | sentence and personal name begin lowercase | incorrect under R3 and R5 |
| `Shabeel baa yimid.` | sentence-initial digraph | correct under R2 and R3 |
| `SHabeel baa yimid.` | both digraph characters uppercased in ordinary text | incorrect under R2 |
| `Soomaaliya, Muqdisho, Hargeysa` | reviewed geographic names | correct under R5 |
| `Sayid Maxamed` | two reviewed proper-name components | correct under R6 |
| `Maanta waa Sabti.` | weekday calendar name | correct under R8 |
| `Waxaan ku dhalannay Abriil.` | month calendar name | correct under R8 |
| `Waxaan ahay Soomaali.` | ethnonym | correct under R9 |
| `Waxaan bartaa af Soomaali.` | generic *af* plus language name | correct under R9 and R10 |
| `Waxaan u safray koonfur.` | ordinary direction | correct under R11 |
| `Jiilaal baa bilaabmay.` | common season at sentence start | capitalized only under R3 |

### Per-rule example coverage

| Rule | Positive example | Negative example |
| --- | --- | --- |
| R1 | `goʼaan` preserves the inherited SLS-0001/SLS-0002 spelling. | `go'aan` is emitted as canonical text with the wrong code point. |
| R2 | `Sheeko` capitalizes only the first digraph character. | `SHeeko` capitalizes both characters in ordinary text. |
| R3 | `Cali wuu yimid.` begins with an uppercase cased letter. | `cali wuu yimid.` begins an independent sentence in lowercase. |
| R4 | `Liisku waa kan: qalin, buug.` leaves the post-colon common word lowercase. | `Liisku waa kan: Qalin, buug.` capitalizes solely because a colon precedes it. |
| R5 | `Cali` and `Muqdisho` begin reviewed proper names with capitals. | `cali` and `muqdisho` lowercase those reviewed names. |
| R6 | `Sayid Maxamed` capitalizes both reviewed proper-name components. | `Sayid maxamed` lowercases one reviewed proper-name component. |
| R7 | `Cali waa macallin.` leaves the common noun lowercase. | `Cali waa Macallin.` capitalizes the common noun merely by emphasis. |
| R8 | `Maanta waa Sabti; waxaan ku dhalannay Abriil.` uses calendar-name capitals. | `Maanta waa sabti; waxaan ku dhalannay abriil.` lowercases the same calendar names. |
| R9 | `Waxaan ahay Soomaali.` capitalizes the ethnonym. | `Waxaan ahay soomaali.` lowercases the ethnonym. |
| R10 | `Waxaan bartaa af Soomaali.` leaves generic *af* lowercase. | `Waxaan bartaa Af Soomaali.` capitalizes generic *af* without another rule. |
| R11 | `Waxaan u safray koonfur xilligii jiilaalka.` uses common terms. | `Waxaan u safray Koonfur xilligii Jiilaalka.` capitalizes those common uses. |
| R12 | A source field labelled `source-faithful` preserves its registered casing. | That casing is presented as a general Somali rule without separate review. |

## Edge Cases

- **Titles and headings.** This draft does not decide sentence case versus
  title case for headings, books, articles, or creative works.
- **Institutional names.** R5–R6 apply only to reviewed proper-name components;
  a future institutional style standard must decide internal generic words.
- **Honorifics and offices.** Forms such as *Sayid* can be part of an attested
  proper name, but this draft does not create a universal honorific rule.
- **After a colon.** R4 prevents automatic capitalization. A following
  independent sentence can still satisfy R3.
- **Direct quotations.** An independent quoted sentence satisfies R3; a quoted
  fragment does not become a sentence merely because quotation marks enclose
  it.
- **Hyphenated names.** Each side is capitalized only if independently
  established as a proper-name component; no mechanical all-components rule is
  adopted.
- **Acronyms and all caps.** Reviewed acronyms and explicit all-capital fields
  may be preserved under R12, but this standard does not generate them.
- **Foreign and registered casing.** Trademarks, usernames, and unadapted names
  may be source-faithful; they do not change ordinary Somali rules.
- **Seasons and directions in proper names.** R11 requires a reviewed proper
  name, not geographic importance or emphasis alone.
- **Evidence limitation.** R8–R11 are provisional because the direct chapter
  is supplementary rather than Somali-primary.

## Compliance Requirements

Because this standard is at `Proposed`, this checklist is provisional and does
not support a `Stable` compliance claim.

| # | Requirement | Traces to | Level |
| --- | --- | --- | --- |
| C1 | Text satisfies SLS-0002 and inherited alphabet case rules | R1 | MUST |
| C2 | Capitalized Somali digraphs uppercase only their first character | R2 | MUST |
| C3 | Independent sentences begin with a capital letter | R3, R4 | MUST |
| C4 | Reviewed proper names and proper-name components begin with capitals | R5, R6 | MUST |
| C5 | Common uses are not capitalized merely by association with a proper name | R7 | MUST |
| C6 | Weekday and month calendar names begin with capitals | R8 | MUST |
| C7 | The ethnonym/language name *Soomaali* is capitalized and generic *af* remains lowercase | R9, R10 | MUST |
| C8 | Common season and direction terms remain lowercase absent another rule | R11 | MUST |
| C9 | Nonstandard source-faithful casing is explicitly labeled | R12 | MUST |

## References

- SLS-0000, *SLS Standards Process Standard*.
- SLS-0001, *Somali Alphabet Standard*.
- SLS-0002, *Somali Orthography Standard*.
- SLS-0004, *Somali Punctuation Standard*.
- [`SLS-0005 Capitalization Evidence Map`](../../docs/standards/SLS-0005-evidence-map.md).
- Morgan Nilsson, *Beginner's Somali Grammar* §2.3 (2024 preliminary), curated
  as an interim supplement in
  [`resources/qoraal/06-xarafka-weyn.md`](../../resources/qoraal/06-xarafka-weyn.md).
- Proper-noun evidence curated in
  [`resources/naxwe/13-aasaaska-naxwaha.md`](../../resources/naxwe/13-aasaaska-naxwaha.md).

## Revision History

| Version | Date | Change |
| --- | --- | --- |
| 0.1.0 | 2026-08-23 | Initial evidence-mapped draft with explicit supplementary-source limitation |
