# Resource OCR Cleanup — Phase 1 Report

**Phase:** 1 — Complete Inventory and Provenance  
**Date completed:** 2026-08-09  
**Branch:** `resource-ocr-cleanup`  
**Result:** Complete; awaiting maintainer/user review before Phase 2

## 1. Phase objective

Account for every current `resources/` file and its claimed source before any
book/OCR text is corrected. Phase 1 performed provenance inventory only. It did
not generate, rewrite, normalize, or correct source wording.

## 2. Outcome summary

| Measure | Result |
| --- | ---: |
| Files under `resources/` at inventory boundary | 145 |
| Manifest source-file relationship rows | 146 |
| Direct source works provisionally identified | 45 |
| Internal project-metadata source records | 1 |
| Source-dependent unique files | 128 |
| Internal metadata files | 17 |
| Exact PDFs/page-image sets found | 0 |
| Direct sources with source SHA-256 | 0 |
| Source-dependent files marked `blocked` | 128 |
| Files marked `authenticated` | 0 |
| Numbered book/OCR content files changed | 0 |

There are 146 relationships for 145 files because
`resources/erey-bixin/05-xisaab.md` contains material from two declared source
works (`SRC-EB-005` and `SRC-EB-006`).

## 3. Tasks completed

### Task 1.1 — Define controlled provenance data

**Issue:** Provenance existed only as prose distributed among collection notes.
Missing values were expressed inconsistently as dashes or “to be confirmed.”

**Action:** Added `data/provenance/README.md` with stable source-ID rules,
controlled missing values, and status rules. Existing `S-003` was retained;
new work IDs use `SRC-<COLLECTION>-NNN`.

**Improvement:** `unknown`, `not_applicable`, `not_available_in_workspace`, and
`unassigned` now have distinct meanings. An ID identifies a work but does not
claim that its edition or transcription is authenticated.

**Verification:** Schema documentation exists, required links resolve, and all
manifest fields are populated with values rather than blanks.

### Task 1.2 — Inventory direct sources

**Issue:** No central source catalog connected titles, authors, dates, source
availability, rights, OCR provenance, and outstanding evidence.

**Action:** Added `data/provenance/sources.tsv` containing 45 direct works and
one internal project-metadata record. Existing collection claims were preserved
as `unverified`; missing facts were not inferred.

**Improvement:** Every direct source has a unique stable ID, controlled
availability/rights/metadata fields, and linked issue IDs.

**Verification:** 46 rows, 17 columns, zero duplicate IDs, zero blank fields,
and all 45 direct IDs are used by the resource manifest.

### Task 1.3 — Inventory all resource files and relationships

**Issue:** There was no exact machine-readable inventory of the 145 current
files, their hashes, source relationships, review state, or unresolved status.

**Action:** Added `data/provenance/resource-manifest.tsv` with required
bibliographic/provenance fields plus relationship type, file byte count, and
resource SHA-256. It contains one row per source-file relationship.

**Improvement:** Every current file is now detectably missing, changed, or
unexpected when compared with the manifest. Multi-source and derived
relationships are explicit.

**Verification:** 146 rows represent 145 unique paths; actual and manifest paths
match exactly; zero missing/extra paths; zero blank fields; zero unknown source
IDs; all 145 current file hashes match. The only repeated path is the expected
two-source mathematics glossary.

### Task 1.4 — Locate exact source scans

**Issue:** Authentication requires the exact PDF/page images, but repository
search found none.

**Action:** Searched the workspace for PDF, PNG, JPEG, and TIFF source evidence.
Recorded all 45 direct source locations as
`not_available_in_workspace`, opened M-100, and marked dependent files
`blocked`.

**Improvement:** Absence is now explicit and cannot be mistaken for completed
source verification. Future scans have a lawful Git-ignored location defined in
Phase 0.

**Verification:** All 45 direct catalog rows have the unavailable location; all
128 source-dependent unique files are blocked from authentication.

### Task 1.5 — Record source and OCR checksums

**Issue:** No source scan or separate raw-OCR file was available to hash.

**Action:** Recorded source SHA-256 as `unavailable` and opened M-102. Computed
SHA-256 for every current resource file so the inventory boundary itself is
fixed.

**Improvement:** The absence of source hashes is visible, while all current
repository resource files now have verified inventory hashes.

**Verification:** Zero direct sources falsely claim a scan hash; 145/145
resource hashes were recomputed and matched.

### Task 1.6 — Record bibliographic and page metadata

**Issue:** Title-page/colophon identity, edition, publisher, publication place,
page count, represented page range, and physical page conditions cannot be
verified without scans.

**Action:** Retained only existing collection-note claims as `unverified` and
recorded unavailable facts as `unknown`. Page ranges and page-condition details
remain explicitly unknown under M-101 rather than guessed.

**Improvement:** Known claims and unavailable evidence are separated. No OCR
body text was used to manufacture bibliography.

**Verification:** Every source and relationship field is nonblank; no
source-dependent file has a false page range or review claim.

### Task 1.7 — Record rights separately

**Issue:** Bibliographic identity and permission to possess/process/redistribute
a source are different questions, but prior notes mixed or omitted them.

**Action:** Added a dedicated `rights_status` field. All 45 direct sources remain
`unresolved` under M-104; dictionary-specific republication evidence remains
M-003.

**Improvement:** A correct title or source location can no longer be mistaken
for legal permission.

