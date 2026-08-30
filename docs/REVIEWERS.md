# Review and corrections

How an SLS standard gets checked, who checks it, and how anyone can correct it.

SLS uses a **maintainer-steward model**: the maintainers review content and
approve lifecycle transitions, and correctness is defended after publication by
an open correction channel rather than before publication by appointed
gatekeepers. The authority for this is [`GOVERNANCE.md`](../GOVERNANCE.md); the
normative rules are
[`SLS-0000`](../standards/SLS-0000-standards-process.md) R9, R17, and R18.

This is a deliberate simplification of an earlier model that required an
independent linguist reviewer and an independent technical reviewer before
anything could advance. Those people did not exist, and a rule nobody can follow
is worse than a modest one that everybody can.

## What this means for a reader of the standards

A `Stable` SLS standard is **not** a claim that a panel of experts signed it off.
It is a narrower claim, and one the project can actually back:

- every rule traces to cited evidence in an evidence map;
- every objection raised so far has a written answer in a review log;
- any new objection will get one, at any stage, including `Stable`.

If you need an independent-review signal before adopting SLS, that signal is not
there yet — read the evidence maps and the review logs and judge for yourself.
Everything needed to do that is in the repository.

## Reporting an error

Reporting one is a contribution. It needs no git, no JSON, and no pull request.

1. **Open an issue** naming the standard and the requirement — for example
   "SLS-0003 G14-R4" — or comment on that standard's public-comment issue. The
   venues are listed in
   [`MILESTONE-2-READINESS.md`](standards/MILESTONE-2-READINESS.md).
2. **Say what is wrong.** The most valuable report is not "this rule is
   incorrect" but "this rule is right about its own example and wrong in
   general". These standards drive spellcheckers and grammar checkers: a rule
   stated too broadly turns correct Somali into a reported error for every user
   of every tool that adopts it.
3. **Name your evidence** where you can — attested usage, a dialect, a published
   grammar, a corpus. Rules change on evidence, including against the
   maintainers' own drafts. Where the evidence genuinely conflicts, SLS records
   the conflict and returns `not covered` rather than inventing a rule.

Somali or English, whichever is easier.

### The four verdicts

When judging a specific rule, these labels carry the most information:

| Verdict | Meaning |
| --- | --- |
| **OK** | The rule is right and the examples are natural Somali. |
| **Narrow** | Right in the example, but stated too broadly — it would flag correct Somali. |
| **Wrong** | The rule or its example is not correct Somali. |
| **Unsure** | Genuinely variable, dialectal, or you would want a second opinion. |

`Narrow` and `Unsure` are as useful as `Wrong`. SLS refuses to standardize what
its evidence does not support, so "this is real variation" is an outcome the
process wants to hear, not an inconvenience.

## How a report is handled

Per `SLS-0000` R17, a substantive report **must** be recorded in that standard's
review log (`docs/standards/SLS-XXXX-review-log.md`) with a disposition and a
written resolution, and must not be closed without one:

| Disposition | Meaning |
| --- | --- |
| `accepted` | The change was made in the standard. |
| `accepted-editorial` | Applied as a PATCH-level fix with no normative effect. |
| `deferred` | Valid, but assigned to a later standard or version; the target is named. |
| `declined` | Not adopted; the recorded reason states why. |
| `open` | Under discussion; blocks the `Proposed → Review` transition. |

A confirmed error that invalidates a MUST-level requirement produces a corrected
version, or returns the standard to `Review`. A declined report keeps its reason
on the record, so a later reader can disagree with the reasoning rather than
guess at it.

You are credited by the name or handle you choose, and you can ask to be left
uncredited.

## Reviewing a whole standard

To go through an entire standard rather than a single rule, there is a packet for
that: every rule with its conforming and non-conforming examples in one table,
plus the questions that most need a native-speaker judgment. The first one is
[`SLS-0003-reviewer-packet.md`](standards/SLS-0003-reviewer-packet.md) — roughly
one to two hours, and a partial answer still counts.

## Maintainer review

Every content pull request gets one maintainer review — content correctness and
schema conformance — plus passing CI. Lifecycle transitions are approved by a
maintainer and recorded in the standard's review log with date, approver, and the
evidence satisfying the gate (`SLS-0000` R18).

A maintainer review of the maintainers' own draft is not independent, and the
review logs label it as what it is — `maintainer audit`, `maintainer review` —
never dressed up as third-party validation. Where a maintainer is party to a
disagreement, both sides are recorded verbatim.

## Independent reviewers, if they turn up

Nothing in the process requires them, and no standard's progress waits on
recruitment. But an independent review is still worth more than a maintainer's,
and anyone offering one is welcome: say so on the
[call for reviewers](https://github.com/goobolabs/somali-language-standard/issues/16)
or on any standard's comment issue. An independent review is recorded as such,
with the reviewer named in that standard's `reviewers` metadata, so a later
reader can see which standards have had outside eyes and which have not.

If enough reviewers accumulate to justify a Language Council or per-domain
editors, those are established through the ordinary change process
([`GOVERNANCE.md`](../GOVERNANCE.md)). Neither is promised on a schedule.

## Attribution and licensing

Reviews and error reports happen in public issues and pull requests. Linguistic
content in this repository is licensed CC BY 4.0 and tooling MIT, as described in
[`CONTRIBUTING.md`](../CONTRIBUTING.md); by commenting you agree your
contribution may be incorporated under those terms.
