# Phase 3 Report - Gold Sample and Process Calibration

**Start date:** 2026-08-09

**Branch:** `resource-ocr-cleanup`

**Scope:** Representative source-page sample and calibration for `resources/`

**Status:** Complete as a limited calibration pilot. All 20 selected pages
received a maintainer-authorized visual review; every former reading marker was
resolved or reclassified as a source-preservation or resource-divergence issue.
M-105 is closed by direct verification of the actual 2012 dictionary source.
This does not authenticate any resource file, clear rights, or authorize a
source-text change.

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

### 3.3 Independent transcription and comparison - transcription complete for all 20; comparison pass complete; second review mostly outstanding

All 20 gold-sample pages now have a private blind transcription draft
(`source-evidence/work/gold-transcription/GS-001.md`–`GS-020.md`), each
transcribed from the page image before the current resource was consulted.
Uncertain or unreadable tokens are marked with the plan's editorial-notation
markers rather than guessed. Only `GS-001` has completed a second independent
blind pass (`GS-001-review2.md`); the remaining 19 still need a second reader
before any can move past `draft_pending_second_visual_review`.

**Page-number correction.** Re-checking the visible printed folio on the
`GS-013`/`GS-014` renders (`SRC-SUU-014`, PDF image 60) found the page
actually shown does not match the `printed_page` value originally recorded
(80/81 actual vs. 90/91 recorded). `gold-sample.tsv` has been corrected and
the affected rows tagged `corrected_printed_page`. The neighboring pair at
image index 120 (`GS-015`/`GS-016`, recorded 200/201) was re-checked and
confirmed correct — the error is isolated to the one image-index mapping,
not systematic across the source. This is the same class of error the
Phase 3 report already caught once for `GS-008`'s resource-file mapping;
re-verifying recorded metadata against the rendered page itself, rather than
trusting an earlier manual mapping pass, continues to catch real errors.

**Current-resource comparison (all 20, two-way only — see gap below).** Each
gold draft was compared against the currently committed `resources/` text for
its source. Findings by source (full detail, including quoted restricted
source wording, is kept private in
`source-evidence/work/gold-transcription/PHASE_3_COMPARISON.md`; only
non-quoting aggregate findings are recorded here):

- `naxwe/15-naxwaha-sifayneed.md` (`SRC-NAX-015`, `GS-001`–`003`): severe.
  Beyond ordinary character noise, sampled sections show real content
  omission and column/layout scrambling — some multi-line example lists in
  the gold draft have no counterpart at all in the current text. Confirms
  this file needs full page-by-page reconstruction (Phase 4 batch 4A), not
  spot correction.
- `naxwe/17-naxwaha-af-soomaaliga.md` (`SRC-NAX-017`, `GS-004`–`006`):
  comparatively good. Several sampled lines match the current text closely
  or exactly; defects look like ordinary OCR character noise rather than
  structural damage.
- `qaamuus/01-b.md`, `24-i.md`, `08-s.md` (`GS-007`–`009`): the archived
  `SRC-QAA-001` PDF is confirmed as Yaasiin C. Keenadiid's unrelated 1976
  dictionary and remains unlinked. M-105 is now **resolved**: the actual
  source is Annarita Puglielli & Cabdalla Cumar Mansuur, *Qaamuuska
  Af-Soomaaliga* (RomaTrE-Press, Roma, 2012; ISBN 978-88-97524-02-1),
  registered as `SRC-QAA-002`. The 970-page public PDF was downloaded,
  hashed (`28658fd204f9156ed02fb83654366f0c97faf9139682d4a1597a29b2a3ebdbaa`),
  and directly matched to repository entries `baraarujin`, `islaamid`, and
  `sagal`. The source displays a RomaTrE-Press copyright notice, so
  `qaamuus/` correction work remains blocked by rights and full page mapping,
  not source identity.
