# Resource OCR Cleanup — Phase 0 Report

**Phase:** 0 — Governance, Safety, and Freeze  
**Date completed:** 2026-08-09  
**Branch:** `resource-ocr-cleanup`  
**Result:** Complete; awaiting maintainer/user review before Phase 1

## 1. Phase objective

Prevent loss of authentic evidence and establish binding rules before any OCR
text is corrected. Phase 0 did not correct or rewrite any book content.

## 2. Baseline audit

| Check | Finding |
| --- | --- |
| Current cleanup branch | `resource-ocr-cleanup` |
| Pre-cleanup commit | `f04354c608f9f2028b9d101e27dde01181a6f910` |
| Recovery tag | `resources-pre-ocr-cleanup-2026-08-09` |
| PDFs/page images found in workspace | 0 |
| Source-image extensions searched | PDF, PNG, JPEG, TIFF (case-appropriate repository search) |
| Existing raw-OCR policy | Conflicted with the goal of a clean authenticated transcription |
| Numbered resource content changed in Phase 0 | 0 |

The baseline tag resolves to the exact pre-cleanup commit. This preserves the
complete tracked state of `resources/` before future content changes.

## 3. Tasks completed

### Task 0.1 — Approve the evidence model

**Issue:** The repository treated `resources/` as curated evidence while also
requiring OCR artifacts to remain there verbatim. That made corrupted
conversion output look like canonical source evidence.

**Action:** Adopted five clearly separated roles:

```text
original scan/PDF -> immutable raw OCR -> verified resources/ transcription
                                                |
                                                v
                                  governed data/ records
                                                |
                                                v
                                    normative spec/ rules
```

**Improvement:** OCR defects may now be removed from the verified transcription
only when the source page proves the correction. Normalization, modernization,
translation, and new wording remain outside the transcription.

**Verification:** The model is stated consistently in
`docs/TRANSCRIPTION_POLICY.md`, `docs/RESOURCES.md`, and
`docs/ARCHITECTURE.md`.

### Task 0.2 — Define lawful source and raw-OCR storage

**Issue:** No original PDF, page image, or dedicated raw-OCR storage existed in
the workspace. Restricted books must not be accidentally committed publicly.

**Action:** Added `source-evidence/README.md` and Git-ignore rules for:

- `source-evidence/originals/`
- `source-evidence/raw-ocr/`
- `source-evidence/work/`

**Improvement:** The project now has an explicit local-only location for lawful
copies and immutable OCR evidence, with rules for checksums, access
restrictions, and external archive references.

**Verification:** `git check-ignore -v` confirmed all three evidence locations
are ignored, while `source-evidence/README.md` remains trackable.

### Task 0.3 — Freeze unsafe content edits

**Issue:** Bulk cleanup could otherwise begin before sources, hashes, page
mappings, and manifests exist.

**Action:** The transcription policy prohibits guessed changes, global spelling
correction, and unverified automation. It requires a recoverable state, dry-run
reports for bounded bulk operations, source-page evidence, and diff review.

**Improvement:** Automated systems may find candidates but cannot generate or
approve replacement text. Direct resource correction is frozen until its
source evidence is available and the sequential review workflow is followed.

**Verification:** Required prohibitions and evidence rules were searched and
confirmed in `docs/TRANSCRIPTION_POLICY.md`.

### Task 0.4 — Create cleanup branch and pre-cleanup recovery point

**Issue:** The work needed isolation and an unambiguous pre-cleanup state.

**Action:** Work is on `resource-ocr-cleanup`. Created annotated tag
`resources-pre-ocr-cleanup-2026-08-09` at commit
`f04354c608f9f2028b9d101e27dde01181a6f910`.

**Improvement:** Every future transcription diff can be compared with or
recovered from the named baseline.

**Verification:** `git rev-list -n 1 resources-pre-ocr-cleanup-2026-08-09`
returned the expected baseline commit.

### Task 0.5 — Reconcile repository documentation

**Issue:** Obsolete instructions in `resources/README.md`,
`docs/RESOURCES.md`, `resources/wordlists/README.md`, and
`resources/qaamuus/00-sources.md` required OCR artifacts or corrections to stay
downstream, contradicting source-faithful cleanup.

**Action:** Updated those documents and `docs/ARCHITECTURE.md` to reference the
new transcription and review controls.

**Improvement:** Documentation now distinguishes correction of source conversion
errors from linguistic normalization. Wordlists are designated for regeneration
from a verified dictionary rather than independent guessing.

