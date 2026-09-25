# Skill: File-format signatures

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P0-S02 actually need — not a general treatise on the topic.

## Scope

Bounded input detection and extension-mismatch reporting for P0-S02. Not full file validation, antivirus scanning or proof that a parser will succeed.

Blueprint/architecture aliases: `file-format signatures`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Read a bounded prefix, then distinguish signature confidence from format validation. Record detected content and the original extension independently.

2. For ZIP candidates, inspect the archive directory and package markers: content types/relationships for OOXML, mimetype/container.xml for EPUB, and ODF mimetype/manifest. A PK prefix alone is not a DOCX identifier; an empty ZIP has a different signature from an ordinary local file header.

3. CFB/OLE2 is a container, not a DOC signature. Preserve the distinction between supported BIFF XLS, unsupported legacy Word/PowerPoint, and encrypted Office packages. Validate container streams when implementing stronger detection; escalate any broader format support decision.

4. Treat a raw /Encrypt occurrence as a conservative suspicion only: PDF names can occur in strings/comments. The existing raw scan is a documented false-positive risk, not a complete PDF encryption parser.

5. Return an exact controlled error for empty/truncated inputs. Do not let the extension override a contradictory recognized image signature.

## Not This

Do not identify every ZIP as OOXML, every OLE2 as DOC, or every printable prefix as trustworthy UTF-8 text.

## Common Mistakes

Confusing a container signature with its payload type; scanning only the start of a PDF for trailer metadata; calling a header-only fixture a valid complete image.

## Verification Checklist

- [ ] PNG renamed .docx produces a mismatch report.
- [ ] Empty/truncated ZIP and PDF have defined errors.
- [ ] Legacy XLS is not accidentally covered by the legacy-DOC rejection test.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) — zip. Accessed 2026-09-16; retrieved. Evidence key: `zip`.
- [2](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-cfb/53989ce4-7b05-4f8d-829b-d08d6148375b) — [MS-CFB]: Compound File Binary File Format | Microsoft Learn. Accessed 2026-09-16; retrieved. Evidence key: `cfb`.
- [3](https://docs.rs/lopdf/0.45.0/lopdf/struct.Document.html) — Document in lopdf - Rust. Accessed 2026-09-16; retrieved. Evidence key: `lopdf`.
- [4](https://www.w3.org/TR/png-3/) — Portable Network Graphics (PNG) Specification (Third Edition). Accessed 2026-09-16; retrieved. Evidence key: `png`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
