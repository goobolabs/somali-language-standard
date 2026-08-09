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
    audit_gold_sample(state, sources, actual)


def audit_gold_sample(
    state: AuditState,
    sources: list[dict[str, str]],
    actual: dict[str, dict[str, object]],
) -> None:
    path = state.root / "data/provenance/gold-sample.tsv"
    if not path.exists():
        return
    path_rel = relative(path, state.root)
    try:
        rows = read_tsv(path)
    except (UnicodeDecodeError, csv.Error) as exc:
        state.add("error", path_rel, 0, "PROV_GOLD_SAMPLE_INVALID",
                  f"Cannot parse gold-sample TSV: {exc}",
                  "Repair sample metadata without supplying source wording.")
        return
    required = (
        "sample_id", "source_id", "source_sha256", "pdf_image_index",
        "printed_page", "spread_side", "resource_path", "coverage",
        "selection_basis", "evidence_status", "transcription_status",
        "reviewer", "review_status",
    )
    source_by_id = {row.get("source_id", ""): row for row in sources}
    sample_ids: list[str] = []
    page_ids: list[tuple[str, str]] = []
    for number, row in enumerate(rows, 2):
        if any(not row.get(field, "").strip() for field in required):
            state.add("error", path_rel, number, "PROV_GOLD_SAMPLE_BLANK",
                      "Gold-sample row has a blank required field",
                      "Use an explicit controlled value; do not infer source text.")
            continue
        sample_ids.append(row["sample_id"])
        page_ids.append((row["source_id"], row["printed_page"]))
        source = source_by_id.get(row["source_id"])
        if source is None:
            state.add("error", path_rel, number, "PROV_GOLD_SAMPLE_SOURCE",
                      f"Unknown sample source ID: {row['source_id']}",
                      "Use a registered source ID.")
        elif row["source_sha256"] != source.get("source_sha256"):
            state.add("error", path_rel, number, "PROV_GOLD_SAMPLE_HASH",
                      "Sample hash differs from registered source hash",
                      "Stop and identify the exact immutable source copy.")
        if row["resource_path"] not in actual:
            state.add("error", path_rel, number, "PROV_GOLD_SAMPLE_RESOURCE",
                      f"Sample resource is not inventoried: {row['resource_path']}",
                      "Correct only the metadata relationship.")
        try:
            if int(row["pdf_image_index"]) < 1:
                raise ValueError
        except ValueError:
            state.add("error", path_rel, number, "PROV_GOLD_SAMPLE_PAGE_INDEX",
                      "PDF image index must be a positive integer",
                      "Visually identify the exact PDF image index.")
        if row["spread_side"] not in {"left", "right", "not_applicable"}:
            state.add("error", path_rel, number, "PROV_GOLD_SAMPLE_SPREAD_SIDE",
                      f"Invalid spread side: {row['spread_side']}",
                      "Use left, right, or not_applicable after visual inspection.")
    if len(rows) < 20:
        state.add("error", path_rel, 0, "PROV_GOLD_SAMPLE_SIZE",
                  f"Gold sample has {len(rows)} rows; at least 20 are required",
                  "Select additional exact source pages before transcription.")
    if len(sample_ids) != len(set(sample_ids)):
        state.add("error", path_rel, 0, "PROV_GOLD_SAMPLE_DUPLICATE_ID",
                  "Gold sample contains duplicate sample IDs",
                  "Assign one stable ID per selected printed page.")
    if len(page_ids) != len(set(page_ids)):
        state.add("error", path_rel, 0, "PROV_GOLD_SAMPLE_DUPLICATE_PAGE",
                  "Gold sample selects the same source printed page more than once",
                  "Keep each exact source page once.")
