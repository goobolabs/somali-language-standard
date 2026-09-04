# SLS-0100 Dictionary Evidence Map

- **Status:** initial evidence audit for the SLS-0100 Draft; non-normative
- **Prepared:** 2026-09-04
- **Target standard:** SLS-0100 Dictionary Standard
- **Draft venue:** [Milestone 4 issue #24](https://github.com/goobolabs/somali-language-standard/issues/24)
- **Normative dependencies:**
  [SLS-0001](../../spec/orthography/0001-alphabet.md),
  [SLS-0002](../../spec/orthography/0002-spelling-rules.md), and
  [SLS-0003](../../spec/grammar/0018-somali-grammar-standard.md)

This map establishes what the existing repository can and cannot support when
curating `data/lexicon/core/`. It does not approve an entry. A source spelling,
code, or definition becomes SLS data only after the individual record has a
written review outcome.

## Evidence policy

Evidence is considered in this order:

1. **Normative dependencies.** SLS-0001 and SLS-0002 govern characters and
   canonical spelling; SLS-0003 governs primary word classes, reviewed noun
   gender, and the prohibition on guessed plurals.
2. **Principal dictionary evidence.** [`resources/qaamuus/`](../../resources/qaamuus/)
   contains 48,119 cleaned headword entries with grammatical codes,
   definitions, examples, and `ld`/`eeg` cross-references.
3. **Derived headword cross-check.**
   [`resources/madax-ereyo/`](../../resources/madax-ereyo/) contains 47,502
   short-vowel-file headwords derived from the dictionary. It can detect a
   transcription mismatch but is not an independent lexical authority.
4. **Morphology and grammar evidence.**
   [`resources/sarfe/`](../../resources/sarfe/) and
   [`resources/naxwe/`](../../resources/naxwe/) provide reviewed paradigms and
   grammatical constraints. They support a listed form; they do not license a
   plural generated for an unlisted noun.
5. **Maintainer lexical review.** The maintainer decides whether the proposed
   headword, word class, meanings, morphology, variety, and newly written
   bilingual definitions represent Standard Somali. Unanswered fields remain
   draft or stay outside the dataset.

Repository frequency, spelling resemblance, or structural schema validity is
never enough to settle a lexical fact by itself.

## Source limitations that affect the lexicon

- The compiler/editor, edition, publisher, and republication rights of
  *Qaamuuska Af-Soomaaliga* remain unconfirmed in the resource metadata queue.
  The source therefore supports factual checking, but its definition prose
  must not be silently republished as new CC BY 4.0 project text.
- The dictionary was cleaned at collection scale, but the 48,119 entries have
  not been checked one by one against page scans.
- Dictionary categories are richer and sometimes differently organized than
  SLS-0003's nine primary word classes.
- Noun parentheses can contain one plural, several plurals, an invariant form,
  plural gender, or mixed alternatives. Some nouns list no plural.
- `ld` means a related variant and `eeg` directs the reader elsewhere; neither
  token alone proves synonymy, canonical preference, or sense identity.
- Superscript digits distinguish source homographs. They are editorial labels,
  not Somali headword characters.
- The source does not systematically provide loanword origin, IPA, dialect
  tags, usage frequency, English definitions, or separately citable example
  sentence IDs.

## Field audit

| SLS field | Available evidence | Conversion boundary | State |
|---|---|---|---|
| `sls_id` | No source equivalent | Assign sequentially during review; never derive from source order or renumber later | resolved |
| `word` | Dictionary bold headword; derived headword list | Remove source-only homograph superscripts; preserve reviewed spelling; do not normalize silently | pilot review required |
| `part_of_speech` | Dictionary grammatical codes | Map only an unambiguous code to the nine SLS-0003 classes; hold mixed or source-specific classes for review | pilot review required |
| `gender` | `m.l`, `m.dh`, combined labels, agreement evidence | Record only a reviewed value; combined or missing labels are not guessed | pilot review required |
| `plural` | Parenthetical forms and morphology tables | Record an attested form; use JSON `null` only for a reviewed absence of an ordinary plural | schema correction required |
| `dialect` | Usually not explicitly labeled | Core entries require maintainer confirmation that `so` is appropriate | pilot review required |
| `definitions[].so_gloss` | Somali definition prose | Write and review a concise project gloss; do not copy rights-unconfirmed prose by default | pilot review required |
| `definitions[].en` | Not supplied systematically | Write a matching English definition and review it against the same sense | pilot review required |
| `synonyms` / `antonyms` | `ld`, `eeg`, and definition prose | Do not convert automatically; defer full relation policy to SLS-0102 | deferred |
| `is_loanword` / `loan_origin` | Sparse domain or origin clues | Allow explicit unresolved `null`; require evidence before `true`, `false`, or an origin claim | schema correction required |
| `ipa` | Not systematically supplied | Omit pending a future phonology method or source | deferred |
| `example_sentences` | Examples embedded in some definitions | Omit until sentences receive independent `sls:sent:` records and provenance | deferred |
| `frequency_rank` | No balanced frequency corpus | Omit pending SLS-0105 and a documented corpus method | deferred |
| `metadata` | Collection registries and correction log | Name the exact source file/headword plus the origin of new glosses; record reviewer at review | resolved policy |

## Primary-class mapping boundary

The following mappings are safe only when the source code actually describes
the entry's primary use. Additional suffixes and mixed codes still require
entry-level review.

| Source code | Source label | Candidate SLS-0003 class | Boundary |
|---|---|---|---|
| `m` | magac | `magac` | Gender and number codes remain separate facts. |
| `mu` | magacuyaal | `magacuyaal` | Subtype codes do not create another primary class. |
| `f` | fal | `fal` | Conjugation and transitivity belong to later morphology work. |
| `fk` | falkaab | `falkaab` | Context-sensitive uses remain separate lexemes or senses after review. |
| `h` | horyaale | `meeleeye` | Use the reviewed modern SLS label. |
| `xi` | xiriiriye | `xiriiriye` | Bound versus free behaviour still follows SLS-0002/SLS-0003. |
| `q` | qodob | `qodob` | Do not confuse with the source's separate `qr` particle code. |
| `s`, `ti`, `t` | sifo, tilmaame, tiraale | `tilmaame` candidate | SLS-0003 groups the modifier domain, but each lexical use still needs review. |
| `e.d` | erey dareen | `yaab` candidate | Not automatic; ideophones and interjections can require a context decision. |
| `qr` | qurub | no automatic mapping | The nine-class system has no one-to-one general particle class. |

## Schema findings

### S1 — ordinary-plural absence cannot be represented

The v1.0.0 lexicon schema requires every noun to contain a non-empty plural
string. SLS-0003 G11-R4 separately recognizes mass and collective readings,
and the dictionary contains nouns for which no ordinary plural is listed.
Using an invented string would violate G11-R3.

**Draft resolution:** expand `plural` to accept either a non-empty attested form
or JSON `null`. `null` means a reviewer confirmed that an ordinary plural is not
applicable to the recorded reading; it never means “not checked.” Keep the
field required for every noun.

### S2 — binary loanword status forces an unsupported claim

The v1.0.0 schema requires `is_loanword` to be Boolean, while the principal
dictionary usually provides no etymology. Defaulting to `false` would turn
missing evidence into a lexical claim.

**Draft resolution:** expand `is_loanword` to `true`, `false`, or JSON `null`.
`null` means unresolved and is not interpreted as `false`. `loan_origin`
remains required only when the status is `true`.

Both changes accept every v1.0.0 record and add explicit representations, so
the proposed schema version is 1.1.0.

## Pilot sample

The first review packet deliberately includes easy and difficult cases:

| Candidate | What it tests |
|---|---|
| `baabuur` | clear noun gender/plural; loanword status absent |
| `buug` | several source-listed plural alternatives and genders |
| `naag` | feminine singular with listed plural |
| `nin` | masculine singular and source-listed plural behaviour |
| `mindi` | noun morphology plus `ld middi` relationship |
| `qalin` | several senses but no listed plural |
| `sonkor` / `sokor` | mass reading and cross-reference direction |
| `biyo¹` / `biyo²` | plural/collective shape and general versus technical homographs |
| `guri¹` / `guri²` | noun and verb homographs |
| `hooyo¹` / `hooyo²` | cross-reference-only versus independent entry |
| `kab¹` / `kab²` / `kab³` | three word-class and sense distinctions under one spelling |
| `macallin` | multiple plurals and a source definition that is too narrow for direct reuse |

The detailed questions are in
[`SLS-0100-reviewer-packet.md`](SLS-0100-reviewer-packet.md). Batch 1 is now
assigned IDs `sls:lex:000001`–`sls:lex:000004` and counted as four reviewed
records. Batch 2 is now assigned IDs `sls:lex:000005`–`sls:lex:000008`, bringing
the reviewed pilot total to eight records. Batch 3 is now assigned IDs
`sls:lex:000009`–`sls:lex:000014`, bringing the reviewed pilot total to fourteen
records. The specialized splint noun remains deferred.

## Draft decisions

| ID | Decision | Reason |
|---|---|---|
| D1 | Begin the first real ID at `sls:lex:000001` only after its full record is approved. | IDs are permanent; candidates should not consume them before review. |
| D2 | Keep unresolved candidates in the review packet, not in `data/lexicon/core/`. | The Milestone 4 dataset is intended to be reviewed, not a staging dump. |
| D3 | Partition approved records by Somali initial letter/digraph. | Stable files such as `b.jsonl`, `dh.jsonl`, and `sh.jsonl` align with the source and keep diffs reviewable. |
| D4 | Treat the dictionary and headword collection as evidence, not as text to relicense wholesale. | Source metadata and rights remain incomplete. |
| D5 | Write concise new Somali glosses and matching English definitions, then obtain maintainer approval. | The schema requires both languages, while the source only supplies Somali prose. |
| D6 | Do not populate IPA, frequency, sentence references, synonyms, or antonyms in the first pilot. | Their evidence and owning standards are not ready. |

## Gates before the first data record

- the schema can represent a reviewed absence of an ordinary plural;
- the schema can represent unresolved loanword status without treating it as
  non-loanword;
- the maintainer answers the pilot's lexical questions and approves the exact
  bilingual definitions;
- each approved record identifies its exact source and reviewer;
- the assigned ID is the next unused permanent lexicon ID;
- the JSONL file passes local repository validation.

Batch 1 evidence outcome: the four reviewed records are in
`data/lexicon/core/batch-1.jsonl`; `baabuur` has reviewed Arabic/Italian loan
origin, `naag` and `nin` have reviewed non-loanword status, and `mindi` records
`middi` as a reviewed variant.

## Lifecycle handoff

Issue #24 sponsors the planned-to-`Draft` transition. The evidence audit and
formal document meet the minimum Draft template gate on 2026-09-04. SLS-0100
does not enter `Proposed` until a maintainer accepts the resolved pilot policy
and explicitly opens its ≥14-day public-comment period.
