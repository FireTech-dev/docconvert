# Static review — revision 3 source candidate

Date: 2026-09-16. Scope: filesystem/text inspection only.

- 30 application Rust files and 15 test/helper Rust files inspected.
- 106 written `#[test]` annotations (including one ignored sample writer),
  not discovered tests or passing tests. Three checked-in text goldens.
- External module and include-file paths exist. Guide --test target files exist.
- A lightweight lexical delimiter scan found no unmatched delimiters. This is **not**
  a Rust parser, borrow checker, compiler, formatter or test run.
- Required new PDF/image/OCR/batch sources, docs, CI and packaging helper sources exist.
- Relative README/docs links resolve to workspace files.
- All six supplied originals compare byte-for-byte with uploaded files; hashes below.
- Cargo.lock is absent by authorization; no generated dependencies/build outputs exist.

Source readback corrections included PDFium scaled_font_size API, glyph ownership,
column gap threshold, two-band prose/table ambiguity, positioned image events, image
header index types and current CLI/report/doc integration. Published API evidence is
in pdfium-api-evidence.md; it does not prove resolved-version compatibility.

## Still unverified / incomplete

No installation, Cargo/Rust invocation, build, application run, test, generated
fixture run, CI, native/engine invocation, audit, fuzz session or binary measurement
was performed. Full Phase 0–6 conformance is not established. In particular native
PDFium fidelity/security, full required fixture catalogue, dependency allocation
limits, blocking OCR capture/cleanup, cross-platform output safety and release gates
remain open. See implementation-notes.md and format-quirks.md.

## Supplied original hashes

| File | SHA256 |
| --- | --- |
| `project/agent-archive.template.md` | `7ece869bbc54e5297b67adee55f8920ce123a43622288f86908e635f55b0f039` |
| `project/agent.template.md` | `309d2074b0bbc7ce3779ccefc20a7418270db3e9953355542f7cea3855e4ea0b` |
| `project/architecture.md` | `8a303f6b72f6a6f130342b9ad549677737f081f9a3b7203e713daf9a9629f1a1` |
| `project/blueprint.md` | `d998b02d698ebfc9e84dfd4adbe4f6484065ec60317310109328cdc247ca4cf6` |
| `roles/implementer.md` | `b5bf45520d5c0b8a9e977aa34a8dc5db7438dfaf9617f64c253c5cf6b388599b` |
| `roles/orchestrator.md` | `0e106a34da5f3f99d1f69cece725500fbcdc71f3584d214ea42faeb89844b0eb` |

## Source snapshot hashes

