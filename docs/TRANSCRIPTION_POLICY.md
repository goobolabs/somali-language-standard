# Source-Faithful Transcription Policy

**Applies to:** `resources/`  
**Effective:** 2026-08-09  
**Status:** Active for the OCR cleanup project

## 1. Core rule

Text in `resources/` is a faithful transcription of an identified authentic
book or PDF. It is not newly authored text. A person or automated system may
flag a likely error, but resource content may be changed only after the exact
reading is verified on the source page.

AI, language models, spellcheckers, dictionaries, and contextual guessing are
never sufficient evidence for a correction. They may help locate candidates;
they may not supply replacement wording.

## 2. Evidence layers

```text
original scan/PDF -> immutable raw OCR -> verified resources/ transcription
                                                |
                                                v
                               normalized records in data/
                                                |
                                                v
                                  normative rules in spec/
```

- **Original:** the exact book PDF or page images used for verification. Never
  edit them.
- **Raw OCR:** exact machine output. Preserve it unchanged.
- **Verified transcription:** readable, source-faithful Markdown or TSV in
  `resources/`.
- **Normalized data:** preferred spellings, merged records, error annotations,
  or governed terminology in `data/`.
- **Normative text:** standards adopted in `spec/`; never inferred merely by
  cleaning a source.

Source evidence and raw OCR are stored locally under `source-evidence/` as
described in its `README.md`. Those subdirectories are Git-ignored because the
material may be copyrighted or access-restricted.

## 3. What may be corrected in `resources/`

A verified transcription may correct conversion defects when the source page
is legible, including:

- wrong, missing, or extra characters introduced by OCR;
- incorrect spaces or word breaks introduced by OCR;
- false hyphenation created at a line or page break;
- duplicated or missing OCR lines;
- headers, footers, page numbers, and scan debris incorrectly inserted into the
  body;
- incorrect reading order from columns, tables, footnotes, or text boxes;
- Markdown structure that misrepresents the printed heading, table, paragraph,
  dialogue, or verse layout.

The correction log must identify the resource path, source ID, page, old text,
new text, error class, reviewer, and reason for every substantive correction.
Mechanical formatting-only changes may be grouped when their exact scope and
rule are recorded.

## 4. What must not be silently changed

Retain the source's readable wording even when it appears outdated, unusual, or
incorrect. Do not silently:

- modernize spelling, punctuation, vocabulary, or grammar;
- replace dialectal or regional forms;
- harmonize two editions or different authors;
- “correct” a factual or linguistic error printed in the book;
- add missing explanations, translations, examples, headings, or verses;
- translate or generate text omitted from the conversion;
- merge duplicates that are present in the source;
- update scientific names or terminology to current usage;
- reflow poetry in a way that changes lineation, refrain, rhythm, or speaker;
- expand unclear abbreviations without printed evidence.

Such improvements belong in linked records under `data/`, with provenance and
review status, rather than in the source transcription.

## 5. Unreadable and uncertain text

- If the source is legible, transcribe exactly what it shows.
- If characters are not legible, use `⟦illegible⟧` and record the source page
  and unresolved span in the review log.
- If a tentative reading is necessary for review, use `⟦reading?⟧`; it remains
  unresolved and cannot be treated as authenticated text.
- Never replace unreadable text with a plausible reconstruction.
- If a page is missing, cropped, or unavailable, record the gap and mark the
  file `blocked` or leave it below `authenticated` status.

The notation may be refined before Phase 3, but existing uncertainty markers
must not be removed without source-page evidence.

## 6. Page anchors and source mapping

Every transcription must be traceable to its represented source pages. During
review, record page anchors in a machine-readable mapping. The mapping must
distinguish printed page numbers from PDF image indexes.

Presentation builds may hide page anchors, but the project must retain the
mapping. A file cannot be marked `authenticated` if its page range, missing
pages, or reading order is unknown.

## 7. Layout-specific rules

### Prose and dialogue

Preserve paragraph and speaker boundaries supported by the source. Repair line
wraps created only by OCR, but do not combine distinct printed paragraphs.

### Poetry and oral literature

Preserve verse order, line breaks, stanza boundaries, refrains, speaker names,
and deliberate repetition. Apparent repetition is not an OCR duplicate unless
the page proves it.

### Tables, glossaries, and dictionaries

Preserve row and column relationships, entry order, homonym markers,
abbreviations, cross-references, and source variants. Never infer a missing
cell from neighboring rows.

### Symbols and specialist notation

IPA, mathematical symbols, superscripts, apostrophes, quotation marks, and
other specialist characters must be compared visually with the source. A
Unicode normalization or character substitution requires review when it can
change meaning.

### Historical spelling and dialect

Historical, regional, author-specific, and inconsistent forms remain as
printed. Search aids or preferred forms belong in downstream structured data.

### Non-Somali passages

Do not translate or generate omitted passages. If collection scope excludes a
language, record the exact omitted page range and reason in provenance. If a
non-Somali passage is necessary to preserve a glossary/table relationship,
transcribe it according to the declared collection scope and rights.

## 8. Bulk changes and automation

- Automated checks are read-only until a reviewer approves a bounded rule.
- Every bulk replacement requires a dry-run report and examples checked against
  source pages.
- A replacement must be limited to verified occurrences; visual similarity or
  frequency is not proof.
- Do not run global spelling correction on `resources/`.
- Preserve a recoverable pre-change state and review the diff after every task.

## 9. Authentication boundary

Cleaning proves what an identified source says. It does not prove that the
source is correct or establish a Somali language standard. Authentication
requires source identity, page mapping, checksum or stable lawful reference,
completed review, validation, rights status, and an explicit file-level status.

The roles and approval requirements are defined in `docs/REVIEW_GUIDE.md`.
