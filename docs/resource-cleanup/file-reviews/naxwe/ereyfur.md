# Audit record — Naxwe grammar terminology (`ereyfur.md`)

- **Resource path:** `resources/naxwe/ereyfur.md`
- **Audited predecessor:** `resources/naxwe/ereyfur.tsv` (removed after the
  approved bilingual-format amendment)
- **Collection / family:** naxwe / Somali-English grammar terminology
- **Priority:** P2
- **Method:** repository-only, row-by-row terminology, structure, and format audit
- **Audit status:** approved; cleanup applied; awaiting maintainer cleanup review
- **Audit started:** 2026-08-12
- **File size at audit start:** 269 lines; 1,166 words; 9,735 bytes
- **Resource SHA-256 at audit start:**
  `8eb14e4e095ef660488612278c70563834f0135291c42e2207304ac5c7cfa541`
- **Resource-text changes during this audit:** none
- **Maintainer extension:** on 2026-08-12 the maintainer requested that cleanup
  also provide the glossary in Markdown format.

## Original target output model

The existing TSV remains the machine-readable evidence file: one lowercase
three-field header followed by 268 Somali/English/Italian terminology rows.
Cleanup must preserve every row and the three-column schema. It may apply only
the individually approved, repository-supported corrections below.

The requested `ereyfur.md` should be a human-readable companion, not a separate
terminology rewrite. It should contain one title, a short provenance and
non-normative-scope note, and a three-column Markdown table mechanically
mirroring the cleaned TSV in the same row order. Both formats must contain the
same 268 data records. The TSV is the synchronization source for the Markdown
presentation; neither file becomes a normative SLS terminology standard.

Necessary navigation updates may identify both formats in the naxwe README,
the naxwe source registry, the top-level resources map, and the descriptive
resources documentation. Those map changes must not alter grammar content or
create a second independent tracker workflow for the derived view.

## Maintainer scope amendment

After the first cleanup was applied but before cleanup approval, the maintainer
rejected the three-language/two-format presentation and approved a revised
single-file direction on 2026-08-12. The active resource is grammar
terminology—its subject is naxwe and its content type is ereybixin—so it remains
inside `resources/naxwe/` as one Somali-English Markdown reference.

This decision supersedes the output-format portions of ERF-R002, ERF-R003,
ERF-R009, and ERF-R010 without reopening the row-level corrections:

- preserve all 268 Somali terms and their cleaned English mappings;
- remove the Italian column from the active curated presentation;
- remove `ereyfur.tsv` and keep only `ereyfur.md`;
- add Somali prose explaining why the terminology belongs to naxwe, how it
  differs from a grammar lesson, its benefits, coverage, provenance, and
  descriptive/non-normative limit;
- retain the source's approximate row order instead of inventing semantic
  classifications for individual rows; and
- update every live grammar link, collection map, and the tracker to the single
  Markdown resource.

The removed Italian mappings remain recoverable from repository history and
from the original audit baseline. Their omission is an approved resource-scope
decision, not evidence that the 1998 source lacked Italian.

## Audit progress

| Original rows | Status | Finding |
| --- | --- | --- |
| 1 | reviewed | ERF-R001, ERF-R002 |
| 2-38 (`aan suntanayn`–`cutub`) | reviewed | ERF-R003, ERF-R004, ERF-R007 |
| 39-64 (`dahsoon`–`durkin`) | reviewed | ERF-R003, ERF-R004 |
| 65-94 (`erey`–`gudbe`) | reviewed | ERF-R003, ERF-R007, ERF-R008 |
| 95-124 (`haasaawe`–`iswaafaqsanaan`) | reviewed | ERF-R003, ERF-R004, ERF-R007 |
| 125-159 (`jinsi`–`ma (doorsoome)`) | reviewed | ERF-R003, ERF-R005, ERF-R007, ERF-R008 |
| 160-192 (`macne`–`nuxur`) | reviewed | ERF-R003 |
| 193-222 (`oraah faleed`–`suntane`) | reviewed | ERF-R003, ERF-R005 |
| 223-240 (`tagto`–`wadar`) | reviewed | ERF-R003, ERF-R005 |
| 241-269 (`weer`–`yeele dahsoon`) | reviewed | ERF-R003, ERF-R006, ERF-R007 |
| whole file and requested Markdown view | reviewed | ERF-R009, ERF-R010 |

## Findings

