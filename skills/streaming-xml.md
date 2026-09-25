# Skill: Streaming XML

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P0-S06 actually need — not a general treatise on the topic.

## Scope

Bounded quick-xml package/XHTML parsing and namespace/encoding handling. Not a DOM rewrite or schema validation engine.

Blueprint/architecture aliases: `streaming XML`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Use pull events with an explicit bounded context stack. With read_event_into-style APIs, process/copy retained attributes/text before clearing the scratch buffer; the event can borrow it.

2. Match namespace URI plus local name when semantic identity matters. Prefixes are arbitrary and can be rebound; XML names are case-sensitive. NsReader documents namespace-resolved APIs, but verify the exact version before replacing shared helpers.

3. Honor encoding declarations supported by the selected parser configuration. Do not assume that decoding a whole XML member as UTF-8 before parsing handles UTF-16 XML correctly.

4. Reject disallowed DOCTYPE/entity declarations and disable any external resolution at the appropriate boundary. A parser that does not fetch entities is not automatically safe against application-written entity expansion or deeply nested content.

5. Enforce member bytes, normalized bytes, depth, event/attribute counts and any repeated-value expansion. Distinguish truly streaming processing from retaining a tree of all descendants; immutable byte views must not secretly cache a child tree.

6. An unexpected EOF or mismatched close in a strict XML part is a controlled error, not successful partial extraction.

## Not This

Do not compare only local names across unrelated namespaces, lower-case all XML attributes, retain events after buffer reuse, or turn malformed package XML into permissive HTML recovery.

## Common Mistakes

Borrowed events surviving clear(); default namespace assumptions applied to unprefixed attributes; bounded parser input producing unbounded normalized output.

## Verification Checklist

- [ ] Alternate prefixes/default namespaces and UTF-16 declaration fixtures are covered.
- [ ] DOCTYPE/entity, mismatch, excessive-depth and normalized-size cases are bounded.
- [ ] No retained recursive child tree or unbounded event cache exists.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://docs.rs/quick-xml/latest/quick_xml/reader/struct.NsReader.html) — NsReader in quick_xml::reader - Rust. Accessed 2026-09-16; retrieved. Evidence key: `quickxml`.
- [2](https://www.w3.org/TR/xml/) — Extensible Markup Language (XML) 1.0 (Fifth Edition). Accessed 2026-09-16; retrieved. Evidence key: `xml`.
- [3](https://cheatsheetseries.owasp.org/cheatsheets/XML_External_Entity_Prevention_Cheat_Sheet.html) — XML External Entity Prevention - OWASP Cheat Sheet Series. Accessed 2026-09-16; retrieved. Evidence key: `xxe`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
