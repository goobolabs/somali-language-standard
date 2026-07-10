# Specification Index

This is the index of SLS normative specification documents. Every document in
`spec/` is numbered within a reserved category block (see
[`docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md) §5) and carries a lifecycle
`status` in its front-matter. Documents that also hold a global `SLS-XXXX`
identifier are tracked in the standards catalog
([`standards/REGISTRY.md`](../standards/REGISTRY.md)).

Numbering blocks are append-only: a number, once used, is never reused or
renumbered.

## Category blocks

| Block | Category | Directory |
|---|---|---|
| `00xx` | Orthography (alphabet, spelling, capitalization, punctuation) | `spec/orthography/` |
| `01xx` | Grammar (parts of speech, morphology, syntax, …) | `spec/grammar/` |
| `03xx` | Style (registers: formal, academic, journalism, …) | `spec/style/` |
| `05xx` | Translation (EN↔SO guidelines, idioms, false friends, …) | `spec/translation/` |

## Documents

### Orthography (`00xx`)

| ID | Title | Standard | Status | File |
|---|---|---|---|---|
| 0001 | Somali Alphabet | SLS-0001 | Proposed | [`orthography/0001-alphabet.md`](orthography/0001-alphabet.md) |
| 0002 | Spelling Rules | SLS-0002 | *planned* | — |
| 0003 | Capitalization | SLS-0005 | *planned* | — |
| 0004 | Punctuation | SLS-0004 | *planned* | — |

### Grammar (`01xx`)

*Planned — see [`IMPLEMENTATION_PLAN.md`](../IMPLEMENTATION_PLAN.md) Phase 3.*

### Style (`03xx`)

*Planned — see [`IMPLEMENTATION_PLAN.md`](../IMPLEMENTATION_PLAN.md).*

### Translation (`05xx`)

*Planned — see [`IMPLEMENTATION_PLAN.md`](../IMPLEMENTATION_PLAN.md) Phase 7.*

---

Reserved-but-unopened blocks (phonology `07xx`, dialects `08xx`, …) are described
in [`docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md) §21 and open only when their
workstream begins.
