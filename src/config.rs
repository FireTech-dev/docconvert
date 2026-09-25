use crate::error::{ConvertError,Result};
use clap::ValueEnum;
use std::path::{Path,PathBuf};
#[derive(Debug,Clone,Copy,PartialEq,Eq,Default,ValueEnum)]
pub enum FormatChoice {#[default] Auto,Md,Txt}
#[derive(Debug,Clone,Copy,PartialEq,Eq,Default,ValueEnum)]
pub enum OcrMode {#[default] Off,Auto,Force}
#[derive(Debug,Clone,Copy,PartialEq,Eq,Default,ValueEnum)]
pub enum ReportKind {None,Text,#[default] Json}
#[derive(Debug,Clone)]
pub struct Options {
 pub format_choice:FormatChoice,pub embed_assets:bool,pub ocr_mode:OcrMode,pub ocr_lang:String,pub ocr_command:String,pub include_hidden_sheets:bool,pub preserve_formulas:bool,pub include_comments:bool,pub overwrite:bool,pub strict:bool,pub max_file_bytes:u64,pub report_kind:ReportKind,pub describe_images:bool,pub include_notes:bool,pub export_charts:bool,pub jobs:usize,pub output:Option<PathBuf>,pub quiet:bool,pub verbose:bool,pub minimal_placeholders:bool,pub recursive:bool,pub strip_headers_footers:bool,
 #[cfg(feature="pdf-layout")] pub pdfium_lib_path:Option<PathBuf>,
}
impl Default for Options {fn default()->Self{Self{format_choice:FormatChoice::Auto,embed_assets:false,ocr_mode:OcrMode::Off,ocr_lang:"eng".into(),ocr_command:"tesseract".into(),include_hidden_sheets:false,preserve_formulas:true,include_comments:true,overwrite:false,strict:false,max_file_bytes:200*1024*1024,report_kind:ReportKind::Json,describe_images:false,include_notes:false,export_charts:false,jobs:0,output:None,quiet:false,verbose:false,minimal_placeholders:false,recursive:false,strip_headers_footers:true,
 #[cfg(feature="pdf-layout")] pdfium_lib_path:None}}}
impl Options {
 // P0-S08 profile effects (each documented per the blueprint's Explicit Decisions
 // principle; precedence is defaults -> profile -> config file -> CLI):
 // academic -> Md + formulas + comments; technical -> Md + formulas;
 // plain -> Txt output regardless of complexity (no formula-listing change);
 // spreadsheet -> formulas + comments + hidden sheets on;
 // archive -> Md, embedding off, Json report; accessibility -> Md + notes on;
 // tts -> Txt with minimal asset placeholders; ocr -> OCR mode Auto;
 // strict -> strict batch stop on.
 pub fn apply_profile(&mut self,name:&str)->Result<()>{match name{
  "academic"=>{self.format_choice=FormatChoice::Md;self.preserve_formulas=true;self.include_comments=true},
  "technical"=>{self.format_choice=FormatChoice::Md;self.preserve_formulas=true},
  "plain"=>self.format_choice=FormatChoice::Txt,"spreadsheet"=>{self.preserve_formulas=true;self.include_comments=true;self.include_hidden_sheets=true},
  "archive"=>{self.format_choice=FormatChoice::Md;self.embed_assets=false;self.report_kind=ReportKind::Json},
  "accessibility"=>{self.format_choice=FormatChoice::Md;self.include_notes=true},"tts"=>{self.format_choice=FormatChoice::Txt;self.minimal_placeholders=true},"ocr"=>self.ocr_mode=OcrMode::Auto,"strict"=>self.strict=true,_=>return Err(ConvertError::Config("unknown profile".into()))}Ok(())}
 pub fn load_config_file(&mut self,path:&Path)->Result<()>{let s=std::fs::read_to_string(path)?;for (i,line) in s.lines().enumerate(){let line=line.trim();if line.is_empty()||line.starts_with('#'){continue}let (k,v)=line.split_once('=').ok_or_else(||ConvertError::Config(format!("expected key = value on line {}",i+1)))?;self.set(k.trim(),v.trim().trim_matches('"'))?}Ok(())}
 pub fn set(&mut self,k:&str,v:&str)->Result<()>{let boolean=||match v{"true"|"1"=>Ok(true),"false"|"0"=>Ok(false),_=>Err(ConvertError::Config(format!("{k} requires true or false")))};match k {
 "format"=>self.format_choice=FormatChoice::from_str(v,true).map_err(|_|ConvertError::Config("format requires auto, md or txt".into()))?,
 "ocr"=>self.ocr_mode=OcrMode::from_str(v,true).map_err(|_|ConvertError::Config("ocr requires off, auto or force".into()))?,
 "report"=>self.report_kind=ReportKind::from_str(v,true).map_err(|_|ConvertError::Config("report requires none, text or json".into()))?,
 "output"=>self.output=Some(v.into()),"ocr-lang"=>self.ocr_lang=v.into(),"ocr-command"=>self.ocr_command=v.into(),
 "embed-assets"=>self.embed_assets=boolean()?,"include-hidden-sheets"=>self.include_hidden_sheets=boolean()?,"no-formulas"=>self.preserve_formulas=!boolean()?,"no-comments"=>self.include_comments=!boolean()?,"overwrite"=>self.overwrite=boolean()?,"strict"=>self.strict=boolean()?,"describe-images"=>self.describe_images=boolean()?,"include-notes"=>self.include_notes=boolean()?,"export-charts"=>self.export_charts=boolean()?,"quiet"=>self.quiet=boolean()?,"verbose"=>self.verbose=boolean()?,
 "recursive"=>self.recursive=boolean()?,"no-strip-headers-footers"=>self.strip_headers_footers=!boolean()?,
 #[cfg(feature="pdf-layout")] "pdfium-lib-path"=>self.pdfium_lib_path=Some(v.into()),
 "jobs"=>self.jobs=v.parse().map_err(|_|ConvertError::Config("jobs requires a nonnegative integer".into()))?,
 "max-size"=>self.max_file_bytes=v.parse::<u64>().ok().and_then(|n|n.checked_mul(1024*1024)).filter(|n|*n>0).ok_or_else(||ConvertError::Config("max-size requires positive MB".into()))?,
 _=>{let keys=["format","ocr","report","output","ocr-lang","ocr-command","embed-assets","include-hidden-sheets","no-formulas","no-comments","overwrite","strict","describe-images","include-notes","export-charts","quiet","verbose","jobs","max-size","recursive","no-strip-headers-footers","pdfium-lib-path"];let closest=keys.iter().min_by_key(|s|distance(k,s)).copied().unwrap_or("format");return Err(ConvertError::Config(format!("unknown key {k:?}; did you mean {closest:?}?")))} }Ok(())}
}
fn distance(a:&str,b:&str)->usize{let mut row:Vec<usize>=(0..=b.chars().count()).collect();for (i,x) in a.chars().enumerate(){let mut prev=row[0];row[0]=i+1;for(j,y)in b.chars().enumerate(){let old=row[j+1];row[j+1]=(row[j]+1).min(old+1).min(prev+usize::from(x!=y));prev=old}}*row.last().unwrap_or(&0)}
