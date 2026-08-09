# Resources OCR Cleanup and Source Authentication Plan

**Project:** Somali Language Standard (SLS)  
**Scope:** `resources/`  
**Plan date:** 2026-08-09  
**Status:** In progress — Phase 1 complete; awaiting review before Phase 2
**Primary outcome:** A traceable, source-faithful, human-reviewed evidence library that can safely support standards, dictionaries, corpora, translation, and AI work.

## 1. Purpose

Most documents in `resources/` were converted from PDFs, and some conversions contain OCR errors. The folder is already useful as a collection, but it cannot yet be treated uniformly as authenticated source text. This plan defines how to:

1. preserve the original evidence;
2. identify OCR and conversion defects;
3. verify corrections against source pages;
4. distinguish faithful transcription from editorial normalization;
5. record provenance, rights, and review status;
6. publish only material that passes measurable quality gates.

The cleanup must not modernize spelling, silently improve an author's language, merge variants, or replace historical forms merely because they look unusual. A correction is accepted as a transcription correction only when the source page supports it.

## 2. Current Situation

### 2.1 Repository baseline

The current `resources/` folder contains **145 files**, approximately **8.35 MB**, and approximately **176,127 lines**. It is divided into eight collections.

| Collection | Files | Approx. lines | Current role | Initial status |
| --- | ---: | ---: | --- | --- |
| `qaamuus/` | 34 | 48,320 | Monolingual dictionary and abbreviation key | Review; bibliography and rights incomplete |
| `wordlists/` | 28 | 47,611 | Headwords derived from `qaamuus/` | Derived; 31 unmatched heads already documented |
| `naxwe/` | 21 | 14,977 | Grammar chapters, supplements, and `ereyfur.tsv` | Mixed; files `13`–`17` explicitly carry OCR risk |
| `erey-bixin/` | 11 | 12,377 | Technical and administrative glossaries | Mixed; file `09` is partial |
| `suugaan/` | 26 | 50,631 | Literature, poetry, proverbs, and school texts | Mixed; several large OCR-derived files are noisy |
| `orthography/` | 8 | 855 | Writing and punctuation references | Reported as manually verified, with one interim supplement |
| `phonology/` | 10 | 990 | Somali phonology reference | Reported as curated/manually verified |
| `morphology/` | 6 | 366 | Derived paradigm tables | Curated derivative evidence |

All 144 Markdown files have a top-level `#` heading where expected. The remaining file is `naxwe/ereyfur.tsv`. The strange sequences sometimes displayed as `â€”` or `Â` by the current PowerShell console are not, by themselves, evidence that those sequences are stored in the files; repository searches did not find widespread mojibake strings. Encoding and Unicode normalization still need formal automated validation.

### 2.2 What is already good

- The library has a clear eight-collection layout.
- Every collection has a `README.md` and `00-sources.md`.
- Most files follow a consistent numbered naming scheme.
- The repository distinguishes descriptive evidence in `resources/` from future governed records in `data/` and normative rules in `spec/`.
- Several collections already record limitations and some manual-verification claims.
- The dictionary and wordlists have documented entry counts and a known 31-headword mismatch queue.
- Existing Git history provides a starting recovery mechanism for files already tracked.

### 2.3 Confirmed or strongly indicated problems

#### A. Raw OCR noise remains in content

The clearest high-risk batch is:

- `resources/naxwe/13-aasaaska-naxwaha.md`
- `resources/naxwe/14-naxwaha-cusub.md`
- `resources/naxwe/15-naxwaha-sifayneed.md`
- `resources/naxwe/16-weeraynta-soomaaliga.md`
- `resources/naxwe/17-naxwaha-af-soomaaliga.md`

Their own source inventory says approximately **0.9%–6.3% of lines per file** still contain raw OCR noise. Examples found during this audit include broken words, stray symbols, scan fragments, and damaged prose.

The highest-risk literary files, based on repeated suspicious symbols, broken-token patterns, and visible OCR fragments, include:

