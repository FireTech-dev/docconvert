# docconvert v1.0

Offline Rust CLI/library converting documents to Markdown or plain text.
Released as [v1.0](https://github.com/FireTech-dev/docconvert/releases/tag/v1.0) with published binaries for Linux, macOS and Windows (default and `pdf-layout` profiles). Default-profile suite: 142 passed, 1 ignored, zero build warnings, toolchain 1.98.0.

## Two PDF build profiles

- **Default / 4.A:** pdf-extract decoding plus lopdf structure/scanned-page checks.
  Reading order is approximate; no PDF rasterization or layout reconstruction.
- **`pdf-layout` / 4.B:** optional PDFium glyph/font/geometry reconstruction, images,
  and raster OCR. Requires a trusted ABI-matched native PDFium library. Default stays
  4.A; 4.B is the prioritized layout source path, not an automatic fallback.

Both profiles support standalone-image OCR through a user-installed Tesseract-compatible
command. No engine, model or native library is downloaded during conversion.
See [project/architecture.md](project/architecture.md) AD-1 for why two profiles exist.

## Start here

Grab a binary from the [v1.0 release](https://github.com/FireTech-dev/docconvert/releases/tag/v1.0), or build from source — see **[docs/BUILDING.md](docs/BUILDING.md)** (prerequisites, native and cross-platform builds, tests, packaging).
`Cargo.lock` is committed (supply-chain pinning). After setup:

```sh
cargo run --locked -- input.docx -o converted
cargo run --locked -- documents --recursive --jobs 2 -o converted
cargo run --locked -- input.pdf -o converted
cargo run --locked --features pdf-layout -- input.pdf -o converted-layout
cargo run --locked -- scan.png --ocr auto -o converted
```

Existing outputs skip unless overwrite is enabled. Source documents are never intended
to be overwritten. Recursive output mirrors relative directories; same-stem collisions
are rejected before concurrent writes. Strict mode stops new work; in-flight files finish.
Configuration precedence: defaults → profile → config file → explicit CLI.

## Researched project skills

[skills/README.md](skills/README.md) indexes **34 researched skills**, with coverage of
all 42 detailed blueprint steps and the architecture component map. Each follows the
supplied template and includes actionable guidance, anti-patterns, checklists and
sources dated 2026-09-16. See its research limitations and Architect questions before
implementation. This documentation update does not change or validate application code.

## Documentation and status

- [Building from source](docs/BUILDING.md) — prerequisites, native/cross builds, tests, packaging
- [Run/test guide](RUN_TEST_GUIDE.md) — user-side verification, fake OCR, samples, diagnostics
- [CLI flags](docs/cli-reference.md) — live-help inventory checked by written tests
- [Format quirks](docs/format-quirks.md) — per-format behavior and incomplete fidelity
- [PDFium sourcing](docs/pdfium-setup.md) — library paths, ABI and CI setup
- [Implementation gaps](docs/implementation-notes.md) — remaining source/acceptance work
- [Architecture decisions](docs/architecture-addenda.md) — preserved-spec conflicts
- [Adding a format](docs/adding-a-format.md)
- [Static review](docs/static-review.md) — filesystem checks, **not** build evidence
- `project/agent.md` — living state; supplied specs/roles preserved byte-for-byte

Supported source paths include DOCX, EPUB, ODT, RTF, HTML/Markdown/text, CSV/TSV,
XLSX/XLS/ODS, PPTX, PDF and standalone image assets. Legacy DOC/PPT/ODP remain
unsupported/detect-and-report. Known limits: corpus depth for some formats is
representative-only (see the release notes), and external OCR has no timeout or
subprocess sandbox. See [format quirks](docs/format-quirks.md) before converting
untrusted documents.

## License

Dual-licensed `MIT OR Apache-2.0` — see [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).