| ID | Rows | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| ERF-R001 | 1-269 | Provenance is recoverable and the collection role is descriptive, but the file itself contains no provenance or scope note / medium | `resources/naxwe/00-sources.md` associates the glossary with Puglielli and Mansuur's 1998 *Barashada Naxwaha Af Soomaaliga* and records no separate byline; the cleaned naxwe README distinguishes descriptive evidence from normative `spec/` rules | Keep TSV data-only. Put the title, 1998 source association, no-separate-byline limitation, and descriptive/non-normative boundary in the Markdown companion; retain source attribution in `00-sources.md` rather than adding metadata rows to the TSV. | `repository-supported`; `provenance-clarification`; `scope-correction`; `intentional-retained` |
| ERF-R002 | 1-269 | Structurally sound UTF-8 TSV / retain | Validation finds one three-field header and 268 three-field data rows, no blank fields, no leading/trailing cell whitespace, valid UTF-8, LF endings, 268 unique Somali heads, 264 unique English glosses, and 265 unique Italian glosses | Preserve the exact three-column schema, lowercase header names, tabs, UTF-8/LF encoding, and one-record-per-line structure. Add no definition, status, alias, or notes column during this cleanup. | `same-file-exact`; `structural-only`; `intentional-retained` |
| ERF-R003 | all data rows except separately listed corrections | The terminology inventory is coherent source evidence; many older or source-specific forms are not used in the maintained chapters / retain | Exact matching finds 100 Somali heads absent from current naxwe Markdown, consistent with the README's approved warning that not every glossary entry is necessarily explained in a chapter | Preserve every unlisted Somali head and both translations. Do not remove terms merely because cleaned chapters do not use them, and do not modernize source terminology wholesale. The Markdown companion must mirror the same complete inventory. | `same-file-exact`; `source-specific`; `intentional-retained` |
| ERF-R004 | 5, 8, 41, 42, 110 | Five Somali forms conflict with direct repository headwords or same-concept dictionary phrases and have the shape of transcription errors / high | `qaamuus/22-a.md` gives `af qoraal` and `af tirabeed`; `qaamuus/10-dh.md` gives `dhalanrog` and `dhammaad`; `qaamuus/20-h.md`, the wordlist, and naxwe file 08 give `hogatus`. The current forms are `af goraal`, `af tiraaheed`, `dhalanrag`, `dharamaad`, and `hogatuus` | After approval, change only those five Somali cells to `af qoraal`, `af tirabeed`, `dhalanrog`, `dhammaad`, and `hogatus`. Retain their English and Italian cells. Log every old/new pair separately; do not use these corrections as permission for general spelling normalization. | `repository-supported`; `transcription-correction` |
| ERF-R005 | 136, 143, 158, 213-214, 229 | Six English glosses conflict with internal trilingual evidence, repository definitions, or the glossary's own translation pattern / high | Row 136's Italian `predicato verbale` and rows 76/193 translate `faleed` as “verbal”; dictionary `lafdi` denotes the expression/sign form while Italian has `significante`; dictionary `luuq²` denotes a sung tune and Italian has `melodia`; `vocale arretrata/avanzata` correspond to standard “back/front vowel”; `tebi` means transmit and Italian `emittente` distinguishes row 229 from `hadle` “speaker” at row 105 | Change only the English cells: `verb predicate` → `verbal predicate`; `significant` → `signifier`; `rhythm` → `melody`; `backward vowel` → `back vowel`; `forward vowel` → `front vowel`; and row 229 `speaker` → `sender`. Retain the Somali and Italian cells. Log each correction separately. | `repository-supported`; `same-file-internal`; `translation-correction`; `false-friend-correction` |
| ERF-R006 | 4/156, 80/268, 105/229, 245/258 | Four repeated English concepts are not duplicate Somali records: `af`/`luqad`, `fale`/`yeele`, `hadle`/`tebiye`, and `weer caddayn`/`weer tebineed` / medium | The dictionary defines `af` through `luqad`; supplementary file 17 uses `fale` while core chapters use `yeele`; files 06 and 12 use both sentence labels. Italian distinguishes `hadle` (`parlante`) from `tebiye` (`emittente`), exposing the English error covered by ERF-R005 | Preserve all eight Somali rows. Do not deduplicate aliases or collapse source traditions. Correct only `tebiye`'s English mapping under ERF-R005; retain the other repeated target glosses as evidence of distinct Somali labels. | `repository-supported`; `source-specific`; `intentional-retained` |
| ERF-R007 | 12, 69, 92, 118, 152, 157-159, 188, 211, 243; whole ordering | Apostrophes, hyphens, parentheses, slashes, and source ordering are internally valid but several forms have no repository parallel; bytewise sorting reports five local inversions / medium | `alifba'`, sense labels such as `luuq (cilmi-afeed)`/`luuq (suugaan)`, and the synonym label `weer adag (ballaaran)` carry meaning. No exact internal parallel resolves `fure (keed)` or `ma (doorsoome)`. The glossary is approximately Latin-alphabetical but no Somali collation policy is declared | Retain punctuation, parenthetical disambiguators, `fure (keed)`, `ma (doorsoome)`, and the existing row order. Do not globally sort or silently reconstruct unresolved heads. Mirror them exactly in Markdown and record the collation limitation in the review, not as glossary data. | `same-file-exact`; `unresolved`; `intentional-retained` |
| ERF-R008 | 81, 93, 112-113, 138, 142, 195, 211, 235-236, 261 | Number, slash, and English style vary (`compound verb`, `decodification`, singular target glosses for plural Somali category labels, `bi-syllabic`, `morpho-syntax`) / medium | The three columns often use conventional citation forms rather than matching grammatical number; no local source scan or repository policy proves these stylistic variants erroneous | Retain these cells in this cleanup. Do not impose a new English house style or infer singular/plural repairs. They may be proposed later as structured terminology records with explicit review status. | `source-specific`; `unresolved`; `intentional-retained` |
| ERF-R009 | requested output | A machine-readable TSV alone is inconvenient for human browsing; maintainer explicitly requested Markdown / medium | `docs/ARCHITECTURE.md` favors machine-first, human-readable resources, and the repository's terminology collections commonly use Markdown tables/prose. No existing TSV/Markdown pair supplies an automatic converter | Add `resources/naxwe/ereyfur.md` during approved cleanup. Include one H1, a short Somali-first scope/provenance note, and one three-column table with exactly 268 rows. Generate the table mechanically from the cleaned TSV, escape Markdown-sensitive cell content if needed, and validate cell-for-cell equality. Do not add definitions or independent linguistic wording. | `maintainer-requested`; `format-companion`; `structural-only`; `navigation-update` |
| ERF-R010 | collection maps | Adding a companion without updating maps would make repository counts and navigation stale / medium | `resources/naxwe/README.md`, `resources/naxwe/00-sources.md`, `resources/README.md`, and `docs/RESOURCES.md` currently name only `ereyfur.tsv` | During approved cleanup, update those four maps minimally to link both TSV and Markdown presentations as one glossary resource. Keep the tracker gate on the existing `ereyfur.tsv` row and document the companion in this same review; do not represent the derived view as a second independently sourced glossary. | `repository-supported`; `maintainer-requested`; `navigation-update`; `structural-only` |

