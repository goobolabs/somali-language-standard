# Phase 3 Report - Gold Sample and Process Calibration

**Start date:** 2026-08-09

**Branch:** `resource-ocr-cleanup`

**Scope:** Representative source-page sample and calibration for `resources/`

**Status:** In progress - institutional scan candidates found; download and page
selection await confirmation that the project may use CC BY-NC-ND evidence

## Source-fidelity boundary

Phase 3 may compare current text and raw OCR only with an exact, lawfully held
source page. It must not infer page wording from the current resource, generate
a substitute transcription, or silently treat a likely reading as authentic.
No source content has been edited in this phase.

## Task log

### 3.0 Verify authentic evidence availability - complete

The workspace was searched recursively for PDF, PNG, JPEG, TIFF, and DjVu
evidence, including Git-ignored paths and `source-evidence/`. No PDF or page
image was found. `source-evidence/` contains only its policy README; the expected
`originals/`, `raw-ocr/`, and `work/` evidence directories contain no evidence.

The Phase 1 provenance records independently confirm the same state:

- all 45 direct-source records use `not_available_in_workspace`;
- 129 resource-manifest relationships use `not_available_in_workspace`;
- required source hashes are `unavailable` and rights remain unresolved;
- metadata issues M-100 through M-104 block exact source identity, page mapping,
  checksums, conversion provenance, and rights review.

The first recursive search encountered access-denied errors in ignored temporary
test-fixture directories. The search was rerun with those disposable directories
excluded and completed successfully. This did not hide `source-evidence/` or any
supported evidence-file extension.

### 3.1 Locate institutional source candidates - complete

A read-only public-catalog search located exact-title candidates in Roma Tre
University's ArcAdiA archive. These are stable institutional catalog records,
not yet registered source copies and not yet proven to be the copies used for
the existing OCR conversion.

| Source ID | Archive record | Catalog identity | Candidate file |
| --- | --- | --- | --- |
| `SRC-NAX-015` | [hdl:2307/2606](https://hdl.handle.net/2307/2606) | Maxamed X. Xuseen Raabbi, 1994, 150 pages | *Naxwaha Sifayneed ee Afsoomaaliga: mugga kowaad (mi): Ereyeynta* |
| `SRC-NAX-017` | [hdl:2307/776](https://hdl.handle.net/2307/776) | Jaalle Shire Jaamac Axmed, 1976, 156 pages | *Naxwaha Af Soomaaliga* |
| `SRC-SUU-014` | [hdl:2307/2701](https://hdl.handle.net/2307/2701) | Maxamed Ibraahim Warsama (Hadraawi), 1993, 453 pages | *Hal-Karaan* |
| `SRC-SUU-018` | [hdl:2307/898](https://hdl.handle.net/2307/898) | Somali Ministry of Education/Curriculum Office, 118 pages | *Buugga Suugaanta: Dugsiga Sare, Fasalka Afraad* |
| `SRC-QAA-001` | [hdl:2307/2021](https://hdl.handle.net/2307/2021) | Yaasiin C. Keenadiid, 1976, 498 pages | *Qaamuuska Af-Soomaaliga* |
| `SRC-EB-004` | [hdl:2307/787](https://hdl.handle.net/2307/787) | Somali Ministry of Education, 1987, 47 pages | *Qaamuuska eray-bixinta ee Kimistari* |

Each archive page exposed a direct PDF and linked the item to
[CC BY-NC-ND 3.0 Italy](https://creativecommons.org/licenses/by-nc-nd/3.0/it/).
That license permits sharing with attribution for noncommercial purposes, but
does not permit distribution of transformed or adapted material. The archive
claim therefore supports candidate access but does not by itself authorize this
project to publish corrected transcriptions. Project purpose, attribution,
NoDerivatives implications, and any applicable exception or separate permission
require a recorded rights decision before evidence is downloaded or derived
text is committed.

### 3.2 Select at least 20 exact pages - blocked

Exact page numbers cannot be selected responsibly until the corresponding
source copies and editions are registered. The plan specifically requires pages
from the following source families:

| Required material | Source ID | Resource relationship | Current blocker |
| --- | --- | --- | --- |
| Descriptive grammar | `SRC-NAX-015` | `resources/naxwe/15-naxwaha-sifayneed.md` | Exact edition and scan unavailable |
| Somali grammar | `SRC-NAX-017` | `resources/naxwe/17-naxwaha-af-soomaaliga.md` | Exact edition and scan unavailable |
| Poetry and deliberate lineation | `SRC-SUU-014` | `resources/suugaan/14-hal-karaan-hadrawi.md` | Exact edition and scan unavailable |
| Schoolbook prose and layout | `SRC-SUU-018` | `resources/suugaan/18-dugsiga-fasalka-4aad-buugga.md` | Exact edition and scan unavailable |
| Dictionary columns | `SRC-QAA-001` | `resources/qaamuus/` | Edition, compiler, scan, and rights unresolved |
| Glossary tables | `SRC-EB-001` through `SRC-EB-010` | `resources/erey-bixin/01` through `09` | Exact editions/page coverage unavailable |

The following is a provisional coverage allocation, not a selected sample and
not a claim about any page's contents:

| Coverage target | Minimum unique pages |
| --- | ---: |
| Clean prose | 2 |
| Severe OCR noise | 4 |
| Dictionary columns | 3 |
| Glossary tables | 3 |
| Grammar tables and examples | 3 |
| Poetry with meaningful line breaks | 3 |
| Old print, damage, unusual fonts, headers/footers, or multi-column layout | 2 |
| **Total** | **20** |

Candidate pages will be chosen only after visual inspection of the registered
copies. They must collectively exercise Somali apostrophes, quotation marks,
long vowels, `c`, `x`, `kh`, `sh`, and `dh` without inventing examples.

## Evidence required to continue

For each selected source, the maintainer or rights reviewer must first confirm
that the project's intended use is permitted. The institutional candidates give
stable access references, but their noncommercial and NoDerivatives conditions
must be resolved for this workflow. Any approved local evidence belongs under
the ignored `source-evidence/originals/` directory and must not be committed.

Before a page enters the sample:

1. keep the original PDF/image immutable;
2. record the source ID, exact title/edition, lawful location, and rights basis;
3. compute and record SHA-256 and page count;
4. verify title page and colophon metadata;
5. map PDF image indexes to printed page numbers;
6. render candidate pages under `source-evidence/work/` and inspect them
   visually for legibility, columns, tables, headers, footers, and lineation;
7. record the final 20 or more page IDs before transcription begins.

## Tasks not started

- Independent transcription from page images
- Current text/raw OCR/gold comparison
- Error classification and CER/WER measurement
- Automated-rule false-positive calibration
- Editorial notation and correction-log testing
- Review-effort estimation
- Reviewer sign-off

These tasks remain deliberately unstarted because proceeding without authentic
pages would violate the transcription policy and the user's instruction not to
generate source content.

## Current verification

- Phase 2 documentation corrections were reviewed with `git diff --check`.
- No source evidence files were found after a successful supported-extension
  search outside disposable test directories.
- No file under `resources/` was modified.
- No roadmap, implementation-plan, specification, or standards file was
  modified.

## Next gate

Phase 3 resumes when the maintainer confirms that the project may download and
privately analyze the CC BY-NC-ND archive candidates, and a rights reviewer
defines what derived review material may be committed or published. The next
task is immutable download, hashing, PDF metadata inspection, and visual page
inspection followed by exact sample selection; it is not transcription.
