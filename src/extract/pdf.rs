use super::ExtractCtx;
use crate::{config::OcrMode,error::{ConvertError,Result},model::*};
use std::collections::HashSet;
pub const MAX_PAGES:usize=10_000;
pub const MAX_STREAM:usize=64*1024*1024;
pub const APPROXIMATE:&str="PDF default path: approximate reading order; columns, tables, images and font metrics are not reconstructed";
pub const OCR_BUILD:&str="OCR of scanned PDF pages requires the pdf-layout build; this is the default build. Rebuild with --features pdf-layout, or convert standalone images directly.";
pub(crate) fn preflight(bytes:&[u8])->Result<lopdf::Document>{
    if !bytes.starts_with(b"%PDF-")||!bytes.windows(5).any(|b|b==b"%%EOF"){return Err(ConvertError::Corrupt("PDF structure could not be loaded".into()))}
    if bytes.windows(8).any(|b|b==b"/Encrypt"){return Err(ConvertError::Encrypted("password-protected PDF is not supported".into()))}
    let doc=std::panic::catch_unwind(||lopdf::Document::load_mem(bytes)).map_err(|_|ConvertError::Corrupt("PDF structure could not be loaded".into()))?.map_err(|_|ConvertError::Corrupt("PDF structure could not be loaded".into()))?;
    if doc.is_encrypted()||doc.trailer.has(b"Encrypt"){return Err(ConvertError::Encrypted("password-protected PDF is not supported".into()))}
    let n=doc.get_pages().len();if n==0||n>MAX_PAGES{return Err(ConvertError::Corrupt("PDF page count outside supported limits".into()))}Ok(doc)
}
pub fn extract_pdf(bytes:&[u8],ctx:&mut ExtractCtx<'_>)->Result<Document>{
    #[cfg(feature="pdf-layout")]{super::pdf_layout::extract_pdf_layout(bytes,ctx)}
    #[cfg(not(feature="pdf-layout"))]{extract_pdf_flat(bytes,ctx)}
}
fn decode_whole(bytes:&[u8])->Result<String>{
    // P4-S01 step 3 (correctness-critical path): CMap/CFF/Type1 font-encoding is
    // handled internally by pdf-extract, never reimplemented here. Verified against
    // the resolved pdf-extract 0.12.1 source: `pdf_extract::extract_text_from_mem`
    // (`buffer: &[u8]) -> Result<String, OutputError>`. pdf-extract's handling of
    // pathological/malformed embedded fonts has historically been inconsistent
    // across versions, so this call sits behind catch_unwind per architecture §9
    // ("no input may crash the process" regardless of originating dependency).
    let outcome=std::panic::catch_unwind(||pdf_extract::extract_text_from_mem(bytes));
    match outcome{
        Ok(Ok(text))=>Ok(text),
        Ok(Err(_))|Err(_)=>Err(ConvertError::Corrupt("PDF text could not be decoded".into())),
    }
}
fn has_text_ops(stream:&[u8])->bool{
    // P4-S01 step 4: byte-level presence scan for text-showing operators only
    // (no tokenization). Approximate signal, not a full classifier.
    stream.windows(2).any(|w|w==b"Tj"||w==b"TJ")||stream.contains(&b'\'')||stream.contains(&b'"')
}
fn attribute_pages<'a>(decoded: &'a str, page_count: usize) -> Vec<&'a str>{
    let parts: Vec<&'a str> = decoded.split('\x0c').collect();
    if page_count > 0 && parts.len() == page_count {
        parts
    } else {
        let mut v = vec![""; page_count];
        if page_count > 0 {
            v[0] = decoded;
        }
        v
    }
}
pub fn extract_pdf_flat(bytes:&[u8],ctx:&mut ExtractCtx<'_>)->Result<Document>{
    let source=preflight(bytes)?;let pages=source.get_pages();
    let mut doc=Document::default();doc.meta.page_count=Some(pages.len());doc.warnings.push(APPROXIMATE.into());
    let decoded=decode_whole(bytes)?;
    // pdf-extract conventionally separates pages with form-feed; attribute
    // per-page only when the split count matches lopdf's page-tree enumeration
    // (P4-S02 owns any deeper rework of this attribution).
    // ONE decompressed fetch per page; the same bytes feed the presence scan.
    // (Previously the pre-scan loop and per-page decode each decompressed.)
    let mut fetched:Vec<(u32,Vec<u8>)>=Vec::with_capacity(pages.len());let mut total=0usize;
    for(number,id)in &pages{
        let stream=source.get_page_content_with_limit(*id,MAX_STREAM).map_err(|_|ConvertError::Corrupt("PDF page content could not be decoded within limits".into()))?;
        total=total.saturating_add(stream.len());
        if total>200*1024*1024{return Err(ConvertError::Corrupt("PDF total content limit exceeded".into()))}
        fetched.push((*number,stream));
    }
    let mut scanned=HashSet::new();
    for(number,stream)in &fetched{
        if !has_text_ops(stream)&&stream.iter().filter(|b|!b.is_ascii_whitespace()).count()>8{scanned.insert(*number);}
    }
    let attr = attribute_pages(&decoded, fetched.len());
    let title = doc.meta.title.clone();
    for(i,(number,_))in fetched.iter().enumerate(){
        doc.blocks.push(Block::Boundary(Boundary::Page(*number as usize)));
        if scanned.contains(number){
            doc.blocks.push(scanned_block(*number as usize));
        }else{
            let text = attr[i];
            if !text.trim().is_empty(){doc.blocks.extend(flat_blocks(text,title.as_deref()))}
        }
    }
    // P5-S03/AD-15: default build has no rasterizer (only pdfium can bitmap a page),
    // so scanned pages stay Placeholder under ANY --ocr mode (Off/Auto/Force) with
    // the explicit OCR_BUILD warning when OCR was requested — never fatal, never silent.
    if !scanned.is_empty(){doc.warnings.push(format!("{} PDF page(s) need OCR",scanned.len()));if ctx.options.ocr_mode!=OcrMode::Off{doc.warnings.push(OCR_BUILD.into())}}
    Ok(doc)
}
pub(crate) fn scanned_block(n:usize)->Block{Block::Placeholder{kind:PlaceholderKind::ScannedPage,label:Some(format!("page {n}")),asset:None}}
fn is_list_line(s:&str)->bool{let t=s.trim_start();t.starts_with("• ")||t.starts_with("‣ ")||t.starts_with("▪ ")||t.starts_with("* ")||t.starts_with("- ")||{let n=t.bytes().take_while(u8::is_ascii_digit).count();n>0&&n<=9&&t.get(n..).map(|r|r.starts_with(". ")||r.starts_with(") ")).unwrap_or(false)}}
// P4-S02 heading heuristic (explicitly weaker than Stage 4.B's font-metric
// approach in pdf_layout.rs: no font sizes are available on the default path,
// so every candidate becomes level 2 and reading order stays approximate).
fn flat_blocks(s:&str,title:Option<&str>)->Vec<Block>{let normalized=s.replace("\r\n","\n").replace('\r',"\n");normalized.split("\n\n").filter(|p|!p.trim().is_empty()).map(|p|{
 // A11 decision (P4-S02 steps 2–3): single internal newlines collapse to spaces
 // (PDF wraps at visual width) UNLESS a line is a list item or tabular
 // (≥2-space runs) — then breaks stay. Heading candidates exclude list items;
 // a candidate matching Meta.title becomes level 1, others level 2.
 let lines=p.lines().map(str::trim).filter(|l|!l.is_empty()).collect::<Vec<_>>();
 if lines.len()==1{let line=lines[0];if !is_list_line(line)&&line.chars().count()<=80&&!line.ends_with(['.',',',';',':','!','?']){let level=if title==Some(line){1}else{2};return Block::Heading{level,inlines:vec![Inline::Text(line.into())]}}}
 let keep_breaks=lines.iter().any(|l|is_list_line(l)||l.contains("  "));
 Block::Paragraph(vec![Inline::Text(if keep_breaks{lines.join("\n")}else{lines.join(" ") }.into())])}).collect()}
