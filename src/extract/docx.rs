use super::{ooxml,package::{self,Package},ExtractCtx};
use crate::{error::Result,model::*};
use std::collections::HashMap;

pub fn extract_docx(bytes:&[u8],ctx:&mut ExtractCtx<'_>)->Result<Document>{
    let pkg=Package::open(bytes)?;let root=pkg.xml("word/document.xml")?;
    let rels=pkg.rels("word/document.xml");let mut doc=Document::default();package::metadata(&pkg,&mut doc.meta);
    let mut headings=HashMap::new();
    if let Some(styles)=pkg.optional_xml("word/styles.xml",ctx.warnings){for s in styles.desc("style"){
        let outline=s.desc("outlineLvl").next().and_then(|n|n.attr("val").parse::<usize>().ok()).map(|n|n+1)
            .or_else(||s.child("name").and_then(|n|n.attr("val").to_ascii_lowercase().strip_prefix("heading ").and_then(|s|s.parse().ok())));
        if let Some(level)=outline{headings.insert(s.attr("styleId").to_owned(),level.clamp(1,6));}
    }}
    let mut levels=HashMap::new();let mut nums=HashMap::new();
    if let Some(numbering)=pkg.optional_xml("word/numbering.xml",ctx.warnings){
        for a in numbering.desc("abstractNum"){for l in a.children().filter(|n|package::local(&n.name)=="lvl"){
            let ordered=l.child("numFmt").map(|n|!matches!(n.attr("val"),"bullet"|"none")).unwrap_or(false);
            let start=l.child("start").and_then(|n|n.attr("val").parse::<u64>().ok()).unwrap_or(1);
            levels.insert((a.attr("abstractNumId").to_owned(),l.attr("ilvl").to_owned()),(ordered,start));
        }}
        for n in numbering.desc("num"){if let Some(a)=n.child("abstractNumId"){nums.insert(n.attr("numId").to_owned(),a.attr("val").to_owned());}}
    }
    for part in ["word/footnotes.xml","word/endnotes.xml"]{
        if let Some(notes)=pkg.optional_xml(part,ctx.warnings){let key=if part.contains("endnotes"){"endnote"}else{"footnote"};
            for n in notes.desc(key){if matches!(n.attr("type"),"separator"|"continuationSeparator"){continue}doc.footnotes.push((n.attr("id").into(),ooxml::run_inlines(&n,&rels,ctx.warnings)));}
        }
    }
    let body=root.desc("body").next().unwrap_or(root);let mut last_num=String::new();
    for n in body.children(){match package::local(&n.name){
        "p"=>{
            let inline=ooxml::run_inlines(&n,&rels,ctx.warnings);let pr=n.child("pPr");
            let style=pr.as_ref().and_then(|p|p.child("pStyle")).map(|n|n.attr("val").to_owned()).unwrap_or_default();
            let num=pr.as_ref().and_then(|p|p.child("numPr"));
            let num_id=num.as_ref().and_then(|p|p.child("numId")).map(|n|n.attr("val").to_owned()).unwrap_or_default();
            let level=num.as_ref().and_then(|p|p.child("ilvl")).map(|n|n.attr("val").to_owned()).unwrap_or_else(||"0".into());
            if style.eq_ignore_ascii_case("caption"){if let Some(Block::Image{caption,..})=doc.blocks.last_mut(){*caption=Some(inline_text(&inline));continue}}
            if let Some(&level)=headings.get(&style){doc.blocks.push(Block::Heading{level,inlines:inline});last_num.clear()}
            else if let Some(&(ordered,start))=nums.get(&num_id).and_then(|id|levels.get(&(id.clone(),level.clone()))){
                let item=ListItem{level:level.parse().unwrap_or(0),blocks:vec![Block::Paragraph(inline)]};
                if last_num==num_id{if let Some(Block::List{ordered:old,items,..})=doc.blocks.last_mut(){if *old==ordered{items.push(item)}else{doc.blocks.push(Block::List{ordered,start:ordered.then_some(start),items:vec![item]})}}else{doc.blocks.push(Block::List{ordered,start:ordered.then_some(start),items:vec![item]})}}
                else{doc.blocks.push(Block::List{ordered,start:ordered.then_some(start),items:vec![item]})}last_num=num_id;
            }else{
                last_num.clear();if !num_id.is_empty(){ctx.warnings.push("numbering definition missing; list preserved as paragraph".into())}
                if inline.len()==1{if let Inline::MathInline(s)=&inline[0]{doc.blocks.push(Block::MathBlock(s.clone()))}else if !inline_text(&inline).is_empty(){doc.blocks.push(Block::Paragraph(inline))}}
                else if !inline.is_empty(){doc.blocks.push(Block::Paragraph(inline))}
            }
            doc.blocks.extend(ooxml::images(&n,&pkg,"word/document.xml",&rels,ctx));
        },
        "tbl"=>{last_num.clear();doc.blocks.push(Block::Table(ooxml::table(&n,'w',&rels,ctx.warnings)))},_=>{},
    }}
    Ok(doc)
}
