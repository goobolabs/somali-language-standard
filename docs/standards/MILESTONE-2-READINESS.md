# Milestone 2 Review Readiness

- **Verified:** 2026-08-23
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
| SLS-0001 Alphabet | [`0001-alphabet.md`](../../spec/orthography/0001-alphabet.md) | incorporated in the standard and source library | synchronized | `Proposed` since 2026-07-10 |
| SLS-0002 Orthography | [`0002-spelling-rules.md`](../../spec/orthography/0002-spelling-rules.md) | [`SLS-0002-evidence-map.md`](SLS-0002-evidence-map.md) | synchronized | `Proposed` in PR #7 since 2026-08-23 |
| SLS-0003 Grammar | [`0018-somali-grammar-standard.md`](../../spec/grammar/0018-somali-grammar-standard.md) plus topic files 0010–0017 | [`SLS-0003-evidence-map.md`](SLS-0003-evidence-map.md) | synchronized | `Proposed` in PR #7 since 2026-08-23 |
| SLS-0004 Punctuation | [`0004-punctuation.md`](../../spec/orthography/0004-punctuation.md) | [`SLS-0004-evidence-map.md`](SLS-0004-evidence-map.md) | synchronized | `Proposed` in PR #7 since 2026-08-23 |
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
  agree on identifier, title, version, and lifecycle state;
- every `implements` path exists and the dependency graph is acyclic;
- JSON parses, local Markdown links resolve, and `git diff --check` passes.

## Lifecycle evidence and remaining gates

SLS-0000 and SLS-0001 were published as `Proposed` in
[pull request #1](https://github.com/goobolabs/somali-language-standard/pull/1)
on 2026-07-10. As of 2026-08-23, the public pull-request history contains no
comment or review on that proposal and the repository has no public issues.
The 14-day minimum has elapsed, so SLS-0001 is eligible for a recorded
`Proposed → Review` transition once active reviewers are assigned.

SLS-0002 through SLS-0005 entered formal public review in
[pull request #7](https://github.com/goobolabs/somali-language-standard/pull/7)
on 2026-08-23. Their 14-day minimum ends on 2026-09-06. The following gates
cannot be completed by repository edits alone:

1. Keep each proposal open for at least 14 days and record every substantive
   comment and its resolution.
2. Assign at least one native-speaker or linguist reviewer and one technical
   reviewer before any `Review → Candidate` transition. Implementation Phase 3
   additionally requires a linguist to be engaged as a recurring reviewer.
3. Do not mark a standard `Candidate` or `Stable` as part of Milestone 2. Those
   stages require reviewer approval, implementation experience, dependency
   maturity, and later Council action.

## Completion interpretation

The versioned content, evidence, registry, and `Proposed` transition package
required for Roadmap Milestone 2 are complete, and the package is publicly
available for formal comment in PR #7. Implementation Phases 2 and 3 remain
open for their stricter elapsed-time and reviewer criteria; this record is the
checklist for closing those gates without overstating their status.
