# Audit record — Ereyada (qaamuuska)

- **Resource path:** `resources/naxwe/01-ereyada.md`
- **Collection / family:** naxwe / primary grammar
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-12
- **File size at audit start:** 301 lines; 1,990 words; 11,757 bytes
- **Resource SHA-256 at audit start:** `8c5ee7e0d8ec63d278d47c11ceda3dcaa3dde7135be46901f0f1740b9c1cdb69`
- **Resource-text changes during audit:** none

## Target output model

This file is already a coherent primary-grammar chapter rather than a raw OCR
transcript. It has one H1, two H2 sections, nine H3 subsections, six valid
two-column tables, and 24 numbered examples. It contains no title-page matter,
exercises, scan debris, false headings, or damaged reading order.

Cleanup should preserve its two-part progression from word formation and
lexical relations to sounds and writing. It should retain the derivation,
semantic, compounding, vowel, codkac, and consonant examples wherever their
reading is secure; link detailed morphology, phonology, and orthography
resources; correct only uniquely supported forms; distinguish the normative
SLS alphabet from the source's count of written sound sequences; and qualify
historical or universal claims that the repository does not support as written.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-16 | reviewed | N01-R001 |
| 17-56 | reviewed | N01-R002 |
| 57-68 | reviewed | N01-R003 |
| 69-105 | reviewed | N01-R004 |
| 106-125 | reviewed | N01-R005 |
| 126-159 | reviewed | N01-R006 |
| 160-195 | reviewed | N01-R007 |
| 196-211 | reviewed | N01-R008 |
| 212-240 | reviewed | N01-R009 |
| 241-265 | reviewed | N01-R010 |
| 266-284 | reviewed | N01-R011 |
| 285-291 | reviewed | N01-R012 |
| 292-301 | reviewed | N01-R013 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| N01-R001 | 1-16 | Clean introduction comparing words to building materials and distinguishing `qaamuus` from `weereyn`; no OCR or structural damage / low | `resources/naxwe/ereyfur.md`; `resources/naxwe/09-weer-fudud.md`; `resources/qaamuus/` | Retain the title, analogy, and distinction. Add concise navigation to the syntax chapters without expanding the introduction. | `repository-supported`; `structural-only` |
| N01-R002 | 17-56 | Clear derivation section and valid table; the broad one-word/one-referent statement at lines 25-30 conflicts with the chapter's own polysemy discussion; `fardoole` occurs three times but the dictionary and wordlist record only collective `fardooley`, so a silent normalization would erase a plausible singular/source distinction / medium | Same file lines 57-68; `resources/sarfe/03-dhismaha-ereyga.md`; `resources/qaamuus/13-f.md` entries `fardooley`; `resources/madax-ereyo/13-f.md`; repository-wide search finds `fardoole` only here | Retain the derivation topic, table, suffix definitions, and examples. Qualify the one-to-one lexical claim so it does not contradict polysemy. Keep `fardoole` as an explicitly unresolved source form; do not change it to `fardooley` by inference. Link the fuller morphology resource. | `repository-supported`; `unresolved` for `fardoole`; `intentional-retained` |
| N01-R003 | 57-68 | Clean semantic-broadening explanation using the two senses of `xagal`; it directly establishes that one form may have more than one meaning / low | Same file lines 25-30 and 59-64; dictionary collection contains multiple numbered senses and same-form headwords | Retain the subsection and example. Use it to limit, rather than repeat, the earlier absolute claim that every distinct referent must have a distinct word. | `intentional-retained`; `repository-supported` |
| N01-R004 | 69-105 | Coherent synonymy section; `irrid/illin` and `qalbi` meaning `xasuus` are independently supported, while the perfect-synonym and regional-distribution statements and the English/Arabic etymological notes are source claims rather than repository-wide conclusions / medium | `resources/qaamuus/08-s.md` entry `sammi²`; `resources/qaamuus/24-i.md` entries `illin¹` and `irrid`; `resources/qaamuus/14-q.md` entry `qalbi`; `resources/madax-ereyo/`; same-file examples | Retain all examples and the semantic distinctions. Frame the regional and etymological statements as the source's explanation rather than new SLS policy. Preserve the source's capitalization style unless a future approved capitalization standard requires normalization. | `repository-supported`; `intentional-retained` |
| N01-R005 | 106-125 | Clear antonymy section, but `dug` at lines 112 and 118 is the wrong lexical form for “old”: the same paragraph uses `duug` at line 120, while the dictionary defines `dug` as shelter/leeward and `duug` as old / high | Same file line 120; `resources/qaamuus/06-d.md` entries `dug` and `duug¹`; `resources/madax-ereyo/06-d.md`; `kharaar` and `cusayb` are independently supported | Retain the list, explanation, and `-darro` examples. Correct both occurrences of `dug` to `duug`; make no other lexical normalization. | `repository-supported` |
| N01-R006 | 126-159 | Clean compounding section and valid table; `afmiinshaar`, `xiddigdhul`, `madaxbannaan`, `biyamareen`, `xeerilaaliye`, and `gaaridameer` are supported within the repository / low | `resources/qaamuus/22-a.md` entry `afmiinshaar`; `resources/qaamuus/04-x.md` entry `xiddigdhul`; `resources/naxwe/13-aasaaska-naxwaha.md`; `resources/naxwe/15-naxwaha-sifayneed.md`; wordlists | Retain the complete subsection, table, and examples. Add a link to the detailed word-formation resource instead of introducing further compound examples. | `repository-supported`; `intentional-retained` |
| N01-R007 | 160-195 | Useful transition from meaning to sound, but “a word has one meaning” conflicts with lines 57-68, statements about all languages are overbroad, and “Af Soomaaligu af tiraab buu ahaa ilaa 1972” can misleadingly imply that Somali had no earlier writing rather than that the official Latin orthography was adopted in 1972; `lagama-maarmaan` has competing repository spellings / high | `spec/orthography/0001-alphabet.md` records official adoption on 21 October 1972 and reserves alternate scripts for separate treatment; `resources/qoraal/06-xarafka-weyn.md`; `resources/qaamuus/02-t.md` entry `tiraab²`; `resources/dhawaaq/`; dictionary and current resources disagree among `lagamamaarmaan`, `lagama maarmaan`, and the source form | Retain the sound/meaning discussion and examples 6-8, but qualify one-meaning and universal wording. Replace only the historical claim with the documented adoption of the official Latin script; do not make claims about the absence of earlier scripts. Leave `lagama-maarmaan` unchanged as unresolved. Link the phonology collection. | `repository-supported` for 1972 scope; `unresolved` for spelling; `intentional-retained` |
| N01-R008 | 196-211 | Structurally valid alphabet table, but the source calls 22 consonant signs plus five short and five long vowel spellings “32 symbols”; this conflicts with the normative SLS inventory, where long vowels are doubled sequences rather than additional alphabet letters / high | `spec/orthography/0001-alphabet.md` R1-R4 and R8 define 26 base letters—21 consonants and 5 vowels—plus the glottal-stop sign, with `dh`, `kh`, and `sh` as digraph letters and vowel length written by doubling; `resources/qoraal/README.md`; `resources/dhawaaq/README.md` | Preserve the alphabet topic but present the normative SLS inventory and collation clearly. Explain that `aa, ee, ii, oo, uu` encode long vowels as doubled vowel letters rather than five additional base letters. Do not silently retain the source's count as the SLS alphabet. | `repository-supported` |
| N01-R009 | 212-240 | Clean vowel-length and heavy/light-vowel material with valid table and examples; the five short/five long phonological distinction is compatible with SLS spelling, while detailed vowel quality belongs in dhawaaq / low | `spec/orthography/0001-alphabet.md` R8; `resources/dhawaaq/04-shaqaallada.md`; `resources/qaamuus/09-sh.md` entry `shaqal²`; `resources/naxwe/02-sarfaha-magacyada.md` | Retain the table and all five minimal comparisons. Explicitly distinguish five vowel letters from ten short/long vowel realizations and link detailed vowel quality to phonology. | `repository-supported`; `intentional-retained` |
| N01-R010 | 241-265 | Useful introductory codkac tables, but the prose states a general masculine/feminine and singular/plural rule more broadly than the supporting chapters; `ínan/inán`, `shílin/shilín`, and `áwr` are independently supported, while the exact accented `dameer`, `tuug`, and `Soomaali` rows occur only here / medium | `resources/naxwe/02-sarfaha-magacyada.md` lines 109-145 and 301-309; `resources/sarfe/01-magacyada.md`; `resources/sarfe/04-isbeddelka-codka.md`; `resources/dhawaaq/05-codadka-sare.md`; repository-wide accented-form search | Retain codkac as an introductory pattern, not an exceptionless rule. Keep independently supported rows; retain source-only accented rows only with clear source status or defer them rather than altering accents. Link the detailed noun and prosody resources. | `repository-supported`; `unresolved` for source-only accent rows |
| N01-R011 | 266-284 | The final-consonant discussion is readable, and `gunud`, `ilig`, `nin`, and `gacan` are supported, but the universal ban on final `/t/, /k/, /m/` has only partial repository support; the supplementary phonology analysis instead discusses final voiceless stops and explicit Arabic-loan exceptions / high | `resources/dhawaaq/08-gariirka-iyo-glotis-furan.md` lines 28-41 and 87-92; `resources/sarfe/04-isbeddelka-codka.md`; `resources/qaamuus/12-g.md` entries `guntid` and `gunud²`; dictionary/wordlist evidence for the remaining forms | Retain the source examples and the topic of final-position alternation, but do not present `/t/, /k/, /m/` as an exceptionless SLS phonotactic rule. Delegate the full analysis and exceptions to the phonology resources; do not infer that `/m/` should be replaced with another segment in the source list. | `repository-supported` in part; `unresolved` for the universal inventory |
| N01-R012 | 285-291 | Clean three-row assimilation set; `l + t → sh` and related morphophonological alternations are independently supported / low | `resources/naxwe/03-sarfaha-tifaftireyaasha.md` line 56; `resources/sarfe/04-isbeddelka-codka.md`; `resources/qaamuus/15-k.md` entry `-ka` includes `buug+ka → buugga` | Retain all three examples and link the detailed morphophonology resource. No reconstruction or added paradigm is needed. | `repository-supported` |
| N01-R013 | 292-301 | The three contraction/coalescence examples are legible but occur only in this source; the final explanation that the changes exist so speakers do not lose time is a teleological claim, while the orthography collection supports only the descriptive relation between rapid speech and contraction / medium | `resources/qoraal/01-hadal-iyo-qoraal.md` lines 9-18; `resources/qoraal/02-eray-kooban-hadalka.md`; repository-wide exact-sentence search finds examples 22-24 only here | Retain the source examples with visible source status and link the orthography treatment. Replace the causal conclusion with a descriptive statement about contraction in connected speech; do not turn the examples into normative spelling rules. | `intentional-retained`; `unresolved` for exact transformations |

