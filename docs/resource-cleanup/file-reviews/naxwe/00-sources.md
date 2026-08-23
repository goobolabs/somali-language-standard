# Audit record — Sources: naxwe

- **Resource path:** `resources/naxwe/00-sources.md`
- **Collection / family:** naxwe / source registry
- **Priority:** P3
- **Method:** repository-only, row-by-row metadata and status audit
- **Audit status:** approved; conservative registry cleanup applied and awaiting review
- **Audit started:** 2026-08-12
- **File size at audit start:** 33 lines; 250 words; 1,698 bytes
- **Resource SHA-256 at audit start:**
  `0954a6456e23d5e64cd565c9488080600cd8ce125b391618f0faf19672634784`
- **Resource-text changes during this audit:** none
- **Pre-audit state note:** the baseline already includes the separately
  approved N14-R001 correction adding `Maxamed Xaaji Xuseen Raabi` to the file
  14 row; this audit did not introduce that change.

## Target output model

This file is a compact collection registry, not a grammar chapter or OCR
transcript. It has one H1, three H2 headings, two valid Markdown tables with
seven data rows total, five coverage-note bullets, and no Markdown links. The
two primary rows represent thirteen chapter files plus the glossary; the five
supplementary rows map one-to-one to files 13-17. Every listed resource exists.

