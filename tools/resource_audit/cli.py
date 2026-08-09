from __future__ import annotations

import argparse
import csv
import json
from dataclasses import asdict, fields, replace
from pathlib import Path
from typing import Sequence

from .collections import audit_collections
from .common import audit_files
from .lexical import audit_lexical
from .model import AuditState, Finding, SEVERITY, inventory_summary, payload
from .provenance import audit_provenance, read_tsv


def run_full(root: Path) -> tuple[list[dict[str, object]], list[Finding]]:
    state = AuditState(root)
    audit_files(state)
    if state.resources.is_dir():
        audit_provenance(state)
        audit_lexical(state)
        audit_collections(state)
    return state.inventory, state.sorted_findings()


def write_json(path: Path, value: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2, sort_keys=True) + "\n",
                    encoding="utf-8", newline="\n")


def write_tsv(path: Path, findings: Sequence[Finding]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    names = [field.name for field in fields(Finding)]
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=names, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        for finding in findings:
            writer.writerow(asdict(finding))


def suppressions(path: Path | None) -> tuple[dict[str, str], list[Finding]]:
    if path is None:
        return {}, []
    if not path.exists():
        return {}, [Finding("error", path.as_posix(), 0, "unknown",
                            "CONFIG_SUPPRESSIONS_MISSING", "Suppression file missing",
                            "Create a reviewed suppression file.").finalized()]
    result: dict[str, str] = {}
    errors: list[Finding] = []
    for number, row in enumerate(read_tsv(path), 2):
        required = ("finding_id", "reason", "approved_by", "date")
        if any(not row.get(key, "").strip() for key in required):
            errors.append(Finding(
                "error", path.as_posix(), number, "unknown", "CONFIG_SUPPRESSION_REASON",
                "Suppression lacks ID, reason, approver, or date",
                "Complete the review record; unexplained suppression is invalid.",
                excerpt=str(row),
            ).finalized())
        else:
            result[row["finding_id"].strip()] = row["reason"].strip()
    return result, errors


def apply_suppressions(findings: Sequence[Finding], values: dict[str, str]) -> list[Finding]:
    return [
        replace(finding, resolution=f"suppressed: {values[finding.finding_id]}")
        if finding.finding_id in values else finding
        for finding in findings
    ]


def baseline_ids(path: Path | None) -> set[str]:
    if path is None:
        return set()
    data = json.loads(path.read_text(encoding="utf-8"))
    return {row["finding_id"] for row in data.get("findings", [])}


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(
        description="Read-only SLS resource audit; never changes or generates source wording."
    )
    commands = result.add_subparsers(dest="command", required=True)
    inventory = commands.add_parser("inventory")
    inventory.add_argument("--root", type=Path, default=Path.cwd())
    inventory.add_argument("--json-out", type=Path)
    for name in ("audit", "baseline"):
        command = commands.add_parser(name)
        command.add_argument("--root", type=Path, default=Path.cwd())
        command.add_argument("--json-out", type=Path)
        command.add_argument("--tsv-out", type=Path)
        command.add_argument("--suppressions", type=Path)
        if name == "audit":
            command.add_argument("--baseline", type=Path)
            command.add_argument("--fail-on-new", choices=tuple(SEVERITY) + ("none",),
                                 default="error")
        else:
            command.add_argument("--output", type=Path, required=True)
    return result


def main(argv: Sequence[str] | None = None) -> int:
    args = parser().parse_args(argv)
    if args.command == "inventory":
        state = AuditState(args.root)
        audit_files(state)
        result = inventory_summary(state.inventory)
        if args.json_out:
            write_json(args.json_out, result)
        print(json.dumps({key: value for key, value in result.items() if key != "files"}, indent=2))
        return 1 if any(f.severity == "fatal" for f in state.findings) else 0

    inventory, findings = run_full(args.root)
    values, config_errors = suppressions(args.suppressions)
    findings = apply_suppressions([*findings, *config_errors], values)
    findings = sorted(findings, key=lambda f: (f.path, f.line, f.rule, f.finding_id))
    result = payload(inventory, findings)
    if args.command == "baseline":
        write_json(args.output, result)
        if args.tsv_out:
            write_tsv(args.tsv_out, findings)
        print(f"Wrote {len(findings)} findings to {args.output}")
        return 1 if any(f.severity == "fatal" for f in findings) else 0

    if args.json_out:
        write_json(args.json_out, result)
    if args.tsv_out:
        write_tsv(args.tsv_out, findings)
    known = baseline_ids(args.baseline)
    threshold = 99 if args.fail_on_new == "none" else SEVERITY[args.fail_on_new]
    new = [
        finding for finding in findings
        if finding.finding_id not in known
        and not finding.resolution.startswith("suppressed:")
        and SEVERITY[finding.severity] >= threshold
    ]
    fatal = [finding for finding in findings
             if finding.severity == "fatal" and not finding.resolution.startswith("suppressed:")]
    print(json.dumps({"counts": result["finding_counts"], "new_at_threshold": len(new)}, indent=2))
    for finding in new[:20]:
        print(f"{finding.severity}: {finding.path}:{finding.line} {finding.rule} {finding.finding_id}")
    return 1 if new or fatal else 0
