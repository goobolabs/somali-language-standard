# Audit record — Shibbanayaasha

- **Resource path:** `resources/dhawaaq/03-shibbanayaasha.md`
- **Collection / family:** dhawaaq / consonants
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; SLS-native cleanup applied and awaiting
  maintainer cleanup review
- **Audit started:** 2026-08-19
- **File size at audit start:** 291 lines; 1,586 words; 9,998 bytes
- **Resource SHA-256 at audit start:**
  `99abfd4b2806aadeb18ffcdbd46421ba453a46e6e74a8a779720067b2251445d`
- **Resource-text changes during this audit:** none

## Target output model

This file is already a compact SLS consonant inventory, not an OCR transcript.
It has one H1, nine H2 manner sections, twenty-one H3 phoneme subsections, two
tables, and no page markers.

Cleanup should keep it as the maintained manner-class reference. It should
present this source's IPA labels as *Codaynta*'s transcription, not as
SLS-0001 letters; repair uniquely supported OCR (`xabbbad`, `facarada`); and
link files `02`, `04`, `07`, `08`, and `naxwe/01`. It must not invent a `/ɖ/`
or `/ʕ/` row, swap the `/ʤ/` and `/tʃ/` example sets, or rewrite IPA to a
modern textbook mapping.

## Source and repository evidence

Mapped to *Codaynta* consonant chapters. Comparison:

- cleaned `naxwe/01` §1.2: 21 consonant letters + glottal sign, including
  `dh`, `kh`, `c`, `j`, `sh`;
- `spec/orthography/0001-alphabet.md` (phonology is out of that spec's
  scope);
- `resources/dhawaaq/08-gariirka-iyo-glotis-furan.md` for final-stop
  analysis without replacing this inventory;