- `resources/suugaan/14-hal-karaan-hadrawi.md`
- `resources/suugaan/18-dugsiga-fasalka-4aad-buugga.md`
- `resources/suugaan/16-dugsiga-fasalka-1aad-1983.md`
- `resources/suugaan/11-hubsiimo-laan.md`
- `resources/suugaan/20-suugaanta-carruurta.md`
- `resources/suugaan/07-hal-ka-haleel.md`
- `resources/suugaan/08-qiso-kalgacal.md`
- `resources/suugaan/05-sheekooyin-laysku-soo-ururshay.md`
- `resources/suugaan/17-dugsiga-fasalka-4aad.md`
- `resources/suugaan/10-dhaartii-dhabta-ahayd.md`
- `resources/suugaan/12-bisaddii-bubaysta.md`
- `resources/suugaan/13-xeebtii-dahabka.md`
- `resources/suugaan/04-sheekooyin-soomaaliyeed.md`
- `resources/suugaan/15-dugsiga-fasalka-1aad-1976.md`
- `resources/suugaan/19-dugsiga-fasalka-5aad.md`
- `resources/suugaan/21-suugaanta-dhallaanka.md`

This is a heuristic priority list, not a judgment that every flagged line is wrong. Poetry, tables, decorative separators, abbreviations, historical spelling, and code-switching can legitimately trigger automated checks.

#### B. Source authentication is incomplete

- `qaamuus/00-sources.md` does not yet confirm compiler/editor, edition, publisher, or republication rights.
- Several `suugaan/` works have no confirmed year, and some author/edition data requires verification.
- Several supplementary `naxwe/` records have incomplete author/year data.
- `erey-bixin/09-farsamada-culuunta.md` is explicitly partial and needs a better source conversion before completion.
- The source PDFs or page images are not part of the current `resources/` layout, so claims of exact page fidelity cannot be independently reproduced from this folder alone.
- A statement such as “manually verified” is not enough without the verifier, date, edition, page coverage, method, and unresolved-item count.

#### C. The current policy conflicts with the requested outcome

Current documentation says both that `resources/` is a curated evidence library and that OCR artifacts should be preserved verbatim and corrected only downstream. That approach preserves conversion history, but it also allows corrupted text to appear as canonical evidence.

The cleanup must resolve this before content edits. The recommended policy is:

- preserve scans and raw OCR immutably;
- keep `resources/` as the readable, source-faithful transcription layer;
- keep linguistic normalization and normative decisions in `data/` or `spec/`;
- record every non-mechanical transcription correction in an audit log.

#### D. Quality assurance is not automated

`tools/` is currently empty except for `.gitkeep`, and the future `data/` subdirectories are empty. There is no reproducible validator for encoding, Markdown structure, dictionary entry shape, glossary delimiters, wordlist equivalence, duplicate content, page coverage, or unresolved OCR flags.

### 2.4 What “authentic source” will mean

A file is **authenticated** only when all of the following are true:

- its exact source edition is identified;
- the source scan or a lawful archival reference is recorded;
- the scan has a cryptographic checksum where a local copy is retained;
- the text is traceable to source page(s);
- OCR corrections were checked against the page image, not guessed from context alone;
- omissions, unreadable passages, supplied characters, and editorial interventions are marked;
- an independent reviewer completed the required review;
- automated validators pass;
- rights and permitted use are recorded;
- the file has an explicit status and review date.

“Authentic” means faithful to the identified source. It does not mean that every statement in the source is linguistically correct, current, or normative.

## 3. Target Information Model

### 3.1 Three evidence layers

```text
source scan/PDF  →  immutable raw OCR  →  verified transcription in resources/
                                              ↓
                           normalized/governed records in data/
                                              ↓
                               normative rules in spec/
```

1. **Source layer:** Original PDF or page images, never edited. Store only when rights allow; otherwise store an archival identifier, access note, and checksum obtained during review.
2. **Raw OCR layer:** Exact machine output, retained for reproducibility and comparison, never presented as authenticated text.
3. **Verified transcription layer:** Clean, readable, source-faithful Markdown/TSV in `resources/`.

Normalization—such as choosing a preferred spelling, merging duplicates, or correcting an apparent error printed in the source—belongs downstream in `data/`, with a link back to the transcription.

