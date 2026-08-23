# Audit record — Hadal iyo Qoraal

- **Resource path:** `resources/qoraal/01-hadal-iyo-qoraal.md`
- **Collection / family:** qoraal / writing principles
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; SLS-native cleanup applied and awaiting
  maintainer cleanup review
- **Audit started:** 2026-08-19
- **File size at audit start:** 93 lines; 732 words; 4,444 bytes
- **Resource SHA-256 at audit start:**
  `7a0066231e68344a7731b6d9b47284d264b45394f5a41e9fa63ff5ef277e1b0f`
- **Resource-text changes during this audit:** none

## Target output model

This file is already a compact SLS introduction chapter rather than an OCR
transcript. It has one H1, seven H2 sections, two H3 subsections, two fenced
speech/writing pairs, two numbered lists, one bullet list, and no cover
matter, exercises, page markers, or damaged reading order.

Cleanup should keep it as the collection's principles chapter: speech versus
writing, the two splitting rules, a scoped punctuation preview, the source's
five word-classes, and contraction versus expansion. It should stop speaking
as if this file *is* the 1977 booklet, repair uniquely supported OCR splits,
qualify the five-class list against `naxwe/`, and add links to files `02`–`05`
and to punctuation versus prosody. It must not modernize the source examples,
invent a replacement for unresolved expansions, or duplicate later files.

## Source and repository evidence

`resources/qoraal/00-sources.md` identifies *Habka Qoraalka* (Maxamed
Xaaji Xuseen Raabi, 1977) as the primary source for files `01`–`05`. The same
author's supplementary grammar already points here:

- cleaned `resources/naxwe/14-naxwaha-cusub.md` §4.1–4.2 restates the two
  splitting rules and links this file as the maintained orthography account.

Repository comparison also included:

- `resources/qoraal/02-eray-kooban-hadalka.md` through `05-astaamaynta.md`;
- `resources/qoraal/README.md` and `docs/RESOURCES.md`;
- `resources/naxwe/ereyfur.md` for `falkaab`, `tilmaame`, `qurub`, `astaamayn`;
- `resources/dhawaaq/05-codadka-sare.md` for prosodic *astaamaynta*;
- dictionary and wordlist checks as corroboration, not as authority to rewrite
  source examples.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-7 | reviewed | O01-R001 |
| 9-22 | reviewed | O01-R002 |
| 24-35 | reviewed | O01-R003 |
| 37-46 | reviewed | O01-R004 |
| 48-59 | reviewed | O01-R005 |
| 61-71 | reviewed | O01-R006 |
| 73-82 | reviewed | O01-R007 |
| 84-93 | reviewed | O01-R008 |
| whole file | reviewed | O01-R009 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| O01-R001 | 1-7 | Useful two-topic opening, but `Buuggani` presents this SLS chapter as the 1977 booklet; `hufo` is dictionary-supported (`huf²` sifayn) / medium | Collection README and `00-sources.md` keep author/year in the registry; naxwe content files do not speak as the source book; `resources/qaamuus/20-h.md` `huf²` | Retain the title, the speech-versus-writing plus punctuation scope, and `hufo`. Reframe the opening as this collection's introduction, not the physical booklet. Do not add author/year here. | `repository-supported`; `scope-correction`; `intentional-retained` |
| O01-R002 | 9-22 | Clear speech/writing contrast and the maintained `Lana iman` / `La ma iman` pair; `laga ma naarmaan` is a split of repository `lagama maarmaan`; `dhannaanta` has no independent match; the clause that learning speech *and* writing precedes learning speech contradicts the same sentence / high | `qoraal/05` and `naxwe/10` use `lagama maarmaan`; dictionary `dhammaan` supports end-versus-beginning; `lama huraan` / `lamahuraan` support `la na huraan`; naxwe 14 §4.1 restates the same contrast and already links this file | Retain the section, the fenced pair, and the `ma` + negative-particle analysis. Correct `laga ma naarmaan` to `lagama maarmaan` and `la na huraan` to `lama huraan`. Correct `dhannaanta` to `dhammaanta`. Keep the first clause that language begins in speech; repair the contradictory second clause so writing is learned after speech, without adding a new example. | `repository-supported`; `form-correction`; `logic-correction` |
| O01-R003 | 24-35 | Two splitting rules and the `waxaan` / `waxa aan` pair match the cleaned naxwe 14 summary; `doorsoomayo` / `door soconayo` is this source's stem example, not naxwe 14's `arag` / `ar ag` / medium | Cleaned `naxwe/14` §4.1 gives the same two rules and points here; file `02` independently lists `waxaan` / `waxa aan` | Retain both rules and both examples. Do not replace `doorsoomayo` with `arag`. Add a link to naxwe 14 only if it helps navigation; do not import its table. | `repository-supported`; `intentional-retained` |
| O01-R004 | 37-46 | Valid punctuation preview and the comma-split `Cali ayaa, toogtay libaax` contrast; the section does not distinguish orthographic from prosodic *astaamaynta* / medium | `ereyfur.md` `astaamayn`; `qoraal/05-astaamaynta.md` is the mark inventory; `dhawaaq/05-codadka-sare.md` is juncture/prosody; `docs/RESOURCES.md` already records that split | Retain both example sentences and the “one mark once the clause is sound” claim as this chapter's preview. Link `05-astaamaynta.md` and note that prosodic *astaamaynta* lives in `dhawaaq/05`. Do not copy the twelve-mark inventory here. | `repository-supported`; `scope-correction`; `intentional-retained` |
| O01-R005 | 48-59 | Source's *hadal* / *qoraal* labels are clear; `Lugadaha` conflicts with the same file's `luqaddu` and with dictionary `luqad` / `luuqad`; the universal “all languages” framing is broader than the repository can establish / medium | `resources/qaamuus/16-l.md` `luqad` ld `luuqad`; line 13 already uses `luqaddu`; naxwe 00 scoped similar universals to spoken languages under discussion | Correct `Lugadaha` to `Luqadaha`. Retain the *hadal* / *qoraal* definitions and the two reading diagnostics. Scope the opening to the spoken languages this source is describing. | `repository-supported`; `form-correction`; `scope-correction`; `intentional-retained` |
| O01-R006 | 61-71 | The five-class list is the source's compact split, but it can look like the complete SLS word-class system; the same `laga ma naarmaan` split recurs / high | `ereyfur.md` supports `magac`, `fal`, `tilmaame`, `falkaab`, `qurub`; naxwe chapters 02–07 give the maintained detailed classes; file `02` repeats the same introductory sentence | Retain all five labels. Present them as this source's writing-oriented classes and link `naxwe/01` for the detailed grammar. Correct `laga ma naarmaan` to `lagama maarmaan`. Do not add extra word classes. | `repository-supported`; `scope-correction`; `form-correction`; `intentional-retained` |
| O01-R007 | 73-82 | Useful contraction/expansion principle and the `waxaan` pattern; the expansion `Ku ma baa aad tahay?` for `kumaad tahay?` inserts `baa` that the pronoun reading does not support / high | Dictionary `kuma` is the masculine interrogative “who”; `kumaad` as a numeral is a homograph and does not take `tahay`; file `02` expands `waxaan` as `waxa aan` and `buu` as `baa uu`, but has no `kumaad` row; naxwe 14's verified pairs do not include this expansion | Retain `kumaad tahay?` and the principle that expansion must not change grammatical meaning. Correct the expansion to `kuma aad tahay?`. Do not add `baa` or a new contracted pair. | `repository-supported`; `form-correction` |
| O01-R008 | 84-93 | Accurate four-part map of the rest of the 1977 book, but the targets are not linked and the closing still speaks as `Buuggani` / medium | Files `02`–`05` exist and match the four bullets; file `06` is a later Nilsson supplement and is not one of the source's four parts | Retain all four coverage bullets. Convert them to local links to `02`–`05`. Do not add `06` to this source map. Reframe `Buuggani` as this collection. | `repository-supported`; `navigation-update`; `intentional-retained` |
| O01-R009 | whole file | Compact, already SLS-shaped, and free of page-marker OCR, but it still lacks collection links and carries a few pre-cleanup splits / medium | Seven H2 sections, two fences, no `## OCR Page` markers; cleaned naxwe 14 already treats this file as canonical | Preserve the seven-section sequence and both example pairs. Apply only the approved form, scope, and link repairs. Do not duplicate files `02`–`05`. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Hadal iyo Qoraal** and keep this
sequence:

