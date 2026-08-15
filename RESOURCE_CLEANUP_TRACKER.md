# Resource Cleanup Tracker

This is the live, file-by-file queue for `resources/`. Work each row in order:

1. **Audit** — complete the whole-file audit and structural blueprint; no resource-text edits.
2. **Audit approval** — maintainer approves the audit scope and proposed structure.
3. **Cleanup** — apply only the approved cleanup and record evidence.
4. **Cleanup approval** — maintainer verifies the cleaned file.
5. **Complete** — mark only after both approvals and validation pass.

Use `[x]` only for completed stages. `P0` is the active starting file; work then proceeds through P1, P2, and P3 in order.

Current work: all 21 naxwe tracker rows have approved audits and applied
cleanups awaiting maintainer cleanup review: the source registry, content files
00 through 17, `README.md`, and the Somali–English grammar terminology
reference `ereyfur.md`. A primary-PDF source pass subsequently corrected the
1999 provenance, restored source-backed structure in files 07 and 09, and added
Somali explanations to all 268 glossary terms; these remain within the same
cleanup-review gate. None is marked complete. The records are in
`docs/resource-cleanup/file-reviews/naxwe/`.

| Priority | Resource file | Audit | Audit approval | Cleanup | Cleanup approval | Complete |
| --- | --- | --- | --- | --- | --- | --- |
| P3 | `resources/README.md` | [ ] | [ ] | [ ] | [ ] | [ ] |

## Erey-bixin

| Priority | Resource file | Audit | Audit approval | Cleanup | Cleanup approval | Complete |
| --- | --- | --- | --- | --- | --- | --- |
| P3 | `resources/erey-bixin/00-sources.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/erey-bixin/01-bayoolaji.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/erey-bixin/02-fisikis.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/erey-bixin/03-juqraafi.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P1 | `resources/erey-bixin/04-kimistari.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/erey-bixin/05-xisaab.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/erey-bixin/06-wasaaradaha.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/erey-bixin/07-barbaarinta-jirka.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/erey-bixin/08-magacyada-dhirta.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P1 | `resources/erey-bixin/09-farsamada-culuunta.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P3 | `resources/erey-bixin/README.md` | [ ] | [ ] | [ ] | [ ] | [ ] |

## Morphology

| Priority | Resource file | Audit | Audit approval | Cleanup | Cleanup approval | Complete |
| --- | --- | --- | --- | --- | --- | --- |
| P3 | `resources/morphology/00-sources.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/morphology/01-magacyada.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/morphology/02-falalka.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/morphology/03-dhismaha-ereyga.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/morphology/04-isbeddelka-codka.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P3 | `resources/morphology/README.md` | [ ] | [ ] | [ ] | [ ] | [ ] |

## Naxwe

| Priority | Resource file | Audit | Audit approval | Cleanup | Cleanup approval | Complete |
| --- | --- | --- | --- | --- | --- | --- |
| P2 | `resources/naxwe/00-luqadda-iyo-fekerka.md` | [x] | [x] | [x] | [ ] | [ ] |
| P3 | `resources/naxwe/00-sources.md` | [x] | [x] | [x] | [ ] | [ ] |
| P2 | `resources/naxwe/01-ereyada.md` | [x] | [x] | [x] | [ ] | [ ] |
| P2 | `resources/naxwe/02-sarfaha-magacyada.md` | [x] | [x] | [x] | [ ] | [ ] |
| P2 | `resources/naxwe/03-sarfaha-tifaftireyaasha.md` | [x] | [x] | [x] | [ ] | [ ] |
| P2 | `resources/naxwe/04-sarfaha-magacuyaallada.md` | [x] | [x] | [x] | [ ] | [ ] |
| P2 | `resources/naxwe/05-sarfaha-tirada.md` | [x] | [x] | [x] | [ ] | [ ] |
| P2 | `resources/naxwe/06-sarfaha-iskuxireyaasha.md` | [x] | [x] | [x] | [ ] | [ ] |
| P2 | `resources/naxwe/07-sarfaha-falalka.md` | [x] | [x] | [x] | [ ] | [ ] |
| P2 | `resources/naxwe/08-hogatuska-baradigmaha-falalka.md` | [x] | [x] | [x] | [ ] | [ ] |
| P2 | `resources/naxwe/09-weer-fudud.md` | [x] | [x] | [x] | [ ] | [ ] |
| P2 | `resources/naxwe/10-dhismaha-oraah-magaceedyada.md` | [x] | [x] | [x] | [ ] | [ ] |
| P2 | `resources/naxwe/11-weerta-adag.md` | [x] | [x] | [x] | [ ] | [ ] |
| P2 | `resources/naxwe/12-noocyada-weeraha.md` | [x] | [x] | [x] | [ ] | [ ] |
| P1 | `resources/naxwe/13-aasaaska-naxwaha.md` | [x] | [x] | [x] | [ ] | [ ] |
| P1 | `resources/naxwe/14-naxwaha-cusub.md` | [x] | [x] | [x] | [ ] | [ ] |
| P0 | `resources/naxwe/15-naxwaha-sifayneed.md` | [x] | [x] | [x] | [ ] | [ ] |
| P1 | `resources/naxwe/16-weeraynta-soomaaliga.md` | [x] | [x] | [x] | [ ] | [ ] |
| P1 | `resources/naxwe/17-naxwaha-af-soomaaliga.md` | [x] | [x] | [x] | [ ] | [ ] |
| P3 | `resources/naxwe/README.md` | [x] | [x] | [x] | [ ] | [ ] |
| P2 | `resources/naxwe/ereyfur.md` | [x] | [x] | [x] | [ ] | [ ] |

