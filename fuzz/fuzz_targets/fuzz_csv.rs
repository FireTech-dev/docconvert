#![no_main]
use libfuzzer_sys::fuzz_target;

// §14 target 3: CSV/TSV parsing (quoting, escapes, embedded newlines — AD-2's
// correctness trap) must never panic. Any Document return is acceptable.
fuzz_target!(|data: &[u8]| {
    let opts = docconvert::config::Options::default();
    let meta = docconvert::model::Meta::default();
    let _ = docconvert::extract::csv::extract_csv(data, meta.clone(), b',', &opts);
    let _ = docconvert::extract::csv::extract_csv(data, meta, b'\t', &opts);
});
