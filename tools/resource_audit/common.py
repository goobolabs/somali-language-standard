from __future__ import annotations

import hashlib
import re
import unicodedata
from pathlib import Path

from .model import AuditState, relative

MOJIBAKE = ("Ã", "Â", "â€", "ðŸ", "�")
ZERO_WIDTH = ("\u200b", "\u200c", "\u200d", "\u2060", "\ufeff")
MD_LINK_RE = re.compile(r"\[[^\]]*\]\(([^)]+)\)")


def audit_files(state: AuditState) -> None:
    if not state.resources.is_dir():
        state.add("fatal", "resources", 0, "INV_RESOURCES_MISSING",
                  "resources/ directory does not exist", "Restore resources/.")
        return
    for path in sorted(p for p in state.resources.rglob("*") if p.is_file()):
        audit_file(state, path)


def audit_file(state: AuditState, path: Path) -> None:
    path_rel = relative(path, state.root)
    data = path.read_bytes()
    state.inventory.append({
        "path": path_rel,
        "bytes": len(data),
        "sha256": hashlib.sha256(data).hexdigest(),
        "extension": path.suffix.lower(),
        "lines": data.count(b"\n") + (1 if data and not data.endswith(b"\n") else 0),
    })
    try:
        text = data.decode("utf-8", errors="strict")
    except UnicodeDecodeError as exc:
        state.add("error", path_rel, 0, "ENC_INVALID_UTF8",
                  f"Invalid UTF-8 at byte {exc.start}",
                  "Recover exact characters from the source page.")
        return
    state.text[path_rel] = text
    audit_encoding(state, path_rel, data, text)
    if path.suffix.lower() == ".md":
        audit_markdown(state, path, path_rel, text)
    audit_ocr(state, path_rel, text)


def audit_encoding(state: AuditState, path: str, data: bytes, text: str) -> None:
    if data.startswith(b"\xef\xbb\xbf"):
        state.add("warning", path, 1, "ENC_UTF8_BOM", "UTF-8 BOM is present",
                  "Verify before mechanical removal.")
    normalized = unicodedata.normalize("NFC", text)
    if normalized != text:
        state.add("warning", path, 0, "ENC_NOT_NFC",
                  "Text differs from Unicode NFC normalization",
                  "Inspect source glyphs; do not auto-normalize.")
    crlf = data.count(b"\r\n")
    lf = data.count(b"\n") - crlf
    cr = data.count(b"\r") - crlf
    if sum(value > 0 for value in (crlf, lf, cr)) > 1:
        state.add("error", path, 0, "ENC_MIXED_LINE_ENDINGS",
                  f"Mixed line endings CRLF={crlf}, LF={lf}, CR={cr}",
                  "Review conversion boundaries before normalization.")
    elif crlf or cr:
        state.add("warning", path, 0, "ENC_NON_LF_LINE_ENDINGS",
                  f"Expected LF; found CRLF={crlf}, CR={cr}",
                  "Normalize only as a reviewed mechanical change.")
    for number, line in enumerate(text.splitlines(), 1):
        if "\ufffd" in line:
            state.add("error", path, number, "ENC_REPLACEMENT_CHARACTER",
                      "Unicode replacement character found",
                      "Recover the exact source character.", line)
        if "\u00a0" in line:
            state.add("warning", path, number, "ENC_NBSP", "Non-breaking space found",
                      "Confirm whether source layout requires it.", line)
        if any(char in line for char in ZERO_WIDTH):
            state.add("warning", path, number, "ENC_ZERO_WIDTH",
                      "Zero-width character found", "Inspect before removal.", line)
        controls = [c for c in line if unicodedata.category(c) == "Cc" and c != "\t"]
        if controls:
            state.add("error", path, number, "ENC_CONTROL_CHARACTER",
                      "Unexpected control character found",
                      "Inspect the source conversion.", line)
        if any(signature in line for signature in MOJIBAKE):
            state.add("error", path, number, "ENC_MOJIBAKE_CANDIDATE",
                      "Common mojibake signature found",
                      "Compare source bytes/page; never globally re-encode.", line)