#[cfg(test)]mod tests{use super::*;fn para_text(b:&Block)->String{match b{Block::Paragraph(xs)=>inline_text(xs),_=>String::new()}}
#[test]fn flat_heading_is_heuristic(){assert!(matches!(&flat_blocks("Heading\n\nA sentence.",None)[0],Block::Heading{level:2,..}));}
#[test]fn flat_title_match_is_level_one(){let bs=flat_blocks("My Doc\n\nBody text here.",Some("My Doc"));assert!(matches!(&bs[0],Block::Heading{level:1,..}));}
#[test]fn flat_list_and_tabular_keep_breaks(){let bs=flat_blocks("• a\n• b",None);assert!(matches!(&bs[0],Block::Paragraph(_)));assert!(para_text(&bs[0]).contains('\n'));let bs=flat_blocks("a  b\nc  d",None);assert!(para_text(&bs[0]).contains('\n'));let bs=flat_blocks("wrapped line one\ncontinued here",None);assert!(!para_text(&bs[0]).contains('\n'));let bs=flat_blocks("• solo\n\nNext.",None);assert!(matches!(&bs[0],Block::Paragraph(_)));}
#[test]fn attribute_aligned_two_pages(){let d="a\x0cb";let a=attribute_pages(d,2);assert_eq!(a,vec!["a","b"]);}
#[test]fn attribute_nonaligned_single_chunk(){let d="whole doc";let a=attribute_pages(d,3);assert_eq!(a.len(),3);assert_eq!(a[0],"whole doc");assert_eq!(a[1],"");assert_eq!(a[2],"");}
#[test]fn attribute_empty_decode(){let d="";let a=attribute_pages(d,2);assert_eq!(a.len(),2);assert!(a.iter().all(|s|s.is_empty()));}
#[test]fn attribute_trailing_formfeed_is_nonaligned(){let d="a\x0cb\x0c";let a=attribute_pages(d,2);assert_eq!(a.len(),2);assert_eq!(a[0],d);assert_eq!(a[1],"");}}
