# Reviewers

SLS standards cannot advance on maintainer effort alone. Two independent
reviewers are required before any standard reaches `Candidate`, and the roles
are deliberately different people:

1. **Native-speaker / linguist reviewer** — is the content correct Somali?
2. **Technical reviewer** — does it conform to the schemas, identifiers, and
   formats?

This page describes what each role involves, what it does not, and how a
review is recorded. Governance authority for the roles is defined in
[`GOVERNANCE.md`](../GOVERNANCE.md); the lifecycle gates they satisfy are in
[`standards/SLS-0000-standards-process.md`](../standards/SLS-0000-standards-process.md)
R9.

## Open positions

| Standard | Needs | Status |
| --- | --- | --- |
| SLS-0001 Alphabet | linguist + technical | unfilled — blocks `Proposed → Review` |
| SLS-0002 Orthography | linguist + technical | unfilled |
| SLS-0003 Grammar | **recurring** linguist + technical | unfilled — blocks Implementation Phase 3 |
| SLS-0004 Punctuation | linguist + technical | unfilled |
| SLS-0005 Capitalization | linguist + technical | unfilled; a Somali-primary capitalization source is the top need |

To volunteer, say so on the standard's public-comment issue (linked from
[`docs/standards/MILESTONE-2-READINESS.md`](standards/MILESTONE-2-READINESS.md))
or open an issue. No application, interview, or organizational affiliation is
required.

## Native-speaker / linguist reviewer

**You judge:** whether each numbered rule states correct Somali; whether the
examples are natural; and — most valuable — whether a rule that is right about
its example is stated so broadly that it would flag correct Somali as an error.

**You do not need to:** use git, read JSON, run tooling, or review anything
outside the standard you accepted. Answers in Somali are welcome.

**What it takes:** the first pass is a reviewer packet — every rule in one
page with its examples and a verdict column. For SLS-0003 that is
[`SLS-0003-reviewer-packet.md`](standards/SLS-0003-reviewer-packet.md),
roughly one to two hours, and a partial answer still counts. After that, you
are notified only when the standard you signed up for changes.

**Verdicts** are `OK`, `Narrow` (right here, stated too broadly), `Wrong`, or
`Unsure`. `Narrow` and `Unsure` are as useful as `Wrong`: SLS refuses to
standardize what its evidence does not support, and `not covered` is always a
valid outcome.

## Recurring reviewer

A **recurring** reviewer is a linguist reviewer who agrees to stay available
for one domain rather than a single document — the gate Implementation
Phase 3 requires for Somali grammar. In practice:

- you are asked to look at changes in that domain when they are proposed,
  typically a few times a release cycle;
- you are not on call, not expected to respond within a fixed time, and not
  responsible for keeping the standard current;
- you can step down at any time by saying so in an issue; the roster below is
  updated and no explanation is required.

## Technical reviewer

**You judge:** conformance to the formal template and the standards process —
required front-matter fields, numbered RFC 2119 requirements, compliance rows
that trace to real rules, registry and `meta/` agreement, `implements` paths
that resolve, and an acyclic dependency graph. For data records, schema
validity, identifier discipline (`sls:*` identifiers are permanent and never
reused), and complete provenance.

Familiarity with JSON Schema and GitHub is enough; no Somali is required.

## How a review is recorded

Nothing is counted unless it is written down:

1. Your comments are copied into the standard's review log
   (`docs/standards/SLS-XXXX-review-log.md`) with a disposition — `accepted`,
   `accepted-editorial`, `deferred`, `declined`, or `open` — and a written
   resolution. A comment marked `open` blocks the `Proposed → Review`
   transition.
2. When you accept a reviewer role, your name or handle — your choice — is
   added to the `reviewers` array in `standards/meta/SLS-XXXX.json` and to the
   standard's front matter, and to the roster below.
3. Reviewer approval is the gate for `Review → Candidate`, and is recorded in
   the review log's transition table with the date.

Disagreements escalate the way everything else does: to the Domain Editor,
then to the Language Council, per [`GOVERNANCE.md`](../GOVERNANCE.md). A
reviewer who does not want a rule adopted is not overruled silently — the
objection is recorded with its resolution, or the rule stays `open`.

## Attribution and licensing

Reviews happen in public issues and pull requests. Linguistic content in this
repository is licensed CC BY 4.0 and tooling MIT, as described in
[`CONTRIBUTING.md`](../CONTRIBUTING.md); by commenting you agree your
contribution may be incorporated under those terms. You are credited by the
name or handle you choose, and you may ask to be removed from the roster at
any time.

Being listed as a reviewer is not an endorsement of the whole repository, and
it does not make you responsible for what the Council later ratifies.

## Roster

| Standard | Linguist reviewer | Technical reviewer | Since |
| --- | --- | --- | --- |
| — | *(none yet)* | *(none yet)* | — |

The Language Council itself is not yet constituted; the founding maintainer
acts as interim Council until v1.0, as recorded in
[`GOVERNANCE.md`](../GOVERNANCE.md).
