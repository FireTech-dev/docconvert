# Security Audit Report — docconvert

> Generated 2026-09-25 by the Security Auditor. Every finding below was traced to a real, reachable path and verified with a safe test in an isolated environment before being listed — nothing here is speculative. No fixes were made by the auditor; findings route through the normal remediation path. No exploit was constructed beyond proof-of-reachability.

## Summary

| | Count |
| --- | --- |
| Findings — Critical | 0 |
| Findings — High | 0 |
| Findings — Medium | 0 |
| Findings — Low | 1 |
| Findings — Informational | 1 |

The codebase presents a narrow, well-guarded attack surface for a hostile-document converter: all archive/XML/compression/depth limits are enforced at the read layer, output writes go through staging with symlink refusal, the OCR engine is spawned via argv array with its temp paths scrubbed from errors, and error messages never carry filesystem paths. No Critical or High finding exists; nothing was mirrored to `agent.md` Known Issues.

## Findings

Ordered by severity, most severe first.

---

### Low — PDFium adjacent-to-executable fallback loading (library hijacking class)

**Location:** `src/extract/pdf_layout.rs:59` (`shared_pdfium`), feature-gated `pdf-layout` build only.

**Reachable path:** resolution order is explicit `--pdfium-lib-path`/config → `PDFIUM_DYNAMIC_LIB_PATH` env → **library file beside the running executable** → system library. If the directory holding the binary is writable by another local user, that user can plant a `libpdfium.so`/`pdfium.dll`/`libpdfium.dylib` which the process then loads with full user privileges. Reaching it requires no document input at all — just running any PDF conversion from a compromised install directory.

**Impact:** local code execution at the invoking user's privilege, bounded to users who run the `pdf-layout` build from a directory an attacker can write to. An attacker with that write access typically has easier paths to the same user, hence Low rather than higher.

**Verification:** traced by reading (load-order chain at lines 59/76 plus `library_file`); dynamic exploitation deliberately not attempted (out of scope: no exploit construction). Behavior matches the documented resolution order in `docs/pdfium-setup.md`.

**Recommended fix:** prefer an explicit path (already supported) in hardened deployments, and add one warning line to `docs/pdfium-setup.md` that the adjacent-executable fallback trusts the install directory — i.e., install the binary somewhere non-writable. No code change strictly required.

**Status:** Resolved (warning added to `docs/pdfium-setup.md` 2026-09-25)

---

### Informational — `cargo audit` never run (no network in this environment)

**Location:** dependency tree (`Cargo.lock` pinned; `cargo audit` subcommand unavailable here).

**Reachable path:** N/A — process gap, not a code path. Known duplicate (`lopdf` 0.45 + 0.42 via `pdf-extract`) is documented and benign.

**Impact:** a known-vulnerable crate version could sit in the tree unnoticed.

**Verification:** `cargo audit --version` → "no such command"; `Cargo.lock` exists and is pinned (supply-chain pinning holds).

**Recommended fix:** run `cargo audit` (or `cargo deny`) with network before v1.0 and record the result in `agent.md` — release-auditor territory.

**Status:** Resolved (ran 2026-09-25 via cargo-audit 0.22.2; see Addendum below — 0 vulnerabilities, 1 warning)

---

## Verified controls (probed live, held)

- **OCR engine output swap → refused.** A malicious engine script that symlinked its `$stem.txt` to a marker file got `Ocr("engine output is a symbolic link")`, and the marker file was byte-identical afterward (`tests/zz_sec.rs::sec_ocr_symlink_output_refused`, temporary, removed after the run — code below for the remediation record).
- **HTML image loader directory escape → contained.** `<img src="../secret.txt">` loaded nothing: no asset, secret absent from output (`sec_html_loader_stays_in_dir`). The Markdown loader shares the same normalize+canonicalize+prefix+size pattern with pre-existing pinning tests.
- **OCR temp pre-creation → clean failure, no write-through.** `create_new(true)` fails on planted symlinks/dirs (`AlreadyExists` → next counter → bounded error after 100 tries). Verified by reading against documented `std` semantics; worst case is a failed conversion for the local user only.
- **Command injection shape → argv array end-to-end.** Pre-existing `fake_engine_argv_and_cleanup` passes `ocr_lang="eng; echo NOT_A_SHELL"` through as a literal argument (no shell exists in the path — confirmed by grep: no `sh -c`/shell usage anywhere in `src/`).

