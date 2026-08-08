# Private Source Evidence Workspace

This directory defines the local workspace for source books, PDF scans, page
images, and raw OCR used to verify `resources/`. Evidence placed in the three
subdirectories below is ignored by Git and must not be committed.

```text
source-evidence/
  originals/   immutable PDFs or page images
  raw-ocr/     exact, uncorrected OCR output
  work/        temporary page renders, alignment files, and review exports
```

Create these subdirectories locally only when needed. A source may be stored
here only when the project or reviewer has the right to possess and process it.
Public-domain or openly licensed evidence may eventually use a public archive,
but it still requires a source-manifest record and checksum before use.

If a source cannot lawfully be copied into the workspace, the future manifest
must record a stable catalog/archive identifier, access restrictions, and the
checksum of the exact copy used during review when one can lawfully be
calculated. Do not evade access controls or redistribute restricted books.

## Preservation rules

- Never edit a file under `originals/` or `raw-ocr/`.
- Compute SHA-256 before review and confirm it again when review finishes.
- Use a source ID in filenames once IDs are assigned in Phase 1.
- Keep derivatives and experiments under `work/`; they are not source evidence.
- Do not move text from raw OCR into `resources/` without page verification.
- Git commit `f04354c608f9f2028b9d101e27dde01181a6f910` is the repository's
  pre-cleanup resource baseline. Phase 0 creates a named Git tag for it.

No source PDFs or page images were present in the workspace during the Phase 0
audit on 2026-08-09.
