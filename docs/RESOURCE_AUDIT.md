# Resource Audit

The resource audit is a read-only quality and provenance check for `resources/`.
It reports candidates for human review; it never corrects, normalizes, or
generates source wording.

## Requirements

- Python 3.12 or newer
- no third-party Python packages

Run all commands from the repository root.

## Commands

Inventory the resource tree:

```shell
python tools/audit_resources.py inventory --root .
```

Run the review gate used by CI:

```shell
python tools/audit_resources.py audit --root . --baseline data/provenance/audit-baseline.json --suppressions data/provenance/audit-suppressions.tsv --fail-on-new warning
```

Write machine-readable JSON and TSV reports when investigating findings:

```shell
python tools/audit_resources.py audit --root . --baseline data/provenance/audit-baseline.json --suppressions data/provenance/audit-suppressions.tsv --fail-on-new warning --json-out temp/resource-audit.json --tsv-out temp/resource-audit.tsv
```

Run the tests:

```shell
python -m unittest discover -s tools/tests -v
```

If the package cannot be imported in a particular environment, set
`PYTHONPATH=tools` for the test command.

## Findings

Each finding records a stable ID, severity, path, line, page when known, rule,
message, suggested action, assignee, resolution, and excerpt. Stable IDs are
derived from the rule and evidence location so an unchanged issue retains its
identity. Heuristic warnings are review candidates and can be false positives,
especially in poetry, historical spelling, abbreviations, and tables.

The audit checks strict UTF-8, Unicode and line endings, Markdown structure, OCR
signals, collection-specific structure, and the Phase 1 provenance records. It
also blocks unreviewed resource hash drift and bulk changes.

## Baseline and suppressions

`data/provenance/audit-baseline.json` records the accepted Phase 2 review queue.
CI fails when a warning-or-higher finding appears that is not in that baseline.
The baseline must never be regenerated automatically. A maintainer may update it
only after reviewing the diff and confirming that no source wording was invented
or silently normalized.

`data/provenance/audit-suppressions.tsv` is for confirmed rule exceptions. Every
row requires a finding ID, reason, approver, and date. Suppression does not edit
the file or declare the underlying source authenticated.

To regenerate the baseline after an approved validator or evidence change:

```shell
python tools/audit_resources.py baseline --root . --suppressions data/provenance/audit-suppressions.tsv --output data/provenance/audit-baseline.json
```

Review the resulting JSON and Git diff before committing it.
