# Resource Provenance Data

This directory contains machine-readable provenance for the OCR cleanup. It
describes evidence; it does not supply or correct source wording.

## Files

- `sources.tsv` — one row per identified source work or internal/derived source.
- `resource-manifest.tsv` — one row for each of the 145 files that existed under
  `resources/` at the Phase 1 inventory boundary.
- `audit-baseline.json` records the machine-readable Phase 2 review queue used
  to detect new findings without treating existing OCR candidates as corrected.
- `audit-suppressions.tsv` records reviewed rule exceptions; every row requires
  a finding ID, reason, approver, and date.
- [`../../docs/resource-cleanup/METADATA_ISSUES.md`](../../docs/resource-cleanup/METADATA_ISSUES.md)
  — tracked queue for unresolved provenance.

## Controlled missing values

- `unknown` — the value is not established by current evidence.
- `not_applicable` — the field does not apply to this record.
- `not_available_in_workspace` — the required evidence was searched for but was
  not present during the 2026-08-09 audit.
- `unassigned` — a review role has not yet been assigned.

Blank values are not permitted. Existing collection notes are recorded as
`unverified` until checked against the exact title page/colophon and source
scan. A catalog title is not authentication.

## Source IDs

Existing stable ID `S-003` is retained for *Barashada Naxwaha Af Soomaaliga*.
New IDs use `SRC-<COLLECTION>-NNN`; they identify works and do not imply that the
work, edition, rights, or transcription has been authenticated.

`SRC-SLS-INTERNAL` identifies project-authored metadata files such as READMEs and
collection inventories. It is not a book source.

## Status rules

- Project documentation is `derived`.
- A source-dependent file whose exact scan is absent is `blocked`.
- A derived wordlist/paradigm remains `blocked` while its source transcription
  is unauthenticated.
- No Phase 1 row is `authenticated`.

Fields and acceptance rules follow
[`../../docs/TRANSCRIPTION_POLICY.md`](../../docs/TRANSCRIPTION_POLICY.md) and
[`../../docs/REVIEW_GUIDE.md`](../../docs/REVIEW_GUIDE.md).

Audit operation and baseline review are documented in
[`../../docs/RESOURCE_AUDIT.md`](../../docs/RESOURCE_AUDIT.md). Neither the
baseline nor a suppression authenticates source text.
