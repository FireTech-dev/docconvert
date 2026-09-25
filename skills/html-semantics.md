# Skill: HTML/XHTML semantics

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P0-S06, P1-S03 actually need — not a general treatise on the topic.

## Scope

P0-S06 and EPUB semantic extraction into CDM, not browser rendering, CSS layout or HTML sanitization for web publication.

Blueprint/architecture aliases: `HTML semantics`, `HTML`, `HTML/XHTML semantics`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Map heading/list/table/quote/pre/code/link/figure elements to semantic CDM structures; preserve inline order and meaningful whitespace inside pre/code. Associate figcaption with its figure rather than duplicating its text.

2. Keep HTML and XML parsing modes distinct. HTML has void elements, implied end tags, raw-text handling and a specified recovery algorithm; lowercasing tags and adding closing slashes is not a complete implementation of that algorithm. Under the approved lightweight parser constraint, label recovery as partial and preserve unsupported content.

3. Skip script/style and approved navigation/interface content with explicit counted warnings. Do not classify every aside or repeated paragraph as navigation.

4. Use the injected asset loader: it resolves approved local resources or package members; the semantic extractor must not fetch URLs. Preserve remote link targets according to the output policy, without executing them.

5. Decode entities in the correct context once. Match XML expanded names in XHTML, not arbitrary prefix strings. Rendered Markdown containing links/raw HTML is not automatically safe HTML.

## Not This

Do not execute scripts, fetch CSS/resources, collapse pre whitespace, globally decode entities repeatedly, or claim browser-equivalent recovery from quick-xml.

## Common Mistakes

Footnotes lost by dropping every aside; figure caption emitted twice; local paths opened directly from an img src; HTML boolean attributes treated as XML syntax without a documented recovery path.

## Verification Checklist

- [ ] Fixtures exercise void/unclosed/uppercase HTML and namespace-qualified XHTML.
- [ ] No loader network access occurs; traversal targets are rejected.
- [ ] Inline images, captions, lists and preformatted content preserve order.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://html.spec.whatwg.org/multipage/parsing.html) — HTML Standard. Accessed 2026-09-16; retrieved. Evidence key: `html`.
- [2](https://www.w3.org/TR/xml/) — Extensible Markup Language (XML) 1.0 (Fifth Edition). Accessed 2026-09-16; retrieved. Evidence key: `xml`.
- [3](https://www.w3.org/TR/epub-33/) — EPUB 3.3. Accessed 2026-09-16; retrieved. Evidence key: `epub`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
