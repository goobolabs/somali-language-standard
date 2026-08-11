# Resource Metadata Issue Queue

**Opened:** 2026-08-09  
**Status:** Active  
**Owner convention:** The named role owns coordination; it must record the
individual assignee before resolving an issue.

This queue replaces vague “to be confirmed” notes with stable issue IDs,
owners, affected sources, missing evidence, and closure conditions. Unknown
values must remain `unknown` until evidence is found; they must not be inferred
from OCR text or generated.

| ID | Priority | Owner | Scope | Issue/evidence missing | Closure condition | Status |
| --- | --- | --- | --- | --- | --- | --- |
| M-001 | P0 | Source curator | `SRC-QAA-001` | Exact edition, printing, publication year, title page, and colophon | Values transcribed from the registered scan and independently checked | open |
| M-003 | P0 | Rights reviewer | `SRC-QAA-001` | Publisher and republication/processing rights | Rights basis and permitted uses recorded with evidence | open |
| M-015 | P0 | Source curator | `SRC-QAA-001` | Compiler/editor attribution | Exact title-page/colophon attribution recorded | open |
| M-100 | P0 | Source curator | All direct book/PDF sources | No exact PDF or page image is available in the workspace | Each source has a lawful location/reference and exact edition identity | open |
| M-101 | P0 | Source curator | All source-dependent resource files | Represented printed pages and PDF image indexes are unknown | Complete file-to-page mapping recorded | open |
| M-102 | P0 | Provenance reviewer | All direct scans/raw OCR | Source and raw-OCR SHA-256 values unavailable | Hashes recorded and matched before/after review | open |
| M-103 | P1 | OCR curator | All OCR-derived files | OCR engine, version, conversion settings, and date are unknown | Conversion provenance recovered or explicitly documented as irrecoverable | open |
| M-104 | P0 | Rights reviewer | All direct sources | Rights status is not established at source level | One evidence-backed controlled rights value per source | open |
| M-110 | P0 | Source curator | `SRC-NAX-014`–`SRC-NAX-017` | Missing authors/compilers and/or publication years; editions unconfirmed | Title-page and colophon metadata recorded for each source | open |
| M-111 | P0 | Source curator | `SRC-SUU-006`, `010`, `012`, `014`, `017`–`019` | Publication year and editions unconfirmed | Exact year/edition recorded from registered scans | open |
| M-112 | P1 | Source curator | `SRC-SUU-015`–`SRC-SUU-019` | Textbook compiler/institution/publisher metadata incomplete | Credits/title-page metadata recorded | open |
| M-113 | P1 | Source curator | `SRC-EB-001`–`SRC-EB-010` | Edition, full author/compiler credits, publication place, and page coverage incomplete | Each glossary checked against title page/colophon and page map | open |
| M-114 | P1 | Source curator | `SRC-PHO-002`, `SRC-ORT-002` | Supplement edition/publication details and exact represented section incomplete | Full bibliographic and page/section identity recorded | open |
| M-115 | P0 | Source curator | `SRC-EB-010` | `erey-bixin/09` is partial and omitted-page/record coverage is unknown | Usable exact source registered and all represented/omitted pages declared | open |
| M-116 | P1 | Source curator | `S-003` | Current note says “HAAN grammar (in repo)” but no PDF was found | Exact lawful scan location, checksum, edition, and page count recorded | open |
| M-117 | P1 | Provenance reviewer | Files with prior “manually verified” claims | Reviewer, date, exact pages, method, and unresolved count were not recorded | Reproducible review records linked for each claim | open |
| M-118 | P1 | Data curator | `resources/wordlists/01`–`26` | 31 headwords do not directly match current dictionary entries | Differences source-verified and deterministic derivation reconciled | open |
| M-105 | P0 | Source curator | Resolved 2026-08-11: downloaded public copy SHA-256 `28658fd204f9156ed02fb83654366f0c97faf9139682d4a1597a29b2a3ebdbaa`; title/colophon identify Annarita Puglielli and Cabdalla Cumar Mansuur, *Qaamuuska Af-Soomaaliga*, RomaTrE-Press, Roma, 2012 (ISBN 978-88-97524-02-1); direct entry/style comparison matched `baraarujin`, `islaamid`, and `sagal` | Source identity and sample match recorded in `sources.tsv`, manifest, and collection inventories. Separate rights and full page-mapping issues remain open. | resolved — Sharafdin-authorized maintainer review |

## Resolution rules

1. Attach or reference evidence; do not close an issue from memory or likelihood.
2. Update `sources.tsv`, `resource-manifest.tsv`, the applicable
   `00-sources.md`, and the phase report together.
3. Record resolver identity and date.
4. A missing source keeps dependent files `blocked`.
5. Closing a metadata issue does not by itself authenticate a transcription.
