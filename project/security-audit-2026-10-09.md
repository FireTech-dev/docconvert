# Security Audit Report — docconvert

> Generated 2026-10-09 by the Security Auditor. Every finding below was traced to a real, reachable path and verified with a safe test in an isolated environment before being listed — nothing here is speculative. No fixes were made by the auditor; findings route through the normal remediation path. No exploit was constructed beyond proof-of-reachability. This is a re-audit of the 2026-09-25 report, focused on the delta since (commit c17171d: vendored pdf-extract/CMap hardening, `parent_or_dot` path helper, quick-xml/calamine bumps, pdf_layout preflight-recovery + glyph-dedupe, fuzz batch runner) plus a full re-proof of the standing controls.

## Summary

| | Count |
| --- | --- |
| Findings — Critical | 0 |
| Findings — High | 0 |
| Findings — Medium | 0 |
| Findings — Low | 0 |
| Findings — Informational | 1 |

The codebase holds the posture of the prior audit through a material hostile-input delta: the vendored PDF/CMap hardening (the highest-risk new code — hand-edits in attacker-reachable parsing) is contained on all 16 retained crash inputs with zero signal deaths, the full suite is green with zero warnings, `cargo audit` reports 0 vulnerabilities, and three fresh live probes (bare-CWD loader escape, shell-metachar engine string, OCR symlink-output re-proof) all held. The prior Low (PDFium adjacent-exe fallback) remains Resolved by documentation; the prior Informational (`cargo audit` never run) is now closed by execution. No Critical or High finding exists; nothing was mirrored to `agent.md` Known Issues.

## Findings

Ordered by severity, most severe first.

---

### Informational — Vendored path-patches bypass advisory version matching

**Location:** `Cargo.toml` / `fuzz/Cargo.toml` `[patch.crates-io]` (`patches/pdf-extract`, `patches/adobe-cmap-parser`, same versions 0.12.1/0.4.1).

**Reachable path:** N/A — process observation, not a code path. `cargo audit` matches advisories by registry version; path-patched crates resolve outside that matching, so a future RUSTSEC entry against `pdf-extract 0.12.1` / `adobe-cmap-parser 0.4.1` would not flag this tree even though the code descends from those versions.

**Impact:** a known-vulnerable upstream release could sit in the tree unnoticed if the team relies on `cargo audit` alone.

**Verification:** `cargo audit` run 2026-10-09 (1296 advisories, 177 deps): 0 vulnerabilities, 1 allowed warning (`ttf-parser` unmaintained residual, unchanged). The vendored crates' hardening rationale is recorded in `patches/` comments and AQ-013; post-patch fuzz ran 792,655 PDF inputs with zero new findings, which is the substantive coverage advisories can't provide here.

**Recommended fix:** no code change. When checking advisories, manually compare upstream `pdf-extract`/`adobe-cmap-parser` entries against the vendored trees (note the version pins in both `Cargo.toml` files); keep the 24h fuzz gate as the standing behavioral backstop for these two crates.

**Status:** Open (process note; accepted mitigation in place)

---

## Prior findings carried forward

- **Low — PDFium adjacent-to-executable fallback loading** (2026-09-25): still present by design in `src/extract/pdf_layout.rs` (`shared_pdfium` load order re-read this session: explicit path → `PDFIUM_DYNAMIC_LIB_PATH` → adjacent-exe → system; first-wins source check, per-call explicit-path validation). Docs warning present (`docs/pdfium-setup.md`). Read-verified the Encrypted arm still hard-stops *before* any library load, and Corrupt→warn+PDFium-attempt returns the preflight error when recovery also fails — no downgrade of the encryption contract. **Status:** Resolved (unchanged).
- **Informational — `cargo audit` never run** (2026-09-25): executed this session, see above. **Status:** Closed.

## Verified controls (probed live, held)

- **Bare-CWD loader escape → contained (new `parent_or_dot` interaction).** Bare-name `a.html` converted from its own directory with `<img src="../secret.txt">` loaded nothing and leaked nothing (`tests/zz_sec2.rs::sec_bare_name_loader_escape_contained`, temporary, removed after the run — code below).
- **OCR engine string is literal, no shell.** `ocr_command="nonexistent-engine; echo pwned"` under Force produced a typed `Ocr` start-failure and no side-effect marker file (`sec_ocr_command_is_literal_no_shell`). No `sh -c`/shell usage anywhere in `src/` (re-grepped).
- **OCR symlink-output refusal re-proof.** Prior 2026-09-25 control re-run against current `ocr.rs` (behaviorally unchanged since): evil engine symlinking `$stem.txt` to a marker got `Ocr("engine output is a symbolic link")`, marker byte-identical (`sec_ocr_symlink_output_refused_reproof`).
- **Hostile-PDF containment on the warning-silenced tree.** All 16 retained `fuzz/artifacts/fuzz_pdf/crash-*` inputs through the debug binary post-fix: 16 graceful (typed error or success), 0 signal deaths — the `dst_cid_lo` rename and warning-silence edits changed no reachable behavior.

