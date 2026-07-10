# Frequently Asked Questions

## What is SLS?

The **Somali Language Standard** is an open, versioned, machine-readable
standard for the Somali language. It specifies orthography, grammar, style,
translation guidance, and — critically — governed technical terminology, all
published as normative documents and schema-validated data that any system
(an AI model, a translation tool, a spellchecker, a curriculum) can implement
against.

Think of it the way you'd think of an RFC series, ECMA-262, or a Unicode
Technical Report — but for a natural language.

## Why another Somali project?

Existing Somali resources are valuable but scattered: dictionaries without
versioning, corpora without provenance, glossaries without governance. None of
them is *citable* in an engineering sense — there is no way to say "we
implement version X of the Somali standard" and have it mean something.

SLS is different in three ways:

1. **Provenance over volume** — every record carries contributor, source,
   review status, and license. Smaller and trustworthy beats large and
   unverifiable.
2. **Governance** — a Language Council and Domain Editors run an RFC-style
   process, so "standard" means reviewed and ratified, not just published.
3. **A compliance surface** — numbered standards (`SLS-XXXX`) with versions
   and checklists, so adopters can make precise, falsifiable claims.

## Is this a dataset?

No — it is a **standard that includes datasets**. The datasets (lexicon,
terminology, translation pairs, corpora) are evidence and implementation
material for the normative specifications in `spec/`. A corpus tells you what
people wrote; SLS tells you what the standard form is, who decided that, and
when.

That said, all SLS data is published in ML-native formats (JSONL, schema-
validated) precisely so it can be consumed like a dataset when that's what you
need.

## Is this for AI only?

No. AI systems are the most urgent consumer — Somali is badly served by
current models, and the `ai/` and `benchmarks/` layers exist for them — but
the same normative content serves human audiences: educators, translators,
journalists, government bodies, and software developers building spellcheckers
or keyboards. A grammar rule is a grammar rule; SLS just publishes it in forms
both audiences can use.

## Can anyone contribute?

Yes. Native speakers, linguists, translators, domain experts, and developers
are all needed — you do not need to be a linguist to contribute terminology
from your professional field, and you do not need to be a developer to
contribute language data.

Start with [CONTRIBUTING.md](../CONTRIBUTING.md). Small additive
contributions (new entries within existing schemas) go straight to a pull
request; structural proposals start as discussions. Every contribution is
credited — provenance is built into the record format itself.

## How are standards approved?

Through a staged, public lifecycle (defined in
[GOVERNANCE.md](../GOVERNANCE.md) and ARCHITECTURE.md §25):

```text
Draft → Proposed → Review → Candidate → Stable
```

A proposal gets a public comment period, review by at least one
native-speaker/linguist and one technical reviewer, then a "soak period" in
which something real (a schema, a validator, a dataset) must actually
implement it. Only then can the Language Council ratify it as `Stable` by
majority vote. Nothing becomes `Stable` while its dependencies are still
unstable.

## Which dialect does SLS describe?

SLS describes **standard Somali** (BCP 47: `so`), based on the official Latin
orthography. Dialects (Maay, Benadiri, and others) are planned as tagged
extensions (`so-x-maay`) layered on top of — never replacing — the standard.
SLS documents real variation rather than suppressing it: where genuine
dialectal or stylistic alternatives exist, they are recorded as alternatives.

## Can I use SLS commercially? Can I train AI models on it?

Yes. Linguistic content is licensed **CC BY 4.0**, which permits commercial
use including AI training, with attribution. Code and schemas are **MIT**.
This is a deliberate choice — the goal is for SLS to become the reference
every Somali-capable system uses, and licensing friction would defeat that.

## Where do I ask something not covered here?

Open an [issue](https://github.com/goobolabs/somali-language-standard/issues)
— and if the answer turns out to be generally useful, we'll add it here.
