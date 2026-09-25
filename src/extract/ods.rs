use super::ExtractCtx;
use crate::{error::Result,model::Document};
pub fn extract_ods(bytes:&[u8],ctx:&mut ExtractCtx<'_>)->Result<Document>{super::xlsx::read_workbook(bytes,ctx)}
