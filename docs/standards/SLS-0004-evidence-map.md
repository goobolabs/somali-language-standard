# SLS-0004 Punctuation Evidence Map

- **Status:** reviewed evidence map; approved 2026-08-23 as input to the
  initial SLS-0004 `Draft` and retained for its `Proposed` review;
  non-normative
- **Prepared:** 2026-08-23
- **Target standard:** SLS-0004 Somali Punctuation Standard
- **Normative dependencies:** SLS-0002, which in turn depends on SLS-0001

This document maps the completed `resources/` evidence baseline to the topics
that SLS-0004 must resolve. It is a research and decision aid, not a
punctuation standard. A mark or example appearing here is not approved merely
because a historical source or current resource records it.

## Evidence policy

Evidence is considered in this order:

1. **Inherited requirements** — SLS-0001 defines the glottal stop as a letter,
   not punctuation. The SLS-0002 `Draft` assigns punctuation and hyphen
   rendering to SLS-0004 while retaining authority over whether a reviewed
   compound is hyphenated. SLS-0004 must not contradict either boundary.
2. **Direct punctuation evidence** —
   [`resources/qoraal/05-astaamaynta.md`](../../resources/qoraal/05-astaamaynta.md)
   preserves the punctuation section of Maxamed Xaaji Xuseen Raabi's *Habka
   Qoraalka* (1977). It is the principal source for the inventory, functions,
   and Somali examples.
