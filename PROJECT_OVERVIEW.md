# Understanding the Somali Language Standard (SLS)

A plain-language overview of what SLS is, why it exists, what it is built on, who will use it, and what the world will look like when it is complete.

---

## Table of Contents

- [Understanding the Somali Language Standard (SLS)](#understanding-the-somali-language-standard-sls)
  - [Table of Contents](#table-of-contents)
  - [1. What is SLS?](#1-what-is-sls)
  - [2. Why Somali?](#2-why-somali)
  - [3. The Problem SLS Solves](#3-the-problem-sls-solves)
  - [4. What SLS Actually Does](#4-what-sls-actually-does)
  - [5. What SLS is Built On](#5-what-sls-is-built-on)
  - [6. Comparable Projects in Other Languages](#6-comparable-projects-in-other-languages)
  - [7. Who Will Use SLS and How?](#7-who-will-use-sls-and-how)
    - [AI and NLP Researchers](#ai-and-nlp-researchers)
    - [Language Model Developers (e.g., Google, Meta, Anthropic, OpenAI)](#language-model-developers-eg-google-meta-anthropic-openai)
    - [Software Engineers and Product Teams](#software-engineers-and-product-teams)
    - [Translators and Translation Agencies](#translators-and-translation-agencies)
    - [Educators, Universities, and Publishers](#educators-universities-and-publishers)
    - [Government and Institutional Bodies](#government-and-institutional-bodies)
    - [Linguists and Language Researchers](#linguists-and-language-researchers)
    - [The Somali-Speaking Public](#the-somali-speaking-public)
  - [8. What the Ecosystem Looks Like After Completion](#8-what-the-ecosystem-looks-like-after-completion)
  - [9. What SLS is NOT](#9-what-sls-is-not)
  - [10. A Note on Governance and Trust](#10-a-note-on-governance-and-trust)

---

## 1. What is SLS?

The **Somali Language Standard (SLS)** is an open, machine-readable, and versioned standard for the Somali language. It defines — with formal, citable authority — how Somali words are spelled, how Somali grammar works, what the correct technical terminology is in modern domains, and how translation between Somali and other languages should be handled.

Think of it like an **RFC series** (such as those that define how the internet works), or a **Unicode Technical Report**, but applied to the Somali language. It is not a dictionary you look up words in. It is a specification that systems and humans implement *against*. When a software company builds a Somali spellchecker, a hospital builds a Somali patient portal, or an AI lab trains a Somali language model, they need an authoritative reference to rely on. SLS is that reference.

Every fact SLS asserts — a word, a grammar rule, a technical term — is:
- **Numbered** with a permanent, unique identifier (e.g., `sls:lex:0042`).
- **Versioned**, so adopters can precisely cite which version of a rule they are implementing.
- **Sourced**, with full provenance back to original Somali publications.
- **Validated**, through an automated CI pipeline that rejects incorrect or incomplete records.

---

## 2. Why Somali?

Somali is spoken by over **20 million people** across Somalia, Djibouti, Ethiopia, Kenya, and large diaspora communities worldwide. It is the official language of Somalia and one of the most widely spoken Cushitic languages on earth.

Despite this, Somali is one of the most **under-resourced major languages** in computing and natural language processing. The Somali writing system — using the Latin alphabet — was officially standardized only in **1972**. This means the language has a relatively short formal written tradition compared to many other major world languages.

As a result, when modern computing tools need to handle Somali — from spell-checkers to translation engines to AI assistants — they have almost no canonical, authoritative, machine-readable reference to build upon.

---

## 3. The Problem SLS Solves

Today, anyone trying to build software or AI systems that handle Somali faces a fundamental problem: **there is no single authoritative source of truth for the Somali language in machine-readable form**.

The consequences of this are significant:

- **Spell-checkers** are unreliable because there is no agreed, canonical word list.
- **AI language models** trained on Somali produce inconsistent spelling, grammatical errors, and incorrect or invented technical terminology.
- **Translation systems** produce awkward or inconsistent results because there are no normative translation guidelines.
- **Technical communication in Somali is nearly impossible** because most modern domains (AI, medicine, law, engineering) have no standardized Somali terminology.
- **Educators and publishers** use inconsistent spelling and grammar conventions because no single modern standard has the institutional authority to settle disputes.
- **Researchers** who want to evaluate AI systems on Somali have no standard benchmarks to work with.

SLS is designed to close all of these gaps in a single, coherent, open-source project.

---

## 4. What SLS Actually Does

SLS is organized as a numbered catalog of standards (similar to Python PEPs or W3C Web Standards). Each standard addresses a specific layer of the language:

| Standard Area | What it covers | Examples |
|---|---|---|
| **Orthography** | Official spelling rules, the Somali alphabet, punctuation, capitalization | SLS-0001 (Alphabet), SLS-0002 (Spelling Rules) |
| **Grammar** | Parts of speech, verb conjugation, noun morphology, sentence structure | SLS-0003 (Core Grammar) |
| **Lexicon** | A curated, schema-validated Somali dictionary with definitions, provenance, and unique IDs | `data/lexicon/` |
| **Terminology** | Standardized Somali vocabulary for 20 modern domains: AI, Medicine, Law, Engineering, Science, etc. | `data/terminology/artificial-intelligence.jsonl` |
| **Translation** | Normative guidance for translating between Somali and English (and vice versa), including technical and idiomatic translation | SLS-0300, SLS-0301 |
| **AI Resources** | Datasets, system prompts, fine-tuning corpora, and RAG-ready knowledge chunks ready for AI consumption | `ai/prompts/`, `ai/datasets/` |
| **Benchmarks** | Evaluation suites for testing AI systems on Somali grammar, spelling, and translation accuracy | `benchmarks/` |

---

## 5. What SLS is Built On

SLS follows a foundational principle: **we do not invent Somali.** We collect, preserve, study, and analyze existing authoritative Somali publications and scholarly works. These form the empirical evidence base from which all standards, datasets, and specifications are derived.

The `resources/` directory — the canonical linguistic source library of this project — contains original materials including:

- **Monolingual Somali dictionaries** (e.g., the digitized Somali dictionary spanning 31 letter-based chapter files)
- **Authoritative grammar books** (e.g., *Barashada Naxwaha Af Soomaaliga* by Puglielli & Mansuur)
- **Literature, poetry, and proverbs** (*gabay*, *maanso*, *maahmaahyo*)
- **School and university textbooks**
- **Academic linguistic studies** on Somali morphology, phonology, and syntax
- **Domain-specific references** in fields such as medicine, law, and science
- **Historical texts** and early publications from the post-1972 standardization period

Every standard SLS publishes is traceable back to this evidence base. No rule, word, or grammatical claim is made without a citation to an authoritative source.

---

## 6. Comparable Projects in Other Languages

SLS draws design inspiration from well-established language infrastructure projects in other languages. These are the closest equivalents:

| Project | Language | What it does |
|---|---|---|
| **Unicode CLDR** (Common Locale Data Repository) | Multi-language | Defines locale-specific data (numbers, dates, plurals) for every major language; used by virtually all software |
| **RFC Series / IETF** | N/A (protocol standard) | The model for SLS's numbered, versioned, lifecycle-governed standards process |
| **TermBank (Finland)** | Finnish | Government-backed terminology database standardizing Finnish technical vocabulary |
| **Real Academia Española** | Spanish | Normative authority for the Spanish language, defining spelling, grammar, and vocabulary standards |
| **Académie française** | French | Similar to Real Academia, the official institution governing the French language |
| **KNAB (Latvian)** | Latvian | Official terminology authority; standardizes Latvian technical vocabulary to protect against English dominance |
| **W3C Internationalization (i18n)** | Multi-language | Standards for representing languages in web technologies |
| **Masader (Arabic NLP)** | Arabic | Open repository of Arabic NLP datasets, somewhat analogous to SLS's `data/` layer |
| **IndicNLP / AI4Bharat** | Indic languages | Open NLP resources, datasets, and benchmarks for Hindi, Tamil, Bengali, etc. |
| **AfricanNLP / Masakhane** | African languages | Community-driven NLP and translation research for African languages |

SLS is the first project of this kind for Somali at this level of formalism, governance, and machine-readability. The projects above each solve parts of the problem; SLS attempts to create the full infrastructure stack for a single language.

---

## 7. Who Will Use SLS and How?

SLS is designed to serve multiple audiences, each of whom will consume it differently:

### AI and NLP Researchers
*   **How:** Directly consume `ai/datasets/` for fine-tuning and instruction training; use `benchmarks/` for evaluation; use RAG chunks from `tools/` to ground AI assistants in normative Somali knowledge.
*   **Value:** An AI model trained with SLS resources can claim conformance to a citable standard, making its Somali quality verifiable and improvable over time.

### Language Model Developers (e.g., Google, Meta, Anthropic, OpenAI)
*   **How:** Use SLS's structured lexicon, grammar specs, terminology datasets, and translation pairs as a canonical reference for Somali within their models.
*   **Value:** Instead of guessing at Somali conventions, models can implement against a normative standard — meaning users get consistent, authoritative Somali output.

### Software Engineers and Product Teams
*   **How:** Use the validated wordlist for spell-checkers; use the lexicon schema for dictionary features; use system prompts for Somali AI assistants.
*   **Value:** Building Somali language features becomes a matter of implementing against a known, versioned standard rather than reverse-engineering scattered resources.

### Translators and Translation Agencies
*   **How:** Consult `spec/translation/` for normative guidance; use `data/translation-pairs/` as reference corpora.
*   **Value:** Translation Memory tools can be seeded with SLS-standard pairs, ensuring terminological consistency across all documents in a translation project.

### Educators, Universities, and Publishers
*   **How:** Cite SLS standards for spelling and grammar decisions in textbooks, curricula, and publications.
*   **Value:** Resolves linguistic disputes with reference to an evidence-backed, publicly governed standard rather than personal opinion.

### Government and Institutional Bodies
*   **How:** Adopt SLS as the official machine-readable reference for Somali in government digital services, official publications, and regulatory documents.
*   **Value:** Enables consistent Somali-language digital infrastructure — from ID cards to court documents to public health communications.

### Linguists and Language Researchers
*   **How:** Cite SLS data and specifications in academic work; contribute new findings back through the formal proposal process.
*   **Value:** A living, versioned, and publicly auditable record of the state of standardized Somali knowledge.

### The Somali-Speaking Public
*   **How:** Indirectly, through every tool, application, and AI system that implements SLS — receiving better spell-checkers, more accurate AI assistants, and software that speaks correct Somali.
*   **Value:** A language that is served well by modern technology, with a future that belongs to its speakers rather than being defined by outsiders.

---

## 8. What the Ecosystem Looks Like After Completion

When SLS reaches v1.0 and the core standards are ratified `Stable`, the following becomes possible:

**For AI systems:** A language model can state *"This system implements SLS-0001, SLS-0002, and SLS-0003 v1.0"* — a precise, verifiable claim that the model's Somali spelling, grammar, and core vocabulary conform to the standard. Third parties can then test that claim against the SLS benchmark suites.

**For software:** A Somali spell-checker can be built by loading `data/lexicon/` and validated against `benchmarks/spelling/`. Any update to the lexicon automatically flows downstream to every tool built on it.

**For education:** Publishers can cite SLS-0002 §4.3 when explaining why a word is spelled a particular way, backed by the same evidence base that linguists consulted when drafting the rule.

**For AI training:** Fine-tuning datasets in `ai/datasets/` allow any lab or researcher to improve their model's Somali capability against a known, contamination-audited benchmark.

**For terminology:** When a Somali-language government publication needs the word for "artificial intelligence," there is a standard answer: `SLS-0200-AI-0001 — garaad gacmeed` (or whichever term is ratified), citable and permanently versioned.

**For the global NLP community:** SLS becomes a reference in academic papers, dataset cards, and model cards — establishing Somali as a language with a proper, citable infrastructure, comparable in rigor (if smaller in scope) to what Unicode and CLDR provide for writing systems generally.

---

## 9. What SLS is NOT

To avoid confusion, it is worth being explicit about what SLS is *not*:

- **Not a dictionary app or website** — SLS is the data and specifications that power such applications; it is not the application itself.
- **Not a corpus dump** — Every record in SLS has provenance, is schema-validated, and is reviewed. Volume without quality is explicitly rejected.
- **Not a prescriptive authority that invents language** — SLS standardizes what already exists in authoritative Somali sources. It does not invent vocabulary except through its governed terminology program, which requires community consensus.
- **Not a finished product** — SLS is a living standard. It will continue to grow as new domains are covered, new research emerges, and the language itself evolves.
- **Not a replacement for native speaker judgment** — All standards are authored by or reviewed by native speakers and linguists. SLS provides the infrastructure; human expertise provides the authority.

---

## 10. A Note on Governance and Trust

Language standards are only as trustworthy as the process that creates them. SLS is governed by a **Language Council** — a small body of named, accountable linguists and community representatives — modeled on how W3C working groups and Python's PEP editors operate. No standard reaches `Stable` status without Council review. Every ratified standard is permanently archived and versioned.

The standard is open-source under a **Creative Commons BY 4.0** license for all linguistic content. This means anyone — including commercial AI companies — can freely use and build upon SLS, as long as they credit the source. Fencing off Somali language infrastructure behind proprietary walls is explicitly rejected by the project's design.

SLS is maintained by **Goobo Labs** and the Somali Language Standard contributor community.
