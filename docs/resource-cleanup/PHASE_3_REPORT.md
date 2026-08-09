# Phase 3 Report - Gold Sample and Process Calibration

**Start date:** 2026-08-09

**Branch:** `resource-ocr-cleanup`

**Scope:** Representative source-page sample and calibration for `resources/`

**Status:** In progress - source intake and exact 20-page selection complete;
independent transcription, comparison, metrics, review, and rights resolution
remain incomplete

## Source-fidelity boundary

Phase 3 compares current text and raw OCR only with an exact source page. It
must not infer wording from the current resource, silently repair an uncertain
reading, or generate substitute source content. The downloaded PDFs, page
renders, and working transcriptions remain in the Git-ignored
`source-evidence/` area. No file under `resources/` has been edited.

The user's approval to continue was treated as approval for private evidence
analysis only. It was not treated as permission to publish a transformed
transcription or to relicense source content.

## Task log

### 3.0 Verify authentic evidence availability - complete

The initial workspace search found no PDF or page-image evidence. A subsequent
institutional-catalog search located six exact-title candidates in Roma Tre
University's ArcAdiA archive. Each candidate was downloaded to the ignored
`source-evidence/originals/` directory, kept out of Git, hashed, reopened in
strict PDF mode, and visually checked at its title or credit pages.

| Source ID | Stable record | PDF images | SHA-256 |
| --- | --- | ---: | --- |
| `SRC-NAX-015` | `https://hdl.handle.net/2307/2606` | 158 | `f1897a380208da4064aead6457cb2e554e6bbd427710f5e480713064e36fb733` |
| `SRC-NAX-017` | `https://hdl.handle.net/2307/776` | 162 | `e399daf78c5d2923a860f4fabc3ec982a87c2f6d1685a490075ed285648f3c08` |
| `SRC-QAA-001` | `https://hdl.handle.net/2307/2021` | 518 | `9eee4c2b36b399330cebd3820d4f8fb5cb5d61814513d1c6281e8d2ba953b4a1` |
| `SRC-EB-004` | `https://hdl.handle.net/2307/787` | 49 | `f5c3da528f6379eb89024b53600f36bbc2a714fecc97d1c0a628bf915e8a85ba` |
| `SRC-SUU-014` | `https://hdl.handle.net/2307/2701` | 246 | `67310f61f86caa4b18d67eb32d51c0f0216884e09484a6f16a664a165e80eba4` |
| `SRC-SUU-018` | `https://hdl.handle.net/2307/898` | 65 | `de244933076e9cfc3106fce1b3fd61745a139bfccd597b1f3abd5b0758c49e7f` |

PDF checks confirmed the `%PDF` signature, EOF trailer, unencrypted state,
strict reopen, image count, and recorded SHA-256 for every file. The title and
credit-page evidence was used to improve title, author, publisher, place, and
year fields in `sources.tsv`; 63 matching relationships were propagated to
`resource-manifest.tsv`. These metadata improvements do not authenticate the
existing resource transcription.

### 3.1 Rights classification - open gate

The institutional records link the scans to CC BY-NC-ND 3.0 Italy. Inspected
book pages also include source-specific copyright or publication-rights
statements. Because the repository's data is offered under CC BY 4.0 and is
intended to permit commercial reuse, the archive license cannot simply be
carried into corrected resource text.

The six source records therefore use `rights_status=restricted` and retain
`metadata_status=unverified`. The scans may support this private calibration,
but no derived transcription will be committed or published until a qualified
rights decision or separate permission is recorded.

### 3.2 Select at least 20 exact pages - complete

`data/provenance/gold-sample.tsv` registers 20 unique source/printed-page
selections across all six source families. It records only evidence IDs,
source hashes, PDF image indexes, printed-page mappings, spread sides, related
resource paths, coverage goals, and workflow status. It contains no source
wording.

| Material | Source | Samples | Coverage emphasis |
| --- | --- | ---: | --- |
| Descriptive grammar | `SRC-NAX-015` | 3 | prose, examples, damaged OCR |
| Somali grammar | `SRC-NAX-017` | 3 | grammar prose and examples |
| Dictionary | `SRC-QAA-001` | 3 | multi-column entries and typography |
| Chemistry glossary | `SRC-EB-004` | 3 | glossary/table structure |
| Poetry | `SRC-SUU-014` | 4 | spread mapping and meaningful lineation |
| Schoolbook | `SRC-SUU-018` | 4 | spread mapping, prose, and page layout |
| **Total** | **6 sources** | **20** | |

