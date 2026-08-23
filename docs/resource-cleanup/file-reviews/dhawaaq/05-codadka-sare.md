# Audit record — Codadka sare

- **Resource path:** `resources/dhawaaq/05-codadka-sare.md`
- **Collection / family:** dhawaaq / suprasegmentals
- **Priority:** P2
- **Method:** repository-only, line-by-line SLS content audit
- **Audit status:** approved; SLS-native cleanup applied and awaiting
  maintainer cleanup review
- **Audit started:** 2026-08-19
- **File size at audit start:** 206 lines; 1,146 words; 7,183 bytes
- **Resource SHA-256 at audit start:**
  `fb658ed7bfd888457b517e2ef06064079803261650a8413627471ebadfa0070c`
- **Resource-text changes during this audit:** none

## Target output model

This file is already a compact SLS prosody chapter, not an OCR transcript. It
has one H1, four H2 sections, three H3 subsections, one table, and an existing
qoraal/prosody distinction.

Cleanup should keep stress, pitch, tone, and juncture as this collection's
suprasegmental reference. It should turn the orthography pointer into a real
link, strip the drill list, qualify universals, and link `naxwe/01`–`02` for
Somali tone pairs without copying those tables. It must not reconstruct damaged
juncture IPA or import punctuation marks from `qoraal/05`.

## Source and repository evidence

Mapped to *Codaynta* xoogayn / kicin / astaanayn. `docs/RESOURCES.md` already
splits orthographic versus prosodic *astaamaynta*. Cleaned
`qoraal/05-astaamaynta.md` (O05-R008) now points here. Cleaned `naxwe/01`
§1.2.2 and `naxwe/02` hold `ínan` / `inán` pairs.

## Audit progress

| Original lines | Status | Finding |
| --- | --- | --- |
| 1-31 | reviewed | PH05-R001 |
| 33-70 | reviewed | PH05-R002 |
| 72-116 | reviewed | PH05-R003 |
| 118-145 | reviewed | PH05-R004 |
| 147-206 | reviewed | PH05-R005 |
| whole file | reviewed | PH05-R006 |

## Findings

| ID | Lines | Class / severity | Repository comparison | Proposed SLS action | Evidence label |
| --- | --- | --- | --- | --- | --- |
| PH05-R001 | 1-31 | Accurate four-way split and an existing orthography distinction; the path is a backtick, not a link; “af waliba” / “afafka adduunku” are broader than the repository can establish; `tooggaada` is not an independent verb / medium | O01/O05 scope universals; cleaned `qoraal/05-astaamaynta.md`; N00-R006 | Retain the four numbered classes and the distinction that this chapter is juncture, not punctuation. Convert the orthography path to a Markdown link. Scope the opening to spoken languages this source describes. Leave `tooggaada` unresolved. | `repository-supported`; `scope-correction`; `navigation-update`; `unresolved`; `intentional-retained` |
| PH05-R002 | 33-70 | Stress account and four levels are usable; *Nin baa libaax dilay* is a maintained focus example; `Laylis:` is an exercise list / high | SLS-native naxwe/orthography cleanup excludes exercises; cleaned `naxwe/09` focus particles are a different chapter | Retain the four stress levels, *fure* / *wehel*, and the *Nin baa libaax dilay* pair. Omit the `Laylis:` drill list. Do not invent replacement stress-marked forms. | `repository-supported`; `structural-only`; `intentional-retained` |
| PH05-R003 | 72-116 | Pitch levels and the *Kaalay* / *Ninka ayaa jooga guriga* illustrations are this source's intonation set / low | File `07` repeats the four pitch names | Retain the table, all four *Kaalay* lines, and the four sentence readings. Do not add pitch diacritics the source does not write. | `repository-supported`; `intentional-retained` |
| PH05-R004 | 118-145 | Tone section correctly says Somali has two tones used for gender, but the examples are Arabic and Italian rather than the Somali pairs already maintained in naxwe / medium | Cleaned `naxwe/01` `ínan` / `inán`; `naxwe/02` §2.2.1; `sarfe/01` | Retain the two-tone claim and the Arabic/Italian comparisons as this source's typology. Link `naxwe/01` §1.2.2 and `naxwe/02` for Somali pairs. Do not copy those tables here or invent new accented rows. | `repository-supported`; `navigation-update`; `intentional-retained` |
| PH05-R005 | 147-206 | Five juncture types are the maintained speech/writing boundary; several IPA lines are unreadable (`so:+gi`, `baig`, `gah`) / high | Cleaned `qoraal/05` closing; omit-unreadable-OCR convention | Retain the five numbered types and the readable Somali sentences (*Barihii, Cali, waa kan*; *Ma aragtaa falkii?*). Leave damaged IPA lines unresolved; do not reconstruct them. Keep the closing IPA-versus-orthography sentence and link file `07`. | `repository-supported`; `unresolved`; `intentional-retained` |
| PH05-R006 | whole file | Already SLS-native and correctly split from punctuation; drills and damaged IPA remain / medium | Four H2 sections, existing orthography note, no OCR page markers | Preserve stress / pitch / tone / juncture. Change only the approved scope, omission, and link repairs. | `structural-only`; `repository-supported`; `intentional-retained` |

## Proposed SLS-native blueprint

The cleaned file should remain titled **Codadka sare** and keep: four-class
opening with a live orthography link; stress; pitch; tone with naxwe links;
five juncture types. No punctuation inventory is copied. The drill list is
removed. Damaged IPA stays unresolved rather than guessed.

## Approval gate

- **Audit approval:** approved by the maintainer on 2026-08-19 with the
  instruction, "go ahead."
- **Approved finding IDs:** PH05-R001 through PH05-R006
- **Cleanup:** applied; awaiting maintainer review
- **Cleanup approval:** not started
- **Complete:** no

## Audit validation

- Resource SHA-256 after audit:
  `fb658ed7bfd888457b517e2ef06064079803261650a8413627471ebadfa0070c`
  (unchanged).
- Resource diff during this audit: none.
- Structure checked: one H1, four H2 headings, three H3 subsections, one
  table, one blockquote, no OCR page markers.

## Cleanup result and review

### Applied cleanup

- Scoped the opening to the spoken languages this source describes.
- Converted the orthography path to a Markdown link.
- Omitted the `Laylis:` drill list.
- Linked Somali tone pairs to `naxwe/01` §1.2.2 and `naxwe/02`.
- Linked the closing IPA note to `07-xuruufta-caalamiga.md`.

### Deliberately retained

- `tooggaada`.
- Arabic and Italian tone comparisons.
- Damaged juncture IPA lines (`so:+gi`, `baig`, `gah`).
- The four stress levels and the *Nin baa libaax dilay* pair.

### Cleanup validation

- `git diff --check`: passed.
- Structure: one H1, four H2 headings, three H3 subsections, one table, one
  blockquote.
- Pre-cleanup `Laylis:` is absent.
- Cleaned file size: 210 lines; 1,154 words; 7,374 bytes.
- Cleaned SHA-256:
  `f53f95056b6ce5ca9d42b80694ee0579c8533dedd2cc8a0ac400334a1b1a8cd4`.
