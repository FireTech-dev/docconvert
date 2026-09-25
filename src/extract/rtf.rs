use super::ExtractCtx;
use crate::{error::Result,model::*};
use logos::Logos;
#[derive(Logos,Debug,PartialEq)]
#[logos(skip r"[\r\n]+") ]
enum Token {
 #[token("{")] Open,#[token("}")] Close,
 #[regex(r"\\[a-zA-Z]+-?[0-9]* ?")] Word,
 #[regex(r"\\'[0-9a-fA-F]{2}",priority=3)] Hex,
 #[regex(r"\\[^a-zA-Z\r\n]")] Symbol,
 #[regex(r"[^\\{}\r\n]+") ] Text,
}
#[derive(Clone)]
struct State {style:TextStyle,skip:bool,codepage:u16,uc:usize,picture:bool,picture_owner:bool,picture_kind:Option<ImageKind>,hex:String,size:u32,field_inst:bool,field_result:bool,instruction:String,url:Option<String>}
impl Default for State{fn default()->Self{Self{style:TextStyle::default(),skip:false,codepage:1252,uc:1,picture:false,picture_owner:false,picture_kind:None,hex:String::new(),size:24,field_inst:false,field_result:false,instruction:String::new(),url:None}}}
fn decode(bytes:&[u8],cp:u16)->String{let label=match cp{65001=>"utf-8".into(),932=>"shift_jis".into(),936=>"gbk".into(),949=>"euc-kr".into(),950=>"big5".into(),n=>format!("windows-{n}")};let enc=encoding_rs::Encoding::for_label(label.as_bytes()).unwrap_or(encoding_rs::WINDOWS_1252);enc.decode_without_bom_handling(bytes).0.into_owned()}

