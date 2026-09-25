# Skill: RTF control-word grammar

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P1-S05 actually need — not a general treatise on the topic.

## Scope

P1-S05 byte-oriented RTF tokenization, group state, Unicode fallbacks, fields and pictures. Not a Word-compatible layout engine or OLE object executor.

Blueprint/architecture aliases: `RTF control-word grammar`, `RTF`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Use RTF 1.9.1 control-word rules: ASCII letters are case-sensitive and may include uppercase; the older 1.5 mirror’s lowercase-only wording is superseded. Limit names to 32 letters and parse the supported signed numeric range with overflow checks. Tokenize original bytes into group delimiters, control words with optional signed numeric parameters, control symbols, plain bytes and binary payloads. A delimiter space ends a control word and is consumed; a non-space delimiter generally still needs processing.

2. Maintain a bounded stack for formatting, destination, code page/font selection and uc fallback count. Restore state on group exit; skipping a destination must still maintain structural balance.

3. For uN, interpret the signed UTF-16 code unit correctly, combine valid surrogate pairs, and skip the prescribed ANSI fallback units according to RTF rules, not Unicode character count. Escaped/control constructs affect fallback counting; do not skip arbitrary UTF-8 bytes.

4. For binN consume exactly N raw bytes without interpreting braces/backslashes. Reject negative/oversized/truncated lengths. For hex pict data decode bounded pairs and finalize the image at its destination boundary; preserve unknown picture types without inventing raster data.

5. Keep fldinst instructions separate from fldrslt displayed content. Execute neither fields nor embedded objects. Unknown ignorable destinations must not leak into body text; unsupported non-ignorable content needs visible fallback/warning.

## Not This

Do not parse RTF as UTF-8 XML, count binary braces as groups, skip fallback by characters indiscriminately, or treat cp1252 as the universal font encoding.

## Common Mistakes

Group formatting leaking to following text; a picture finalized twice; nested destination output leaked; surrogate halves emitted as invalid Unicode.

## Verification Checklist

- [ ] Fixtures cover signed Unicode/surrogates, scoped uc, non-default code pages and escaped delimiters.
- [ ] Binary payload containing braces does not alter the stack.
- [ ] Field display text, nested pictures and malformed/unterminated groups are checked.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [5](https://officeprotocoldoc.z19.web.core.windows.net/files/Archive_References/[MSFT-RTF].pdf) — Microsoft Rich Text Format Specification 1.9.1 (2008). Accessed 2026-09-16; search excerpt only. Evidence key: `rtf`.
- [2](https://www.biblioscape.com/rtf15_spec.htm) — Microsoft RTF 1.5 (1997), Biblioscape mirror: stable grammar corroboration only. Accessed 2026-09-16; retrieved partial. Evidence key: `rtf15`.
- [3](https://docs.rs/logos/latest/logos/) — logos - Rust. Accessed 2026-09-16; retrieved. Evidence key: `logos`.
- [4](https://www.unicode.org/reports/tr29/tr29-47.html) — UAX #29: Unicode Text Segmentation. Accessed 2026-09-16; retrieved. Evidence key: `unicode17`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
