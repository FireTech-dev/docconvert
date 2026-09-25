# Skill: FFI and native-library boundaries

> Researched and written 2026-09-16 by the Skill Writer. Scoped to what P4-S04 actually need — not a general treatise on the topic.

## Scope

P4-S04 optional PDFium binding, ownership, ABI, threading and controlled environment errors. Not a new native-process sandbox architecture.

Blueprint/architecture aliases: `FFI`. Project policy and numeric thresholds come from the preserved specifications; external sources below support the technical practice. Current application conformance is not implied.

## Do This

1. Keep all PDFium imports and configuration feature-gated. Document exact Rust binding version, selected PDFium ABI feature, native revision/hash, OS and CPU architecture together.

2. Load only trusted user-configured/system/release-adjacent native libraries in the approved precedence order. Do not fetch one at document-conversion runtime; do not load library paths provided by the PDF.

3. Preserve the ownership chain: Pdfium bindings outlive documents; documents outlive pages/text/objects/bitmaps borrowing them. Copy bounded glyph/bitmap data into owned CDM-facing structures before dropping native handles.

4. pdfium-render documents that PDFium itself is not generally thread-safe and its thread_safe feature serializes access. Do not remove serialization to increase worker throughput or assume Send/Sync means native operations are parallel.

5. Map binding/ABI failure to actionable Config diagnostics and parse failures to controlled document errors. catch_unwind does not catch segmentation faults, access violations or aborts. Native-process isolation is an Architect decision; list it as a gap, not as implemented protection.

6. Budget bitmap dimensions/pixels before native rendering as far as the API permits, and record allocations the native loader can perform before those checks.

## Not This

Do not invent API signatures from another version, retain native references after their parent drops, disable mutex guards, or advertise in-process FFI as crash containment.

## Common Mistakes

Host architecture confused with process architecture; matching a crate release to an arbitrary PDFium binary; cfg! instead of cfg for optional imports.

## Verification Checklist

- [ ] Both builds resolve the intended dependency graph.
- [ ] Known-good native loading and missing/wrong ABI cases have real user-run tests.
- [ ] Lifetime, thread and native-fault limits are explicitly reviewed.
- [ ] Consult the mapped step and resolve conflicts through the Architect; user-only build/run/test restrictions remain in force.

## Sources

- [1](https://docs.rs/pdfium-render/0.9.4/pdfium_render/) — pdfium_render - Rust. Accessed 2026-09-16; retrieved. Evidence key: `pdfium`.
- [2](https://docs.rs/pdfium-render/0.9.4/pdfium_render/prelude/struct.PdfPageTextChar.html) — PdfPageTextChar in pdfium_render::prelude - Rust. Accessed 2026-09-16; retrieved. Evidence key: `pdfglyph`.
- [3](https://doc.rust-lang.org/cargo/reference/features.html) — Features - The Cargo Book. Accessed 2026-09-16; retrieved. Evidence key: `features`.
- [4](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html) — catch_unwind in std::panic - Rust. Accessed 2026-09-16; retrieved. Evidence key: `panic`.

**Freshness:** Specifications are cited at the stated baseline, not asserted to be the newest standard for every input. Recheck moving `latest`/branch URLs and all native/API/security advice when the toolchain, lockfile or platform changes. Search-only and partial retrievals are labeled; no installed-version or runtime verification is claimed. See [research notes](RESEARCH.md) for evidence limits and [open questions](ARCHITECT-QUESTIONS.md) for project conflicts.
