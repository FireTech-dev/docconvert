//! Feature-gated PDFium boundary and deterministic, approximate layout heuristics.
use super::{ExtractCtx,pdf};
use crate::{config::OcrMode,error::{ConvertError,Result},model::*};
use pdfium_render::prelude::*;
use std::{collections::{BTreeMap,BTreeSet},path::{Path,PathBuf}};
#[derive(Debug,Clone)]
pub struct Glyph{pub ch:char,pub bbox:(f32,f32,f32,f32),pub font_size:f32,pub font_name:String}
#[derive(Debug,Clone)]
pub struct Line{pub glyphs:Vec<Glyph>,pub y:f32,pub x_range:(f32,f32),pub is_header_footer:bool}
impl Line{
    fn height(&self)->f32{median(self.glyphs.iter().map(|g|(g.bbox.3-g.bbox.1).abs()).collect()).max(1.)}
    fn size(&self)->f32{median(self.glyphs.iter().map(|g|g.font_size).collect())}
    fn text(&self)->String{let mut out=String::new();let mut right=None;let width=median(self.glyphs.iter().map(|g|g.bbox.2-g.bbox.0).collect()).max(1.);for g in &self.glyphs{if let Some(x)=right{if g.bbox.0-x>width*0.65&&!out.ends_with(' ')&&!g.ch.is_whitespace(){out.push(' ')}}out.push(g.ch);right=Some(g.bbox.2)}out.trim().into()}
    fn mono(&self)->bool{!self.glyphs.is_empty()&&self.glyphs.iter().filter(|g|{let n=g.font_name.to_lowercase();n.contains("mono")||n.contains("courier")||n.contains("consolas")||n.contains("menlo")}).count()*5>=self.glyphs.len()*4}
    fn bold(&self)->bool{self.glyphs.iter().filter(|g|g.font_name.to_lowercase().contains("bold")).count()*2>self.glyphs.len()}
}
fn median(mut v:Vec<f32>)->f32{v.retain(|f|f.is_finite()&&*f>0.);v.sort_by(f32::total_cmp);v.get(v.len()/2).copied().unwrap_or(1.)}
fn line(mut glyphs:Vec<Glyph>)->Line{glyphs.sort_by(|a,b|a.bbox.0.total_cmp(&b.bbox.0));let y=glyphs.iter().map(|g|(g.bbox.1+g.bbox.3)*0.5).sum::<f32>()/glyphs.len().max(1)as f32;let left=glyphs.first().map(|g|g.bbox.0).unwrap_or(0.);let right=glyphs.iter().map(|g|g.bbox.2).fold(left,f32::max);Line{glyphs,y,x_range:(left,right),is_header_footer:false}}
pub fn cluster_lines(glyphs:&[Glyph])->Vec<Line>{let tolerance=median(glyphs.iter().map(|g|g.bbox.3-g.bbox.1).collect())*0.3;let mut sorted=glyphs.to_vec();sorted.sort_by(|a,b|((b.bbox.1+b.bbox.3)*0.5).total_cmp(&((a.bbox.1+a.bbox.3)*0.5)));let mut groups:Vec<Vec<Glyph>>=vec![];for glyph in sorted{let y=(glyph.bbox.1+glyph.bbox.3)*0.5;if let Some(last)=groups.last_mut(){let first=&last[0];if ((first.bbox.1+first.bbox.3)*0.5-y).abs()<=tolerance{last.push(glyph);continue}}groups.push(vec![glyph]);}groups.into_iter().map(line).filter(|l|!l.text().is_empty()).collect()}
fn gaps(l:&Line)->Vec<(f32,f32)>{let width=median(l.glyphs.iter().map(|g|g.bbox.2-g.bbox.0).collect());l.glyphs.windows(2).filter_map(|g|{let gap=g[1].bbox.0-g[0].bbox.2;if gap>width*0.65{Some((g[0].bbox.2,g[1].bbox.0))}else{None}}).collect()}
fn cuts(lines:&[Line],page_width:f32)->Vec<f32>{
    let all:Vec<f32>=lines.iter().flat_map(gaps).map(|(a,b)|b-a).collect();let char_width=median(lines.iter().flat_map(|l|l.glyphs.iter()).map(|g|g.bbox.2-g.bbox.0).collect());let threshold=(median(all).min(char_width*2.)*2.).max(char_width*3.);let bin=(page_width/100.).max(2.);let mut votes:BTreeMap<i32,usize>=BTreeMap::new();
    for l in lines{let mut seen=BTreeSet::new();for(a,b)in gaps(l){if b-a>threshold{for k in ((a+bin)/bin)as i32..((b-bin)/bin)as i32{seen.insert(k);}}}for k in seen{*votes.entry(k).or_default()+=1;}}
    let mut runs:Vec<Vec<i32>>=vec![];for(k,n)in votes{if n*5>=lines.len()*2{if let Some(last)=runs.last_mut(){if last.last()==Some(&(k-1)){last.push(k);continue}}runs.push(vec![k]);}}
    runs.into_iter().map(|r|r[r.len()/2]as f32*bin).filter(|x|*x>page_width*0.1&&*x<page_width*0.9).take(7).collect()
}
pub fn order_reading(lines:Vec<Line>,page_width:f32)->Vec<Line>{let boundaries=cuts(&lines,page_width);if boundaries.is_empty(){let mut lines=lines;lines.sort_by(|a,b|b.y.total_cmp(&a.y));return lines}let mut spanning=vec![];let mut columns:Vec<Vec<Line>>=vec![vec![];boundaries.len()+1];for l in lines{let span=boundaries.iter().any(|x|l.x_range.0<*x&&l.x_range.1>*x&&!gaps(&l).iter().any(|(a,b)|a<x&&b>x));if span{spanning.push(l);continue}let mut parts:Vec<Vec<Glyph>>=vec![vec![];columns.len()];for g in l.glyphs{let col=boundaries.partition_point(|x|*x<(g.bbox.0+g.bbox.2)*0.5);parts[col].push(g)}for(c,gs)in parts.into_iter().enumerate(){if !gs.is_empty(){columns[c].push(line(gs))}}}spanning.sort_by(|a,b|b.y.total_cmp(&a.y));for c in &mut columns{c.sort_by(|a,b|b.y.total_cmp(&a.y))}let mut out=vec![];for full in spanning{for c in &mut columns{let n=c.partition_point(|l|l.y>full.y);out.extend(c.drain(..n))}out.push(full)}for c in columns{out.extend(c)}out}
fn normalized(s:&str)->String{s.split_whitespace().map(|s|s.chars().filter(|c|!c.is_ascii_digit()).map(|c|c.to_ascii_lowercase()).collect::<String>()).collect::<Vec<_>>().join(" ")}
fn strip_repetition(pages:&mut [PageData]){if pages.len()<2{return}let mut counts:BTreeMap<(i32,String),usize>=BTreeMap::new();for p in pages.iter(){let keys:BTreeSet<_>=p.lines.iter().filter(|l|l.y<p.height*0.1||l.y>p.height*0.9).map(|l|(((l.y/p.height)*100.).round()as i32,normalized(&l.text()))).collect();for key in keys{*counts.entry(key).or_default()+=1}}let n=pages.len();for p in pages{for l in &mut p.lines{let key=(((l.y/p.height)*100.).round()as i32,normalized(&l.text()));l.is_header_footer=(l.y<p.height*0.1||l.y>p.height*0.9)&&counts.get(&key).copied().unwrap_or(0)*5>=n*3;}}}
#[derive(Clone,Copy)]struct Segment{a:(f32,f32),b:(f32,f32)}
struct Picture{asset:usize,bbox:(f32,f32,f32,f32)}
struct PageData{lines:Vec<Line>,width:f32,height:f32,segments:Vec<Segment>,pictures:Vec<Picture>,ocr:Option<Vec<Block>>,scanned:bool}
fn library_file(path:PathBuf)->PathBuf{if path.is_dir(){path.join(if cfg!(target_os="windows"){ "pdfium.dll" }else if cfg!(target_os="macos"){ "libpdfium.dylib" }else{"libpdfium.so"})}else{path}}
fn binding_error()->ConvertError{ConvertError::Config("pdfium library not found or incompatible. Set PDFIUM_DYNAMIC_LIB_PATH or --pdfium-lib-path, or install libpdfium.".into())}
// pdfium-render promotes bindings into a process-global OnceCell: only the first
// bind per process succeeds, later binds fail with AlreadyInitialized. A CLI batch
// converting N PDFs must therefore share one Pdfium (upstream design: one instance,
// many documents) — binding per file broke every PDF after the first in one process.
static PDFIUM:std::sync::OnceLock<Pdfium>=std::sync::OnceLock::new();
static PDFIUM_SOURCE:std::sync::Mutex<Option<PathBuf>>=std::sync::Mutex::new(None);
fn shared_pdfium(configured:Option<PathBuf>)->Result<&'static Pdfium>{
    let resolved=configured.map(library_file);
    // Validate an explicitly configured path on EVERY call, so a typo fails
    // actionably even after the global instance already exists.
    if let Some(path)=&resolved{if !path.is_file(){return Err(binding_error())}}
    {
        let mut source=PDFIUM_SOURCE.lock().unwrap_or_else(|p|p.into_inner());
        match (&*source,&resolved){
            (Some(a),Some(b))if a!=b=>return Err(ConvertError::Config("pdfium library path differs from the already-initialized library; one native library per process. Restart with a single --pdfium-lib-path.".into())),
            (None,Some(b))=>*source=Some(b.clone()),
            _=>{}
        }
    }
    // Slow path runs under the caller's NATIVE serialization. `OnceLock::get_or_try_init`
    // is still unstable, so init by hand: bind failures leave both cells unset and the
    // next file retries instead of poisoning (a racing loser Pdfium drops harmlessly —
    // `Drop` only frees an optional font provider, never the library).
    if let Some(pdfium)=PDFIUM.get(){return Ok(pdfium)}
    let bindings=if let Some(path)=resolved{Pdfium::bind_to_library(path).map_err(|_|binding_error())?}else{let adjacent=std::env::current_exe().ok().and_then(|p|p.parent().map(Path::to_path_buf)).map(library_file);if let Some(path)=adjacent.filter(|p|p.is_file()){Pdfium::bind_to_library(path).map_err(|_|binding_error())?}else{Pdfium::bind_to_system_library().map_err(|_|binding_error())?}};
    Ok(PDFIUM.get_or_init(||Pdfium::new(bindings)))
}
pub fn extract_pdf_layout(bytes:&[u8],ctx:&mut ExtractCtx<'_>)->Result<Document>{
    // Serialize the native library lifecycle; file-level workers still parallelize other formats.
    static NATIVE:std::sync::Mutex<()>=std::sync::Mutex::new(());
    let _native=NATIVE.lock().unwrap_or_else(|p|p.into_inner());
    // Finding fix (user-authorized): preflight's lopdf load rejects some valid
    // PDFs (e.g. incremental-update xref chains pdfium itself reads fine). An
    // Encrypted verdict still hard-stops here; a Corrupt verdict only warns and
    // lets PDFium attempt recovery — if it also fails, the preflight error is
    // returned, so truly corrupt files see the identical contract as before.
    let preflight_error=match pdf::preflight(bytes){
        Ok(_)=>None,
        Err(e)if matches!(e,ConvertError::Encrypted(_))=>return Err(e),
        Err(e)=>Some(e),
    };
    let configured=ctx.options.pdfium_lib_path.clone().or_else(||std::env::var_os("PDFIUM_DYNAMIC_LIB_PATH").map(PathBuf::from));
    let pdfium=shared_pdfium(configured)?;
    // A12: PDFium password/security refusals surface as Encrypted (same variant as
    // the default path), never misreported as Corrupt.
    let source=match pdfium.load_pdf_from_byte_slice(bytes,None){
        Ok(source)=>source,
        Err(e)=>return Err(preflight_error.unwrap_or_else(||match e{
            PdfiumError::PdfiumLibraryInternalError(PdfiumInternalError::PasswordError)|PdfiumError::PdfiumLibraryInternalError(PdfiumInternalError::SecurityError)=>ConvertError::Encrypted("password-protected PDF is not supported".into()),
            _=>ConvertError::Corrupt("PDFium could not load PDF".into()),
        })),
    };
    let recovered=preflight_error.is_some();
    let mut pages=vec![];let mut doc=Document::default();if recovered{doc.warnings.push("default structure scan could not load this PDF; layout recovery applied".into())}let mut total_glyphs=0usize;let mut total_pixels=0usize;let mut total_objects=0usize;
    for page in source.pages().iter(){
        let width=page.width().value;let height=page.height().value;if !width.is_finite()||!height.is_finite()||width<=0.||height<=0.{return Err(ConvertError::Corrupt("invalid PDF page dimensions".into()))}
        let text=page.text().map_err(|_|ConvertError::Corrupt("PDFium page text unavailable".into()))?;let chars=text.chars();let mut glyphs=vec![];
        for ch in chars.iter(){total_glyphs+=1;if total_glyphs>1_000_000||glyphs.len()>=100_000{return Err(ConvertError::Corrupt("PDF glyph limit exceeded".into()))}if let Some(c)=ch.unicode_char(){if c.is_control(){continue}let bounds=ch.loose_bounds().map_err(|_|ConvertError::Corrupt("PDF glyph bounds unavailable".into()))?;let bbox=(bounds.left().value,bounds.bottom().value,bounds.right().value,bounds.top().value);let size=ch.scaled_font_size().value;if [bbox.0,bbox.1,bbox.2,bbox.3,size].iter().any(|v|!v.is_finite()){return Err(ConvertError::Corrupt("invalid PDF glyph metrics".into()))}glyphs.push(Glyph{ch:c,bbox,font_size:size,font_name:format!("{}{}{}",ch.font_name(),if ch.font_is_fixed_pitch(){" mono"}else{""},if ch.font_is_bold_reenforced(){" bold"}else{""})});}}
        // Finding fix (user-authorized): some producers double-draw glyphs (faux
        // bold); PDFium reports both copies, which used to render as "AABB".
        // Drop exact duplicates (same char, near-identical box); legitimately
        // repeated characters sit at distinct positions and survive.
        let glyphs=drop_shadows(dedupe_glyphs(glyphs));
        let mut pictures=vec![];let mut segments=vec![];
        for object in page.objects().iter(){total_objects+=1;if total_objects>100_000{return Err(ConvertError::Corrupt("PDF object limit exceeded".into()))}
            if let Some(image)=object.as_image_object(){let bounds=image.bounds().map_err(|_|ConvertError::Corrupt("PDF image bounds unavailable".into()))?;let w=image.width().map_err(|_|ConvertError::Corrupt("PDF image dimensions unavailable".into()))?;let h=image.height().map_err(|_|ConvertError::Corrupt("PDF image dimensions unavailable".into()))?;let pixels=(w as i64).checked_mul(h as i64).filter(|n|w>0&&h>0&&*n<=20_000_000).ok_or_else(||ConvertError::Corrupt("PDF image pixel limit exceeded".into()))? as usize;total_pixels=total_pixels.saturating_add(pixels);if total_pixels>50_000_000{return Err(ConvertError::Corrupt("PDF total image pixel limit exceeded".into()))}
                let bitmap=image.get_processed_bitmap_with_size(&source,w,h).map_err(|_|ConvertError::Corrupt("PDF image bitmap unavailable".into()))?;let png=super::image::rgba_png(bitmap.width()as u32,bitmap.height()as u32,&bitmap.as_rgba_bytes())?;let asset=ctx.assets.add(png,"PDF image rendered to PNG",None);pictures.push(Picture{asset,bbox:(bounds.left().value,bounds.bottom().value,bounds.right().value,bounds.top().value)});
            }else if let Some(path)=object.as_path_object(){let path_segments=path.segments();let mut previous=None;let mut start=None;for s in path_segments.iter(){let point=(s.x().value,s.y().value);if s.segment_type()==PdfPathSegmentType::MoveTo{start=Some(point)}if s.segment_type()==PdfPathSegmentType::LineTo{if let Some(a)=previous{segments.push(Segment{a,b:point});}}if s.is_close(){if let Some(a)=start{segments.push(Segment{a:point,b:a});}}previous=Some(point);if segments.len()>20_000{return Err(ConvertError::Corrupt("PDF path segment limit exceeded".into()))}}}
        }
        let scanned=glyphs.iter().all(|g|g.ch.is_whitespace())&&(!pictures.is_empty()||!segments.is_empty());let mut ocr=None;
        if ctx.options.ocr_mode==OcrMode::Force||(scanned&&ctx.options.ocr_mode==OcrMode::Auto){let w=(width*2.).ceil();let h=(height*2.).ceil();if w*h>20_000_000.||w>i32::MAX as f32||h>i32::MAX as f32{return Err(ConvertError::Corrupt("PDF OCR raster pixel limit exceeded".into()))}let bitmap=page.render(w as i32,h as i32,None).map_err(|_|ConvertError::Corrupt("PDF OCR rasterization failed".into()))?;let png=super::image::rgba_png(bitmap.width()as u32,bitmap.height()as u32,&bitmap.as_rgba_bytes())?;if let Some(result)=crate::ocr::attempt_ocr(&png,ctx.options,&mut doc.warnings)?{if !result.text.trim().is_empty(){ocr=Some(super::text::extract_txt(result.text.as_bytes(),Meta::default()).blocks)}}}
        pages.push(PageData{lines:cluster_lines(&glyphs),width,height,segments,pictures,ocr,scanned});
    }
    doc.meta.page_count=Some(pages.len());if ctx.options.strip_headers_footers{strip_repetition(&mut pages)}let stripped=pages.iter().flat_map(|p|&p.lines).filter(|l|l.is_header_footer).count();if stripped>0{doc.warnings.push(format!("PDF layout removed {stripped} repeated header/footer lines; use --no-strip-headers-footers to retain"))}
    let mut frequencies:BTreeMap<i32,usize>=BTreeMap::new();for g in pages.iter().flat_map(|p|&p.lines).filter(|l|!l.is_header_footer).flat_map(|l|&l.glyphs){*frequencies.entry((g.font_size*2.).round()as i32).or_default()+=1}let body=frequencies.iter().max_by_key(|(_,n)|*n).map(|(s,_)|*s as f32/2.).unwrap_or(12.);let sizes:Vec<i32>=frequencies.keys().rev().filter(|s|**s as f32/2.>=body*1.15).copied().collect();
    for(n,p)in pages.into_iter().enumerate(){doc.blocks.push(Block::Boundary(Boundary::Page(n+1)));if let Some(ref blocks)=p.ocr{doc.blocks.extend(blocks.clone());for picture in p.pictures{doc.blocks.push(Block::Image{asset:picture.asset,alt:None,caption:None})}continue}if p.scanned{doc.blocks.push(pdf::scanned_block(n+1));doc.warnings.push(format!("PDF page {} needs OCR",n+1))}doc.blocks.extend(reconstruct(p,body,&sizes,&mut doc.warnings));}
    doc.warnings.push("PDF layout reconstruction is heuristic; math is literal Unicode, not LaTeX; extracted image pixels are re-encoded as PNG".into());Ok(doc)
}
fn dedupe_glyphs(mut glyphs:Vec<Glyph>)->Vec<Glyph>{
    // Tolerance 1.0pt: faux-bold copies at small sizes measured dx=0.74 dy=0.52.
    // Legitimate same-char advances run 3pt+ even at 6pt type (verified against
    // the 6.0 advance of the test helper and 4.6pt 9pt-body advances in the wild),
    // and stacked diacritics differ in char — so nothing real merges.
    glyphs.sort_by(|a,b|a.bbox.0.total_cmp(&b.bbox.0).then_with(||a.bbox.1.total_cmp(&b.bbox.1)).then_with(||a.ch.cmp(&b.ch)));
    let mut out:Vec<Glyph>=Vec::with_capacity(glyphs.len());
    for g in glyphs{
        let dup=out.last().map(|l|l.ch==g.ch&&(l.bbox.0-g.bbox.0).abs()<1.0&&(l.bbox.1-g.bbox.1).abs()<1.0&&(l.bbox.2-g.bbox.2).abs()<1.0&&(l.bbox.3-g.bbox.3).abs()<1.0).unwrap_or(false);
        if !dup{out.push(g)}
    }
    out
}
/// Drop-shadow copies: display type is often drawn twice with a fixed small
/// offset (measured: dx=1.96 dy=2.00 at 40pt, dx=0.74 dy=0.52 at 12pt on a real
/// cover), which otherwise renders as "AABB". A shadow is systematic: one
/// same-char offset shared across several distinct characters. Candidate bins
/// are tried largest-first (>=3 pairs each); the first whose disjointly paired
/// participants span >=3 distinct chars wins. Dotted leaders fail the
/// distinct-chars bar; small body text fails the 10pt size bar — both are
/// never merged.
fn drop_shadows(glyphs:Vec<Glyph>)->Vec<Glyph>{
    // Size bar at 10pt: subtitle faux-bold measured at 12pt with a systematic
    // (0.74, 0.52) offset. Below 10pt, advances shrink toward the 3pt pair
    // window, so small body text stays out entirely. At >=10pt, running text is
    // still safe: pair formation needs same-char neighbors within 3pt (body
    // advances run 4.6pt+), and even a pathological voter still needs >=3
    // distinct chars sharing one offset at >=40% participation.
    let big=glyphs.iter().enumerate().filter(|(_,g)|g.font_size>=10.).map(|(i,_)|i).collect::<Vec<_>>();
    if big.len()<4||big.len()>5000{return glyphs}
    // Bucket by char first: only same-char pairs can be shadow copies.
    let mut by_char:std::collections::BTreeMap<char,Vec<usize>>=std::collections::BTreeMap::new();
    for &i in &big{by_char.entry(glyphs[i].ch).or_default().push(i)}
    let mut votes:std::collections::BTreeMap<(i32,i32),Vec<(usize,usize)>>=std::collections::BTreeMap::new();
    for members in by_char.values(){
        for (k,&i) in members.iter().enumerate(){
            for &j in &members[k+1..]{
                let dx=glyphs[j].bbox.0-glyphs[i].bbox.0;let dy=glyphs[j].bbox.1-glyphs[i].bbox.1;
                if dx.abs()>3.||dy.abs()>3.{continue}
                let key=((dx*4.).round() as i32,(dy*4.).round() as i32);
                if key==(0,0){continue}
                votes.entry(key).or_default().push((i,j));
            }
        }
    }
    // Try bins largest-first, looping until no bin passes: one page can carry
    // several shadow layers at different offsets (measured: title (1.96,2.00)
    // plus subtitle (0.74,0.52) on one cover). A decoy bin (dotted leaders,
    // repeated ornaments) may outvote a real one, so every passing bin applies
    // in turn. Guards per bin: >=3 pairs, >=3 distinct chars among the
    // disjointly paired participants (leaders use one char).
    let mut alive=vec![true;glyphs.len()];
    loop{
        let mut bins=votes.iter().collect::<Vec<_>>();
        bins.sort_by(|a,b|b.1.len().cmp(&a.1.len()));
        let mut applied=false;
        for (_,pairs) in bins{
            let fresh=pairs.iter().filter(|(i,j)|alive[*i]&&alive[*j]).collect::<Vec<_>>();
            if fresh.len()<3{continue}
            let mut used=vec![false;glyphs.len()];let mut kept=vec![];
            for (i,j) in fresh{if !used[*i]&&!used[*j]{used[*i]=true;used[*j]=true;kept.push((*i,*j))}}
            let distinct=glyphs.iter().enumerate().filter(|(i,_)|used[*i]).map(|(_,g)|g.ch).collect::<std::collections::BTreeSet<_>>();
            if distinct.len()<3{continue}
            for (_,j) in kept{alive[j]=false}
            applied=true;
            break;
        }
        if !applied{break}
    }
    // Recompute the vote table from survivors each round would be cleaner; the
    // fresh-filter above is equivalent because dropped glyphs only shrink pairs.
    glyphs.into_iter().enumerate().filter(|(i,_)|alive[*i]).map(|(_,g)|g).collect()
}
fn math_base(g:&Glyph)->bool{let n=g.font_name.to_lowercase();n.contains("cambria math")||n.contains("stix")||n.contains("symbol")||n.contains("euclid math")||matches!(g.ch as u32,0x1d400..=0x1d7ff|0x2200..=0x22ff)}
fn is_arrow(c:char)->bool{matches!(c as u32,0x2190..=0x21ff)}
/// P4-S11: arrows (U+2190–U+21FF) count as math only adjacent to other
/// math-flagged glyphs — arrows alone are too common in prose to flag.
fn math_flags(glyphs:&[Glyph])->Vec<bool>{let mut flags=glyphs.iter().map(math_base).collect::<Vec<_>>();for i in 0..glyphs.len(){if !flags[i]&&is_arrow(glyphs[i].ch){flags[i]=(i>0&&flags[i-1])||(i+1<glyphs.len()&&flags[i+1])}}flags}
fn inlines(l:&Line)->Vec<Inline>{let flags=math_flags(&l.glyphs);if !flags.iter().any(|f|*f){let text=l.text();if l.bold(){vec![Inline::Styled{style:TextStyle{bold:true,..TextStyle::default()},children:vec![Inline::Text(text)]}]}else{vec![Inline::Text(text)]}}else{let mut out=vec![];let mut buffer=String::new();let mut current=false;for(i,g)in l.glyphs.iter().enumerate(){let m=flags[i];if m!=current&&!buffer.is_empty(){out.push(if current{Inline::MathInline(std::mem::take(&mut buffer))}else{Inline::Text(std::mem::take(&mut buffer))})}current=m;buffer.push(g.ch)}if !buffer.is_empty(){out.push(if current{Inline::MathInline(buffer)}else{Inline::Text(buffer)})}out}}
fn heading(l:&Line,lines:&[Line],body:f32)->bool{l.size()>=body*1.15&&l.text().chars().count()<=120&&lines.iter().filter(|other|(other.y-l.y).abs()>0.1&&other.x_range.0<l.x_range.1&&other.x_range.1>l.x_range.0).all(|other|(other.y-l.y).abs()>=l.height()*1.5)}
fn cell(l:&Line)->TableCell{TableCell{inlines:inlines(l),..TableCell::default()}}
fn axis(v:&mut Vec<f32>){v.sort_by(f32::total_cmp);v.dedup_by(|a,b|(*a-*b).abs()<1.);}
fn ruled(lines:&mut Vec<Line>,segments:&[Segment])->Option<(f32,Block)>{
    let horizontal:Vec<_>=segments.iter().filter(|s|(s.a.1-s.b.1).abs()<0.5&&(s.a.0-s.b.0).abs()>5.).collect();let vertical:Vec<_>=segments.iter().filter(|s|(s.a.0-s.b.0).abs()<0.5&&(s.a.1-s.b.1).abs()>5.).collect();let mut xs:Vec<_>=vertical.iter().map(|s|s.a.0).collect();let mut ys:Vec<_>=horizontal.iter().map(|s|s.a.1).collect();axis(&mut xs);axis(&mut ys);if xs.len()<2||ys.len()<2||xs.len()>50||ys.len()>500{return None}
    let(x0,x1,y0,y1)=(xs[0],*xs.last()?,ys[0],*ys.last()?);
    if !xs.iter().all(|x|vertical.iter().any(|s|(s.a.0-x).abs()<1.&&s.a.1.min(s.b.1)<=y0+1.&&s.a.1.max(s.b.1)>=y1-1.))||!ys.iter().all(|y|horizontal.iter().any(|s|(s.a.1-y).abs()<1.&&s.a.0.min(s.b.0)<=x0+1.&&s.a.0.max(s.b.0)>=x1-1.)){return None}
    ys.reverse();let mut rows=vec![vec![TableCell::default();xs.len()-1];ys.len()-1];let mut bold_header=true;let mut used=false;
    let mut retained=vec![];for l in lines.drain(..){if l.y>y0&&l.y<y1&&l.x_range.0>=x0-1.&&l.x_range.1<=x1+1.{let r=ys.windows(2).position(|v|l.y<v[0]&&l.y>=v[1]).unwrap_or(0);for c in 0..xs.len()-1{let gs:Vec<_>=l.glyphs.iter().filter(|g|{let x=(g.bbox.0+g.bbox.2)*0.5;x>=xs[c]&&x<xs[c+1]}).cloned().collect();if !gs.is_empty(){let part=line(gs);if r==0{bold_header&=part.bold()}if !rows[r][c].inlines.is_empty(){rows[r][c].inlines.push(Inline::Break)}rows[r][c].inlines.extend(inlines(&part));used=true}}}else{retained.push(l)}}*lines=retained;if !used{return None}let header=if bold_header{rows.remove(0)}else{vec![]};Some((y1,Block::Table(Table{header,rows,approximate:false,caption:None})))
}
fn unruled(lines:&mut Vec<Line>)->Vec<(f32,Block)>{
    fn bands(l:&Line)->Vec<Line>{let threshold=median(l.glyphs.iter().map(|g|g.bbox.2-g.bbox.0).collect())*3.;let mut groups:Vec<Vec<Glyph>>=vec![vec![]];let mut right=None;for g in &l.glyphs{if right.map(|r|g.bbox.0-r>threshold).unwrap_or(false){groups.push(vec![])}groups.last_mut().unwrap().push(g.clone());right=Some(g.bbox.2)}groups.into_iter().filter(|g|!g.is_empty()).map(line).collect()}
    let mut out=vec![];let mut i=0;while i+2<lines.len(){let first=bands(&lines[i]);if first.len()<2||first.len()>12||first.iter().any(|l|l.text().chars().count()>40){i+=1;continue}let mut end=i+1;while end<lines.len(){let parts=bands(&lines[end]);if parts.len()!=first.len()||(lines[end-1].y-lines[end].y).abs()>lines[i].height()*2.5||parts.iter().zip(&first).any(|(a,b)|(a.x_range.0-b.x_range.0).abs()>3.||a.text().chars().count()>40){break}end+=1}if end-i<3{i+=1;continue}if first.len()==2&&!first.iter().all(Line::bold)&&!lines[i..end].iter().flat_map(bands).any(|part|part.text().trim().parse::<f64>().is_ok()){i+=1;continue}let y=lines[i].y;let mut rows:Vec<Vec<TableCell>>=lines.drain(i..end).map(|l|bands(&l).iter().map(cell).collect()).collect();let header=if first.iter().all(Line::bold){rows.remove(0)}else{vec![]};out.push((y,Block::Table(Table{header,rows,approximate:true,caption:None})));}out
}
fn list_marker(s:&str)->Option<(bool,Option<u64>,&str)>{if let Some(rest)=s.strip_prefix("• ").or_else(||s.strip_prefix("- ")).or_else(||s.strip_prefix("* ")){return Some((false,None,rest))}let n=s.bytes().take_while(u8::is_ascii_digit).count();if n>0&&n<=9{let tail=s.get(n..)?;if let Some(rest)=tail.strip_prefix(". ").or_else(||tail.strip_prefix(") ")){return Some((true,s[..n].parse().ok(),rest))}}None}
fn reconstruct(mut p:PageData,body:f32,sizes:&[i32],warnings:&mut Vec<String>)->Vec<Block>{
    p.lines.retain(|l|!l.is_header_footer);let mut positioned=vec![];let table_x=p.segments.iter().map(|s|s.a.0.min(s.b.0)).fold(p.width,f32::min);if let Some((y,block))=ruled(&mut p.lines,&p.segments){positioned.push((y,table_x,block))}let unruled_x=p.lines.first().map(|l|l.x_range.0).unwrap_or(0.);let tables=unruled(&mut p.lines);if !tables.is_empty(){warnings.push("PDF unruled table inferred from repeated x-alignment; approximate".into())}positioned.extend(tables.into_iter().map(|(y,b)|(y,unruled_x,b)));
    for picture in p.pictures{let h=median(p.lines.iter().map(Line::height).collect());let below=p.lines.iter().enumerate().filter(|(_,l)|l.y<=picture.bbox.1&&picture.bbox.1-l.y<=h*2.&&l.x_range.0<picture.bbox.2&&l.x_range.1>picture.bbox.0&&!heading(l,&p.lines,body)).min_by(|(_,a),(_,b)|(picture.bbox.1-a.y).total_cmp(&(picture.bbox.1-b.y))).map(|(i,_)|i);let above=||p.lines.iter().enumerate().filter(|(_,l)|l.y>=picture.bbox.3&&l.y-picture.bbox.3<=h*2.&&l.x_range.0<picture.bbox.2&&l.x_range.1>picture.bbox.0&&!heading(l,&p.lines,body)).min_by(|(_,a),(_,b)|(a.y-picture.bbox.3).total_cmp(&(b.y-picture.bbox.3))).map(|(i,_)|i);let caption=below.or_else(above).map(|i|p.lines.remove(i).text());positioned.push((picture.bbox.3,picture.bbox.0,Block::Image{asset:picture.asset,alt:None,caption}));}
    let spatial=p.lines.clone();let boundaries=cuts(&spatial,p.width);let lines=order_reading(p.lines,p.width);let mut out=vec![];let mut i=0;let mut math_warned=false;
    let mut events:Vec<(usize,f32,Block)>=positioned.into_iter().map(|(y,x,b)|{let col=boundaries.partition_point(|cut|*cut<x);let slot=lines.iter().position(|l|{let c=boundaries.partition_point(|cut|*cut<l.x_range.0);(c==col&&l.y<=y)||c>col}).unwrap_or(lines.len());(slot,y,b)}).collect();events.sort_by(|a,b|a.0.cmp(&b.0).then_with(||b.1.total_cmp(&a.1)));let mut events=events.into_iter().peekable();
    while i<lines.len(){let l=&lines[i];while events.peek().map(|(slot,_,_)|*slot<=i).unwrap_or(false){out.push(events.next().unwrap().2)}
        if l.mono(){let mut end=i+1;while end<lines.len()&&lines[end].mono()&&(lines[end].x_range.0-l.x_range.0).abs()<l.height()*2.&&(lines[end-1].y-lines[end].y).abs()<l.height()*2.5{end+=1}if end-i>=3{out.push(Block::CodeBlock{language:None,text:lines[i..end].iter().map(Line::text).collect::<Vec<_>>().join("\n")});warnings.push("code block detected via monospace font heuristic; language unknown".into());i=end;continue}}
        // P4-S10: nesting level from relative x-offset steps; wrapped continuation
        // lines at a deeper indent stay attached to the preceding item.
        let text=l.text();
        if let Some((ordered,start,rest))=list_marker(&text){
            let mut items=vec![ListItem{level:0,blocks:vec![Block::Paragraph(vec![Inline::Text(rest.into())])]}];
            let mut indents=vec![l.x_range.0];
            let mut end=i+1;
            while end<lines.len(){
                let next=lines[end].text();
                if let Some((kind,_,rest))=list_marker(&next){
                    if kind==ordered{
                        let x=lines[end].x_range.0;
                        let tol=l.height()*0.5;
                        while indents.len()>1&&x<indents[indents.len()-1]-tol{indents.pop();}
                        if x>indents[indents.len()-1]+tol{indents.push(x)}
                        items.push(ListItem{level:indents.len()-1,blocks:vec![Block::Paragraph(vec![Inline::Text(rest.into())])]});
                        end+=1;
                        continue;
                    }
                }else if lines[end].x_range.0-l.x_range.0>l.height()*0.5&&!next.trim().is_empty(){
                    if let Some(last)=items.last_mut(){
                        last.blocks.push(Block::Paragraph(vec![Inline::Text(next.into())]));
                        end+=1;
                        continue;
                    }
                }
                break;
            }
            out.push(Block::List{ordered,start,items});
            i=end;
            continue;
        }
        // P4-S11: literal Unicode, never LaTeX; the warning fires once per document.
        let flags=math_flags(&l.glyphs);let mut any_math=false;let mut all_math=true;for(gi,g)in l.glyphs.iter().enumerate(){if g.ch.is_whitespace(){continue}any_math=true;all_math=all_math&&flags[gi]}
        if all_math&&any_math{out.push(Block::MathBlock(text));if !math_warned{warnings.push("mathematical content rendered as literal Unicode characters, not LaTeX; PDF's raw glyph stream does not expose equation structure the way DOCX's native math markup does.".into());math_warned=true}}
        else if heading(l,&spatial,body){let level=sizes.iter().position(|s|*s==(l.size()*2.).round()as i32).map(|i|(i+1).min(6)).unwrap_or(6);out.push(Block::Heading{level,inlines:inlines(l)})}
        else{out.push(Block::Paragraph(inlines(l)))}i+=1;
    }out.extend(events.map(|(_,_,b)|b));out
}
#[cfg(test)]mod tests{
use super::*;
fn glyphs(s:&str,x:f32,y:f32)->Vec<Glyph>{s.chars().enumerate().map(|(i,ch)|Glyph{ch,bbox:(x+i as f32*6.,y,x+i as f32*6.+5.,y+10.),font_size:12.,font_name:"Helvetica".into()}).collect()}
#[test]fn columns_are_column_major(){let mut gs=vec![];for(y,a,b)in [(700.,"Left one","Right one"),(680.,"Left two","Right two"),(660.,"Left end","Right end")]{gs.extend(glyphs(a,30.,y));gs.extend(glyphs(b,330.,y));}let ordered=order_reading(cluster_lines(&gs),600.);let s=ordered.iter().map(Line::text).collect::<Vec<_>>().join("|");assert!(s.find("Left end").unwrap()<s.find("Right one").unwrap(),"{s}");}
#[test]fn repetition_requires_distinct_pages(){let mut pages:Vec<_>=(0..3).map(|_|PageData{lines:vec![line(glyphs("Header",20.,770.)),line(glyphs("Body",20.,400.))],width:600.,height:800.,segments:vec![],pictures:vec![],ocr:None,scanned:false}).collect();strip_repetition(&mut pages);assert!(pages.iter().all(|p|p.lines[0].is_header_footer&&!p.lines[1].is_header_footer));}
#[test]fn double_drawn_glyphs_dedupe(){let g=glyphs("A",0.,0.)[0].clone();assert_eq!(dedupe_glyphs(vec![g.clone(),g.clone()]).len(),1);assert_eq!(dedupe_glyphs(glyphs("AA",0.,0.)).len(),2);let mut near=glyphs("A",0.,0.)[0].clone();near.bbox.0+=0.74;near.bbox.1+=0.52;near.bbox.2+=0.74;near.bbox.3+=0.52;assert_eq!(dedupe_glyphs(vec![g,near]).len(),1);}
#[test]fn shadow_copies_drop_but_leaders_survive(){
    // Systematic double-draw at 24pt with a fixed (1.96, 2.00) offset.
    let mut cover=vec![];
    for(rep,dx,dy)in[(0,0.,0.),(1,1.96,2.)]{for(i,ch)in "Hi!".chars().enumerate(){cover.push(Glyph{ch,bbox:(30.+i as f32*14.+dx,700.+dy,35.+i as f32*14.+dx,712.+dy),font_size:24.,font_name:"Display".into()});let _=rep;}}
    assert_eq!(drop_shadows(cover).len(),3);
    // Dotted leaders: one repeated char at a regular 2pt advance — fraction and
    // offset look shadow-like, but the single distinct char vetoes the merge.
    let mut leaders=vec![];
    for i in 0..8{leaders.push(Glyph{ch:'.',bbox:(30.+i as f32*2.,700.,31.+i as f32*2.,706.),font_size:24.,font_name:"Display".into()});}
    assert_eq!(drop_shadows(leaders).len(),8);
    // Small type never merged, even doubled.
    let small=glyphs("AA",0.,0.);
    assert_eq!(drop_shadows(small).len(),2);
    // Decoy bin (leaders, 7 pairs) outvotes the shadow bin (3 pairs) but fails
    // the distinct-chars guard, so the shadow bin still wins.
    let mut mixed=vec![];
    for i in 0..8{mixed.push(Glyph{ch:'.',bbox:(200.+i as f32*2.,700.,201.+i as f32*2.,706.),font_size:24.,font_name:"Display".into()});}
    for(rep,dx,dy)in[(0,0.,0.),(1,1.96,2.)]{for(i,ch)in "Hi!".chars().enumerate(){mixed.push(Glyph{ch,bbox:(30.+i as f32*14.+dx,700.+dy,35.+i as f32*14.+dx,712.+dy),font_size:24.,font_name:"Display".into()});let _=rep;}}
    assert_eq!(drop_shadows(mixed).len(),8+3);
    // Two shadow layers at different offsets on one page both apply.
    let mut layers=vec![];
    for(rep,dx,dy)in[(0,0.,0.),(1,1.96,2.)]{for(i,ch)in "Hi!".chars().enumerate(){layers.push(Glyph{ch,bbox:(30.+i as f32*14.+dx,700.+dy,35.+i as f32*14.+dx,712.+dy),font_size:24.,font_name:"Display".into()});let _=rep;}}
    for(rep,dx,dy)in[(0,0.,0.),(1,0.74,0.52)]{for(i,ch)in "Yo?".chars().enumerate(){layers.push(Glyph{ch,bbox:(30.+i as f32*14.+dx,640.+dy,35.+i as f32*14.+dx,652.+dy),font_size:24.,font_name:"Display".into()});let _=rep;}}
    assert_eq!(drop_shadows(layers).len(),6);
}
#[test]fn math_range_and_font(){assert!(math_base(&glyphs("∑",0.,0.)[0]));assert!(!math_base(&glyphs("A",0.,0.)[0]));assert!(!math_flags(&glyphs("→",0.,0.))[0]);let mut lone=glyphs("→x",0.,0.);lone[1].font_name="Cambria Math".into();let flags=math_flags(&lone);assert!(flags[0]&&flags[1]);}