**Verification:** Rights-status count is 45 `unresolved`; no unsupported license
was assigned.

### Task 1.8 — Create an owned metadata issue queue

**Issue:** Existing M-001/M-003/M-015 references had no complete local queue,
and many other gaps lacked stable tracking or ownership.

**Action:** Added `docs/resource-cleanup/METADATA_ISSUES.md` with 17 issues,
priority, coordinating owner role, scope, missing evidence, closure condition,
and status.

**Improvement:** Dictionary, supplementary grammar, literature, terminology,
rights, pages, checksums, OCR history, prior review claims, and derived wordlist
gaps are explicitly tracked.

**Verification:** All 16 issue IDs referenced by source rows exist in the queue;
M-118 additionally tracks the known wordlist mismatch. There are no missing or
orphaned IDs.

### Task 1.9 — Reconcile collection inventories

**Issue:** Collection source documents lacked stable IDs and could overstate
authentication through terms such as “clean extraction” or “manually verified.”

**Action:** Added a Phase 1 provenance section to all eight `00-sources.md`
files, linked source IDs/manifests/issues, and made scan absence and blocked
status explicit. Replaced vague dash/confirmation placeholders with issue-linked
`unknown` values.

**Improvement:** Human-readable and machine-readable provenance now agree.
Earlier structural/manual claims are not treated as reproducible authentication.

**Verification:** All eight inventories contain the Phase 1 section; central
resource documentation links the manifest; no vague placeholder table values
remain in the source inventories.

### Task 1.10 — Confirm scope isolation

**Issue:** Resource cleanup must not implement roadmap milestones, normative
standards, or unrelated project work.

**Action:** Limited changes to resource metadata, provenance data, and cleanup
documentation.

**Improvement:** The branch remains narrowly reviewable as resource-authentication
work.

**Verification:** No changes exist in `ROADMAP.md`, `IMPLEMENTATION_PLAN.md`,
`spec/`, or `standards/`. No numbered resource content file changed.

## 4. Priority metadata cases

| Scope | Result |
| --- | --- |
| `qaamuus/` | `SRC-QAA-001`; compiler, edition, publisher, year, scan, and rights explicitly unknown/open under M-001, M-003, M-015, M-100–M-104 |
| `naxwe/13`–`17` | Stable IDs assigned; missing authors/years/editions/page coverage tracked by M-110 and common evidence issues |
| `suugaan/` | 24 source IDs assigned; missing years/editions and textbook credits tracked by M-111/M-112 |
| `erey-bixin/09` | `SRC-EB-010`; partial coverage and missing source/pages/rights tracked by M-115 and common evidence issues |
| Supplements/derivatives | Primary, supplementary summary, integrated extract, headword derivative, and paradigm derivative relationships distinguished |

## 5. Errors encountered during implementation

| Error | Impact | Resolution and verification |
| --- | --- | --- |
| First manifest build produced `SRC-NAX-13` instead of zero-padded `SRC-NAX-013` | Build stopped before writing the manifest | Corrected mapping to three-digit IDs; rerun resolved all source IDs and produced 145-file coverage |
| First collection-status verification used an invalid PowerShell pipeline form | No files changed; verification command failed | Wrapped the generated object list before formatting; all eight source inventories passed |
| A Windows wildcard was passed literally to `rg` during placeholder checking | That check produced an I/O warning; no file impact | Re-ran with `rg`'s `-g '00-sources.md'` filter; no vague placeholders found |
| Source-inventory docs changed after initial resource hashes were generated | Metadata-file hashes became stale during implementation | Regenerated the complete manifest after reconciliation and rechecked all hashes |
| Final gate found trailing whitespace on the updated cleanup-plan status line | Formatting check failed; no data or source-text impact | Removed the two trailing spaces and reran the complete phase gate |

## 6. Files added or changed

### Added

- `data/provenance/README.md`
- `data/provenance/sources.tsv`
- `data/provenance/resource-manifest.tsv`
- `docs/resource-cleanup/METADATA_ISSUES.md`
- `docs/resource-cleanup/PHASE_1_REPORT.md`

### Updated

- `resources/README.md`
- all eight `resources/*/00-sources.md` files
- `docs/RESOURCES.md`
- `RESOURCE_OCR_CLEANUP_PLAN.md`

No roadmap, implementation-plan, normative specification, standards registry,
or numbered book/OCR content file was changed.

## 7. Exit-criteria verification

| Exit criterion | Result | Evidence |
| --- | --- | --- |
| Every content file has a manifest record and source ID | Pass | 145 unique manifest paths exactly match 145 actual files; 146 valid source relationships |
| Every unknown is explicit and tracked | Pass at Phase 1 inventory level | Controlled values, 17 owned issue records, and no blank manifest/catalog fields |
| No file proceeds to authentication without accessible source pages | Pass | All 128 source-dependent files are `blocked`; zero files are `authenticated` |

## 8. Remaining limitations and Phase 2 input

- Exact authentic source PDFs/page images must be supplied or lawfully located.
- No source title, edition, page range, checksum, rights status, or prior manual
  review can be authenticated from the current workspace alone.
- Physical page defects remain unknown because pages cannot be inspected.
- OCR engines, versions, conversion settings, and dates remain unknown.
- Phase 2 should automate validation of these static TSV records and detect
  resource/hash drift; it must not attempt to repair source text.

Phase 2 must not begin until this report and the manifests are reviewed and
accepted.
