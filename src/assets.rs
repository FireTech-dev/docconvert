use crate::{detect::image_kind,error::{ConvertError,Result},model::Asset};
use std::{collections::{HashMap,hash_map::DefaultHasher},hash::{Hash,Hasher},path::Path,io::Write};
#[derive(Debug,Default)]
pub struct AssetManager {pub assets:Vec<Asset>,by_hash:HashMap<u64,Vec<usize>>}
impl AssetManager {
 pub fn add(&mut self,bytes:Vec<u8>,origin:&str,alt:Option<String>)->usize{
 // DefaultHasher is process-stable, not a cross-version content identity. Compare bytes on collisions.
 let mut h=DefaultHasher::new();bytes.hash(&mut h);let hash=h.finish();if let Some(ids)=self.by_hash.get(&hash){for &id in ids{if self.assets[id].bytes==bytes{return id}}}
 let (ext,mime)=image_kind(&bytes).map(|k|(k.ext(),k.mime())).unwrap_or(("bin","application/octet-stream"));let id=self.assets.len();let filename=format!("image-{:03}.{ext}",id+1);
 self.assets.push(Asset{rel_path:format!("assets/{filename}"),filename,mime:mime.into(),bytes,alt,origin:origin.into(),hash,duplicate_of:None});self.by_hash.entry(hash).or_default().push(id);id
 }
 pub fn write_all(&self,dir:&Path)->Result<usize>{let dest=dir.join("assets");if self.assets.is_empty(){return Ok(0)}if dest.is_symlink(){return Err(ConvertError::Output("assets directory is a symbolic link".into()))}std::fs::create_dir_all(&dest)?;
 for a in &self.assets{if a.filename.contains('/')||a.filename.contains('\\'){return Err(ConvertError::Output("unsafe asset filename".into()))}let path=dest.join(&a.filename);let mut file=std::fs::OpenOptions::new().write(true).create_new(true).open(path)?;file.write_all(&a.bytes)?;}Ok(self.assets.len())}
 pub fn manifest(&self)->serde_json::Value{serde_json::json!({"assets":self.assets.iter().map(|a|serde_json::json!({"filename":a.filename,"mime":a.mime,"bytes":a.bytes.len(),"origin":a.origin,"duplicate_of":a.duplicate_of})).collect::<Vec<_>>()})}
}