| File | SHA256 |
| --- | --- |
| `Cargo.toml` | `c427e4a076b365283e49b22dae009cd30c0cf0cc0a7b91cfd61ac4a6765ba503` |
| `src/assets.rs` | `8323995fa86c3b5192fc0e35cf9460bfeb8a38533c025af5c2a185f974d0e139` |
| `src/batch.rs` | `760e3ff6bc73cfdd9d33d1088ada3e3ff7ddb0173fbc1cd0b7565b24900a8a59` |
| `src/config.rs` | `833c27186719c7a28f2c9bc42cd19adb60478d8ee9a9f6ff6cc566d0fe446ae4` |
| `src/convert.rs` | `0bc97bd74e913d726893359ada8e205c10397a0ef9aba4ccc245a1e6a8d752c2` |
| `src/detect.rs` | `da24dcf8e64b8fddea26babba627e5c8732d3f6ff519fbd19dd2deb19af90210` |
| `src/error.rs` | `a47241bacc54f8184cc9a452ff8fec4a2cb0254a6d116afe9b9d1d2f767a819f` |
| `src/extract/csv.rs` | `c73f6012864a0695f7463e8d6128201321f021939dfa9a28acaa161f6f112dbc` |
| `src/extract/docx.rs` | `e27152df8fe88ae9bfb0f713df05e7abe5a5daaa824535c467f44e73a7c32220` |
| `src/extract/epub.rs` | `9acc48d59cff56140feff03c6207f6a3fe3ef619e8fde81974c856e323e5f796` |
| `src/extract/html.rs` | `ed9473af49ae9ac3207609cd00492276e374510dc87372379313743ce32c84a4` |
| `src/extract/image.rs` | `c84a0dbf7e960a42e730fe5c0e8d89b6e230eb991d79adc250d45d9741b1edaa` |
| `src/extract/mod.rs` | `da712af23f692c6c23e59620c1c1fd240ab869a1b2a8ef28db40537cd4c80414` |
| `src/extract/ods.rs` | `12fd0cc40c2184589ba176bc429bb73ad4eed8a90f9e7e14bbc8990745f82ff3` |
| `src/extract/odt.rs` | `d4350e66e4b08281bd5d9506cfadd89bfdea8ee1ea8dce8c48221fc26f3da047` |
| `src/extract/ooxml.rs` | `ea01d56bd8ee1723518fb8b73e5e76bc6aca6d09904425b02db89b055d52546c` |
| `src/extract/package.rs` | `9f5d01561bc454a261e0f75651495ad60796f92b49c53889b1d288a5f3b7c5fd` |
| `src/extract/pdf.rs` | `85e78ca5bbfa568c84f7b8570b179309fe0803415748dd48fc65e3f041171243` |
| `src/extract/pdf_layout.rs` | `8552ca9dd7bc57754d2b5644de129b2f05947909ccaa12712f53c07ae9dc2bdf` |
| `src/extract/pptx.rs` | `1e365797509bedce59c37442fc59a582898b0c312d3461ef808f75315cc0bf29` |
| `src/extract/rtf.rs` | `62c905b291e26e3c620419fa6ade076de334991b68d5069517be3427227e1931` |
| `src/extract/text.rs` | `943fca4f16e19f1767b1ba60526e7f32b99e8b4832ebe8fbd42b7a38967f7e63` |
| `src/extract/xlsx.rs` | `3d44101c4470ce4e3b5b8f7b376710084168a0d2291d546bd99939ad525297e7` |
| `src/lib.rs` | `b0d076e867a2a347fd3cde6f3811e735dce3609238ce5d5e0d8b9d76ee0f6e1c` |
| `src/main.rs` | `abb58388203058422ab2122a01aaf96aae95aba751be53916e17e8aaddd7f64d` |
| `src/model.rs` | `71ec47dea21a77b4ee5c2fbf5e21ccdda217c85ced2972dc1cafa803052cfff2` |
| `src/ocr.rs` | `6e109de9f909eda888a6dcdf24eb8eba5708341c2eefb047abc1b0e931840bd9` |
| `src/render/markdown.rs` | `07ced71b393cf6c40bfacdd5e36b7086d1cb82dd9473aab170bbd180280b7f00` |
| `src/render/mod.rs` | `fd119d28c760b55e6bb983e33d262b4d6ca93a377ac73c05236a75acd569e920` |
| `src/render/txt.rs` | `64ac479f73ccb8da3a33286f746fe8314882a0943486df4dc89af8f4d6f2da4c` |
| `src/report.rs` | `e20bc4a3e92a1981e1c54fa04dbece5d8dcced415730ecb0c1d4e2c467f501f7` |
| `tests/cli_docs.rs` | `5e7b86408878a93d22b23c977608641f25d592efd488c686f851a67927a6aaf1` |
| `tests/common/mod.rs` | `c841842467f4ff11407e6910aa78bf17c901d04e81aa1fd1fc27f39f7cb01d2f` |
| `tests/detect.rs` | `1ea489db8ce46ad996e1f87a4128598aafd73999d7f0068cbe8801fb066340f3` |
| `tests/e2e.rs` | `9e33f6b87d37c47fe4831ec8cd13a7043cab02bc668fb24b1b65e2f831e71c98` |
| `tests/fixtures/fake-ocr/main.rs` | `358bd88a7dc08aefc4382b0ef614c9d8917d236eee6feb23baf7008b6f6c3318` |
| `tests/model.rs` | `0101bb861ecd9f0f2ab2b3966ff2a3b1c8c7ae545b7c03419a61c94e01f29304` |
| `tests/phase0.rs` | `5b12fbb61d1f114a176723eab087c5a829656678e1b2eb2d3ef652d3a953e5f7` |
| `tests/phase1_integration.rs` | `1aaeaa97a199b8b6804dbea8f42ce5de28ad93e4f47490876ee94f051ab40251` |
| `tests/phase2_integration.rs` | `b49349d3b15c492805a5a7b9f1f5049badf48211f0811b5854d53b5dcb45d6c7` |
| `tests/phase3_integration.rs` | `923b2993430bd3fbe1e420ca5e53b50349bed7870ce2323de98928e4905a1a0c` |
| `tests/phase4_integration.rs` | `3e1c367797be0fe745cc236b7ccd13ae983f64bb9504a30c4d2eed3dd5d77e36` |
| `tests/phase5_integration.rs` | `c5f65b870056e46174038be6ff33ceaf64569484197c628ae814c480dbc7bce7` |
| `tests/phase6_integration.rs` | `7fdbf274c1785cfab71e344508b2e1ef594ed2f0dc5a7a77c75b6c1c107c0e1f` |
| `tests/regressions.rs` | `d1106fcefd6b49b68066917e2d21f01991e5636f344ff457da6aa8afed67dff3` |
| `tests/samples_gen.rs` | `7c4244db04bf7bf1a50ad236d094fcd82d02c0810e122843eb36b28b31424b74` |
