# SLS-0005 Capitalization Evidence Map

- **Status:** reviewed evidence map; approved 2026-08-23 as input to the
  initial SLS-0005 `Draft` and retained for its `Proposed` review;
  non-normative
- **Prepared:** 2026-08-23
- **Target standard:** SLS-0005 Somali Capitalization Standard
- **Normative dependency:** SLS-0002, which depends on SLS-0001

This document maps the available capitalization evidence to the deliberately
narrow first scope of SLS-0005. It is not a capitalization standard. The
founding maintainer's instruction to complete Milestone 2 sponsors this map and
its bounded drafting decisions; it does not make the supporting source a
Somali-primary authority.

## Evidence policy and limitation

1. **Inherited character rules** — SLS-0001 defines case pairs and requires a
   capitalized digraph to uppercase only its first character. SLS-0005 must
   inherit, not redefine, those forms.
2. **Direct capitalization evidence** —
   [`resources/qoraal/06-xarafka-weyn.md`](../../resources/qoraal/06-xarafka-weyn.md)
   is an interim supplement curated from Morgan Nilsson's *Beginner's Somali
   Grammar* §2.3 (2024 preliminary).
3. **Primary-source gap** — *Habka Qoraalka* (1977) does not define
   capitalization, and the sampled 1974 Ministry publication documents
   literacy history rather than capitalization rules. The limitation is
   recorded in
   [`resources/qoraal/00-sources.md`](../../resources/qoraal/00-sources.md#coverage-notes).
4. **Grammar support** — the `naxwe/` collection distinguishes common and
   proper nouns and supplies proper-name examples, but does not independently
   define a complete capitalization system.
5. **Usage validation** — literature can attest forms, but preserves source
   style and OCR variation. Frequency cannot supply missing authority.

Because the direct evidence is supplementary, the first standard must use a
small compliance surface and preserve the source limitation through public
review.

## Scope boundary

### Intended for the first SLS-0005 draft

- capitalization at the beginning of an independent sentence;
- personal, geographic, and other reviewed proper names;
- conventional capitalization of weekday and month names;
- *Soomaali* as an ethnonym and language name, including lowercase generic
  *af* in *af Soomaali*;
- lowercase common uses of season and direction terms;
- inherited capitalization of Somali digraphs.

### Deferred or governed elsewhere

- heading and title case, publication titles, and house style: SLS-0400 block;
- official institutional names and government naming conventions: SLS-0403;
- acronym and abbreviation formation: future style/lexicon standards;
- all-capital emphasis and display typography: style standards;
- capitalization after punctuation whose sentence status is unresolved:
  coordination with SLS-0004;
- foreign-name adaptation: SLS-0103;
- product, trademark, username, and source-faithful casing: later style and
  data-profile rules.

## Topic map

| ID | Topic | Evidence | Current reading | State |
| --- | --- | --- | --- | --- |
| K01 | Sentence beginning | [`qoraal/06` §Bilowga weedha](../../resources/qoraal/06-xarafka-weyn.md#bilowga-weedha) | An independent sentence begins with a capital letter. | strong within supplement |
| K02 | Proper names | [`qoraal/06` §Magacyada gaarka ah](../../resources/qoraal/06-xarafka-weyn.md#magacyada-gaarka-ah), [`naxwe/13` §4.1](../../resources/naxwe/13-aasaaska-naxwaha.md#41-magac-guud-iyo-magac-gaar) | Personal and geographic proper names begin with capitals. | supported |
| K03 | Multiword proper names | *Sayid Maxamed* and other repository examples | More than one proper-name component can be capitalized, but particles and internal generic words lack a complete rule. | partial |
| K04 | Weekdays | [`qoraal/06`](../../resources/qoraal/06-xarafka-weyn.md#maalmaha-toddobaadka-iyo-bilaha) | Weekday names are conventionally capitalized in the supplement. | candidate with limitation |
| K05 | Months | same section | Month names are conventionally capitalized in the supplement. | candidate with limitation |
| K06 | Ethnonym | [`qoraal/06` §Qowmiyadaha](../../resources/qoraal/06-xarafka-weyn.md#qowmiyadaha-iyo-luqadaha) | *Soomaali* is capitalized when naming the people. | candidate |
| K07 | Language construction | same section | The supplement writes *af Soomaali*: generic *af* lowercase, language name capitalized. | candidate |
| K08 | Seasons | [`qoraal/06` §Xilliyada](../../resources/qoraal/06-xarafka-weyn.md#xilliyada-sannadka) | Uppercase and lowercase uses are shown, but “used as a name” is not an implementable class by itself. | resolved by proper/common distinction |
| K09 | Directions | [`qoraal/06` §Jihada](../../resources/qoraal/06-xarafka-weyn.md#jihada-afarta) | Uppercase and lowercase uses are shown; ordinary directional uses behave as common nouns. | resolved by proper/common distinction |
| K10 | Digraph case | SLS-0001 R12 | *Sh, Dh,* and *Kh* uppercase only their first character in ordinary capitalization. | inherited |
| K11 | Punctuation interaction | SLS-0004 R3–R5 | Sentence-final punctuation can establish the next independent sentence; colon, semicolon, dash, and quotation cases need narrower analysis. | partial |
| K12 | Titles, institutions, acronyms, all caps | primary-source gap | Repository usage exists, but the capitalization chapter supplies no complete rules. | deferred |

## Decisions approved for the initial draft

### D1 — Source limitation

SLS-0005 will identify the Nilsson-derived chapter as supplementary evidence
and remain conservative. Absence of a Somali-primary capitalization chapter
is a documented limitation, not permission to copy English rules wholesale.

### D2 — Proper versus common use

Reviewed proper names receive initial capitals. Common nouns remain lowercase
unless another rule, such as sentence-initial position, applies. This
distinction supplies the narrow interpretation for seasons and directions:
ordinary *jiilaal, koonfur, waqooyi, bari,* and *galbeed* are lowercase; a
reviewed proper name can capitalize the same lexical material.

### D3 — Calendar names

Weekday and month names follow the explicit supplementary examples and are
capitalized. This rule is provisional pending Somali-primary review.

### D4 — Ethnonym and language name

*Soomaali* is capitalized as an ethnonym or language name. In the ordinary
construction *af Soomaali*, generic *af* remains lowercase unless it begins a
sentence or belongs to a reviewed full proper title.

### D5 — Multiword names

Each component established as part of a reviewed proper name is capitalized.
No universal title-case rule is inferred for particles, connectors, generic
descriptors, headings, or publication titles.

### D6 — Punctuation boundary

The first cased letter of a new independent sentence is capitalized, including
an independent sentence inside a quotation. A colon, semicolon, comma, or dash
does not automatically trigger capitalization; sentence structure decides.

### D7 — Acronyms and display capitals

The initial standard does not generate acronyms or mandate all-capital display
text. A reviewed acronym or source-faithful form may preserve its registered
casing under the standard that owns it.

## Resolution register

| Question | Written resolution for Milestone 2 |
| --- | --- |
| Which seasons and directions are capitalized? | Lowercase in ordinary common use; capitalize only sentence-initially or inside a reviewed proper name. |
| How are multiword names handled? | Capitalize only components established as proper-name components; no universal title case. |
| Are official institutions covered? | Deferred to SLS-0403; reviewed proper components still follow the core rule. |
| Are headings and publication titles covered? | Deferred to the SLS-0400 style block. |
| Are acronyms and abbreviations covered? | Formation deferred; reviewed forms preserve registered casing. |
| Does punctuation automatically trigger a capital? | Only a new independent sentence triggers the core rule. |
| Is all-capital emphasis standardized? | No; deferred to style and accessibility guidance. |
| Is the evidence sufficient for a broad rule “like English”? | No. The phrase is rejected as a normative rule. |

## Drafting gates — satisfied 2026-08-23

- the supplementary-source limitation is disclosed in the map and Draft;
- every candidate rule is restricted to an attested or inherited class;
- English capitalization is not imported by analogy;
- unresolved style, institutional, acronym, and typography matters are
  assigned to named future standards rather than silently standardized;
- SLS-0001 digraph capitalization is referenced rather than rewritten
  differently;
- the founding maintainer sponsored the planned-to-`Draft` transition under
  the interim-Council provision in `GOVERNANCE.md`.

## Lifecycle handoff

The approved decisions are represented in
[`spec/orthography/0003-capitalization.md`](../../spec/orthography/0003-capitalization.md),
now at `status: Proposed` after the founding maintainer accepted the resolved
Draft for Milestone 2 publication. Public comment begins when the proposal
branch is published. The transition does not remove the primary-source
limitation or ratify any requirement as stable policy.
