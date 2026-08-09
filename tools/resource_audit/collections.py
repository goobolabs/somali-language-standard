from __future__ import annotations

import csv
import re

from .model import AuditState, relative


def audit_collections(state: AuditState) -> None:
    audit_naxwe(state)
    audit_erey_bixin(state)
    audit_suugaan(state)
    audit_specialist(state)


def audit_naxwe(state: AuditState) -> None:
    file_path = state.resources / "naxwe/ereyfur.tsv"
    path = relative(file_path, state.root)
    text = state.text.get(path)
    if text is not None:
        seen: set[tuple[str, ...]] = set()
        for number, row in enumerate(csv.reader(text.splitlines(), delimiter="\t"), 1):
            if len(row) != 3:
                state.add("error", path, number, "NAX_TSV_COLUMNS",
                          f"Glossary row has {len(row)} columns; expected 3",
                          "Compare source table boundary.", "\t".join(row))
            key = tuple(cell.casefold().strip() for cell in row)
            if key in seen:
                state.add("warning", path, number, "NAX_TSV_DUPLICATE",
                          "Duplicate grammar glossary row",
                          "Check whether source intentionally repeats it.", "\t".join(row))
            seen.add(key)
    for file_path in sorted((state.resources / "naxwe").glob("[0-9][0-9]-*.md")):
        if file_path.name.startswith("00-"):
            continue
        path = relative(file_path, state.root)
        candidates: list[tuple[int, str]] = []
        for number, line in enumerate(state.text.get(path, "").splitlines(), 1):
            if line.startswith("*") and not line.startswith(("* ", "**")):
                candidates.append((number, line))
        if candidates:
            number, line = candidates[0]
            state.add("info", path, number, "NAX_EXAMPLE_MARKER",
                      f"{len(candidates)} possible example marker(s) or OCR asterisk(s)",
                      "Verify markers against source.", line)


def audit_erey_bixin(state: AuditState) -> None:
    for file_path in sorted((state.resources / "erey-bixin").glob("[0-9][0-9]-*.md")):
        path = relative(file_path, state.root)
        seen: set[str] = set()
        duplicates: list[tuple[int, str, str]] = []
        for number, line in enumerate(state.text.get(path, "").splitlines(), 1):
            if not line.strip() or line.startswith(("#", "|", "<!--")):
                continue
            count = line.count(" — ")
            if count != 1:
                state.add("warning", path, number, "EB_TERM_DELIMITER",
                          f"Term record has {count} em-dash delimiters; expected 1",
                          "Compare source/target columns with source page.", line)
                continue
            left, right = (part.strip() for part in line.split(" — ", 1))
            if not left or not right:
                state.add("error", path, number, "EB_EMPTY_TERM_SIDE",
                          "Empty source or target term",
                          "Recover exact cell from source page.", line)
            key = left.casefold()
            if key in seen:
                duplicates.append((number, left, line))
            seen.add(key)
        if duplicates:
            number, left, line = duplicates[0]
            state.add("warning", path, number, "EB_DUPLICATE_SOURCE_TERM",
                      f"{len(duplicates)} duplicate source-term occurrence(s); first: {left}",
                      "Check domain/sense repetition against source.", line)


def audit_suugaan(state: AuditState) -> None:
    for file_path in sorted((state.resources / "suugaan").glob("[0-9][0-9]-*.md")):
        path = relative(file_path, state.root)
        lines = state.text.get(path, "").splitlines()
        if not any(line.startswith("## ") for line in lines):
            state.add("info", path, 0, "SUU_NO_SECTION_HEADINGS",
                      "No H2 chapter/poem headings",
                      "Confirm source structure; never invent headings.")
        layout = [(number, line) for number, line in enumerate(lines, 1)
                  if re.search(r"\S {5,}\S|[|_=]{5,}", line)]
        if layout:
            number, line = layout[0]
            state.add("warning", path, number, "SUU_LAYOUT_CANDIDATE",
                      f"{len(layout)} line(s) may have damaged prose/verse layout",
                      "Inspect lineation, columns, speaker, and repetition.", line)
        speakers = [(number, line) for number, line in enumerate(lines, 1)
                    if re.match(r"^[A-ZÀ-ÖØ-Þ][^:]{1,35}\s+:\s*", line)]
        if speakers:
            number, line = speakers[0]
            state.add("info", path, number, "SUU_SPEAKER_LABEL_SPACING",
                      f"{len(speakers)} possible speaker label(s) with space before colon",
                      "Check printed labels; do not normalize automatically.", line)


def audit_specialist(state: AuditState) -> None:
    for collection in ("orthography", "phonology", "morphology"):
        source_path = f"resources/{collection}/00-sources.md"
        source_text = state.text.get(source_path, "")
        if "## Phase 1 provenance status" not in source_text:
            state.add("error", source_path, 0, "COLLECTION_SOURCE_STATUS",
                      "Collection source inventory lacks Phase 1 provenance status",
                      "Reconcile with provenance manifest.")
    for file_path in sorted((state.resources / "morphology").glob("[0-9][0-9]-*.md")):
        if file_path.name.startswith("00-"):
            continue
        path = relative(file_path, state.root)
        if "**Ilaha:**" not in state.text.get(path, ""):
            state.add("error", path, 0, "MOR_SOURCE_NOTE",
                      "Derived morphology file lacks **Ilaha:** source note",
                      "Restore verifiable relationship; do not add linguistic content.")
    orth = state.text.get("resources/orthography/05-astaamaynta.md", "")
    if orth and not all(symbol in orth for symbol in (".", ",", "?", "!")):
        state.add("warning", "resources/orthography/05-astaamaynta.md", 0,
                  "ORT_PUNCTUATION_COVERAGE", "Core punctuation glyph sample is incomplete",
                  "Compare punctuation sections with source inventory.")
    phon = state.text.get("resources/phonology/07-xuruufta-caalamiga.md", "")
    if phon and not re.search(r"[ɐ-ʶ]", phon):
        state.add("warning", "resources/phonology/07-xuruufta-caalamiga.md", 0,
                  "PHO_IPA_COVERAGE", "IPA-range symbol not detected in IPA reference file",
                  "Check symbol encoding against source; never substitute by appearance.")