- `erey-bixin/04-kimistari.md` (`SRC-EB-004`, `GS-010`–`012`): **an entire
  printed page is missing.** This entry originally read "good alignment"; the
  verification pass (§3.7) overturned that. All 40 headwords of printed page 13
  (`Diatomic` … `Dysprosium`) are absent from the current resource: its `## D`
  section runs alphabetically to `Diastase — Diaasteys` and then jumps
  straight to `## E`. The earlier "good alignment" verdict came from
  spot-checking entries that happened to survive and is exactly the failure
  mode sampling is prone to. This file shares the `naxwe/15` defect class —
  silent content loss, not character noise — and needs section-by-section
  coverage auditing rather than sampling. One term-level divergence was also
  found (`Substituent`: page reads `Barobixiye`, resource has `Bedbixiye`).
- `suugaan/14-hal-karaan-hadrawi.md` (`SRC-SUU-014`, `GS-013`–`016`): content
  locatable and unmangled at the checked anchors; the page-number correction
  above is this source's main finding, not text-content defects.
- `suugaan/18-dugsiga-fasalka-4aad-buugga.md` (`SRC-SUU-018`, `GS-017`–`020`):
  mixed. A structured vocabulary-list sample matched the current text
  exactly; narrative-prose samples sit inside visibly column-interleaved
  current text (two unrelated passages merged onto shared lines). Confirms
  Phase 4 batch 4B's page-by-page review requirement for this file.

**Gap: no raw-OCR layer exists to compare against.** Only the immutable
source PDFs and the already-committed `resources/` text are available for
these six sources — there is no separately preserved raw, uncorrected OCR
pass to complete the plan's intended three-way gold/raw-OCR/current
comparison. This should be resolved as an explicit decision (generate raw
OCR for the six gold-sample sources, or accept two-way comparison as
sufficient for Phase 3 calibration purposes) before Phase 3 is declared
complete.

**Poetry/alliteration — correction to an earlier claim in this report.** An
earlier revision of this section stated that alliterative verse "will
false-positive" against the audit's OCR heuristics. That was written from the
page evidence without checking the tool, and it was wrong. The gold-sample
pages do confirm dense alliteration (`m`-alliteration in `SRC-SUU-014`,
`x`-alliteration in `SRC-SUU-018` — every stressed word in an eleven-line
`gabay` carrying an `x`), but `tools/resource_audit/` implements **no**
repeated-token or duplicate-word heuristic, so no such false positive exists
today. Verified by running the audit against both poetry files: the rules that
fire there are `OCR_ISOLATED_GLYPH`, `OCR_LINE_END_HYPHEN`, `OCR_DIGIT_LETTER`,
`OCR_REPEATED_PUNCT`, and Markdown-structure rules — none keyed on repetition.
The alliteration evidence therefore stands as a **constraint on any future
rule**, not a defect in the current one: if a duplicate-token heuristic is ever
added, it must exempt verse.

**Measured false-positive calibration (`OCR_DIGIT_LETTER`) — fixed in code.**
Expanding every match (the audit reports only the first occurrence per rule per
file) found one rule with a real, measurable false-positive rate. Letter+digit
tokens were being reported as OCR noise even when they are the sources' own
notation:

| File | Before | After | Nature of the removed hits |
| --- | ---: | ---: | --- |
| `naxwe/15-naxwaha-sifayneed.md` | 22 | 1 | `Q1d`/`Q2d`/`Q3d` (Qofka 1aad–3aad, grammatical person), `L1d`–`L3d`, `W1d`/`W2d` — the book defines them in its own text, printing "Lahjadda 2d (L2d)" |
| `morphology/01-magacyada.md` | 4 | 0 | `L2a`–`L2d` paradigm-table row labels |
| `qaamuus/22-a.md` | 3 | 0 | `H2O`, `CH3CH`, `CH3CO` in `(kiim.)` chemistry entries |
| `qaamuus/01-b.md` | 1 | 0 | `H2O` |

`tools/resource_audit/common.py` now exempts exactly two evidence-backed
families — section/paradigm notation (`^[A-Z][1-9][a-d]$`) and chemical
formulae validated symbol-by-symbol against real element names — covered by
`test_digit_letter_exempts_documented_source_notation`.

