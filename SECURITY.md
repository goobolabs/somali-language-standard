# Security Policy

The Somali Language Standard is primarily a documentation and data project,
but it ships schemas, CI workflows, and (in later phases) validation tooling —
and the integrity of its published data is itself a security surface: a
poisoned dataset or tampered release is a supply-chain risk for every AI
system that consumes SLS.

## Reporting a vulnerability

**Do not open a public issue for security problems.**

Report vulnerabilities privately:

- Email: **sharafdinyusuf@gmail.com** with the subject line `[SLS SECURITY]`
- Or use GitHub's private vulnerability reporting
  ("Security" tab → "Report a vulnerability"), if enabled on this repository.

Please include:

- A description of the issue and its impact
- Steps to reproduce
- Affected files, versions, or releases
- Any suggested remediation

You will receive an acknowledgment within **72 hours** and a status update
within **14 days**.

## What counts as a security issue here

- Vulnerabilities in tooling under `tools/` or in CI workflows
  (`.github/workflows/`)
- Data-integrity attacks: deliberately corrupted or poisoned entries,
  checksum/manifest tampering in release bundles
- Compromise of the release or publication pipeline
- Exposure of contributor personal data beyond what they consented to publish

Ordinary content errors (a wrong definition, a disputed term) are **not**
security issues — report those as normal issues.

## Supported versions

| Version | Supported |
|---|---|
| Latest tagged release | ✅ Security fixes |
| `main` branch | ✅ Actively developed |
| Older tagged releases | ❌ Not patched; upgrade to the latest release |

Release bundles are immutable once published; a compromised release will be
yanked and superseded by a patched version rather than modified in place.

## Responsible disclosure

We ask reporters to:

- Give us reasonable time to investigate and fix before public disclosure
  (we aim for **90 days maximum** from acknowledgment to coordinated
  disclosure)
- Avoid accessing or modifying data beyond what is needed to demonstrate the
  issue
- Act in good faith

In return, we commit to:

- Not pursuing legal action against good-faith security research
- Crediting reporters in the changelog and release notes (unless anonymity is
  requested)
- Publishing a post-fix advisory for any confirmed vulnerability
