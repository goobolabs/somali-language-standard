from __future__ import annotations

import hashlib
from dataclasses import asdict, dataclass, replace
from pathlib import Path
from typing import Sequence

SEVERITY = {"info": 0, "warning": 1, "error": 2, "fatal": 3}


@dataclass(frozen=True)
class Finding:
    severity: str
    path: str
    line: int
    page: str
    rule: str
    message: str
    suggested_action: str
    assignee: str = "unassigned"
    resolution: str = "open"
    excerpt: str = ""
    finding_id: str = ""

    def finalized(self) -> "Finding":
        evidence = "|".join((self.rule, self.path, str(self.line), self.excerpt.strip()))
        digest = hashlib.sha256(evidence.encode("utf-8")).hexdigest()[:16]
        return replace(self, finding_id=f"RF-{digest}")


class AuditState:
    def __init__(self, root: Path):
        self.root = root.resolve()
        self.resources = self.root / "resources"
        self.findings: list[Finding] = []
        self.inventory: list[dict[str, object]] = []
        self.text: dict[str, str] = {}

    def add(
        self,
        severity: str,
        path: str,
        line: int,
        rule: str,
        message: str,
        action: str,
        excerpt: str = "",
        page: str = "unknown",
    ) -> None:
        self.findings.append(
            Finding(
                severity, path, line, page, rule, message, action,
                excerpt=excerpt[:240],
            ).finalized()
        )

    def sorted_findings(self) -> list[Finding]:
        unique = {finding.finding_id: finding for finding in self.findings}
        return sorted(unique.values(), key=lambda f: (f.path, f.line, f.rule, f.finding_id))


def relative(path: Path, root: Path) -> str:
    return path.resolve().relative_to(root.resolve()).as_posix()


def inventory_summary(inventory: Sequence[dict[str, object]]) -> dict[str, object]:
    extensions: dict[str, int] = {}
    for item in inventory:
        ext = str(item["extension"])
        extensions[ext] = extensions.get(ext, 0) + 1
    canonical = "\n".join(
        f"{item['path']}\t{item['sha256']}"
        for item in sorted(inventory, key=lambda row: str(row["path"]))
    )
    return {
        "file_count": len(inventory),
        "total_bytes": sum(int(item["bytes"]) for item in inventory),
        "total_lines": sum(int(item["lines"]) for item in inventory),
        "extensions": dict(sorted(extensions.items())),
        "resource_tree_sha256": hashlib.sha256(canonical.encode("utf-8")).hexdigest(),
        "files": list(inventory),
    }


def payload(inventory: Sequence[dict[str, object]], findings: Sequence[Finding]) -> dict[str, object]:
    counts = {severity: 0 for severity in SEVERITY}
    for finding in findings:
        counts[finding.severity] += 1
    return {
        "schema_version": 1,
        "inventory": inventory_summary(inventory),
        "finding_counts": counts,
        "findings": [asdict(finding) for finding in findings],
    }