**Verification:** Repository search found no remaining instances of the obsolete
instructions “OCR artifacts preserved verbatim,” “fix downstream, not here,” or
the equivalent hand-edit prohibition. Only resource metadata/documentation was
changed; no numbered content file was edited.

### Task 0.6 — Establish the transcription policy

**Issue:** The project lacked one binding rule for source fidelity, unreadable
spans, page mapping, poetry, tables, historical spelling, specialist symbols,
non-Somali passages, and automation.

**Action:** Added `docs/TRANSCRIPTION_POLICY.md`.

**Improvement:** Reviewers now have explicit rules that prohibit invented text
and require visible source evidence. The policy protects historical/dialectal
forms, poetic lineation, dictionary/table relationships, and specialist
notation.

**Verification:** Content assertions and Markdown whitespace checks passed.

### Task 0.7 — Establish roles and acceptance rules

**Issue:** Prior statements such as “manually verified” did not identify who
reviewed which pages, by what method, or who could authenticate a file.

**Action:** Added `docs/REVIEW_GUIDE.md` with roles, risk levels, a sequential
per-file workflow, page checklist, collection-specific checks, correction-log
fields, rejection rules, and acceptance records.

**Improvement:** Only a project maintainer may assign `authenticated`, and only
after source, review, validation, provenance, and rights requirements pass.
High-risk files require a Somali-language reviewer other than the transcriber.

**Verification:** The guide contains the authentication authority, independent
review rule, sequential workflow, rejection triggers, and required acceptance
record.

### Task 0.8 — Separate transcription correction from normalization

**Issue:** “Fixing” OCR could be confused with choosing preferred Somali forms
or changing an author's printed language.

**Action:** Defined transcription correction as repairing only source-proven OCR
or conversion defects. Defined normalization as a downstream governed-data
operation.

**Improvement:** Authentic source wording remains protected even when it is
historical, dialectal, inconsistent, or appears linguistically incorrect.

**Verification:** The distinction is present in all three policy/architecture
documents and in the updated resource convention.

## 4. Errors encountered during implementation

| Error | Impact | Resolution and verification |
| --- | --- | --- |
| The built-in Windows patch sandbox refused an edit before writing any file | No repository change from the failed attempt | Used the same approved patch engine through its repository-safe executable; subsequent diffs and status were inspected |
| An initial quiet-search assertion treated empty output as failure even when the search exit code was success | No file impact; verification display was wrong | Re-ran assertions using the command exit code; all required policy statements passed |
| Git could not initially write the branch/tag ref under read-only `.git` permissions | No partial ref was created | Used authorized Git ref access; branch and annotated tag were then verified |

## 5. Files added or changed

### Added

- `RESOURCE_OCR_CLEANUP_PLAN.md`
- `docs/TRANSCRIPTION_POLICY.md`
- `docs/REVIEW_GUIDE.md`
- `docs/resource-cleanup/PHASE_0_REPORT.md`
- `source-evidence/README.md`

### Updated

- `.gitignore`
- `resources/README.md`
- `resources/qaamuus/00-sources.md`
- `resources/wordlists/README.md`
- `docs/RESOURCES.md`
- `docs/ARCHITECTURE.md`

## 6. Exit-criteria verification

| Exit criterion | Result | Evidence |
| --- | --- | --- |
| Preservation and correction policy approved for implementation | Pass | Transcription policy, review guide, and aligned architecture documents |
| Raw evidence is recoverable | Pass for current tracked corpus | Named tag resolves to exact pre-cleanup tracked resource state; local immutable locations are defined for future original/raw files |
| Cleanup cannot silently destroy original OCR or source mapping | Pass at policy/governance level | Baseline tag, ignored immutable evidence locations, mandatory page mapping, correction log, diff review, and independent approval |

## 7. Remaining limitations and Phase 1 input

- No authentic PDFs/page images are currently present in the workspace.
- No separate original raw-OCR exports were found; the pre-cleanup tracked
  corpus is preserved by the baseline tag.
- File-level source IDs, page ranges, checksums, rights states, and review status
  still need to be created in Phase 1.
- No file is newly declared `authenticated` by Phase 0.
- No OCR/source text can be corrected until its exact source is located and
  registered.

Phase 1 must start by creating the manifest and locating the exact source PDF or
lawful archive record for every resource file. It must not begin until this
Phase 0 report is reviewed and accepted.
