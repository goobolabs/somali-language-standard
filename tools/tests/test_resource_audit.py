from __future__ import annotations

import csv
import hashlib
import io
import json
import os
import tempfile
import unittest
from contextlib import redirect_stdout
from pathlib import Path

from resource_audit.cli import main, suppressions
from resource_audit.collections import audit_erey_bixin
from resource_audit.common import audit_file, audit_files
from resource_audit.lexical import code_is_valid
from resource_audit.model import AuditState, Finding
from resource_audit.provenance import audit_provenance


def temporary_directory() -> tempfile.TemporaryDirectory[str]:
    base = os.environ.get("SLS_TEST_TMP")
    if base:
        Path(base).mkdir(parents=True, exist_ok=True)
    return tempfile.TemporaryDirectory(dir=base)


class ResourceAuditTests(unittest.TestCase):
    def test_finding_id_is_deterministic(self) -> None:
        finding = Finding("warning", "resources/a.md", 3, "unknown", "RULE",
                          "message", "action", excerpt="same").finalized()
        again = Finding("warning", "resources/a.md", 3, "unknown", "RULE",
                        "different message", "different action", excerpt="same").finalized()
        self.assertEqual(finding.finding_id, again.finding_id)
        self.assertRegex(finding.finding_id, r"^RF-[0-9a-f]{16}$")

    def test_invalid_utf8_is_error(self) -> None:
        with temporary_directory() as directory:
            root = Path(directory)
            path = root / "resources/bad.md"
            path.parent.mkdir()
            path.write_bytes(b"# title\n\xff")
            state = AuditState(root)
            audit_file(state, path)
            self.assertIn("ENC_INVALID_UTF8", {item.rule for item in state.findings})

    def test_cli_blocks_new_encoding_error(self) -> None:
        with temporary_directory() as directory:
            root = Path(directory)
            resource = root / "resources/bad.md"
            resource.parent.mkdir()
            bad_bytes = b"# title\n\xff"
            resource.write_bytes(bad_bytes)
            provenance = root / "data/provenance"
            provenance.mkdir(parents=True)
            (provenance / "sources.tsv").write_text(
                "source_id\nSRC-1\n", encoding="utf-8")
            (provenance / "resource-manifest.tsv").write_text(
                "resource_path\tsource_id\tresource_sha256\ttranscription_status\tsource_location\n"
                f"resources/bad.md\tSRC-1\t{hashlib.sha256(bad_bytes).hexdigest()}\tblocked\tunknown\n",
                encoding="utf-8",
            )
            baseline = root / "baseline.json"
            baseline.write_text(json.dumps({"findings": []}), encoding="utf-8")
            with redirect_stdout(io.StringIO()):
                result = main([
                    "audit", "--root", str(root), "--baseline", str(baseline),
                    "--fail-on-new", "error",
                ])
            self.assertEqual(1, result)

    def test_audit_is_read_only(self) -> None:
        with temporary_directory() as directory:
            root = Path(directory)
            path = root / "resources/ok.md"
            path.parent.mkdir()
            original = b"# Title\n\nSomali text.\n"
            path.write_bytes(original)
            state = AuditState(root)
            audit_files(state)
            self.assertEqual(original, path.read_bytes())

    def test_ocr_candidates_are_summarized_per_file(self) -> None:
        with temporary_directory() as directory:
            root = Path(directory)
            path = root / "resources/noisy.md"
            path.parent.mkdir()
            path.write_text("# Title\nabc1def\nxyz2abc\n", encoding="utf-8")
            state = AuditState(root)
            audit_file(state, path)
            matches = [item for item in state.findings if item.rule == "OCR_DIGIT_LETTER"]
            self.assertEqual(1, len(matches))
            self.assertIn("2 occurrence", matches[0].message)

    def test_compound_dictionary_codes_use_abbreviation_units(self) -> None:
        codes = {"m", "dh", "sh.r", "e.d", "f", "g", "mg"}
        self.assertTrue(code_is_valid("e.d", codes))
        self.assertTrue(code_is_valid("m.dh.sh.r", codes))
        self.assertTrue(code_is_valid("f.g/mg2", codes))
        self.assertFalse(code_is_valid("m.unknown", codes))

    def test_glossary_empty_side_is_error(self) -> None:
        with temporary_directory() as directory:
            root = Path(directory)
            path = root / "resources/erey-bixin/01-test.md"
            path.parent.mkdir(parents=True)
            path.write_text("# Terms\n\n — Somali\n", encoding="utf-8")
            state = AuditState(root)
            state.text["resources/erey-bixin/01-test.md"] = path.read_text(encoding="utf-8")
            audit_erey_bixin(state)
            self.assertIn("EB_EMPTY_TERM_SIDE", {item.rule for item in state.findings})

    def test_provenance_hash_drift_is_error(self) -> None:
        with temporary_directory() as directory:
            root = Path(directory)
            resources = root / "resources"
            provenance = root / "data/provenance"
            resources.mkdir(parents=True)
            provenance.mkdir(parents=True)
            resource = resources / "a.md"
            resource.write_text("# A\n", encoding="utf-8")
            with (provenance / "sources.tsv").open("w", encoding="utf-8", newline="") as handle:
                writer = csv.DictWriter(handle, fieldnames=["source_id"], delimiter="\t")
                writer.writeheader(); writer.writerow({"source_id": "SRC-1"})
            fields = ["resource_path", "source_id", "resource_sha256",
                      "transcription_status", "source_location"]
            with (provenance / "resource-manifest.tsv").open("w", encoding="utf-8", newline="") as handle:
                writer = csv.DictWriter(handle, fieldnames=fields, delimiter="\t")
                writer.writeheader(); writer.writerow({
                    "resource_path": "resources/a.md", "source_id": "SRC-1",
                    "resource_sha256": "0" * 64, "transcription_status": "blocked",
                    "source_location": "not_available_in_workspace",
                })
            state = AuditState(root)
            audit_files(state); audit_provenance(state)
            self.assertIn("PROV_RESOURCE_HASH_MISMATCH", {item.rule for item in state.findings})

    def test_suppression_requires_reason_and_approval(self) -> None:
        with temporary_directory() as directory:
            path = Path(directory) / "s.tsv"
            path.write_text("finding_id\treason\tapproved_by\tdate\nRF-1\t\t\t\n", encoding="utf-8")
            values, errors = suppressions(path)
            self.assertEqual({}, values)
            self.assertEqual(["CONFIG_SUPPRESSION_REASON"], [item.rule for item in errors])


if __name__ == "__main__":
    unittest.main()
