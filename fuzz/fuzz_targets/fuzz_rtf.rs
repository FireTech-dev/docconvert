#![no_main]
use docconvert::extract::{rtf, ExtractCtx};
use libfuzzer_sys::fuzz_target;

// §14 target 1: RTF tokenizer + group walker must never panic on hostile input
// (P1-S05: degrade with a warning, only hard I/O fails). Any return (Ok or Err)
// is acceptable; a panic or hang is a finding.
fuzz_target!(|data: &[u8]| {
    let opts = docconvert::config::Options::default();
    let mut assets = docconvert::assets::AssetManager::default();
    let mut warnings = Vec::new();
    let mut ctx = ExtractCtx {
        options: &opts,
        assets: &mut assets,
        warnings: &mut warnings,
    };
    let _ = rtf::extract_rtf(data, &mut ctx);
});
