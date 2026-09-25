use crate::{error::{ConvertError,Result},model::{Format,ImageKind}};
use std::{fs::File,io::Read,path::Path};
#[derive(Debug,Clone)]
pub struct Detection {pub format:Format,pub mime:Option<String>,pub extension:String,pub extension_match:bool,pub image_kind:Option<ImageKind>,pub notes:Vec<String>,pub encrypted:bool}
pub fn image_kind(b:&[u8])->Option<ImageKind>{if b.starts_with(b"\x89PNG"){Some(ImageKind::Png)}else if b.starts_with(b"\xff\xd8\xff"){Some(ImageKind::Jpeg)}else if b.starts_with(b"GIF87a")||b.starts_with(b"GIF89a"){Some(ImageKind::Gif)}else if b.starts_with(b"BM"){Some(ImageKind::Bmp)}else if b.starts_with(b"II*\0")||b.starts_with(b"MM\0*"){Some(ImageKind::Tiff)}else if b.starts_with(b"RIFF")&&b.get(8..12)==Some(b"WEBP"){Some(ImageKind::Webp)}else if b.starts_with(b"\0\0\0\x0cjP"){Some(ImageKind::Jp2)}else{None}}
pub fn expected_formats(ext:&str)->Vec<Format>{use Format::*;match ext {"pdf"=>vec![Pdf],"docx"=>vec![Docx],"doc"=>vec![Doc],"odt"=>vec![Odt],"ods"=>vec![Ods],"odp"=>vec![Odp],"rtf"=>vec![Rtf],"epub"=>vec![Epub],"html"|"htm"|"xhtml"|"xht"=>vec![Html],"txt"|"md"|"markdown"=>vec![Txt,Markdown],"pptx"=>vec![Pptx],"xlsx"=>vec![Xlsx],"xls"=>vec![Xls],"csv"=>vec![Csv],"tsv"=>vec![Tsv],"png"|"jpg"|"jpeg"|"gif"|"bmp"|"tif"|"tiff"|"webp"|"jp2"=>vec![Image],_=>vec![]}}
pub fn detect_file(path:&Path)->Result<Detection>{let mut head=Vec::new();File::open(path)?.take(65536).read_to_end(&mut head)?;detect_header(&head,path)}
fn detect_header(b:&[u8],path:&Path)->Result<Detection>{
 use Format::*;if b.is_empty(){return Err(ConvertError::Corrupt("file is empty".into()))}
 let ext=path.extension().and_then(|s|s.to_str()).unwrap_or("").to_lowercase();let mut notes=vec![];let mut encrypted=false;let kind=image_kind(b);let mut mime=None;
 let format=if b.starts_with(b"%PDF"){encrypted=b.windows(8).any(|w|w==b"/Encrypt");mime=Some("application/pdf".into());Pdf}
  else if b.starts_with(b"PK\x03\x04") || b.starts_with(b"PK\x05\x06"){
 let mut zip=zip::ZipArchive::new(File::open(path)?).map_err(|_|ConvertError::Corrupt("incomplete or damaged ZIP archive".into()))?;
 if zip.len()>10000{return Err(ConvertError::Corrupt("ZIP entry count exceeds limit".into()))}
 let mut names=std::collections::HashSet::new();let mut mt=None;
 for i in 0..zip.len(){let entry=zip.by_index(i).map_err(|_|ConvertError::Corrupt("unreadable ZIP entry".into()))?;let name=entry.name().to_ascii_lowercase();if name=="mimetype"{let mut raw=Vec::new();entry.take(256).read_to_end(&mut raw).map_err(|_|ConvertError::Corrupt("unreadable ZIP mimetype".into()))?;mt=Some(String::from_utf8_lossy(&raw).trim().to_owned())}names.insert(name);}
 match mt.as_deref(){Some("application/epub+zip")=>Epub,Some("application/vnd.oasis.opendocument.text")=>Odt,Some("application/vnd.oasis.opendocument.spreadsheet")=>Ods,Some("application/vnd.oasis.opendocument.presentation")=>Odp,_=>if names.contains("[content_types].xml"){if names.contains("word/document.xml"){Docx}else if names.contains("ppt/presentation.xml"){Pptx}else if names.contains("xl/workbook.xml"){Xlsx}else{notes.push("OOXML package with unrecognized parts".into());Unknown}}else{notes.push("ZIP archive without recognizable document structure".into());Unknown}}
 }else if b.starts_with(b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1"){
 let needle:Vec<u8>="EncryptedPackage".encode_utf16().flat_map(u16::to_le_bytes).collect();encrypted=b.windows(needle.len()).any(|w|w==needle);if encrypted{notes.push("encrypted Office package".into());Unknown}else{match ext.as_str(){"doc"=>Doc,"xls"=>Xls,_=>{notes.push("legacy binary Office format — detected, conversion unsupported".into());Unknown}}}
 }else if String::from_utf8_lossy(b).trim_start_matches('\u{feff}').trim_start().starts_with("{\\rtf"){Rtf}
 else if let Some(k)=kind{mime=Some(k.mime().into());Image}else{
 let s=String::from_utf8_lossy(b);let low=s.chars().take(2048).collect::<String>().to_lowercase();
 if low.contains("<!doctype html")||low.contains("<html"){if low.contains("<?xml"){notes.push("XHTML".into())}Html}
 else if low.trim_start().starts_with("<?xml"){notes.push("XML document without recognized schema".into());Unknown}
 else if s.chars().filter(|c|*c=='�'||*c=='\0').count()*100<=s.chars().count(){
  // AQ-008: CSV/TSV delimiter sniff — candidates `,`, `;`, `\t`, `|`; count occurrences in first line (text before first `\n`); highest wins; tie or zero → default (comma for .csv, tab for .tsv).
  match ext.as_str(){"csv"=>{let first=s.split('\n').next().unwrap_or("");let counts=[("comma",first.matches(',').count()),("semicolon",first.matches(';').count()),("tab",first.matches('\t').count()),("pipe",first.matches('|').count())];let max=counts.iter().map(|(_,n)|*n).max().unwrap_or(0);let name=if max==0||counts.iter().filter(|(_,n)|*n==max).count()!=1{"comma"}else{counts.iter().find(|(_,n)|*n==max).map(|(n,_)|*n).unwrap()};notes.push(format!("CSV delimiter: {name}").into());Csv},"tsv"=>{let first=s.split('\n').next().unwrap_or("");let counts=[("comma",first.matches(',').count()),("semicolon",first.matches(';').count()),("tab",first.matches('\t').count()),("pipe",first.matches('|').count())];let max=counts.iter().map(|(_,n)|*n).max().unwrap_or(0);let name=if max==0||counts.iter().filter(|(_,n)|*n==max).count()!=1{"tab"}else{counts.iter().find(|(_,n)|*n==max).map(|(n,_)|*n).unwrap()};notes.push(format!("TSV delimiter: {name}").into());Tsv},"md"|"markdown"=>Markdown,_=>if s.lines().filter(|l|l.starts_with("# ")).count()>=2||s.contains("```")||s.contains("]("){Markdown}else{Txt}}
 }else{notes.push("no recognizable signature".into());Unknown}};
 let extension_match=expected_formats(&ext).contains(&format);if !extension_match{notes.push(format!("extension \".{ext}\" does not match detected content ({format})"))}
 Ok(Detection{format,mime,extension:ext,extension_match,image_kind:kind,notes,encrypted})
}
