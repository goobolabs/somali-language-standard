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
| Bytes | 8,172,035 |
| Lines | 187,470 |
| Markdown files | 144 |
| TSV files | 1 |
| Resource-tree SHA-256 | `5b5adb5b4780a98e6ae76d2712b019b30912c5488a2dd8ef6260b7e8c8a0d81b` |

The Phase 2 baseline contains 564 findings:

| Severity | Count |
| --- | ---: |
| Info | 75 |
| Warning | 478 |
| Error | 11 |
| Fatal | 0 |

Two of the 11 error-severity findings are suppressed rule matches on one
project-authored README line that deliberately shows a replacement character.
No book or transcription finding is suppressed. The remaining findings are an
explicit review queue, not automatically accepted corrections and not proof that
every heuristic match is an error.

The most frequent categories are missing/uncertain dictionary grammatical codes
(197), digit-bearing dictionary headwords (49), malformed-list candidates (35),
dangling dictionary cross-reference candidates (30), wordlist sort-order
candidates (26), and line-end hyphenation candidates (25). Source-page
comparison remains required before changing authentic content.

Open error-severity candidates include seven control-character locations, one
malformed dictionary-entry candidate, and one glossary record with an empty
side. They remain unchanged because Phase 2 is discovery and regression tooling,
and the exact source scans are unavailable in the workspace.

## Issues encountered and how they were handled

| Issue or error | Resolution and improvement |
| --- | --- |
| The first single-file implementation exceeded the Windows command-size limit. | Split the validator into small responsibility-based modules and retained a thin CLI wrapper. |
| Initial calibration produced 15,680 findings, dominated by false positives. | Restricted digit checks to ASCII where appropriate, excluded collection metadata from content validators, summarized repeated candidates per file, and parsed compound dictionary abbreviations as units. The calibrated queue is 709 findings. |
| A local report path under `C:\\tmp` was blocked by the managed filesystem. | Kept disposable reports in the ignored repository `temp/` directory; no source or committed evidence was affected. |
| Python temporary test directories were blocked by the restricted sandbox ACL. | Verified the suite outside that ACL using isolated system temporary directories. All nine tests pass. |
| A literal replacement-character example triggered both replacement and mojibake rules. | Recorded two explicit, dated suppressions for that project-authored documentation example only. |
| Existing findings would make a new strict validator fail immediately. | Created a reviewed, deterministic baseline. CI now blocks warning-or-higher findings that are new, while preserving the existing queue for later source-based review. |
| The first GitHub Actions run found that Phase 1 hashes reflected Windows CRLF checkout bytes, producing 146 false provenance failures on Linux. | Made the audit use indexed Git bytes whenever workspace content differs only by automatic line-ending conversion, migrated only the provenance hashes, added a regression test, and regenerated a cross-platform baseline. No resource file changed. |

## Verification performed

- `python -m compileall -q tools` — passed.
- `python -m unittest discover -s tools/tests -v` with `PYTHONPATH=tools` —
  10/10 passed.
- Inventory command — passed with the Phase 1 resource-tree hash unchanged.
- Baseline audit with `--fail-on-new warning` — passed with zero new findings.
- Two independent baseline regenerations and the committed file all produced
  SHA-256 `1ef9b3073502630794968b449ab940944f37896d45e2404f601d23aa1ca983d2`.
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
