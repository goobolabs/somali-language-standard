# SLS-0002 Orthography Evidence Map

- **Status:** reviewed evidence map; approved 2026-08-23 as input to the
  initial SLS-0002 `Draft` and retained for its `Proposed` review;
  non-normative
- **Prepared:** 2026-08-23
- **Target standard:** SLS-0002 Somali Orthography Standard
- **Normative dependency:**
  [`SLS-0001`](../../spec/orthography/0001-alphabet.md)

This document maps the completed `resources/` evidence baseline to the topics
that SLS-0002 must resolve. It is a research and decision aid, not a spelling
standard. A form appearing here is not approved merely because a historical
source or current resource records it.

## Evidence policy

Evidence is considered in this order:

1. **Inherited requirements** — SLS-0001 already fixes alphabet identity,
   digraph encoding, the glottal-stop character, vowel-length notation, and
   the basic representation of consonant gemination. SLS-0002 may refine
   spelling conditions but must not contradict that dependency.
2. **Direct orthography evidence** —
   [`resources/qoraal/`](../../resources/qoraal/) preserves *Habka Qoraalka*
   (1977), the primary Somali source for speech/writing distinctions,
   word-boundary proposals, and punctuation.
3. **Grammar and morphology evidence** —
   [`resources/naxwe/`](../../resources/naxwe/) and
   [`resources/sarfe/`](../../resources/sarfe/) show how morphemes, clitics,
   compounds, and sound changes surface in written forms.
4. **Supporting phonological evidence** —
   [`resources/dhawaaq/`](../../resources/dhawaaq/) explains sound contrasts
   relevant to spelling. It is descriptive phonology, not by itself authority
   for a normative orthographic choice.
5. **Usage validation** — dictionary, headword, and broader resource searches
   can establish which spellings are attested and how consistently they are
   used. Frequency alone does not override the sources or lifecycle review.

Conflicting, source-specific, or under-supported evidence remains an open
question. It must not be turned into a **MUST** rule without a written
resolution under SLS-0000.

## Scope boundary

### Intended for SLS-0002

- written-word boundaries and the treatment of contracted speech forms;
- when consonant doubling represents gemination;
- spelling outcomes of supported morphophonemic changes;
- spelling of vowel sequences and semivowels where SLS-0001 does not decide
  the surface form;
- the orthographic treatment of compounds;
- placement conditions for the glottal-stop letter, while inheriting its
  character encoding from SLS-0001;
- the orthography-side boundary between adapted and unadapted loanwords.

### Governed elsewhere

- alphabet inventory, collation, character encoding, and digraph identity:
  SLS-0001;
- punctuation: SLS-0004;
- capitalization: SLS-0005;
- grammatical analysis independent of spelling: SLS-0003;
- detailed loanword adoption and terminology policy: SLS-0103 and related
  lexicon standards;
- dialect-specific inventories and norms: the future SLS-0700 block;
- phonetic, tone, and stress notation: the future SLS-0600 block.

## Topic map

