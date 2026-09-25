# Source-batch decisions — originals remain preserved

- Authorization: user extended source-only work through Phases 4–6 and chose both
  PDF profiles, prioritizing optional 4.B; default remains 4.A.
- AD-15 implementation boundary: standalone-image OCR works without PDFium, but
  PDF raster OCR is feature-gated. Default PDFs retain placeholders and explain the
  required build. This companion record preserves the supplied architecture file.
- Release panic conflict: original optimization `panic=abort` defeats required
  `catch_unwind`. Source now uses `panic=unwind` to prioritize the specified decoder
  and per-file panic boundary. Binary-size impact is unmeasured; ≤5 MB gate remains.
  Native faults, OOM aborts and some stack overflows remain outside that boundary.
- Precedence conflict retained: defaults → profile → config file → explicit CLI,
  per architecture §5 and P0-S08. P6-S03's contradictory example is not adopted.
- PDFium ABI API evidence is published documentation, not dependency resolution.
  Dynamic library lifecycle is serialized; other formats still use whole-file parallelism.
- Packaging is a source candidate, not v1.0. CI source requires user-supplied locked
  dependencies, toolchain/native provenance before it can establish any evidence.
