use crate::{config::{Options,FormatChoice,ReportKind},detect::{detect_file,Detection},error::{ConvertError,Result},extract,model::{Document,Format},render::{self,OutFormat},report::Report};
use std::{fs,io::Write,path::{Path,PathBuf},sync::atomic::{AtomicU64,Ordering}};

fn load(path:&Path,opts:&Options,passthrough:bool)->Result<(Detection,Document,OutFormat,String)>{
    if fs::metadata(path)?.len()>opts.max_file_bytes{return Err(ConvertError::Unsupported("file exceeds --max-size".into()))}
    let detection=detect_file(path)?;
    if detection.encrypted{return Err(ConvertError::Encrypted("file is encrypted; password-protected files are not supported".into()))}
    let mut doc=if passthrough&&detection.format==Format::Markdown&&opts.format_choice==FormatChoice::Md{
        Document::new(path.file_name().and_then(|s|s.to_str()).unwrap_or("input"),Format::Markdown)
    }else{extract::extract_file(path,&detection,opts)?};
    let(format,reason)=render::choose_output(&doc,opts.format_choice);
    if passthrough&&detection.format==Format::Markdown&&format==OutFormat::Md{doc.warnings.push("Markdown passthrough preserves source-relative references; keep referenced files available beside the copied document".into())}
    Ok((detection,doc,format,reason))
}
pub fn preview(path:&Path,opts:&Options)->Result<String>{
    let(d,doc,f,reason)=load(path,opts,false)?;
    let report=Report::from_document(&doc,&d,f,reason,opts);
    Ok(format!("{}Title:       {}\nText length: {}\n",report.to_text(),doc.meta.title.as_deref().unwrap_or(""),doc.text_len()))
}
static NEXT_STAGE:AtomicU64=AtomicU64::new(0);
fn reserve(parent:&Path,kind:&str)->Result<(PathBuf,fs::File)>{
    for _ in 0..100{
        let serial=NEXT_STAGE.fetch_add(1,Ordering::Relaxed);
        let path=parent.join(format!(".docconvert-{kind}-{}-{serial}",std::process::id()));
        match fs::OpenOptions::new().write(true).create_new(true).open(&path){
            Ok(file)=>return Ok((path,file)),
            Err(error)if error.kind()==std::io::ErrorKind::AlreadyExists=>continue,
            Err(error)=>return Err(error.into()),
        }
    }
    Err(ConvertError::Output("could not reserve a unique staging file".into()))
}
struct Staged { target:PathBuf, temporary:PathBuf, backup:Option<PathBuf>, committed:bool }
impl Drop for Staged { fn drop(&mut self){let _=fs::remove_file(&self.temporary);} }
fn rollback(files:&mut [Staged])->bool{
    let mut clean=true;
    for file in files.iter_mut().rev(){
        if file.committed{if fs::remove_file(&file.target).is_err(){clean=false;continue}file.committed=false;}
        if let Some(backup)=file.backup.take(){if fs::rename(&backup,&file.target).is_err(){file.backup=Some(backup);clean=false;}}
    }
    clean
}
/// Stage all files first; rollback ordinary commit errors. Not a power-loss atomic
/// multi-file transaction. Recovery backups are intentionally retained if rollback fails.
fn write_outputs(outputs:&[(PathBuf,Vec<u8>)],overwrite:bool)->Result<()>{
    for(path,_)in outputs{
        if path.is_symlink(){return Err(ConvertError::Output("refusing to write through a symbolic link".into()))}
        if path.exists()&&(!overwrite||!path.is_file()){return Err(ConvertError::Output("output exists or is not a regular file".into()))}
        let parent=path.parent().unwrap_or(Path::new("."));
        if parent.is_symlink(){return Err(ConvertError::Output("output parent is a symbolic link".into()))}
    }
    let mut staged:Vec<Staged>=vec![];
    for(path,bytes)in outputs{
        let parent=path.parent().unwrap_or(Path::new("."));fs::create_dir_all(parent)?;
        let(temporary,mut file)=reserve(parent,"stage")?;
        staged.push(Staged{target:path.clone(),temporary,backup:None,committed:false});
        file.write_all(bytes)?;file.sync_all()?;
    }
    for index in 0..staged.len(){
        let outcome=(||->Result<()>{
            let file=&mut staged[index];
            if file.target.is_symlink(){return Err(ConvertError::Output("destination became a symbolic link".into()))}
            if file.target.exists(){
                if !overwrite||!file.target.is_file(){return Err(ConvertError::Output("destination changed during conversion".into()))}
                let parent=file.target.parent().unwrap_or(Path::new("."));
                let(backup,handle)=reserve(parent,"backup")?;drop(handle);
                // The reservation is ours; removing it allows rename on Windows too.
                fs::remove_file(&backup)?;
                fs::rename(&file.target,&backup)?;file.backup=Some(backup);
            }
            // A hard link publishes the fully written file without replacing a new
            // concurrent destination. Staging is on the same filesystem as its target.
            fs::hard_link(&file.temporary,&file.target)?;file.committed=true;
            Ok(())
        })();
        if let Err(error)=outcome{
            return if rollback(&mut staged){Err(error)}else{Err(ConvertError::Output("commit and rollback failed; preserve .docconvert-backup-* files for recovery".into()))};
        }
    }
    for file in &mut staged{if let Some(backup)=file.backup.take(){if fs::remove_file(&backup).is_err(){return Err(ConvertError::Output("output committed but recovery-backup cleanup failed; preserve .docconvert-backup-* files".into()))}}}
    Ok(())
}
/// Asset subdirectory for one input stem. Readable stems pass through; anything
/// else is hex-encoded. Either form is capped so the result always fits in a
/// single path component (AQ-011: an uncapped `~hex` encoding of a 164-byte stem
/// produced 329 bytes and failed with ENAMETOOLONG). Overlong names fall back to
/// a readable prefix plus a stem hash; the report manifest keeps full traceability.
fn asset_namespace(stem:&str)->(String,bool){
    let raw=if stem.bytes().all(|b|b.is_ascii_alphanumeric()||b==b'-'||b==b'_'){stem.to_owned()}else{format!("~{}",stem.as_bytes().iter().map(|b|format!("{b:02x}")).collect::<String>())};
    if raw.len()<=200{(raw,false)}else{
        use std::collections::hash_map::DefaultHasher;use std::hash::{Hash,Hasher};
        let mut hasher=DefaultHasher::new();stem.hash(&mut hasher);
        let short:String=stem.chars().filter(|c|c.is_ascii_alphanumeric()||*c=='-'||*c=='_').take(32).collect();
        (format!("long-{short}-{h:016x}",h=hasher.finish()),true)
    }
}
pub fn convert_file(path:&Path,opts:&Options)->Result<Report>{
    let started=std::time::Instant::now();let(detection,mut doc,format,reason)=load(path,opts,true)?;
    let dir=opts.output.clone().unwrap_or_else(||extract::package::parent_or_dot(path).to_path_buf());
    let stem=path.file_stem().and_then(|s|s.to_str()).unwrap_or("document");
    let(namespace,shortened)=asset_namespace(stem);
    if shortened{doc.warnings.push("long input filename shortened in asset paths; outputs keep the full name".into())}
    let destination=dir.join(format!("{stem}.{}",format.ext()));
    let mut report=Report::from_document(&doc,&detection,format,reason,opts);
    if destination.exists()&&!opts.overwrite{report.status="skipped (output exists)".into();return Ok(report)}
    if destination.exists()&&destination.canonicalize()?==path.canonicalize()?{return Err(ConvertError::Output("refusing to overwrite the source document; select another output directory".into()))}
    if dir.is_symlink(){return Err(ConvertError::Output("output directory is a symbolic link".into()))}
    let asset_dir=dir.join("assets").join(&namespace);
    if dir.join("assets").is_symlink()||asset_dir.is_symlink(){return Err(ConvertError::Output("asset directory is a symbolic link".into()))}
    for asset in &mut doc.assets{asset.rel_path=format!("assets/{namespace}/{}",asset.filename)}
    let passthrough=detection.format==Format::Markdown&&format==OutFormat::Md;
    let rendered=if passthrough{fs::read(path)?}else{match format{
        OutFormat::Md=>render::markdown::render_markdown(&doc,opts.embed_assets).into_bytes(),
        OutFormat::Txt=>render::txt::render_txt_with_options(&doc,opts.minimal_placeholders).into_bytes(),
    }};
    let report_path=match opts.report_kind{ReportKind::None=>None,ReportKind::Text=>Some(dir.join(format!("{stem}.report.txt"))),ReportKind::Json=>Some(dir.join(format!("{stem}.report.json")))};
    let mut outputs=vec![];
    if !passthrough{for asset in &doc.assets{outputs.push((asset_dir.join(&asset.filename),asset.bytes.clone()))}}
    report.assets=if passthrough{vec![]}else{doc.assets.iter().map(|a|a.rel_path.clone()).collect()};
    report.outputs.push(destination.file_name().and_then(|n|n.to_str()).unwrap_or("").into());
    outputs.push((destination,rendered));report.duration_ms=started.elapsed().as_millis();
    if let Some(path)=report_path{
        report.outputs.push(path.file_name().and_then(|n|n.to_str()).unwrap_or("").into());
        let text=if opts.report_kind==ReportKind::Json{report.to_json()?}else{report.to_text()};outputs.push((path,text.into_bytes()));
    }
    write_outputs(&outputs,opts.overwrite)?;
    Ok(report)
}
/// Compatibility helper for immediate directory children. See batch for new CLI scanning.
pub fn immediate_inputs(paths:&[PathBuf])->Vec<PathBuf>{
    let mut out=vec![];
    for path in paths{
        if path.is_dir(){if let Ok(entries)=fs::read_dir(path){let mut files:Vec<_>=entries.flatten().map(|e|e.path()).filter(|p|p.is_file()&&p.extension().and_then(|s|s.to_str()).map(|s|!crate::detect::expected_formats(&s.to_ascii_lowercase()).is_empty()).unwrap_or(false)).collect();files.sort();out.extend(files)}}else{out.push(path.clone())}
    }out
}
pub fn explicit_markdown_options()->Options{Options{format_choice:FormatChoice::Md,..Options::default()}}

// Blueprint-compatible library entry points.
pub use crate::batch::{expand_inputs,convert_batch,BatchResult};

#[cfg(test)]mod tests{
    use super::asset_namespace;
    #[test]fn namespace_readable_passthrough(){assert_eq!(asset_namespace("report-2024_final"),("report-2024_final".into(),false))}
    #[test]fn namespace_hex_short_kept(){let(ns,short)=asset_namespace("a b");assert!(!short);assert!(ns.starts_with('~'))}
    #[test]fn namespace_long_bounded(){let stem="word ".repeat(30);let(ns,short)=asset_namespace(&stem);assert!(short);assert!(ns.starts_with("long-"));assert!(ns.len()<=200);assert_eq!(asset_namespace(&stem).0,ns)}
    #[test]fn namespace_corpus_case_bounded(){let base="How to Become an Expert Software Engineer (and Get Any Job You Want)";let stem=format!("{base} {base}");let(ns,short)=asset_namespace(&stem);assert!(short);assert!(ns.len()<=200)}
}
