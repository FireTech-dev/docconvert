mod common;
#[test]
fn generated_samples_have_recognized_signatures(){let t=common::Temp::new();for(name,bytes)in samples(){let p=t.write(name,&bytes);assert_ne!(docconvert::detect::detect_file(&p).unwrap().format,docconvert::model::Format::Unknown);}}
fn encrypted_pdf()->Vec<u8>{let mut b=common::pdf_streams(&["BT /F1 12 Tf 30 700 Td (Generated prose.) Tj ET"]);b.extend(b"\n/Encrypt 9 0 R\n");b}
// P6-S05 instruction 7: one generatable sample per blueprint-referenced input —
// every supported Format variant plus the detection-relevant variants (scanned and
// encrypted PDF, XHTML, legacy XLS, ODP detect-only, one header per image kind).
// Enumerated 2026-09-24 against blueprint P0–P5 testing requirements; anything with
// no entry here is a gap to flag, not silent skippage.
fn samples()->Vec<(&'static str,Vec<u8>)>{vec![
("basic.pdf",common::pdf_streams(&["BT /F1 12 Tf 30 700 Td (Generated prose.) Tj ET"])),
("scanned.pdf",common::pdf_streams(&["0 0 0 rg 10 10 100 100 re f"])),
("encrypted.pdf",encrypted_pdf()),
("basic.docx",common::docx("<w:p><w:r><w:t>Generated sample</w:t></w:r></w:p>")),
("basic.epub",common::epub()),
("basic.odt",common::odt()),
("sample.odp",common::zip(&[("mimetype",b"application/vnd.oasis.opendocument.presentation")])),
("basic.pptx",common::pptx()),
("basic.xlsx",common::xlsx()),
("legacy.xls",common::xls()),
("basic.ods",common::ods()),
("basic.txt",b"Generated prose.\n".to_vec()),
("rich.md",b"# Title\n\n## Section\n\nSome text with a [link](https://example.org).\n".to_vec()),
("basic.csv",b"name,value\na,1\n".to_vec()),
("basic.tsv",b"name\tvalue\na\t1\n".to_vec()),
("article.html",b"<html><body><h1>Hi</h1><p>Body.</p></body></html>".to_vec()),
("article.xhtml",b"<?xml version=\"1.0\"?><html><body><p>Hi</p></body></html>".to_vec()),
("basic.rtf",br"{\rtf1\ansi Hello}".to_vec()),
("header-only.png",common::png()),
("header-only.gif",b"GIF89a\x01\x00\x01\x00\x00\x00\x00;".to_vec()),
("header-only.jpg",b"\xff\xd8\xff\xe0\x00\x10JFIF".to_vec()),
("header-only.bmp",b"BM\x1a\x00\x00\x00\x00\x00\x00\x00".to_vec()),
("header-only.tiff",b"II*\x00\x08\x00\x00\x00".to_vec()),
("header-only.webp",b"RIFF\x04\x00\x00\x00WEBP".to_vec()),
("header-only.jp2",b"\x00\x00\x00\x0cjP  \x0d\x0a\x87\x0a".to_vec()),
]}
#[test]
#[ignore="User explicitly generates samples in samples/generated"]
fn write_samples(){let dir=std::path::Path::new("samples/generated");std::fs::create_dir_all(dir).unwrap();for(name,bytes)in samples(){std::fs::write(dir.join(name),bytes).unwrap();}}
