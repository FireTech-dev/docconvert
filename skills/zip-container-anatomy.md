# Skill: ZIP container anatomy

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P1-S01, P1-S02, P1-S03, P1-S04, P2-S01, P2-S02, P2-S04, P3-S01, P3-S02 actually need — not a general treatise on the topic.

## Scope

Architecture §12 package handling reused by DOCX/XLSX/PPTX/EPUB/ODT/ODS. No extracting arbitrary archive trees to disk.

Blueprint/architecture aliases: `ZIP`, `ZIP container anatomy`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Read the central directory through the approved zip crate; do not implement local-header-only enumeration. Account for data descriptors, ZIP64 and supported compression methods through verified library APIs.

2. Before reading members, enforce the project entry-count, declared member-size and aggregate limits. While decompressing, also count actual emitted bytes and stop at the budget: declared compressed/uncompressed sizes are not trustworthy enforcement by themselves.

3. Keep package URI resolution separate from filesystem path joining. Reject root escapes, drive/device paths, absolute paths, NULs and disallowed separator forms; normalize once, then validate the resulting member key. Detect ambiguous duplicate members and normalization/case collisions without silently merging their data.

4. Prefer reading selected members into bounded buffers. Generated output asset names must not be derived from archive paths. If any extraction to disk is introduced, review enclosed_name plus symlink/ancestor checks; enclosed_name is not a full race-proof sandbox.

5. Let the library verify supported checksums/errors on complete member reads. Do not call CRC32 a security/authentication mechanism.

## Not This

Do not trust size declarations, unpack everything for convenience, use a sanitized renamed path as though it were the original relationship target, or let encrypted members fail as unexplained empty text.

## Common Mistakes

Checking only ../ while missing drive prefixes; normalizing twice; treating case-insensitive matching as the ZIP standard; relying on an input-file byte limit against a decompression bomb.

## Verification Checklist

- [ ] Fixtures cover duplicate/colliding names, traversal, CRC failure and oversized expansion.
- [ ] Limits apply during member reads and aggregate expansion.
- [ ] No archive-owned path becomes an unchecked output path.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) — zip. Accessed 2026-09-16; retrieved. Evidence key: `zip`.
- [2](https://docs.rs/zip/latest/zip/read/struct.ZipFile.html) — ZipFile in zip::read - Rust. Accessed 2026-09-16; retrieved. Evidence key: `zipapi`.
- [3](https://cheatsheetseries.owasp.org/cheatsheets/File_Upload_Cheat_Sheet.html) — File Upload - OWASP Cheat Sheet Series. Accessed 2026-09-16; retrieved. Evidence key: `security`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
