# Resource Audit, Cleanup, and Review Plan

**Project:** Somali Language Standard (SLS)
**Scope:** every file under `resources/`
**Strategy:** audit one whole file line by line, review the audit, apply the
approved fixes, then review the fix
**Status:** active

## 1. Purpose

`resources/` is a working Somali-language evidence library. It contains useful
grammar, dictionary, terminology, literature, and reference material, but some
files contain OCR errors, broken tables, mixed reading order, scan debris, and
inconsistent Markdown.

This process improves each file using evidence already present in this
repository. The output is a topic-focused SLS resource, not a cleaned facsimile
of a book or PDF. It does not require source scans, external evidence, or a
source-verification claim.

Book-only material—including covers, author biographies, acknowledgements,
contents pages, pagination, running headers, exercises, and unrelated
bibliographies—is excluded. Retained linguistic content may be reordered,
condensed, and cross-linked to canonical SLS resources.

## 2. Required workflow

Each resource file follows these stages, in order:

1. **Line-by-line audit** — inspect every line in the file, identify usable
   topic content and book-only material, record defects and repository evidence,
   and define the target SLS topic structure.
2. **Audit review** — the maintainer reviews and approves, rejects, or changes
   the recorded findings and proposed structure.
3. **Cleanup** — apply only the approved findings.
4. **Cleanup review** — the maintainer reviews the resulting diff and confirms
   that the approved fixes were applied correctly.
5. **Complete** — mark the file complete only after validation and both
   reviews pass.

The tracker in `RESOURCE_CLEANUP_TRACKER.md` is the authoritative queue. Its
checkboxes are updated only when a stage is complete.

## 3. Repository-only evidence rules

The repository is the sole correction evidence. For every suspicious token or
structure, search:

- the same file, especially repeated examples, headings, and parallel lists;
- related files in the same collection;
- `resources/qaamuus/` and `resources/madax-ereyo/`;
- glossary, grammar, and terminology records for the same word or concept;
- existing source notes, provenance records, and review reports in the
  repository.

The repository may support a likely correction, but it does not make a reading
certain when plausible alternatives remain. Preserve unusual, historical,
dialectal, technical, author-specific, literary, or example forms unless the
repository gives clear evidence that they are OCR damage.

### Evidence labels

| Label | Meaning | Resulting status |
| --- | --- | --- |
| `repository-supported` | A likely OCR error has one unambiguous reading supported by identified repository locations. | `in-review` |
| `structural-only` | The reading is unchanged; only Markdown/TSV layout is repaired. | Existing status remains |
| `unresolved` | Evidence is insufficient or competing readings remain. | `in-review` or `blocked` |
| `intentional-retained` | An unusual form was reviewed and deliberately left unchanged. | Existing status remains |

A `repository-supported` correction is allowed only when all of these are
true:

1. the existing token has a clear OCR-like defect;
2. one intended reading is supported by the same file or another identified
   repository resource;
3. no plausible competing reading is found in the repository; and
4. the audit records the old text, proposed text, and supporting locations.

Otherwise leave the text unchanged and record it as `unresolved`.

## 4. Line-by-line audit standard

The audit is read-only. Do not modify resource text during this stage.

For every line, check for:

- OCR substitutions, inserted characters, broken words, malformed encoding,
  and false line breaks;
- duplicate fragments, page furniture, headers, footers, and scan debris;
- missing or mixed text, incorrect reading order, and incomplete examples;
- invalid headings, lists, tables, TSV rows, and paragraph boundaries; and
- intentional forms that should be retained rather than normalized.

Each finding receives a stable ID and records:

```text
finding ID
exact line number or line range
current text or a concise non-quoting description
issue class and severity
repository locations searched
proposed action
evidence label
unresolved alternatives, if any
```

The audit also defines a file-specific structural blueprint: heading hierarchy,
paragraph and example boundaries, table/list shape, entry order, and any
layout risks. A generic template must not invent source content or merge
separate entries.

## 5. Audit review

When the whole-file audit is complete, present the review record for maintainer
approval. The maintainer may approve all findings, approve a subset, request
changes, or defer unresolved items.

No cleanup begins until this approval is recorded in the tracker and the review
record.

## 6. Cleanup and cleanup review

Cleanup applies only approved findings and the approved SLS topic structure:

1. repair approved word-level OCR defects using their recorded repository
   evidence;
2. remove book-only material, debris, and duplicate OCR fragments;
3. reorganize usable linguistic content by topic rather than source-page order;
4. replace damaged duplicated exposition with links to an existing canonical
   SLS resource when that resource already covers the topic;
5. preserve unresolved linguistic claims only when they remain useful and are
   clearly marked; and
6. add a correction-log row for every substantive change.

The cleanup record must state the findings applied, findings deferred, remaining
unresolved items, and validation results. The maintainer then reviews the diff
before the cleanup approval and completion boxes are checked.

## 7. Required records and validation

For every file, maintain:

- `docs/resource-cleanup/file-reviews/<collection>/<file>.md` — the audit,
  repository searches, structural blueprint, approvals, cleanup result, and
  unresolved items;
- `data/provenance/correction-log.tsv` — when introduced, one row for each
  substantive cleanup change: old text, new text, evidence label, repository
  locations, reviewer, status, and date; and
- `RESOURCE_CLEANUP_TRACKER.md` — stage progress.

Before cleanup approval:

1. `git diff --check` passes.
2. The file conforms to its approved structural blueprint.
3. Relevant Markdown and TSV checks pass.
4. Every substantive edit maps to an approved audit finding and evidence label.
5. Unresolved items are explicit.
6. The maintainer has reviewed the cleanup diff.

## 8. Execution order

First audit every file, without editing resource text. Audit order is:

1. `resources/naxwe/` supplementary grammars (`13`–`17`), beginning with
   `15-naxwaha-sifayneed.md`;
2. high-risk `resources/suugaan/` files;
3. `resources/erey-bixin/`;
4. `resources/qaamuus/` and derived `resources/madax-ereyo/`;
5. remaining `naxwe/`, `qoraal/`, `dhawaaq/`, and `sarfe/` files;
6. collection source notes and metadata files.

After an audit is approved, clean one approved file at a time, then obtain the
cleanup review before moving it to complete.
