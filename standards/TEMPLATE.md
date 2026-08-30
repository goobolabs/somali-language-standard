---
sls_id: SLS-XXXX
title: <Standard Title>
version: 0.1.0            # independent SemVer — see ARCHITECTURE.md §27
status: Draft            # Draft | Proposed | Review | Candidate | Stable | Deprecated | Archived (§25)
category: <meta | foundation | lexicon | terminology | translation | style | ai>
owner: maintainers
reviewers: []            # [linguist-reviewer, technical-reviewer] — filled in at Review (§25)
dependencies: []         # [SLS-0001, SLS-0002, ...] — must form a DAG (§28)
implements:              # physical files this standard governs (§30); paths must exist
  - <path/to/file.md>
publication_date: null   # ISO date set when the standard first reaches Stable
supersedes: null
superseded_by: null
revision_history:
  - version: 0.1.0
    date: <YYYY-MM-DD>
    change: Initial draft
---

<!--
This is the formal SLS standard template (ARCHITECTURE.md §24). Use it for any
document assigned an SLS-XXXX identifier. Ordinary spec notes that do not need
full standards-track rigor use the lighter template in ARCHITECTURE.md §5
instead.

Every section below is required. Delete these HTML comments before publishing.
Normative language uses RFC 2119 keywords (MUST, MUST NOT, SHOULD, SHOULD NOT,
MAY), written in bold uppercase so they are unambiguous and machine-greppable.
-->

## Abstract

One paragraph: what this standard governs and why it exists.

## Purpose

The problem this standard solves and who relies on it.

## Scope

What is explicitly **in scope** and — just as important — explicitly **out of
scope**.

## Definitions

Every term used normatively below, defined once, unambiguously.

## Normative Requirements

Numbered rules using RFC 2119 keywords. Number them so the Compliance
Requirements section can reference each one directly (e.g. **R1**, **R2**, …).

- **MUST** / **MUST NOT** — required for any compliance claim (§29).
- **SHOULD** / **SHOULD NOT** — strong recommendation; deviation must be justified.
- **MAY** — genuinely optional.

## Recommendations

Non-binding best practice that did not rise to a SHOULD.

## Examples

Canonical positive and negative examples, table form where useful.

## Edge Cases

Known hard cases and how the Normative Requirements resolve them.

## Compliance Requirements

The checklist an implementer or auditor uses to score a compliance claim (§29).
Each item MUST trace to a numbered Normative Requirement above.

| # | Requirement | Traces to | Level |
|---|---|---|---|
| C1 | <checklist item> | R1 | MUST |

## References

Other `SLS-XXXX` standards, external standards (ISO, BCP 47, Unicode), academic
sources.

## Revision History

Human-readable mirror of the front-matter `revision_history`.

| Version | Date | Change |
|---|---|---|
| 0.1.0 | <YYYY-MM-DD> | Initial draft |