Each page was rendered privately at 144 DPI and visually inspected. Landscape
spread scans were split only for page identification: Hal-Karaan PDF images 60
and 120 map to printed pages 90/91 and 200/201; the schoolbook images 30 and 45
map to printed pages 50/51 and 80/81. Render boundaries were checked for
clipping. Source hashes in every sample row match both `sources.tsv` and the
local immutable PDF.

The audit now validates this registry: required fields, minimum size, source
IDs and hashes, positive image indexes, spread-side values, existing resource
paths, unique sample IDs, and unique source/page selections. A unit test covers
the 20-row minimum.

### 3.3 Independent transcription and comparison - started, not complete

One private blind draft (`GS-001`) was transcribed from the page image before
consulting the current resource. Uncertain and unreadable tokens were marked as
uncertain instead of guessed. The draft is explicitly pending a second visual
review and is not accepted gold text.

Only after that blind draft, the corresponding current segment in
`resources/naxwe/15-naxwaha-sifayneed.md` was inspected. The comparison found
omitted examples and list items, word substitutions, character confusions,
broken words, and lost layout. These are calibration observations only; the
resource was not corrected because the sample has not passed independent
review and publication rights remain unresolved.

The remaining 19 blind drafts, second reviews, adjudication, current-text/raw-
OCR comparisons, and accepted gold records have not been completed.

## Issues and errors encountered

| Issue | Resolution or current state |
| --- | --- |
| The first recursive evidence search hit access-denied temporary test directories. | Reran the read-only search excluding only disposable test fixtures; supported evidence paths remained in scope. |
| A batch `Invoke-WebRequest` stalled and left a truncated QAA PDF. | Strict PDF reopening rejected it. The exact partial file was verified and removed, then the source was downloaded atomically, rehashed, and revalidated. |
| Poppler tools were not installed. | Used PyMuPDF and Pillow for private rendering, then inspected every selected page. This fallback does not alter the source PDFs. |
| Direct local-image viewing was blocked by the Windows split-root sandbox. | Generated reduced private preview contact sheets and viewed them through an in-memory data URL; nothing was committed. |
| The initial `GS-008` dictionary resource mapping pointed to the wrong letter file. | Validation caught the mismatch; visual/page content review corrected the mapping to `resources/qaamuus/24-i.md`. |
| The archive's BY-NC-ND terms conflict with publishing adapted text under the repository's permissive data terms. | Sources remain restricted; evidence and draft transcription remain private pending rights review. |

## Improvements delivered

- Six exact institutional scans now have stable records, cryptographic hashes,
  strict PDF checks, image counts, and visually checked bibliographic metadata.
- Sixty-three manifest relationships now carry the improved source metadata.
- Twenty exact source pages are registered across grammar, dictionary, glossary,
  poetry, and schoolbook layouts without storing source wording.
- The audit enforces gold-sample integrity and the test suite covers its minimum
  sample-size gate.
- The first blind comparison confirms that the workflow can expose substantive
  omissions and substitutions without silently changing authentic material.

## Verification

- `python -m py_compile tools/resource_audit/provenance.py tools/tests/test_resource_audit.py` - passed.
- `python -m unittest discover -s tools/tests -v` - 11 tests passed.
- Repository audit - 75 info, 478 warning, 11 known error, 0 fatal, and 0 new
  warning-or-higher findings against the Phase 2 baseline.
- All 63 affected manifest relationships match their source records.
- All 20 sample IDs and source/page selections are unique and all referenced
  resource paths exist.
- Git ignore checks confirm PDFs, renders, and working transcription remain
  outside version control.
- No file under `resources/`, `spec/`, or `standards/`, and no roadmap or
  implementation-plan file, was modified.

## Remaining Phase 3 tasks and stop condition

1. Obtain a recorded decision about private review artifacts and publication of
   corrected transcriptions from these restricted scans.
2. Assign independent transcriber, second reviewer, and adjudicator roles.
3. Complete and review all 20 blind transcriptions without consulting the
   current resource during the first pass.
4. Compare accepted gold text with current text and raw OCR; classify errors and
   measure CER/WER.
5. Calibrate automated rules, editorial notation, correction logs, and review
   effort.
6. Record reviewer sign-off and only then decide whether Phase 3 passes.

Phase 3 is **not complete**. Work must stop for review at the phase gate; this
report does not authorize source-text changes or publication of restricted
derived material.