## Original SLS-native blueprint (superseded in format)

Approved cleanup should produce one synchronized glossary resource in two
formats:

1. retain `ereyfur.tsv` as header plus 268 tab-separated records;
2. apply exactly five Somali-cell corrections from ERF-R004;
3. apply exactly six English-cell corrections from ERF-R005;
4. preserve every other cell and the existing row order;
5. create `ereyfur.md` with a Somali-first title/scope note and a 268-row
   Somali/English/Italian table derived from the cleaned TSV;
6. state the source association and non-normative evidence role outside the
   data table;
7. update the four collection/repository maps to expose both formats; and
8. validate exact cross-format row equality, links, schema, uniqueness, UTF-8,
   and LF endings.

No term should be added or removed. No definitions, usage rulings, canonical
status, inferred alias relationships, or global orthographic modernization
should be introduced.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-12 with the
  instruction, "go ahead for the one we audited."
- **Approved finding IDs:** ERF-R001 through ERF-R010, as amended by the
  maintainer's bilingual single-file decision
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no
- **Decision requested:** review and approve the bilingual `ereyfur.md` before
  cleanup approval or completion is marked.

## Audit validation

- Resource SHA-256 after audit:
  `8eb14e4e095ef660488612278c70563834f0135291c42e2207304ac5c7cfa541`
  (unchanged).
- Resource diff during this audit: none.
- Structure: one header plus 268 data rows; every row has exactly three
  tab-separated fields; no empty or edge-whitespace cells.
- Encoding: valid UTF-8 with LF line endings.
- Uniqueness: 268 unique Somali heads; 264 unique English glosses; 265 unique
  Italian glosses.
- Repeated English mappings: four pairs (`language`, `speaker`, `subject`, and
  `declarative sentence`); no duplicate full records.
- Chapter coverage check: 100 Somali heads have no exact case-insensitive match
  in current naxwe Markdown. This is recorded as scope evidence, not deletion
  authority.
- Punctuation check: twelve Somali heads contain an apostrophe, hyphen, or
  parenthetical disambiguator; all are preserved during audit.
- Ordering check: five adjacent bytewise inversions; no global reorder is
  proposed without a declared collation policy.
- Requested `ereyfur.md`: not created during audit; creation remains behind the
  audit-approval gate.

## Cleanup result and review

### Applied cleanup

