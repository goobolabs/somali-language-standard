# Contributing to the Somali Language Standard

Thank you for helping build the authoritative standard for the Somali language.
This document explains how to contribute. Read it fully before opening your
first pull request.

All contributors are expected to follow our
[Code of Conduct](CODE_OF_CONDUCT.md).

---

## The two contribution tracks

SLS distinguishes between **structural** and **additive** changes:

| Track | Examples | Process |
|---|---|---|
| **Structural** | New spec document, new terminology domain, schema change, governance change | **Discussion first.** Open a GitHub Discussion or issue using the appropriate template before writing anything. |
| **Additive** | New lexicon entries, terminology entries, example sentences, translation pairs within existing schemas | **Direct pull request.** Must pass CI and the review gate below. |

When in doubt, open a discussion. Review time spent re-deriving context is
review time not spent improving the standard.

## Contribution workflow

1. **Check the architecture.** [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)
   is the single source of truth. Contributions that conflict with it will be
   asked to go through the discussion track first.
2. **Open an issue or discussion** (structural changes) or **fork and branch**
   (additive changes).
3. **Make your change**, following the [Style Guide](docs/STYLE_GUIDE.md) and
   the dataset/spec rules below.
4. **Sign off every commit** (DCO — see Commit conventions).
5. **Open a pull request** using the PR template. Link the related issue or
   discussion.
6. **Address review feedback.** Reviews are about the content, not the
   contributor.

## Branch naming

Use the area-prefixed convention from the architecture:

```text
terminology/<domain>-<short-desc>     e.g. terminology/ai-neural-network-terms
spec/<category>-<short-desc>          e.g. spec/orthography-capitalization-edge-cases
lexicon/<short-desc>                  e.g. lexicon/seed-batch-agriculture
fix/<area>-<short-desc>               e.g. fix/docs-broken-links
docs/<short-desc>                     e.g. docs/faq-licensing
```

## Commit conventions

- Use **Conventional Commits**: `type(scope): summary`
  - Types: `feat`, `fix`, `docs`, `data`, `spec`, `chore`, `ci`
  - Examples: `data(terminology): add 12 proposed AI terms`,
    `spec(grammar): clarify negation rule examples`
- Keep the summary under 72 characters, imperative mood.
- **Every commit must carry a DCO sign-off** (`git commit -s`), which adds:

  ```text
  Signed-off-by: Your Name <your@email.com>
  ```

  By signing off you assert you have the right to contribute the content under
  the project's licenses (MIT for code, CC BY 4.0 for linguistic content).

## Pull request process

1. One logical change per PR. Do not mix spec changes with data changes.
2. Fill in the PR template completely, including the licensing declaration.
3. CI must pass (schema validation, linting, cross-reference checks — once
   implemented).
4. The **review gate** must be satisfied (see below).
5. A Maintainer merges. Squash-merge is the default; the squashed commit keeps
   the DCO sign-off.

## Review process

Every content PR requires **one maintainer review** — content correctness and
schema conformance — plus passing CI. The reasoning behind the decision is
recorded, not left implicit.

Additionally:

- Promoting a terminology entry to `status: standard` is a maintainer decision,
  recorded with its rationale.
- Changing a spec document's lifecycle status requires the gates defined in
  [GOVERNANCE.md](GOVERNANCE.md), and the transition is recorded in that
  standard's review log.

## Found an error?

Reporting one is a contribution, and it does not require git, JSON, or a pull
request. This is how SLS stays correct without a panel of appointed reviewers.

1. Open an issue naming the standard and the requirement (for example
   "SLS-0003 G14-R4"), or comment on that standard's public-comment issue.
   The venues are listed in
   [`docs/standards/MILESTONE-2-READINESS.md`](docs/standards/MILESTONE-2-READINESS.md).
2. Say what is wrong. The most valuable report is that a rule is right about its
   own example but stated too broadly — an over-broad rule turns correct Somali
   into a reported error in every tool that adopts SLS.
3. Name your evidence where you can: attested usage, a dialect, a published
   grammar, a corpus. Rules change on evidence, including against the
   maintainers' own drafts.

Every substantive report is copied into that standard's review log with a
disposition and a written resolution — including the reason when it is
declined. Reports against `Stable` standards are welcome; no stage is beyond
correction. See [`docs/REVIEWERS.md`](docs/REVIEWERS.md) for how review works
end to end.

## Documentation standards

- All prose follows [`docs/STYLE_GUIDE.md`](docs/STYLE_GUIDE.md).
- Spec documents use Markdown with YAML front-matter and the required fields
  (`id`, `title`, `status`, `category`, ...).
- Write for two audiences at once: humans reading prose and machines parsing
  structure. Never put normative facts only in prose — they must exist as
  structured records too.
- English is the working language for meta-documentation; Somali content
  appears in the data and spec examples themselves.

## Dataset contribution rules

- **Format**: JSONL only — one record per line. No CSV, no XML, no
  spreadsheets.
- **Schema**: every record must validate against its schema in `schemas/`
  (once published). Do not invent fields.
- **IDs**: `sls:<type>:<zero-padded-sequence>` (e.g. `sls:lex:000123`). IDs are
  **append-only and permanent** — never renumber, never reuse, even for
  deprecated entries. Take the next available sequence number.
- **Provenance is mandatory**: every record carries `contributor`, `source`,
  `date_added`, `review_status`, and `license`. Records without provenance
  will not be merged.
- **Rights**: only contribute text you have the right to contribute. Corpora
  material must be rights-cleared or permissioned; when in doubt, don't.
- **Licensing**: all data contributions are made under **CC BY 4.0**.
- **No benchmark contamination**: content added to `benchmarks/` must never
  also appear in `ai/datasets/`, and vice versa.
- **Deprecate, don't delete**: mark superseded records
  `review_status: deprecated` rather than removing them.

## Spec contribution rules

- Spec documents live in `spec/`, numbered within reserved category blocks
  (`00xx` orthography, `01xx` grammar, `03xx` style, `05xx` translation).
  Numbers are never reused.
- Every spec document starts life as `status: Draft` and moves through the
  lifecycle defined in [GOVERNANCE.md](GOVERNANCE.md). A document reaches
  `Stable` only after its public comment periods have elapsed with no
  unresolved objection, and the transition is recorded in its review log.
- Normative requirements use RFC 2119 keywords (**MUST**, **SHOULD**,
  **MAY**) once a document enters the standards track.
- Every rule needs canonical examples — positive and negative.
- Changes to a `Stable` spec are proposals for a new version, not edits; open
  a spec-change proposal issue first.

## Questions?

Open a GitHub Discussion, or check the [FAQ](docs/FAQ.md) first.
