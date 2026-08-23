# Audit record — Astaamaynta Af Soomaaliga

- **Resource path:** `resources/qoraal/05-astaamaynta.md`
- **Collection / family:** qoraal / punctuation inventory
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 304 lines; 1,681 words; 10,582 bytes
- **Resource SHA-256 at audit start:**
  `1bbbaeefdb357fc725cd4ff05cb3debb40dc4361345e6e8fd5c884ec80f6b853`
- **Resource-text changes during this audit:** none

## Target output model

This file is already a compact SLS punctuation inventory, not an OCR
transcript. It has one H1, six H2 sections, twelve H3 mark subsections, the
four source groups, and no cover matter, exercises, or page markers.

Cleanup should keep it as the maintained twelve-mark reference. It should
reframe the 1977-era opening so this file is the collection chapter, repair
`dhammaato` / `dhammaan` / `qurxoon` / `nidaam iyo hab`, and add links to
file `01` and to prosodic *astaamaynta*. It must not invent semicolon
examples, import the U+02BC hamsa rule as this chapter's spelling, reduce
the inventory, or modernize source example names.

## Source and repository evidence

`resources/qoraal/00-sources.md` maps this file to *Habka Qoraalka* (1977)
Qaybta II. Cleaned `naxwe/14` §5 already points here and does not copy the
twelve marks.

Repository comparison also included:

- cleaned `resources/qoraal/01-hadal-iyo-qoraal.md` (O01-R002, O01-R004);
- `resources/dhawaaq/05-codadka-sare.md`; `docs/RESOURCES.md` punctuation
  versus prosody split;
- `resources/naxwe/ereyfur.md` `astaamayn`, `hakad`, `kolmo`, `qaanso`;
- dictionary `hamsa`, `kolmo`, `qurxoon`, `dhammaan`;
- `spec/orthography/0001-alphabet.md` R6 for hamsa as a letter, not as this
  chapter's punctuation mark.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-11 | reviewed | O05-R001 |
