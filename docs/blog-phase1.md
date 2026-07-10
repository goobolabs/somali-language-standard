---
title: "Introducing the Somali Language Standard (SLS) – Phase 1 Complete"
date: "2026-07-10"
author: "Goobo Labs Team"
tags: ["NLP", "Somali", "Standards", "Tokenization", "Open Source"]
---

At Goobo Labs, we have spent years building the infrastructure required to bring the Somali language into the modern era of Artificial Intelligence. Through our work on [SomNLP Corpus v2](/blog/somnlp-corpus-v2), the [SoPLang programming language](/blog/soplang-launch), and our [morphological tokenizers](/blog/tokenizer-morphology), we’ve encountered one recurring, fundamental bottleneck: **inconsistency**.

In the world of NLP, inconsistency is the enemy of performance. Currently, Somali language datasets are riddled with conflicting spelling conventions, varying loanword adaptations, and erratic punctuation usage. Because there hasn't been a centralized, machine-readable source of truth for Somali orthography and grammar, every tech company, AI researcher, and linguist has had to guess—or invent their own rules.

Today, we are thrilled to announce the **Somali Language Standard (SLS)** project, and the successful completion of **Phase 1**.

## What is the Somali Language Standard (SLS)?

The SLS is designed to act as the W3C or IETF for the Somali language. It is a strictly governed, open-source framework that centralizes the rules of the language—from the basic alphabet to complex grammar and machine-learning terminology. 

Crucially, SLS is not a static PDF. It is a **machine-readable, CI/CD-validated repository**. Every standard is tracked via a JSON registry, follows a strict lifecycle (Draft → Proposed → Review → Candidate → Stable), and traces normative requirements (`MUST`, `SHOULD`) to compliance checklists. This allows developers to programmatically verify whether their datasets and tokenizers are "SLS-Compliant."

## Completing Phase 1: The Bedrock

You cannot build a dictionary or an LLM instruction dataset if your systems cannot agree on what constitutes a "letter." Phase 1 was entirely focused on building the constitutional framework and locking in the alphabet.

We have officially published our first two standards into the `Proposed` state:

### 1. SLS-0000: The Standards Process
This is the "meta-standard." Analogous to the IETF's RFC 2026, SLS-0000 defines how all future standards are numbered, structured, versioned, and governed. It establishes the Directed Acyclic Graph (DAG) dependency model, ensuring that, for example, a grammar standard cannot be ratified until the spelling standard it depends on is stable.

### 2. SLS-0001: The Somali Alphabet Standard
SLS-0001 formally defines the Somali Latin alphabet (*Far Soomaali*). It locks in the 21 consonants and 5 vowels, defines how digraphs (`dh`, `sh`, `kh`) are treated as single units for sorting (collation), and clarifies the notation for vowel length (doubling, not diacritics).

## The Big Technical Win: Solving the Glottal Stop Tokenization

If you have ever tried to train a tokenizer or an LLM on Somali text, you are intimately familiar with the glottal stop problem. 

Historically, the glottal stop has been written using the ASCII apostrophe (`'` or U+0027). The problem? Standard NLP tokenizers (like WordPiece or BPE) classify the ASCII apostrophe as punctuation (`Po`). When they encounter a word like `su'aal` (question), the tokenizer aggressively splits it into three tokens: `["su", "'", "aal"]`. This shatters the semantic integrity of the word, forcing models to learn fragments instead of whole concepts.

In SLS-0001, we have made the normative ruling to standardize the glottal stop as **U+02BC MODIFIER LETTER APOSTROPHE (`ʼ`)**. 

Because U+02BC is classified in Unicode as a letter (`Lm`), standard tokenizers will now treat `suʼaal` as a single, contiguous string. This seemingly minor typographical ruling will dramatically improve the efficiency of Somali tokenization and the downstream performance of large language models.

## Looking Ahead: Phase 2

With Phase 1 complete and currently in its public comment period, we are immediately pivoting to **Phase 2: The Orthography Standard**. This next phase will introduce:
- **SLS-0002 (Spelling Rules):** Standardizing gemination, assimilation, and vowel harmony.
- **SLS-0004 (Punctuation):** Aligning punctuation rules.
- **SLS-0005 (Capitalization):** Defining exact case-pairing behaviors for digraphs.

By building from the alphabet upwards, we are creating a mathematically sound, dependency-checked foundation for the future of the Somali language.

We invite linguists, developers, and researchers to review the proposed standards and participate in the public comment period on our [GitHub repository](#). Let's build the future of Somali NLP together.
