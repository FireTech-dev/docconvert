# Building docconvert from source

> Baseline: every native-build command below was exercised 2026-10-09 on
> linux-x86_64, stable Rust 1.98.0 (`cargo build --locked --release --offline`
> clean; binary 4,320,280 B). All five release targets are additionally proven by
> the v1.0 tag CI run, which builds and tests each one on native runners.

Yes — the published source archive compiles into the app with a stock Rust
toolchain. No code generation, no submodules, no extra SDK for the default
profile (it is pure Rust with zero native dependencies).

## Prerequisites

| Need | Linux | macOS | Windows |
| --- | --- | --- | --- |
| Rust stable via rustup | `build-essential curl ca-certificates` + [rustup](https://rustup.rs/) | Xcode Command Line Tools (`xcode-select --install`) + rustup | Visual Studio Build Tools ("Desktop development with C++") + rustup |
| Network | Only for setup (rustup install, first dependency fetch) — conversion itself is offline | Same | Same |
| Disk | ~3 GB for `target/` on a full release build | Same | Same |

`Cargo.lock` is committed, so `cargo build --locked` reproduces the exact
dependency set. The first build downloads crates from crates.io unless the
cargo cache is already primed; pass `--offline` only when it is.

Get the source — either archive (no git needed):

```sh
tar -xzf docconvert-v1.0-source.tar.gz
cd docconvert-v1.0
```

or pinned clone:

```sh
git clone --branch v1.0 https://github.com/FireTech-dev/docconvert.git
cd docconvert
```

## Native build (default profile)

```sh
cargo build --locked --release
./target/release/docconvert --help
```

Debug build for development: `cargo build --locked`. Run without installing:

```sh
cargo run --locked -- input.docx -o converted
```

Run the test suite:

```sh
cargo test --locked
```

## `pdf-layout` profile build

This profile adds full PDF page-layout fidelity but needs a native PDFium
library at build and run time — Cargo cannot fetch it for you. Follow
[pdfium-setup.md](pdfium-setup.md) first (sourcing, ABI match, trust notes),
then:

```sh
cargo build --locked --release --features pdf-layout
```

At runtime the library is found via `PDFIUM_DYNAMIC_LIB_PATH`, a vendored copy
next to the binary, or `--pdfium-lib-path` (details in
[pdfium-setup.md](pdfium-setup.md)). Without a usable library, layout builds
fail fast with an actionable error instead of silently falling back.

## Cross-compilation

The default profile has no C/native dependencies, so cross-compiling is a
toolchain + linker question, not a source-code question. The proven path is
native builds per platform (exactly what the v1.0 CI matrix does); use it for
anything you ship. Cross builds are for convenience and carry the caveats
below.

| Target | Triple | Extra needs | Status |
| --- | --- | --- | --- |
| Linux x86_64 | `x86_64-unknown-linux-gnu` | None beyond the host toolchain | Verified locally + CI |
| Linux aarch64 | `aarch64-unknown-linux-gnu` | `rustup target add aarch64-unknown-linux-gnu`, an aarch64 linker (e.g. `gcc-aarch64-linux-gnu`) | CI-native proven; cross command not run here |
| macOS x86_64 | `x86_64-apple-darwin` | Apple SDK — realistically: build on a Mac | CI-native proven; cross from Linux not recommended |
| macOS arm64 | `aarch64-apple-darwin` | Apple SDK — realistically: build on a Mac | CI-native proven; cross from Linux not recommended |
| Windows x86_64 | `x86_64-pc-windows-gnu` | `rustup target add x86_64-pc-windows-gnu`, mingw linker (e.g. `x86_64-w64-mingw32-gcc`) | Cross `cargo check` attempted here: rustc is fine, but the first fetch of Windows-only crates (e.g. `anstyle-wincon`) needs network — `--offline` fails with "failed to download … attempting to make an HTTP request, but --offline was specified". With network once, then repeatable offline |

Pattern for any row (needs network on first run):

```sh
rustup target add <triple>
cargo build --locked --release --target <triple>
```

macOS cross-compilation from Linux additionally requires Apple's SDK, which
Apple licenses only with its own tooling — that is why the table says to
build macOS binaries on a Mac (or take them from the published release).

## Packaging the result

```sh
python3 scripts/package_binary.py default
```

Produces `release-artifacts/docconvert-<os>-<arch>-default.tar.gz` (binary +
README + `pdfium-setup.md` + `Cargo.lock`) plus a `.json` manifest with byte
size, SHA-256 hashes, and the ≤5 MB budget check. The script refuses to pass
an over-budget default binary silently.

## Troubleshooting

- `error: failed to download <crate> … --offline was specified` — the cargo
  cache lacks that crate for the requested target. Re-run once with network,
  then `--offline` works from then on.
- Linker errors on `--target` builds (`cc not found`, `unable to find library`)
  — the target's linker/SDK is missing; see the table above.
- `pdf-layout` runtime error about PDFium — no usable native library was
  found; see [pdfium-setup.md](pdfium-setup.md), never worked around by
  copying an unrelated platform's binary.