### 3.2 Required source manifest

Create a machine-readable manifest, preferably `resources/MANIFEST.tsv` or `data/provenance/sources.tsv`, with one record per source-file relationship.

Required fields:

| Field | Meaning |
| --- | --- |
| `resource_path` | Repository path of the transcription |
| `source_id` | Stable identifier such as `S-0001` |
| `title` | Exact title-page title |
| `author_compiler` | Exact credited author, compiler, or institution |
| `edition` | Edition or printing |
| `publisher` | Publisher/institution |
| `publication_place` | Place if printed |
| `year` | Confirmed year or explicit `unknown` |
| `language` | Source language(s) |
| `page_range` | Pages represented in the resource file |
| `source_location` | Lawful local path or stable archive/catalog identifier |
| `source_sha256` | SHA-256 of the reviewed scan when available |
| `rights_status` | Public domain, permission, licensed, restricted, or unresolved |
| `ocr_engine` | Engine/version if known |
| `ocr_date` | Conversion date if known |
| `transcription_status` | Status from the controlled vocabulary below |
| `reviewer` | Account/name or review record ID |
| `review_date` | ISO date |
| `unresolved_count` | Number of unresolved spans |
| `notes` | Scope, omissions, or exceptions |

### 3.3 Controlled status vocabulary

| Status | Meaning | Allowed as trusted evidence? |
| --- | --- | --- |
| `inventory-only` | File discovered; source not authenticated | No |
| `raw-ocr` | Machine conversion with no complete page review | No |
| `triaged` | Automated audit complete; defects are queued | No |
| `in-review` | Being compared with the scan | No |
| `verified` | Page comparison and first review complete | With caution |
| `authenticated` | Independent review, validation, provenance, and rights checks complete | Yes |
| `blocked` | Source/rights/page image missing or unreadable | No |
| `derived` | Reproducibly generated from authenticated input | Yes, if generation passes |

Status must be assigned per file, not only per collection.

### 3.4 Editorial notation

Adopt a small, documented notation and do not invent fixes silently.

- Exact readable source text: transcribe as printed.
- Clear OCR error with legible source: correct the OCR in the transcription and log the change.
- Source itself contains an apparent error: retain it; add a structured editorial note downstream if needed.
- Unreadable characters: use a consistent marker such as `⟦illegible⟧` with page reference.
- Supplied reading: use `⟦reading?⟧` only when policy permits, and keep it unresolved until reviewer confirmation.
- Omitted non-Somali or out-of-scope passage: mark the omission and page range in provenance rather than silently deleting it.
- Page boundaries: retain machine-readable page anchors during review, even if presentation builds hide them later.

## 4. Work Phases and Tasks

## Phase 0 — Governance, Safety, and Freeze

**Goal:** Prevent loss of evidence and agree on what may be changed.

### Tasks

- [x] Approve the three-layer evidence model.
- [x] Decide the lawful location for source PDFs/page images and raw OCR. Do not commit restricted scans to a public repository.
- [x] Freeze direct bulk edits to `resources/` until backups, hashes, and manifests exist.
- [x] Create a cleanup branch and tag or commit the pre-cleanup baseline.
- [x] Update `resources/README.md`, `docs/RESOURCES.md`, and `docs/ARCHITECTURE.md` so they no longer instruct maintainers to preserve OCR corruption inside the curated transcription.
- [x] Write `docs/TRANSCRIPTION_POLICY.md` covering fidelity, omissions, uncertain readings, page anchors, historical spelling, punctuation, tables, poetry, and non-Somali passages.
- [x] Write `docs/REVIEW_GUIDE.md` with reviewer roles and acceptance rules.
- [x] Define who can mark a file `authenticated`.
- [x] Confirm that corrections to source transcription are different from linguistic normalization.

### Exit criteria

- The preservation and correction policy is approved.
- Raw evidence is recoverable.
- No cleanup can silently destroy the original OCR or source mapping.

## Phase 1 — Complete Inventory and Provenance

**Goal:** Account for every file and every source before text correction.

### Tasks

