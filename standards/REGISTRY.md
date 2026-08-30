# SLS Standards Registry

The master catalog of SLS numbered standards. This table is the **human-readable
mirror** of [`registry.json`](registry.json), which is the machine-readable
**source of truth** for CI and tooling. When the two disagree, `registry.json`
wins and the mismatch is a bug (see SLS-0000, requirement R13).

- **Identifiers** (`SLS-NNNN`) are permanent and never reused or renumbered.
- **Governance metadata** for each standard lives in [`meta/`](meta/).
- **Normative prose** lives in the files listed in each standard's `implements`
  array — never duplicated here.
- **Lifecycle** (`Draft → Proposed → Review → Candidate → Stable → Deprecated →
  Archived`) and the process governing this registry are defined in
  [`SLS-0000-standards-process.md`](SLS-0000-standards-process.md).

`planned` marks a reserved identifier from
[`docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md) §31 that has not yet been
drafted.

## Meta

| ID | Title | Status | Version | Owner |
|---|---|---|---|---|
| SLS-0000 | SLS Standards Process Standard | Review | 0.2.0 | maintainers |

## Foundation (`SLS-0001`–`0099`)

| ID | Title | Status | Version | Owner |
|---|---|---|---|---|
| SLS-0001 | Somali Alphabet Standard | Review | 0.1.1 | maintainers |
| SLS-0002 | Somali Orthography Standard | Proposed | 0.1.1 | maintainers |
| SLS-0003 | Somali Grammar Standard | Proposed | 0.2.0 | maintainers |
| SLS-0004 | Somali Punctuation Standard | Proposed | 0.2.0 | maintainers |
| SLS-0005 | Somali Capitalization Standard | Proposed | 0.1.1 | maintainers |

## Lexicon (`SLS-0100`–`0199`)

| ID | Title | Status | Version | Owner |
|---|---|---|---|---|
| SLS-0100 | Dictionary Standard | planned | — | maintainers |
| SLS-0101 | Definition Standard | planned | — | maintainers |
| SLS-0102 | Synonym & Antonym Standard | planned | — | maintainers |
| SLS-0103 | Loanword Standard | planned | — | maintainers |
| SLS-0104 | Morphology Standard | planned | — | maintainers |
| SLS-0105 | Word Frequency Standard | planned | — | maintainers |

## Terminology (`SLS-0200`–`0299`)

| ID | Title | Status | Version | Owner |
|---|---|---|---|---|
| SLS-0200 | Artificial Intelligence Terminology Standard | planned | — | maintainers |
| SLS-0201 | Machine Learning Terminology Standard | planned | — | maintainers |
| SLS-0202 | Data Science Terminology Standard | planned | — | maintainers |
| SLS-0203 | Cybersecurity Terminology Standard | planned | — | maintainers |
| SLS-0204 | Computer Science Terminology Standard | planned | — | maintainers |
| SLS-0205 | Software Engineering Terminology Standard | planned | — | maintainers |
| SLS-0206 | Cloud Computing Terminology Standard | planned | — | maintainers |
| SLS-0207 | Networking Terminology Standard | planned | — | maintainers |
| SLS-0208 | Blockchain Terminology Standard | planned | — | maintainers |
| SLS-0209 | Medicine Terminology Standard | planned | — | maintainers |
| SLS-0210 | Law Terminology Standard | planned | — | maintainers |
| SLS-0211 | Government Terminology Standard | planned | — | maintainers |
| SLS-0212 | Finance Terminology Standard | planned | — | maintainers |
| SLS-0213 | Education Terminology Standard | planned | — | maintainers |
| SLS-0214 | Agriculture Terminology Standard | planned | — | maintainers |
| SLS-0215 | Engineering Terminology Standard | planned | — | maintainers |
| SLS-0216 | Mathematics Terminology Standard | planned | — | maintainers |
| SLS-0217 | Physics Terminology Standard | planned | — | maintainers |
| SLS-0218 | Chemistry Terminology Standard | planned | — | maintainers |
| SLS-0219 | Biology Terminology Standard | planned | — | maintainers |

## Translation (`SLS-0300`–`0399`)

| ID | Title | Status | Version | Owner |
|---|---|---|---|---|
| SLS-0300 | English→Somali Translation Standard | planned | — | maintainers |
| SLS-0301 | Somali→English Translation Standard | planned | — | maintainers |
| SLS-0302 | Technical Translation Standard | planned | — | maintainers |
| SLS-0303 | Idiom & Expression Translation Standard | planned | — | maintainers |
| SLS-0304 | Translation Consistency Standard | planned | — | maintainers |

## Writing & Style (`SLS-0400`–`0499`)

| ID | Title | Status | Version | Owner |
|---|---|---|---|---|
| SLS-0400 | Formal Writing Standard | planned | — | maintainers |
| SLS-0401 | Academic Writing Standard | planned | — | maintainers |
| SLS-0402 | Journalism Standard | planned | — | maintainers |
| SLS-0403 | Government Writing Standard | planned | — | maintainers |
| SLS-0404 | Technical Writing Standard | planned | — | maintainers |
| SLS-0405 | Marketing Writing Standard | planned | — | maintainers |
| SLS-0406 | Social Media Writing Standard | planned | — | maintainers |
| SLS-0407 | Conversational Somali Standard | planned | — | maintainers |

## AI & Computational (`SLS-0500`–`0599`)

| ID | Title | Status | Version | Owner |
|---|---|---|---|---|
| SLS-0500 | AI Prompt Standard | planned | — | maintainers |
| SLS-0501 | AI Assistant Standard | planned | — | maintainers |
| SLS-0502 | Grammar Correction Standard | planned | — | maintainers |
| SLS-0503 | Translation Evaluation Standard | planned | — | maintainers |
| SLS-0504 | Benchmark Standard | planned | — | maintainers |
| SLS-0505 | RAG Knowledge Standard | planned | — | maintainers |
| SLS-0506 | Fine-Tuning Dataset Standard | planned | — | maintainers |
| SLS-0507 | Instruction Dataset Standard | planned | — | maintainers |

## Reserved blocks (unopened)

| Block | Category | Opens when |
|---|---|---|
| `SLS-0600`–`0699` | Speech & Phonology | audio/IPA workstream begins |
| `SLS-0700`–`0799` | Dialects & Regional Variation | dialect tagging begins |
| `SLS-0800`–`0899` | Historical & Alternate Scripts | Osmanya/Wadaad work begins |
| `SLS-0900`–`0999` | Compliance, Certification & Testing | conformance program begins |
| `SLS-1000`+ | Open allocation | Council-approved new macro-category |