Both exemptions were deliberately tightened after a first attempt
over-suppressed: a looser `^[A-Z]\d+[a-z]$` also swallowed `S0o`, and a
shape-only formula check (`^(?:[A-Z][a-z]?\d*)+$`) swallowed `CXGL1L`, `J8Aa`,
and `M1CQ` — all four are severe OCR debris in `suugaan/20`, and all four are
correctly reported again. A corpus-wide check confirms the exemption list now
resolves to exactly 14 distinct tokens, every one verified source notation,
with 97 genuine defect occurrences still reported. The rule's anchor line in
`naxwe/15` moved from a false positive to real corruption (`g0OrayNlU`, for
"goorayn") — the intended effect. `data/provenance/audit-baseline.json` was
regenerated to record the calibrated state; warnings fell 478 → 475 and the
error count returned to the Phase 2 baseline of 11.

**Nasal-assimilation calibration — qualified by the verification pass.**
`/m/` → `/n/` before `/k/` (e.g. `muslinku`, `Muslinkiyo`) appears across two
independent sources (`SRC-QAA-001`, `SRC-SUU-014`) and is regular Somali
morphophonology, grounded in `resources/naxwe/01-ereyada.md`'s own documented
assimilation rules. It is not an OCR or print defect.

An earlier revision of this section additionally claimed the dictionary page
was *consistent* in using the assimilated form across all five relevant
entries, and leaned on that consistency as evidence. Re-reading `GS-008`
disproves it: the `Islaamid` entry prints `muslimka`, unassimilated, in the
same `/k/` environment where `Islaamin`, `Islaan`, `Islaannimo`, and `Islaam`
all print `muslin-`. Four of five assimilate, not five of five. The
practical consequence is the important part: because the source is internally
inconsistent, **neither spelling may be treated as a mechanical correction
target in either direction** — which is a stronger constraint than the
original, overstated claim implied.

### 3.4 CER/WER and the raw-OCR layer — recorded decision

The plan's Phase 3 asks for a three-way comparison (gold vs. raw OCR vs.
current text) and CER/WER "where meaningful". Neither is achievable as
specified, for reasons that are structural rather than schedule-related:

1. **No raw-OCR layer exists.** `source-evidence/` holds `originals/` (the six
   PDFs) and `work/`; no `raw-ocr/` directory was ever populated, and the
   original conversions predate this cleanup effort with no preserved machine
   output (`ocr_engine` and `ocr_date` are `unknown` corpus-wide, tracked as
   M-103). Re-OCRing the PDFs today would produce *a* raw layer, but not *the*
   one the current text came from, so it would not support the comparison the
   plan intends.
2. **CER/WER is not meaningful where content is missing.** For `naxwe/15`,
   whole multi-line example lists in the gold transcription have no
   counterpart in the current text at all. Edit distance measured over
   unalignable spans reports a large number that describes absence, not
   character accuracy, and would understate severity by making omission look
   like noise.

**Decision (2026-08-11):** two-way comparison (gold vs. current text) is
accepted as sufficient for Phase 3 calibration, and per-file severity is
recorded qualitatively plus by defect-class rather than as a single CER/WER
figure. CER/WER is deferred to Phase 4, where it becomes meaningful per page
once a file is page-aligned. This is a scope decision, not a silent skip; M-103
remains open.

### 3.5 Review-effort estimate by document type

Revised after the §3.7 verification pass, which invalidated the first version
of this table. That version ranked effort by first-pass "flags per page";
re-reading resolved most of those flags, so the metric was measuring reader
hesitancy on poor print, not real difficulty. Flag counts below are
post-verification.

| Material | Samples | Open readings | Effort | Dominant cost |
| --- | ---: | ---: | --- | --- |
| Poetry (`suugaan/14`) | 4 | 0 | Low–medium | Text clean and unmangled; cost is protecting lineation and stanza quoting |
| Grammar prose/tables (`naxwe/17`) | 3 | 0 | Low–medium | Ordinary character noise |
| Schoolbook prose (`suugaan/18`) | 4 | 0 | Medium | Print *looks* bad but reads reliably; real cost is column interleaving in the current text |
| Dictionary (`qaamuus`) | 3 | 0 | Medium–high | Many short entries, each needing code/cross-ref checks; source identity is now resolved |
| Bilingual glossary (`erey-bixin/04`) | 3 | 0 | **High** | Reclassified: a whole printed page is missing, so coverage must be audited entry by entry |
| Grammar, structurally damaged (`naxwe/15`) | 3 | 0 | **High** | Column interleaving and missing content need page-by-page reconstruction |

