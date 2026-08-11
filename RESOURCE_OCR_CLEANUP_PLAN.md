# Resource Audit, Cleanup, and Structure Plan

**Project:** Somali Language Standard (SLS)
**Scope:** every file under `resources/`
**Strategy:** audit one file, then clean and structure that file
**Status:** active

## 1. Purpose

`resources/` is a working Somali-language evidence library. It contains useful
material from grammars, dictionaries, terminology lists, literature, and
reference works, but some files have OCR errors, mixed reading order, broken
tables, scan debris, and inconsistent Markdown structure.

This plan makes every resource readable, consistently structured, traceable,
and safe to use according to its evidence level. It does not silently rewrite
an author's Somali, normalize historical forms, or treat a likely correction as
source-verified when it has not been checked against a source page.

## 2. Core strategy

Every file follows the same two steps, in this order:

1. **Audit and structural blueprint** — inspect the entire file, identify
   issues, use internal resources to investigate suspicious words, and define
   the file's intended Markdown or TSV structure.
2. **Clean, structure, and validate** — repair documented issues, apply the
   structural blueprint, preserve source meaning and variants, and run checks.

Work is file-by-file. A file is not cleaned opportunistically because a word
looks wrong; it is first audited as a complete unit. This prevents unrelated
pages, columns, examples, and headings from being mixed together.

## 3. Evidence model

### 3.1 Internal repository evidence

The repository is an important source of correction evidence. When a token is
damaged, reviewers should search:

- the same file, especially repeated examples, headings, and parallel lists;
- related files in the same collection;
- `resources/qaamuus/` and `resources/wordlists/`;
- glossary, grammar, and terminology records that define the same word;
- existing source notes, provenance records, and review reports.

This evidence can identify the intended Somali word and distinguish a likely
OCR error from a legitimate historical, dialectal, technical, or author-specific
form.

### 3.2 Correction levels

Every substantive cleanup correction must have one evidence level in
`data/provenance/correction-log.tsv`.

| Level | Meaning | File status after cleanup |
| --- | --- | --- |
| `source-verified` | Exact wording and layout were checked against the source page. | May be `verified` after review. |
| `corpus-supported` | The repository gives one unambiguous reading, but the exact page was not checked. | `in-review`. |
| `structural-only` | Reading is unchanged; Markdown/TSV layout was repaired. | Existing file status remains. |
| `unresolved` | Evidence is insufficient or readings conflict. | `in-review` or `blocked`. |
| `intentional-source-form` | An unusual form was reviewed and retained as a source form. | Existing file status remains. |

Corpus-supported corrections are allowed only when all of these are true:

1. the original token has a clear OCR-like defect;
2. the intended reading is supported by the same work or an identified
   repository reference;
3. no plausible competing form is found;
4. the correction and its supporting locations are recorded.

The repository dictionary and wordlists support cleanup, but they never
override a source page. A corpus-supported file must not be called
`verified` or `authenticated` until source-page review is complete.

### 3.3 Preservation rules

Do not silently change:

- historical spelling, dialect, names, quotations, or author-specific usage;
- a readable error printed in the source;
- poetry lineation, refrains, dialogue, or intentional repetition;
- dictionary headword order, homonym markers, grammatical codes, or examples;
- terminology variants, scientific names, or source-language pairings.

Use `⟦illegible⟧`, `⟦reading?⟧`, or an `unresolved` log entry where evidence
does not support a confident reading.

## 4. File structure standard

Each file receives a file-specific structural blueprint during audit. It must
make the content readable while following the source's logical order. A generic
template must never invent a heading, split an entry, merge separate prose, or
alter verse structure.