## Proposed SLS-native blueprint

The cleaned chapter should remain titled **1. Ereyada (qaamuuska)** and retain
its present topical sequence:

1. words, lexicon, and the relation to syntax;
2. word formation: derivation, semantic broadening, synonymy, antonymy, and
   compounding;
3. sounds, writing, and the documented adoption of the official Latin script;
4. the normative SLS alphabet, separating base letters, the glottal-stop sign,
   digraphs, and doubled long-vowel spellings;
5. vowel length and vowel quality;
6. codkac as a basic pattern with links to the fuller noun and prosody
   treatments; and
7. final-position changes, assimilation, and connected-speech contraction,
   each linked to the relevant morphology, phonology, or orthography resource.

The cleanup must remain a conservative revision of this chapter, not a new
phonology textbook. It must preserve all secure source examples and tables,
make the two approved `dug → duug` corrections only if N01-R005 is approved,
and not invent accent marks, lexical forms, historical claims, or phonological
exceptions. Source-only forms and analyses must remain visible as such rather
than silently becoming SLS rules.

No new linguistic example may be introduced. Cross-links and short scope notes
may be added where they prevent duplication or overstatement.

## Audit approval

- **Audit approval:** approved by the maintainer on 2026-08-12 with the
  instruction, "go ahead."