1. collection-level purpose (speech versus writing, plus punctuation);
2. the `Lana iman` / `La ma iman` contrast;
3. the two splitting rules and the `doorsoomayo` / `waxaan` examples;
4. a punctuation preview linking to `05` and to dhawaaq/05;
5. *hadal* / *qoraal* labels, scoped to spoken language;
6. the source's five word-classes, linked to `naxwe/01`;
7. contraction versus expansion, with `kuma aad tahay?`; and
8. linked coverage of files `02`–`05`.

No new orthographic example should be introduced. `doorsoomayo` / `door
soconayo` and `Lana iman` / `La ma iman` must stay. File `06` stays out of
this source map. The same `laga ma naarmaan` split in file `02` is out of
scope until that file is audited.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** O01-R001 through O01-R009
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Audit validation

- Resource SHA-256 after audit:
  `7a0066231e68344a7731b6d9b47284d264b45394f5a41e9fa63ff5ef277e1b0f`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, seven H2 headings, two H3 subsections, two fenced
  pairs, two numbered lists, one bullet list, no OCR page markers.

## Cleanup result and review

### Applied cleanup

- Reframed the opening and closing from `Buuggani` to this collection's
  introduction chapter; retained `hufo` and the two-topic scope.
- Corrected `laga ma naarmaan` to `lagama maarmaan`, `la na huraan` to
  `lama huraan`, `dhannaanta` to `dhammaanta`, and `Lugadaha` to `Luqadaha`.
- Repaired the contradictory learning-order clause so writing is learned after
  speech.
- Linked the two splitting rules to `naxwe/14` §4.1 without importing its
  table.
- Linked punctuation to `05-astaamaynta.md` and prosodic *astaamaynta* to
  `dhawaaq/05`.
- Scoped the five word-classes as this source's writing split and linked
  `naxwe/01`.
- Corrected the expansion `Ku ma baa aad tahay?` to `kuma aad tahay?`.
- Linked the four-part map to files `02`–`05`; file `06` was not added.

### Deliberately retained

- `Lana iman` / `La ma iman`.
- `doorsoomayo` / `door soconayo` and `waxaan` / `waxa aan`.
- Both `Cali ayaa toogtay libaax` sentences.
- All five source word-class labels.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, seven H2 headings, two H3 subsections, two fenced pairs.
- Pre-cleanup forms `Buuggani`, `naarmaan`, `dhannaanta`, `Lugadaha`, and
  `Ku ma baa` are absent.
- Cleaned file size: 99 lines; 775 words; 5,163 bytes.
- Cleaned SHA-256:
  `8118c5923fd24f2e2cbb32b3e75cdde43254d13c8709803dec46f021bc39290f`.