| Resource family | Required structure |
| --- | --- |
| Collection metadata and source notes | One H1; clear source records; valid tables or lists. |
| Dictionaries | One H1; alphabet/letter sections; complete entry blocks; definitions, examples, and cross-references retained together. |
| Wordlists | One H1 where applicable; one headword per record; declared derivation and stable ordering. |
| Grammars and linguistic references | One H1; sequential chapters and sections; paragraphs; examples; paradigms; well-formed tables. |
| Terminology and glossaries | One H1; domain or alphabet sections; one complete term pair per line or table row; language pairs preserved. |
| Literature and textbooks | One H1; source-supported work, chapter, and speaker divisions; prose paragraphs; exact stanzas and verse lines. |
| Orthography, phonology, and morphology | One H1; ordered sections; notation and examples preserved; valid tables and cross-references. |
| TSV resources | UTF-8; stable header; fixed column count; one logical record per row. |

The blueprint records the target heading hierarchy, list/table shape, example
boundaries, entry or verse boundaries, source-order risks, and unresolved
layout questions.

## 5. Step 1 — Audit and structural blueprint

Step 1 is read-only. Complete it for the entire file before any cleanup.

1. Run the resource audit and collect stable finding IDs.
2. Read the whole file, not only flagged lines.
3. Identify OCR damage, broken words, malformed encoding, line-order problems,
   mixed columns, headers/footers, duplicate fragments, missing content,
   invalid Markdown/TSV, and structural inconsistencies.
4. Search the repository for suspicious words and record the supporting or
   competing readings.
5. Classify each candidate with an evidence level from §3.2.
6. Create the file's structural blueprint under
   `docs/resource-cleanup/file-reviews/`.
7. Record audit scope, finding count, unresolved count, proposed cleanup
   actions, and source availability.
8. Assign the file a queue priority:
   - **P0:** mixed reading order, major omissions, pervasive OCR garbage, or
     invalid structured data;
   - **P1:** repeated word-level OCR errors or damaged local structure;
   - **P2:** isolated defects and formatting inconsistencies;
   - **P3:** clean sample or metadata-only review.

### Step 1 deliverable

One review record per file containing:

```text
resource path
collection and resource family
audit date and reviewer
automated finding IDs
source availability and source ID
internal evidence searched
candidate corrections and evidence level
structural blueprint
unresolved items
cleanup scope and priority
```

## 6. Step 2 — Clean, structure, and validate

Start only after the Step 1 record is complete.

1. Remove confirmed scan debris, false line breaks, duplicate OCR fragments,
   and accidental headers, footers, and page numbers.
2. Restore correct reading order for prose, entries, columns, examples,
   tables, footnotes, dialogue, and verse.
3. Apply the approved structural blueprint.
4. Apply only documented source-verified or corpus-supported word corrections.
5. Preserve intentional forms and mark unresolved readings instead of guessing.
6. Add a correction-log row for every substantive change, including evidence
   level and supporting repository locations.
7. Update the file's byte count and checksum in
   `data/provenance/resource-manifest.tsv`.
8. Run the audit, collection-specific validators, Markdown/TSV checks, and
   diff review.
9. Update the review record with results, remaining issues, and the resulting
   status.

### Step 2 completion rule

A file completes this pass only when it has:

- a completed Step 1 review record;
- a readable, valid structure matching its blueprint;
- all substantive changes logged with evidence level;
- a current manifest checksum and byte count;
- no unreviewed severe audit findings; and
- an explicit count of unresolved spans.

## 7. Repository-wide execution order

### 7.1 First pass: audit every file

Run Step 1 across all files in `resources/` before broad cleanup. This creates
a complete queue and prevents attention from being driven only by the files
that happen to look worst at first glance.

The live file-by-file queue and its approval stages are in
[`RESOURCE_CLEANUP_TRACKER.md`](RESOURCE_CLEANUP_TRACKER.md). Update that
tracker as the authoritative progress record before beginning or completing a
stage.

Audit order:

1. `resources/naxwe/` high-risk supplementary grammars (`13`–`17`);
2. `resources/suugaan/` files with interleaved prose, damaged textbooks, or
   verse-risk flags;
