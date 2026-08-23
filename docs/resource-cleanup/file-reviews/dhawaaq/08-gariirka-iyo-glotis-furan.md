# Audit record — Gariirka iyo astaanta spread glottis

- **Resource path:** `resources/dhawaaq/08-gariirka-iyo-glotis-furan.md`
- **Collection / family:** dhawaaq / phonation supplement
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 98 lines; 549 words; 3,739 bytes
- **Resource SHA-256 at audit start:**
  `2c766491c3243c1a6ad7ddaf5c5ba736e2ee2e1bf23794b7c86c1218c3aee598`
- **Resource-text changes during this audit:** none

## Target output model

This file is already a compact SLS supplement, not an OCR of *Codaynta*. It has
one H1, seven H2 sections, one table, and an explicit non-displacement note.

Cleanup should keep it as the Orwin phonation chapter: voice pairs, final-stop
rule, problems with [voice], [+spread glottis] alternative, and the *Codaynta*
relation. It should link `00-sources.md` and files `03`/`naxwe/01`. It must not
import extra Orwin sections, overwrite file `03`'s labels, or present this
analysis as SLS-0600.

## Source and repository evidence

`resources/dhawaaq/00-sources.md` maps this file to Martin Orwin, *Phonation
in Somali phonology* (c. 1990s), supplementary only. Cleaned `naxwe/01`
N01-R011 already delegates the universal `/t/ /k/ /m/` ban here. Cleaned
`sarfe/04` points here for [+spread glottis].

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-13 | reviewed | PH08-R001 |
| 15-41 | reviewed | PH08-R002 |
| 43-85 | reviewed | PH08-R003 |
| 87-98 | reviewed | PH08-R004 |
| whole file | reviewed | PH08-R005 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| PH08-R001 | 1-13 | Clear supplement charter; `03-shibbanayaasha.md` is a backtick, not a link; first-use *phonation* matches the Nilsson/morphology gloss practice / low | Collection `00-sources.md`; file `03` opening | Retain the title, the non-displacement sentence, and *phonation*. Convert the file `03` path to a Markdown link. Link `00-sources.md`. Do not present this file as drafted SLS-0600. | `repository-supported`; `navigation-update`; `intentional-retained` |
| PH08-R002 | 15-41 | Three voice pairs and the final-stop rule are the usable core; the table is narrower than file `03`; `/j/` among voiceless stops is this paper's grouping; Arabic-loan exceptions *xaj* / *taaj* are source examples / medium | Cleaned `naxwe/01` N01-R011; file `03` `/t/` `/k/` `/g/`; dictionary does not uniquely expand the pair table | Retain the three-row table, the `/t/ /k/ /j/` final-stop statement, the *durug* / *durkay* / *durugtay* set, and the Arabic exceptions. Do not add pairs from file `03` or remove `/j/` by SLS-letter analogy. | `repository-supported`; `intentional-retained` |
| PH08-R003 | 43-85 | [voice] problems and [+spread glottis] replacement are this supplement's analysis; `/t/ (s)` in the fricative list is not a unique symbol; Armstrong 1934 is cited, not excerpted / medium | Same-file numbered problems 1–5; `00-sources.md` does not excerpt Armstrong | Retain all five problems, both [±spread glottis] lists, and the Armstrong attributions. Leave `/t/ (s)` unresolved. Do not copy Armstrong's paper into this file. | `unresolved`; `intentional-retained` |
| PH08-R004 | 87-98 | Restated final-stop rule and accurate *Codaynta* relation / low | File `03` remains primary for labels | Retain both closing sections. Do not merge this analysis into file `03`. | `repository-supported`; `intentional-retained` |
| PH08-R005 | whole file | Already SLS-native and correctly marked a supplement / medium | Seven H2 sections, one table, no OCR page markers | Preserve the seven-section sequence. Change only the approved links. Keep English technical terms as first-use labels. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Gariirka iyo astaanta spread glottis**
and keep: supplement charter; voice pairs; final-stop rule; [voice] problems;
[+spread glottis]; *Codaynta* relation. Link `03` and `00-sources.md`. Do not
add Orwin metrics or extra sections.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** PH08-R001 through PH08-R005
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `2c766491c3243c1a6ad7ddaf5c5ba736e2ee2e1bf23794b7c86c1218c3aee598`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, seven H2 headings, one table, no OCR page
  markers.

## Cleanup result and review

### Applied cleanup

- Converted the file `03` paths to Markdown links.
- Linked `00-sources.md`.
- Did not present this file as drafted SLS-0600.

### Deliberately retained

- The three-row voice table and `/j/` among voiceless stops.
- `/t/ (s)` and the Armstrong 1934 attributions.
- The *durug* / *durkay* / *durugtay* set and Arabic-loan exceptions.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, seven H2 headings, one table.
- Cleaned file size: 99 lines; 555 words; 3,846 bytes.
- Cleaned SHA-256:
  `51d16d607d4ef318194ea05296a6a4f73e47a3ef2fb69682d82bf6c7d8d70128`.