- [x] Create the source manifest with all 145 current files.
- [x] Link every content file to one or more stable source IDs.
- [x] Locate the exact PDF/scan used for each OCR conversion.
- [x] Compute SHA-256 checksums for scans and raw OCR.
- [x] Record title-page metadata, edition, publisher, year, page count, and represented page range.
- [x] Record whether pages are missing, duplicated, out of order, cropped, skewed, blurred, or handwritten.
- [x] Record rights and redistribution status separately from factual provenance.
- [x] Resolve or explicitly mark `unknown` values; do not use vague “to be confirmed” text without an issue ID and owner.
- [x] Reconcile the manifest with every collection's `00-sources.md`.
- [x] Open tracked metadata issues for unresolved items, especially the dictionary and supplementary grammars.

### Priority metadata cases

- [x] `qaamuus/`: compiler/editor, edition, publisher, year, scan identity, and republication rights.
- [x] `naxwe/13`–`17`: author/compiler, year, edition, and exact page coverage.
- [x] `suugaan/`: missing years/editions and exact source identity for each book.
- [x] `erey-bixin/09-farsamada-culuunta.md`: exact source pages, omissions, and rights.
- [x] Supplements in `orthography/`, `phonology/`, and `morphology/`: confirm the distinction between primary, supplementary, and derived evidence.

### Exit criteria

- Every content file has a manifest record and source ID.
- Every unknown is explicit and tracked.
- No file proceeds to authentication without accessible source pages.

## Phase 2 — Build the Audit and Validation Tooling

**Goal:** Make defect discovery and regression checking reproducible.

### Tasks

- [ ] Add a read-only inventory command under `tools/` that reports file counts, byte counts, line counts, and hashes.
- [ ] Add strict UTF-8 validation and Unicode normalization reporting. Report normalization differences; do not auto-normalize without review.
- [ ] Detect replacement characters, control characters, mojibake signatures, non-breaking spaces, zero-width characters, and mixed line endings.
- [ ] Add Markdown checks: one H1, heading hierarchy, valid tables, balanced emphasis, malformed list items, and accidental page furniture.
- [ ] Add OCR heuristics for broken words, isolated glyphs, repeated punctuation, long garbage runs, interleaved columns, headers/footers, page numbers, hyphenation at line breaks, and suspicious digit/letter substitutions.
- [ ] Add Somali-aware candidate checks for common OCR confusions, but emit review suggestions only.
- [ ] Add collection-specific validators:
  - [ ] `qaamuus/`: entry grammar, headword extraction, homonym markers, grammatical codes, `ld`/`eeg` targets, and alphabetical placement.
  - [ ] `wordlists/`: one headword per line, duplicates, sort order, and reproducible equivalence with `qaamuus/`.
  - [ ] `naxwe/`: TSV column count, table shape, example markers, and heading integrity.
  - [ ] `erey-bixin/`: exactly one intentional term delimiter per record, section integrity, duplicates, and empty source/target sides.
  - [ ] `suugaan/`: verse/prose structure, speaker labels, chapter headings, and preservation of deliberate lineation.
  - [ ] `orthography/`, `phonology/`, `morphology/`: table shape, symbols, examples, cross-references, and source notes.
- [ ] Produce machine-readable findings with stable IDs, severity, file, line/page, rule, suggested action, assignee, and resolution.
- [ ] Add CI checks that block new encoding errors, unreviewed bulk changes, broken provenance, and validator regressions.

### Exit criteria

- A clean audit can be rerun from one documented command.
- Findings are reproducible and suppressions require a reason.
- Automated suggestions never modify source text without review.

## Phase 3 — Establish a Gold Sample and Calibrate the Process

**Goal:** Prove the workflow on a representative sample before bulk cleanup.

### Sample set

Choose at least 20 pages covering:

- clean prose;
- severe OCR noise;
- dictionary columns;
- glossary tables;
- grammar tables and examples;
- poetry with meaningful line breaks;
- old print, damaged scans, and unusual fonts;
- apostrophes, curly quotation marks, long vowels, `c`, `x`, `kh`, `sh`, and `dh`;
- page headers, footers, footnotes, and multi-column layouts.