#[test]fn two_text_columns_are_not_mistaken_for_unruled_table(){let mut gs=vec![];for y in [700.,680.,660.]{gs.extend(glyphs("Left prose",30.,y));gs.extend(glyphs("Right prose",330.,y));}let mut lines=cluster_lines(&gs);assert!(unruled(&mut lines).is_empty());assert_eq!(lines.len(),3);}
#[test]fn closed_grid_becomes_table(){let mut lines=vec![line(glyphs("A",20.,75.)),line(glyphs("B",120.,25.))];let mut segments=vec![];for x in [0.,100.,200.]{segments.push(Segment{a:(x,0.),b:(x,100.)})}for y in [0.,50.,100.]{segments.push(Segment{a:(0.,y),b:(200.,y)})}let(_,block)=ruled(&mut lines,&segments).unwrap();assert!(lines.is_empty());assert!(matches!(block,Block::Table(t)if !t.approximate&&t.rows.len()==2));}
#[test]fn isolated_large_font_is_heading(){let mut title=line(glyphs("Large heading",20.,740.));for g in &mut title.glyphs{g.font_size=24.}let body=line(glyphs("Body text",20.,650.));assert!(heading(&title,&[title.clone(),body],12.));}
#[test]fn code_and_lists_reconstruct_without_double_counting(){let mut lines:Vec<_>=[700.,680.,660.].into_iter().map(|y|line(glyphs("code",30.,y))).collect();for l in &mut lines{for g in &mut l.glyphs{g.font_name="Courier".into()}}lines.push(line(glyphs("- Item",30.,600.)));let p=PageData{lines,width:600.,height:800.,segments:vec![],pictures:vec![],ocr:None,scanned:false};let blocks=reconstruct(p,12.,&[],&mut vec![]);assert!(matches!(&blocks[0],Block::CodeBlock{text,..}if text=="code\ncode\ncode"));assert!(matches!(&blocks[1],Block::List{ordered:false,..}));}
}
