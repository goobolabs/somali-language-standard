# Audit record — Qiso Kalgacal

- **Resource path:** `resources/suugaan/08-qiso-kalgacal.md`
- **Collection / family:** suugaan / sheeko-curis jacayl
- **Priority:** P1
- **Method:** whole-file literary-content audit; direct comparison with the
  local source scan and its raw OCR conversion
- **Audit status:** approved; source-guided cleanup applied, awaiting maintainer review
- **Audit date:** 2026-08-14
- **File size at audit:** 3,592 lines; 18,304 words; 113,926 bytes
- **Resource SHA-256:**
  `bbdc603f0e20f2e8d243ece26aaf958c0b086d2597055595759b09ff62543ffa`
- **Resource-text changes during audit:** none

## Target output model

The cleaned file must remain a complete, source-faithful edition of M.A.
Gurhan's single first-person novella. It follows Mahad Nuur's meeting with
Maryan Buwe, their developing relationship, jealousy and misunderstandings,
family opposition, marriage, the birth of their son Siciid, Maryan's illness
and death, and her final letter and three-part gift.

Cleanup may restore the omitted authorial front matter, readable paragraphs,
dialogue, letter layout, list structure, source spelling, and words divided
only by printed line wrapping. It may remove verified page numbers, scan
symbols, and publication furniture. It must not shorten the narrative,
rewrite its relationships or moral argument, sanitize dated conduct, invent
chapters, modernize the author's voice, or infer illegible words from plot
alone.

## Controlling source and metadata finding

The controlling source is the local image-only PDF:

`temp/soomaali/literature/Qiso kalgacal.pdf`

The scan has 93 single-page images and no text layer. Unlike several earlier
resources, it does not contain facing-page spreads, so its prose is in the
correct page sequence. Its source layout is:

| PDF pages | Source matter |
| ---: | --- |
| 1 | Illustrated cover: *Qiso Kalgacal*, M.A. Gurhan, `Sheeko curis ah` |
| 2 | Library/title scan furniture |
| 3 | Other books, copyright statement, and edition notice |
| 4 | `Mahadnaq`, author signature, and fiction disclaimer |
| 5 | Five-line epigraph attributed to C.D. Soraan |
| 6-91 | Complete novella, printed pages 1-86 |
| 92 | Promotional synopsis for *Walaashay Shukri* |
| 93 | Back-cover matter repeating acknowledgements |

The publication page distinguishes two dates: `Qiso KALGACAL, M.A. Gurhan,
1983, Muqdisho`, followed by `Daabacaaddii 1aad, 1985, Muqdisho`. The active
source registry currently records 1983. For a publication-year field, 1985 is
the directly supported first-edition date; 1983 should be retained in
provenance as the earlier work/copyright date rather than lost. The cover uses
`M.A. Gurhan`, while the acknowledgement is signed `Mike A. Gurhan (Qoraha)`.
No author-name change is proposed.

The two raw OCR conversions at
`temp/soomaali/md/literature/Qiso kalgacal.md` and
`temp/md/literature/Qiso kalgacal.md` are byte-identical. They are navigation
aids only; the page images control every correction.

## Source contents and structure

This is one continuous narrative, not a collection. The source begins its
story with the standalone Roman numeral `I`, but no later Roman-numbered or
titled chapter boundary appears. The current Markdown correctly retains `I`.
Cleanup must not manufacture further chapters merely to make a long file look
more structured.

The source has three meaningful prefatory units:

1. `Mahadnaq`, thanking the people and organizations involved in typing and
   producing the book;
2. the author's notice that the story and its names are invented and do not
   refer directly or indirectly to real persons; and
3. the five-line `Kalgacaylku waa qani` epigraph, attributed to C.D. Soraan.

All three are absent from the current resource. They are authorial context,
not merely cover furniture, and should be restored ahead of `I`. The list of
other books, rights notice, repeated back-cover thanks, library marks, and
promotional synopsis of another book need not enter the reader-facing text.

## Current structural and textual condition