3. `resources/erey-bixin/`, especially partial and malformed terminology
   sources;
4. `resources/qaamuus/` and derived `resources/wordlists/`;
5. the remaining `naxwe/`, `orthography/`, `phonology/`, and `morphology/`
   files;
6. collection source notes and metadata files.

### 7.2 Second pass: clean by priority

Clean audited files one at a time in P0-to-P3 order. Finish Step 2, validate,
and log the result before starting Step 2 on another file. Multiple files may
be audited before cleanup, but their content must not be mixed in a single
correction.

### 7.3 Current starting point

`resources/naxwe/15-naxwaha-sifayneed.md` is the active P0 file. Its printed
pages 12–16 have source-verified corrections in the log. Under this plan it
must receive a complete Step 1 review record and structural blueprint before
the remaining pages are cleaned.

## 8. Collection-specific checks

| Collection | Audit focus | Cleanup focus |
| --- | --- | --- |
| `qaamuus/` | entry starts, alphabetical placement, codes, homonym markers, cross-references, broken definitions | repair only documented candidates; retain source variants; preserve entry blocks |
| `wordlists/` | duplicate heads, one-item records, sort order, correspondence with dictionary | regenerate only from documented rules and eligible dictionary data |
| `naxwe/` | headings, examples, paradigms, columns, tables, page fragments | reconstruct logical chapter and example order; preserve linguistic notation |
| `erey-bixin/` | delimiter count, empty sides, language pairing, duplicate terms, coverage | restore complete pairs and tables without choosing preferred terminology |
| `suugaan/` | verse, stanza, speaker, chapter, and repeated-line preservation | remove only proven debris; preserve every meaningful line boundary |
| `orthography/`, `phonology/`, `morphology/` | notation, IPA, tables, examples, references | retain symbols and rule ordering; make tables and headings valid |

## 9. Provenance and review records

The following records are required:

- `data/provenance/resource-manifest.tsv` — file identity, source relationship,
  status, byte count, checksum, and current notes.
- `data/provenance/correction-log.tsv` — every substantive correction, old and
  new text, evidence level, internal references, source page when available,
  reviewer, status, and date.
- `docs/resource-cleanup/file-reviews/<collection>/<file>.md` — Step 1 audit,
  structural blueprint, Step 2 result, and unresolved items.
- `docs/resource-cleanup/` reports — queue progress and cross-file findings.

## 10. Quality gates

Before accepting a Step 2 cleanup:

1. `git diff --check` passes.
2. The resource audit reports no new unsuppressed fatal finding.
3. Relevant unit tests and collection validators pass.
4. Markdown tables, headings, lists, and TSV records match the blueprint.
5. Every substantive edit has an evidence label and log entry.
6. Manifest byte count and SHA-256 match the file.
7. Unresolved items are explicit.

`source-verified` changes may progress to `verified` after review.
`corpus-supported` changes remain `in-review`; they must receive source-page
checking before any `verified` or `authenticated` claim.

## 11. Completion criteria

The cleanup programme is complete when:

- every resource file has a Step 1 audit and structural blueprint;
- every P0 and P1 file has completed Step 2;
- remaining P2/P3 items have an explicit review decision or cleanup result;
- every resource has a coherent structure appropriate to its family;
- all corrections have traceable evidence levels;
- no severe OCR garbage remains unmarked; and
- manifest, correction log, review records, and automated checks agree.

## 12. Immediate next actions

1. Create the Step 1 review record and structural blueprint for
   `resources/naxwe/15-naxwaha-sifayneed.md`.
2. Complete its audit across the whole file, including the remaining
   chapter-4 interleaving.
3. Continue its Step 2 cleanup page by page, using corpus evidence for
   unambiguous word candidates and page images where available.
4. Audit the other high-risk `naxwe/` files before beginning their cleanup.
5. Build the complete repository audit queue and proceed P0 to P3.
