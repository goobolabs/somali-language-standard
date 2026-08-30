# Implementation Plan

The team's execution checklist: every phase from repository foundation to the
first public release, with objectives, deliverables, completion criteria, and
dependencies. This plan operationalizes [`ROADMAP.md`](ROADMAP.md) and must
stay consistent with [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md), the
single source of truth.

Conventions: a phase is **done** only when every completion criterion is met.
Phases may overlap where dependencies allow, but nothing ships in a release
until its phase's criteria pass.

---

## Phase 0 — Repository foundation ✅

**Objective:** A professional, contributor-ready repository skeleton with
governance, documentation, and templates in place — before any standard
content exists.

**Deliverables**

- [x] Directory tree (`spec/`, `schemas/`, `data/*`, `ai/*`, `benchmarks/`,
      `tools/`, `examples/`, `docs/`, `.github/*`)
- [x] `README.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `GOVERNANCE.md`,
      `SECURITY.md`, `CHANGELOG.md`
- [x] `LICENSE-CODE` (MIT) and `LICENSE-DATA` (CC BY 4.0) placeholders
- [x] `docs/GETTING_STARTED.md`, `docs/STYLE_GUIDE.md`, `docs/FAQ.md`
- [x] `ROADMAP.md`, `IMPLEMENTATION_PLAN.md`
- [x] Issue templates (bug, feature, terminology proposal, standard proposal)
      and PR template
- [x] Placeholder workflows (`validate.yml`, `docs.yml`, `release.yml`)

**Completion criteria:** a first-time visitor can understand what SLS is, how
it is governed, and how to contribute, without asking a maintainer. Tagged
`v0.1.0`.

**Dependencies:** none.

---

## Phase 1 — Alphabet Standard ✅

**Objective:** Draft and publish the root of the entire dependency tree:
SLS-0001, the Somali Alphabet Standard (`spec/orthography/0001-alphabet.md`).

**Deliverables**

- [x] `standards/` registry scaffolding (`REGISTRY.md`, `registry.json`,
      `TEMPLATE.md`, `meta/`) per ARCHITECTURE.md §30
- [x] SLS-0000 (Standards Process) drafted from ARCHITECTURE.md Part II
- [x] SLS-0001 Alphabet Standard drafted in the formal template: letter
      inventory, ordering, digraphs, character set (Unicode code points),
      normative MUST/SHOULD requirements
- [x] `spec/0000-index.md` created

**Completion criteria:** SLS-0001 reaches `Proposed` with its public comment
period opened; registry entries validate against the template. *(Completed:
SLS-0000 and SLS-0001 have been promoted to `status: Proposed`, opening their
comment period.)*

**Dependencies:** Phase 0.

---

## Prerequisite — Resources baseline

**Status:** complete — accepted with documented limitations on 2026-08-23.

Normative spec work draws on the curated library in `resources/` — descriptive
evidence, not `spec/`. Scope, boundaries, limitations, and collection status
are documented in [`docs/RESOURCES.md`](docs/RESOURCES.md).

Implementation Phase 2 (orthography) depends on Milestone 1b in
[`ROADMAP.md`](ROADMAP.md). That dependency is now satisfied. Phase 1
(alphabet) did not depend on it.

**Note:** Resources curation milestones are **not** the same as the
implementation phases in this document. See
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) §4.1 and §20.1.

---

## Phase 2 — Orthography Standard

**Status:** proposal published — SLS-0002, SLS-0004, and SLS-0005 entered
public review in [pull request #7](https://github.com/goobolabs/somali-language-standard/pull/7)
on 2026-08-23; the comment period cannot close before 2026-09-06. Comments are
filed on issues [#10](https://github.com/goobolabs/somali-language-standard/issues/10),
[#12](https://github.com/goobolabs/somali-language-standard/issues/12), and
[#13](https://github.com/goobolabs/somali-language-standard/issues/13) and
recorded in the matching review logs under `docs/standards/`. One SLS-0004
question (R16's straight quotation profile versus U+0027) is `open` and blocks
that standard's `Proposed → Review` transition.

**Objective:** The spelling layer on top of the alphabet: SLS-0002
(orthography/spelling rules), plus punctuation (SLS-0004) and capitalization
(SLS-0005) drafts.

**Deliverables**

- [x] `spec/orthography/0002-spelling-rules.md` (SLS-0002, initial `Draft`)
- [x] `spec/orthography/0003-capitalization.md` (SLS-0005, initial `Draft`)
- [x] `spec/orthography/0004-punctuation.md` (SLS-0004, initial `Draft`)
- [x] Each with numbered normative rules and positive/negative examples

**Starting task:** draft `spec/orthography/0002-spelling-rules.md` as
SLS-0002. First build the cited
[`SLS-0002 evidence map`](docs/standards/SLS-0002-evidence-map.md) from
`resources/qoraal/`, `resources/dhawaaq/`, `resources/naxwe/`, and
`resources/sarfe/`; then define the document scope and numbered rules for
spelling, word boundaries, vowel length, digraphs, apostrophe/glottal-stop
usage, morphophonemic spelling, and loanword treatment. Record conflicting or
insufficient evidence as open questions instead of silently choosing a rule.

**Execution order:**

1. SLS-0002 evidence map, scope, and first draft. ✅
2. SLS-0004 evidence map and punctuation draft based primarily on
   `resources/qoraal/`. ✅
3. SLS-0005 evidence map and capitalization draft, explicitly retaining the
   current primary-source limitation. ✅
4. Pre-comment decisions recorded and all known drafting questions resolved. ✅
5. Publish the proposal for formal public comment. ✅
6. Complete the public-comment periods and resolve any new questions raised
   there.

**Completion criteria:** all three documents at `Proposed` or beyond;
SLS-0001 promoted to at least `Review`; no orthography question raised during
comment periods left without a written resolution.

**Dependencies:** Phase 1 (SLS-0002 depends on SLS-0001); completed Milestone
1b ([`docs/RESOURCES.md`](docs/RESOURCES.md)).

---

## Phase 3 — Grammar Standard

**Status:** proposal published — SLS-0003 and all eight topic documents entered
public review in [pull request #7](https://github.com/goobolabs/somali-language-standard/pull/7)
on 2026-08-23; comments are filed on
[issue #11](https://github.com/goobolabs/somali-language-standard/issues/11) and
recorded in [`docs/standards/SLS-0003-review-log.md`](docs/standards/SLS-0003-review-log.md).
The maintainer review is the **only** completion criterion still open: the eight
topic documents and the wrapper are drafted, and SLS-0003 is at `Proposed`.
[`docs/standards/SLS-0003-reviewer-packet.md`](docs/standards/SLS-0003-reviewer-packet.md)
puts all 42 rules, their examples, and twelve priority questions on one page,
which is the working surface for that review and for any outside report. The
three review batches answered Q1–Q12 on 2026-08-30. The first two per-rule
batches approved G10-R1 through G10-R4 and G11-R1 through G11-R4; 8 of 42
verdicts are complete and 34 remain. The recurring-independent-reviewer gate
that previously sat here was
removed with the 2026-08-30 governance change (`SLS-0000` 0.2.0).

**Objective:** The core grammar layer (SLS-0003): parts of speech, noun
morphology, verb system, pronouns, sentence structure, negation, questions,
common mistakes.

**Deliverables**

- [x] `spec/grammar/0010-parts-of-speech.md` through
      `spec/grammar/0017-common-mistakes.md` (eight documents per the
      architecture tree)
- [x] SLS-0003 standard wrapper listing the grammar documents it implements
- [x] Gloss-table examples (Somali | Gloss | English) for every rule

**Completion criteria:** all eight grammar documents drafted and SLS-0003 at
`Proposed`; a maintainer native-speaker review recorded in
[`docs/standards/SLS-0003-review-log.md`](docs/standards/SLS-0003-review-log.md)
covering every topic document, with every finding carrying a disposition and a
written resolution. Independent review is welcome but is not a gate
(`SLS-0000` R9); the correction channel (R17) stays open permanently.

**Dependencies:** Phases 1–2 (SLS-0003 depends on SLS-0001, SLS-0002).

Lifecycle verification and the remaining external gates are recorded in
[`docs/standards/MILESTONE-2-READINESS.md`](docs/standards/MILESTONE-2-READINESS.md).

---

## Phase 4 — Schemas

**Objective:** The machine-readable contracts for every record type, and the
CI that enforces them — the point where the repository becomes self-checking.

**Deliverables**

- [ ] `schemas/metadata-common.schema.json` (provenance block)
- [ ] `schemas/lexicon-entry.schema.json`
- [ ] `schemas/terminology-entry.schema.json`
- [ ] `schemas/sentence-pair.schema.json`
- [ ] `schemas/grammar-rule.schema.json`, `benchmark-item.schema.json`,
      `style-example.schema.json`, `correction-pair.schema.json`
- [ ] `data/terminology/_domains.json` controlled vocabulary
- [ ] `tools/validators/`: schema validator + cross-reference (duplicate /
      dangling `sls_id`) validator
- [ ] `validate.yml` implemented: lint, schema validation, cross-ref checks,
      metadata completeness on every PR

**Completion criteria:** CI fails a deliberately malformed test record and
passes a valid one; every schema carries a `schema_version`; validators run
locally with one command.

**Dependencies:** Phase 0; informed by Phases 1–3 (field semantics follow the
spec drafts).

---

## Phase 5 — Lexicon

**Objective:** The seed dictionary proving the data pipeline end-to-end:
entry → schema validation → review gate → merge.

**Deliverables**

- [ ] `data/lexicon/core/` seeded with ~500–1,000 curated entries
      (definitions, POS, gender, plurals, loanword flags, provenance)
- [ ] Loanword and morphology file skeletons per the architecture tree
- [ ] SLS-0100 (Dictionary Standard) drafted
- [ ] Review workflow exercised: every entry has a maintainer review on record
      and passes CI

**Completion criteria:** ≥500 entries merged and schema-valid; zero entries
without complete provenance; `sls:lex:` ID sequence clean (no gaps created by
renumbering, no duplicates).

**Dependencies:** Phase 4 (schemas + CI); Phase 3 recommended (POS values
should match the grammar spec).

---

## Phase 6 — Terminology

**Objective:** The neologism governance process live on real vocabulary — the
highest-leverage layer of SLS.

**Deliverables**

- [ ] Pilot domains populated: `artificial-intelligence.jsonl` and
      `computer-science.jsonl` (≥100 terms each, `coinage_type` recorded)
- [ ] Domain Editors recruited for both pilot domains *(optional — domains
      without an editor are maintained by the maintainers)*
- [ ] Terminology-proposal issue template exercised by real external
      contributors
- [ ] First batch of terms promoted `proposed → discussed → standard` by
      recorded maintainer decision (or Domain Editor sign-off where one exists)
- [ ] Remaining 18 domain files created from `_template.jsonl` (may be empty)

**Completion criteria:** at least 25 terms at `status: standard` across the
two pilot domains; every promotion traceable to a recorded review; SLS-0200
and SLS-0204 (AI, CS terminology standards) at `Proposed`.

**Dependencies:** Phases 4–5 (schema, lexicon cross-references).

---

## Phase 7 — Translation

**Objective:** Normative translation guidance plus the parallel data that
evidences it.

**Deliverables**

- [ ] `spec/translation/` documents: EN→SO (SLS-0300), SO→EN (SLS-0301),
      technical translation, idioms, false friends, literal-vs-natural
- [ ] `data/translation/` seeded: general EN↔SO pairs, technical pairs,
      idiom pairs — every record tagged `translation_type: literal|natural`
- [ ] Technical pairs consistent with `status: standard` terminology from
      Phase 6

**Completion criteria:** ≥500 reviewed sentence pairs merged; translation
spec documents at `Proposed`; CI cross-checks pair domains against
`_domains.json`.

**Dependencies:** Phases 4, 6 (technical translation is governed by standard
terminology).

---

## Phase 8 — Benchmarks

**Objective:** Evaluation suites that make SLS compliance measurable, with a
hard firewall against training-data contamination.

**Deliverables**

- [ ] `benchmarks/SCORING.md` — methodology recorded as a maintainer decision
      (including the public-dev vs. held-out-test policy decision)
- [ ] Grammar, spelling, and translation eval suites (v1) in the
      benchmark-item schema
- [ ] Contamination check in CI: no `sls:bench:` content duplicated in
      `ai/datasets/`
- [ ] SLS-0504 (Benchmark Standard) drafted

**Completion criteria:** each suite ≥100 items, schema-valid, with difficulty
labels; scoring reproducible from SCORING.md alone; contamination check
enforced and passing.

**Dependencies:** Phases 3–7 (benchmarks evaluate what the standards define).

---

## Phase 9 — AI resources

**Objective:** The consumption layer for LLMs: prompts, datasets, and
RAG-ready knowledge derived from the ratified content.

**Deliverables**

- [ ] `ai/prompts/`: Somali-assistant system prompt + parameterized prompt
      templates (translation, grammar-check, terminology lookup)
- [ ] `ai/datasets/`: first instruction dataset and correction-pair dataset
      (bad → corrected → explanation triples)
- [ ] RAG chunk generator in `tools/`: ~512-token chunks of spec + lexicon +
      terminology, each carrying source `sls_id`s
- [ ] `examples/rag-integration-guide.md` and
      `examples/load-lexicon-python.md`

**Completion criteria:** RAG chunks regenerate deterministically from source
(never hand-edited); instruction dataset passes schema + contamination
checks; a third party can follow the examples unaided.

**Dependencies:** Phases 3–8 (derived from ratified spec + data; contamination
firewall requires Phase 8).

---

## Phase 10 — First public release

**Objective:** SLS v1.0 — the first citable, implementable release.

**Deliverables**

- [ ] Core standards (SLS-0001, 0002, 0003) ratified `Stable`: soak period
      served, second comment period closed cleanly, dependencies `Stable`
- [ ] Release tooling: dataset compiler, `manifest.json` with checksums,
      `standards-manifest.json`
- [ ] `release.yml` implemented: tag-triggered bundle build + GitHub Release
- [ ] `docs.yml` implemented: documentation site built and deployed
- [ ] Hugging Face dataset mirror published
- [ ] Launch announcement + adoption outreach (≥1 external project adopting
      SLS as its Somali reference)

**Completion criteria:** `v1.0.0` tagged; every `Stable` standard's
dependency chain fully `Stable` (or waived with a recorded rationale); release
bundle downloadable and loadable with standard tooling; compliance claims
against SLS-0001/0002/0003 possible per the ARCHITECTURE.md §29 model.

**Dependencies:** all previous phases.
