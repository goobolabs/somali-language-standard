# Roadmap

The path from empty repository to the first public release of the Somali
Language Standard. Each milestone below maps to one or more phases in
[`IMPLEMENTATION_PLAN.md`](IMPLEMENTATION_PLAN.md) (the execution checklist)
and is consistent with the phased roadmap in
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) §20.

Dates are targets, not promises; the lifecycle gates in
[`GOVERNANCE.md`](GOVERNANCE.md) take precedence over the calendar.

---

## Milestone 1 — Repository foundation ✅ *(v0.1.0)*

Professional open-source scaffolding: governance, contribution workflow,
licensing structure, documentation, planning documents, GitHub templates, and
placeholder CI. The repository is ready to receive contributors.

## Milestone 1b — Resources evidence baseline ✅ *(completed 2026-08-23)*

The eight-collection source library in `resources/` completed its file-by-file
audit, cleanup, maintainer approval, and provenance closeout. The 145-file
baseline is documented in [`docs/RESOURCES.md`](docs/RESOURCES.md) and
[`RESOURCE_CLEANUP_TRACKER.md`](RESOURCE_CLEANUP_TRACKER.md).

The collections remain **accepted with limitations**. Open metadata, rights,
and deferred-source gaps stay visible in `docs/RESOURCES.md`, but they are
follow-up evidence work rather than blockers to normative drafting.

Resources milestones are **not** the same as implementation phases in
[`IMPLEMENTATION_PLAN.md`](IMPLEMENTATION_PLAN.md). See
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) §4.1.

## Milestone 2 — Core standards *(complete; public review open — target v0.2.0)*

The foundation layer of the standards catalog, drafted and moved into public
review: the Alphabet Standard (SLS-0001), Orthography Standard (SLS-0002), and
the core Grammar Standard (SLS-0003), plus the standards registry
(`standards/`) that tracks them. Standards are reviewed and advanced by the
maintainers; correctness is defended by the correction channel, open to anyone
at any stage (`SLS-0000` R17).

The evidence maps, normative drafts, compliance surfaces, and registry records
for SLS-0002 (orthography), SLS-0003 (grammar), SLS-0004 (punctuation), and
SLS-0005 (capitalization) are complete and accepted for `Proposed` publication.
SLS-0005 retains its supplementary-source limitation. The eight SLS-0003 topic
documents and their formal wrapper are included.

The proposal entered public Milestone 2 review in
[pull request #7](https://github.com/goobolabs/somali-language-standard/pull/7)
on 2026-08-23. Each `Proposed` standard now has a comment venue —
issues [#8–#13](https://github.com/goobolabs/somali-language-standard/issues) —
and a review log under `docs/standards/`. The ≥14-day comment periods cannot
close before 2026-09-06,
and the later human-review gates remain external lifecycle work; their exact
state is tracked in
[`docs/standards/MILESTONE-2-READINESS.md`](docs/standards/MILESTONE-2-READINESS.md).

## Milestone 3 — Schemas *(in progress — target v0.3)*

The machine-readable contracts everything else validates against:
`metadata-common`, lexicon entry, terminology entry, sentence pair, grammar
rule, benchmark item. First version of the CI validation pipeline enforcing
them on every pull request.

All planned shared and payload schemas are now implemented: metadata, lexicon,
terminology and its domain vocabulary, bilingual sentence pairs, grammar
rules, style examples, benchmark items, and correction pairs. The Rust
JSON/JSONL validator foundation is also implemented. Cross-reference checks,
automatic schema routing, and CI enforcement remain open.

## Milestone 4 — Lexicon *(v0.4)*

The seed dictionary: ~500–1,000 curated core entries with part of speech,
gender, plurals, definitions, and provenance — enough to prove the schema and
review workflow end-to-end and to power the spellcheck layer of CI.

## Milestone 5 — Terminology *(v0.5)*

The neologism governance process live in practice: two pilot domains
(Artificial Intelligence, Computer Science) populated with proposed and
discussed terms, the `coinage_type` and status lifecycle exercised on real
vocabulary (Domain Editors where volunteers exist; maintainer-run otherwise). Remaining launch domains open
progressively afterward.

## Milestone 6 — Translation *(v0.6)*

Translation standards drafted (EN→SO, SO→EN, technical translation, idioms)
and the first parallel sentence-pair datasets published under the
sentence-pair schema, tagged literal/natural.

## Milestone 7 — Benchmarks *(v0.7)*

First evaluation suites — grammar, spelling, translation — with the scoring
methodology (`benchmarks/SCORING.md`) recorded as a maintainer decision, and the
contamination firewall between `benchmarks/` and training data enforced in CI.

## Milestone 8 — AI resources *(v0.8)*

The AI consumption layer: system prompts, prompt templates, first instruction
and correction datasets, and the build tooling that generates RAG-ready
knowledge chunks from `spec/` and `data/` with `sls_id` citations.

## Milestone 9 — Public release *(v1.0)*

The first citable release: core standards `Stable`, docs site live, compiled
release bundles published (GitHub Releases + Hugging Face), and at least one
external project adopting SLS as its Somali reference. From here, SLS is
something an AI lab, university, or government body can implement and cite.

---

## Beyond v1.0

Dialect coverage as tagged extensions, speech and phonology resources,
historical scripts (Osmanya, Wadaad), a public leaderboard on SLS benchmarks,
and a compliance certification program — see ARCHITECTURE.md §20–§21 for the
long-term vision.
