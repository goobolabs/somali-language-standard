# Audit record — Hordhac

- **Resource path:** `resources/dhawaaq/01-hordhac.md`
- **Collection / family:** dhawaaq / introduction
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; SLS-native cleanup applied and awaiting
  maintainer cleanup review
- **Audit started:** 2026-08-19
- **File size at audit start:** 69 lines; 400 words; 2,462 bytes
- **Resource SHA-256 at audit start:**
  `d71d7f0bd136f1a870ba9ba237e2f843324b00bfbd31175bb26d79a6afec1aea`
- **Resource-text changes during this audit:** none

## Target output model

This file is already a compact SLS introduction, not an OCR transcript. It has
one H1, five H2 sections, three numbered lists, and no cover matter, exercises,
or page markers.

Cleanup should keep it as the collection's opening chapter: purpose, four
descriptive axes, three positions in the word, what speech is, and the three
speech tasks. It should reframe booklet voice as this collection, repair
`talagalay`, and add links to files `02`–`07` and to `naxwe/01`. It must not
add a phoneme inventory here or import later chapters.

## Source and repository evidence

`resources/dhawaaq/00-sources.md` maps files `01`–`07` to *Codaynta Af
Soomaaliga* (Maxamed Xaaji Xuseen Raabi, 1977). Cleaned `naxwe/14` §1 already
points here for *codayn* and does not copy this chapter.

Repository comparison also included:

- cleaned `resources/qoraal/01-hadal-iyo-qoraal.md` (O01-R001 booklet
  voice);
- cleaned `resources/naxwe/01-ereyada.md` for alphabet versus sound;
- `resources/naxwe/ereyfur.md`; dictionary `sargoyn`, `talagalay` via naxwe
  `loogu talagalay`.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-13 | reviewed | PH01-R001 |
| 15-26 | reviewed | PH01-R002 |
| 28-39 | reviewed | PH01-R003 |
| 41-60 | reviewed | PH01-R004 |
| 62-69 | reviewed | PH01-R005 |
| whole file | reviewed | PH01-R006 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| PH01-R001 | 1-13 | Useful two-part scope (segmental / suprasegmental), but `Buuggani` / `Buuggu` present this SLS chapter as the 1977 booklet; `talogalay` conflicts with repository `talagalay`; `sargoyn` is dictionary-supported / medium | O01-R001 reframed `Buuggani`; cleaned naxwe uses `loogu talagalay`; `resources/qaamuus/08-s.md` `sargoyn` | Reframe the opening as this collection's introduction, not the physical booklet. Correct `talogalay` to `talagalay`. Retain `sargoyn`, the two-part split, and the first-use glosses *phonology*, *segmental sounds*, *suprasegmental sounds*. Do not add author/year here. | `repository-supported`; `scope-correction`; `form-correction`; `intentional-retained` |
| PH01-R002 | 15-26 | The four descriptive axes and the dialect-variation preview match later files / low | Files `02`–`04` implement axes 1–3; file `03` uses bilow/dhex/dhammaad; file `06` is dialect variation | Retain all four numbered axes and the dialect sentence. Link `06-lahjadaha.md`. Do not import dialect tables. | `repository-supported`; `navigation-update`; `intentional-retained` |
| PH01-R003 | 28-39 | The three-position method (`bilow`, `dhex`, `dhammaad`) is this collection's transcription charter; `/b/` is only a method example / low | File `03` `/b/` rows use the same three positions | Retain the method and the `/b/` illustration. Do not add further phoneme examples here. | `repository-supported`; `intentional-retained` |
| PH01-R004 | 41-60 | Clear speech-as-airflow definition and the three-way split that maps the rest of the collection / medium | Files `02`, `03`–`04`, and `05` match the three numbered parts; cleaned `naxwe/14` §1 points to this collection | Retain the definition and the three numbered parts. Convert them to local links to `02`, `03`/`04`, and `05`. Keep the first-use glosses *stream of air*, *speech organs*. | `repository-supported`; `navigation-update`; `intentional-retained` |
| PH01-R005 | 62-69 | The three speech tasks preview file `02` / low | File `02` headings Bilaabidda, Codsiinta, Jeojinta | Retain all three tasks and their English first-use glosses. Link `02-xubnaha-hadalka.md`. | `repository-supported`; `navigation-update`; `intentional-retained` |
| PH01-R006 | whole file | Already SLS-shaped and free of page-marker OCR; booklet voice and missing collection links remain / medium | Five H2 sections, no `## OCR Page` markers; naxwe 14 treats this collection as canonical *codayn* | Preserve the five-section sequence. Apply only the approved form, scope, and link repairs. Do not duplicate files `02`–`07`. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Hordhac** and keep this sequence:
collection purpose; four axes; three positions; what speech is; three speech
tasks. Links go to `02`–`06` as used. No phoneme chart is added. `sargoyn`
stays.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** PH01-R001 through PH01-R006
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Audit validation

- Resource SHA-256 after audit:
  `d71d7f0bd136f1a870ba9ba237e2f843324b00bfbd31175bb26d79a6afec1aea`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, five H2 headings, three numbered lists, no OCR
  page markers.

## Cleanup result and review

### Applied cleanup

- Reframed `Buuggani` / `Buuggu` / `Ujeeddada buugga` as this collection's
  introduction; corrected `talogalay` to `talagalay`.
- Linked dialect variation to `06-lahjadaha.md`.
- Converted the three speech-science parts to links to files `02`, `03`/`04`,
  and `05`; linked the three speech tasks to `02-xubnaha-hadalka.md`.

### Deliberately retained

- `sargoyn`.
- First-use English glosses (*phonology*, *segmental sounds*,
  *suprasegmental sounds*, *stream of air*, *speech organs*, *initiation*,
  *phonation*, *articulation*).
- The `/b/` three-position illustration.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, five H2 headings, three numbered lists, no OCR page
  markers.
- Pre-cleanup forms `Buuggani`, `Buuggu`, and `talogalay` are absent.
- Cleaned file size: 73 lines; 413 words; 2,752 bytes.
- Cleaned SHA-256:
  `d2ce5ee52036b0e0f52d9b1dc07d91d1c70e9b6a97245fe3e64abf5849e18f26`.