Include pages from `naxwe/15`, `naxwe/17`, `suugaan/14`, `suugaan/18`, `qaamuus/`, and `erey-bixin/`.

### Tasks

- [ ] Independently transcribe the sample from page images.
- [ ] Compare current text, raw OCR, and gold transcription.
- [ ] Classify errors: substitution, insertion, deletion, spacing, punctuation, line order, layout, metadata, or omission.
- [ ] Measure character error rate and word error rate where meaningful.
- [ ] Calibrate automated rules against false positives in valid Somali and poetry.
- [ ] Test the editorial notation and correction log.
- [ ] Estimate review effort by document type and risk level.
- [ ] Obtain reviewer sign-off on the workflow before scaling it.

### Exit criteria

- Reviewers apply the policy consistently.
- High-impact defects are detected without unacceptable false positives.
- The correction log can reconstruct what changed and why.

## Phase 4 — High-Risk OCR Remediation

**Goal:** Repair material that is currently unsafe to cite as clean text.

### Batch 4A: supplementary grammar

- [ ] Review `naxwe/13-aasaaska-naxwaha.md` page by page.
- [ ] Review `naxwe/14-naxwaha-cusub.md` page by page.
- [ ] Review `naxwe/15-naxwaha-sifayneed.md` page by page.
- [ ] Review `naxwe/16-weeraynta-soomaaliga.md` page by page.
- [ ] Review `naxwe/17-naxwaha-af-soomaaliga.md` page by page.
- [ ] Reconstruct tables and column order only from the source layout.
- [ ] Remove scan debris, headers, footers, and duplicate page fragments while logging removals.
- [ ] Mark files `blocked` if a legible source scan cannot be found.

### Batch 4B: highest-risk literature and textbooks

- [ ] Review `suugaan/14-hal-karaan-hadrawi.md` with special protection for verse boundaries.
- [ ] Review `suugaan/18-dugsiga-fasalka-4aad-buugga.md`.
- [ ] Review `suugaan/16-dugsiga-fasalka-1aad-1983.md`.
- [ ] Review `suugaan/11-hubsiimo-laan.md`.
- [ ] Review `suugaan/20-suugaanta-carruurta.md`.
- [ ] Review `suugaan/07-hal-ka-haleel.md`.
- [ ] Review `suugaan/08-qiso-kalgacal.md`.
- [ ] Review `suugaan/05-sheekooyin-laysku-soo-ururshay.md`.
- [ ] Review the remaining flagged school texts, stories, and children's books in risk order.
- [ ] Preserve intentional dialect, historical spelling, punctuation, repetitions, and poetic devices.

### Batch 4C: partial terminology source

- [ ] Reacquire or reconvert the best available source for `erey-bixin/09-farsamada-culuunta.md`.
- [ ] Compare the current partial set against every represented source page.
- [ ] Restore readable omitted records where rights and source quality allow.
- [ ] Identify Italian-only or unreadable records explicitly rather than silently excluding them.
- [ ] Keep the file `partial`/`blocked` until its declared coverage matches reality.

### Per-file review loop

For every file in Phase 4:

1. confirm scan hash and page range;
2. run automated triage;
3. align resource sections to pages;
4. correct only source-verifiable OCR/conversion errors;
5. log each substantive correction;
6. mark unresolved spans;
7. run validators;
8. perform a first full review;
9. perform an independent review of all high-risk pages and a defined sample of clean pages;
10. update manifest status and collection documentation.

### Exit criteria

- No severe OCR garbage remains unmarked.
- All represented pages were checked.
- All unresolved spans are counted and traceable.
- Independent review and automated checks pass.

## Phase 5 — Structured Collections and Derived Data

**Goal:** Validate large regular datasets without damaging their evidentiary form.

### Batch 5A: dictionary (`qaamuus/`)

