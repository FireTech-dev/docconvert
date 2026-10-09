pub mod pdf;
#[cfg(feature="pdf-layout")]
pub mod pdf_layout;
pub mod image;
pub mod package;
pub mod text;
pub mod csv;
pub mod html;
pub mod ooxml;
pub mod docx;
pub mod epub;
pub mod odt;
pub mod rtf;
pub mod xlsx;
pub mod ods;
pub mod pptx;
use crate::{assets::AssetManager,config::Options,detect::Detection,error::{ConvertError,Result},model::*};
use std::path::Path;
pub struct ExtractCtx<'a>{pub options:&'a Options,pub assets:&'a mut AssetManager,pub warnings:&'a mut Vec<String>}
pub fn cell_ref(row:u32,col:u32)->String{let mut n=col as u64+1;let mut s=String::new();while n>0{n-=1;s.insert(0,(b'A'+(n%26)as u8)as char);n/=26}format!("{s}{}",row as u64+1)}
pub fn parse_ref(s:&str)->Option<(u32,u32)>{let s=s.replace('$',"");let n=s.chars().take_while(|c|c.is_ascii_alphabetic()).count();if n==0{return None}let mut c=0u32;for b in s[..n].bytes(){c=c.checked_mul(26)?.checked_add((b.to_ascii_uppercase()-b'A'+1)as u32)?}Some((s[n..].parse::<u32>().ok()?.checked_sub(1)?,c.checked_sub(1)?))}
pub fn sheet_blocks(sheet:&Sheet,opts:&Options)->Vec<Block>{
 if sheet.hidden&&!opts.include_hidden_sheets{return vec![]}
 let mut blocks=vec![Block::Boundary(Boundary::Worksheet(sheet.name.clone())),Block::Heading{level:2,inlines:vec![Inline::Text(format!("Worksheet: {}",sheet.name))]}];
 if sheet.max_row<200&&sheet.max_col<20{
 let mut rows=vec![];for r in 0..=sheet.max_row{let mut row=vec![];for c in 0..=sheet.max_col{let mut cell=TableCell::default();let mut covered=false;for range in &sheet.merged{if let Some((a,b))=range.split_once(':').and_then(|(a,b)|Some((parse_ref(a)?,parse_ref(b)?))){if r>=a.0&&r<=b.0&&c>=a.1&&c<=b.1{if(r,c)==a{cell.colspan=(b.1-a.1+1)as usize;cell.rowspan=(b.0-a.0+1)as usize}else{covered=true}}}}if covered{continue}
 if let Some(v)=sheet.cells.get(&(r,c)){cell.inlines=if let Some(url)=&v.hyperlink{vec![Inline::Link{url:url.clone(),children:vec![Inline::Text(v.display.clone())]}]}else{vec![Inline::Text(v.display.clone())]};if matches!(v.kind,CellKind::Number){cell.align=Align::Right}}row.push(cell)}rows.push(row)}let header=if rows.is_empty(){vec![]}else{rows.remove(0)};blocks.push(Block::Table(Table{header,rows,..Table::default()}));
 }else{
 // Sparse, enormous dimensions must not force a dense allocation or iteration.
 let area=(sheet.max_row as u64+1).saturating_mul(sheet.max_col as u64+1);let mut w=::csv::Writer::from_writer(Vec::new());
 if area<=1_000_000{for r in 0..=sheet.max_row{let _=w.write_record((0..=sheet.max_col).map(|c|sheet.cells.get(&(r,c)).map(|v|v.display.as_str()).unwrap_or("")));}}
 else{let _=w.write_record(["Cell","Value"]);for v in sheet.cells.values(){let _=w.write_record([v.reference.as_str(),v.display.as_str()]);}}
 let bytes=w.into_inner().unwrap_or_default();blocks.push(Block::CodeBlock{language:Some("csv".into()),text:String::from_utf8_lossy(&bytes).into_owned()});blocks.push(Block::Paragraph(vec![Inline::Text(format!("{} rows, {} columns, {} non-empty cells; large/irregular sheet exported as CSV listing",sheet.max_row as u64+1,sheet.max_col as u64+1,sheet.cells.len()))]));
 }
 if opts.preserve_formulas{let fs=sheet.cells.values().filter_map(|c|c.formula.as_ref().map(|f|(c.reference.clone(),f.clone()))).collect::<Vec<_>>();if !fs.is_empty(){blocks.push(Block::FormulaList(fs))}}
 if opts.include_comments&&!sheet.comments.is_empty(){blocks.push(Block::List{ordered:false,start:None,items:sheet.comments.iter().map(|(r,t)|ListItem{level:0,blocks:vec![Block::Paragraph(vec![Inline::Text(format!("{r}: {t}"))])]}).collect()})}
 for title in &sheet.charts{blocks.push(Block::Placeholder{kind:PlaceholderKind::Chart,label:title.clone(),asset:None})}blocks
}
fn push_once(warnings:&mut Vec<String>,message:String){if !warnings.iter().any(|w|*w==message){warnings.push(message)}}
// Note: extract_xlsx/extract_ods lower their workbook and extract_file lowers
// again, so these pushes must be idempotent — never emit the same warning twice.
pub fn lower_workbook(d:&mut Document,opts:&Options){if let Some(w)=&d.workbook{d.blocks.clear();let mut hidden=0;for s in &w.sheets{if s.hidden&&!opts.include_hidden_sheets{hidden+=1;continue}if s.max_row>=200||s.max_col>=20{push_once(&mut d.warnings,"large/irregular sheet exported as CSV listing".into())}d.blocks.extend(sheet_blocks(s,opts))}if hidden>0{push_once(&mut d.warnings,format!("{hidden} hidden sheets skipped"))}}}
pub fn extract_file(path:&Path,detection:&Detection,options:&Options)->Result<Document>{
 if detection.encrypted{return Err(ConvertError::Encrypted("file is encrypted; password-protected files are not supported".into()))}
 let bytes=package::read_capped(path,options.max_file_bytes)?;let meta=Meta{source_filename:path.file_name().and_then(|s|s.to_str()).unwrap_or("input").into(),source_format:detection.format,..Meta::default()};
 let mut assets=AssetManager::default();let mut warnings=vec![];let mut ctx=ExtractCtx{options,assets:&mut assets,warnings:&mut warnings};
 let mut d=match detection.format{
 Format::Txt=>text::extract_txt(&bytes,meta.clone()),Format::Markdown=>{let base=package::parent_or_dot(path).canonicalize()?;let mut loader=|src:&str,alt:&str|{let name=package::normalize("",src)?;let file=base.join(name).canonicalize().ok()?;if !file.starts_with(&base)||std::fs::metadata(&file).ok()?.len()>package::MAX_ENTRY as u64{return None}let bytes=std::fs::read(file).ok()?;Some(ctx.assets.add(bytes,"Markdown image",Some(alt.into())))};text::extract_markdown_with_loader(&bytes,meta.clone(),&mut loader)},Format::Csv|Format::Tsv=>csv::extract_csv(&bytes,meta.clone(),if detection.format==Format::Tsv{b'\t'}else{b','},options),
 Format::Html=>{let base=package::parent_or_dot(path).canonicalize()?;let mut loader=|src:&str|{let name=package::normalize("",src)?;let file=base.join(name).canonicalize().ok()?;if !file.starts_with(&base)||std::fs::metadata(&file).ok()?.len()>package::MAX_ENTRY as u64{return None}std::fs::read(file).ok()};html::extract_html(&bytes,meta.clone(),&mut loader,&mut ctx)},
 Format::Docx=>docx::extract_docx(&bytes,&mut ctx)?,Format::Epub=>epub::extract_epub(&bytes,&mut ctx)?,Format::Odt=>odt::extract_odt(&bytes,&mut ctx)?,Format::Rtf=>rtf::extract_rtf(&bytes,&mut ctx)?,Format::Xlsx|Format::Xls=>xlsx::extract_xlsx(&bytes,&mut ctx)?,Format::Ods=>ods::extract_ods(&bytes,&mut ctx)?,Format::Pptx=>pptx::extract_pptx(&bytes,&mut ctx)?,
 Format::Doc|Format::Odp=>return Err(ConvertError::Unsupported(format!("{} conversion unsupported; legacy DOC/PPT and ODP are detect-and-report only",detection.format))),
 Format::Pdf=>pdf::extract_pdf(&bytes,&mut ctx)?,Format::Image=>image::extract_image(&bytes,meta.clone(),detection.image_kind.ok_or_else(||ConvertError::Corrupt("image signature not recognized".into()))?,&mut ctx)?,Format::Unknown=>return Err(ConvertError::Unsupported(format!("unknown file format: {}",detection.notes.join("; ")))),
 };
 d.meta.source_filename=meta.source_filename;d.meta.source_format=meta.source_format;d.assets=assets.assets;d.warnings.extend(warnings);d.warnings.extend(detection.notes.clone());lower_workbook(&mut d,options);if options.describe_images{d.warnings.push("image description is not part of the offline core".into())}Ok(d)
}
