use super::{html, package::{Element,Package}, ExtractCtx};
use crate::{error::{ConvertError,Result},model::*};
use std::collections::HashMap;

pub fn extract_epub(bytes:&[u8],ctx:&mut ExtractCtx<'_>)->Result<Document>{
    let pkg=Package::open(bytes)?;
    let container=pkg.xml("META-INF/container.xml")?;
    let opf=container.desc("rootfile").next().map(|n|n.attr("full-path").to_owned()).ok_or_else(||ConvertError::Corrupt("EPUB rootfile missing".into()))?;
    let root=pkg.xml(&opf)?;
    let mut doc=Document::default();
    doc.meta.title=root.desc("title").next().map(|n|n.text());
    doc.meta.author=root.desc("creator").next().map(|n|n.text());
    doc.meta.language=root.desc("language").next().map(|n|n.text());
    let manifest:HashMap<String,Element>=root.desc("item").map(|n|(n.attr("id").into(),n)).collect();
    let spine:Vec<_>=root.desc("itemref").collect();
    doc.meta.chapter_count=Some(spine.len());
    let mut chapters=vec![]; let mut notes=HashMap::new();
    for (i,itemref) in spine.iter().enumerate(){
        let item=manifest.get(itemref.attr("idref")).ok_or_else(||ConvertError::Corrupt("EPUB spine references missing manifest item".into()))?;
        let part=pkg.resolve(&opf,item.attr("href")).ok_or_else(||ConvertError::Corrupt("unsafe EPUB chapter path".into()))?;
        let data=pkg.get(&part).ok_or_else(||ConvertError::Corrupt("EPUB chapter missing".into()))?;
        let mut loader=|src:&str|pkg.resolve(&part,src).and_then(|p|pkg.get(&p)).map(<[u8]>::to_vec);
        let mut chapter=html::extract_epub_html(data,Meta::default(),&mut loader,ctx);
        for (id,_) in &mut chapter.footnotes { let unique=format!("ch{}-{id}",i+1); notes.insert(format!("{part}#{id}"),unique.clone()); *id=unique; }
        chapters.push((part,chapter));
    }
    for (i,(part,mut chapter)) in chapters.into_iter().enumerate(){
        let mut rewrite=|inline:&mut Inline|{
            if let Inline::Link{url,..}=inline {
                if let Some((target,fragment))=url.split_once('#') {
                    let target=if target.is_empty(){Some(part.clone())}else{pkg.resolve(&part,target)};
                    if let Some(id)=target.and_then(|p|notes.get(&format!("{p}#{fragment}"))).cloned(){*inline=Inline::FootnoteRef(id)}
                }
            }
        };
        walk_inlines_mut(&mut chapter.blocks,&mut rewrite);
        for (_,xs) in &mut chapter.footnotes { let mut wrapper=vec![Block::Paragraph(std::mem::take(xs))];walk_inlines_mut(&mut wrapper,&mut rewrite);if let Some(Block::Paragraph(v))=wrapper.pop(){*xs=v;} }
        let title=chapter.blocks.iter().find_map(|b|if let Block::Heading{inlines,..}=b{Some(inline_text(inlines))}else{None});
        if i>0{doc.blocks.push(Block::Boundary(Boundary::Chapter(title)))}
        doc.blocks.extend(chapter.blocks);doc.footnotes.extend(chapter.footnotes);doc.warnings.extend(chapter.warnings);
    }
    let cover_id=root.desc("meta").find(|n|n.attr("name")=="cover").map(|n|n.attr("content").to_owned());
    let cover=manifest.values().find(|n|n.attr("properties").split_whitespace().any(|p|p=="cover-image")).or_else(||cover_id.as_ref().and_then(|id|manifest.get(id)));
    if let Some(cover)=cover{if let Some(path)=pkg.resolve(&opf,cover.attr("href")){if let Some(b)=pkg.get(&path){ctx.assets.add(b.to_vec(),"EPUB cover",Some("Cover".into()));}}}
    Ok(doc)
}
