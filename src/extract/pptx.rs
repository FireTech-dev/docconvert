use super::{ooxml,package::{self,Element,Package,Relationship},ExtractCtx};
use crate::{error::{ConvertError,Result},model::*};
use std::collections::BTreeMap;

fn is_title(shape:&Element)->bool{shape.desc("ph").any(|n|matches!(n.attr("type"),"title"|"ctrTitle"))}
fn shape_blocks(root:&Element,pkg:&Package,part:&str,rels:&BTreeMap<String,Relationship>,ctx:&mut ExtractCtx<'_>)->Vec<Block>{
    let mut blocks=vec![];
    for object in root.children(){match package::local(&object.name){
        "sp"=>{
            if is_title(&object){continue}
            for p in object.desc("p"){
                let xs=ooxml::run_inlines(&p,rels,ctx.warnings);let pr=p.child("pPr");
                let bullet=pr.as_ref().and_then(|n|n.child("buChar")).is_some();
                let numbered=pr.as_ref().and_then(|n|n.child("buAutoNum"));
                if bullet||numbered.is_some(){
                    let ordered=numbered.is_some();let start=numbered.and_then(|n|n.attr("startAt").parse().ok()).unwrap_or(1);
                    let level=pr.as_ref().and_then(|n|n.attr("lvl").parse().ok()).unwrap_or(0);
                    let item=ListItem{level,blocks:vec![Block::Paragraph(xs)]};
                    if let Some(Block::List{ordered:o,items,..})=blocks.last_mut(){if *o==ordered{items.push(item);continue}}
                    blocks.push(Block::List{ordered,start:ordered.then_some(start),items:vec![item]});
                }else if !inline_text(&xs).trim().is_empty(){blocks.push(Block::Paragraph(xs))}
            }
        },
        "pic"=>blocks.extend(ooxml::images(&object,pkg,part,rels,ctx)),
        "graphicFrame"=>{
            for table in object.desc("tbl"){blocks.push(Block::Table(ooxml::table(&table,'a',rels,ctx.warnings)))}
            for chart in object.desc("chart"){
                let title=rels.get(chart.attr("id")).filter(|r|!r.external).and_then(|r|pkg.resolve(part,&r.target)).and_then(|p|pkg.xml(&p).ok()).and_then(|n|n.desc("title").next().map(|n|n.text()));
                blocks.push(Block::Placeholder{kind:PlaceholderKind::Chart,label:title,asset:None});
                ctx.warnings.push("chart inventory preserved; chart rasterization is not supported".into());
            }
        },
        "grpSp"=>blocks.extend(shape_blocks(&object,pkg,part,rels,ctx)),
        _=>{},
    }}blocks
}
pub fn extract_pptx(bytes:&[u8],ctx:&mut ExtractCtx<'_>)->Result<Document>{
    let pkg=Package::open(bytes)?;let root=pkg.xml("ppt/presentation.xml")?;let presentation_rels=pkg.rels("ppt/presentation.xml");
    let mut doc=Document::default();package::metadata(&pkg,&mut doc.meta);
    let slides:Vec<_>=root.desc("sldId").collect();doc.meta.slide_count=Some(slides.len());
    for(i,entry)in slides.iter().enumerate(){
        let rel=presentation_rels.get(entry.attr("r:id")).filter(|r|!r.external).ok_or_else(||ConvertError::Corrupt("slide relationship missing".into()))?;
        let part=pkg.resolve("ppt/presentation.xml",&rel.target).ok_or_else(||ConvertError::Corrupt("unsafe slide path".into()))?;
        let slide=pkg.xml(&part)?;let rels=pkg.rels(&part);
        let title=slide.desc("sp").find(is_title).map(|shape|shape.desc("p").map(|p|inline_text(&ooxml::run_inlines(&p,&rels,ctx.warnings))).collect::<Vec<_>>().join(" "));
        if i>0{doc.blocks.push(Block::Boundary(Boundary::Slide{n:i+1,title:title.clone()}))}
        if let Some(title)=title{doc.blocks.push(Block::Heading{level:1,inlines:vec![Inline::Text(title)]})}
        if let Some(tree)=slide.desc("spTree").next(){doc.blocks.extend(shape_blocks(&tree,&pkg,&part,&rels,ctx))}
        if ctx.options.include_notes{
            for rel in rels.values().filter(|r|r.kind.ends_with("/notesSlide")&&!r.external){
                if let Some(path)=pkg.resolve(&part,&rel.target){
                    if let Some(notes)=pkg.optional_xml(&path,ctx.warnings){
                        let notes_rels=pkg.rels(&path);let mut content=vec![Block::Paragraph(vec![Inline::Styled{style:TextStyle{italic:true,..TextStyle::default()},children:vec![Inline::Text("Speaker notes:".into())]}])];
                        for shape in notes.desc("sp"){
                            if shape.desc("ph").any(|n|matches!(n.attr("type"),"sldNum"|"dt"|"hdr"|"ftr"|"sldImg")){continue}
                            for p in shape.desc("p"){let xs=ooxml::run_inlines(&p,&notes_rels,ctx.warnings);if !inline_text(&xs).trim().is_empty(){content.push(Block::Paragraph(xs))}}
                        }
                        doc.blocks.push(Block::Quote(content));
                    }
                }
            }
        }
    }
    Ok(doc)
}