fn finish_group(mut state:State,parent:Option<&mut State>,doc:&mut Document,sizes:&mut Vec<u32>,ctx:&mut ExtractCtx<'_>){
    if state.field_inst&&!state.instruction.is_empty(){
        let instruction=state.instruction.trim();
        if let Some(rest)=instruction.strip_prefix("HYPERLINK"){
            let rest=rest.trim();state.url=if let Some(quoted)=rest.strip_prefix('"'){quoted.find('"').map(|end|quoted[..end].to_owned())}else{rest.split_whitespace().next().map(str::to_owned)};
        }
    }
    if state.picture_owner&&!state.hex.is_empty(){
        let hex:String=state.hex.chars().filter(|c|!c.is_whitespace()).collect();
        let bytes:Option<Vec<u8>>=hex.as_bytes().chunks(2).map(|chunk|if chunk.len()==2{std::str::from_utf8(chunk).ok().and_then(|s|u8::from_str_radix(s,16).ok())}else{None}).collect();
        if let Some(bytes)=bytes{if state.picture_kind.is_some(){let asset=ctx.assets.add(bytes,"RTF picture",None);doc.blocks.push(Block::Image{asset,alt:None,caption:None});sizes.push(0)}}else{ctx.warnings.push("malformed RTF picture hex data".into())}
    }
    if let Some(parent)=parent{
        if state.field_inst{parent.url=state.url;}
        if state.picture&&!state.picture_owner&&parent.picture{parent.hex.push_str(&state.hex);if state.picture_kind.is_some(){parent.picture_kind=state.picture_kind}}
    }
}
pub fn extract_rtf(bytes:&[u8],ctx:&mut ExtractCtx<'_>)->Result<Document>{
 // Latin-1 transport preserves the original bytes until the current RTF codepage is known.
 let source: String=bytes.iter().map(|b|char::from(*b)).collect();let mut lexer=Token::lexer(&source);let mut stack=vec![State::default()];let mut d=Document::default();let mut para=vec![];let mut sizes=vec![];let mut para_size=24;let mut skip_fallback=0usize;let mut pending_high=None;let mut raw=vec![];let mut table_cells:Vec<TableCell>=vec![];
 fn add(para:&mut Vec<Inline>,s:String,state:&State){if s.is_empty()||state.skip{return}let mut item=if state.style.is_plain(){Inline::Text(s)}else{Inline::Styled{style:state.style.clone(),children:vec![Inline::Text(s)]}};if let Some(url)=&state.url{if state.field_result{item=Inline::Link{url:url.clone(),children:vec![item]}}}para.push(item)}
 fn flush_bytes(raw:&mut Vec<u8>,para:&mut Vec<Inline>,state:&State){if !raw.is_empty(){add(para,decode(raw,state.codepage),state);raw.clear()}}
 fn flush_para(para:&mut Vec<Inline>,d:&mut Document,sizes:&mut Vec<u32>,size:u32){if !para.is_empty(){d.blocks.push(Block::Paragraph(std::mem::take(para)));sizes.push(size)}}
 while let Some(token)=lexer.next(){let slice=lexer.slice();if !matches!(token,Ok(Token::Hex)){if let Some(s)=stack.last(){flush_bytes(&mut raw,&mut para,s)}}
 match token{
 Ok(Token::Open)=>{if stack.len()>=64{ctx.warnings.push("RTF group depth limit reached; remaining content preserved as text".into());if let Some(s)=stack.last(){add(&mut para,lexer.remainder().into(),s)}break}let mut child=stack.last().cloned().unwrap_or_default();child.picture_owner=false;if child.picture{child.hex.clear()}stack.push(child)},
 Ok(Token::Close)=>{if stack.len()>1{let state=stack.pop().unwrap_or_default();if state.picture_owner{flush_para(&mut para,&mut d,&mut sizes,para_size)}finish_group(state,stack.last_mut(),&mut d,&mut sizes,ctx)}else{ctx.warnings.push("unmatched RTF closing group".into())}},
 Ok(Token::Word)=>{let word=slice.trim_end().trim_start_matches('\\');let n=word.chars().take_while(char::is_ascii_alphabetic).count();let key=&word[..n];let val=word[n..].parse::<i32>().ok();let Some(s)=stack.last_mut()else{break};let on=val!=Some(0);match key{
 "fonttbl"|"colortbl"|"stylesheet"|"info"|"generator"=>s.skip=true,"b"=>s.style.bold=on,"i"=>s.style.italic=on,"strike"=>s.style.strike=on,"ul"=>s.style.underline=on,"ulnone"=>s.style.underline=false,"super"=>{s.style.sup=true;s.style.sub=false},"sub"=>{s.style.sub=true;s.style.sup=false},"nosupersub"=>{s.style.sup=false;s.style.sub=false},"plain"=>s.style=TextStyle::default(),"ansicpg"=>s.codepage=val.unwrap_or(1252).clamp(0,65535)as u16,"uc"=>s.uc=val.unwrap_or(1).clamp(0,16)as usize,"fs"=>{s.size=val.unwrap_or(24).max(1)as u32;if para.is_empty(){para_size=s.size}},
 "trowd"=>{flush_para(&mut para,&mut d,&mut sizes,para_size);table_cells.clear()},
 "cell"=>table_cells.push(TableCell{inlines:std::mem::take(&mut para),..TableCell::default()}),
 "row"=>{if !para.is_empty(){table_cells.push(TableCell{inlines:std::mem::take(&mut para),..TableCell::default()})}let row=std::mem::take(&mut table_cells);if let Some(Block::Table(t))=d.blocks.last_mut(){t.rows.push(row)}else{d.blocks.push(Block::Table(Table{header:row,..Table::default()}));sizes.push(0)}},
 "par"|"pard"=>{flush_para(&mut para,&mut d,&mut sizes,para_size);para_size=s.size},"line"=>para.push(Inline::Break),"tab"=>add(&mut para,"\t".into(),s),
 "u"=>{let u=val.unwrap_or(0)as i16 as u16;if(0xd800..=0xdbff).contains(&u){pending_high=Some(u)}else if let Some(high)=pending_high.take(){let cp=0x10000+((high as u32-0xd800)<<10)+(u as u32).saturating_sub(0xdc00);add(&mut para,char::from_u32(cp).unwrap_or('�').to_string(),s)}else{add(&mut para,char::from_u32(u as u32).unwrap_or('�').to_string(),s)}skip_fallback=s.uc},
 "pict"=>{s.picture=true;s.picture_owner=true;s.hex.clear()},"pngblip"=>s.picture_kind=Some(ImageKind::Png),"jpegblip"=>s.picture_kind=Some(ImageKind::Jpeg),"wmetafile"|"emfblip"=>{s.skip=true;ctx.warnings.push("WMF/EMF image not decoded".into());d.blocks.push(Block::Placeholder{kind:PlaceholderKind::Drawing,label:None,asset:None});sizes.push(0)},
 "fldinst"=>{s.field_inst=true;s.skip=false;s.instruction.clear()},"fldrslt"=>{s.field_inst=false;s.field_result=true;s.skip=false},_=>{}}},
 Ok(Token::Hex)=>{if let Some(s)=stack.last(){if skip_fallback>0{skip_fallback-=1}else if !s.skip{if let Ok(b)=u8::from_str_radix(&slice[2..],16){raw.push(b)}}}},
 Ok(Token::Symbol)=>{if let Some(s)=stack.last_mut(){match &slice[1..]{"*"=>s.skip=true,"~"=>add(&mut para,"\u{a0}".into(),s),"_"=>add(&mut para,"‑".into(),s),"-"=>{},"\\"|"{"|"}"=>add(&mut para,slice[1..].into(),s),"'"=>ctx.warnings.push("truncated RTF hex escape".into()),_=>{}}}},
 Ok(Token::Text)=>{if let Some(s)=stack.last_mut(){if s.picture{s.hex.push_str(slice)}else if s.field_inst{s.instruction.push_str(slice)}else{let text:Vec<u8>=slice.chars().skip(skip_fallback).map(|c|c as u8).collect();skip_fallback=skip_fallback.saturating_sub(slice.chars().count());add(&mut para,decode(&text,s.codepage),s)}}},Err(_)=>ctx.warnings.push("malformed RTF token preserved where possible".into())}
 }
 if let Some(s)=stack.last(){flush_bytes(&mut raw,&mut para,s)}flush_para(&mut para,&mut d,&mut sizes,para_size);if stack.len()>1{ctx.warnings.push("unterminated RTF groups closed at end of input".into());while stack.len()>1{let state=stack.pop().unwrap_or_default();finish_group(state,stack.last_mut(),&mut d,&mut sizes,ctx)}}
 let mut counts=std::collections::BTreeMap::new();for s in &sizes{if *s>0{*counts.entry(*s).or_insert(0usize)+=1}}let body=counts.iter().max_by_key(|(_,n)|*n).map(|(s,_)|*s).unwrap_or(24);let mut levels=sizes.iter().copied().filter(|s|*s as f64>=body as f64*1.2).collect::<Vec<_>>();levels.sort_unstable_by(|a,b|b.cmp(a));levels.dedup();for(i,b)in d.blocks.iter_mut().enumerate(){let s=sizes.get(i).copied().unwrap_or(body);if s as f64>=body as f64*1.2&&sizes.get(i+1).copied().unwrap_or(body)<=body{if let Block::Paragraph(xs)=b{*b=Block::Heading{level:levels.iter().position(|x|*x==s).unwrap_or(0).min(3)+1,inlines:std::mem::take(xs)}}}}
 Ok(d)
}
