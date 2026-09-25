use clap::{Parser,ArgAction};
use docconvert::{config::{Options,FormatChoice,OcrMode,ReportKind},convert,error::ConvertError};
use std::path::PathBuf;
#[derive(Parser,Debug)]
#[command(version,about="Offline document conversion to Markdown or plain text (PDF layout available with pdf-layout)")]
struct Cli {
 paths:Vec<PathBuf>,
 #[arg(short='r',long,action=ArgAction::SetTrue)]recursive:Option<bool>,
 #[arg(long,help="Worker count (0: auto, at most 8; explicit maximum 256)")]jobs:Option<usize>,
 #[arg(long,action=ArgAction::SetTrue)]no_strip_headers_footers:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]export_charts:Option<bool>,
 #[cfg(feature="pdf-layout")]
 #[arg(long,help="Trusted PDFium shared library file or directory")]pdfium_lib_path:Option<PathBuf>,
 #[arg(short='o',long)]output:Option<PathBuf>,
 #[arg(short='f',long,value_enum)]format:Option<FormatChoice>,
 #[arg(long)]profile:Option<String>,#[arg(long)]config:Option<PathBuf>,
 #[arg(long,value_enum,help="OCR: off, auto or force; standalone images are eligible when auto is explicitly requested")]ocr:Option<OcrMode>,#[arg(long)]ocr_lang:Option<String>,#[arg(long)]ocr_command:Option<String>,
 #[arg(long,action=ArgAction::SetTrue)]embed_assets:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]include_hidden_sheets:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]no_formulas:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]no_comments:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]describe_images:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]include_notes:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]overwrite:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]strict:Option<bool>,
 // AQ-009: negation companions so explicit CLI can disable a profile/config `true`
 // (precedence defaults → profile → config → CLI). Positive applied first, negation
 // after, so `--no-*` wins on direct conflict — deterministic without order tracking.
 #[arg(long,action=ArgAction::SetTrue)]no_embed_assets:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]no_include_hidden_sheets:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]formulas:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]comments:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]no_describe_images:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]no_include_notes:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]no_export_charts:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]strip_headers_footers:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]no_overwrite:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]no_strict:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]no_recursive:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]no_quiet:Option<bool>,
 #[arg(long,action=ArgAction::SetTrue)]no_verbose:Option<bool>,
 #[arg(long)]max_size:Option<u64>,#[arg(long,value_enum)]report:Option<ReportKind>,
 #[arg(short='q',long,action=ArgAction::SetTrue)]quiet:Option<bool>,
 #[arg(short='v',long,action=ArgAction::SetTrue)]verbose:Option<bool>,
 #[arg(long,value_name="FILE")]preview:Option<PathBuf>,
}
fn options(c:&Cli)->Result<Options,ConvertError>{let mut o=Options::default();if let Some(p)=&c.profile{o.apply_profile(p)?}if let Some(p)=&c.config{o.load_config_file(p)?}
 if let Some(v)=c.jobs{o.jobs=v}if c.recursive==Some(true){o.recursive=true}if c.no_recursive==Some(true){o.recursive=false}if c.no_strip_headers_footers==Some(true){o.strip_headers_footers=false}if c.strip_headers_footers==Some(true){o.strip_headers_footers=true}if c.export_charts==Some(true){o.export_charts=true}if c.no_export_charts==Some(true){o.export_charts=false}
 #[cfg(feature="pdf-layout")] if let Some(v)=&c.pdfium_lib_path{o.pdfium_lib_path=Some(v.clone())}
 if let Some(v)=&c.output{o.output=Some(v.clone())}if let Some(v)=c.format{o.format_choice=v}if let Some(v)=c.ocr{o.ocr_mode=v}if let Some(v)=&c.ocr_lang{o.ocr_lang=v.clone()}if let Some(v)=&c.ocr_command{o.ocr_command=v.clone()}
 if c.embed_assets==Some(true){o.embed_assets=true}if c.no_embed_assets==Some(true){o.embed_assets=false}if c.include_hidden_sheets==Some(true){o.include_hidden_sheets=true}if c.no_include_hidden_sheets==Some(true){o.include_hidden_sheets=false}if c.no_formulas==Some(true){o.preserve_formulas=false}if c.formulas==Some(true){o.preserve_formulas=true}if c.no_comments==Some(true){o.include_comments=false}if c.comments==Some(true){o.include_comments=true}if c.describe_images==Some(true){o.describe_images=true}if c.no_describe_images==Some(true){o.describe_images=false}if c.include_notes==Some(true){o.include_notes=true}if c.no_include_notes==Some(true){o.include_notes=false}if c.overwrite==Some(true){o.overwrite=true}if c.no_overwrite==Some(true){o.overwrite=false}if c.strict==Some(true){o.strict=true}if c.no_strict==Some(true){o.strict=false}if c.quiet==Some(true){o.quiet=true}if c.no_quiet==Some(true){o.quiet=false}if c.verbose==Some(true){o.verbose=true}if c.no_verbose==Some(true){o.verbose=false}if let Some(v)=c.report{o.report_kind=v}if let Some(v)=c.max_size{o.max_file_bytes=v.checked_mul(1024*1024).filter(|v|*v>0).ok_or_else(||ConvertError::Config("max-size requires positive MB".into()))?}Ok(o)}
fn main(){
    let cli=Cli::parse();let opts=match options(&cli){Ok(o)=>o,Err(e)=>{eprintln!("{e}");std::process::exit(2)}};
    if let Some(path)=&cli.preview{match convert::preview(path,&opts){Ok(s)=>print!("{s}"),Err(e)=>{eprintln!("{e}");std::process::exit(1)}}return}
    if cli.paths.is_empty(){eprintln!("provide at least one input path or --preview FILE");std::process::exit(2)}let batch=docconvert::batch::convert_batch(cli.paths.clone(),&opts);if batch.reports.is_empty(){eprintln!("no supported input files found");std::process::exit(1)}let total=batch.reports.len()+batch.not_started;let mut failed=0;let mut skipped=0;let mut converted=0;let mut warnings=0;for(i,(path,result))in batch.reports.iter().enumerate(){match result{Ok(r)=>{if r.status.starts_with("skipped"){skipped+=1}else{converted+=1}warnings+=r.warnings.len();if opts.verbose&&!opts.quiet{eprintln!("{}",r.to_text())}if !opts.quiet{println!("[{}/{}] {} → {} ({}, {} warnings; {})",i+1,total,r.input,r.outputs.first().map(String::as_str).unwrap_or("no output"),r.output_format,r.warnings.len(),r.status)}},Err(e)=>{failed+=1;eprintln!("{}: {e}",path.file_name().and_then(|s|s.to_str()).unwrap_or("input"));}}}if !opts.quiet{eprintln!("Batch: {converted} converted, {skipped} skipped, {failed} failed, {warnings} warnings, {} not started",batch.not_started)}if failed>0{std::process::exit(1)}}
#[cfg(test)]mod cli_precedence_tests{use super::*;use clap::Parser;#[test]fn negation_overrides_profile(){let cli=Cli::try_parse_from(["docconvert","--profile","strict","--no-strict","x.txt"]).unwrap();let o=options(&cli).unwrap();assert!(!o.strict);let cli=Cli::try_parse_from(["docconvert","--no-formulas","--formulas","x.txt"]).unwrap();assert!(options(&cli).unwrap().preserve_formulas);let cli=Cli::try_parse_from(["docconvert","--no-strip-headers-footers","--strip-headers-footers","x.txt"]).unwrap();assert!(options(&cli).unwrap().strip_headers_footers)}}