Planning consequences:

- **Legibility and effort are different axes.** `suugaan/18` has the worst
  print quality of the six sources and the fewest genuine ambiguities; its
  cost sits in the current resource's layout damage, not in reading the page.
- **The expensive files are the ones that lost content**, not the ones that
  look noisy: `naxwe/15` and `erey-bixin/04` both need reconstruction and
  coverage auditing. `erey-bixin/04` moved from "low" to "high" only because
  the verification pass looked for absence rather than error.
- **Zero open readings does not mean low review cost.** `suugaan/14` and
  `naxwe/15` both have none, for opposite reasons — one is clean, the other is
  damaged in ways word-level checking cannot see.

Remaining after this task: rights and full source-page mapping before any
corrected text moves from private evidence into `resources/`.

### 3.7 Verification pass over all 19 outstanding samples (2026-08-11)

Every sample except `GS-001` (which already had a second pass) was re-read
against its page image and compared line by line with its first draft.

**What this pass is, and is not.** It was performed by the same agent that
produced the first drafts, with those drafts in context. It is therefore a
**verification pass, not an independent review**: it can catch outright
transcription errors, and did, but it cannot provide reviewer independence and
must not be counted as satisfying `REVIEW_GUIDE.md` §3. A Somali-language
reviewer's pass is still outstanding.

**Results across 19 samples:**

| Outcome | Count |
| --- | ---: |
| Clean — no discrepancy found | 8 |
| Corrections applied to the draft | 6 |
| Uncertainty markers resolved by re-reading | 24 |
| New uncertainties raised where the draft over-asserted | 4 |
| Open readings remaining after the pass | 8 |

**Transcription errors found and corrected** (first-draft reading → page
reading): `Dibaqalloocyada` → `Dabaqalloocyada`; `codal ahaan` → `codad
ahaan`; `codkan` → `codadkan`; `magacyadan oo` → `magacyadan ku`; `wixii ka
horreeya` → `wixii ke horreeya`; `Sibasq` → `Sibaaq`; `sagaashameeye` →
`sagaashameeyo` (the draft had made a variant identical to its own headword);
`baroeranaysey` → `barooranaysey`; plus a dropped clause restored in `GS-020`.

**Two findings matter more than the individual corrections:**

1. **The first draft silently normalised a source typo.** `GS-003` prints
   `Joogayn **garigal**?` in bold; the draft rendered it as `guriga`, quietly
   "fixing" the source toward the expected word. This is precisely what
   `TRANSCRIPTION_POLICY.md` forbids, and it was produced by the same process
   that generated all twenty drafts. It is corrected, but it is direct
   evidence that AI-assisted transcription drifts toward expectation and
   cannot be trusted without page-level checking.
2. **The first draft systematically over-flagged poor print.** `GS-019` and
   `GS-020` carried ~17 uncertainty markers between them; 16 resolve cleanly
   on careful re-reading. The markers tracked how bad the page *looked*, not
   how ambiguous it was. This invalidated the original effort model (§3.5,
   now revised) and is worth remembering as a bias in the opposite direction
   from finding 1 — the same process both over-asserted where it should have
   hedged and over-hedged where it should have read.

The agent-pass statement that eight readings remained was an arithmetic and
evidence-record error: its enumerated list contained ten reading markers.
The maintainer visual review then resolved all of them: `nool`; printed
`j oogto`; printed `dhigi.d`; `Islä` / `Id`; `Daliigo` / `ruobka`; `OROD`; and
`dhaweyd`. `Substituent — Barobixiye` is a clear resource divergence, not an
uncertain reading. The two GS-001 misreadings (`Waxan` → `Waxa`, and
`ereyada` → `ereyyada`) and its `kadin` marker were also resolved. All 20
sample records now have zero unresolved reading markers.

### 3.6 Reviewer sign-off

**Status: SIGNED — 2026-08-11, Sharafdin (maintainer).**

This sign-off is recorded only after explicit maintainer authorization to
complete the visual review, resolve M-105, and sign in Sharafdin's name. It
covers Phase 3's calibration record, not authentication of any resource file.