The Markdown contains one H1 and no Markdown subheadings. Its standalone `I`
is plain text, matching the source's only internal division. The continuous
narrative order is substantially intact, and the final `Dhammaad.` survives.
However, the file remains a raw-looking transcription rather than readable
literary Markdown.

The present resource is shorter than the raw OCR because an earlier mechanical
pass removed converter front matter, source page wrappers, prefatory pages,
the final advertisement, and many line-wrap divisions. That pass did not
perform reliable source transcription. It left hundreds of damaged names and
ordinary words, including recurring `Maryan`/`Mahad` forms read with `l`, `i`,
or punctuation; `oo` read as `00`; letters read as digits; false symbols;
fused page numbers; and hyphen or equals signs inherited from typewritten line
breaks.

The scan also includes four uncaptioned full-page drawings at printed pages
20, 41, 63, and 84. Their omission from a text-only Markdown transcription is not
missing prose. Cleanup should document them in provenance but must not invent
captions or prose descriptions and present those as the author's words.

## Audit coverage and findings

| ID | Original lines / source pages | Class / severity | Source evidence and finding | Proposed action |
| --- | --- | --- | --- | --- |
| SUU08-R001 | lines 1-3,592; PDF 1-93 | identity, hierarchy, and metadata / fatal | Direct cover, edition-page, and whole-scan inspection confirms one 86-page novella with only division `I`, plus authorial front matter and non-story back matter. The registry's 1983 conflicts with the explicit 1985 first-edition notice. | Retain one H1 and source division `I`; do not create chapters. Update the registry year to 1985 during approved cleanup and record 1983 separately as earlier source evidence. Exclude verified cover, library, rights, advertising, and repeated back-cover furniture. |
| SUU08-R002 | omitted before line 3; PDF 4-5 | acknowledgements, disclaimer, and epigraph / high | The current file starts directly at `I`, omitting the complete `Mahadnaq`, the author's fiction disclaimer, and C.D. Soraan epigraph. These pages are legible and substantively frame the work. | Restore all three units from the scan before `I`, using restrained headings and verse layout. Verify every personal and organizational name; preserve the attribution exactly. |
| SUU08-R003 | lines 3-493; printed pp. 1-12 | opening meeting and recognition / fatal | Mahad meets a young woman near Afar-Irdood, struggles to identify her, learns she is Maryan Buwe and connected to Caasho and Yuusuf, then becomes preoccupied with finding her. Narrative order survives, but pervasive substitutions corrupt names, places, dialogue, punctuation, and paragraph boundaries; printed page numbers remain embedded. | Reconstruct all opening prose and dialogue page by page. Preserve the narrator's retrospective voice, parenthetical asides, social setting, named cinemas and streets, and the father's conversation. Remove only verified page numbers and line-wrap marks. |
| SUU08-R004 | lines 498-1,174; printed pp. 13-28 | first calls, meetings, and early misunderstanding / fatal | Mahad obtains contact with Maryan, telephones her, meets her at Beerta Arbacarukun, declares his feelings, misreads her response, visits Caasho's home, and receives Maryan's letter and photographs. The OCR contains a wholly meaningless symbol block where printed page 20 is an illustration, plus many letter/digit substitutions and fused page transitions. | Restore continuous source paragraphs, speaker turns, Maryan's written message, and all names and locations. Omit the uncaptioned illustration from the text body while documenting it; delete only its OCR symbols, not adjacent prose. |
| SUU08-R005 | lines 1,175-1,949; printed pp. 29-46 | party, jealousy, and deepening relationship / fatal | Maryan joins Mahad, Caasho, and Yuusuf; a birthday gathering and dancing provoke jealousy; Mahad attempts to take Maryan to Guriga Shabeelle; their repeated meetings expose his earlier attitudes and her objections. A second uncaptioned illustration replaces printed page 41. The OCR damages dialogue, character names, place names, tone, and printed-page joins throughout. | Reconstruct the entire sequence directly from pages 29-40 and 42-46. Preserve uncomfortable or dated gender and relationship conduct as source narration without endorsement or sanitization. Remove the page-41 picture's symbol debris without fabricating a textual substitute. |
| SUU08-R006 | lines 1,950-2,579; printed pp. 42-65 | rumours, separation, and reconciliation / fatal | The relationship continues for six months; Caasho's actions and rumours cause conflict; Mahad investigates with Yuusuf; Maryan is away for twenty days and later explains herself. Printed page 63 is another uncaptioned drawing. Names are repeatedly misread, `oo` becomes `00`, and dialogue and letters are visually fragmented. | Verify the overlapping narrative span page by page, restoring every conversation, letter, interval, and causal transition. Preserve repetitions and emotional language. Treat the illustration as non-textual evidence and remove only its OCR residue. |
| SUU08-R007 | lines 2,580-3,229; printed pp. 64-81 | Qoryooley, reunion, and family opposition / fatal | Mahad begins work at the Somali Commercial and Savings Bank's Qoryooley branch on 10 August 1980; he and Maryan exchange letters, reunite, decide to marry, and face refusal from both families. Their friends help them consider what to do. Dates, institutions, names, and speaker turns are textually important but heavily damaged. | Reconstruct prose, correspondence, dates, institutional names, and dialogue from the scan. Preserve the source's political/social setting and the narrator's decisions; do not silently revise them into contemporary advice. |
| SUU08-R008 | lines 3,230-3,592; printed pp. 82-86 | marriage, birth, death, and closing letter / fatal | Mahad and Maryan establish their household; she becomes pregnant, falls seriously ill after returning from Muqdisho, gives birth to Siciid, and dies in November 1981. The ending includes her address to Mahad, an enumerated three-part gift, the revealed objects, a moral reflection, and `Dhammaad.` OCR damage affects medically and emotionally central sentences, the date, the letter, and list numbering. | Restore the complete ending without compression. Format Maryan's letter as a source letter and retain its salutation, signature, numbered list, religious language, gifts, closing reflection, and `Dhammaad.` Verify rather than regularize ambiguous wording. |
| SUU08-R009 | PDF 92-93 | other-work promotion and back-cover matter / structural | The two final scan pages are outside the novella: a synopsis for *Walaashay Shukri* and repeated acknowledgement/back-cover material. They are absent from the current resource. | Keep them out of the literary body. Record their existence here as source provenance; do not treat their omission as a missing ending. |

