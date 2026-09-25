mod common;
use docconvert::{batch,config::{Options,FormatChoice},error::ConvertError};
#[test]fn recursive_outputs_mirror_tree_and_isolate_errors(){let t=common::Temp::new();let root=t.0.join("in");t.write("in/a/one.txt",b"One.");t.write("in/b/two.txt",b"Two.");t.write("in/ignore.exe",b"not a document");let opts=Options{recursive:true,jobs:2,format_choice:FormatChoice::Md,output:Some(t.0.join("out")),..Options::default()};let result=batch::convert_batch(vec![t.0.join("missing.txt"),root],&opts);assert_eq!(result.reports.len(),3);assert_eq!(result.reports.iter().filter(|(_,r)|r.is_err()).count(),1);assert!(t.0.join("out/a/one.md").exists());assert!(t.0.join("out/b/two.md").exists());}
#[test]fn strict_one_worker_stops_queue(){let t=common::Temp::new();let good=t.write("good.txt",b"Good.");let opts=Options{strict:true,jobs:1,output:Some(t.0.join("out")),..Options::default()};let result=batch::convert_batch(vec![t.0.join("missing.txt"),good],&opts);assert_eq!(result.reports.len(),1);assert_eq!(result.not_started,1);assert!(result.reports[0].1.is_err());}
#[test]fn same_stem_collision_rejected_before_writes_race(){let t=common::Temp::new();let a=t.write("a/same.txt",b"A.");let b=t.write("b/same.txt",b"B.");let opts=Options{jobs:2,overwrite:true,format_choice:FormatChoice::Md,output:Some(t.0.join("out")),..Options::default()};let result=batch::convert_batch(vec![a,b],&opts);assert!(matches!(&result.reports[1].1,Err(ConvertError::Output(s))if s=="batch inputs have colliding output stems"));}
#[test]fn cli_config_overrides_profile_cli_overrides_config(){
 // P6-S03 intent: all four layers together. Implemented order per P0-S08
 // (defaults → profile → config → CLI, i.e. CLI > config > profile) is pinned here.
 // NOTE: P6-S03's prose example swaps the middle two ("profile overrides config");
 // the code follows P0-S08 (config wins over profile), so this test pins P0-S08
 // and documents the prose swap instead of encoding the contradiction.
 let t=common::Temp::new();let input=t.write("source.txt",b"Content.");
 let config=t.write("settings.conf",b"format = md\n");
 // CLI md wins over everything (CLI > config > profile).
 let status=std::process::Command::new(env!("CARGO_BIN_EXE_docconvert")).arg(&input).arg("--profile").arg("plain").arg("--config").arg(&config).arg("--format").arg("md").arg("--output").arg(t.0.join("out")).status().unwrap();
 assert!(status.success());assert!(t.0.join("out/source.md").exists());
 // Config md wins over profile plain/txt (config > profile per P0-S08).
 let input2=t.write("second.txt",b"Content.");
 let status=std::process::Command::new(env!("CARGO_BIN_EXE_docconvert")).arg(&input2).arg("--profile").arg("plain").arg("--config").arg(&config).arg("--output").arg(t.0.join("out2")).status().unwrap();
 assert!(status.success());assert!(t.0.join("out2/second.md").exists());
 // CLI txt wins over config md (CLI > config).
 let input3=t.write("third.txt",b"Content.");
 let status=std::process::Command::new(env!("CARGO_BIN_EXE_docconvert")).arg(&input3).arg("--config").arg(&config).arg("--format").arg("txt").arg("--output").arg(t.0.join("out3")).status().unwrap();
 assert!(status.success());assert!(t.0.join("out3/third.txt").exists());
}
#[test]fn profiles_all_apply(){for p in ["academic","technical","plain","spreadsheet","archive","accessibility","tts","ocr","strict"]{let mut opts=Options::default();opts.apply_profile(p).unwrap();}let mut opts=Options::default();assert!(opts.apply_profile("unknown").is_err());
// P6-S03: each profile's documented effects actually manifest.
let mut o=Options::default();o.apply_profile("academic").unwrap();assert_eq!(o.format_choice,FormatChoice::Md);assert!(o.preserve_formulas&&o.include_comments);
let mut o=Options::default();o.apply_profile("technical").unwrap();assert_eq!(o.format_choice,FormatChoice::Md);assert!(o.preserve_formulas);
let mut o=Options::default();o.apply_profile("plain").unwrap();assert_eq!(o.format_choice,FormatChoice::Txt);
let mut o=Options::default();o.apply_profile("spreadsheet").unwrap();assert!(o.preserve_formulas&&o.include_comments&&o.include_hidden_sheets);
let mut o=Options::default();o.apply_profile("archive").unwrap();assert_eq!(o.format_choice,FormatChoice::Md);assert!(!o.embed_assets&&o.report_kind==docconvert::config::ReportKind::Json);
let mut o=Options::default();o.apply_profile("accessibility").unwrap();assert!(o.include_notes);
let mut o=Options::default();o.apply_profile("tts").unwrap();assert_eq!(o.format_choice,FormatChoice::Txt);assert!(o.minimal_placeholders);
let mut o=Options::default();o.apply_profile("ocr").unwrap();assert_eq!(o.ocr_mode,docconvert::config::OcrMode::Auto);
let mut o=Options::default();o.apply_profile("strict").unwrap();assert!(o.strict);}
#[test]fn xls_is_supported_not_rejected(){// AD-14 cross-ref for P6-S04: legacy XLS converts (detect-and-report is DOC/PPT only).
let t=common::Temp::new();let p=t.write("legacy.xls",&common::xls());let detection=docconvert::detect::detect_file(&p).unwrap();assert!(matches!(detection.format,docconvert::model::Format::Xls));assert!(docconvert::convert::preview(&p,&Options::default()).is_ok());}
#[test]fn deeply_nested_xml_bounded(){// §9 billion-laughs row: depth beyond the cap → Corrupt, never unbounded growth.
let mut xml=String::new();for _ in 0..100{xml.push_str("<a>")}for _ in 0..100{xml.push_str("</a>")}assert!(matches!(docconvert::extract::package::parse(xml.as_bytes(),false),Err(ConvertError::Corrupt(s))if s=="XML complexity limit exceeded"));}
#[test]fn cli_batch_exit_codes_and_progress(){
 // P6-S02: failure isolation + exit codes at the CLI seam (lib-level covered above).
 let t=common::Temp::new();let good=t.write("good.txt",b"Good.");
 let bad=t.write("bad.docx",b"PK\x03\x04broken");
 // Mixed batch → exit 1, but the valid file still converts (isolation, not abort).
 let out=t.0.join("out");let status=std::process::Command::new(env!("CARGO_BIN_EXE_docconvert")).arg(&good).arg(&bad).arg("--output").arg(&out).status().unwrap();
 assert_eq!(status.code(),Some(1));assert!(out.join("good.md").exists()||out.join("good.txt").exists());
 // All-valid batch → exit 0.
 let out2=t.0.join("out2");let status=std::process::Command::new(env!("CARGO_BIN_EXE_docconvert")).arg(&good).arg("--output").arg(&out2).status().unwrap();
 assert_eq!(status.code(),Some(0));
}
#[test]fn negative_entrypoints_exact_errors(){
 // P6-S04: each case maps to an architecture.md §9 threat-table row (comment per case).
 let t=common::Temp::new();let opts=Options::default();
 // §9 malformed ZIP → Corrupt, never partial output.
 let cases=[("truncated.pdf",b"%PDF-1.4\n".as_slice(),"PDF structure could not be loaded"),("bad.docx",b"PK\x03\x04broken".as_slice(),"incomplete or damaged ZIP archive")];for(name,bytes,message)in cases{let p=t.write(name,bytes);assert!(matches!(docconvert::convert::preview(&p,&opts),Err(ConvertError::Corrupt(s))if s==message));}
 // §9 oversized file → size-guard Unsupported.
 let p=t.write("large.txt",b"abc");let small=Options{max_file_bytes:2,..Options::default()};assert!(matches!(docconvert::convert::preview(&p,&small),Err(ConvertError::Unsupported(s))if s=="file exceeds --max-size"));
 // §9 encrypted PDF → Encrypted, no partial output.
 let mut enc=common::pdf_streams(&["BT /F1 12 Tf 30 700 Td (secret) Tj ET"]);enc.extend(b"\n/Encrypt 9 0 R\n");let p=t.write("enc.pdf",&enc);assert!(matches!(docconvert::convert::preview(&p,&Options::default()),Err(ConvertError::Encrypted(_))));
 // §9 legacy OLE2 DOC → Unsupported detect-and-report (AD-5; XLS excluded per AD-14).
 let mut ole=vec![0u8;64];ole[..8].copy_from_slice(b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1");let p=t.write("legacy.doc",&ole);assert!(matches!(docconvert::convert::preview(&p,&Options::default()),Err(ConvertError::Unsupported(s))if s.contains("detect-and-report")));
 // §9 unknown format → Unsupported with detection notes.
 let p=t.write("mystery.bin",b"\x00\x01\x02\x03\x04\x05\x06\x07binary blob");assert!(matches!(docconvert::convert::preview(&p,&Options::default()),Err(ConvertError::Unsupported(s))if s.contains("unknown file format")));
 // §9 extension mismatch re-asserted at report level (not just Detection struct).
 let p=t.write("mismatch.docx",&common::png());let r=docconvert::convert::convert_file(&p,&Options{output:Some(t.0.join("out")),..Options::default()}).unwrap();assert!(!r.extension_match);assert!(r.warnings.iter().any(|w|w.contains("does not match detected content")));
}