Cleanup should preserve all seven inventory rows, the primary/supplementary
division, every confirmed title, author, and year, and both cross-collection
notes. It should add only metadata recoverable from exact title-page evidence;
leave unrecoverable years and authors explicitly unresolved; replace the stale
2026-07-18 OCR-remediation warning with the actual post-audit status; and turn
file references into useful local links without converting the registry into a
long bibliography or workflow report.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-4 | reviewed | N00S-R001 |
| 6-12 | reviewed | N00S-R002 |
| 13-17 | reviewed | N00S-R003 |
| 18 | reviewed | N00S-R004 |
| 19 | reviewed | N00S-R005 |
| 20 | reviewed | N00S-R006 |
| 21 | reviewed | N00S-R007 |
| 23-31 | reviewed | N00S-R008 |
| 32-33 | reviewed | N00S-R009 |
| whole file | reviewed | N00S-R010 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| N00S-R001 | 1-4 | Clear registry title and attribution-once policy; English framing differs from grammar prose but matches every sibling `00-sources.md` registry / low | Orthography, phonology, qaamuus, wordlists, erey-bixin, and suugaan source registries use the same English inventory convention; `resources/README.md` assigns title/author/year to this file | Retain the title and concise purpose. Do not translate only this registry or add book front matter to content files. | `repository-supported`; `structural-only`; `intentional-retained` |
| N00S-R002 | 6-12 | Primary title, two authors, year, chapter range, and trilingual glossary relationship are consistent across the repository; the glossary's dash can look like missing provenance even though it is grouped under the same 1998 source / medium | `resources/README.md`, `PROJECT_OVERVIEW.md`, `docs/RESOURCES.md`, and `resources/sarfe/00-sources.md` all identify *Barashada Naxwaha Af Soomaaliga*, Mansur/Mansuur and Puglielli, 1998. The naxwe README identifies files 00-12 and `ereyfur.md` as one collection | Retain both rows, title, authors, and year. Italicize the book title consistently. Clarify that `ereyfur.md` is the trilingual glossary associated with the same source and has no separate byline in the registry; do not invent separate authorship. | `repository-supported`; `metadata-clarification`; `intentional-retained` |
| N00S-R003 | 13-17 | File 13 title, corporate author/compiler, and 1973 are exactly recoverable / no defect | Original title page says Muqdisho 1973 and lists seven members writing and arranging the book as members of `Guddiga Af Soomaaliga`; the completed N13 audit keeps publication apparatus in this registry rather than the topic file | Retain the row unchanged apart from link/typographic normalization. The corporate attribution is an appropriate compact representation; do not copy the seven-name title-page list into the table. | `same-file-exact`; `repository-supported`; `intentional-retained` |
| N00S-R004 | 18 | File 14's author is now present through an earlier approved correction; its year is not recoverable from repository evidence / medium | Original title page repeats `QORE: Maxamed Xaaji Xuseen Raabbi`; N14-R001 and the qoraal/phonology registries support normalized `Maxamed Xaaji Xuseen Raabi`. No source year occurs in the recoverable metadata | Preserve the already approved author correction and leave the year unresolved. Add a concise unresolved-metadata note; do not infer a year from related works. | `repository-supported`; `unresolved`; `intentional-retained` |
| N00S-R005 | 19 | File 15 omits an exactly recoverable author and its source-title cell omits the repeated volume identification / high | The original title page twice gives `Naxwaha Sifayneed Ee Afsoomaaliga`, `Mugga Kowaad`, `Ereyeynta`, and `GORE: Maxamed x. Xuseen Raabbi`; file 14 and two other source registries establish the normalized author form. No publication year is recoverable | Retain the row and add `Maxamed Xaaji Xuseen Raabi`. Expand the source title conservatively to identify **Mugga Kowaad: Ereyeynta**. Leave the year unresolved; do not derive one from dates cited inside the book. | `same-file-exact`; `repository-supported`; `metadata-repair`; `unresolved` for year |
| N00S-R006 | 20 | File 16's title is known, while `(Xilliyada 5aad)` exists only as registry metadata and neither author nor year is recoverable / medium | Original OCR opens with `WEEraynta Soomaaliga` followed by unreadable debris and contains no author/year metadata; N16-R001 explicitly preserves `Xilliyada 5aad` only as source metadata and normalizes the content-resource title separately | Preserve the source-title styling and `Xilliyada 5aad` note in the inventory, but explicitly label author/year unresolved. Do not infer either field or move the note into the content resource. | `repository-supported` in part; `unresolved`; `intentional-retained` |
| N00S-R007 | 21 | File 17's title and author are confirmed, but the blank year cannot safely be filled from the damaged bibliography / medium | Original title page says `NAXWAHA AF SOOMAALIGA`, `waxaa qoray Jaalle Shire Jaamac Axmed`. An apparent 1974 reference occurs only in file 15's closing bibliography, which N15-R056/R057 classified as interleaved and unsafe for reconstructing author-title-year associations; N17-R021 likewise prohibits inference from its collapsed bibliography | Retain title and author. Keep year explicitly unresolved and record the rejected 1974 candidate as insufficient evidence in the audit, not as registry fact. | `same-file-exact`; `unresolved`; `intentional-retained` |
| N00S-R008 | 23-31 | Coverage note is materially stale: it says files 13-17 still contain 0.9-6.3% raw OCR, remain flagged review, and await remediation, although all five now have approved whole-file audits and applied SLS-native cleanups / fatal | Completed N13-N17 records document removal of book apparatus and OCR debris, exclusion of unrecoverable material, preservation of recoverable topics, and cleanup validation on 2026-08-12. The tracker shows all five at Audit + Audit approval + Cleanup. Those records do not claim page-by-page verification against source scans | Remove the obsolete percentages, review flag, and pending-remediation wording. State that files 00-12 are topic-focused SLS curations rather than “clean extractions”; state that files 13-17 received audited SLS-native cleanup, with unrecoverable OCR excluded rather than guessed; and retain the important limitation that cleanup is not the same as full scan verification. Link the tracker/review records instead of hard-coding a transient approval count. | `repository-supported`; `status-correction`; `scope-correction`; `structural-only` |
| N00S-R009 | 32-33 | Orthography and phonology boundary notes are correct but use unlinked code paths / low | `resources/qoraal/00-sources.md` identifies *Habka Qoraalka* (1977); `resources/dhawaaq/00-sources.md` identifies *Codaynta Af Soomaaliga* (1977) | Retain both collection-boundary notes and convert their directories or source registries to local Markdown links. | `repository-supported`; `navigation-update`; `intentional-retained` |
| N00S-R010 | whole file | All seven data rows are structurally valid, but none of the file/directory references is clickable and missing metadata is represented by unexplained dashes / low | Every named file and target directory exists; `resources/naxwe/README.md` maps the same collection; sibling registries retain unresolved fields rather than guessing them | Preserve two tables and all seven rows. Link the primary range endpoints, glossary, and each supplementary file; normalize table delimiters; add one compact unresolved-metadata note for files 14-17; add no new bibliography rows, inferred dates, or publication claims. | `structural-only`; `repository-supported`; `unresolved`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned registry should retain its compact sequence:

1. title and attribution-once purpose;
2. primary 1998 grammar table with the chapter range and glossary;
3. supplementary grammar table with files 13-17;
4. a compact note identifying which metadata fields remain unresolved; and
5. coverage notes distinguishing SLS curation, audited OCR cleanup, and full
   scan verification, followed by linked qoraal/phonology boundaries.

All seven existing rows must remain. Confirmed fields must not be weakened;
unresolved author/year fields must not be guessed. The only new bibliographic
content approved for proposal is file 15's repeated title-page author and
volume identification. The previously approved file 14 author must remain.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-12 with the
  instruction, "go ahead."