## Cross-file and technical findings

1. The source scan is single-page and sequential. Cleanup does not require the
   facing-page splitting used for `07-hal-ka-haleel.md`.
2. Git history contains no cleaner committed transcription of this resource.
3. The baseline contains 280 lines matching high-risk OCR indicators such as
   line-final `~`/`=`, `00` for `oo`, or obvious false symbols; 27 short
   nonblank lines of four or fewer characters; and one line longer than 160
   characters. These counts identify review risk and are not automatic repair
   rules.
4. The earlier mechanical pass joined some typewritten wrap-hyphens correctly
   and others incorrectly. Every join must be confirmed against the image and
   Somali syntax; it cannot be accepted merely because it forms a plausible
   word.
5. Dialogue quotation marks alternate between straight, curly, doubled, and
   missing marks in the OCR. Source speaker sequence must be established before
   punctuation is normalized.
6. Names and place forms—including Mahad Nuur, Maryan Buwe, Yuusuf, Caasho,
   Qoryooley, Afar-Irdood, and source-era Muqdisho locations—must be verified
   from repeated clear instances and the corresponding scan, not globally
   replaced without context.
7. The book's jealousy, family conflict, romantic and sexual implications,
   gendered claims, illness, death, and religious counsel are literary source
   content. Cleanup preserves them without presenting them as current SLS
   guidance or factual endorsement.
8. Other repository resources may corroborate ordinary Somali spellings, but
   they cannot replace this edition's wording, narrative voice, or historical
   orthography.

## Proposed source-faithful structure

