# Adding a format

1. Read preserved `project/architecture.md`, blueprint, roles and current agent state.
   Obtain scope approval; source presence never bypasses runtime acceptance.
2. Add a `Format` variant/display mapping, signature detection and extension mapping
   in `src/model.rs` / `src/detect.rs`. Extensions are hints, never sole trust evidence.
3. Add one extractor module under `src/extract/` and dispatch in `extract_file`.
   Return a CDM `Document`; use `ExtractCtx` for options, assets and warnings.
4. Reuse `package.rs` streaming cursors/limits and `ooxml.rs` helpers rather than a new
   XML tree. DOCX and PPTX demonstrate shared relationships/styles; XLSX/ODS lower
   workbooks through the existing shared path.
5. Preserve unavailable content as meaningful placeholders/warnings; never download
   document resources, execute document code, or emit uncontrolled library errors.
6. Let the existing output selection, renderers, staging and reporting do their work.
   A batch worker owns one full conversion; no shared extractor state except the
   explicitly serialized native PDFium lifecycle.
7. Write source-generated fixtures, goldens, corruption/size/depth/error cases and
   CLI integration coverage. Add documentation, CLI drift inventory if needed, and
   verify both feature profiles. See RUN_TEST_GUIDE.md for exact user-run commands.
8. Record real build/test/corpus/audit/size results in agent.md. Review allocations
   before parser/FFI calls: post-allocation limits alone do not establish safety.
