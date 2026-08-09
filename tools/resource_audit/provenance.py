from __future__ import annotations

import csv
from pathlib import Path

from .model import AuditState, relative


def read_tsv(path: Path) -> list[dict[str, str]]:
    with path.open("r", encoding="utf-8", errors="strict", newline="") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def audit_provenance(state: AuditState) -> None:
    source_path = state.root / "data/provenance/sources.tsv"
    manifest_path = state.root / "data/provenance/resource-manifest.tsv"
    if not source_path.exists() or not manifest_path.exists():
        state.add("fatal", "data/provenance", 0, "PROV_FILES_MISSING",
                  "sources.tsv or resource-manifest.tsv is missing",
                  "Restore Phase 1 provenance records.")
        return
    try:
        sources = read_tsv(source_path)
        manifest = read_tsv(manifest_path)
    except (UnicodeDecodeError, csv.Error) as exc:
        state.add("fatal", relative(manifest_path, state.root), 0, "PROV_TSV_INVALID",
                  f"Cannot parse provenance TSV: {exc}",
                  "Repair metadata structure without changing resource text.")
        return
    source_ids = [row.get("source_id", "") for row in sources]
    if len(source_ids) != len(set(source_ids)):
        state.add("error", relative(source_path, state.root), 0,
                  "PROV_DUPLICATE_SOURCE_ID", "Duplicate source IDs found",
                  "Keep one stable ID per source work.")
    for number, row in enumerate(sources, 2):
        for field, value in row.items():
            if not field or not (value or "").strip():
                state.add("error", relative(source_path, state.root), number,
                          "PROV_BLANK_SOURCE_FIELD", f"Blank source field: {field}",
                          "Use a controlled explicit missing value.")
    actual = {str(item["path"]): item for item in state.inventory}
    by_path: dict[str, list[dict[str, str]]] = {}
    for number, row in enumerate(manifest, 2):
        path = row.get("resource_path", "")
        by_path.setdefault(path, []).append(row)
        if row.get("source_id") not in source_ids:
            state.add("error", path or relative(manifest_path, state.root), number,
                      "PROV_UNKNOWN_SOURCE_ID",
                      f"Unknown source ID: {row.get('source_id')}",
                      "Add verified source metadata or correct relationship.")
        for field, value in row.items():
            if not field or not (value or "").strip():
                state.add("error", relative(manifest_path, state.root), number,
                          "PROV_BLANK_MANIFEST_FIELD", f"Blank manifest field: {field}",
                          "Use a controlled explicit missing value.")
    for path in sorted(set(actual) - set(by_path)):
        state.add("error", path, 0, "PROV_UNMANIFESTED_RESOURCE",
                  "Resource is absent from manifest",
                  "Add reviewed source relationship before acceptance.")
    for path in sorted(set(by_path) - set(actual)):
        state.add("error", path, 0, "PROV_MISSING_RESOURCE",
                  "Manifested resource is missing",
                  "Restore file or record reviewed removal.")
    mismatches: list[str] = []
    for path in sorted(set(actual) & set(by_path)):
        expected = by_path[path][0].get("resource_sha256", "")
        if actual[path]["sha256"] != expected:
            mismatches.append(path)
            state.add("error", path, 0, "PROV_RESOURCE_HASH_MISMATCH",
                      "Resource differs from Phase 1 manifest hash",
                      "Review full diff and source evidence before updating manifest.")
    if len(mismatches) >= 20:
        state.add("error", "resources", 0, "PROV_UNREVIEWED_BULK_CHANGE",
                  f"{len(mismatches)} resource hashes changed",
                  "Stop; require bounded rule, dry run, source checks, and review.")
    for row in manifest:
        if (row.get("transcription_status") == "authenticated"
                and row.get("source_location") == "not_available_in_workspace"):
            state.add("error", row.get("resource_path", ""), 0,
                      "PROV_FALSE_AUTHENTICATION",
                      "Authenticated status with unavailable exact source",
                      "Downgrade or register and review exact evidence.")