| 13-21 | reviewed | O05-R002 |
| 24-41 | reviewed | O05-R003 |
| 45-82 | reviewed | O05-R004 |
| 86-202 | reviewed | O05-R005 |
| 203-252 | reviewed | O05-R006 |
| 256-296 | reviewed | O05-R007 |
| 298-304 | reviewed | O05-R008 |
| whole file | reviewed | O05-R009 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| O05-R001 | 1-11 | Useful four-group preview, but the opening still speaks as the 1977 booklet waiting for books; `lagama maarmaan` is already correct; `nidaam ann hab` is debris; `yeelano` / `nalee` are weaker / medium | O01-R001 reframed `Buuggani`; same paragraph already uses `nid`; dictionary does not uniquely repair `nalee` | Retain the 1977-era diagnosis as this source's situation, not as SLS still lacking books. Correct `nidaam ann hab` to `nidaam iyo hab`. Leave `yeelano` and `la iska nalee` unresolved. Add a back-link to file `01`. Do not add author/year here. | `repository-supported`; `scope-correction`; `form-correction`; `unresolved`; `intentional-retained` |
| O05-R002 | 13-21 | The four groups list all twelve planned marks / low | Orthography README; `ereyfur.md` `astaamayn` | Retain the four numbered groups and every mark name. Do not add or drop a mark. | `repository-supported`; `intentional-retained` |
| O05-R003 | 24-41 | Hamsa is correctly scoped as a letter, not a meaning-changing punctuation mark; the hyphen compounds are this source's examples / medium | Dictionary `hamsa`; `spec/orthography/0001-alphabet.md` R6; file `02` `dhuuxi` does not uniquely license `dhuubnaan` → `duubnaan` | Retain both subsections and `Isku-dhuubnaan` / `Wax-wada-qabsi`. Link `naxwe/01` and the alphabet spec for hamsa-as-letter. Do not import U+02BC into the examples. | `repository-supported`; `scope-correction`; `intentional-retained` |
| O05-R004 | 45-82 | Sentence-final marks are usable; `dhannaato` conflicts with the same paragraph's `dhammaatay` / high | O01-R002 `dhannaanta` → `dhammaanta`; dictionary `dhammaan` / `dhammaad` | Correct both `dhannaato` to `dhammaato`. Retain all joogsi / weydiin / yaab examples, including `Maxaa aad u tidhi?`. | `repository-supported`; `form-correction`; `intentional-retained` |
| O05-R005 | 86-202 | The hakad qalad/sax pairs are the core inventory; `gurxoon` is dictionary `qurxoon`; `dhannaan` is the same `dhammaan` split / high | Dictionary `qurxoon`; O01-R002; both qalad and sax lines keep `narsan` | Correct `gurxoon` to `qurxoon` and both `dhannaan` to `dhammaan`. Retain every qalad/sax pair, including `narsan rinji` and the qalad `ba`. Do not invent a replacement for `narsan`. | `repository-supported`; `form-correction`; `unresolved`; `intentional-retained` |
| O05-R006 | 203-252 | Semicolon, colon, and dash sections are usable; semicolon has no unique examples; dash `Buuggan` is an example sentence, not booklet voice / low | Colon qalad/sax already contrast `;` versus `,` and `:` | Retain all three subsections. Do not invent semicolon sentences. Keep `Jaanac`, `Buuggan ma gado`, and `na kula tahay` unresolved. | `unresolved`; `intentional-retained` |
| O05-R007 | 256-296 | Quotation and parenthesis marks match `kolmo` / `qaanso`; the kolmo heading shows `' '` while the examples use `" "` for outer quotes; `imanayna` lacks the 1pl ending; nested `narkii` / `narkuu` are not uniquely `markii` / medium | Dictionary `kolmo`; `ereyfur.md` `qaanso`; Somali letter order `b t j x` in the `(a) (b) (t) (j) (x)` list; O04 `narkii` → `markii` was a when-clause, not this nested quote | Retain both quote subsections and the qaanso list. Correct `imanayna` to `imanaynaa`. Leave `narkii` / `narkuu`, `Agelade`, and the mixed `(a)/(t)` numbering unresolved. Do not force the kolmo heading's glyphs onto the examples. | `repository-supported`; `form-correction`; `unresolved`; `intentional-retained` |
| O05-R008 | 298-304 | Accurate closing that speech and writing punctuation still need care; it does not point to prosody or back to file `01` / low | O01-R004; `dhawaaq/05`; `naxwe/14` §5 | Retain the three error types. Link `dhawaaq/05` and file `01`. Do not reduce “afarta astaanood” to four marks; it is the four groups. | `repository-supported`; `navigation-update`; `intentional-retained` |
| O05-R009 | whole file | Already SLS-native and twelve-mark complete; a few OCR splits and booklet voice remain / medium | Six H2 sections, twelve H3 marks, no OCR page markers | Preserve the four-group sequence and every mark. Change only the approved form, scope, and link repairs. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Astaamaynta Af Soomaaliga** and keep
this sequence: opening; four groups; hamsa and hyphen; sentence-final marks;
in-clause marks; special marks; speech versus writing. Links go to file `01`,
`naxwe/01`, `dhawaaq/05`, and the alphabet spec. No mark is added or
removed. `narsan`, `Agelade`, `Isku-dhuubnaan`, `yeelano`, and nested
`narkii` stay unresolved.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** O05-R001 through O05-R009
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `1bbbaeefdb357fc725cd4ff05cb3debb40dc4361345e6e8fd5c884ec80f6b853`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, six H2 headings, twelve H3 mark subsections, no
  OCR page markers.

## Cleanup result and review

### Applied cleanup

- Reframed the opening as this collection's punctuation chapter; retained the
  source's 1977 diagnosis as the book's situation, not as SLS still lacking
  books.
- Corrected `nidaam ann hab` to `nidaam iyo hab`.
- Linked the opening and closing to `01-hadal-iyo-qoraal.md`.
- Linked hamsa-as-letter to `naxwe/01-ereyada.md` and
  `spec/orthography/0001-alphabet.md` without importing U+02BC.
- Corrected both `dhannaato` to `dhammaato`, `gurxoon` to `qurxoon`, both
  `dhannaan` to `dhammaan`, and `imanayna` to `imanaynaa`.
- Linked the closing to `dhawaaq/05-codadka-sare.md`.

### Deliberately retained

- `yeelano` and `la iska nalee`.
- `Isku-dhuubnaan` / `Wax-wada-qabsi`.
- All joogsi / weydiin / yaab examples, including `Maxaa aad u tidhi?`.
- Every hakad qalad/sax pair, including `narsan rinji` and the qalad `ba`.
- No invented semicolon examples; `Jaanac`, `Buuggan ma gado`, and
  `na kula tahay`.
- Kolmo heading glyphs versus `" "` examples; nested `narkii` / `narkuu`;
  `Agelade`; mixed `(a)/(t)` numbering.
- All twelve marks and “afarta astaanood” as the four groups.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, seven H2 headings, twelve H3 mark subsections, no OCR
  page markers.
- Pre-cleanup forms `nidaam ann hab`, `dhannaato`, `gurxoon`, `dhannaan`, and
  `Waan imanayna.` are absent.
- Cleaned file size: 310 lines; 1,722 words; 11,134 bytes.
- Cleaned SHA-256:
  `bfedea0403a4aee7a2fce45db95831a1d12545cfb5de511fe28d3856bbd52607`.
