# Sources — naxwe

Collection inventory for the curated grammar reference library. Attribution is kept
here once, so content files stay clean.

## Primary grammar (Puglielli & Mansuur, 1998)

| File | Source title | Authors | Year |
|---|---|---|---|
| `00-luqadda-iyo-fekerka.md` … `12-noocyada-weeraha.md` | Barashada Naxwaha Af Soomaaliga | Abdalla Omar Mansur; Annarita Puglielli | 1998 |
| `ereyfur.tsv` | Grammar-term glossary (Somali / English / Italian) | See S-003 | 1998 |

## Supplementary grammars

| File | Source title | Author / compiler | Year |
|---|---|---|---|
| `13-aasaaska-naxwaha.md` | Aasaaska Naxwaha Af Soomaaliga | Guddiga Af Soomaaliga | 1973 |
| `14-naxwaha-cusub.md` | Naxwaha Cusub ee Af Soomaaliga | unknown (M-110) | unknown (M-110) |
| `15-naxwaha-sifayneed.md` | Naxwaha Sifayneed ee Af Soomaaliga | unknown (M-110) | unknown (M-110) |
| `16-weeraynta-soomaaliga.md` | WEEraynta Soomaaliga (Xilliyada 5aad) | unknown (M-110) | unknown (M-110) |
| `17-naxwaha-af-soomaaliga.md` | Naxwaha Af Soomaaliga | Jaalle Shire Jaamac Axmed | unknown (M-110) |

## Coverage notes

- Chapters `00`–`12` are structured from the 1998 HAAN grammar (clean extraction).
- Supplementary files (`13`–`17`) are curated from OCR sources, but a 2026-07-18
  review found that 0.9–6.3% of lines per file still contain raw, unrepaired OCR
  noise (broken words, stray symbols, interleaved table-of-contents fragments)
  recurring throughout each document. These five files are flagged **review**
  pending a remediation decision and should not be treated as fully verified
  against their source scans yet.
- *Habka Qoraalka* (1977) is curated in `resources/orthography/`.
- *Codaynta Af Soomaaliga* (1977) is curated in `resources/phonology/`.

## Phase 1 provenance status (2026-08-09)

| Files | Provisional source ID |
| --- | --- |
| `00-luqadda-iyo-fekerka.md`–`12-noocyada-weeraha.md`; `ereyfur.tsv` | `S-003` |
| `13-aasaaska-naxwaha.md` | `SRC-NAX-013` |
| `14-naxwaha-cusub.md` | `SRC-NAX-014` |
| `15-naxwaha-sifayneed.md` | `SRC-NAX-015` |
| `16-weeraynta-soomaaliga.md` | `SRC-NAX-016` |
| `17-naxwaha-af-soomaaliga.md` | `SRC-NAX-017` |

No exact scan was present for any of these works. All source-dependent files are
therefore `blocked`, including chapters previously described as clean
extractions. Missing supplement metadata is tracked by M-110 and missing S-003
evidence by M-116. See [`sources.tsv`](../../data/provenance/sources.tsv),
[`resource-manifest.tsv`](../../data/provenance/resource-manifest.tsv), and
[`METADATA_ISSUES.md`](../../docs/resource-cleanup/METADATA_ISSUES.md).