**Work performed, and by whom:**

| Step | Performed by | State |
| --- | --- | --- |
| First-pass transcription, 20 samples | Agent, from page images | Complete — candidates only |
| Verification pass, 19 samples (§3.7) | Agent, same reader | Complete — not independent |
| Audit heuristic calibration (§3.3) | Agent | Complete, tested, baselined |
| Maintainer visual / Somali-language review, 20 samples | **Sharafdin, assisted by Codex** | Complete — all readings checked against page images; single-reviewer limitation recorded |
| Maintainer sign-off | **Sharafdin** | **Given — explicitly authorized** |

**Maintainer review completed.** All former reading markers were checked on
the page images. `garigal`, `ke horreeya`, `ruobka`, `muslimka`, and
`yeer-i` / `keeri-ya` were confirmed as source-internal forms and retained.
The chemistry page-13 omission was confirmed at 40 headwords, changing
`erey-bixin/04` Phase 4 scope to a full coverage audit.

**Standing limitations regardless of sign-off:**

- **Single-reviewer project.** `REVIEW_GUIDE.md` §3 independence is not
  satisfied and is not waived. Phase 4 may produce `verified`, never
  `authenticated`, while this holds.
- **AI-assisted transcription is a candidate generator, not a reviewer.** §3.7
  finding 1 is concrete evidence of it drifting toward expected wording.
- **Two-way comparison only**, per the §3.4 decision.
- No `resources/` content file has been changed on the basis of any of this.

**Phase 3 exit criteria assessed:**

| Criterion | Status | Evidence |
| --- | --- | --- |
| Reviewers apply the policy consistently | **Not met — evidence against** | §3.7 finding 1: the first-pass process silently normalised a printed source typo (`garigal` → `guriga`) in `GS-003`, violating the fidelity rule, and separately over-flagged legible text across `GS-019`/`GS-020`. Policy was applied consistently in most places, but not reliably, and the failures were only caught by a second look at the page. |
| High-impact defects detected without unacceptable false positives | **Met** | Detected: `naxwe/15` structural omission; a whole missing printed page in `erey-bixin/04`; `suugaan/18` column interleaving; a wrong `printed_page` on `GS-013`/`GS-014`; the `qaamuus/` wrong-source link. Audit false positives measured and fixed in code (§3.3), verified not over-suppressed. |
| The correction log can reconstruct what changed and why | **Not exercised** | No correction has been applied to any `resources/` file. The private transcription record and this report document the calibration corrections; the Phase 4 machine-readable log remains required. |

**Maintainer decision: Phase 3 is complete as a calibration pilot, with one
exit criterion not met and no authentication authority.** A previous revision recorded this criterion as
"partially met" and declared Phase 3 passed; the verification pass produced
direct evidence against it, so it is downgraded here rather than left
flattering.

That is arguably the pilot working as intended. It was built to find out
whether the workflow could be trusted at scale, and it answered concretely:
the mechanical parts hold up, the language-judgment parts drift toward
expectation and need a human reader. Better to learn that on 20 private pages
than across 145 published files.

**Binding conditions carried into Phase 4:**

1. `qaamuus/` and `wordlists/` remain **blocked** until rights and complete
   source-page mapping are resolved; M-105 itself is closed.
2. No text derived from the restricted scans may be committed to `resources/`
   until the rights question is resolved.
3. **No gold sample may be cited as authenticated evidence** — all 20 are
   reviewed calibration evidence, not independently authenticated text.
4. `authenticated` status remains unavailable to any high-risk file while the
   project has one reviewer (`REVIEW_GUIDE.md` §3). Phase 4 may produce
   `verified`, not `authenticated`.
5. **Every AI-assisted transcription in Phase 4 must be page-checked by a
   human before it is committed**, on the §3.7 finding-1 evidence.
6. `erey-bixin/04` is re-scoped from sampling to full coverage audit.

## Issues and errors encountered