| ID | Topic | Principal evidence | Current reading | State |
| --- | --- | --- | --- | --- |
| E01 | Speech and writing | [`qoraal/01`](../../resources/qoraal/01-hadal-iyo-qoraal.md#hadalka-iyo-qoraalka-waa-kala-duwan), [`qoraal/02`](../../resources/qoraal/02-eray-kooban-hadalka.md#hordhac), [`naxwe/14` §4](../../resources/naxwe/14-naxwaha-cusub.md#4-hadal-iyo-qoraal) | Rapid speech can merge forms that writing may distinguish. Written segmentation requires grammatical and semantic analysis, not sound alone. | candidate principle |
| E02 | Expanded written forms | [`qoraal/02`](../../resources/qoraal/02-eray-kooban-hadalka.md#ereyada-laba-eray-ah) | The source proposes expansions such as *waxa aan*, *baa uu*, *aniga oo*, and *la isku*. The inventory is historical and does not authorize blanket expansion; reviewed surface spellings are preserved form by form. | resolved boundary |
| E03 | Bound clitics and conjunctions | [`naxwe/06` §6.2](../../resources/naxwe/06-sarfaha-iskuxireyaasha.md#62-xiriiriyeyaal), [`qoraal/02`](../../resources/qoraal/02-eray-kooban-hadalka.md#ereyada-laba-eray-ah) | Current grammar evidence writes *-na* and *-se* on a host, while the writing source expands examples such as *waqtigu na*. SLS-0002 governs those two clitics only; other attachment classes require SLS-0003 or lexical review. | resolved |
| E04 | Negative and interrogative segmentation | [`qoraal/04`](../../resources/qoraal/04-kala-qoridda-lama-qasban.md), [`naxwe/12` §12.3](../../resources/naxwe/12-noocyada-weeraha.md#123-weer-diidmo) | The source presents dialect-dependent *aan* combinations and unsettled question analyses. SLS-0002 preserves named reviewed surface forms; SLS-0003 owns the complete paradigm. | resolved scope handoff |
| E05 | Compounds and hyphens | [`naxwe/01` §1.1.5](../../resources/naxwe/01-ereyada.md#115-lammaanin-compounding), [`naxwe/15` §7.3](../../resources/naxwe/15-naxwaha-sifayneed.md#73-lammaanin), [`qoraal/05`](../../resources/qoraal/05-astaamaynta.md#jiitin-gaaban--) | Grammar resources show closed and hyphenated compounds. Established lexemes preserve reviewed spelling; no productive global join/split/hyphen rule is adopted. | resolved |
| E06 | Glottal stop | [`SLS-0001` R6–R7](../../spec/orthography/0001-alphabet.md#glottal-stop), [`qoraal/05`](../../resources/qoraal/05-astaamaynta.md#hamsa-) | The sign is a consonant letter, not punctuation; U+02BC is canonical. Placement is lexical or morphologically reviewed; no automatic position-based insertion or deletion rule is adopted. | resolved boundary |
| E07 | Vowel length | [`SLS-0001` R8](../../spec/orthography/0001-alphabet.md#vowel-length), [`naxwe/01` §1.2.1](../../resources/naxwe/01-ereyada.md#121-shaqallada), [`dhawaaq/04`](../../resources/dhawaaq/04-shaqaallada.md#shaqaallada-dhaadheer) | Long vowels are written by doubling. Minimal pairs confirm that length can distinguish words. SLS-0002 should reference this rule rather than redefine the alphabet-level requirement. | inherited |
| E08 | Consonant gemination | [`SLS-0001` R8](../../spec/orthography/0001-alphabet.md#vowel-length), [`dhawaaq/03`](../../resources/dhawaaq/03-shibbanayaasha.md#shibbanayaasha-labanlaabnan-geminates), [`sarfe/04`](../../resources/sarfe/04-isbeddelka-codka.md) | SLS-0001 and morphology support doubled reviewed geminates, including *d*. The isolated historical single-*d* recommendation does not override the dependency. | resolved |
| E09 | Morphophonemic spelling | [`sarfe/04`](../../resources/sarfe/04-isbeddelka-codka.md), [`naxwe/01` §1.2.3](../../resources/naxwe/01-ereyada.md#123-shibbanayaasha), [`naxwe/15` §6](../../resources/naxwe/15-naxwaha-sifayneed.md#6-codbeddelidda-dhismaha-ereyga) | Supported alternations include *qaad + tay → qaadday*, *badi + tay → badisay*, *gal + tay → gashay*, *buug + ka → buugga*, and *bil + ta → bisha*. These demonstrate surface spelling but do not yet establish a complete productive rule set. | candidate rules, limited scope |
| E10 | Digraphs at morpheme boundaries | [`SLS-0001` edge cases](../../spec/orthography/0001-alphabet.md#edge-cases) | `sh`, `dh`, and `kh` are single letters by default, but a true two-segment sequence can arise across a reviewed boundary. No special separator is invented; reviewed spelling and morphology govern the form. | resolved boundary |
| E11 | Vowel sequences, *y*, and *w* | [`dhawaaq/04`](../../resources/dhawaaq/04-shaqaallada.md#shaqaallada-lammaan-diphthongs), [`naxwe/17` §1](../../resources/naxwe/17-naxwaha-af-soomaaliga.md#1-cod-xaraf-alan-iyo-erey) | Phonological analyses preserve competing interpretations. SLS-0002 therefore preserves reviewed surface spelling and prohibits an automatic vowel↔*y/w* rewrite. | resolved boundary |
| E12 | Tone and vowel quality | [`naxwe/01` §1.2.1–1.2.2](../../resources/naxwe/01-ereyada.md#121-shaqallada), [`dhawaaq/05`](../../resources/dhawaaq/05-codadka-sare.md) | Tone and the heavy/light vowel-quality distinction can differentiate readings but are normally unmarked in ordinary Somali orthography. SLS-0002 should state the non-marking boundary without creating a phonetic notation system. | candidate boundary |
| E13 | Loanword spelling | [`SLS-0001` R11](../../spec/orthography/0001-alphabet.md#excluded-letters), [`naxwe/15` §7.5](../../resources/naxwe/15-naxwaha-sifayneed.md#75-erey-amaah-iyo-soomaaliyeyn), [`naxwe/17` §2.6](../../resources/naxwe/17-naxwaha-af-soomaaliga.md#26-magacyo-amaah-ah) | Established adapted loans follow Somali characters. A complete letter-by-letter algorithm is explicitly assigned to SLS-0103 rather than left open in SLS-0002. | resolved scope handoff |
| E14 | Dialect variation | [`naxwe/15` §8](../../resources/naxwe/15-naxwaha-sifayneed.md#8-lahjadaha-iyo-qoraalka), [`dhawaaq/06`](../../resources/dhawaaq/06-lahjadaha.md) | A documented dialect form is not an error. Choosing a general SLS spelling requires explicit scope and review; source variation must not be erased as OCR noise. | governance boundary |

## Repository attestation pass

The first attestation pass searched all files under `resources/`. Counts are
directional evidence only: collections can repeat the same source, and some
literature retains source-specific or OCR-damaged spellings. They are not
balanced-corpus frequencies and cannot decide a rule by majority vote.

| Area | Repository result | Interpretation |
| --- | --- | --- |
| Bound *-na/-se* | The dictionary marks *na²* as `(-na)` and illustrates attachment; *se* is illustrated by *aniguse* and combined forms such as *laakiinse*. Grammar §6.2 says neither form stands independently in this function. Literature supplies repeated *waana, adiguna, kumase, Xasanse,* and *laakiinse*. | Strong support for attachment of these two conjunction clitics; contrary expanded rows in `qoraal/02` cannot be generalized. |
| Negative and question forms | The dictionary analyzes *maxaad* as *maxaa+aad* and *maxay* as *maxaa+ay*. Resources contain hundreds of *maxaa, maxay, maxaad, aanan, aanu,* and *aysan* tokens. Grammar records bound *-aan* in focus constructions and separate *ma* before a verb. | Fused forms are established orthographic forms, but morphology determines where fusion applies. A full paradigm belongs to SLS-0003. |
| Compounds | Exact forms vary: *afmiinshaar* is closed, *xeer-ilaaliye* is hyphenated, and the dictionary contains more than 1,300 hyphenated headword records. Forms such as *madaxbannaan*, *madax-bannaan,* and *madax bannaan* all occur in the evidence library. | Both closed and hyphenated compounds are established. The current evidence does not support one productive rule for every compound. |
| Gemination | SLS-0001 requires doubling; grammar and morphology repeatedly give *qaadday, badda, buugga,* and *doofaarro*. The dictionary's *-ta* entry explicitly gives *bad+ta → badda*. | The dependency and converging morphological evidence outweigh the isolated historical recommendation to suppress doubled *d*. |

## Conflicts and provisional resolutions

### C1 — Separate words versus bound clitics

*Habka Qoraalka* strongly prefers expanded written forms, but the grammar
resources treat at least *-na* and *-se* as bound elements and record other
clitic combinations. SLS-0002 needs a classification by element, not a blanket
rule derived from either source alone.

**Provisional resolution:** write conjunction clitics *-na* and *-se* on their
host. Do not derive a universal splitting rule from the expanded forms in
`qoraal/02`. Other focus, subject, object, and preposition combinations are
assigned form by form to SLS-0003 or reviewed lexical records.

### C2 — Negative and question forms

The writing source records dialect-dependent negative combinations and calls
some interrogative expansions unsettled. Before a rule is drafted, each
proposed form needs a morphology check and an attestation scan across the
dictionary and literature collections.

**Provisional resolution:** preserve dictionary- and grammar-supported fused
forms such as *maxaa, maxay, maxaad,* and the applicable *-aan* combinations;
do not mandate speculative expansions such as a universal *maxaa + pronoun*
spelling. Keep free preverbal *ma* separate. SLS-0003 owns the complete
negative and interrogative paradigms; SLS-0002 records only their reviewed
surface spellings.

### C3 — Closed compound versus hyphen

The grammar layer records many compounds as one orthographic word, while the
punctuation source assigns the hyphen a compound-forming role. A resolution
must distinguish lexicalized compounds, productive formations, transparent
phrases, and cases where a hyphen prevents misreading.

**Provisional resolution:** use the attested dictionary headword spelling for
an established lexeme in SLS examples, whether closed or hyphenated. Do not
generate a new compound spelling by analogy alone. Defer a productive
closed-versus-hyphenated rule until compound classes are reviewed; coordinate
the function and rendering of the hyphen with SLS-0004.

### C4 — Written *d* gemination

SLS-0001 and the morphology evidence support doubled consonants, including
*qaadday*. The phonology source recommends writing a particular single and
geminate *d* contrast alike. SLS-0002 must resolve whether that recommendation
is historical practice, a restricted exception, or evidence superseded by the
current dependency.

**Provisional resolution:** inherit SLS-0001 R8 and write phonemic or
morphologically derived gemination by doubling, including *d*. Treat the
single-*d* recommendation as historical descriptive evidence that does not
override the normative dependency. This resolution does not authorize changing
an individual word without morphological or lexical evidence.

## Pre-comment resolution register

| ID | Question | Evidence needed before resolution | Status after first pass |
| --- | --- | --- | --- |
| Q1 | Which clitics and particles are obligatorily bound, optionally bound, or separate? | **Resolved for SLS-0002:** *-na/-se* are governed here; every other class requires an explicit grammar or lexical decision under SLS-0003. No analogy-based attachment is permitted. |
| Q2 | Which contraction expansions are normative in formal writing? | **Resolved:** no blanket expansion rule. Preserve a reviewed surface form; form-specific additions require recorded review. |
| Q3 | How should negative *aan* combinations and *maxaa/maxay* question forms be segmented? | **Resolved scope handoff:** preserve the reviewed fused forms named in the Draft; SLS-0003 owns the complete paradigm and conditions. |
| Q4 | When is a compound closed, hyphenated, or spaced? | **Resolved:** established lexemes preserve their reviewed lexical spelling. New or unresolved compounds are not standardized by a productive global transformation. |
| Q5 | Are any consonants restricted or exceptional when gemination is written? | **Resolved for representation:** every reviewed geminate is doubled, including *d*. The lexicon or morphology supplies word-specific evidence; no consonant exception is adopted. |
| Q6 | How are genuine `s+h`, `d+h`, or `k+h` sequences distinguished from digraphs across boundaries? | **Resolved:** no special separator is invented. Preserve the reviewed surface spelling and morphological boundary metadata; tools must not insert punctuation to force a parse. |
| Q7 | Which vowel sequences are written with *y/w*, and which remain adjacent vowels? | **Resolved:** preserve reviewed lexical or morphological surface spelling; no automatic vowel↔*y/w* conversion is standardized. |
| Q8 | What minimum loanword rules belong in SLS-0002 before SLS-0103 exists? | **Resolved:** adapted loans obey SLS-0001 characters; unadapted labeled material follows SLS-0001 R11; detailed mappings belong exclusively to SLS-0103. |

All eight pre-comment questions now have written resolutions for the scope of
SLS-0002. A future proposal can reopen a decision with stronger evidence, but
none remains an unassigned Milestone 2 question.

## Drafting gates — satisfied 2026-08-23

The following gates were applied before SLS-0002 moved from the evidence map
to its first `Draft` and were rechecked during pre-comment closeout:

- every proposed rule cites at least one mapped source and identifies any
  contrary evidence;
- every example is checked against the resource baseline and labeled as
  source-attested, repository-supported, or editorial;
- C1–C4 and Q1–Q8 have written maintainer-approved resolutions;
- rules inherited from SLS-0001 are referenced rather than duplicated with
  different wording;
- punctuation, capitalization, grammar, dialect, and loanword matters are
  handed to their owning standards when they exceed SLS-0002's scope;
- no unassigned pre-comment question appears in the compliance checklist.

## Approved drafting slice

The SLS-0002 `Draft` includes:

1. metadata and lifecycle fields from `standards/TEMPLATE.md`;
2. purpose, scope, definitions, and explicit ownership boundaries;
3. inherited references to SLS-0001 for alphabet, glottal-stop encoding, vowel
   length, digraphs, and the general representation of gemination;
4. informative summaries of the word-boundary and morphophonemic evidence;
5. the written resolution register for Q1–Q8.

Normative word-boundary, compound, gemination, and loanword boundaries now
reflect those resolutions without claiming a complete grammar or lexicon.

## Lifecycle handoff

The founding maintainer, acting as interim Council under `GOVERNANCE.md`,
approved C1–C4 as provisional drafting inputs and sponsored the transition
from planned identifier to `Draft` on 2026-08-23. The later instruction to
complete Milestone 2 accepted the resolved Draft for `Proposed` publication.
The decisions are represented in
[`spec/orthography/0002-spelling-rules.md`](../../spec/orthography/0002-spelling-rules.md),
now at `status: Proposed`. Public comment begins when the proposal branch is
published; no requirement is ratified as stable policy.
