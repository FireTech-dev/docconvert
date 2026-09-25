use super::ExtractCtx;
use crate::{error::Result,model::*};
fn word(b:&[u8],i:usize,le:bool)->Option<u16>{let a:[u8;2]=b.get(i..i.checked_add(2)?)?.try_into().ok()?;Some(if le{u16::from_le_bytes(a)}else{u16::from_be_bytes(a)})}
fn dword(b:&[u8],i:usize,le:bool)->Option<u32>{let a:[u8;4]=b.get(i..i.checked_add(4)?)?.try_into().ok()?;Some(if le{u32::from_le_bytes(a)}else{u32::from_be_bytes(a)})}
fn triple(b:&[u8],i:usize)->Option<u32>{let s=b.get(i..i.checked_add(3)?)?;Some(s[0] as u32|((s[1] as u32)<<8)|((s[2] as u32)<<16))}
pub fn read_dimensions(b:&[u8],kind:ImageKind)->Option<(u32,u32)>{
    let size=match kind{
        ImageKind::Png=>{if b.get(12..16)!=Some(b"IHDR"){return None}(dword(b,16,false)?,dword(b,20,false)?)},
        ImageKind::Gif=>(word(b,6,true)? as u32,word(b,8,true)? as u32),
        ImageKind::Bmp=>{let header=dword(b,14,true)?;if header==12{(word(b,18,true)? as u32,word(b,20,true)? as u32)}else{let w=dword(b,18,true)? as i32;let h=dword(b,22,true)? as i32;// A negative height means top-down storage; only the magnitude is a dimension (pixel data is never read here).
(w.checked_abs()? as u32,h.checked_abs()? as u32)}},
        ImageKind::Jpeg=>{
            let mut i=2;let mut found=None;
            while i<b.len(){if b[i]!=0xff{return None}while b.get(i)==Some(&0xff){i+=1}let marker=*b.get(i)?;i+=1;
                if matches!(marker,0xd9|0xda){break}if marker==0x01||(0xd0..=0xd8).contains(&marker){continue}
                let length=word(b,i,false)? as usize;if length<2||i.checked_add(length)?>b.len(){return None}
                if matches!(marker,0xc0..=0xc3|0xc5..=0xc7|0xc9..=0xcb|0xcd..=0xcf){found=Some((word(b,i+5,false)? as u32,word(b,i+3,false)? as u32));break}i+=length;
            }found?
        },
        ImageKind::Webp=>{
            if b.get(8..12)!=Some(b"WEBP"){return None}let mut i:usize=12;let mut found=None;
            while i.checked_add(8)?<=b.len(){let tag=b.get(i..i+4)?;let length=dword(b,i+4,true)? as usize;let start=i+8;let end=start.checked_add(length)?;if end>b.len(){return None}
                if tag==b"VP8X"&&length>=10{found=Some((triple(b,start+4)?+1,triple(b,start+7)?+1));break}
                if tag==b"VP8 "&&length>=10&&b.get(start+3..start+6)==Some(b"\x9d\x01\x2a"){found=Some(((word(b,start+6,true)?&0x3fff)as u32,(word(b,start+8,true)?&0x3fff)as u32));break}
                if tag==b"VP8L"&&length>=5&&b.get(start)==Some(&0x2f){let bits=dword(b,start+1,true)?;found=Some(((bits&0x3fff)+1,((bits>>14)&0x3fff)+1));break}i=end.checked_add(length%2)?;
            }found?
        },
        ImageKind::Tiff=>{
            let le=match b.get(..2)?{b"II"=>true,b"MM"=>false,_=>return None};let offset=dword(b,4,le)? as usize;let n=word(b,offset,le)? as usize;if n>4096{return None}let mut w=None;let mut h=None;
            for k in 0..n{let pos=offset.checked_add(2)?.checked_add(k.checked_mul(12)?)?;let tag=word(b,pos,le)?;let typ=word(b,pos+2,le)?;let count=dword(b,pos+4,le)?;if count!=1{continue}let val=match typ{3=>word(b,pos+8,le)? as u32,4=>dword(b,pos+8,le)?,_=>continue};if tag==256{w=Some(val)}if tag==257{h=Some(val)}}(w?,h?)
        },
        ImageKind::Jp2=>{
            fn boxes(b:&[u8],depth:usize)->Option<(u32,u32)>{if depth>8{return None}let mut i:usize=0;while i.checked_add(8)?<=b.len(){let length=dword(b,i,false)? as usize;let tag=b.get(i+4..i+8)?;let(header,length)=if length==1{let high=dword(b,i+8,false)?;if high!=0{return None}(16,dword(b,i+12,false)? as usize)}else{(8,if length==0{b.len()-i}else{length})};if length<header{return None}let end=i.checked_add(length)?;let payload=b.get(i+header..end)?;if tag==b"ihdr"{return Some((dword(payload,4,false)?,dword(payload,0,false)?))}if tag==b"jp2h"{if let Some(size)=boxes(payload,depth+1){return Some(size)}}i=end;}None}boxes(b,0)?
        },
    };
    if size.0==0||size.1==0{None}else{Some(size)}
}
pub fn extract_image(bytes:&[u8],meta:Meta,kind:ImageKind,ctx:&mut ExtractCtx<'_>)->Result<Document>{
    let mut doc=Document{meta,..Document::default()};
    let size=read_dimensions(bytes,kind);if size.is_none(){doc.warnings.push("image dimensions could not be read; original asset preserved".into())}
    let asset=ctx.assets.add(bytes.to_vec(),"standalone image",None);doc.blocks.push(Block::Image{asset,alt:None,caption:None});
    if let Some(result)=crate::ocr::attempt_ocr(bytes,ctx.options,&mut doc.warnings)?{
        if !result.text.trim().is_empty(){doc.blocks.extend(super::text::extract_txt(result.text.as_bytes(),Meta::default()).blocks)}
    }
    Ok(doc)
}

