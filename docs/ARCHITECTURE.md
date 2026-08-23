# Somali Language Standard (SLS) — Architecture & Implementation Plan

**Status:** Draft v0.1 — pre-implementation design
**Scope:** This document is the master specification for the SLS project. It does not contain implementation code. It defines *what* the repository is, *how* it is organized, *what formats it uses*, and *how it evolves*.
**Extended 2026-07-02:** Part II adds the SLS Standards Framework (§22–§32) as a new governing layer. Part I (§1–§21, plus the closing Summary) is unchanged.

---

## Table of Contents

1. [Vision & Design Principles](#1-vision--design-principles)
2. [Governance Model](#2-governance-model)
3. [Repository Architecture (Full Tree)](#3-repository-architecture-full-tree)
4. [Directory-by-Directory Reference](#4-directory-by-directory-reference)
5. [The Spec Layer (RFC-style normative documents)](#5-the-spec-layer)
6. [Data Formats](#6-data-formats)
7. [Core Schemas](#7-core-schemas)
8. [Naming Conventions](#8-naming-conventions)
9. [Terminology Domains & the Coining Process](#9-terminology-domains--the-coining-process)
10. [Translation Standards](#10-translation-standards)
11. [Style Guides](#11-style-guides)
12. [AI Resources Layer](#12-ai-resources-layer)
13. [Benchmarks & Scoring](#13-benchmarks--scoring)
14. [Versioning Strategy](#14-versioning-strategy)
15. [Licensing](#15-licensing)
16. [Contribution Workflow](#16-contribution-workflow)
17. [CI/CD Pipeline](#17-cicd-pipeline)
18. [Validation & Automation Tools](#18-validation--automation-tools)
19. [Developer / API Surface](#19-developer--api-surface)
20. [Roadmap & Phases](#20-roadmap--phases)
21. [Future Expansion](#21-future-expansion)

**Part II — The SLS Standards Framework** (extension, added 2026-07-02)

22. [Standards Framework Overview](#22-standards-framework-overview)
23. [Standard Numbering System](#23-standard-numbering-system)
24. [The Formal Standard Template](#24-the-formal-standard-template)
25. [Standard Lifecycle](#25-standard-lifecycle)
26. [Governance & Publication Workflow](#26-governance--publication-workflow)
27. [Standard Versioning (Fourth Axis)](#27-standard-versioning-fourth-axis)
28. [Dependency Model](#28-dependency-model)
29. [Compliance Model](#29-compliance-model)
30. [Repository Integration](#30-repository-integration)
31. [Initial Standards Registry (Launch Set)](#31-initial-standards-registry-launch-set)
32. [Long-Term Scaling Vision](#32-long-term-scaling-vision)

---

## 1. Vision & Design Principles

SLS is to the Somali language what an RFC series or a language specification (ECMA-262, Unicode TR) is to a technical domain: a versioned, normative, machine-readable reference that any system — human or AI — can implement against. It is not a corpus dump. It is a **standard with a compliance surface**.

Five principles drive every decision below:

| Principle | Meaning |
|---|---|
| **Machine-first, human-readable** | Every fact SLS asserts (a word, a grammar rule, a terminology mapping) exists as a structured record (JSON/JSONL) with a schema. Markdown prose explains and contextualizes but is never the sole source of truth. |
| **Provenance over volume** | Every record carries `source`, `contributor`, `review_status`, `date`. A smaller, trustworthy standard beats a large, unverifiable one — this is what makes it *citable* by AI vendors. |
| **Additive by default** | The standard should almost never need to delete data, only deprecate it. Structure decisions today must not block tomorrow's dialects, scripts, or modalities (speech, sign). |
| **Separation of normative vs. generated** | Hand-authored source data lives in `data/`, `spec/`, `ai/`, `benchmarks/`. Compiled, distributable bundles live in `releases/` and are never hand-edited — they are build artifacts. |
| **One canonical ID space** | Every entry, in every domain, resolves to a stable, permanent `sls_id`. IDs are never reused or renumbered, even when content is deprecated. |

---

## 2. Governance Model

A standard without governance turns into an unreviewable pile of PRs. SLS adopts a lightweight **council model**, similar to CPython PEP editors or W3C working groups — enough authority to resolve disputes, not so much that it gates contribution.

- **Language Council** — final authority on orthography, grammar, and disputed terminology. Small (3–7 people), named in `GOVERNANCE.md`, term-limited.
- **Domain Editors** — one per terminology domain (AI/ML, Medicine, Law, etc.), responsible for reviewing that domain's glossary PRs. Can be a single trusted contributor initially; domains without an editor default to Council review.
- **Maintainers** — own CI, schemas, tooling, releases. Technical, not linguistic, authority.

Every non-trivial change to `spec/` starts as a **Draft** (front-matter `status: Draft`), gets discussed in a GitHub Discussion/issue, and only a Council member can flip it to `status: Stable`. This mirrors RFC/PEP lifecycle and gives the project the "standard" credibility it needs — vendors can point at a `Stable` spec and know it won't shift under them.

---

## 3. Repository Architecture (Full Tree)

```
somali-language-standard/
├── README.md
├── GOVERNANCE.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── CHANGELOG.md
├── LICENSE-CODE                     # MIT — tooling, schemas, scripts
├── LICENSE-DATA                     # CC BY 4.0 — all linguistic content
├── VERSION                          # single source of truth, e.g. "1.0.0"
│
├── .github/
│   ├── workflows/
│   │   ├── validate.yml             # schema + lint + cross-ref checks on every PR
│   │   ├── build-release.yml        # compiles releases/ bundle on tag push
│   │   ├── docs-deploy.yml          # builds & publishes docs site
│   │   └── benchmark-check.yml      # structural validation of benchmark PRs
│   ├── ISSUE_TEMPLATE/
│   │   ├── new-terminology.md
│   │   ├── spec-change-proposal.md
│   │   └── bug-report.md
│   └── PULL_REQUEST_TEMPLATE.md
│
├── spec/                            # normative documents — the "standard" itself
│   ├── 0000-index.md
│   ├── orthography/
│   │   ├── 0001-alphabet.md
│   │   ├── 0002-spelling-rules.md
│   │   ├── 0003-capitalization.md
│   │   └── 0004-punctuation.md
│   ├── grammar/
│   │   ├── 0010-parts-of-speech.md
│   │   ├── 0011-noun-morphology-gender-plurals.md
│   │   ├── 0012-verb-system-tense-aspect-mood.md
│   │   ├── 0013-pronouns.md
│   │   ├── 0014-sentence-structure-word-order.md
│   │   ├── 0015-negation.md
│   │   ├── 0016-question-formation.md
│   │   └── 0017-common-mistakes.md
│   ├── style/
│   │   ├── 0030-formal-writing.md
│   │   ├── 0031-government-documents.md
│   │   ├── 0032-academic-writing.md
│   │   ├── 0033-news-journalism.md
│   │   ├── 0034-technical-documentation.md
│   │   ├── 0035-marketing-copy.md
│   │   ├── 0036-social-media.md
│   │   └── 0037-conversational-somali.md
│   └── translation/
│       ├── 0050-en-so-guidelines.md
│       ├── 0051-so-en-guidelines.md
│       ├── 0052-idioms-and-expressions.md
│       ├── 0053-false-friends.md
│       ├── 0054-technical-translation.md
│       └── 0055-literal-vs-natural.md
│
├── schemas/                         # JSON Schema (2020-12) — one per record type
│   ├── metadata-common.schema.json  # shared $defs: provenance, license, review_status
│   ├── lexicon-entry.schema.json
│   ├── terminology-entry.schema.json
│   ├── sentence-pair.schema.json
│   ├── grammar-rule.schema.json
│   ├── style-example.schema.json
│   ├── benchmark-item.schema.json
│   └── correction-pair.schema.json
│
├── data/
│   ├── lexicon/
│   │   ├── core/                    # a.jsonl … z.jsonl, curated dictionary
│   │   ├── loanwords.jsonl
│   │   ├── morphology/
│   │   │   ├── noun-paradigms.jsonl
│   │   │   └── verb-conjugations.jsonl
│   │   └── frequency/
│   │       └── word-frequency-list.jsonl
│   ├── terminology/                 # one JSONL per domain (see §9)
│   │   ├── artificial-intelligence.jsonl
│   │   ├── machine-learning.jsonl
│   │   ├── ... (20 domains total)
│   │   └── _template.jsonl
│   ├── corpora/
│   │   ├── example-sentences.jsonl
│   │   ├── literature-excerpts/     # rights-cleared only
│   │   └── news-samples/            # rights-cleared / permissioned only
│   └── translation-pairs/
│       ├── general/en-so.jsonl
│       ├── technical/en-so-tech.jsonl
│       └── idioms/idioms-en-so.jsonl
│
├── ai/
│   ├── system-prompts/somali-assistant-persona.md
│   ├── instruction-datasets/instructions-v1.jsonl
│   ├── fine-tuning/sft-somali-v1.jsonl
│   ├── rag-knowledge/chunks/        # pre-chunked spec + lexicon for retrieval
│   ├── prompt-templates/templates.yaml
│   └── correction-datasets/grammar-correction-pairs.jsonl
│
├── benchmarks/
│   ├── grammar/grammar-eval-v1.jsonl
│   ├── spelling/spelling-eval-v1.jsonl
│   ├── translation/mt-eval-en-so-v1.jsonl
│   ├── reasoning/reasoning-so-v1.jsonl
│   ├── reading-comprehension/rc-so-v1.jsonl
│   ├── writing-quality/writing-rubric.md
│   ├── terminology-consistency/term-consistency-v1.jsonl
│   └── SCORING.md
│
├── resources/                       # curated source evidence (pre-spec); see §4.1
│   ├── README.md
│   ├── qaamuus/                     # monolingual dictionary
│   ├── madax-ereyo/                 # bare headword baseline
│   ├── naxwe/                       # grammar reference
│   ├── erey-bixin/                  # technical terminology glossaries
│   ├── suugaan/                     # literature, proverbs, poetry
│   ├── qoraal/                      # writing and punctuation evidence
│   ├── dhawaaq/                     # sound system and IPA evidence
│   └── sarfe/                       # inflectional paradigm tables
│
├── tools/                           # designed here, implemented in a later phase
│   ├── validators/
│   ├── build/
│   ├── exporters/
│   └── docs-generator/
│
├── docs/                            # human-facing documentation site source
│   ├── index.md
│   ├── getting-started.md
│   ├── ARCHITECTURE.md              # this document
│   ├── RESOURCES.md                 # curated evidence library (implementation)
│   └── faq.md
│
├── examples/
│   ├── load-lexicon-python.md
│   └── rag-integration-guide.md
│
└── releases/                        # generated only, never hand-edited
    └── v1.0.0/
        ├── sls-lexicon-v1.0.0.jsonl
        ├── sls-terminology-v1.0.0.jsonl
        └── manifest.json
```

---

## 4. Directory-by-Directory Reference

| Directory | Purpose | Who edits it | Hand-authored or generated |
|---|---|---|---|
| `spec/` | Normative prose: the actual rules of Somali orthography, grammar, style, translation. Every AI vendor "implementing SLS" starts here. | Language Council + Domain Editors | Hand-authored |
| `schemas/` | JSON Schema contracts every dataset file must satisfy. Change here = potential MAJOR version bump. | Maintainers | Hand-authored |
| `data/lexicon/` | The dictionary: word, definitions, POS, morphology, frequency, loanword flags. | Contributors + linguist reviewers | Hand-authored |
| `data/terminology/` | Domain glossaries (AI, medicine, law, etc.) — where most long-term value accrues, since this vocabulary barely exists in Somali today. | Domain Editors | Hand-authored |
| `data/corpora/` | Example sentences and rights-cleared text used for grounding, RAG, and training context. | Contributors | Hand-authored / curated |
| `data/translation-pairs/` | Parallel EN↔SO sentence/phrase pairs for MT training and eval. | Contributors + reviewers | Hand-authored |
| `ai/` | Everything shaped for direct AI consumption: system prompts, instruction/fine-tuning sets, RAG chunks, correction pairs. | Maintainers + contributors | Hand-authored, partly derived from `spec/` + `data/` |
| `benchmarks/` | Eval suites + scoring methodology, kept separate from training data to avoid contamination. | Maintainers + Council | Hand-authored |
| `resources/` | The canonical linguistic source library and evidence base. Eight curated collections (qaamuus, naxwe, erey-bixin, suugaan, qoraal, dhawaaq, sarfe) plus derived madax-ereyo. Documented in [`docs/RESOURCES.md`](RESOURCES.md). | Maintainers | Curated source data |
| `tools/` | Validator/build/export scripts. Empty in the planning phase; scaffolded in Phase 0 implementation. | Maintainers | Code (future) |
| `docs/` | The public documentation site (built from `spec/` + narrative docs). | Maintainers | Generated + hand-authored |
| `releases/` | Compiled, versioned, distributable bundles — the thing most consumers actually download. | CI only | Generated |

### 4.1 Resources evidence library

Before normative work in `spec/` or structured records in `data/`, the project
curates **descriptive evidence** in `resources/`. Implementation documentation:
[`docs/RESOURCES.md`](RESOURCES.md).

**Layer separation:**

```text
resources/  →  curated source evidence (descriptive)
data/       →  structured records (later)
spec/       →  normative rules (later)
ai/         →  downstream material derived later
```

**Collection layout** (every curated collection under `resources/`):

```text
00-sources.md              collection inventory (title, author, year)
00-<meta>.md               optional second 00-file (e.g. qaamuus/00-abbreviations.md)
01-<topic>.md … NN-<topic>.md   numbered content files, flat per collection
README.md                  charter and conventions
```

Live file map: [`resources/README.md`](../resources/README.md). Full scope,
boundaries, limitations, and baseline status: [`docs/RESOURCES.md`](RESOURCES.md).

The resources baseline completed on 2026-08-23. Orthography implementation in
[`IMPLEMENTATION_PLAN.md`](../IMPLEMENTATION_PLAN.md) can therefore proceed
under Milestone 2 in [`ROADMAP.md`](../ROADMAP.md).

**Phase naming disambiguation** — three different “Phase 2” labels exist in this
repository:

| Name | Document | Meaning |
| --- | --- | --- |
| Resources baseline | [`docs/RESOURCES.md`](RESOURCES.md) | Curated evidence library — accepted with limitations |
| Implementation Phase 2 | `IMPLEMENTATION_PLAN.md` | Draft SLS-0002/0004/0005 orthography specs |
| Architecture Phase 2 | §20 table below | “v1.0 Standard” milestone (~6 months) |

---

## 5. The Spec Layer

`spec/` is deliberately RFC-shaped: numbered files, monotonic IDs that are never reused, and a lifecycle status in front-matter.

**Template for every spec document:**

```markdown
---
id: 0012
title: Verb System — Tense, Aspect, Mood
status: Stable        # Draft | Review | Stable | Deprecated
since_version: 1.0.0
category: grammar
supersedes: null
---

## Summary
One-paragraph normative statement.

## Rule
The actual rule(s), numbered, with canonical examples.

## Examples
| Somali | Gloss | English |
|---|---|---|

## Edge Cases & Common Mistakes
## Related
- [[0013-pronouns]]
- [[0014-sentence-structure-word-order]]
```

Numbering blocks are reserved by category (`00xx` orthography, `01xx` grammar, `03xx` style, `05xx` translation) so new categories (e.g. `07xx` phonology, `08xx` dialects) can be added later without renumbering anything — an append-only ID space, same discipline as the dataset IDs in §7.

---

## 6. Data Formats

| Format | Used for | Why |
|---|---|---|
| **JSON Lines (`.jsonl`)** | All datasets: lexicon, terminology, corpora, translation pairs, benchmarks, fine-tuning data | Streams line-by-line, diffs cleanly in git (one entry = one line = one diff hunk), is the native format of every ML data pipeline (HF `datasets`, JAX/PyTorch loaders). |
| **JSON** | Schemas, manifests, config, small fixed structures (e.g. `prompt-templates`) | Universally parseable, no ambiguity. |
| **Markdown + YAML front-matter** | `spec/`, style guides, docs site | Human-readable normative prose; front-matter makes status/version machine-extractable without parsing prose. |
| **YAML** | `templates.yaml`, CI config | Used only where a human is expected to hand-edit config; never for datasets (indentation-sensitivity is a liability at scale). |

Explicitly **not used**: CSV (silently corrupts on commas/quotes inside Somali text, no nesting, no types), XML (verbose, no ecosystem benefit here), binary formats (breaks git diffing and human review — the whole point is auditability).

---

## 7. Core Schemas

All schemas share a **common metadata block** via `$ref` to `metadata-common.schema.json`, so provenance is enforced identically everywhere:

```json
{
  "$id": "metadata-common.schema.json",
  "$defs": {
    "provenance": {
      "type": "object",
      "required": ["contributor", "date_added", "review_status", "license"],
      "properties": {
        "contributor": { "type": "string" },
        "reviewers": { "type": "array", "items": { "type": "string" } },
        "date_added": { "type": "string", "format": "date" },
        "date_modified": { "type": "string", "format": "date" },
        "review_status": { "enum": ["draft", "reviewed", "verified", "deprecated"] },
        "source": { "type": "string" },
        "license": { "type": "string" },
        "schema_version": { "type": "string" },
        "since_version": { "type": "string" }
      }
    }
  }
}
```

**Lexicon entry** (`lexicon-entry.schema.json`) — one line of `data/lexicon/core/*.jsonl`:

```json
{
  "sls_id": "sls:lex:000123",
  "word": "baabuur",
  "part_of_speech": "noun",
  "gender": "masculine",
  "plural": "baabuurro",
  "ipa": "baːbuːr",
  "dialect": "standard",
  "definitions": [
    { "sense": 1, "en": "vehicle, car", "so_gloss": "gaari lagu safro" }
  ],
  "synonyms": ["gaari"],
  "antonyms": [],
  "is_loanword": true,
  "loan_origin": "Italian (vapore)",
  "example_sentences": ["sls:sent:004421"],
  "frequency_rank": 812,
  "metadata": { "...": "see metadata-common" }
}
```

**Terminology entry** (`terminology-entry.schema.json`) — see §9 for the full field rationale.

**Sentence pair** (`sentence-pair.schema.json`) — used by `translation-pairs/` and `corpora/example-sentences.jsonl`:

```json
{
  "sls_id": "sls:pair:001987",
  "en": "The model was trained on a large dataset.",
  "so": "Qaabka waxaa lagu tababaray xog aad u weyn.",
  "register": "technical",
  "translation_type": "natural",
  "domain": "machine-learning",
  "metadata": { "...": "..." }
}
```

**Benchmark item** (`benchmark-item.schema.json`) — generic envelope shared by all eval suites:

```json
{
  "sls_id": "sls:bench:grammar:000045",
  "task": "grammar-correction",
  "input": "Waan aaday dukaanka shalay.",
  "expected_output": "Waxaan aaday dukaanka shalay.",
  "explanation": "Missing subject-marking waxaa-construction.",
  "difficulty": "intermediate",
  "metadata": { "...": "..." }
}
```

Every domain-specific schema is a thin extension of this pattern — same discipline, different payload fields. Full schema files are written in the implementation phase, not in this planning document; the shapes above are the contract they must satisfy.

---

## 8. Naming Conventions

- **Files**: lower-kebab-case, always `.jsonl` for data, `.md` for prose, `.schema.json` for schemas. Domain glossary files are named after the domain in English kebab-case (`machine-learning.jsonl`), never abbreviated.
- **IDs**: `sls:<type>:<zero-padded-sequence>`, e.g. `sls:lex:000123`, `sls:term:ai:000045`, `sls:bench:grammar:000012`. IDs are **append-only and permanent** — never renumbered or reused, even on deprecation. This is the single most important rule for long-term stability: external systems will cite `sls_id`s directly.
- **Language/locale tags**: BCP 47. Standard Somali is `so`. Dialects use private extensions until formally namespaced (§21): `so-x-maay`, `so-x-benadiri`. English targets use `en`.
- **Domain codes**: short, stable, kebab-case slugs (`ai`, `ml`, `cybersec`, `medicine`, `law`...) defined once in `data/terminology/_domains.json` (a controlled vocabulary — new domains require a Council-approved PR, not an inline string anyone can invent).
- **Branch/PR naming**: `terminology/<domain>-<short-desc>`, `spec/<category>-<short-desc>`, `fix/<area>-<short-desc>`.

---

## 9. Terminology Domains & the Coining Process

This is the highest-leverage part of SLS: most of this vocabulary (AI, ML, cybersecurity, cloud computing, blockchain...) **does not yet exist in standard Somali**. Treating this as a simple glossary undersells the problem — SLS needs an explicit **neologism governance process**, similar in spirit to Iceland's Árnastofnun or the Académie française, adapted to open-source speed.

**Terminology entry fields:**

```json
{
  "sls_id": "sls:term:ai:000045",
  "domain": "artificial-intelligence",
  "en_term": "neural network",
  "so_term": "shabakad neerawi ah",
  "so_term_alternatives": ["shabakad dareen-jir ah"],
  "definition_en": "A computing system inspired by biological neural networks.",
  "definition_so": "...",
  "part_of_speech": "noun phrase",
  "coinage_type": "calque",          
  "status": "proposed",              
  "example_en": "The neural network was trained on images.",
  "example_so": "Shabakadda neerawiga ah waxaa lagu tababaray sawirro.",
  "reviewers": [],
  "metadata": { "...": "..." }
}
```

`coinage_type` ∈ `{loanword, calque, existing-word-repurposed, neologism}` — this field alone makes SLS more valuable than any prior Somali glossary, because it records *how* a term was decided, not just what it is.

`status` lifecycle: `proposed → discussed → standard` (or `rejected`). Only Domain Editors / Council can promote to `standard`. Multiple `so_term_alternatives` are allowed to persist at `proposed` indefinitely if genuine dialectal or stylistic variation exists — SLS documents real usage, it doesn't force false consensus.

**The 20 launch domains** (each its own JSONL file, same schema, one Domain Editor each once available): Artificial Intelligence, Machine Learning, Data Science, Cybersecurity, Computer Science, Software Engineering, Cloud Computing, Networking, Blockchain, Medicine, Law, Government, Finance, Education, Agriculture, Engineering, Mathematics, Physics, Chemistry, Biology.

---

## 10. Translation Standards

`spec/translation/` holds the normative guidance; `data/translation-pairs/` holds the evidence (parallel sentence pairs tagged by the same categories):

- **EN→SO / SO→EN guidelines** — directionality matters: Somali VSO/SOV flexibility, waxaa-focus constructions, and evidentiality markers don't map 1:1 onto English syntax, so each direction gets its own document rather than one "translation guide."
- **Idioms & expressions** — stored as *pairs*, never literal glosses, with a `literalness` field so training pipelines can choose to exclude idioms if they only want literal-pair data.
- **False friends** — an explicit dataset (`sls:falsefriend:*`) of lookalike words that mislead (e.g. loanwords that shifted meaning), because these are exactly what breaks naive MT and what a spellchecker/grammar checker needs to flag.
- **Technical translation** — governed by whatever `data/terminology/*` says is `status: standard`; a translation-consistency benchmark (§13) checks compliance automatically.
- **Literal vs. natural** — every `sentence-pair` record carries `translation_type: literal|natural` so downstream consumers (MT training vs. gloss-learning tools) can filter appropriately instead of guessing.

---

## 11. Style Guides

Eight registers, one file each under `spec/style/`, each following the same skeleton: **tone**, **sentence length norms**, **vocabulary register** (formal/loanword tolerance), **honorifics/pronoun conventions**, **do/don't examples**. Government and academic writing additionally reference the terminology domains they most depend on (`law`, `government`, `education`) so the style guide and glossary stay cross-linked rather than duplicating definitions.

---

## 12. AI Resources Layer

`ai/` is the layer purpose-built for LLM consumption, kept separate from raw `data/` because it's *derived and shaped*, not primary source material:

| Subfolder | Contents | Derived from |
|---|---|---|
| `system-prompts/` | Ready-to-use persona/system prompts for "a Somali-fluent assistant that follows SLS conventions" | `spec/` |
| `instruction-datasets/` | Instruction→response pairs in Somali for SFT | `data/`, `spec/`, native contribution |
| `fine-tuning/` | Larger curated SFT sets, versioned like model checkpoints | `instruction-datasets/` + review |
| `rag-knowledge/chunks/` | Pre-chunked (~512 token), embedding-ready fragments of `spec/` + `lexicon/` + `terminology/`, each chunk carrying its source `sls_id`s for citation | Automated build from `spec/` + `data/` |
| `prompt-templates/` | Parameterized templates (translation, grammar-check, terminology lookup) any app can drop in | Hand-authored |
| `correction-datasets/` | (bad, corrected, explanation) triples for grammar/spelling correction models | Contributor-submitted + benchmark overlap |

`rag-knowledge/` is intentionally a **build artifact**, not hand-edited — regenerated by tooling whenever `spec/` or `data/` changes, so it never drifts out of sync with the source of truth.

---

## 13. Benchmarks & Scoring

Benchmarks live apart from training data specifically to prevent contamination — a `benchmarks/` item must never also appear in `ai/fine-tuning/`. If the project later wants a leaderboard, the underlying policy question (public dev set vs. held-out test set, GLUE/SuperGLUE-style) belongs in `benchmarks/SCORING.md` as a Council decision, not something contributors decide ad hoc.

| Suite | Scoring method |
|---|---|
| Spelling | Exact-match against `data/lexicon` (canonical forms) |
| Grammar | Rule-based checker (derived from `spec/grammar/`) + LLM-judge rubric fallback for rules too fuzzy to encode |
| Translation | chrF++ primary (better than BLEU for a morphologically rich, low-resource language), BLEU secondary, human adequacy/fluency 1–5 for calibration samples |
| Reasoning in Somali | Accuracy on MCQ / exact-match on short-answer |
| Reading comprehension | Exact-match / token-F1 on extractive QA |
| Writing quality | Rubric-based LLM-judge, periodically calibrated against human ratings; report inter-rater agreement (Cohen's κ) so the rubric's reliability is itself measured |
| Terminology consistency | Precision/recall of domain-`standard` terms against a model's output on domain text — directly exercises §9's data |

`benchmark-item.schema.json` (§7) is the shared envelope; each suite adds task-specific fields on top.

---

## 14. Versioning Strategy

Three independent version axes, deliberately decoupled:

1. **Repository release version** (`VERSION`, git tags `vX.Y.Z`, SemVer) — governs the whole standard as a citable snapshot.
   - **MAJOR**: breaking schema change, ID scheme change, orthography rule reversal, removal of a `status: deprecated` field/entry.
   - **MINOR**: new domain/dataset/spec doc, additive optional schema fields.
   - **PATCH**: corrections, typo fixes, new entries within existing schemas.
2. **`schema_version`** (per schema file) — lets tooling know how to parse a record regardless of which repo release it shipped in; only bumped on a breaking field change to that specific schema.
3. **Spec `status`** (per `spec/*.md` doc) — `Draft → Review → Stable → Deprecated`, independent of repo version; a Draft spec can exist inside a `Stable` overall release without implying the whole release is unstable.

**Deprecation policy**: nothing is deleted outright. Mark `review_status: deprecated` (data) or `status: Deprecated` (spec), keep for at least 2 minor versions with a migration note in `CHANGELOG.md`, then remove only in the next MAJOR release.

---

## 15. Licensing

Two licenses, matching the two kinds of content:

| License | Applies to | Rationale |
|---|---|---|
| **MIT** | `tools/`, `schemas/`, CI configs, any code | Maximizes ecosystem adoption; no reason to restrict tooling reuse. |
| **CC BY 4.0** | Everything in `spec/`, `data/`, `ai/`, `benchmarks/` | Permits commercial use *including AI training* with only an attribution requirement. Deliberately **not** CC BY-SA — share-alike is viral and creates real legal friction for commercial model vendors wanting to fine-tune on SLS, which directly works against the "canonical standard every AI model uses" goal. Deliberately **not** CC0 — attribution preserves contributor credit and provenance, which is part of what makes the standard *citable* and *authoritative*. |

Two open governance questions to flag explicitly (Council decision, not a default to silently pick):
- Whether held-out benchmark test sets should be gated/unpublished (GLUE-style) to protect leaderboard integrity, vs. fully open under CC BY 4.0 like everything else.
- **Contribution provenance**: recommend **DCO (sign-off commits)** over a formal CLA — lighter weight, same effect (used by Linux kernel, Kubernetes), and appropriate here since contributors are asserting they have the right to contribute specific language data.

---

## 16. Contribution Workflow

1. **Discussion first** for anything structural: new spec doc, new terminology domain, schema change. Opens as a GitHub Discussion or issue using `spec-change-proposal.md`.
2. **Direct PR** for additive content within existing schemas (a handful of dictionary/terminology entries, example sentences) — must pass CI (§17) and get one linguist + one technical review.
3. **Review gate**: at minimum one native-speaker/linguist reviewer (content correctness) and one technical reviewer (schema conformance) before merge; Domain Editor sign-off required for `status: standard` terminology promotions.
4. **Sign-off**: DCO (`Signed-off-by:` trailer) required on every commit.
5. **Attribution**: `CONTRIBUTORS.md` generated from git history plus aggregated `contributor` fields in records — credit is structural, not an afterthought.
6. **Templates**: issue templates for "propose new terminology" and "propose spec change" keep the *initial* framing consistent (domain, coinage rationale, sources) so review isn't spent re-deriving context.

---

## 17. CI/CD Pipeline

GitHub Actions, staged so cheap checks fail fast:

1. **Lint** — markdownlint on `spec/`/`docs/`; front-matter required-field check.
2. **Schema validation** — every touched `.jsonl`/`.json` validated line-by-line against its `schemas/*.schema.json`.
3. **Cross-reference integrity** — duplicate `sls_id` detection; dangling references (e.g., a translation pair citing a `domain` not in the controlled `_domains.json` vocabulary).
4. **Content spellcheck** — new Somali text checked against `data/lexicon`; **warning, not hard failure**, since coining new terminology is expected and desired.
5. **License/metadata completeness** — every new record has the required `metadata-common` fields.
6. **Benchmark structural check** — validates format of any changed `benchmarks/` file without running a full model eval in CI.
7. **Build** — compiles per-domain fragments into unified bundles, regenerates `ai/rag-knowledge/chunks/`, regenerates stats (counts per domain, growth over time).
8. **Docs deploy** — builds and publishes the documentation site on merge to `main`.
9. **Release** — on `vX.Y.Z` tag push: package `releases/vX.Y.Z/*`, generate changelog from conventional commits, publish a GitHub Release, optionally mirror to a Hugging Face Hub dataset repo for direct `datasets.load_dataset()` access.

---

## 18. Validation & Automation Tools

Described here for the implementation phase (Phase 0, §20) — not built yet:

- **Schema validator** — walks every data file, validates against its schema, reports line-precise errors.
- **Cross-reference validator** — builds an in-memory ID graph, flags dangling/duplicate `sls_id`s.
- **Orthography compliance checker** — flags characters/sequences outside the `spec/orthography/0001-alphabet.md` inventory.
- **Dataset compiler** — merges per-domain/per-letter fragments into `releases/vX.Y.Z/` bundles + a `manifest.json` (checksums, counts, schema versions).
- **Stats generator** — coverage per domain, entries per review_status, growth-over-time — feeds a badge/dashboard on the docs site.
- **HF exporter** — packages `releases/` output into a Hugging Face `datasets`-compatible layout.
- **Docs generator** — renders `spec/*.md` front-matter + prose into the static docs site (MkDocs Material or Docusaurus — a documentation-site choice, deferred to implementation).

---

## 19. Developer / API Surface

- **Static JSON "API"**: since everything compiles to versioned JSON/JSONL in `releases/`, hosting that directory on GitHub Pages/CDN gives any developer URL-addressable, cacheable access with zero backend — e.g. `https://…/releases/v1.0.0/sls-terminology-v1.0.0.jsonl`.
- **Language bindings** (future): thin `pip install sls-somali` / `npm install @sls/somali` packages that just wrap fetching + parsing the released bundles — deliberately not core to v1.0, since the raw files are already universally consumable.
- **RAG integration guide** (`examples/rag-integration-guide.md`): documents how to embed `ai/rag-knowledge/chunks/` directly into any vector store, citing `sls_id` back to source for attribution.

---

## 20. Roadmap & Phases

| Phase | Target | Scope |
|---|---|---|
| **Phase 0 — Foundation** (v0.1) | ~4–6 weeks | `GOVERNANCE.md`, licensing, orthography + core grammar spec skeleton, `schemas/` for lexicon + terminology, CI validate pipeline, seed lexicon (~500–1000 words), 2 pilot terminology domains (AI, Computer Science) to prove the model end-to-end. |
| **Phase 1 — Expansion** (v0.5) | ~2–3 months | All 20 terminology domains populated (crowdsourced + partnerships with Somali universities/language bodies), grammar spec complete, translation guidelines + style guides drafted, `tools/` (validators, compiler, stats) implemented. |
| **Phase 2 — v1.0 Standard** | ~6 months | Spec fully `Stable`, benchmark suite v1 (grammar/spelling/translation/RC) released, docs site live, dataset published to Hugging Face, at least one external open-source Somali NLP project adopts SLS as proof point. |
| **Phase 3 — Maturity** (v1.x) | Ongoing | Fine-tuning/instruction datasets grown via partnerships, RAG knowledge base expanded, reasoning/writing-quality benchmarks calibrated against human eval, public leaderboard for model comparison on SLS benchmarks. |
| **Phase 4 — v2.0** | 1–2 years | Dialect coverage as extension namespaces, speech resources (IPA + audio alignment), historical scripts (Osmanya, Wadaad) as archival extension, morphological analyzer/tokenizer spec, formal academic/government partnerships, sustainability model (foundation/grants). |

### 20.1 Resources evidence library (pre-spec)

The §20 table above describes **implementation milestones** toward a published
standard (`spec/`, `data/`, releases). A separate **resources evidence library**
in `resources/` must be curated and accepted first. Documented in
[`docs/RESOURCES.md`](RESOURCES.md).

That prerequisite baseline was completed on 2026-08-23 and remains accepted
with documented limitations. Later source, metadata, and rights improvements
do not reopen the milestone unless they materially invalidate its evidence.

| Stage | Goal |
| --- | --- |
| Curation | Eight collections with `00-sources.md`, charters, and numbered content |
| Baseline | Scope, traceability, and limitations documented per collection |
| Spec drafting | Baseline complete; the SLS-0002 through SLS-0005 proposal package is tracked under Milestone 2 |

Orthography and later normative drafting (`spec/orthography/`, `data/lexicon/`,
etc.) build on the baseline in [`docs/RESOURCES.md`](RESOURCES.md).
`resources/` content is **evidence input**, not the standard.

---

## 21. Future Expansion

- **Dialect coverage** — Maay, Benadiri, Northern Somali as tagged extensions (`so-x-maay`) layered on top of, never replacing, standard orthography.
- **Speech** — IPA transcription layer, phoneme inventory spec, alignment with audio corpora (Common Voice-style crowdsourcing) for TTS/STT training.
- **Historical/alternate scripts** — Osmanya and Wadaad's writing as an archival `spec/orthography/` extension, useful for OCR of historical documents.
- **Code-switching guidance** — Somali-English mixed text is common in diaspora communication; a spec doc on normalization/handling would materially help real-world NLP.
- **Children's/education corpus** — graded-reader-level content, distinct register from `spec/style/`.
- **Tokenizer/BPE vocabulary recommendations** — a documented, reproducible recipe for building Somali-aware subword vocabularies, addressing a real current gap in multilingual tokenizers.
- **Compliance badge** — a self-certification test suite so any AI vendor can claim "SLS v1.0 compliant" the same way software claims POSIX or WCAG compliance.

---

## Summary of Key Design Decisions

- **Format**: JSONL for all datasets, Markdown+YAML front-matter for spec/prose, no CSV/XML.
- **IDs**: `sls:<type>:<sequence>`, permanent, append-only, never renumbered.
- **Licensing**: MIT for code, CC BY 4.0 for content — chosen specifically to make SLS friction-free for commercial AI vendors to adopt.
- **Governance**: lightweight Council + Domain Editors, RFC-style Draft→Stable lifecycle for spec docs.
- **Versioning**: three decoupled axes (repo SemVer, per-schema `schema_version`, per-doc spec `status`).
- **Structure**: hard separation between hand-authored source (`spec/`, `data/`, `ai/`, `benchmarks/`) and generated artifacts (`releases/`, `ai/rag-knowledge/chunks/`) — nothing generated is ever hand-edited.

---
---

# Part II — The SLS Standards Framework

**Status:** Draft v0.1 — extension layer, added 2026-07-02
**Relationship to Part I:** Nothing above this line changes. Part I already contains the raw material this framework governs — `spec/` holds normative prose, §5 already uses category-numbered documents with a `status` lifecycle, §2 already defines a Council and Domain Editors, §14 already defines versioning axes. The Standards Framework does not replace any of that; it **formalizes it into a numbered, citable, IETF/W3C/ISO-style standards catalog** that sits above every directory in §3 and declares which rules they must satisfy. Where a mechanism already exists (Council, review gates, CI, SemVer), this framework reuses it rather than inventing a parallel one.

---

## 22. Standards Framework Overview

Today, SLS is an excellent *repository*: grammar, data, terminology, AI resources, and benchmarks all live in well-organized directories. What it lacks is the thing that makes Unicode, W3C, IETF, and ISO outputs *citable in a legal or engineering sense*: a **numbered catalog of discrete, independently versioned, independently governed standards**, each with a defined scope, a lifecycle stage, and a compliance bar.

The Standards Framework introduces exactly that, as a layer that:

1. Assigns a permanent global identifier (`SLS-XXXX`) to every normative rule-set the project publishes — whether its content lives in `spec/`, `schemas/`, `ai/`, or `benchmarks/`.
2. Wraps every such rule-set in one formal template (§24), with a maturity stage (§25) that is stricter and more granular than the simple `Draft/Review/Stable/Deprecated` used informally in §5.
3. Makes the dependency relationships between rule-sets explicit and machine-checkable (§28), so nothing is declared `Stable` on top of something still shifting.
4. Gives external adopters (an AI lab, a university, a government agency) a precise, falsifiable thing to claim: not "we support Somali," but **"SLS-0003 v1.2.0 Compliant."**

Concretely: **a `SLS-XXXX` standard is a governance and identity wrapper around content that already has, or will have, a home in the Part I repository tree.** No content moves. No directory in §3 is renamed. What changes is that the important documents in `spec/` (and select resources in `data/`, `ai/`, `benchmarks/`) now also carry a global standard ID, a formal version, and a place in the registry described in §30.

---

## 23. Standard Numbering System

Every standard has a permanent ID of the form **`SLS-NNNN`** (four digits, zero-padded, room to grow to `SLS-9999`). IDs are assigned sequentially within reserved category blocks and, like `sls_id`s in Part I (§8), are **never reused or renumbered** — a retired standard's number stays retired forever, marked `Archived` (§25), not recycled.

| Block | Category | Notes |
|---|---|---|
| `SLS-0000` | **Meta** | The standards process itself — see §31. |
| `SLS-0001`–`0099` | **Foundation** | Alphabet, orthography, grammar, punctuation, capitalization; room for phonology, numerals, date/time formatting later. |
| `SLS-0100`–`0199` | **Lexicon** | Dictionary, definitions, synonyms/antonyms, loanwords, morphology, frequency. |
| `SLS-0200`–`0299` | **Terminology** | One ID per domain (§9's 20 domains occupy `0200`–`0219`); 80 slots free for future domains. |
| `SLS-0300`–`0399` | **Translation** | EN↔SO guidelines, technical translation, idioms, consistency. |
| `SLS-0400`–`0499` | **Writing & Style** | The 8 registers from §11. |
| `SLS-0500`–`0599` | **AI & Computational** | Prompts, assistants, correction, evaluation, benchmarks-as-resource, RAG, fine-tuning. |
| `SLS-0600`–`0699` | *Reserved* — Speech & Phonology | Opens when audio/IPA work (§21) begins. |
| `SLS-0700`–`0799` | *Reserved* — Dialects & Regional Variation | Opens when dialect tagging (§21) begins. |
| `SLS-0800`–`0899` | *Reserved* — Historical & Alternate Scripts | Osmanya, Wadaad's writing. |
| `SLS-0900`–`0999` | *Reserved* — Compliance, Certification & Testing | Meta-standards about how conformance itself is measured. |
| `SLS-1000`+ | *Open allocation* | New macro-categories not yet anticipated; opening one requires a Council-approved `SLS-0000`-governed process change, not a unilateral PR. |

This mirrors the block-reservation pattern §5 already uses for `spec/` file numbers (`00xx` orthography, `01xx` grammar, …) but promotes it to the global, cross-directory scale the project needs once "grammar" and "AI resources" and "benchmarks" all need to cite each other by a shared ID space.

---

## 24. The Formal Standard Template

The lightweight template in §5 remains exactly as-is for ordinary `spec/` notes that don't need full standards-track rigor (e.g., a single common-mistakes writeup). A document that is assigned an `SLS-XXXX` ID, however, uses this stricter, RFC/ISO-style template:

```markdown
---
sls_id: SLS-0003
title: Somali Grammar Standard
version: 1.2.0            # independent SemVer — see §27
status: Stable             # see §25
category: foundation
owner: language-council
reviewers: [name-1, name-2]
dependencies: [SLS-0001, SLS-0002]
implements:                 # physical files this standard governs
  - spec/grammar/0010-parts-of-speech.md
  - spec/grammar/0011-noun-morphology-gender-plurals.md
  - spec/grammar/0012-verb-system-tense-aspect-mood.md
publication_date: 2026-09-01
supersedes: null
superseded_by: null
revision_history:
  - version: 1.0.0
    date: 2026-09-01
    change: Initial ratification
  - version: 1.1.0
    date: 2027-02-10
    change: Added negation edge cases (non-breaking)
---

## Abstract
One paragraph: what this standard governs and why it exists.

## Purpose
The problem this standard solves and who relies on it.

## Scope
What is explicitly in scope and — just as important — explicitly out of scope.

## Definitions
Every term used normatively below, defined once, unambiguously.

## Normative Requirements
Numbered rules using RFC 2119 keywords:
- **MUST** / **MUST NOT** — required for any compliance claim (§29).
- **SHOULD** / **SHOULD NOT** — strong recommendation; deviation must be justified.
- **MAY** — genuinely optional.

## Recommendations
Non-binding best practice that didn't rise to a SHOULD.

## Examples
Canonical positive and negative examples, table form where useful.

## Edge Cases
Known hard cases and how the Normative Requirements resolve them.

## Compliance Requirements
The checklist an implementer or auditor uses to score a compliance claim (§29) — each item traceable to a numbered Normative Requirement above.

## References
Other `SLS-XXXX` standards, external standards (ISO, BCP 47, Unicode), academic sources.

## Revision History
Human-readable mirror of the front-matter `revision_history`.
```

The addition over a typical RFC template is the **Compliance Requirements** section: it exists specifically so §29's compliance levels are checklist-driven rather than a subjective claim.

---

## 25. Standard Lifecycle

A standard moves through **seven stages**, more granular than §5's four (`Draft/Review/Stable/Deprecated`) because a numbered, citable standard needs an explicit "we tried it in practice before locking it in" step that an internal spec note doesn't:

```
Draft → Proposed → Review → Candidate → Stable → Deprecated → Archived
```

| Stage | Meaning | Exit condition |
|---|---|---|
| **Draft** | Exploratory; author (with a Domain Editor sponsor) may change scope freely. Not binding on any content. | Minimum template sections filled in. |
| **Proposed** | Opened as a formal PR/Discussion with rationale. Scope is frozen for the comment period. | Minimum 14-day public comment period elapses with no unresolved objection. |
| **Review** | Assigned reviewers actively evaluate; only refinements allowed, no new scope. | ≥1 native-speaker/linguist reviewer **and** ≥1 technical reviewer approve — the same two-reviewer gate §16 already requires for content PRs. |
| **Candidate** | Reviewers approved; the standard must now prove itself against real content before ratification. | **Soak period**: at least one real artifact (a schema, a validator, a benchmark, a dataset field) actually implements/enforces it for one full release cycle — analogous to W3C's implementation-experience requirement before Recommendation. |
| **Stable** | Ratified and binding. New content **MUST** conform; non-conforming existing content is flagged for migration, not silently grandfathered. | Council majority vote, **and** every standard listed in its `dependencies` is itself already `Stable` (or the dependency is explicitly waived with documented Council rationale — mirrors IETF's rule that a normative reference can't be less stable than the document citing it). |
| **Deprecated** | Superseding standard has reached `Stable`, or the Council retires the requirement outright. Still valid for content created before deprecation. | A named successor (`superseded_by`) or an explicit retirement rationale is required — deprecation is never silent. |
| **Archived** | Fully retired; kept for historical/audit record only, excluded from active compliance checks. | At least 2 MINOR repository releases have passed since deprecation, per the existing §14 deprecation policy. |

---

## 26. Governance & Publication Workflow

No new governance body is created — this reuses the Language Council and Domain Editors from §2, with stage-specific authority:

| Transition | Approver |
|---|---|
| — → Draft | Any contributor, with a Domain Editor as sponsor |
| Draft → Proposed | Domain Editor accepts it for formal comment |
| Proposed → Review | Domain Editor, once the comment period closes cleanly |
| Review → Candidate | The two required reviewers (§16 gate) |
| Candidate → Stable | Language Council, by majority vote |
| Stable → Deprecated | Language Council only |
| Deprecated → Archived | Maintainers (mechanical, date-triggered — no vote needed) |

**Publication workflow** plugs directly into the existing release machinery rather than creating a separate one:

1. A standard reaching `Stable` is merged to `main` like any other change, subject to the existing CI pipeline (§17) **plus one new stage**: *Standards registry validation* — checks that `standards/registry.json` (§30), each standard's `meta/SLS-XXXX.json`, and the `implements`-referenced files' own front-matter all agree on `status` and `version`. A mismatch blocks merge.
2. On the next tagged release (`vX.Y.Z`), the build step already described in §17/§19 additionally emits `releases/vX.Y.Z/standards-manifest.json` — a signed snapshot of every standard's `id`, `version`, and `status` at that release. This manifest is what an external adopter cites: *"Compliant with SLS as published in `standards-manifest.json` for release v1.2.0."*
3. `CHANGELOG.md` gains one line per standard transition (new Stable standard, deprecation, etc.), same convention already used for dataset/spec changes.

---

## 27. Standard Versioning (Fourth Axis)

§14 defined three versioning axes (repo release, `schema_version`, spec `status`). The Standards Framework adds a **fourth, independent axis: per-standard SemVer**, carried in each standard's `version` front-matter field:

- **MAJOR** — a Normative Requirement (MUST/MUST NOT) changes or is removed. Breaking for any existing compliance claim; implementers must re-certify.
- **MINOR** — a new Recommendation, example, or non-binding clarification is added; existing compliance claims remain valid.
- **PATCH** — editorial fixes (typos, wording clarity) with no normative effect.

A standard's version is independent of the repository's `VERSION` — `SLS-0003` can move from `1.2.0` to `2.0.0` inside a repo `MINOR` release, provided the repo-level bump rules in §14 are still respected for anything else that changed. The `standards-manifest.json` (§26) is what lets consumers pin to "SLS-0003 v1.2.0" without caring which repo tag it shipped in.

---

## 28. Dependency Model

Every standard's front-matter `dependencies` array must form a **DAG** — CI (§17 extension in §26) rejects cycles. Foundation standards are the root; nearly everything else depends on them transitively:

```
SLS-0001 Alphabet
   └─▶ SLS-0002 Orthography
          ├─▶ SLS-0004 Punctuation
          ├─▶ SLS-0005 Capitalization
          └─▶ SLS-0003 Grammar
                 ├─▶ SLS-0100–0105 Lexicon standards
                 ├─▶ SLS-0200–0219 Terminology standards ──▶ each also depends on relevant Lexicon IDs
                 ├─▶ SLS-0300–0304 Translation standards ──▶ depend on Terminology for technical translation
                 ├─▶ SLS-0400–0407 Writing/Style standards ──▶ depend on Grammar + relevant Terminology
                 └─▶ SLS-0500–0507 AI standards ──▶ depend on whichever of the above they operationalize
                        (e.g. SLS-0502 Grammar Correction depends on SLS-0003+0004+0005;
                         SLS-0504 Benchmark depends on every standard it evaluates;
                         SLS-0505 RAG Knowledge depends on every content standard it indexes)
```

**Hard rule** (enforced at `Candidate → Stable`, §25): a standard cannot become `Stable` while any of its declared dependencies are below `Stable`, unless the Council records an explicit waiver and rationale. This is what prevents, for example, an AI Assistant Standard from locking in behavior built on a Grammar standard that's still in `Review` and could still change underneath it.

---

## 29. Compliance Model

Compliance is claimed **per standard, per version**, never as a blanket "SLS-compliant." Four levels, each with a precise bar:

| Level | Bar |
|---|---|
| **Fully Compliant** | All MUST **and** SHOULD requirements met; passes 100% of the standard's associated automated test/benchmark (where one exists in `benchmarks/` or `tools/validators/`) or a documented manual audit checklist where automation isn't possible. |
| **Compliant** | All MUST requirements met. SHOULD/recommendations may be skipped. |
| **Partially Compliant** | A defined, disclosed subset of MUST requirements met. A partial claim **MUST** enumerate exactly which Compliance Requirements (from the §24 template) are and aren't satisfied — "partial" without a stated gap list is not a valid claim. |
| **Non-Compliant** | Minimum MUST bar not met. |

**Self-declaration format** — any project can publish a machine-readable claim:

```json
{
  "sls_id": "SLS-0003",
  "standard_version": "1.2.0",
  "level": "compliant",
  "unmet_requirements": [],
  "verification": "self-declared",
  "date": "2026-11-01"
}
```

`verification` starts as `self-declared`; a future `sls-validated` value (Council-issued, after submitting to an audit — the "compliance badge" already flagged as future work in §21) is explicitly deferred rather than designed now, consistent with not over-building ahead of need. Where a standard has a corresponding entry in `benchmarks/`, `Fully Compliant`/`Compliant` can be scored mechanically by running that suite — compliance and evaluation share the same infrastructure rather than duplicating it.

---

## 30. Repository Integration

**No directory in §3 is renamed, moved, or restructured.** The framework adds one new top-level directory that acts purely as a **registry/governance layer**, plus two small additive front-matter fields on existing files:

```
standards/
├── REGISTRY.md                # human-readable master table: ID | Title | Category | Status | Version | Owner
├── registry.json              # machine-readable mirror — source of truth for CI and tooling
├── TEMPLATE.md                # the formal §24 template
└── meta/
    ├── SLS-0000.json
    ├── SLS-0001.json          # { id, title, version, status, owner, reviewers, dependencies,
    ├── SLS-0002.json          #   publication_date, revision_history, implements: [...] }
    └── ...
```

- `standards/meta/SLS-XXXX.json` holds governance metadata; it does **not** duplicate normative prose. The prose lives exactly where §3 already puts it (`spec/grammar/0012-...md`, `benchmarks/SCORING.md`, etc.).
- Files that already exist under `spec/` gain two optional, additive front-matter fields — `sls_id` and `standard_version` — so a document like `spec/orthography/0001-alphabet.md` is simultaneously "local spec doc `0001`" (§5's existing convention) and "global standard `SLS-0001`" (this framework). Nothing about its filename, location, or existing front-matter fields changes.
- Standards that don't yet have a natural home (e.g. `SLS-0100` Dictionary Standard, `SLS-0500` AI Prompt Standard) are added as new files inside `spec/`, following the exact extensibility §5 already designed for ("new categories… can be added later without renumbering") — e.g. `spec/lexicon/0100-dictionary-standard.md`, `spec/ai/0500-ai-prompt-standard.md`. This is new content in the pattern that already exists, not a redesign of it.
- `standards/registry.json` is validated against `data/terminology/_domains.json` and every `implements` path for existence — one more check appended to the §17 CI pipeline (§26), not a new pipeline.

---

## 31. Initial Standards Registry (Launch Set)

53 standards at launch — enough to cover every category named in the original architecture, with headroom (§23) for hundreds more over the next decade:

| ID | Title | Depends on |
|---|---|---|
| SLS-0000 | SLS Standards Process Standard (this framework, self-describing — cf. IETF's own BCP 9 / RFC 2026) | — |
| SLS-0001 | Somali Alphabet Standard | — |
| SLS-0002 | Somali Orthography Standard | 0001 |
| SLS-0003 | Somali Grammar Standard | 0001, 0002 |
| SLS-0004 | Somali Punctuation Standard | 0002 |
| SLS-0005 | Somali Capitalization Standard | 0002 |
| SLS-0100 | Dictionary Standard | 0001–0003 |
| SLS-0101 | Definition Standard | 0100 |
| SLS-0102 | Synonym & Antonym Standard | 0100 |
| SLS-0103 | Loanword Standard | 0100 |
| SLS-0104 | Morphology Standard | 0003, 0100 |
| SLS-0105 | Word Frequency Standard | 0100 |
| SLS-0200–0219 | Terminology Standards (AI, ML, Data Science, Cybersecurity, CS, Software Eng., Cloud, Networking, Blockchain, Medicine, Law, Government, Finance, Education, Agriculture, Engineering, Math, Physics, Chemistry, Biology — one ID each, in this order) | 0001–0003, 0100 |
| SLS-0300 | English→Somali Translation Standard | 0001–0003 |
| SLS-0301 | Somali→English Translation Standard | 0001–0003 |
| SLS-0302 | Technical Translation Standard | 0300, 0301, relevant 0200–0219 |
| SLS-0303 | Idiom & Expression Translation Standard | 0300, 0301 |
| SLS-0304 | Translation Consistency Standard | 0300–0303 |
| SLS-0400 | Formal Writing Standard | 0003 |
| SLS-0401 | Academic Writing Standard | 0400 |
| SLS-0402 | Journalism Standard | 0400 |
| SLS-0403 | Government Writing Standard | 0400, 0211 |
| SLS-0404 | Technical Writing Standard | 0400, relevant 0200–0219 |
| SLS-0405 | Marketing Writing Standard | 0400 |
| SLS-0406 | Social Media Writing Standard | 0003 |
| SLS-0407 | Conversational Somali Standard | 0003 |
| SLS-0500 | AI Prompt Standard | 0003, 0400–0407 |
| SLS-0501 | AI Assistant Standard | 0500, 0003 |
| SLS-0502 | Grammar Correction Standard | 0003, 0004, 0005 |
| SLS-0503 | Translation Evaluation Standard | 0300–0304 |
| SLS-0504 | Benchmark Standard | every standard it evaluates |
| SLS-0505 | RAG Knowledge Standard | every content standard it indexes |
| SLS-0506 | Fine-Tuning Dataset Standard | 0501, 0507 |
| SLS-0507 | Instruction Dataset Standard | 0003, 0400–0407 |

This maps directly onto the categories already named in §9 and §11 — no new content categories were invented, only formalized.

---

## 32. Long-Term Scaling Vision

The block-based numbering (§23) is what lets this scale to "hundreds of standards over 10–20 years" without ever renumbering anything: each launch category uses well under half its reserved block (e.g. Foundation uses 5 of 99 slots, Terminology 20 of 100), and four entire blocks (`0600`–`0999`) are reserved but unopened for speech, dialects, historical scripts, and formal compliance/certification standards as those workstreams mature per the §20 roadmap. Opening a genuinely new macro-category beyond `1000` is itself gated through `SLS-0000` (the meta-standard), so the numbering system's own evolution is subject to the same Council governance as everything it numbers — it cannot drift by accretion.

The practical payoff of formalizing this now, while the launch set is only 53 standards: it gives SLS the same shape of authority Unicode's UAX series, W3C's Recommendations, and IETF's RFCs have — a versioned, dependency-checked, citable catalog — so that when SLS is presented to a university, a government body, or an AI lab as a partnership, the pitch is not "look at our repository" but **"implement SLS-0003 v1.2.0, and here is exactly how we'll tell you if you did."**