- **Approved finding IDs:** N00S-R001 through N00S-R010
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no
- **Decision requested:** review and approve the cleaned registry before the
  cleanup-approval or complete stages are marked.

## Audit validation

- Resource SHA-256 after audit:
  `0954a6456e23d5e64cd565c9488080600cd8ce125b391618f0faf19672634784`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, three H2 headings, two valid Markdown tables with
  seven data rows total, five coverage-note bullets, and no Markdown links.
- Inventory coverage checked: thirteen primary chapters, one glossary, and five
  supplementary resources; all files exist.
- Confirmed supplementary metadata: file 13 title/corporate author/year; file
  14 title/author; file 15 title/volume/author; file 16 title/source note; file
  17 title/author.
- Deliberately unresolved: file 14 year; file 15 year; file 16 author/year; file
  17 year. The possible 1974 value for file 17 was rejected as non-unique
  evidence from a bibliography already classified as corrupted.

## Cleanup result and review

### Applied cleanup

- Retained the registry title, attribution-once policy, primary/supplementary
  division, both tables, and all seven inventory rows.
- Italicized source titles and normalized table delimiter spacing without
  changing any confirmed title, author, or year.
- Converted the primary range endpoints, glossary, and all five supplementary
  file references into local Markdown links.
- Clarified that `ereyfur.md` is the trilingual glossary associated with the
  1998 grammar and that no separate byline is recorded; no authorship was
  invented.
- Retained file 13's corporate attribution to `Guddiga Af Soomaaliga` and year
  1973 without expanding the compact row into its seven-person title-page list.
- Preserved the previously approved file 14 author correction and left its year
  unresolved.
- Added file 15's title-page-supported author `Maxamed Xaaji Xuseen Raabi` and
  expanded its title with **Mugga Kowaad: Ereyeynta**; its year remains
  unresolved.
- Preserved file 16's source-title styling and `Xilliyada 5aad` inventory note;
  its author and year remain unresolved.
- Preserved file 17's title and author while rejecting the non-unique 1974
  candidate from the already classified corrupted bibliography.
- Added a compact explanation of every dash in the supplementary table so
  unresolved fields are explicit rather than silently blank.
- Replaced “clean extraction” with an accurate description of files 00-12 and
  the glossary as topic-focused SLS curations.
- Removed the stale 0.9-6.3% raw-OCR percentages, review flag, and pending-
  remediation warning for files 13-17. Recorded their approved 2026-08-12
  audits and conservative SLS-native cleanups instead.
- Retained the boundary that cleanup is not page-by-page scan verification and
  linked all five supplementary review records plus the live cleanup tracker.
- Retained the *Habka Qoraalka* and *Codaynta Af Soomaaliga* collection notes
  and linked their source registries.

### Deliberately retained

- All seven original table rows and their primary/supplementary grouping.
- The confirmed 1998 primary title, author pair, and year.
- File 13 title, corporate author/compiler, and 1973; file 14 title and author;
  file 16 title/source note; and file 17 title and author.
- Dashes for file 14 year, file 15 year, file 16 author/year, and file 17 year,
  now accompanied by an explicit unresolved-metadata note.
- The English registry convention shared by the repository's other source
  inventories.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, three H2 headings, two valid four-column Markdown tables,
  seven table data rows, and five coverage-note bullets.
- Row preservation: both primary rows and all five supplementary rows remain;
  no source was added, removed, split, or merged.
- Metadata checks: file 15 author and volume detail are present; file 14 author
  remains; no year was added to files 14-17 and no author was added to file 16.
- Stale status checks: `0.9–6.3%`, raw-unrepaired-OCR wording, `flagged
  review`, and `pending a remediation decision` are absent.
- Local links: all 19 occurrences resolve across 16 unique existing targets.
- Provenance TSV validation: every row has 10 tab-separated fields.
- Post-cleanup navigation follow-up: the maintainer's approved glossary scope
  amendment replaced the TSV/Markdown pair with one Somali-English
  `ereyfur.md`. The row still records the 1998 trilingual source association
  and no-separate-byline limit; no source attribution or metadata value changed.
- Cleaned file size after that navigation follow-up: 49 lines; 355 words;
  3,104 bytes.
- Cleaned SHA-256:
  `4ff45ce8cfda98a97909bbff6825a302e9ad15e4808d88562f8c732379dae45b`.

## Primary-PDF metadata amendment — 2026-08-12

Inspection of the edition page in the local scan shows **First published
1999** (PDF page 4). The December 1998 date on the preface (PDF page 6) is not
the publication year. The active source registry has therefore been corrected
to 1999. This supersedes the repository-only 1998 conclusion above while
preserving the historical audit record.
