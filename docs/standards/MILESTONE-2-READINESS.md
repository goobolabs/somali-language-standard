# Milestone 2 Review Readiness

- **Verified:** 2026-08-23; re-verified 2026-08-30 (maintainer audit)
- **Scope:** Roadmap Milestone 2 and Implementation Phases 2–3
- **State:** Roadmap Milestone 2 complete and public review open; time-based
  and human-review gates remain for later lifecycle transitions

This record separates work that can be completed and verified in the
repository from public-comment time and human approvals that must not be
fabricated. The founding maintainer's instruction to complete Milestone 2 is
the interim-Council acceptance of SLS-0002 through SLS-0005 for the
`Draft → Proposed` transition. The transition was published in
[pull request #7](https://github.com/goobolabs/somali-language-standard/pull/7)
on 2026-08-23, starting the public-comment periods for SLS-0002 through
SLS-0005.

## Verified proposal set

| Standard | Proposal content | Evidence decision record | Registry/meta | Proposal state |
| --- | --- | --- | --- | --- |
| SLS-0001 Alphabet | [`0001-alphabet.md`](../../spec/orthography/0001-alphabet.md) | incorporated in the standard and source library | synchronized | `Review` since 2026-08-30 (`Proposed` 2026-07-10) |
| SLS-0002 Orthography | [`0002-spelling-rules.md`](../../spec/orthography/0002-spelling-rules.md) | [`SLS-0002-evidence-map.md`](SLS-0002-evidence-map.md) | synchronized | `Proposed` in PR #7 since 2026-08-23 |
| SLS-0003 Grammar | [`0018-somali-grammar-standard.md`](../../spec/grammar/0018-somali-grammar-standard.md) plus topic files 0010–0017 | [`SLS-0003-evidence-map.md`](SLS-0003-evidence-map.md) | synchronized | `Proposed` in PR #7 since 2026-08-23 |
| SLS-0004 Punctuation | [`0004-punctuation.md`](../../spec/orthography/0004-punctuation.md) | [`SLS-0004-evidence-map.md`](SLS-0004-evidence-map.md) | synchronized | `Proposed` in PR #7 since 2026-08-23; R16 narrowed in 0.2.0 |
| SLS-0005 Capitalization | [`0003-capitalization.md`](../../spec/orthography/0003-capitalization.md) | [`SLS-0005-evidence-map.md`](SLS-0005-evidence-map.md) | synchronized | `Proposed` in PR #7 since 2026-08-23; supplementary-source limitation retained |

## Repository checks completed

- every numbered standard has all eleven formal sections;
- SLS-0002, SLS-0004, and SLS-0005 have numbered requirements, positive and
  negative examples, compliance rows, and written pre-comment resolutions;
- every SLS-0003 topic rule has at least one positive and one negative
  three-column example;
- the SLS-0003 wrapper implements all eight topic files and itself;
- `VERSION` records the Milestone 2 target repository version `0.2.0`;
- `registry.json`, `REGISTRY.md`, standard front matter, and per-standard meta
  agree on identifier, title, version, and lifecycle state. Grammar topic files
  `spec/grammar/0010`–`0017` deliberately keep the local spec-note status
  `Draft` while their governing standard SLS-0003 is `Proposed`
  ([`spec/0000-index.md`](../../spec/0000-index.md)); each now carries a
  lifecycle banner saying so;
- every `implements` path exists and the dependency graph is acyclic;
- JSON parses, local Markdown links resolve, and `git diff --check` passes.

## Lifecycle evidence and remaining gates

SLS-0000 and SLS-0001 were published as `Proposed` in
[pull request #1](https://github.com/goobolabs/somali-language-standard/pull/1)
on 2026-07-10. As of 2026-08-23, the public pull-request history contains no
comment or review on that proposal and the repository has no public issues.
The 14-day minimum elapsed with no unresolved objection, so both standards
satisfied the `Proposed → Review` gate in `SLS-0000` R9; the transition was
recorded on 2026-08-30 in their review logs, and both are now at `Review`.

SLS-0002 through SLS-0005 entered formal public review in
[pull request #7](https://github.com/goobolabs/somali-language-standard/pull/7)
on 2026-08-23. Their 14-day minimum ends on 2026-09-06. The following gates
cannot be completed by repository edits alone:

1. Keep each proposal open for at least 14 days and record every substantive
   comment and its resolution. This is a calendar gate: it cannot be shortened,
   and 2026-09-06 is the earliest date the four Milestone 2 standards can close.
2. Record a maintainer review before any `Review → Candidate` transition, with
   every finding dispositioned (`SLS-0000` R9, R18). The reviewer-facing summary
   of SLS-0003 is [`SLS-0003-reviewer-packet.md`](SLS-0003-reviewer-packet.md);
   how review and error reports work is
   [`docs/REVIEWERS.md`](../REVIEWERS.md).
3. Do not mark a standard `Candidate` or `Stable` as part of Milestone 2. Those
   stages require implementation experience (the soak period), dependency
   maturity, and a second comment period.

**Governance change, 2026-08-30.** The reviewer-assignment gate that previously
appeared here — one independent linguist plus one independent technical reviewer
— was removed in `SLS-0000` 0.2.0. Those reviewers did not exist, so finished
work was blocked on recruitment rather than on content. A maintainer now
approves every transition, and the external check is the correction channel
(`SLS-0000` R17): anyone may report an error at any stage, `Stable` included, and
every substantive report gets a recorded disposition and a written resolution.
The decision and its trade-off are recorded in
[`SLS-0000-review-log.md`](SLS-0000-review-log.md) as M-2.

## Public comment venue

Until 2026-08-30 the comment periods had no venue: the proposals were published
in merged pull requests and the repository had no issues, so a reader had
nowhere to file a comment that would be counted. One tracking issue per
standard now carries each period, and one review log per standard records every
substantive comment with a written disposition.

| Standard | Comment venue | Review log | Earliest close |
| --- | --- | --- | --- |
| SLS-0000 | [#8](https://github.com/goobolabs/somali-language-standard/issues/8) | [`SLS-0000-review-log.md`](SLS-0000-review-log.md) | 2026-07-24 (elapsed) |
| SLS-0001 | [#9](https://github.com/goobolabs/somali-language-standard/issues/9) | [`SLS-0001-review-log.md`](SLS-0001-review-log.md) | 2026-07-24 (elapsed) |
| SLS-0002 | [#10](https://github.com/goobolabs/somali-language-standard/issues/10) | [`SLS-0002-review-log.md`](SLS-0002-review-log.md) | 2026-09-06 |
| SLS-0003 | [#11](https://github.com/goobolabs/somali-language-standard/issues/11) | [`SLS-0003-review-log.md`](SLS-0003-review-log.md) | 2026-09-06 |
| SLS-0004 | [#12](https://github.com/goobolabs/somali-language-standard/issues/12) | [`SLS-0004-review-log.md`](SLS-0004-review-log.md) | 2026-09-06 |
| SLS-0005 | [#13](https://github.com/goobolabs/somali-language-standard/issues/13) | [`SLS-0005-review-log.md`](SLS-0005-review-log.md) | 2026-09-06 |

New logs are copied from
[`REVIEW-LOG-TEMPLATE.md`](REVIEW-LOG-TEMPLATE.md) when a standard enters
`Proposed`.

## Maintainer audit, 2026-08-30

A structural and cross-document audit of the six `Proposed` standards produced
five findings. Four were applied as PATCH-level editorial corrections
(version `0.1.0` → `0.1.1` for SLS-0000 through SLS-0005) and are recorded in
the review logs:

- **SLS-0000** — the front-matter `implements` array omitted the standard's own
  normative file, so the document did not satisfy its own R7.
- **SLS-0001** — the Scope section cited R11 for digraph case pairing, which is
  R12; and the Definitions and Edge Cases prose wrote the glottal stop as
  U+0027, which R6 and R7 forbid in released records.
- **SLS-0003** — the eight topic files gave no indication that they were under
  public comment; each now carries a lifecycle banner.
- **SLS-0004** — an ungrammatical sentence in the long-dash edge case.

One finding is **open** and blocks SLS-0004's `Proposed → Review` transition:
R16 offers a straight `" "` / `' '` quotation profile while its own final
sentence forbids classifying straight U+0027 as quotation punctuation. The
resolution is a normative decision, not an editorial fix; it is recorded in
[`SLS-0004-review-log.md`](SLS-0004-review-log.md) and posted for comment in
[issue #12](https://github.com/goobolabs/somali-language-standard/issues/12).

No verified requirement, example, or evidence mapping changed. No standard
advanced a lifecycle stage.

## State after the 2026-08-30 governance change

| Standard | Stage | What it is waiting on |
| --- | --- | --- |
| SLS-0000 | `Review` | A recorded maintainer review before `Candidate` |
| SLS-0001 | `Review` | A recorded maintainer review before `Candidate` |
| SLS-0002 | `Proposed` | Comment minimum, earliest close 2026-09-06 |
| SLS-0003 | `Proposed` | Comment minimum only; no `open` item; twelve priority questions and all 42 per-rule verdicts complete; review evidence ready for the later `Candidate` gate |
| SLS-0004 | `Proposed` | Comment minimum; M-2 resolved in 0.2.0, no `open` item |
| SLS-0005 | `Proposed` | Comment minimum |

No standard is waiting on a person who has not been recruited. The remaining
Milestone 2 gate is calendar time.

## Completion interpretation

The versioned content, evidence, registry, and `Proposed` transition package
required for Roadmap Milestone 2 are complete, and the package is publicly
available for formal comment in PR #7. Phase 3's maintainer-review criterion is
complete. The remaining lifecycle gate for the `Proposed` standards is the
comment minimum ending 2026-09-06; this record remains the checklist for
closing that gate without overstating lifecycle status.