- Preserved all 268 Somali terms, their cleaned English mappings, the original
  row order, and every bilingual pair not individually approved for change.
- Corrected five repository-supported Somali transcription errors:
  `af goraal` → `af qoraal`, `af tiraaheed` → `af tirabeed`, `dhalanrag` →
  `dhalanrog`, `dharamaad` → `dhammaad`, and `hogatuus` → `hogatus`.
- Corrected six supported English mappings: `verb predicate` → `verbal
  predicate`, `significant` → `signifier`, `rhythm` → `melody`, `backward
  vowel` → `back vowel`, `forward vowel` → `front vowel`, and the `tebiye`
  mapping `speaker` → `sender`.
- Rebuilt `resources/naxwe/ereyfur.md` as the sole active resource with one
  Somali-first title, four explanatory sections, and a two-column
  Somali-English table containing all 268 records.
- Added Somali explanations that identify the file as grammar terminology,
  distinguish it from a chapter teaching grammar rules, describe its benefits
  for consistent writing, lookup, translation, NLP, and AI, summarize its
  grammar coverage, and state its descriptive/non-normative limit.
- Identified the glossary's association with *Barashada Naxwaha Af Soomaaliga*
  (1998), stated that no separate byline is recorded, and kept its role
  descriptive rather than normative.
- Recorded that the 1998 source glossary was trilingual while limiting the
  active reference to the maintainer-approved Somali-English scope.
- Removed `resources/naxwe/ereyfur.tsv`; it remains recoverable from Git and
  its original structure and hash remain recorded in this audit.
- Updated `resources/naxwe/README.md`, `resources/naxwe/00-sources.md`,
  `resources/README.md`, and `docs/RESOURCES.md` to expose the single bilingual
  Markdown resource.
- Updated eight grammar files—08 through 12, 14, 16, and 17—so every live
  glossary link resolves to `ereyfur.md`.
- Moved the existing tracker gate from the removed TSV path to `ereyfur.md`
  without changing its cleanup-approval or completion state.

### Deliberately retained

- All 268 concepts and all English mappings other than the six approved
  repairs. Italian is retained as source-history provenance, not as an active
  column.
- Source-specific and unresolved terms, including `fure (keed)`, `ma
  (doorsoome)`, parenthetical sense labels, hyphenation, apostrophes, slashes,
  and varying number/style in target glosses.
- The approximate source order and all intentional alias rows, including
  `af`/`luqad`, `fale`/`yeele`, and `weer caddayn`/`weer tebineed`.
- A reference table without term-by-term definitions, usage rulings, canonical
  status, or a new terminology schema.

### Cleanup validation

- Cleanup approval and completion remain unmarked pending maintainer review.
- `git diff --check`: passed.
- Pair preservation: the 268 Markdown pairs exactly match the original TSV's
  Somali-English columns after applying the five approved Somali and six
  approved English corrections; no pair was added, removed, duplicated, or
  reordered.
- Markdown structure: one H1, four H2 sections, one two-column table header,
  and 268 unique-Somali data rows.
- Scope checks: the introduction explicitly identifies grammar terminology,
  its relationship to naxwe, its difference from a grammar lesson, benefits,
  coverage, 1998 trilingual provenance, omission of Italian from the active
  view, and descriptive/non-normative status.
- Navigation: all ten affected naxwe files and both repository-level maps link
  the single Markdown resource; no live resource, map, documentation, or
  tracker reference to `ereyfur.tsv` remains.
- Removal: `resources/naxwe/ereyfur.tsv` is absent; its audited baseline is
  recoverable from Git and this record.
- Provenance TSV validation: every row has 10 tab-separated fields.
- Cleaned resource size: 319 lines; 1,878 words; 9,915 bytes.
- Cleaned resource SHA-256:
  `44941f118da019e2df588b0a7e84c13ba1d8ae694c0318e9657101e8a7bd842d`.

## Primary-PDF source amendment — 2026-08-12

The original scan was subsequently inspected through the approved primary-PDF
source pass. It establishes that the book was first published in **1999**, not
1998, and that the source glossary has a fourth column, `MICNAHA EREYGA`, with
a Somali explanation for each term (PDF pages 293–318).

The maintainer approved taking useful material from the source. The active
`ereyfur.md` therefore now retains the same 268 Somali–English concepts and
adds a short Somali explanation for every row. Italian remains omitted under
the earlier scope decision. The table is an SLS-readable source adaptation,
not a facsimile; the page-bounded OCR, scan and extraction helper are recorded
in `source-evidence/work/ocr/barashada-naxwaha-1999/`.

This amendment supersedes the earlier no-definitions output restriction and
the 1998 provenance wording. It does not mark cleanup approval or completion.
