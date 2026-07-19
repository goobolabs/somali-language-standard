# Roadmap

The path from empty repository to the first public release of the Somali
Language Standard. Each milestone below maps to one or more phases in
[`IMPLEMENTATION_PLAN.md`](IMPLEMENTATION_PLAN.md) (the execution checklist)
and is consistent with the phased roadmap in
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) §20.

Dates are targets, not promises; the lifecycle gates in
[`GOVERNANCE.md`](GOVERNANCE.md) take precedence over the calendar.

---

## Milestone 1 — Repository foundation ✅ *(v0.1.0 — current)*

Professional open-source scaffolding: governance, contribution workflow,
licensing structure, documentation, planning documents, GitHub templates, and
placeholder CI. The repository is ready to receive contributors.

## Milestone 1b — Resources evidence baseline *(in progress)*

Eight-collection source library in `resources/`, documented in
[`docs/RESOURCES.md`](docs/RESOURCES.md). Collections are curated and
**accepted with limitations**; remaining metadata and rights gaps are tracked
there.

**Before Milestone 2 (orthography and core grammar specs):**

- Close out open items in [`docs/RESOURCES.md`](docs/RESOURCES.md) (dictionary
  metadata, rights confirmation, deferred supplements as needed)

Resources milestones are **not** the same as implementation phases in
[`IMPLEMENTATION_PLAN.md`](IMPLEMENTATION_PLAN.md). See
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) §4.1.

## Milestone 2 — Core standards *(v0.2 — after Milestone 1b)*

The foundation layer of the standards catalog, drafted and moved into public
review: the Alphabet Standard (SLS-0001), Orthography Standard (SLS-0002), and
the core Grammar Standard (SLS-0003), plus the standards registry
(`standards/`) that tracks them. Interim governance is replaced by a named
Language Council before anything is ratified `Stable`.

## Milestone 3 — Schemas *(v0.3)*

The machine-readable contracts everything else validates against:
`metadata-common`, lexicon entry, terminology entry, sentence pair, grammar
rule, benchmark item. First version of the CI validation pipeline enforcing
them on every pull request.

## Milestone 4 — Lexicon *(v0.4)*

The seed dictionary: ~500–1,000 curated core entries with part of speech,
gender, plurals, definitions, and provenance — enough to prove the schema and
review workflow end-to-end and to power the spellcheck layer of CI.

## Milestone 5 — Terminology *(v0.5)*

The neologism governance process live in practice: two pilot domains
(Artificial Intelligence, Computer Science) populated with proposed and
discussed terms, Domain Editors recruited, the `coinage_type` and status
lifecycle exercised on real vocabulary. Remaining launch domains open
progressively afterward.

## Milestone 6 — Translation *(v0.6)*

Translation standards drafted (EN→SO, SO→EN, technical translation, idioms)
and the first parallel sentence-pair datasets published under the
sentence-pair schema, tagged literal/natural.

## Milestone 7 — Benchmarks *(v0.7)*

First evaluation suites — grammar, spelling, translation — with the scoring
methodology (`benchmarks/SCORING.md`) ratified by the Council, and the
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
