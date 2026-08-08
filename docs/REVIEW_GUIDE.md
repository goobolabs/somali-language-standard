# Resource Transcription Review Guide

**Applies to:** OCR cleanup and authentication in `resources/`  
**Effective:** 2026-08-09  
**Companion policy:** [`TRANSCRIPTION_POLICY.md`](TRANSCRIPTION_POLICY.md)

## 1. Review principle

Reviewers compare the repository transcription with the exact authentic book
or PDF page. They do not rewrite the author, complete missing passages from
memory, or accept AI-generated wording. When the page does not prove a reading,
the reviewer records uncertainty or blocks authentication.

## 2. Roles

### Transcriber

- aligns the resource file with source pages;
- corrects only source-verifiable OCR or conversion defects;
- records substantive changes and unresolved spans;
- runs required validators;
- cannot be the sole approver of a high-risk file.

### Somali-language reviewer

- reads every changed passage in context and on the source page;
- checks Somali characters, word boundaries, names, morphology, punctuation,
  historical spelling, dialect, and sentence continuity;
- rejects plausible but unsupported reconstructions.

### Domain reviewer

- checks specialist structure and meaning for dictionaries, grammar,
  terminology, phonology, tables, poetry, or other relevant material;
- confirms that columns, entry codes, examples, verse lines, and symbols were
  not displaced or normalized.

### Provenance and rights reviewer

- confirms title, author/compiler, edition, publisher, year, page coverage,
  scan identity, checksum or lawful stable reference, and rights status;
- confirms that restricted scans are not committed or redistributed.

### Maintainer

- checks that required reviews and automated validations passed;
- verifies the correction log and unresolved count;
- assigns the file-level status;
- approves merges but does not override missing source evidence.

One person may hold several roles for a low-risk file, but the independence
requirements below still apply.

## 3. Who may mark a file `authenticated`

Only a project maintainer may set `authenticated`, and only after the following
approvals are recorded:

1. a completed transcription review;
2. a Somali-language review by a person other than the transcriber for every
   high-risk file;
3. domain review when layout or notation can change meaning;
4. provenance and rights review;
5. passing automated validation;
6. zero unrecorded source/page gaps and zero unresolved replacement readings.

For low-risk files, a maintainer may also be the transcriber only when a second
qualified reviewer completes a recorded source-aligned sample. A file must not
be authenticated by an automated system or by a reviewer who did not have
access to the exact source pages.

## 4. Risk levels and review depth

| Risk | Typical conditions | Required review |
| --- | --- | --- |
| High | Visible OCR garbage, missing/interleaved text, poetry, damaged scan, multi-column layout, complex tables, or failed sample | Full page-by-page review plus independent Somali/domain review |
| Medium | Regular text with some OCR flags and complete legible source | All flagged pages, structural pages, first/last page, and recorded random sample; escalate on failure |
| Low | Previously verified, clean extraction, or reproducibly derived file | Recorded source-aligned sample or reproducible derivation; escalate on any severe/repeated error |
| Blocked | Exact source missing, wrong edition, illegible/missing pages, or rights prevent lawful review | No authentication; record blocker |

The review record must state the assigned risk and why.

## 5. Sequential per-file workflow

Do not begin the next file task until the current one is checked and its result
is recorded.

1. **Identify:** confirm resource path, source ID, exact edition, PDF/image index,
   printed page range, source location, and checksum.
2. **Preserve:** confirm original and raw OCR are immutable and the pre-change
   repository version is recoverable.
3. **Triage:** run read-only checks and record findings; automated flags are not
   corrections.
4. **Align:** map headings, entries, paragraphs, tables, or verses to pages and
   establish correct reading order.
5. **Correct:** compare each affected span visually and change only what the
   page proves.
6. **Log:** record substantive old/new text, page, error class, reason, and
   transcriber. Record mechanical groups with a bounded rule and occurrence
   list.
