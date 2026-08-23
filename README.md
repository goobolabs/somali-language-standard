# Somali Language Standard (SLS)

**The open, machine-readable standard for the Somali language.**

SLS defines grammar, orthography, terminology, translation guidance, style
conventions, and AI/benchmark resources for Somali — as a versioned, normative,
citable standard that any system, human or machine, can implement against.

> **Status:** The Milestone 2 proposal package is complete. SLS-0000 through
> SLS-0005 are represented at `Proposed`, including the eight-topic SLS-0003
> Grammar set, and the 145-file resources evidence baseline is complete. The
> SLS-0002 through SLS-0005 public-comment periods opened in
> [pull request #7](https://github.com/goobolabs/somali-language-standard/pull/7)
> on 2026-08-23 and cannot close before 2026-09-06. No standard is `Stable`. See the
> [Roadmap](ROADMAP.md) and [M2 readiness record](docs/standards/MILESTONE-2-READINESS.md).

---

## Vision

SLS aims to be to Somali what an RFC series or a language specification
(ECMA-262, Unicode Technical Reports) is to a technical domain: the canonical
reference that AI vendors, translation systems, NLP researchers, educators, and
government bodies point to when they need an authoritative answer about the
Somali language.

It is **not a corpus dump**. It is a standard with a compliance surface: every
fact SLS asserts — a word, a grammar rule, a terminology mapping — exists as a
structured, schema-validated record with provenance.

## Goals

- **Normative specifications** for Somali orthography, grammar, style, and
  translation, published through an RFC-style lifecycle.
- **A governed terminology program** that coins and standardizes Somali
  vocabulary for modern domains (AI, medicine, law, and 17 more) where standard
  terms often do not yet exist.
- **Machine-first data**: a curated lexicon, parallel translation pairs, and
  corpora — all JSONL, all schema-validated, all carrying provenance.
- **AI resources and benchmarks**: prompts, instruction datasets, and
  evaluation suites kept strictly separate to prevent contamination.
- **A numbered standards catalog** (`SLS-XXXX`) so adopters can make precise,
  falsifiable claims like *"SLS-0003 v1.2.0 Compliant."*

## Why SLS exists

Somali is spoken by over 20 million people, yet it remains severely
under-resourced in NLP: no authoritative machine-readable grammar, no governed
technical terminology, no standard benchmarks. Existing resources are scattered,
unversioned, and unverifiable. AI systems handling Somali today have nothing
canonical to implement against — SLS fills that gap.

## Repository structure

```text
.github/          Issue/PR templates and CI workflows
docs/             Human-facing documentation (start here)
spec/             Normative specifications — the standard itself
schemas/          JSON Schema contracts for every record type
data/
  lexicon/        The curated dictionary
  terminology/    Domain glossaries (20 launch domains)
  translation/    Parallel EN↔SO sentence and phrase pairs
  corpora/        Example sentences and rights-cleared text
ai/
  prompts/        System prompts and prompt templates
  datasets/       Instruction, fine-tuning, and correction datasets
benchmarks/       Evaluation suites (kept separate from training data)
tools/            Validators, compilers, exporters (implemented later)
examples/         Integration guides for developers
```

The full design is specified in [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)
— the single source of truth for this project.

## Quick start

1. Read [`docs/GETTING_STARTED.md`](docs/GETTING_STARTED.md) for the repository
   layout and recommended reading order.
2. Read [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) to understand the design.
3. Check [`IMPLEMENTATION_PLAN.md`](IMPLEMENTATION_PLAN.md) to see what is being
   worked on right now.
4. Read [`CONTRIBUTING.md`](CONTRIBUTING.md) before opening a pull request.

## Documentation

- [Getting Started](docs/GETTING_STARTED.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Style Guide](docs/STYLE_GUIDE.md)
- [FAQ](docs/FAQ.md)
- [Governance](GOVERNANCE.md)

## Roadmap

See [ROADMAP.md](ROADMAP.md) for milestones from repository foundation through
the first public release, and [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md)
for the phase-by-phase execution checklist.

## Contributing

Contributions are welcome — from native speakers, linguists, translators,
developers, and domain experts. Start with [CONTRIBUTING.md](CONTRIBUTING.md)
and the [Code of Conduct](CODE_OF_CONDUCT.md). Structural changes begin as
discussions; additive content within existing schemas can go straight to a
pull request.

## License

SLS uses two licenses, matching its two kinds of content:

- **Code** (tools, schemas, CI): [MIT](LICENSE-CODE)
- **Linguistic content** (spec, data, AI resources, benchmarks):
  [CC BY 4.0](LICENSE-DATA)

CC BY 4.0 permits commercial use — including AI training — with attribution,
which is deliberate: SLS is designed to be adopted, not fenced off.

When you reuse SLS linguistic content, attribute it as:

> Somali Language Standard (SLS) — © 2026 Somali Language Standard
> contributors, licensed under CC BY 4.0 —
> https://creativecommons.org/licenses/by/4.0/