- dictionary `tahar`, `xabbad`, `xabad`, `dhiig`, `shaah`, `caarada`.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-17 | reviewed | PH03-R001 |
| 19-84 | reviewed | PH03-R002 |
| 86-107 | reviewed | PH03-R003 |
| 109-159 | reviewed | PH03-R004 |
| 161-229 | reviewed | PH03-R005 |
| 231-256 | reviewed | PH03-R006 |
| 258-291 | reviewed | PH03-R007 |
| whole file | reviewed | PH03-R008 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| PH03-R001 | 1-17 | Seven manner classes are the chapter's charter; “kor ku xusan” points at file `02` without a link / low | File `02` three consonant diagnostics | Retain the seven-class opening and the plosive/stop double label. Link `02-xubnaha-hadalka.md`. Do not add a manner class. | `repository-supported`; `navigation-update`; `intentional-retained` |
| PH03-R002 | 19-84 | Plosive subsections are the core stop inventory; `/t/` repeats *tahar* three times; `/g/` pairs *good* with `/ge:d/`; `/ʔ/` spellings `go'aansho` / `la'aansho` do not match the IPA `/goga:n/` / `/la:ga:n/` / medium | Dictionary `tahar`; `geed` versus `good` are different heads; `go'aansasho` exists, `go'aansho` is source-only here; file `08` discusses final `/t/` `/k/` without rewriting these examples | Retain every stop subsection and every listed example. Leave the triple *tahar* set, *good* `/ge:d/`, and the glottal IPA mismatches unresolved. Do not invent replacement words or IPA. Link `08` for final-position analysis, not as a license to add rows. | `repository-supported`; `unresolved`; `intentional-retained` |
| PH03-R003 | 86-107 | Affricate sections exist, but `/ʤ/` examples are *dh*-words and `/tʃ/` examples are *j*-words; `facarada` conflicts with same-file `caarada` / high | SLS letters `j` and `dh` are distinct; dictionary `dhiig`, `dhaqaaq`; same file lines 41, 177, 202 `caarada`; file `07` later equates `/c/` with `/ʤ/`, which does not uniquely repair these examples | Retain both affricate subsections as this source's labels. Correct `facarada` to `caarada`. Do not swap the example sets or import a `/ɖ/` row. Leave the *dh*-under-`/ʤ/` and *j*-under-`/tʃ/` pairings unresolved. | `repository-supported`; `form-correction`; `unresolved`; `intentional-retained` |
| PH03-R004 | 109-159 | Fricative set is usable; `/ħ/` examples are orthographic *x*-words, which matches Somali *x*; the `/ħ/` paragraph says `/x/` is voiced while the `/x/` heading says `codlaawe`; *buuha* is not an independent `/ʃ/` match / medium | Same-file `/x/` heading versus `/ħ/` contrast; dictionary `shaah`, `shuqul`, `xusuus`, `xoolo`; `buushe` is not a unique repair for *buuha* | Retain all six fricative subsections (and the `/r/` mention in the opening count). Leave the `/x/` voicing contradiction and *buuha* unresolved. Do not merge `/x/` and `/ħ/` or respell *xoolo*. | `unresolved`; `intentional-retained` |
| PH03-R005 | 161-229 | Nasal, lateral, trill, and approximant subsections are complete and internally consistent / low | Dictionary supports *marag*, *naas*, *laab*, *rag*, *waran*, *yidhi* | Retain all four manner blocks and their examples. Do not add `/ɲ/` or extra approximants. | `repository-supported`; `intentional-retained` |
| PH03-R006 | 231-256 | Geminate table is the useful contrast set; `xabbbad` is triple-letter OCR of same-row `xabbad`; *hadhay* `/hadai/` versus *hadday* is a source transcription note, not an SLS spelling rule / high | Same table IPA `/xabbad/` and gloss `xabbad`; dictionary `xabbad`; orthography collections must not be rewritten from this note | Correct `xabbbad` to `xabbad`. Retain the four-row table, *alleel*, and the *hadhay* / *hadday* note as this source's transcription remark. Do not turn it into a normative gemination rule. | `repository-supported`; `form-correction`; `intentional-retained` |
| PH03-R007 | 258-291 | Closing table lists 21 numbered consonants while the prose says 22; SLS `dh`, `kh`, and `c` are absent as separate rows / high | Cleaned `naxwe/01` 21 letters + glottal; this table already includes `/ʔ/` and splits `/ʤ/` `/tʃ/` | Retain the 21-row table as this source's summary. Note in prose that the source says 22 and that SLS letter identity lives in `naxwe/01` / SLS-0001. Do not add `dh`, `kh`, or `c` rows by inference. Keep the place-of-articulation gloss list. | `repository-supported`; `scope-correction`; `unresolved`; `intentional-retained` |
| PH03-R008 | whole file | Already SLS-native and manner-complete; source IPA must not be modernized / medium | Nine H2 sections, 21 H3 phonemes, two tables, no OCR page markers | Preserve the seven-class sequence and every subsection. Change only the approved form, scope, and link repairs. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Shibbanayaasha** and keep: seven
manner classes; every current phoneme subsection; geminate table; 21-row
summary. Links go to `02`, `08`, `naxwe/01`, and `07`. No phoneme is added or
removed. Unmatched IPA examples stay unresolved.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** PH03-R001 through PH03-R008
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Audit validation

- Resource SHA-256 after audit:
  `99abfd4b2806aadeb18ffcdbd46421ba453a46e6e74a8a779720067b2251445d`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, nine H2 headings, twenty-one H3 phoneme
  subsections, two tables, no OCR page markers.

## Cleanup result and review

### Applied cleanup

- Linked the seven-class opening to `02-xubnaha-hadalka.md`.
- Linked final-stop analysis to `08-gariirka-iyo-glotis-furan.md`.
- Corrected `facarada` to `caarada` and `xabbbad` to `xabbad`.
- Noted that the source says 22 consonants while the summary table has 21
  rows, and linked `naxwe/01` and SLS-0001 for letter identity.

### Deliberately retained

- The triple *tahar* set, *good* `/ge:d/`, and glottal IPA mismatches.
- `/ʤ/` *dh*-word examples and `/tʃ/` *j*-word examples.
- The 21-row table with no added `dh`, `kh`, or `c` rows.
- *buuha* and the `/x/` voicing contradiction.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, nine H2 headings, twenty-one H3 phoneme subsections,
  two tables.
- Pre-cleanup forms `facarada` and `xabbbad` are absent.
- Cleaned file size: 299 lines; 1,612 words; 10,390 bytes.
- Cleaned SHA-256:
  `eac86b9e55a24740f721c1bfc925792816aaa16f2315337162a5e1957e5856e4`.
