# CI setup — user actions to get the 10-combo matrix green

The workflow (`.github/workflows/ci.yml`, "Source candidate validation") is written,
structurally validated (5 platforms × 2 profiles = 10 jobs, least-privilege
`contents: read`), and partially evidenced locally (linux-x86_64 default green;
`pdf-layout` compiles). Everything below requires **your** GitHub account, runners,
and reviewed provenance — the assistant has no remote, no runner access, and will
not invent URLs, checksums, pushes, or tags.

## 0. Local pre-flight (already done 2026-09-25, do not repeat unless source changes)

- `cargo build --locked` clean, zero warnings; `cargo test --locked` 130 passed,
  0 failed, 1 ignored (toolchain 1.98.0)
- `cargo tree --locked` contains zero `pdfium` entries (default profile)
- Release binary 4,307,952 B (within ≤5MB); `package_binary.py default` artifact hashed
- `cargo check --locked --offline --features pdf-layout` green; layout lists 144 tests
- Fuzz smoke 90s/target, zero crashes (gate needs 24 CPU-hours/target — §4)

## 1. Create the repo and push (needs your explicit "commit" + "push" words)

```sh
git remote add origin <YOUR-EMPTY-REPO-URL>
# after the assistant commits on your explicit request:
git push -u origin master
```

No commits, pushes, or tags happen without you saying so explicitly.

## 2. Set repository Variables (Settings → Secrets and variables → Actions → Variables)

| Variable | Value — reviewed, never invented |
|---|---|
| `RUST_TOOLCHAIN` | Exact toolchain, e.g. `1.98.0` |
| `PDFIUM_URL_LINUX_X64` / `PDFIUM_SHA256_LINUX_X64` | Verified instance in `docs/pdfium-setup.md` (chromium/8066, archive SHA `0b43f405…a97fed2`) — copy from there |
| `PDFIUM_URL_LINUX_ARM64` / `PDFIUM_SHA256_LINUX_ARM64` | Your reviewed pick per `docs/pdfium-setup.md` procedure |
| `PDFIUM_URL_MACOS_X64` / `PDFIUM_SHA256_MACOS_X64` | Same — one reviewed archive per platform/arch |
| `PDFIUM_URL_MACOS_ARM64` / `PDFIUM_SHA256_MACOS_ARM64` | Same |
| `PDFIUM_URL_WINDOWS_X64` / `PDFIUM_SHA256_WINDOWS_X64` | Same (`pdfium.dll`) |

How to review one archive (per platform): download from the pinned release page,
compute SHA256 locally, confirm the contained library matches the process
architecture and the `pdfium-render 0.9.4` binding, review the license files, and
only then paste URL + checksum into Variables. A checksum copied from the same
download page is not independent verification.

## 3. Confirm runners

Your GitHub plan must offer `ubuntu-24.04`, `ubuntu-24.04-arm`, `macos-15-intel`,
`macos-15`, `windows-2022`. If a label is unavailable, that combo stays red —
record it as an environment gap, do not mark the gate green.

## 4. Trigger and read results

Push, pull request, or Actions → "Source candidate validation" → Run workflow.
Expect 10 green jobs. Each uploads `candidate-<PLATFORM>-<profile>` artifacts
(release archive + JSON report + dependency tree). Collect from the run:

- default binary bytes per platform (each must be ≤5,000,000)
- layout binary bytes + SHAs (no cap, must be recorded)
- `dependency-tree.txt` per job (default trees must not mention `pdfium-render`)

A missing/incompatible native library fails its layout job loudly (actionable
Config error) — that is the designed behavior, never silently skipped.

## 5. Full fuzz (local machine or self-hosted runner, ~18 wall-hours on 4 cores)

Use the batch runner — configurable budget, resumable chunks, crash-stop:

```sh
scripts/fuzz_batch.sh                                  # 24h x rtf+pdf+csv, 60-min chunks
scripts/fuzz_batch.sh --targets pdf --hours 24         # one target only
scripts/fuzz_batch.sh --hours 6 --chunk-min 30         # short slices (killed runs resume on re-run)
scripts/fuzz_batch.sh --reset --targets rtf            # drop resume state, keep corpus
```

How it works: each target runs in `--chunk-min` libFuzzer slices until `--hours`
of CPU time accumulates; progress lives in `fuzz/.batch-state/` (gitignored, survives
kills — re-run the same command to resume); corpus grows in `fuzz/corpus/`
(gitignored); any crash/oom/timeout artifact stops everything with exit 1 and the
artifact path; per-run logs go to `fuzz/logs/` with agent.md-ready evidence lines
at the end. Gate: 24 CPU-hours per target, zero findings. On the first artifact:
stop, file the input, fix, restart that target's clock. Manual equivalent (single
unbroken run, needs a host that stays up):

```sh
cargo +nightly fuzz run fuzz_rtf -- -max_total_time=86400
cargo +nightly fuzz run fuzz_pdf -- -max_total_time=86400
cargo +nightly fuzz run fuzz_csv -- -max_total_time=86400
```

## 6. After green: reviews, then tag

1. Run the `release-auditor` role (architecture review) and `security-auditor`
   role; remediate findings one step at a time.
2. Only when `project/agent.md` evidences every §14 item (CI 10/10, fuzz 3×24h,
   corpus, sizes, reviews, commit) — tag `v1.0`. Never tag on partial evidence.