- [ ] Authenticate the dictionary edition before treating entries as final evidence.
- [ ] Parse all `01-b.md`–`31-uu.md` entries into temporary structured records.
- [ ] Validate all grammatical codes against `00-abbreviations.md`.
- [ ] Detect malformed entry starts, lost line wraps, duplicate entries, suspicious alphabet placement, and broken homonym superscripts.
- [ ] Validate every `ld` and `eeg` target; report missing or ambiguous targets.
- [ ] Compare suspicious definitions and examples directly with source pages.
- [ ] Review legitimate historical/variant headwords separately from OCR candidates.
- [ ] Correct the malformed abbreviation table row(s) only after checking the source/key.
- [ ] Record the authenticated entry count after cleanup; do not assume the current 42,511 count is final.

### Batch 5B: wordlists (`wordlists/`)

- [ ] Treat all `01-b.md`–`26-u.md` files as generated derivatives, not separately hand-maintained sources.
- [ ] Resolve the documented 31 unmatched headwords against the authenticated dictionary.
- [ ] Define deterministic extraction, normalization, deduplication, and ordering rules.
- [ ] Regenerate the wordlists from authenticated dictionary records.
- [ ] Add a build check that fails when committed wordlists differ from generated output.

### Batch 5C: terminology (`erey-bixin/01`–`08`)

- [ ] Parse every term pair and validate delimiters and empty fields.
- [ ] Review duplicates, conflicting equivalents, suspicious OCR tokens, and broken line wraps against source pages.
- [ ] Preserve source variants; put preferred-term decisions in `data/terminology/`.
- [ ] Validate scientific names in `08-magacyada-dhirta.md` against the printed source first; external taxonomy updates must be separate annotations, not source-text replacements.

### Batch 5D: grammar glossary

- [ ] Validate `naxwe/ereyfur.tsv` as strict UTF-8 TSV.
- [ ] Confirm column count, headers, empty values, duplicates, and source order.
- [ ] Page-check every suspicious or structurally invalid record.

### Exit criteria

- Regular collections parse without unexplained errors.
- Derived wordlists are reproducible.
- Cross-references and entry counts have auditable reports.

## Phase 6 — Review the Currently “Clean” Collections

**Goal:** Verify prior claims rather than accepting them without evidence.

### Tasks

- [ ] Audit `orthography/01`–`06`, including all 12 documented punctuation sections.
- [ ] Audit `phonology/01`–`08`, with special handling for IPA and other non-ASCII symbols.
- [ ] Reproduce every `morphology/01`–`04` table from its cited `naxwe/` sections.
- [ ] Review primary `naxwe/00`–`12` by source-aligned sampling, escalating to full review if the sample fails.
- [ ] Review lower-risk `suugaan/` files by stratified page samples; promote any failed file to full page-by-page review.
- [ ] Verify that claims such as “manually verified” include evidence in the manifest and review log.
- [ ] Check cross-collection duplication and citations without forcing textual uniformity across different sources.

### Sampling rule

For a file believed clean, review at minimum:

- first and last page;
- every page with an automated flag;
- every page containing a table, poem, footnote, unusual symbol, or multi-column layout;
- a random sample of remaining pages, with the sample size recorded.

Any severe error, repeated error class, missing page, or wrong line order fails the sample and triggers full review.

### Exit criteria

- Every “verified” claim has a reproducible review record.
- Failed samples have been escalated.
- IPA, tables, punctuation examples, and paradigms remain semantically intact.

## Phase 7 — Independent Linguistic and Editorial QA

**Goal:** Catch plausible-looking OCR errors that structural tools cannot detect.

### Roles

- **Transcriber:** Corrects conversion errors from source pages.
- **Somali-language reviewer:** Checks words, morphology, sentence continuity, names, and variants against the page.
- **Domain reviewer:** Checks grammar, terminology, dictionary notation, poetry, or phonology as appropriate.
- **Provenance/rights reviewer:** Confirms edition, source identity, and permitted use.
- **Maintainer:** Runs validation and approves status changes.

One person may fill multiple roles for low-risk files, but the transcriber must not be the only approver of a high-risk file.

### Tasks