## Orthography

| Priority | Resource file | Audit | Audit approval | Cleanup | Cleanup approval | Complete |
| --- | --- | --- | --- | --- | --- | --- |
| P3 | `resources/orthography/00-sources.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/orthography/01-hadal-iyo-qoraal.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/orthography/02-eray-kooban-hadalka.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/orthography/03-kala-qoridda-adag.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/orthography/04-kala-qoridda-lama-qasban.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/orthography/05-astaamaynta.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/orthography/06-xarafka-weyn.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P3 | `resources/orthography/README.md` | [ ] | [ ] | [ ] | [ ] | [ ] |

## Phonology

| Priority | Resource file | Audit | Audit approval | Cleanup | Cleanup approval | Complete |
| --- | --- | --- | --- | --- | --- | --- |
| P3 | `resources/phonology/00-sources.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/phonology/01-hordhac.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/phonology/02-xubnaha-hadalka.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/phonology/03-shibbanayaasha.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/phonology/04-shaqaallada.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/phonology/05-codadka-sare.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/phonology/06-lahjadaha.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/phonology/07-xuruufta-caalamiga.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/phonology/08-gariirka-iyo-spread-glottis.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P3 | `resources/phonology/README.md` | [ ] | [ ] | [ ] | [ ] | [ ] |

## Qaamuus

| Priority | Resource file | Audit | Audit approval | Cleanup | Cleanup approval | Complete |
| --- | --- | --- | --- | --- | --- | --- |
| P2 | `resources/qaamuus/00-abbreviations.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P3 | `resources/qaamuus/00-sources.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/01-b.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/02-t.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/03-j.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/04-x.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/05-kh.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/06-d.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/07-r.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/08-s.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/09-sh.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/10-dh.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/11-c.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/12-g.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/13-f.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/14-q.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/15-k.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/16-l.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/17-m.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/18-n.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/19-w.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/20-h.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/21-y.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/22-a.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/23-e.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/24-i.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/25-o.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/26-u.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/27-aa.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/28-ee.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/29-ii.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/30-oo.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/qaamuus/31-uu.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P3 | `resources/qaamuus/README.md` | [ ] | [ ] | [ ] | [ ] | [ ] |

## Suugaan

| Priority | Resource file | Audit | Audit approval | Cleanup | Cleanup approval | Complete |
| --- | --- | --- | --- | --- | --- | --- |
| P3 | `resources/suugaan/00-sources.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/suugaan/01-maahmaahyada.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/suugaan/02-murtida.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/suugaan/03-xikmad-soomaali.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P1 | `resources/suugaan/04-sheekooyin-soomaaliyeed.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P1 | `resources/suugaan/05-sheekooyin-laysku-soo-ururshay.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/suugaan/06-sheekooyin-fogaan-iyo-dhowaan.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P1 | `resources/suugaan/07-hal-ka-haleel.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P1 | `resources/suugaan/08-qiso-kalgacal.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/suugaan/09-rooxaan.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P1 | `resources/suugaan/10-dhaartii-dhabta-ahayd.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P1 | `resources/suugaan/11-hubsiimo-laan.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P1 | `resources/suugaan/12-bisaddii-bubaysta.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P1 | `resources/suugaan/13-xeebtii-dahabka.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P1 | `resources/suugaan/14-hal-karaan-hadrawi.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P1 | `resources/suugaan/15-dugsiga-fasalka-1aad-1976.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P1 | `resources/suugaan/16-dugsiga-fasalka-1aad-1983.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P1 | `resources/suugaan/17-dugsiga-fasalka-4aad.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P1 | `resources/suugaan/18-dugsiga-fasalka-4aad-buugga.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P1 | `resources/suugaan/19-dugsiga-fasalka-5aad.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P1 | `resources/suugaan/20-suugaanta-carruurta.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P1 | `resources/suugaan/21-suugaanta-dhallaanka.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/suugaan/22-suugaanta-soomaaliyeed.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/suugaan/23-ina-cabdille-xasan.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/suugaan/24-maanso-terminology.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P3 | `resources/suugaan/README.md` | [ ] | [ ] | [ ] | [ ] | [ ] |

## Wordlists

| Priority | Resource file | Audit | Audit approval | Cleanup | Cleanup approval | Complete |
| --- | --- | --- | --- | --- | --- | --- |
| P3 | `resources/wordlists/00-sources.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/01-b.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/02-t.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/03-j.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/04-x.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/05-kh.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/06-d.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/07-r.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/08-s.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/09-sh.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/10-dh.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/11-c.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/12-g.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/13-f.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/14-q.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/15-k.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/16-l.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/17-m.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/18-n.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/19-w.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/20-h.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/21-y.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/22-a.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/23-e.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/24-i.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/25-o.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P2 | `resources/wordlists/26-u.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
| P3 | `resources/wordlists/README.md` | [ ] | [ ] | [ ] | [ ] | [ ] |
