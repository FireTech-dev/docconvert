#![no_main]
use docconvert::extract::{pdf, ExtractCtx};
use libfuzzer_sys::fuzz_target;

// §14 target 2: default-path PDF object scan + text decode must never panic
// (P4-S01: catch_unwind + typed Corrupt/Encrypted only). Any return is fine.
fuzz_target!(|data: &[u8]| {
    let opts = docconvert::config::Options::default();
    let mut assets = docconvert::assets::AssetManager::default();
    let mut warnings = Vec::new();
    let mut ctx = ExtractCtx {
        options: &opts,
        assets: &mut assets,
        warnings: &mut warnings,
    };
    let _ = pdf::extract_pdf_flat(data, &mut ctx);
});