- [ ] Review corrections in context and against page images.
- [ ] Confirm that historical spelling and dialect were not normalized.
- [ ] Confirm that poetry lineation, refrain, speaker changes, and punctuation were preserved.
- [ ] Confirm that dictionary codes and cross-references retain source meaning.
- [ ] Confirm technical term pairs did not shift columns or languages.
- [ ] Resolve or retain every uncertainty marker.
- [ ] Require reasoned approval for bulk replacements.
- [ ] Record reviewer identity, date, pages checked, and outcome.

### Exit criteria

- No high-risk file is self-approved.
- All substantive changes have source evidence.
- Review records are complete.

## Phase 8 — Release, Documentation, and Ongoing Control

**Goal:** Publish a trustworthy baseline and prevent regression.

### Tasks

- [ ] Run the complete inventory, provenance, structure, encoding, and collection-specific test suite.
- [ ] Generate a final quality report by file and collection.
- [ ] Update all collection `README.md` and `00-sources.md` status statements.
- [ ] Update `docs/RESOURCES.md`, `PROJECT_OVERVIEW.md`, `ROADMAP.md`, and `CHANGELOG.md`.
- [ ] Publish counts of authenticated, verified, raw-OCR, partial, and blocked files.
- [ ] Publish unresolved issues without overstating coverage.
- [ ] Create a versioned release/tag for the authenticated baseline.
- [ ] Require source manifest updates and validators for every future resource change.
- [ ] Schedule periodic link, rights, checksum, and regression audits.

### Exit criteria

- The release can be rebuilt and audited.
- Every distributed file exposes an accurate trust status.
- New OCR-derived material cannot enter `resources/` without source identity and review status.

## 5. File Work Matrix

| Scope | Files covered | Required action | Priority |
| --- | --- | --- | --- |
| Collection documentation | `resources/README.md`; all eight collection `README.md` and `00-sources.md` files | Reconcile policy, provenance, status, and audit claims | P0 |
| Grammar supplements | `naxwe/13`–`17` | Full page-by-page transcription review | P0 |
| High-risk literature | Flagged `suugaan/04`–`21`, beginning with `14`, `18`, `16`, `11`, `20`, `07`, `08`, `05` | Risk-ordered full review | P0 |
| Partial terminology | `erey-bixin/09-farsamada-culuunta.md` | Reacquire/reconvert source and declare exact coverage | P0 |
| Dictionary | `qaamuus/00-abbreviations.md`, `01`–`31` | Parse, validate, page-check anomalies, authenticate edition | P1 |
| Wordlists | `wordlists/01`–`26` | Regenerate from authenticated dictionary; resolve 31 mismatches | P1 |
| Other terminology | `erey-bixin/01`–`08` | Parse, validate, page-check anomalies | P1 |
| Grammar glossary | `naxwe/ereyfur.tsv` | Strict TSV and source validation | P1 |
| Primary grammar | `naxwe/00`–`12` | Structured and sampled source review; escalate on failure | P1 |
| Remaining literature | All `suugaan/` files not promoted to P0 | Stratified review; escalate on failure | P1/P2 |
| Orthography | `orthography/01`–`06` | Reproduce prior verification record and sample/full audit | P2 |
| Phonology | `phonology/01`–`08` | Reproduce prior verification; protect IPA/symbols | P2 |
| Morphology | `morphology/01`–`04` | Re-derive tables and verify citations | P2 |

No file is out of scope. File ranges in this matrix include each numbered Markdown file between the stated endpoints. Metadata files are handled in Phases 0, 1, and 8.

## 6. Quality Gates

### Gate A — Preservation

- Source scan/raw OCR is retained or lawfully referenced.
- Hash and exact source identity are recorded.
- Pre-cleanup state is recoverable.

### Gate B — Technical validity

- Strict UTF-8 passes.
- No unexplained replacement/control/zero-width characters remain.
- Markdown or TSV structure passes.
- Collection-specific parser passes.

### Gate C — Source fidelity

- Page coverage is complete and ordered.
- Headers, footers, columns, footnotes, tables, and lineation are handled correctly.
- Every correction is supported by the scan.
- Unreadable and omitted text is explicit.

### Gate D — Human review

