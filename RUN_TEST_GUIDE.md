# Run and Test Guide — Phases 0–6 Source Candidate — Skills Revision 4

> Note (2026-10-09): this guide predates the v1.0 release. The project has since been built, tested (142 passed, 1 ignored, zero warnings), tagged `v1.0`, and published with binaries — see [README.md](README.md) and [docs/BUILDING.md](docs/BUILDING.md). The commands below remain valid for user-side verification.

## Skills reference pack

This revision adds `skills/README.md`, 34 researched skills and a complete step/topic
map. Read the relevant skills before patching the source. Application source, tests and
dependencies are unchanged from revision 3; research has not established their correctness.
The skill checklists are future verification tasks, not instructions for the assistant
to bypass the standing no-install/no-build/no-run/no-test restriction.

## 0. Important status

The assistant did **not** install dependencies, generate a lockfile, compile, run, or test this application. This package contains implementation and test source, not a verified build. Known implementation gaps are listed in `docs/implementation-notes.md`; No phase is marked Complete.

The commands below are for **you** to execute. Work on copies of input documents in a disposable output directory. Do not enable `--overwrite` on important files during first verification.

Extract the archive, then use its project directory for every Cargo command:

```sh
tar -xzf docconvert-with-researched-skills-revision-4.tar.gz
cd docconvert
```

## 1. Install Rust and platform prerequisites

Skip the applicable installation section if you already have a current stable Rust toolchain and native linker. Installation requires network access; subsequent conversion does not.

### Debian / Ubuntu Linux

```sh
sudo apt-get update
sudo apt-get install -y build-essential curl ca-certificates
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup-init.sh
# Inspect the downloaded installer before executing it.
sh rustup-init.sh -y --profile minimal --default-toolchain stable
. "$HOME/.cargo/env"
rm rustup-init.sh
```

Other Linux distributions: install their C toolchain/linker and curl equivalents, then use the same rustup commands.

### macOS

```sh
xcode-select --install
```