7. **Self-check:** reread the changed context and inspect the complete diff.
8. **Validate:** run encoding, structure, collection, and provenance checks.
9. **Independent review:** complete the review required by the risk level.
10. **Resolve:** fix rejected changes or retain an uncertainty/blocker.
11. **Approve:** maintainer verifies evidence and assigns status.
12. **Report:** update the phase report with issue, action, verification,
    improvement, and remaining limitations.

## 6. Page review checklist

- [ ] Exact source edition and checksum/reference match the manifest.
- [ ] PDF image index and printed page number are both recorded.
- [ ] No represented page is missing, duplicated, or out of order.
- [ ] Page headers, footers, numbers, stamps, and scan debris are distinguished
      from body text.
- [ ] Multi-column and footnote reading order matches the page.
- [ ] Paragraphs, headings, lists, dialogue, tables, and verse lineation match
      the source meaning.
- [ ] Apostrophes, quotation marks, superscripts, IPA, math, and specialist
      symbols were inspected visually.
- [ ] Historical, dialectal, inconsistent, and unusual forms remain as printed.
- [ ] Unreadable text is marked; no plausible wording was invented.
- [ ] Omissions and excluded-language passages have page ranges and reasons.
- [ ] All changes are visible in the diff and correction log.

## 7. Collection-specific checks

### `qaamuus/` and `wordlists/`

Check entry boundaries, exact headwords, homonym numbers, grammatical codes,
definitions, examples, alphabetical section, and `ld`/`eeg` references.
Wordlists are derived only after the dictionary source is authenticated.

### `naxwe/` and `morphology/`

Check examples, asterisks for ungrammatical forms, paradigms, table axes,
subscripts/superscripts, brackets, and relationships between derived tables and
cited grammar pages.

### `erey-bixin/`

Check source and target columns, term boundaries, commas/variants, scientific
names, domain headings, and records split across lines or pages.

### `suugaan/`

Check title/author attribution, speaker, stanza and line order, deliberate
repetition, refrains, dialogue, footnotes, and dialect/historical forms. Never
repair rhyme or meter from expectation.

### `orthography/` and `phonology/`

Check examples, punctuation glyphs, capitalization, IPA, diacritics, vowel
length, tables, and the boundary between quoted source content and summaries.

## 8. Correction record

Each substantive correction must eventually have these fields in the Phase 2
machine-readable log:

| Field | Required content |
| --- | --- |
| Finding ID | Stable identifier |
| Resource path | Exact repository path |
| Source ID | Manifest source identifier |
| PDF page | Image index |
| Printed page | Printed number or `none` |
| Old text | Exact pre-change text |
| New text | Exact source-supported text |
| Error class | Substitution, insertion, deletion, spacing, layout, structure, metadata, or omission |
| Evidence note | What is visible on the page |
| Transcriber | Identity |
| Reviewer | Independent reviewer when required |
| Status | Open, accepted, rejected, uncertain, or blocked |
| Date | ISO date |

Until the machine-readable log exists, the same information must be recorded
in the active phase report or pull-request record.

## 9. Rejection and escalation

Reject a proposed correction when:

- the exact source page is unavailable;
- it is based only on likelihood, a language model, spellcheck, dictionary, or
  another edition;
- it modernizes or normalizes readable source text;
- it changes layout or notation without proving the source relationship;
- it cannot be traced in the diff/correction record.

Escalate the file to high-risk/full review when a sample finds a severe error,
wrong reading order, missing page, repeated error class, wrong edition, or
unrecorded omission.

## 10. Acceptance record

The maintainer's acceptance record must include:

- file and source ID;
- final status and risk level;
- pages represented and pages actually checked;
- transcriber and reviewer identities;
- validator result or report link;
- accepted, rejected, and unresolved finding counts;
- checksum or lawful source reference;
- review date;
- remaining limitations.

An `authenticated` status is invalid if any required field is missing.
