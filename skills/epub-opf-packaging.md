# Skill: EPUB/OPF packaging

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P1-S03 actually need — not a general treatise on the topic.

## Scope

P1-S03 EPUB container → package → manifest/spine and chapter-local resources/notes. Not a full EPUB reading system, DRM bypass or scripted renderer.

Blueprint/architecture aliases: `EPUB/OPF packaging`, `EPUB`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Locate package documents through META-INF/container.xml rather than guessing an OPF filename. Read publication metadata and resolve manifest href values relative to the package document.

2. Build the manifest ID map, then follow spine itemref IDs in order. ZIP directory order and chapter filenames are not reading order. Treat linear=no as auxiliary content and follow the project inclusion policy; do not drop notes merely because they are auxiliary.

3. Resolve resource references against the current XHTML part. Normalize package URIs with bounds and root-escape checks; split fragments from member paths. Keep a chapter-qualified note identity so repeated fragment IDs across chapters do not collide.

4. Reuse the injected HTML extractor/asset loader in EPUB mode. Preserve note/backlink relationships across chapter files, including references encountered before the target definition.

5. EPUB 3.3 is the researched publication standard, not a blanket support claim. Fixed-layout, media overlays, scripting, remote resources and encrypted content need explicit unsupported behavior under this offline converter.

## Not This

Do not sort chapters lexicographically, open manifest URLs on the network, infer the root package from directory names, or collapse all footnotes named n1 into one note.

## Common Mistakes

Rootfile path resolved relative to META-INF instead of the container root; manifest paths resolved relative to each chapter; nav removal deleting substantive referenced content.

## Verification Checklist

- [ ] Out-of-order filenames still follow the spine.
- [ ] Images and cross-chapter notes resolve against their correct owners.
- [ ] Auxiliary/remote/encrypted/missing resources have explicit warnings or errors.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://www.w3.org/TR/epub-33/) — EPUB 3.3. Accessed 2026-09-16; retrieved. Evidence key: `epub`.
- [2](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) — zip. Accessed 2026-09-16; retrieved. Evidence key: `zip`.
- [3](https://html.spec.whatwg.org/multipage/parsing.html) — HTML Standard. Accessed 2026-09-16; retrieved. Evidence key: `html`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
