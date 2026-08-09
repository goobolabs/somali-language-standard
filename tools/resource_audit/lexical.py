from __future__ import annotations

import re
from pathlib import Path

from .model import AuditState, relative

ENTRY_RE = re.compile(r"^- \*\*(.+?)\*\*\s+(.+)$")
SUPERSCRIPTS = "¹²³⁴⁵⁶⁷⁸⁹⁰"


def strip_homonym(value: str) -> str:
    return value.rstrip(SUPERSCRIPTS)


def abbreviation_codes(state: AuditState) -> set[str]:
    path = "resources/qaamuus/00-abbreviations.md"
    result: set[str] = set()
    for line in state.text.get(path, "").splitlines():
        if not line.startswith("|"):
            continue
        cells = [cell.strip() for cell in line.strip("|").split("|")]
        if cells and cells[0] not in ("Abbreviation", "---"):
            result.add(cells[0].rstrip("."))
    return result


def audit_lexical(state: AuditState) -> None:
    codes = abbreviation_codes(state)
    qfiles = [
        path for path in sorted((state.resources / "qaamuus").glob("[0-9][0-9]-*.md"))
        if not path.name.startswith("00-")
    ]
    all_heads: set[str] = set()
    heads_by_file: dict[str, list[tuple[int, str]]] = {}
    bodies: list[tuple[str, int, str]] = []
    for file_path in qfiles:
        path = relative(file_path, state.root)
        heads: list[tuple[int, str]] = []
        for number, line in enumerate(state.text.get(path, "").splitlines(), 1):
            if not line.startswith("- "):
                continue
            match = ENTRY_RE.match(line)
            if not match:
                state.add("error", path, number, "QAA_ENTRY_SHAPE",
                          "Dictionary list item lacks bold-headword entry form",
                          "Compare entry boundary with source page.", line)
                continue
            head, body = match.group(1).strip(), match.group(2)
            heads.append((number, head))
            all_heads.add(strip_homonym(head).casefold())
            bodies.append((path, number, body))
            audit_pos_code(state, path, number, body, codes, line)
            if any("0" <= char <= "9" for char in head):
                state.add("warning", path, number, "QAA_HEADWORD_DIGIT",
                          "ASCII digit appears in headword",
                          "Check for OCR or intended superscript.", head)
            if any(char in SUPERSCRIPTS for char in head[:-1]):
                state.add("warning", path, number, "QAA_HOMONYM_POSITION",
                          "Homonym superscript is not at headword end",
                          "Compare exact headword typography with source.", head)
        heads_by_file[path] = heads
    audit_alphabet_placement(state, heads_by_file)
    audit_crossrefs(state, bodies, all_heads)
    audit_wordlists(state, all_heads)


def audit_pos_code(
    state: AuditState, path: str, number: int, body: str, codes: set[str], excerpt: str
) -> None:
    match = re.match(r"([a-z.]+(?:/[a-z.]+)?\d?)\b", body, re.I)
    if not match:
        state.add("warning", path, number, "QAA_POS_MISSING",
                  "Entry has no recognizable leading grammatical code",
                  "Check code and spacing against source/key.", excerpt)
        return
    token = match.group(1).rstrip(".")
    if code_is_valid(token, codes):
        return
    if "." in token or "/" in token:
        state.add("warning", path, number, "QAA_UNKNOWN_GRAMMAR_CODE",
                  f"Unrecognized grammatical code: {token}",
                  "Compare with source and abbreviation key; do not infer a code.", excerpt)
    else:
        state.add("warning", path, number, "QAA_POS_MISSING",
                  "Entry starts with text rather than a recognized grammatical code",
                  "Check headword boundary and code against source.", excerpt)


def code_is_valid(token: str, codes: set[str]) -> bool:
    normalized = re.sub(r"\d+$", "", token.rstrip(".")).replace("/", ".")
    known = [code.rstrip(".").split(".") for code in codes]
    parts = normalized.split(".")
    reachable = {0}
    for index in range(len(parts)):
        if index not in reachable:
            continue
        for code_parts in known:
            if parts[index:index + len(code_parts)] == code_parts:
                reachable.add(index + len(code_parts))
    return len(parts) in reachable


def audit_alphabet_placement(
    state: AuditState, heads_by_file: dict[str, list[tuple[int, str]]]
) -> None:
    order = (
        "b t j x kh d r s sh dh c g f q k l m n w h y a e i o u aa ee ii oo uu"
    ).split()
    for path, heads in heads_by_file.items():
        index = int(Path(path).name.split("-", 1)[0]) - 1
        if index < 0 or index >= len(order):
            continue
        expected = order[index]
        bad = [(line, head) for line, head in heads
               if not strip_homonym(head).casefold().startswith(expected)]
        if bad:
            line, head = bad[0]
            state.add("warning", path, line, "QAA_ALPHABET_PLACEMENT",
                      f"{len(bad)} headword(s) do not start with {expected!r}",
                      "Check source section; never move automatically.", head)


def audit_crossrefs(
    state: AuditState, bodies: list[tuple[str, int, str]], heads: set[str]
) -> None:
    missing: dict[str, list[tuple[int, str, str]]] = {}
    for path, number, body in bodies:
        for target in re.findall(r"\b(?:ld|eeg)\s+([^.;]+)", body):
            candidate = target.split(",", 1)[0].strip().split()[0].strip("*()[],:;")
            candidate = strip_homonym(candidate).casefold()
            if candidate and re.fullmatch(r"[a-z'’-]+", candidate) and candidate not in heads:
                missing.setdefault(path, []).append((number, candidate, body))
    for path, candidates in missing.items():
        number, candidate, body = candidates[0]
        state.add("warning", path, number, "QAA_DANGLING_CROSSREF",
                  f"{len(candidates)} simple cross-reference target(s) not found; first: {candidate}",
                  "Check exact targets/homonyms on source pages.", body)


def audit_wordlists(state: AuditState, dictionary_heads: set[str]) -> None:
    for file_path in sorted((state.resources / "wordlists").glob("[0-9][0-9]-*.md")):
        if file_path.name.startswith("00-"):
            continue
        path = relative(file_path, state.root)
        words: list[tuple[int, str]] = []
        for number, line in enumerate(state.text.get(path, "").splitlines(), 1):
            if line.startswith("#") or not line.strip():
                continue
            if not line.startswith("- ") or not line[2:].strip() or line[2:] != line[2:].strip():
                state.add("error", path, number, "WORDLIST_ENTRY_SHAPE",
                          "Expected one '- headword' item",
                          "Verify against dictionary source.", line)
                continue
            words.append((number, line[2:]))
        folded = [word.casefold() for _, word in words]
        duplicates = len(folded) - len(set(folded))
        if duplicates:
            state.add("warning", path, 0, "WORDLIST_DUPLICATES",
                      f"{duplicates} duplicate headword(s)",
                      "Reconcile with authenticated dictionary.")
        inversions = sum(left > right for left, right in zip(folded, folded[1:]))
        if inversions:
            state.add("warning", path, 0, "WORDLIST_SORT_ORDER",
                      f"{inversions} Unicode-order inversion(s)",
                      "Check declared Somali order before reordering.")
        missing = [word for _, word in words
                   if strip_homonym(word).casefold() not in dictionary_heads]
        if missing:
            state.add("warning", path, 0, "WORDLIST_QAAMUUS_MISMATCH",
                      f"{len(missing)} headword(s) lack direct dictionary match",
                      "Resolve from authenticated dictionary and deterministic extraction.",
                      ", ".join(missing[:8]))