| Issue | Resolution or current state |
| --- | --- |
| The first recursive evidence search hit access-denied temporary test directories. | Reran the read-only search excluding only disposable test fixtures; supported evidence paths remained in scope. |
| A batch `Invoke-WebRequest` stalled and left a truncated QAA PDF. | Strict PDF reopening rejected it. The exact partial file was verified and removed, then the source was downloaded atomically, rehashed, and revalidated. |
| Poppler tools were not installed. | Used PyMuPDF and Pillow for private rendering, then inspected every selected page. This fallback does not alter the source PDFs. |
| Direct local-image viewing was blocked by the Windows split-root sandbox. | Generated reduced private preview contact sheets and viewed them through an in-memory data URL; nothing was committed. |
| The initial `GS-008` dictionary resource mapping pointed to the wrong letter file. | Validation caught the mismatch; visual/page content review corrected the mapping to `resources/qaamuus/24-i.md`. |
| `GS-013`/`GS-014` (`SRC-SUU-014`, PDF image 60) had a recorded `printed_page` (90/91) that did not match the folio number actually visible on the render (80/81). | Re-checked the neighboring sample pair (image 120, recorded 200/201) and confirmed it correct, so the error was isolated rather than systematic. Corrected `gold-sample.tsv` and tagged the affected rows `corrected_printed_page`. |
| The archived `SRC-QAA-001` PDF's sampled headwords do not appear in the current `resources/qaamuus/` text, which is written in a different, more compact house style. | Confirmed `SRC-QAA-001` is the distinct 1976 Keenadiid dictionary and remains unlinked. Downloaded and hashed the actual Puglielli--Mansuur 2012 PDF; title/colophon and sampled entries directly match `resources/qaamuus/`. M-105 is resolved; rights and full page mapping still block corrections. |
| The archive's BY-NC-ND terms conflict with publishing adapted text under the repository's permissive data terms. | Sources remain restricted; evidence and draft transcription remain private pending rights review. |

## Improvements delivered

- Six exact institutional scans now have stable records, cryptographic hashes,
  strict PDF checks, image counts, and visually checked bibliographic metadata.
- Sixty-three manifest relationships now carry the improved source metadata.
- Twenty exact source pages are registered across grammar, dictionary, glossary,
  poetry, and schoolbook layouts without storing source wording.
- The audit enforces gold-sample integrity and the test suite covers its minimum
  sample-size gate.
- All 20 gold-sample pages now have a blind independent-transcription draft
  and a first current-resource comparison pass, confirming the workflow can
  expose substantive omissions, substitutions, layout scrambling, and a
  page-metadata error without silently changing authentic material.
- Two reusable calibration findings, each confirmed across two independent
  sources: classical alliterative verse must be exempted from
  repeated-token OCR heuristics, and `/m/`→`/n/` nasal assimilation before
  `/k/` is regular Somali, not a defect.
- One blocking finding surfaced early, before any bulk correction work
  began: `qaamuus/`'s current text does not appear to be a page-faithful
  transcription of the archived source PDF used for its gold sample.

## Verification

- `python -m py_compile tools/resource_audit/provenance.py tools/tests/test_resource_audit.py` - passed.
- `PYTHONPATH=tools python3 -m unittest tools.tests.test_resource_audit -v` - 12 tests passed.
- Repository audit - 75 info, 475 warning, 11 known error, 0 fatal, and 0 new
  warning-or-higher findings against the Phase 2 baseline.
- All 63 affected manifest relationships match their source records.
- All 20 sample IDs and source/page selections are unique and all referenced
  resource paths exist.
- Git ignore checks confirm PDFs, renders, and working transcription remain
  outside version control.
- No source-text content file under `resources/`, `spec/`, or `standards/`,
  and no roadmap or implementation-plan file, was modified. Only collection
  source inventories were updated for the resolved metadata record.

## Remaining Phase 3 tasks and stop condition

1. Obtain a rights decision for the restricted scans and the 2012 RomaTrE-Press
   dictionary before publishing any derived or corrected text.
2. Complete source-page mapping before changing `qaamuus/` or regenerating
   `wordlists/`.
3. Recruit a second qualified reviewer, or adopt the documented time-separated
   self-review protocol, before any high-risk file can be authenticated.
4. Retain the two-way comparison decision and create the Phase 4 correction log
   before source-text changes begin.

Phase 3 is **complete as a calibration pilot**. It does not authorize
source-text changes or publication of restricted derived material.
