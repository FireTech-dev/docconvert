pub mod markdown;
pub mod txt;
use crate::{config::FormatChoice,model::*};
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum OutFormat {Md,Txt}
impl OutFormat {pub fn ext(self)->&'static str{match self{Self::Md=>"md",Self::Txt=>"txt"}}}
pub fn choose_output(doc:&Document,choice:FormatChoice)->(OutFormat,String){match choice{FormatChoice::Md=>return(OutFormat::Md,"user-specified".into()),FormatChoice::Txt=>return(OutFormat::Txt,"user-specified".into()),_=>{}}
 let mut score=0;let mut signals=vec![];let mut heading=std::collections::BTreeSet::new();
 fn inspect(bs:&[Block],score:&mut usize,signals:&mut Vec<&'static str>,heads:&mut std::collections::BTreeSet<usize>){for b in bs{match b{Block::Table(_)=>{*score+=2;signals.push("tables")},Block::Image{..}|Block::Placeholder{asset:Some(_),..}=>{*score+=2;signals.push("images")},Block::CodeBlock{..}=>signals.push("code"),Block::MathBlock(_)=>signals.push("math"),Block::Heading{level,..}=>{heads.insert(*level);},Block::Boundary(Boundary::Slide{..}|Boundary::Chapter(_))=>{*score+=2;signals.push("slide/chapter boundaries")},Block::List{items,..}=>for i in items{if i.level>0{signals.push("nested lists")}inspect(&i.blocks,score,signals,heads)},Block::Quote(bs)=>inspect(bs,score,signals,heads),_=>{}}}}
 inspect(&doc.blocks,&mut score,&mut signals,&mut heading);let mut blocks=doc.blocks.clone();walk_inlines_mut(&mut blocks,&mut|i|match i{Inline::Link{..}=>signals.push("links"),Inline::MathInline(_)=>signals.push("math"),Inline::Image{..}=>{score+=2;signals.push("images")},_=>{}});signals.sort_unstable();signals.dedup();for signal in ["code","math","links","nested lists"]{if signals.contains(&signal){score+=1}}if !doc.footnotes.is_empty(){score+=1;signals.push("footnotes")}if heading.len()>1{score+=1;signals.push("heading levels")}if doc.workbook.is_some(){score+=2;signals.push("workbook")}
 let format=if score>=2{OutFormat::Md}else{OutFormat::Txt};let reason=if signals.is_empty(){"plain text is sufficient".into()}else{signals.join(", ")};(format,format!("{}: {reason}",format.ext()))
}

/// Expand implicit covered cells without shifting the remaining cells left.
/// Layout-only copies; the document's semantic table retains its real spans.
pub(crate) fn expanded_rows(table:&Table)->Vec<Vec<TableCell>>{
    let mut output=vec![];let mut pending:Vec<usize>=vec![];
    for source in std::iter::once(&table.header).chain(table.rows.iter()){
        let mut row=vec![TableCell::default();pending.len()];let mut column=0;
        for cell in source{
            while column<pending.len()&&pending[column]>0{column+=1}
            if column>=1000{break}
            let span=cell.colspan.clamp(1,1000-column);let end=column+span;
            row.resize(row.len().max(end),TableCell::default());pending.resize(pending.len().max(end),0);
            let mut value=cell.clone();value.colspan=1;value.rowspan=1;row[column]=value;
            for slot in &mut pending[column..end]{*slot=cell.rowspan.max(1)}
            column=end;
        }
        for n in &mut pending{*n=n.saturating_sub(1)}
        output.push(row);
    }
    let width=output.iter().map(Vec::len).max().unwrap_or(0);
    for row in &mut output{row.resize(width,TableCell::default())}
    output
}