```text
# Qiso Kalgacal

## Mahadnaq
[acknowledgement]

[fiction disclaimer]

## Gogoldhig
[the source epigraph, retaining C.D. Soraan attribution]

## I
[complete continuous novella]

[Maryan's closing letter and numbered gift list remain inside I]

Dhammaad.
```

`Gogoldhig` is only a presentation label for the otherwise untitled epigraph.
If the maintainer prefers not to add an editorial label, the verse may instead
appear directly between the disclaimer and `I`. No additional narrative
headings should be introduced.

## Cleanup gates

Cleanup must:

- inspect PDF pages 4-91 directly and preserve all substantive authorial text;
- restore `Mahadnaq`, the fiction disclaimer, and the attributed epigraph;
- retain the complete continuous novella in printed-page order, including all
  narration, dialogue, parenthetical asides, letters, dates, names, lists,
  moral reflection, and the final `Dhammaad.`;
- correct OCR substitutions and joins only from page evidence, using repeated
  clear source forms as corroboration;
- retain source spelling and voice unless the page proves a recognition error;
- update the registry publication year to 1985 while preserving the distinct
  1983 evidence in the audit/provenance log;
- remove only verified page numbers, scan symbols, cover/library/copyright
  furniture, the other-book advertisement, repeated back-cover matter, and
  OCR debris from the three uncaptioned illustrations; and
- record the front-matter restoration, major reconstruction ranges, date
  decision, and illustration handling in the provenance log.

Cleanup must not:

- summarize the novella or remove repeated emotional and conversational
  passages for concision;
- invent a table of contents, chapter titles, or divisions after `I`;
- rewrite dated or troubling conduct into modern commentary;
- infer damaged words from narrative expectation without scan support;
- add descriptions of the three drawings as though the author supplied them;
- include the *Walaashay Shukri* promotional synopsis as part of this story;
  or
- mark cleanup approved or complete without maintainer review.

## Approval state

- **Audit approval:** approved by the maintainer on 2026-08-14 with the
  instruction, "go ahead."
- **Finding IDs:** SUU08-R001 through SUU08-R009
- **Cleanup:** applied on 2026-08-14
- **Cleanup approval:** pending
- **Complete:** no

## Cleanup result

The approved cleanup retained the complete continuous novella and applied a
source-guided repair rather than replacing it with a shortened retelling. The
existing transcription supplied the fuller narrative body; a fresh
single-page OCR pass and direct page inspection controlled recurring names,
dates, page transitions, the four illustration-only pages, the closing letter,
and the author's note.

The cleaned resource now contains 2,349 lines, 17,714 words, and 112,282 bytes.
Its SHA-256 is
`b10b086a54e22a86f186db573bbee7888b43eb9e6d6509b784f673264eb14d15`.

Applied changes include:

- restoration of `Mahadnaq`, Mike A. Gurhan's fiction disclaimer, and the
  five-line C.D. Soraan epigraph;
- promotion of the source's sole narrative division, `I`, without inventing
  later chapters;
- preservation of the complete Mahad–Maryan narrative, including dialogue,
  letters, family conflict, marriage, Siciid's birth, Maryan's death, and the
  closing moral reflection;
- source-guided correction of recurring Mahad/Maryan damage, `00`-for-`oo`,
  word-wrap joins, institutional names, the 10 Ogoosto 1980 employment date,
  and the Nofeembar 1981 death date;
- reconstruction and Markdown formatting of Maryan's final letter, both
  three-item lists, signature, religious counsel, and `Dhammaad.`;
- restoration of the author's postscript explaining that he conceived the
  story in June 1983 and wrote it in July 1983;
- correction of the source registry from the composition year 1983 to the
  explicit first-edition year 1985; and
- removal of printed page numbers, OCR symbol fields from the four uncaptioned
  illustrations, cover/library/copyright furniture, the unrelated
  *Walaashay Shukri* advertisement, and repeated back-cover matter.

The cleanup does not claim a new critical edition: ambiguous character-level
readings that could not be safely resolved from the scan were retained rather
than silently rewritten. Cleanup remains awaiting maintainer approval and is
not marked complete.
