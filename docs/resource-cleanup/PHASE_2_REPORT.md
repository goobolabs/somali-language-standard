# Phase 2 Report — Audit and Validation Tooling

**Date:** 2026-08-09

**Branch:** `resource-ocr-cleanup`

**Scope:** Read-only validation of `resources/` and its Phase 1 provenance

**Status:** Complete; stopped for maintainer review before Phase 3

## What we completed

- Added `tools/audit_resources.py` and the `tools/resource_audit/` package.
- Added a read-only inventory command with per-file and resource-tree SHA-256
  hashes, bytes, lines, extensions, and file counts.
- Added strict UTF-8, Unicode normalization, suspicious character, line-ending,
  Markdown structure, OCR-candidate, Somali-aware, and collection-specific
  checks.
- Added provenance validation for missing records, unknown source IDs, blank
  metadata, file hash drift, unreviewed bulk changes, and unsupported
  authentication claims.
- Added deterministic machine-readable findings with severity, file, line/page,
  rule, suggested action, assignee, resolution, excerpt, and stable ID.
- Added nine unit/regression tests and a resource-scoped GitHub Actions workflow.
- Added a reviewed baseline and reasoned suppression register.
- Documented the one-command audit, baseline policy, and review rules in
  `docs/RESOURCE_AUDIT.md`.

The validator only reports. It contains no correction or generation command and
does not modify resource text.

## Current measured situation

The verified inventory is:

| Measure | Result |
| --- | ---: |
| Resource files | 145 |
| Bytes | 8,359,339 |
| Lines | 187,470 |
| Markdown files | 144 |
| TSV files | 1 |
| Resource-tree SHA-256 | `c1e0b2927fb15cad00f3fb8a5fa3243f74f480f07f3f3b716467ed046bc929b8` |

The Phase 2 baseline contains 709 findings:

| Severity | Count |
| --- | ---: |
| Info | 75 |
| Warning | 613 |
| Error | 21 |
| Fatal | 0 |

Two of the 21 error-severity findings are suppressed rule matches on one
project-authored README line that deliberately shows a replacement character.
No book or transcription finding is suppressed. The remaining findings are an
explicit review queue, not automatically accepted corrections and not proof that
every heuristic match is an error.

The most frequent categories are missing/uncertain dictionary grammatical codes
(197), non-LF line endings (135), digit-bearing dictionary headwords (49),
malformed-list candidates (35), dangling dictionary cross-reference candidates
(30), and wordlist sort-order candidates (26). Source-page comparison remains
required before changing authentic content.

Error-severity candidates include ten mixed-line-ending files, seven control
character locations, one malformed dictionary-entry candidate, and one glossary
record with an empty side. They remain unchanged because Phase 2 is discovery
and regression tooling, and the exact source scans are unavailable in the
workspace.

## Issues encountered and how they were handled

| Issue or error | Resolution and improvement |
| --- | --- |
| The first single-file implementation exceeded the Windows command-size limit. | Split the validator into small responsibility-based modules and retained a thin CLI wrapper. |
| Initial calibration produced 15,680 findings, dominated by false positives. | Restricted digit checks to ASCII where appropriate, excluded collection metadata from content validators, summarized repeated candidates per file, and parsed compound dictionary abbreviations as units. The calibrated queue is 709 findings. |
| A local report path under `C:\\tmp` was blocked by the managed filesystem. | Kept disposable reports in the ignored repository `temp/` directory; no source or committed evidence was affected. |
| Python temporary test directories were blocked by the restricted sandbox ACL. | Verified the suite outside that ACL using isolated system temporary directories. All nine tests pass. |
| A literal replacement-character example triggered both replacement and mojibake rules. | Recorded two explicit, dated suppressions for that project-authored documentation example only. |
| Existing findings would make a new strict validator fail immediately. | Created a reviewed, deterministic baseline. CI now blocks warning-or-higher findings that are new, while preserving the existing queue for later source-based review. |

## Verification performed

- `python -m compileall -q tools` — passed.
- `python -m unittest discover -s tools/tests -v` with `PYTHONPATH=tools` —
  9/9 passed.
- Inventory command — passed with the Phase 1 resource-tree hash unchanged.
- Baseline audit with `--fail-on-new warning` — passed with zero new findings.
- Two independent baseline regenerations and the committed file all produced
  SHA-256 `2068bdf95e3ccdaee6e788bbe7495f55ce5c5db6cc6eadf229fcb6887a20482c`.
- Git resource diff — empty.
- Git diff for `ROADMAP.md`, `IMPLEMENTATION_PLAN.md`, `spec/`, and `standards/`
  — empty.
- Read-only test — confirms resource bytes are unchanged by an audit.
- Regression tests — confirm invalid UTF-8, provenance hash drift, and missing
  suppression justification are detected; a new encoding error returns a
  failing CLI status.

## Phase 2 exit criteria

- [x] A clean audit can be rerun from one documented command.
- [x] Findings are reproducible and suppressions require a reason.
- [x] Automated suggestions never modify source text without review.

## Phase boundary

Phase 3 has not started. It requires representative source page images for a
gold sample and independent transcription. Because those source scans are not
available in this workspace, no content correction or reconstructed wording was
attempted in Phase 2.
