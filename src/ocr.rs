//! Offline, user-selected Tesseract-compatible command. Never invokes a shell.
use crate::{config::{Options,OcrMode},detect::image_kind,error::{ConvertError,Result}};
use std::{fs,io::{Read,Write},path::PathBuf,process::Command,sync::atomic::{AtomicU64,Ordering}};
static NEXT:AtomicU64=AtomicU64::new(0);
const ABSENT:&str="OCR engine not found";
#[derive(Debug)]
pub struct OcrResult{pub text:String,pub confidence:Option<f32>}
struct Scratch(PathBuf);
impl Drop for Scratch{fn drop(&mut self){let _=fs::remove_dir_all(&self.0);}}
fn scratch()->Result<Scratch>{for _ in 0..100{let p=std::env::temp_dir().join(format!("docconvert-ocr-{}-{}",std::process::id(),NEXT.fetch_add(1,Ordering::Relaxed)));#[allow(unused_mut)]let mut b=fs::DirBuilder::new();#[cfg(unix)]{use std::os::unix::fs::DirBuilderExt;b.mode(0o700);}match b.create(&p){Ok(())=>return Ok(Scratch(p)),Err(e)if e.kind()==std::io::ErrorKind::AlreadyExists=>continue,Err(e)=>return Err(e.into())}}Err(ConvertError::Ocr("could not reserve temporary directory".into()))}
pub fn run_ocr(bytes:&[u8],opts:&Options)->Result<OcrResult>{
    if opts.ocr_mode==OcrMode::Off{return Err(ConvertError::Config("OCR adapter called while OCR is off".into()))}
    let temp=scratch()?;let input=temp.0.join(format!("input.{}",image_kind(bytes).map(|k|k.ext()).unwrap_or("bin")));let output=temp.0.join("output");
    let mut file=fs::OpenOptions::new().create_new(true).write(true).open(&input)?;file.write_all(bytes)?;drop(file);
    let result=Command::new(&opts.ocr_command).arg(&input).arg(&output).arg("-l").arg(&opts.ocr_lang).output().map_err(|e|ConvertError::Ocr(if e.kind()==std::io::ErrorKind::NotFound{ABSENT.into()}else{"could not start OCR engine".into()}))?;
    if !result.status.success(){let stderr=String::from_utf8_lossy(&result.stderr).replace(&*temp.0.to_string_lossy(),"<ocr-temp>");let detail:String=stderr.chars().filter(|c|!c.is_control()||*c=='\n').take(512).collect();return Err(ConvertError::Ocr(format!("engine exited unsuccessfully: {}",detail.trim())))}
    let path=output.with_extension("txt");if path.is_symlink(){return Err(ConvertError::Ocr("engine output is a symbolic link".into()))}
    let file=fs::File::open(path).map_err(|_|ConvertError::Ocr("engine did not produce output.txt".into()))?;let mut bytes=vec![];file.take(16*1024*1024+1).read_to_end(&mut bytes)?;if bytes.len()>16*1024*1024{return Err(ConvertError::Ocr("engine output exceeds 16 MiB".into()))}
    let text=String::from_utf8(bytes).map_err(|_|ConvertError::Ocr("engine text is not UTF-8".into()))?;Ok(OcrResult{text,confidence:None})
}
pub(crate) fn attempt_ocr(bytes:&[u8],opts:&Options,warnings:&mut Vec<String>)->Result<Option<OcrResult>>{
    if opts.ocr_mode==OcrMode::Off{return Ok(None)}
    match run_ocr(bytes,opts){Ok(result)=>{warnings.push(if result.text.trim().is_empty(){"OCR completed: no text recognized"}else{"OCR completed: recognized text; confidence unavailable"}.into());Ok(Some(result))},Err(ConvertError::Ocr(s))if s==ABSENT&&opts.ocr_mode==OcrMode::Auto=>{warnings.push("OCR requested but no engine found; original scanned/image content preserved".into());Ok(None)},Err(e)=>Err(e)}
}