Finish the graphical Command Line Tools installation before continuing:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup-init.sh
# Inspect the downloaded installer before executing it.
sh rustup-init.sh -y --profile minimal --default-toolchain stable
. "$HOME/.cargo/env"
rm rustup-init.sh
```

### Windows (PowerShell)

Install Visual Studio Build Tools with the **Desktop development with C++** workload, MSVC compiler, and Windows SDK. If winget is available:

```powershell
winget install --id Microsoft.VisualStudio.2022.BuildTools --exact --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
winget install --id Rustlang.Rustup --exact
```

Complete the installer prompts. Open a **new** PowerShell window, return to the extracted `docconvert` directory, then:

```powershell
rustup default stable
```

If winget is unavailable, use the official Visual Studio Build Tools installer and https://rustup.rs/ Windows installer with the same workload selection.

### Record toolchain details (all platforms)

```sh
rustup update stable
rustc --version --verbose
cargo --version
```

Default tests need Rust and its linker; the portable fake OCR fixture is compiled by the user-run tests using rustc. Layout tests additionally require a matching PDFium library. Python 3 is needed only for CI/native-fetch/packaging helpers. Real OCR requires optional Tesseract, not an OCR-linked Rust crate. No npm is used.

## 2. Resolve, pin, and install project dependencies

`Cargo.lock` is intentionally **absent** because source-only authorization delegates resolution to you. `Cargo.toml` initially uses wildcard versions; do not ship that state. Run these commands to replace them with current stable requirements while preserving the approved features:

```sh
cargo add zip --no-default-features --features deflate
cargo add quick-xml
cargo add clap --features derive
cargo add serde --features derive
cargo add serde_json flate2 walkdir base64 csv encoding_rs logos calamine
cargo add lopdf --no-default-features
cargo add pdf-extract
cargo add pdfium-render --optional --no-default-features --features pdfium_latest,thread_safe
cargo generate-lockfile
cargo fetch --locked
cargo tree --locked
```

These commands may change `Cargo.toml` and create `Cargo.lock`. Preserve and send **both files** with your results. The source was checked against Calamine documentation for 0.36.1, but nothing was locally resolved. If the resolved library APIs differ, report the errors rather than guessing a patch.

Commit the lockfile in your own Git repository (it is deliberately not ignored). Optional lock hash for your report:

Linux:

```sh
sha256sum Cargo.lock
```

macOS:

```sh
shasum -a 256 Cargo.lock
```

PowerShell:

```powershell
Get-FileHash Cargo.lock -Algorithm SHA256
```

## 3. Build / compile

```sh
cargo build --locked
cargo build --locked --release
```

Expected artifacts if builds succeed: `target/debug/docconvert` and `target/release/docconvert` (`.exe` on Windows). No binary size target is claimed as measured here.

To require zero compiler warnings on POSIX shells:

```sh
RUSTFLAGS="-D warnings" cargo build --locked
```

PowerShell:

```powershell
$env:RUSTFLAGS = '-D warnings'
cargo build --locked
Remove-Item Env:RUSTFLAGS
```

If the build fails, stop and send its complete output plus the manifest/lockfile. Do not continue on the assumption that the source-only inspection established compilation.

## 4. Run the application

Show help:

```sh
cargo run --locked -- --help
```

For a portable smoke input, create a file named `sample.txt` containing `Hello from docconvert.` Then:

```sh
cargo run --locked -- sample.txt --format md --output converted --report json
cargo run --locked -- --preview sample.txt
```

Expected **intended** outputs: `converted/sample.md` and `converted/sample.report.json`. Preview should not create output files.

Other examples (replace names with existing input files):

```sh
cargo run --locked -- document.docx -f md -o converted
cargo run --locked -- document.md -f txt -o converted
cargo run --locked -- book.epub --profile plain -o converted
cargo run --locked -- workbook.xlsx --profile spreadsheet -o converted
cargo run --locked -- slides.pptx --include-notes -o converted
```

`-o` is a directory. Output/report conflicts require `--overwrite`, but overwriting the input itself is refused. Asset files use `assets/<stem>/image-NNN.ext` beneath the output directory. PDF 4.A is available by default in source; 4.B requires the pdf-layout feature and a trusted matching native library. Standalone images can be OCRed under either build. See the additional commands below.

## 5. Run all tests

```sh
cargo test --locked
cargo test --locked --release
```

Fixtures are generated by `tests/common/mod.rs` while tests run; there are no committed binary fixtures. Temporary test directories use unique names and a cleanup guard. If a test process crashes or is terminated, leftover `docconvert-test-*` directories may remain in the OS temp directory.

## 6. Run a phase or a specific test

```sh
cargo test --locked --test model
cargo test --locked --test detect
cargo test --locked --test phase0
cargo test --locked --test e2e
cargo test --locked --test phase1_integration
cargo test --locked --test phase2_integration
cargo test --locked --test phase3_integration
cargo test --locked --test regressions
cargo test --locked --test phase4_integration
cargo test --locked --test phase5_integration
cargo test --locked --test phase6_integration
cargo test --locked --test cli_docs
cargo test --locked --test samples_gen
```

Exact individual test commands:

```sh
cargo test --locked --test phase0 markdown_golden -- --exact --nocapture
cargo test --locked --test phase1_integration epub_spine_images_and_footnotes -- --exact --nocapture
cargo test --locked --test phase1_integration rtf_malformed_battery -- --exact --nocapture
cargo test --locked --test phase2_integration xlsx_formula_preservation_end_to_end -- --exact --nocapture
cargo test --locked --test phase2_integration legacy_xls_preserves_cached_formula_value -- --exact --nocapture
cargo test --locked --test phase3_integration slide_order_and_per_slide_relationships -- --exact --nocapture
```

List every actual compiled test name:

```sh
cargo test --locked -- --list
```

Layout tests are now real and intentionally require a native library. Follow section 10 before running them.

### Revision 2 targeted checks

These test functions are written in `tests/regressions.rs`; none has been run by the assistant:

```sh
cargo test --locked --test regressions xml_views_survive_root_drop_without_a_child_tree -- --exact --nocapture
cargo test --locked --test regressions markdown_local_images_are_embedded_with_safe_paths -- --exact --nocapture
cargo test --locked --test regressions epub_cross_chapter_footnote_reference -- --exact --nocapture
cargo test --locked --test regressions pptx_picture_table_chart_keep_shape_order -- --exact --nocapture
cargo test --locked --test regressions ods_repeated_metadata_is_expanded -- --exact --nocapture
cargo test --locked --test regressions staged_overwrite_replaces_outputs_and_cleans_backups -- --exact --nocapture
```

The symlink regression is compiled on Unix only. Ordinary portable regressions remain enabled on Windows.

### Output staging and recovery

Use a local filesystem that supports hard links (for example, NTFS, APFS or a typical Linux filesystem). FAT/exFAT or restricted network shares may reject publication; the application should return an error and attempt rollback. No platform behavior is claimed verified.

Temporary files are reserved with create-new semantics beside each target. A normal success cleans `.docconvert-stage-*` and `.docconvert-backup-*`. If an error specifically reports rollback/backup cleanup failure, stop writing to that output directory and **preserve the backups** for recovery. Do not indiscriminately delete `.docconvert-*` files: backups may be the only surviving copy of prior output. Sudden power loss is not covered by ordinary-error rollback.

Explicit Markdown-to-Markdown copies retain their original relative links. Ensure referenced files are also available relative to the copied Markdown; the application does not rewrite them in passthrough mode.

## 7. Debugging and diagnostics

Verbose build, dependency tree, test output, deterministic test scheduling:

```sh
cargo build --locked -vv
cargo tree --locked
cargo test --locked -- --nocapture --test-threads=1
```

Backtrace on POSIX:

```sh
RUST_BACKTRACE=full cargo test --locked -- --nocapture
```

PowerShell:

```powershell
$env:RUST_BACKTRACE = 'full'
cargo test --locked -- --nocapture
Remove-Item Env:RUST_BACKTRACE
```

Optional source-format and lint tools (not run by the assistant):

```sh
rustup component add rustfmt clippy
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
```

Formatting/lint checks may report findings on this draft; they are not claimed clean. To apply formatting locally, use `cargo fmt --all`, then include changed files with your report.

Common issues:

- **No linker / MSVC error:** finish the platform prerequisite installation and reopen the terminal.
- **Registry/network failure:** verify network/proxy access during installation. Offline conversion does not imply offline first-time dependency installation.
- **Lockfile missing/outdated:** complete section 2; do not delete a lockfile just to conceal a version mismatch.
- **Dependency API errors:** send resolved Cargo.toml/Cargo.lock and full compiler output.
- **Existing outputs:** use a fresh output directory before considering overwrite.
- **Fixture assertion failure:** send the failing test name and assertion output. Do not weaken the assertion to get a passing result.
- **Formatting findings:** dense source can be normalized using the optional rustfmt command above; send any semantic change separately.

## 8. Return results

Send:

1. OS, architecture, `rustc --version --verbose`, `cargo --version`.
2. Resolved `Cargo.toml` and `Cargo.lock` (and lockfile hash if available).
3. Exact build/test command and complete output, including warnings.
4. For conversion bugs, a minimal non-sensitive input and expected versus actual output/report.
5. Any manual edits since extraction.

The assistant will inspect and patch source, update this guide if needed, and repackage without running the application unless you separately change that restriction.


## 10. PDF layout build and all layout tests

Read `docs/pdfium-setup.md` first. Use the exact resolved pdfium-render version's
supported ABI; 0.9.4 documentation was inspected, not installed. Supply your reviewed
library, then (POSIX example for a local `native/pdfium` directory):

```sh
export PDFIUM_DYNAMIC_LIB_PATH="$PWD/native/pdfium"
cargo build --locked --features pdf-layout
cargo build --locked --release --features pdf-layout
cargo test --locked --features pdf-layout
cargo test --locked --release --features pdf-layout
cargo test --locked --features pdf-layout --test phase4_integration -- --nocapture
cargo test --locked --features pdf-layout extract::pdf_layout::tests -- --nocapture
cargo test --locked --features pdf-layout --test cli_docs
cargo run --locked --features pdf-layout -- --preview samples/generated/basic.pdf
cargo run --locked --features pdf-layout -- samples/generated/basic.pdf -o converted-layout --no-strip-headers-footers
```

PowerShell environment equivalent:

```powershell
$env:PDFIUM_DYNAMIC_LIB_PATH = (Resolve-Path native/pdfium).Path
cargo test --locked --features pdf-layout
```

Alternatively pass `--pdfium-lib-path /your/reviewed/library` to CLI conversions.
Tests use the environment/system lookup. After switching Cargo features the executable
in the same target directory is replaced; never infer the build profile from its name.

Confirm no default PDFium dependency (the grep should produce no matches):

```sh
cargo tree --locked --no-default-features > dependency-tree-default.txt
grep pdfium-render dependency-tree-default.txt
cargo tree --locked --features pdf-layout > dependency-tree-layout.txt
```

PowerShell uses `Select-String pdfium-render dependency-tree-default.txt`.

## 11. Generated samples, batch and OCR

Generate the provided representative samples (not the entire required release corpus):

```sh
cargo test --locked --test samples_gen write_samples -- --ignored --exact
cargo run --locked -- samples/generated/basic.pdf -o converted-pdf
cargo run --locked -- samples/generated -r --jobs 2 -o converted-batch
cargo run --locked -- samples/generated --recursive --jobs 1 --strict -o converted-strict
```

`header-only.png` is deliberately only a header fixture, not a decodable OCR image.
Use a **real disposable scan** named `scan.png` for the following real-engine example.

Optional real OCR installation (user only):

```sh
# Debian/Ubuntu
sudo apt-get install -y tesseract-ocr tesseract-ocr-eng
# macOS, only if Homebrew is already installed:
brew install tesseract
```

Windows: install a reviewed Tesseract distribution, for example the UB Mannheim
installer linked from Tesseract's official installation documentation. Verify its
provenance; add its directory to PATH or pass the full executable path.
The executable must accept input-file, output-stem, `-l`, language as four arguments.
Language packs are engine-installed, not fetched by docconvert.

```sh
cargo run --locked -- scan.png --ocr auto --ocr-lang eng -o converted-ocr
cargo run --locked -- scan.png --ocr force --ocr-command tesseract -o converted-forced
cargo run --locked --features pdf-layout -- samples/generated/scanned.pdf --ocr auto -o converted-scan-pdf
```

The synthetic scanned PDF is a vector rectangle; it demonstrates raster/OCR dispatch,
not text recognition quality. Fake-engine tests recover a fixed marker independently
of pixels. Auto + absent engine warns; force + absent engine fails. A default-build
scanned PDF stays a placeholder and warns that raster OCR requires pdf-layout.

Specific regressions:

```sh
cargo test --locked --test phase5_integration fake_engine_argv_and_cleanup -- --exact --nocapture
cargo test --locked --test phase6_integration strict_one_worker_stops_queue -- --exact --nocapture
cargo test --locked --test phase6_integration recursive_outputs_mirror_tree_and_isolate_errors -- --exact --nocapture
cargo test --locked extract::pdf::tests::decoder_unwind_is_converted -- --exact --nocapture
```

## 12. CI, packaging, diagnostics and release gates

CI source in `.github/workflows/ci.yml` covers both feature profiles on five target
platforms. It **cannot pass as delivered**: generate/commit Cargo.lock, configure the
exact `RUST_TOOLCHAIN` repository variable and the ten PDFium URL/hash variables from
`docs/pdfium-setup.md`. Confirm runner availability. Review/pin third-party actions
according to your release security policy before enabling the workflow.

After a successful build/test, measure and package each profile separately:

```sh
cargo build --locked --release
python3 scripts/package_binary.py default
cargo build --locked --release --features pdf-layout
python3 scripts/package_binary.py pdf-layout
```

Windows uses `python` instead of `python3` if appropriate. The packaging script checks
the 5,000,000-byte default binary gate and writes actual sizes/hashes; it does not
invent audit/fuzz evidence or bundle a native PDFium library. Review licenses and
sourcing before distribution. No v1.0 tag or release is authorized by passing this
script alone. The full architecture §14 checklist remains open.

For hangs in OCR, terminate the conversion/engine and inspect/remove abandoned
`docconvert-ocr-*` temporary directories **only after verifying no conversion is using
them**. Normal returns clean up via a drop guard; process kills do not guarantee it.
Current OCR is blocking, has no timeout and captures engine stdout/stderr in memory.
For native PDFium crashes, record exact crate, native revision/hash, OS/architecture
and a minimal redistributable input. Rust backtraces cannot establish FFI safety.

For any failure send: full command/output, manifest + lockfile, rustc -Vv, selected
feature profile, PDFium provenance if used, OCR engine/version if used, and whether
output/temporary files remain. Do not send private documents without permission.
Stop after a build failure rather than treating source inspection as compilation.