def audit_markdown(state: AuditState, file_path: Path, path: str, text: str) -> None:
    lines = text.splitlines()
    h1 = [number for number, line in enumerate(lines, 1) if line.startswith("# ")]
    if len(h1) != 1:
        state.add("error", path, h1[0] if h1 else 0, "MD_H1_COUNT",
                  f"Expected one H1; found {len(h1)}",
                  "Compare structure with source and collection conventions.")
    previous = 0
    malformed: list[tuple[int, str]] = []
    for number, line in enumerate(lines, 1):
        match = re.match(r"^(#{1,6})\s+", line)
        if match:
            level = len(match.group(1))
            if previous and level > previous + 1:
                state.add("warning", path, number, "MD_HEADING_JUMP",
                          f"Heading jumps H{previous} to H{level}",
                          "Check printed heading hierarchy.", line)
            previous = level
        if re.match(r"^[-+*][^\s*-]", line):
            malformed.append((number, line))
        if re.match(r"^\s*(?:page|bogga|ocr page)\s*[:#-]?\s*\d+\s*$", line, re.I):
            state.add("warning", path, number, "MD_PAGE_FURNITURE",
                      "Possible page marker in body text",
                      "Retain page mapping outside presentation text.", line)
    if malformed:
        number, line = malformed[0]
        state.add("warning", path, number, "MD_MALFORMED_LIST",
                  f"{len(malformed)} list-like line(s) lack following space",
                  "Determine whether these are OCR text or malformed Markdown.", line)
    if text.count("**") % 2:
        state.add("warning", path, 0, "MD_UNBALANCED_BOLD",
                  "Odd Markdown bold-marker count", "Inspect OCR asterisks and emphasis.")
    if text.count("`") % 2:
        state.add("warning", path, 0, "MD_UNBALANCED_BACKTICK",
                  "Odd Markdown backtick count", "Inspect notation delimiters.")
    audit_tables(state, path, lines)
    audit_links(state, file_path, path, lines)


def audit_tables(state: AuditState, path: str, lines: list[str]) -> None:
    block: list[tuple[int, str]] = []
    def finish(rows: list[tuple[int, str]]) -> None:
        if len(rows) < 2:
            return
        counts = [row.count("|") for _, row in rows]
        if len(set(counts)) > 1:
            state.add("warning", path, rows[0][0], "MD_TABLE_COLUMN_MISMATCH",
                      f"Table pipe counts differ: {counts}",
                      "Compare row/column layout with source.", rows[0][1])
        if not re.match(r"^\|?\s*:?-{3,}", rows[1][1].strip()):
            state.add("warning", path, rows[1][0], "MD_TABLE_SEPARATOR",
                      "Possible table lacks valid separator",
                      "Verify whether pipes are a table or OCR debris.", rows[1][1])
    for number, line in enumerate(lines, 1):
        if line.strip().startswith("|") and line.count("|") >= 2:
            block.append((number, line))
        else:
            finish(block)
            block = []
    finish(block)


def audit_links(state: AuditState, file_path: Path, path: str, lines: list[str]) -> None:
    for number, line in enumerate(lines, 1):
        for target in MD_LINK_RE.findall(line):
            target = target.strip().split()[0].strip("<>")
            if target.startswith(("http://", "https://", "mailto:", "#")):
                continue
            local = target.split("#", 1)[0]
            if local and not (file_path.parent / local).resolve().exists():
                state.add("error", path, number, "MD_BROKEN_LOCAL_LINK",
                          f"Missing local link target: {local}",
                          "Correct only the project link.", line)


def audit_ocr(state: AuditState, path: str, text: str) -> None:
    patterns = (
        (r"[|_=~]{5,}", "OCR_GARBAGE_RUN", "Long OCR-prone symbol run"),
        (r"([!?.,;:])\1{3,}", "OCR_REPEATED_PUNCT", "Repeated punctuation"),
        (r"\b[A-Za-z]{1,3}\d[A-Za-z]+\b|\b[A-Za-z]+\d[A-Za-z]{1,3}\b",
         "OCR_DIGIT_LETTER", "Digit embedded in alphabetic token"),
        (r"\S {5,}\S", "OCR_SPACING_RUN", "Large internal spacing/layout candidate"),
        (r"(?i)\b(?:scant|[fghr]ocr(?:tii)?|docrr\w*)\b",
         "OCR_SOMALI_CONFUSION", "Known OCR-like Somali token pattern"),
    )
    matches: dict[str, list[tuple[int, str, str]]] = {}
    for number, line in enumerate(text.splitlines(), 1):
        stripped = line.strip()
        for pattern, rule, message in patterns:
            if re.search(pattern, line):
                matches.setdefault(rule, []).append((number, line, message))
        if re.search(r"\w-\s*$", line) and not stripped.startswith(("- ", "|")):
            matches.setdefault("OCR_LINE_END_HYPHEN", []).append(
                (number, line, "Word ends in hyphen at line boundary"))
        if len(stripped) == 1 and not stripped.startswith("#"):
            matches.setdefault("OCR_ISOLATED_GLYPH", []).append(
                (number, line, "Isolated glyph line"))
        if line.count("|") >= 4 and not stripped.startswith("|"):
            matches.setdefault("OCR_INTERLEAVED_COLUMNS", []).append(
                (number, line, "Many pipes outside Markdown table"))
    info_rules = {"OCR_LINE_END_HYPHEN", "OCR_ISOLATED_GLYPH"}
    for rule, candidates in matches.items():
        number, line, message = candidates[0]
        state.add("info" if rule in info_rules else "warning", path, number, rule,
                  f"{message}; {len(candidates)} occurrence(s)",
                  "Candidate only: compare with exact source page.", line)
