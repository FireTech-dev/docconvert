# PDFium: trusted native dependency, layout build only

Default build never needs PDFium. `--features pdf-layout` uses a dynamically loaded
native library, with no runtime download. The source APIs were inspected against
pdfium-render **0.9.4** (PDFium ABI feature `pdfium_latest`, then `pdfium_7881`),
not installed or resolved. Pin the crate and matching native binary together.
A future wildcard resolution can change this requirement.

Resolution order:
1. `--pdfium-lib-path FILE_OR_DIRECTORY`, or config `pdfium-lib-path = ...`.
2. `PDFIUM_DYNAMIC_LIB_PATH` (a file or directory).
3. Library beside the executable.
4. Platform library search path via `bind_to_system_library()`.

Files: Linux `libpdfium.so`; macOS `libpdfium.dylib`; Windows `pdfium.dll`.
Match the **process architecture**, not just the host OS. Linux x86_64 and aarch64,
macOS x86_64 and arm64, Windows x86_64 each require their own native build.
Do not load a library supplied by a document or an untrusted directory.

Warning (security audit 2026-09-25, Low): resolution step 3 loads whatever library
sits beside the executable without verification — install the binary in a
directory attackers cannot write to, or set an explicit path (steps 1–2) so the
fallback never triggers.

Build PDFium yourself using its upstream build instructions, or review a prebuilt
release from https://github.com/bblanchon/pdfium-binaries/releases. Select the
ABI/revision and platform matching the pinned Rust binding. Verify provenance,
license terms, and the archive's SHA256 **independently** before using it.
No native binary or checksum is invented or bundled with this source delivery.

For a reviewed archive, the supplied user-run helper requires both environment
variables (it fails closed if either is missing):

```sh
export PDFIUM_URL='https://YOUR-REVIEWED-ARCHIVE-URL'
export PDFIUM_SHA256='YOUR-INDEPENDENTLY-VERIFIED-64-HEX-CHECKSUM'
python3 scripts/fetch_pdfium.py
export PDFIUM_DYNAMIC_LIB_PATH="$PWD/native/pdfium"
cargo test --locked --features pdf-layout
```

PowerShell: set `$env:PDFIUM_URL` and `$env:PDFIUM_SHA256` to the reviewed values,
run `python scripts/fetch_pdfium.py`, then set
`$env:PDFIUM_DYNAMIC_LIB_PATH = (Resolve-Path native/pdfium).Path`.
The placeholders above are intentionally not presented as runnable download URLs.
Alternatively install/copy your verified library manually and pass its path.

CI repository variables: exact `RUST_TOOLCHAIN`, plus `PDFIUM_URL_` and
`PDFIUM_SHA256_` for suffixes `LINUX_X64`, `LINUX_ARM64`, `MACOS_X64`,
`MACOS_ARM64`, `WINDOWS_X64`. Native layout tests are not silently ignored when the
library is absent. Confirm runner labels/availability in your GitHub plan.
A missing or incompatible library is an actionable Config conversion error.
Native signals/access violations are **not** caught by Rust catch_unwind; native
PDFium is serialized per process, but not isolated in a sandbox process.

## Verified instance (2026-09-24, Linux x86_64 only)

Bindings: `pdfium-render` **0.9.4** (Cargo.lock-pinned, features `pdfium_latest`,
`thread_safe`). Native: `bblanchon/pdfium-binaries` tag `chromium/8066`,
asset `pdfium-linux-x64.tgz` (3,743,765 B):

- URL: `https://github.com/bblanchon/pdfium-binaries/releases/download/chromium/8066/pdfium-linux-x64.tgz`
- Archive SHA256: `0b43f405477cf2cfc4dbff06905093c3309756c6bca1fb9da99234a2ca97fed2`
- `lib/libpdfium.so` SHA256: `7670b3c597b02dfa3f98b23b49c3bb52536312f1ea686b739321731b6011f5a9` (7,833,208 B, ELF x86-64)
- License: MIT-style (Benoit Blanchon) for the packaging plus third-party notices
  (`licenses/`: pdfium/BSD, freetype, icu, libjpeg-turbo, libpng, zlib, abseil, …).
  Review them before redistributing; no binary is bundled with this source delivery.

With `PDFIUM_DYNAMIC_LIB_PATH` pointed at that library: full
`cargo test --features pdf-layout` green (144 passed, 1 ignored), including the
native OCR end-to-end test; layout release binary 4,188,216 B (3.99MiB),
SHA256 `248688cfac694c47674348972ebf4211c1ce5025c13cb840b4b661c49ad6d3f5`
(archive `9b48f485…`). Default release binary 4,307,952 B (4.11MiB, ≤5MB).
Real-PDF quality notes: InDesign double-draws display type (measured shadow
offsets (1.96,2.00)@40pt and (0.74,0.52)@12pt) — handled by glyph-dedupe (1pt)
plus systematic-shadow dropping (10pt bar, multi-bin, distinct-chars guard);
cover ornaments that survive are genuine source layout (poppler-confirmed).
Release builds are same-target sequential: rebuilding one profile overwrites
`target/release/docconvert` — package each profile's binary immediately
(per `skills/packaging.md`), sizes above are the recorded evidence.

Warning: a stale `libpdfium.so` previously present at the repo root (7,688,936 B,
MD5 `d0cc88ec02fa12bca2a68b55eb469187`) FAILS to bind with these bindings
(`LoadLibraryError`-family failure → actionable Config error) and is now
gitignored. Do not use it; replace it with the pinned build above. One process
binds exactly one native library (upstream OnceCell design): `extract_pdf_layout`
shares a single process-global `Pdfium`, validates an explicitly configured path
on every call, and maps PDFium password/security refusals to `Encrypted`.