3. **Supporting grammar and terminology evidence** —
   [`resources/naxwe/14-naxwaha-cusub.md`](../../resources/naxwe/14-naxwaha-cusub.md#5-astaamaynta),
   [`resources/naxwe/ereyfur.md`](../../resources/naxwe/ereyfur.md), and the
   dictionary clarify the terms *astaamayn, hakad, joogsi, kolmo,* and
   *qaanso*.
4. **Supporting prosodic evidence** —
   [`resources/dhawaaq/05-codadka-sare.md`](../../resources/dhawaaq/05-codadka-sare.md)
   relates some spoken boundaries to comma, dash, question, exclamation, and
   sentence-final punctuation. It does not make orthographic rules.
5. **Usage validation** —
   [`resources/suugaan/`](../../resources/suugaan/) can show that a convention
   occurs, but the collection preserves historical, source-specific, and
   OCR-damaged punctuation. Attestation does not by itself establish
   correctness.
6. **Technical character evidence** — [The Unicode Standard, Chapter
   6](https://www.unicode.org/versions/Unicode17.0.0/core-spec/chapter-6/)
   distinguishes quotation marks, U+002D HYPHEN-MINUS, and U+2014 EM DASH.
   Unicode supplies character identity, not Somali-language placement rules.

Conflicting or under-supported evidence must not become a **MUST** rule without
a written resolution under SLS-0000.

## Scope boundary

### Intended for SLS-0004

- the punctuation inventory for ordinary Somali running text;
- the functions and placement of sentence-final marks;
- commas, semicolons, colons, and dashes within sentences;
- quotation marks and punctuation for direct speech and nested quotations;
- parentheses and any other approved paired marks;
- punctuation spacing and character representation;
- the rendering of a hyphen where SLS-0002 or a reviewed lexical record has
  already established that a hyphen is required;
- the distinction between punctuation characters and the glottal-stop letter.

### Governed elsewhere

- alphabet identity and the canonical glottal-stop character: SLS-0001;
- whether an established compound is closed, spaced, or hyphenated: SLS-0002;
- capitalization after or within punctuation: SLS-0005;
- the syntactic analysis used to identify a clause, phrase, vocative, or
  parenthetical: SLS-0003;
- typography, document-house style, and citation style beyond punctuation
  meaning: the SLS-0400 writing and style block;
- phonetic and prosodic transcription: the future SLS-0600 block;
- machine formats for dates, times, decimals, and grouped numbers: their
  future data or style standards.

## Primary-source inventory

The principal source groups twelve named items into four classes. One of the
twelve, *hamsa*, is explicitly rejected as punctuation and treated as a Somali
consonant letter. The resulting punctuation evidence covers eleven items.

| Source group | Somali name | Mark shown | Initial scope reading |
| --- | --- | --- | --- |
| meaning-related | *hamsa* | `'` | excluded from punctuation; letter governed by SLS-0001 |
| meaning-related | *jiitin gaaban* | `-` | hyphen in compound forms; classification belongs partly to SLS-0002 |
| sentence-final | *joogsi* | `.` | ordinary declarative sentence ending |
| sentence-final | *weydiin* | `?` | interrogative sentence ending |
| sentence-final | *yaab* | `!` | exclamatory word or sentence ending |
| sentence-internal | *hakad* | `,` | separation of listed, inserted, introductory, and related material |
| sentence-internal | *dhibic hakad* | `;` | intermediate pause; no distinct source examples |
| sentence-internal | *laba dhibcood* | `:` | introduction, explanation, emphasis, and time notation |
| sentence-internal | *jiitin dheer* | `—` | interruption or specially broken delivery |
| special | *kolmo* | `' '` | nested quotation or literary title in the examples |
| special | *laba kolmo* | `" "` | direct speech and literary title in the examples |
| special | *qaanso* | `()` | supplementary explanation and enumerators |

The table records what the maintained resource displays. Exact ordinary-output
characters, accepted quotation profiles, and spacing are resolved separately
in the pre-comment register rather than inferred from glyph appearance alone.

## Topic map

| ID | Topic | Principal evidence | Current reading | State |
| --- | --- | --- | --- | --- |
| P01 | Punctuation versus glottal stop | [`qoraal/05` lines 28–34](../../resources/qoraal/05-astaamaynta.md#hamsa-), SLS-0001 R6–R7 | *Hamsa* is a consonant letter. ASCII apostrophe can therefore be ambiguous on input and must not automatically be interpreted as punctuation. | inherited boundary |
| P02 | Mark inventory | [`qoraal/05` four groups](../../resources/qoraal/05-astaamaynta.md#afarta-kooxood-ee-astaanaha) | The source provides eleven punctuation items after excluding *hamsa*. It does not claim a complete modern inventory. | candidate baseline |
| P03 | Full stop | [`qoraal/05` joogsi](../../resources/qoraal/05-astaamaynta.md#astaan-joogsi-) | A full stop closes an ordinary complete sentence that is neither a question nor an exclamation. | strong candidate |
| P04 | Question mark | [`qoraal/05` weydiin](../../resources/qoraal/05-astaamaynta.md#astaan-weydiin-) | A question mark closes an interrogative sentence. Ordinary output attaches it to preceding text; source-faithful fields may preserve historical spacing. | resolved |
| P05 | Exclamation mark | [`qoraal/05` yaab](../../resources/qoraal/05-astaamaynta.md#astaan-yaab-) | An exclamation mark can close an exclamatory word or longer utterance. | strong candidate |
| P06 | Commas in series | [`qoraal/05` lines 100–150](../../resources/qoraal/05-astaamaynta.md#astaan-hakad-) | The comma separates coordinated nouns, verbs, modifiers, phrases, and clauses. A final serial comma is optional when grouping remains clear. | resolved |
| P07 | Vocatives and discourse words | [`qoraal/05` lines 152–166](../../resources/qoraal/05-astaamaynta.md#astaan-hakad-) | The source isolates interjections, responses, and some address-like or discourse material with commas. Several source sentences remain linguistically uncertain. | candidate class; examples need review |
| P08 | Introductory and inserted clauses | [`qoraal/05` lines 168–197](../../resources/qoraal/05-astaamaynta.md#astaan-hakad-), [`naxwe/14`](../../resources/naxwe/14-naxwaha-cusub.md#5-astaamaynta) | Introductory or genuinely inserted material can be separated, while a comma must not split tightly connected core sentence material merely by pause. | strong principle; grammar handoff needed |
| P09 | Semicolon | [`qoraal/05` dhibic hakad](../../resources/qoraal/05-astaamaynta.md#dhibic-hakad-) | The source describes relative pause length but supplies no distinct implementable conditions. The mark is recognized; placement is outside the first compliance surface. | resolved scope deferral |
| P10 | Colon | [`qoraal/05` laba dhibcood](../../resources/qoraal/05-astaamaynta.md#laba-dhibcood-) | The colon introduces a displayed list and can precede an explanation or emphasized completion. A time example is also given. | candidate functions; data formatting deferred |
| P11 | Long dash | [`qoraal/05` jiitin dheer](../../resources/qoraal/05-astaamaynta.md#jiitin-dheer-) | The source uses a long dash for interrupted delivery with spaces on both sides. Ordinary output uses U+2014 with one space on each side; alternative source glyphs remain source-faithful. | resolved profile |
| P12 | Quotation marks | [`qoraal/05` kolmo sections](../../resources/qoraal/05-astaamaynta.md#kolmo--), dictionary [*kolmo*](../../resources/qaamuus/15-k.md) | Direct and nested speech are supported. Curly, straight, and guillemet pairs are accepted document profiles; a document uses one consistently unless preserving a source. | resolved profiles |
| P13 | Parentheses | [`qoraal/05` qaanso](../../resources/qoraal/05-astaamaynta.md#qaanso-), dictionary [*qaanso*](../../resources/qaamuus/14-q.md) | Parentheses mark supplementary explanation and enclose enumeration labels. Square and curly bracket functions are scope-deferred. | parentheses resolved; other brackets deferred |
| P14 | Hyphen | [`qoraal/05` jiitin gaaban](../../resources/qoraal/05-astaamaynta.md#jiitin-gaaban--), SLS-0002 R10–R11 | Hyphenated compounds are attested, but SLS-0004 should govern punctuation rendering only after the spelling layer establishes that a lexical form is hyphenated. | ownership boundary |
| P15 | Spacing | Primary examples and literature usage | Ordinary output has no preceding space before closing punctuation, no inner space in paired marks, one following space where prose continues, no spaces around a compound hyphen, and one space around U+2014. | resolved profile |
| P16 | Additional modern marks | Literature usage and dictionary bracket terminology | Ellipses, square/curly brackets, slash, and combined marks occur or are named, but the primary chapter does not define them. They are excluded from v0.1 compliance and assigned to later proposals. | resolved scope deferral |

## Repository attestation pass

The first pass compared the primary chapter with grammar, dictionary, prosody,
and representative literature. Results are directional only because the
literature collection preserves original conventions and OCR damage.

| Area | Repository result | Interpretation |
| --- | --- | --- |
| Sentence endings | Question and exclamation marks are widespread in prose and poetry. Both attached forms (`tahay?`) and source-retained spaced forms (`tahay ?`) occur. | Function is strongly attested. Ordinary output follows attached spacing; labeled source text may preserve variation. |
| Direct speech | `qoraal/05` uses straight double quotes outside and straight single quotes inside. The dictionary displays curly quotation marks. `suugaan/21` frequently uses guillemets, while other literature uses ASCII or mixed quotes. | Direct-speech marking is established, but no single glyph system can be inferred by majority. |
| Commas | The grammar source explicitly contrasts incorrect commas between a noun and its predicate with unpunctuated sentences. The primary punctuation chapter gives lists, introductions, inserted material, and short coordinated clauses. | A syntax-sensitive comma principle is supportable; “write every spoken pause” is not. |
| Semicolon | Semicolons occur in resources, but the primary chapter supplies no unique positive examples and source punctuation may itself be inherited or damaged. | Attestation is insufficient for a normative semicolon rule. |
| Dash and hyphen | Hyphens and long dashes both occur, but Markdown headings and editorial formatting contaminate corpus counts. SLS-0002 already preserves reviewed closed or hyphenated compound spellings. | The explicit profile uses U+002D internally without spaces and U+2014 with surrounding spaces; compound classification is not duplicated. |
| Parentheses | Parenthetical explanations, dates, and stage-like notes are common. The dictionary names round, square, and curly varieties. | Round-parenthesis function is supported; the other paired marks need separate use cases. |

## Approved conflict resolutions

These audit recommendations were approved as provisional inputs to the initial
SLS-0004 `Draft`. They remain non-normative in this evidence map and did not by
themselves promote the standard; the later lifecycle handoff is recorded
below.

### C1 — Apostrophe-shaped letter versus punctuation

The 1977 source displays an ASCII apostrophe for *hamsa*, while SLS-0001
requires U+02BC MODIFIER LETTER APOSTROPHE as the canonical glottal-stop
letter and accepts U+0027/U+2019 as input aliases.

**Approved resolution:** inherit SLS-0001 without redefining the letter.
SLS-0004 should state that quotation parsing and normalization must not corrupt
an in-word U+02BC glottal stop. The exact quotation characters remain C4.

### C2 — Hyphen function versus compound spelling

The punctuation source assigns meaning-related compound use to the short dash,
while SLS-0002 says reviewed lexical evidence decides whether a compound is
closed or hyphenated.

**Approved resolution:** SLS-0002 owns the lexical decision; SLS-0004 owns
the hyphen character, placement, and any typography once a hyphenated spelling
has been approved. Source examples are evidence, not automatically canonical
headwords.

### C3 — Comma before a final conjunction

The source includes both a comma before the last conjunction and sequences
without one. The differences can reflect structure rather than a universal
serial-comma policy.

**Approved resolution:** standardize the supported separation functions but
do not mandate or ban a serial comma until examples are classified with
SLS-0003. A comma must not separate tightly connected core sentence elements.

### C4 — Quotation glyphs and nesting

Straight quotes, typographic curly quotes, and guillemets coexist. The primary
source demonstrates outer double and inner single quotation functions but does
not establish modern Unicode directionality or whether guillemets are an
accepted alternative.

**Approved resolution:** retain direct and nested quotation functions and
accept the three repository-attested paired profiles. Require consistency
within ordinary text without forcing source-faithful text into one profile.

### C5 — Semicolon conditions

The source defines the semicolon mostly by pause length and points to comma
examples. That is not sufficient for implementable placement rules.

**Approved resolution:** include the semicolon as a recognized mark but do
not add a **MUST** placement rule until distinct Somali examples and syntactic
conditions are reviewed.

### C6 — Long-dash character and spacing

The maintained source displays an em dash with surrounding spaces. It does not
establish whether U+2014 is mandatory, whether U+2013 or repeated hyphens are
input aliases, or whether every dash use requires spaces.

**Approved resolution:** preserve the interruption function and use U+2014
with one surrounding space on each side in ordinary output. Unicode character
identity supports distinguishing it from U+002D; source-faithful fields may
retain labeled variation.

### C7 — Punctuation next to quotation marks and parentheses

Source examples vary in whether a full stop or question mark appears inside or
outside the closing mark, and some literature is visibly OCR-damaged.

**Approved resolution:** punctuation placement follows semantic scope: a mark
belonging to quoted or parenthetical material goes inside; a mark belonging to
the containing sentence goes outside.

## Pre-comment resolution register

| ID | Question | Evidence needed before resolution |
| --- | --- | --- |
| Q1 | Which Unicode characters are canonical for single quotes, double quotes, hyphen, and long dash? | **Resolved:** fixed marks use the Draft's code-point table; U+002D is the compound hyphen and U+2014 the long dash. Quotation marks use one of the accepted paired profiles rather than a single mandatory profile. |
| Q2 | Are curly quotes and guillemets equivalent accepted profiles, or is one canonical? | **Resolved:** curly, straight, and guillemet pairs are accepted profiles. One document uses one profile consistently unless preserving source text. |
| Q3 | What are the rules for punctuation inside or outside closing quotation marks and parentheses? | **Resolved:** placement follows semantic scope; a mark belonging to quoted/parenthetical content goes inside, and a mark belonging to the containing sentence goes outside. |
| Q4 | When is a comma required, optional, or forbidden before a final conjunction? | **Resolved:** the final serial comma is optional when grouping stays clear; SLS-0003 can later define narrower syntactic classes. |
| Q5 | Which conditions uniquely require or permit a semicolon? | **Resolved scope deferral:** SLS-0004 v0.1 recognizes U+003B but adopts no placement compliance rule without distinct Somali examples. |
| Q6 | What spacing is canonical around each punctuation mark? | **Resolved:** ordinary spacing is specified in the Draft; source-faithful fields may preserve labeled variation. |
| Q7 | Which functions of ellipsis, square brackets, curly brackets, slash, and combined `?!` belong in the first standard? | **Resolved scope deferral:** none enters v0.1 compliance; later evidence-backed proposals may add them. |
| Q8 | How should punctuation interact with abbreviations, initials, decimals, dates, times, and grouped numbers? | **Resolved scope deferral:** these belong to future style/data-format standards. |
| Q9 | How should dialogue turns be represented? | **Resolved scope deferral:** inline paired quotation is supported; canonical dialogue-turn layout belongs to SLS-0407. |
| Q10 | What line-breaking behavior is required around hyphens, dashes, and paired marks? | **Resolved scope deferral:** core character identity is defined here; typography and line-breaking policy belong to SLS-0400 and implementation profiles. |

All ten questions now have written resolutions for SLS-0004 v0.1. Deferral to
a named owning standard is a scope decision, not an unassigned question.

## Drafting gates — satisfied 2026-08-23

The following gates were applied before SLS-0004 moved from this evidence map
to its first `Draft`:

- the maintainer reviewed the scope map and C1–C7 candidate resolutions;
- every candidate normative rule cites mapped evidence and discloses
  contrary or insufficient evidence;
- examples containing unresolved source wording are not used as canonical
  language examples merely because their punctuation is useful;
- rules inherited from SLS-0001 or SLS-0002 are referenced, not silently
  redefined;
- Unicode character and spacing choices are limited to the reviewed digital
  profile, with source-fidelity exceptions explicit;
- scope-deferred matters do not appear as compliance requirements.

## Approved first drafting slice

The approved first SLS-0004 `Draft` includes:

1. metadata, lifecycle fields, scope, and definitions;
2. the inherited glottal-stop distinction;
3. candidate functions for full stop, question mark, and exclamation mark;
4. syntax-sensitive comma principles supported by the primary and grammar
   sources;
5. candidate colon, direct-quotation, nested-quotation, and parenthesis
   functions;
6. an informative inventory for semicolon and scope-deferred additional marks,
   plus normative hyphen and long-dash identity;
7. a written resolution register carrying Q1–Q10.

The pre-comment Draft accepts three quotation profiles, recognizes but does not
govern semicolon placement, and defines the ordinary fixed-mark and spacing
profile without normalizing labeled source-fidelity fields.

## Lifecycle handoff

The founding maintainer, acting as interim Council under `GOVERNANCE.md`,
approved C1–C7 as provisional drafting inputs and sponsored the transition
from planned identifier to `Draft` on 2026-08-23. The later instruction to
complete Milestone 2 accepted the resolved Draft for `Proposed` publication.
The decisions are represented in
[`spec/orthography/0004-punctuation.md`](../../spec/orthography/0004-punctuation.md),
now at `status: Proposed`. Public comment begins when the proposal branch is
published; no requirement is ratified as stable policy. Q1–Q10 have written
v0.1 resolutions; a later evidence-backed proposal may reopen them through the
normal change process.