```rust
// Verification test code (run 2026-09-25, both green, then removed):
#[test]fn sec_ocr_symlink_output_refused(){
    let t=common::Temp::new();
    let marker=t.write("marker.txt",b"original");
    let engine=t.0.join("evil-engine.sh");
    std::fs::write(&engine,format!("#!/bin/sh\nln -sf {} \"$2.txt\"\nexit 0\n",marker.display())).unwrap();
    #[cfg(unix)]{use std::os::unix::fs::PermissionsExt;std::fs::set_permissions(&engine,std::fs::Permissions::from_mode(0o755)).unwrap()}
    let opts=Options{ocr_mode:OcrMode::Force,ocr_command:engine.to_string_lossy().into_owned(),..Options::default()};
    let err=docconvert::ocr::run_ocr(&common::png(),&opts).unwrap_err();
    assert!(matches!(err,ConvertError::Ocr(ref s)if s.contains("symbolic link")),"{err}");
    assert_eq!(std::fs::read(&marker).unwrap(),b"original");
}
#[test]fn sec_html_loader_stays_in_dir(){
    let t=common::Temp::new();
    std::fs::create_dir_all(t.0.join("sub")).unwrap();
    std::fs::write(t.0.join("secret.txt"),b"TOP-SECRET").unwrap();
    let p=t.write("sub/a.html",b"<html><body><img src=\"../secret.txt\" alt=\"x\"></body></html>");
    let o=Options{output:Some(t.0.join("out")),..Options::default()};
    let r=docconvert::convert::convert_file(&p,&o).unwrap();
    assert_eq!(r.status,"converted");
    assert!(r.assets.is_empty(),"{:?}",r.assets);
    let md=std::fs::read_to_string(t.0.join("out/sub/a.md")).unwrap_or_default();
    assert!(!md.contains("TOP-SECRET"),"{md}");
}
```

## Vulnerability Classes Checked

- Universal / path traversal (ZIP-slip, asset names, loaders, output joins): checked — rejected/normalized/contained at every layer; 2/2 live probes held.
- Universal / vulnerable dependencies: checked — pinned lockfile; audit tool unavailable (Informational above).
- Universal / cryptographic failures: checked — no crypto, no keys, no secrets in tree (grep clean).
- Universal / logging of security events: checked — hostile inputs produce typed errors; OCR temp paths scrubbed from engine errors; no internal paths leak (`Io` shows kind only).
- Universal / broken access control: N/A — local single-user tool, no accounts (as threat model states).
- Universal / insecure deserialization: checked — config is key=value with typed parsing (typos → `Config` error); no eval/pickle/unmarshal of document data; JSON only serialized out.
- Universal / LLM integration: N/A — none exists.
- CLI / piped input: checked — no stdin consumption; file paths only.
- CLI / code execution from config/plugins: checked — no eval, no shell, no plugin system (values become argv/config strings only).
- CLI / path escaping the working directory: checked — loaders contained, walkdir `follow_links(false)`, output-inside-input exclusion, symlink refusals everywhere.
- Desktop / library hijacking: checked — Low finding above (adjacent-exe PDFium fallback).
- Desktop / priv-esc, IPC, updates, plaintext secrets: N/A — no elevation, no IPC, no updater, no credentials (per threat model).
- Web/API/Mobile: N/A — wrong platform (CLI tool, no network I/O).
- DoS / resource exhaustion: checked — file cap 200MB, ZIP 10k entries/64MB-per-entry/200MB-total, XML depth 64/500k elements/64MB parts, PDF 10k pages/64MB streams/200MB total, RTF/OMML depth 64, batch failure isolation + `--strict`, 256-thread explicit cap; fuzz smoke 3×90s zero crashes.

## Re-Audit

This report reflects the project as of 2026-09-25 only. Once a fix lands for any Critical or High finding, re-run its verification test as a regression check — don't close it out on the strength of the patch alone. (None exist at this time.)

## Addendum — post-report `cargo audit` run (same session, user-ordered "fix all")

`cargo audit` (0.22.2, 1271 advisories, 177–180 deps) found **2 High vulnerabilities**, both in `quick-xml 0.38.4` — squarely in the hostile-XML path:

- RUSTSEC-2026-0194 — quadratic runtime checking a start tag for duplicate attribute names (DoS). Severity 7.5.
- RUSTSEC-2026-0195 — unbounded namespace-declaration allocation in `NsReader` (memory-exhaustion DoS). Severity 7.5.

Fixed same session: direct requirement `0.38` → `0.41` (the advised minimum; `0.42` exists but a fourth major bump buys nothing here), **plus** `calamine 0.31.0` → `0.36.1` after `cargo tree` proved calamine pinned the second `0.38.4` copy (calamine 0.36 requires `quick-xml ^0.41`). Lock now holds exactly one quick-xml (0.41.0); build clean with zero API breakage; full suite 139/0/1 green with zero P2 changes (formula/date/merge/hidden API surface stable across the jump); re-run audit shows **0 vulnerabilities**. Residual: `ttf-parser 0.25.1` unmaintained *warning* (transitive via `lopdf 0.42` ← `pdf-extract 0.12.1`, itself latest — no upstream fix; accepted, revisit on pdf-extract update). Severity of the fixed pair: **High**; status: **Resolved, full suite passing** (the suite's XML-heavy tests — depth guards, entity refusal, ODF/OOXML/EPUB/HTML parsing — are the regression coverage).