/// PDFium exposes RGBA pixels, not a direct PNG export in the inspected API.
/// Encode the extracted bitmap with existing flate2; no image-decoder dependency.
#[cfg(feature="pdf-layout")]
pub(crate) fn rgba_png(width:u32,height:u32,rgba:&[u8])->Result<Vec<u8>>{
    use std::io::Write;
    use crate::error::ConvertError;
    let pixels=(width as usize).checked_mul(height as usize).filter(|n|*n<=20_000_000).ok_or_else(||ConvertError::Corrupt("image pixel limit exceeded".into()))?;
    if width==0||height==0||rgba.len()!=pixels*4{return Err(ConvertError::Corrupt("invalid RGBA bitmap".into()))}
    fn crc(data:&[u8])->u32{let mut c=0xffff_ffffu32;for b in data{c^=*b as u32;for _ in 0..8{c=(c>>1)^if c&1!=0{0xedb8_8320}else{0}}}!c}
    fn chunk(out:&mut Vec<u8>,tag:&[u8;4],data:&[u8]){out.extend((data.len()as u32).to_be_bytes());let start=out.len();out.extend(tag);out.extend(data);out.extend(crc(&out[start..]).to_be_bytes());}
    let mut z=flate2::write::ZlibEncoder::new(Vec::new(),flate2::Compression::default());
    for row in rgba.chunks_exact(width as usize*4){z.write_all(&[0])?;z.write_all(row)?;}let compressed=z.finish()?;
    let mut out=b"\x89PNG\r\n\x1a\n".to_vec();let mut ihdr=vec![];ihdr.extend(width.to_be_bytes());ihdr.extend(height.to_be_bytes());ihdr.extend([8,6,0,0,0]);chunk(&mut out,b"IHDR",&ihdr);chunk(&mut out,b"IDAT",&compressed);chunk(&mut out,b"IEND",&[]);Ok(out)
}