- **Approved finding IDs:** N01-R001 through N01-R013
- **Deferred by default:** `fardoole`, `lagama-maarmaan`, the source-only
  accented rows, the exact `/t/, /k/, /m/` generalization, and the three
  contraction analyses remain unresolved; approval does not authorize guessed
  replacements.

## Cleanup result and review

The conservative SLS cleanup was applied on 2026-08-12.

Applied:

- retained both major sections, all nine subsections, all six tables, and all
  24 numbered source examples;
- corrected the two approved occurrences of `dug` to `duug`;
- qualified the one-word/one-meaning statements so they no longer contradict
  the chapter's own semantic-broadening section;
- retained `fardoole` and `lagama-maarmaan` with explicit unresolved-source
  notes rather than inferred normalizations;
- framed the regional synonym and loan-origin statements as explanations from
  the source;
- replaced the oral-only interpretation of the pre-1972 history with the
  repository-supported adoption date of the official Latin script;
- replaced the source's 32-symbol alphabet count with the SLS-0001 inventory
  of 26 base letters plus the glottal-stop sign, distinguishing doubled long
  vowels from additional base letters;
- scoped codkac and final-consonant descriptions so that source patterns do not
  become exceptionless SLS rules;
- retained the three source-only contraction examples while replacing their
  teleological conclusion with a descriptive connected-speech note; and
- added 13 resolving links to detailed syntax, morphology, phonology,
  orthography, and alphabet resources.

Deferred:

- no replacement was inferred for `fardoole` or `lagama-maarmaan`;
- no accent was added, removed, or altered in the source-only `dameer`, `tuug`,
  and `Soomaali` rows;
- the exact `/t/, /k/, /m/` source inventory was not promoted to a universal
  phonotactic rule; and
- the three contraction analyses remain source examples rather than normative
  spelling rules.

Validation:

- `git diff --check`: passed;
- one H1, two H2 sections, and nine H3 subsections;
- all six Markdown tables have consistent two-column structure;
- all 24 numbered examples are byte-for-byte unchanged from the audited file;
- all 13 local Markdown links resolve;
- all nine correction-log rows have ten TSV fields;
- no standalone italicized `dug` remains; and
- final size: 341 lines; 2,146 words; 13,712 bytes; SHA-256
  `7806536f6505fe1e988bebb6b7e47baa56a4df0a5e375ddf1165b30678e4bcc4`.

- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes
