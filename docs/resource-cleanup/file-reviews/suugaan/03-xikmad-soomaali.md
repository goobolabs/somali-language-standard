# Audit record — Xikmad Soomaali

- **Resource path:** `resources/suugaan/03-xikmad-soomaali.md`
- **Collection / family:** suugaan / xikmad (wisdom tales)
- **Priority:** P2
- **Method:** whole-file literary-content audit; repository-only (no controlling scan in repo)
- **Audit status:** approved; cleanup approved 2026-08-23; complete
- **Audit date:** 2026-08-19
- **File size at audit start:** 93 lines; **22** tale sections (`##`)
- **Resource SHA-256 at audit start:**
  `66aa59dde9be12a4a04891dad9daec030ce0b156803756085825b8695f5997d0`
- **Resource-text changes during audit:** none

## Target output model

The cleaned file should remain a complete edition of Muuse Xaaji Ismaaciil Galaal's
1956 wisdom tales: one H1, 22 tale headings, and continuous prose under each.
Cleanup may repair scan-proven OCR substitutions and obvious letter damage. It
must not rewrite tale prose, add morals, or insert per-file provenance blocks.

## Findings

| ID | Severity | Finding | Proposed action |
| --- | --- | --- | --- |
| SUU03-R001 | low | H1, 22 `##` tale headings, and tale order are intact | Retain structure |
| SUU03-R002 | medium | Mechanical OCR: `dyul`→`duul`, `waha`/`wah`→`waxaa`/`wax`, `miidkii`→`midkii`, `Wuhuu`→`Wuxuu`, `dyndum*`→`dundum*`, apostrophe debris on `'ol`, `'asho`, `'arruurta`; heading `sqoryeeyay`→`saaryeyay`, `bjilkeyday`→`bijilkeyday` | Apply mechanical fixes only |
| SUU03-R003 | low | Dialect and uncertain forms (`foo`, `hgoyow`, `yeqaan`, `'oonisbaag`, `Waka`) retained without source scan | Do not regularize without scan |
| SUU03-R004 | low | Attribution stays in `00-sources.md`; no collection note in body | No per-file provenance block |

## Approval state

- **Audit approval:** approved by the maintainer on 2026-08-19 with the instruction, "go ahead."
- **Finding IDs:** SUU03-R001 through SUU03-R004
- **Cleanup:** applied on 2026-08-19
- **Cleanup approval:** approved 2026-08-23
- **Complete:** yes

## Cleanup result

Mechanical OCR repairs from SUU03-R002 were applied across all 22 tales. No tale
was shortened, reordered, or rewritten. Dialect forms in SUU03-R003 were preserved.

- **Post-cleanup:** 93 lines; 22 tale sections unchanged
- **Resource SHA-256 after cleanup:**
  `b508691b9db47d88616e0f050ecf4c1cf6178297c05b5d883dcc48408ad76447`
- **Wordlist parity:** n/a
- **Status:** complete; cleanup approved 2026-08-23
