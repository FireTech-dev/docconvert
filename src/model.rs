use serde::Serialize;
use std::{collections::BTreeMap, fmt};
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub enum Format { Pdf, Docx, Doc, Odt, Ods, Odp, Rtf, Epub, Html, Txt, Markdown, Pptx, Xlsx, Xls, Csv, Tsv, Image, #[default] Unknown }
impl fmt::Display for Format {
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result { f.write_str(match self {
        Self::Pdf=>"pdf",Self::Docx=>"docx",Self::Doc=>"doc",Self::Odt=>"odt",Self::Ods=>"ods",Self::Odp=>"odp",Self::Rtf=>"rtf",Self::Epub=>"epub",Self::Html=>"html",Self::Txt=>"txt",Self::Markdown=>"markdown",Self::Pptx=>"pptx",Self::Xlsx=>"xlsx",Self::Xls=>"xls",Self::Csv=>"csv",Self::Tsv=>"tsv",Self::Image=>"image",Self::Unknown=>"unknown" }) }
}
#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize)]
pub enum ImageKind { Png,Jpeg,Gif,Bmp,Tiff,Webp,Jp2 }
impl ImageKind {
    pub fn ext(&self)->&'static str { match self {Self::Png=>"png",Self::Jpeg=>"jpg",Self::Gif=>"gif",Self::Bmp=>"bmp",Self::Tiff=>"tiff",Self::Webp=>"webp",Self::Jp2=>"jp2"} }
    pub fn mime(&self)->&'static str { match self {Self::Png=>"image/png",Self::Jpeg=>"image/jpeg",Self::Gif=>"image/gif",Self::Bmp=>"image/bmp",Self::Tiff=>"image/tiff",Self::Webp=>"image/webp",Self::Jp2=>"image/jp2"} }
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct Meta {
    pub title:Option<String>,pub author:Option<String>,pub subject:Option<String>,pub description:Option<String>,pub created:Option<String>,pub modified:Option<String>,pub language:Option<String>,
    pub source_filename:String,pub source_format:Format,pub page_count:Option<usize>,pub slide_count:Option<usize>,pub chapter_count:Option<usize>,pub worksheet_count:Option<usize>,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TextStyle { pub bold:bool,pub italic:bool,pub underline:bool,pub strike:bool,pub sup:bool,pub sub:bool }
impl TextStyle {pub fn is_plain(&self)->bool {!(self.bold||self.italic||self.underline||self.strike||self.sup||self.sub)}}
#[derive(Debug,Clone,Serialize)]
pub enum Inline { Text(String),Styled{style:TextStyle,children:Vec<Inline>},Code(String),Link{children:Vec<Inline>,url:String},MathInline(String),FootnoteRef(String),Image{asset:usize,alt:Option<String>},Break }
#[derive(Debug,Clone,Copy,Default,Serialize)]
pub enum Align {#[default] Default,Left,Right,Center}
#[derive(Debug,Clone,Serialize)]
pub struct TableCell {pub inlines:Vec<Inline>,pub colspan:usize,pub rowspan:usize,pub align:Align}
impl Default for TableCell {fn default()->Self {Self{inlines:vec![],colspan:1,rowspan:1,align:Align::Default}}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct Table {pub header:Vec<TableCell>,pub rows:Vec<Vec<TableCell>>,pub caption:Option<String>,pub approximate:bool}
#[derive(Debug,Clone,Default,Serialize)]
pub struct ListItem {pub level:usize,pub blocks:Vec<Block>}
#[derive(Debug,Clone,Serialize)]
pub enum Boundary {Page(usize),Slide{n:usize,title:Option<String>},Chapter(Option<String>),Worksheet(String)}
#[derive(Debug,Clone,Serialize)]
pub enum PlaceholderKind {Chart,Drawing,EmbeddedObject,ScannedPage}
impl PlaceholderKind {pub fn label(&self)->&'static str {match self {Self::Chart=>"Chart",Self::Drawing=>"Drawing",Self::EmbeddedObject=>"Embedded object",Self::ScannedPage=>"Scanned page"}}}
#[derive(Debug,Clone,Serialize)]
pub enum Block {
 Heading{level:usize,inlines:Vec<Inline>},Paragraph(Vec<Inline>),List{ordered:bool,start:Option<u64>,items:Vec<ListItem>},Table(Table),CodeBlock{language:Option<String>,text:String},MathBlock(String),Quote(Vec<Block>),Rule,
 Image{asset:usize,alt:Option<String>,caption:Option<String>},Placeholder{kind:PlaceholderKind,label:Option<String>,asset:Option<usize>},Boundary(Boundary),FormulaList(Vec<(String,String)>),
}
#[derive(Debug,Clone,Default,Serialize)]
pub enum CellKind {#[default] Empty,Number,Text,Bool,Date,Error}
#[derive(Debug,Clone,Default,Serialize)]
pub struct Cell {pub reference:String,pub formula:Option<String>,pub display:String,pub kind:CellKind,pub number_format:Option<String>,pub hyperlink:Option<String>}
#[derive(Debug,Clone,Default,Serialize)]
pub struct Sheet {pub name:String,pub hidden:bool,pub cells:BTreeMap<(u32,u32),Cell>,pub max_row:u32,pub max_col:u32,pub merged:Vec<String>,pub comments:Vec<(String,String)>,pub charts:Vec<Option<String>>}
#[derive(Debug,Clone,Default,Serialize)]
pub struct Workbook {pub title:Option<String>,pub sheets:Vec<Sheet>}
#[derive(Debug,Clone,Default,Serialize)]
pub struct Asset {pub filename:String,pub rel_path:String,pub mime:String,pub bytes:Vec<u8>,pub alt:Option<String>,pub origin:String,pub duplicate_of:Option<String>,pub hash:u64}
#[derive(Debug,Clone,Default,Serialize)]
pub struct Document {pub meta:Meta,pub blocks:Vec<Block>,pub footnotes:Vec<(String,Vec<Inline>)>,pub assets:Vec<Asset>,pub workbook:Option<Workbook>,pub warnings:Vec<String>}
impl Document {
 pub fn new(source_filename:impl Into<String>,source_format:Format)->Self {Self{meta:Meta{source_filename:source_filename.into(),source_format,..Meta::default()},..Self::default()}}
 /// Unicode scalar count of visible body text plus footnotes, not metadata or duplicate workbook data.
 pub fn text_len(&self)->usize {fn count(bs:&[Block])->usize {bs.iter().map(|b|match b {
 Block::Heading{inlines,..}|Block::Paragraph(inlines)=>inline_text(inlines).chars().count(),
 Block::List{items,..}=>items.iter().map(|i|count(&i.blocks)).sum(),Block::Quote(bs)=>count(bs),
 Block::Table(t)=>t.header.iter().chain(t.rows.iter().flatten()).map(|c|inline_text(&c.inlines).chars().count()).sum(),
 Block::CodeBlock{text,..}|Block::MathBlock(text)=>text.chars().count(),Block::FormulaList(fs)=>fs.iter().map(|(r,f)|r.chars().count()+f.chars().count()).sum(),_=>0}).sum()}
 count(&self.blocks)+self.footnotes.iter().map(|(_,i)|inline_text(i).chars().count()).sum::<usize>()}
}
pub fn inline_text(inlines:&[Inline])->String {let mut s=String::new();for i in inlines {match i {Inline::Text(t)|Inline::Code(t)|Inline::MathInline(t)=>s.push_str(t),Inline::Styled{children,..}|Inline::Link{children,..}=>s.push_str(&inline_text(children)),Inline::Break=>s.push('\n'),Inline::FootnoteRef(id)=>s.push_str(&format!("[{id}]")),Inline::Image{alt,..}=>s.push_str(alt.as_deref().unwrap_or(""))}}s}
/// Postorder: visits existing children before their parent; replacement children are not revisited.
pub fn walk_inlines_mut(blocks:&mut [Block],f:&mut dyn FnMut(&mut Inline)) {
 fn visit(xs:&mut [Inline],f:&mut dyn FnMut(&mut Inline)){for x in xs {match x {Inline::Styled{children,..}|Inline::Link{children,..}=>visit(children,f),_=>{}}f(x)}}
 for b in blocks {match b {Block::Heading{inlines,..}|Block::Paragraph(inlines)=>visit(inlines,f),Block::Table(t)=>for c in t.header.iter_mut().chain(t.rows.iter_mut().flatten()){visit(&mut c.inlines,f)},Block::List{items,..}=>for i in items {walk_inlines_mut(&mut i.blocks,f)},Block::Quote(bs)=>walk_inlines_mut(bs,f),_=>{}}}
}