```rust
// Verification test code (run 2026-10-09, all 3 green, then removed):
#[test]fn sec_bare_name_loader_escape_contained(){
    let t=common::Temp::new();std::fs::write(t.0.join("secret.txt"),b"TOP-SECRET").unwrap();
    std::fs::write(t.0.join("a.html"),b"<html><body><img src=\"../secret.txt\" alt=\"x\"></body></html>").unwrap();
    let exe=env!("CARGO_BIN_EXE_docconvert");
    let st=std::process::Command::new(exe).current_dir(&t.0).args(["a.html","-f","md","-o","out"]).status().unwrap();
    assert!(st.success());let md=std::fs::read_to_string(t.0.join("out/a.md")).unwrap();
    assert!(!md.contains("TOP-SECRET"),"{md}");
}
#[test]fn sec_ocr_command_is_literal_no_shell(){
    let t=common::Temp::new();let marker=t.0.join("PWNED");
    let evil2="nonexistent-engine; echo pwned".to_string();
    let opts=Options{ocr_mode:OcrMode::Force,ocr_command:evil2,..Options::default()};
    let err=docconvert::ocr::run_ocr(&common::png(),&opts).unwrap_err();
    assert!(matches!(err,ConvertError::Ocr(_)),"{err}");
    assert!(!marker.exists(),"shell metachars executed!");
}
#[test]fn sec_ocr_symlink_output_refused_reproof(){
    let t=common::Temp::new();let marker=t.write("marker.txt",b"original");
    let engine=t.0.join("evil-engine.sh");
    std::fs::write(&engine,format!("#!/bin/sh\nln -sf {} \"$2.txt\"\nexit 0\n",marker.display())).unwrap();
    #[cfg(unix)]{use std::os::unix::fs::PermissionsExt;std::fs::set_permissions(&engine,std::fs::Permissions::from_mode(0o755)).unwrap()}
    let opts=Options{ocr_mode:OcrMode::Force,ocr_command:engine.to_string_lossy().into_owned(),..Options::default()};
    let err=docconvert::ocr::run_ocr(&common::png(),&opts).unwrap_err();
    assert!(matches!(err,ConvertError::Ocr(ref s)if s.contains("symbolic link")),"{err}");
    assert_eq!(std::fs::read(&marker).unwrap(),b"original");
}
```

## Vulnerability Classes Checked

- Universal / path traversal (ZIP-slip, asset names, loaders, output joins, new `parent_or_dot` + `asset_namespace`): checked — generated names only, `normalize` fails closed at root (`..` pop on empty → None), bare-name maps to `.` (one component, cannot escape), stems are single components, symlink refusals on dir/assets/asset-dir; 1/1 new live probe held plus suite pins (`markdown_local_images_are_embedded_with_safe_paths`, `report_symlink_rejected`, namespace bound tests).
- Universal / vulnerable dependencies: checked — `cargo audit` 0 vulnerabilities; quick-xml unified single 0.41.0 (RUSTSEC-2026-0194/0195 pair gone from tree); `ttf-parser` unmaintained warning stands as accepted residual; vendored path-patches noted above (Informational).
- Universal / cryptographic failures: checked — no crypto, keys, or secrets in tree (sweep clean; only false-positive identifier matches).
- Universal / logging of security events: checked — hostile inputs yield typed errors; OCR temp paths scrubbed (`<ocr-temp>`), stderr filtered to 512 chars; `ConvertError::Display` carries no paths beyond the input filename (unchanged).
- Universal / broken access control: N/A — local single-user tool, no accounts (per threat model).
- Universal / insecure deserialization: checked — config is `key = value` with typed `set()` (typos → `Config` error, nearest-key suggestion); no eval/include/shell directives; JSON only serialized out.
- Universal / LLM integration: N/A — none exists.
- CLI / piped input: checked — no stdin consumption; file paths only.
- CLI / code execution from config/plugins: checked — `ocr-command` value flows only into argv array (live probe above); no plugin system; no shell anywhere in `src/`.
- CLI / path escaping the working directory: checked — `normalize` + canonicalize + prefix + size-cap on both loaders; `walkdir` `follow_links(false)`; symlink inputs refused in batch; output-inside-output excluded by canonicalize compare; stem-collision pre-rejected; `jobs` capped (8 auto / 256 explicit).
- Desktop / library hijacking: checked — prior Low stands Resolved; `native/pdfium/libpdfium.so` (7.8MB, gitignored) hash-matches the documented provenance (`docs/pdfium-setup.md:64`, `7670b3c5…`) and is NOT in the load order (explicit → adjacent-exe → system); no other dynamic loading.
- Desktop / priv-esc, IPC, updates, plaintext secrets: N/A — no elevation, no IPC, no updater, no credentials (per threat model).
- Web/API/Mobile: N/A — wrong platform (CLI tool, no network I/O).
- DoS / resource exhaustion: checked — caps unchanged and suite-pinned (200MB file, ZIP 10k/64MB/200MB, XML depth/elements/parts, PDF pages/streams/total, RTF/OMML depth 64, OCR output 16MiB, batch isolation + `--strict` + `catch_unwind` per file); `unsafe` scan clean (message-string false positives only); zero `expect!/panic!/todo!` in `src/`; locks poison-tolerant; 24h×3 fuzz clean (csv 12.2M, rtf 34.4M, pdf 793k runs, zero new crash/oom/timeout).

## Re-Audit

This report reflects the project as of 2026-10-09 only. Once a fix lands for any Critical or High finding, re-run its verification test as a regression check — don't close it out on the strength of the patch alone. (None exist at this time.)
