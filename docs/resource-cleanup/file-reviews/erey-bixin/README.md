# Audit record — Erey-bixin README

- **Resource path:** `resources/erey-bixin/README.md`
- **Collection / family:** erey-bixin / collection map and conventions
- **Priority:** P3
- **Method:** repository-only, line-by-line documentation audit
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit started:** 2026-08-19
- **File size at audit start:** 60 lines; 340 words; 2,724 bytes
- **Resource SHA-256 at audit start:**
  `4c47276315cc1d1cd2d6eb1623fcaf79843b1274164afc0a4a99a70aa748cc65`
- **Resource-text changes during this audit:** none

## Target output model

This is a compact collection README, not a glossary. It has one H1, four H2
headings, one fenced layout block listing `00`–`09`, an entry-format example
pair, and an intended-use list. Its file map is complete except that this
README itself is omitted from the fence.

Cleanup should keep the historical-glossary charter and every mapped file. It
should identify `resources/` as descriptive evidence rather than a normative
layer, replace the inert layout fence with a linked table, correct the stale
*Ururinta 2aad* “folded in / duplicate heads dropped” wording against cleaned
`05-xisaab.md` and `00-sources.md`, allow the one-line collection note now in
content files, and stop calling 1985 plant binomials “accepted.” It must not
mark the collection complete.

## Source and repository evidence

All eleven collection files exist. The layout examples `Antibody — Lid-jidh
gale` and `Angle, right — Xagal qumman` still occur in cleaned `01` and `05`.
Cleaned sibling READMEs convert the same fence into a linked table and add
`docs/RESOURCES.md` / `spec/` evidence-versus-norm links.

Repository comparison also included:

- cleaned `00-sources.md`: Ururinta 2aad and Sumadaha are separate blocks;
  duplicate English heads across those blocks are kept;
- cleaned `05-xisaab.md` / EB05: `Deviation` retained in both lists;
- cleaned `01`–`09` openings: one Somali collection note plus a link to
  `00-sources.md`, not author/year provenance blocks;
- cleaned `07`: lowercase Somali retained;
- cleaned `08`: 1985 printed binomials retained (`Acacia`, not modern
  accepted names);
- cleaned `09` / `00-sources.md`: Italian-only and unreadable OCR omitted.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-6 | reviewed | EBRD-R001 |
| 8-24 | reviewed | EBRD-R002 |
| 26-42 | reviewed | EBRD-R003 |
| 44-54 | reviewed | EBRD-R004 |
| 56-60 | reviewed | EBRD-R005 |
| whole file | reviewed | EBRD-R006 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| EBRD-R001 | 1-6 | Useful non-normative charter; it names `data/terminology/` but does not point to `docs/RESOURCES.md` / `spec/` as clearly as cleaned sibling READMEs / medium | `docs/RESOURCES.md`; `docs/ARCHITECTURE.md` (`data/terminology/` is a later governed layer); cleaned sarfe/dhawaaq/orthography READMEs | Retain the bilingual historical-glossary purpose and the statement that this is not a normative terminology standard. Add the descriptive-versus-normative boundary with links to `docs/RESOURCES.md` and `spec/`. Keep `data/terminology/` as a downstream destination, not as a claim that those files already exist. | `repository-supported`; `scope-correction`; `intentional-retained` |
| EBRD-R002 | 8-24 | Layout lists `00`–`09` correctly, but the fence is non-navigable, omits this README, and says Ururinta 2aad terms are “folded in” / high | Eleven collection files; cleaned `00-sources.md` and `05-xisaab.md`; cleaned phonology README linked table | Preserve all ten listed files and add a README row. Convert the fence to a compact Markdown table with local links. Describe `05` as the 1987 A–Z glossary plus separate Ururinta 2aad and symbol-table blocks, not as a folded list. Keep `09` labeled partial. | `repository-supported`; `navigation-update`; `status-correction`; `intentional-retained` |
| EBRD-R003 | 26-42 | Entry-format examples still match cleaned files; “accepted scientific binomial” overclaims 1985 taxonomy; heading types understate 05/07/09 / medium | Cleaned `01`/`05` examples; EB08 retained `Acacia` / `Sansevieria`; cleaned `05` extra H2s; `07` sport headings; `09` two domain H2s | Keep the `Antibody` / `Angle, right` examples. Say plant left-sides are the printed 1985 scientific names (author citations omitted), not “accepted” modern names. Keep letter or domain `##` headings, and note that `05` also has Ururinta/Sumadaha blocks. | `repository-supported`; `scope-correction`; `intentional-retained` |
| EBRD-R004 | 44-54 | “No per-file provenance blocks” is still right for author/year, but content files now have a collection note; the 05 duplicate-head drop claim is stale / high | Cleaned `01`–`09` openings; cleaned `00-sources.md` EB00S-R003; `resources/suugaan/` | Retain UTF-8, filenames, and attribution-once in `00-sources.md`. Allow the one-line collection note in content files. Restate Ururinta 2aad as separate blocks with cross-block duplicate heads kept. Link `naxwe/`, `suugaan/`, and `00-sources.md`. Keep the proverbs/wisdom exclusion. Do not modernize printed coinages; point Italian-only/unreadable omits to `00-sources.md`. | `repository-supported`; `status-correction`; `navigation-update`; `intentional-retained` |
| EBRD-R005 | 56-60 | Intended-use bullets are accurate and do not claim a finished `data/terminology/` layer / low | `docs/ARCHITECTURE.md`; cleaned phonology README current-status section | Retain all three bullets. Add a short current-status note linking `00-sources.md` and `docs/resource-cleanup/file-reviews/erey-bixin/`. Do not hard-code a completion count. | `repository-supported`; `status-correction`; `intentional-retained` |
| EBRD-R006 | whole file | Complete map apart from the missing README row; needs navigation and non-stale 05 wording / medium | Eleven collection files; this audit is the last erey-bixin content/docs file | Preserve one H1 and the charter/format/convention/use sections. Add no mapped glossary. Do not mark the collection complete. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned README should retain:

1. title and historical bilingual-glossary purpose;
2. descriptive-evidence versus normative-specification note;
3. a linked layout table of all eleven files, with `05` and `09` described
   as they now stand;
4. the printed `English — Somali` entry format, including the two examples;
5. 1985 printed (not “accepted”) plant binomials; letter or domain headings;
6. attribution-once plus the allowed collection note; Ururinta 2aad as
   separate blocks; suugaan exclusion; and
7. intended use, without a completeness claim.

No mapped file or charter topic should be removed. Printed 1987/2014 coinages
should not be modernized from this README.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** EBRD-R001 through EBRD-R006
- **Cleanup:** applied and approved 2026-08-23
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Audit validation

- Resource SHA-256 after audit:
  `4c47276315cc1d1cd2d6eb1623fcaf79843b1274164afc0a4a99a70aa748cc65`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, four H2 headings, one fenced layout of `00`–`09`,
  two still-valid example pairs, no OCR page markers.

## Cleanup result and review

### Applied cleanup

- Added the descriptive-evidence versus normative-specification note with
  links to `docs/RESOURCES.md` and `spec/`.
- Converted the layout fence into a linked table of all eleven files.
- Described `05` as the 1987 A–Z glossary plus separate Ururinta 2aad and
  symbol-table blocks; cross-block duplicate English heads are kept.
- Restated plant left-sides as printed 1985 scientific names, not “accepted”
  modern binomials.
- Allowed the one-line collection note; linked `00-sources.md`, `naxwe/`,
  and `suugaan/`.
- Added a current-status note linking the file-review folder.

### Deliberately retained

- The `Antibody` / `Angle, right` examples.
- Letter or domain `##` headings, with `09` still labeled partial.
- `data/terminology/` as a later destination.
- All three intended-use bullets.
- No completeness claim.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, five H2 headings, one linked layout table of eleven
  files.
- Pre-cleanup wording `folded in`, `duplicate English heads are dropped`,
  and `accepted scientific binomial` is absent.
- Cleaned file size: 79 lines; 504 words; 4,078 bytes.
- Cleaned SHA-256:
  `023b724f6919886fe8875ed6c0274d49c1c2ec0c46b19360f3cbc048da171bf9`.