- Required Somali/domain review is complete.
- High-risk files have independent approval.
- Reviewer, date, page scope, and unresolved count are recorded.

### Gate E — Release readiness

- Provenance and rights fields are complete or explicitly blocked.
- Cross-file/derived-data checks pass.
- Documentation reports the true status without claiming that partially reviewed material is authenticated.

## 7. Metrics and Reporting

Track progress by evidence quality, not only by number of edited lines.

| Metric | Baseline | Target |
| --- | --- | --- |
| Files inventoried | Partial collection inventories | 145/145 current files |
| Files with exact source ID and edition | Not yet measured; known gaps | 100% or explicitly `blocked` |
| Files with page coverage | Not consistently recorded | 100% of transcriptions |
| Files with status | Collection-level, mixed | 100% file-level |
| Strict UTF-8 failures | To be measured by tool | 0 |
| Unexplained severe OCR findings | Present | 0 in authenticated files |
| Unresolved spans | Not centrally counted | Counted per file; 0 for `authenticated` unless policy allows documented exceptions |
| Dictionary cross-reference failures | Not measured | 0 unexplained |
| Wordlist/dictionary mismatch | 31 known | 0 unexplained; generated output reproducible |
| High-risk files with independent review | 0 recorded centrally | 100% |
| Files with rights status | Incomplete | 100% recorded, including `unresolved` |

Each cleanup pull request should report:

- files and source pages reviewed;
- correction counts by error class;
- unresolved spans;
- validator results;
- reviewer(s);
- provenance changes;
- whether any source wording was intentionally retained despite appearing incorrect.

## 8. Risks and Mitigations

| Risk | Mitigation |
| --- | --- |
| AI or spellchecker “corrects” valid Somali | Treat automated output only as candidates; require source-page verification |
| Historical spelling or dialect is normalized | Preserve printed form; move preferences to `data/` |
| Raw evidence is lost | Immutable scans/raw OCR, hashes, baseline tag, correction logs |
| Wrong PDF edition is used | Verify title page, colophon, pagination, and checksum |
| Poetry layout is damaged | Use genre-specific review and lineation checks |
| Multi-column pages are read in wrong order | Page-layout review and column-aware OCR/alignment |
| Rights prevent scan distribution | Keep restricted storage outside public Git; publish metadata/checksum/status only |
| Bulk replacement creates false corrections | Require dry-run report, bounded rules, source samples, and reviewer approval |
| “Verified” status is overstated | File-level status, explicit gates, independent review records |
| Review becomes unmanageable | Work in risk-ranked batches and use automation only for triage/regression |
| Source is unreadable or missing | Mark file/span `blocked`; never guess and publish as authenticated |

## 9. Definition of Done

The `resources/` cleanup is complete when:

- all 145 current files are represented in the manifest;
- every content file is linked to an exact source and page range, or marked `blocked`;
- source scans/raw OCR are preserved or reproducibly referenced;
- all P0 files have completed page-by-page review;
- P1/P2 files have passed the required full or sampled review, with failures escalated;
- no authenticated file contains unexplained severe OCR corruption;
- encoding, structure, collection parsers, cross-references, and derived-data tests pass;
- every substantive correction is auditable;
- every high-risk file has independent review;
- rights and permitted-use status are recorded;
- wordlists are reproducibly derived from the authenticated dictionary;
- repository documentation accurately distinguishes raw OCR, verified transcription, authenticated source evidence, normalized data, and normative standards;
- a versioned quality report and release identify all remaining limitations.

## 10. Immediate Next Work Package

The first implementation package should be small and reversible:

1. approve the preservation/transcription policy;
2. create the manifest schema and populate all file paths;
3. locate and hash the source scans for `naxwe/15`, `naxwe/17`, `suugaan/14`, and `suugaan/18`;
4. implement read-only UTF-8, structure, and OCR-triage reports;
5. build the 20-page gold sample;
6. clean one grammar sample and one poetry/prose sample through both reviewer stages;
7. revise thresholds and documentation from the pilot results;
8. begin the remaining P0 batches only after the pilot passes.

This sequence establishes evidence preservation and review discipline before any large-scale correction is made.
