use super::{package::{self,Content,Element,Package,Relationship},ExtractCtx};
use crate::model::*;
use std::collections::{HashMap,BTreeMap};

// A5 decision: `Package::rels` (rich Relationship records) is the single canonical
// rels helper — the older string-map `parse_rels` had no callers left and is removed.
// A4 decision: run-property matching is namespace-agnostic (`package::local`),
// so the unused `ns` parameter is removed rather than threaded through untouched.
pub fn parse_run_properties(n:&Element)->TextStyle{
    let enabled=|key:&str|n.child(key).map(|v|!matches!(v.attr("val"),"0"|"false"|"none")).unwrap_or(false);
    TextStyle{bold:enabled("b")||n.attr("b")=="1",italic:enabled("i")||n.attr("i")=="1",underline:enabled("u")||(!n.attr("u").is_empty()&&n.attr("u")!="none"),strike:enabled("strike")||n.attr("strike")=="sngStrike",sup:n.child("vertAlign").map(|n|n.attr("val")=="superscript").unwrap_or(false)||n.attr("baseline").parse::<i32>().unwrap_or(0)>0,sub:n.child("vertAlign").map(|n|n.attr("val")=="subscript").unwrap_or(false)||n.attr("baseline").parse::<i32>().unwrap_or(0)<0}
}
fn latex_escape(s:&str)->String{
    let mut out=String::new();for c in s.chars(){if "\\{}_^#$%&".contains(c){out.push('\\')}out.push(c)}out
}
pub fn omml_to_latex(n:&Element,warnings:&mut Vec<String>)->String{
    fn go(n:&Element,w:&mut Vec<String>,depth:usize)->String{
        // P1-S01 instruction 5: bound recursion at 64 levels (billion-laughs guard);
        // beyond it flatten to text with a warning — never drop content, never panic.
        if depth>64{w.push("OOXML math nesting limit reached; rendered as plain text".into());return latex_escape(&n.text())}
        let child=|k:&str,w:&mut Vec<String>|n.child(k).map(|n|go(&n,w,depth+1)).unwrap_or_default();
        let attr=|k:&str,default:&str|n.desc(k).next().map(|n|n.attr("val").to_owned()).unwrap_or_else(||default.into());
        match package::local(&n.name){
            "t"=>latex_escape(&n.text()),
            "f"=>format!("\\frac{{{}}}{{{}}}",child("num",w),child("den",w)),
            "sSup"=>format!("{{{}}}^{{{}}}",child("e",w),child("sup",w)),
            "sSub"=>format!("{{{}}}_{{{}}}",child("e",w),child("sub",w)),
            "sSubSup"=>format!("{{{}}}_{{{}}}^{{{}}}",child("e",w),child("sub",w),child("sup",w)),
            "rad"=>{let deg=child("deg",w);let e=child("e",w);if deg.is_empty(){format!("\\sqrt{{{e}}}")}else{format!("\\sqrt[{deg}]{{{e}}}")}},
            "nary"=>{let ch=attr("chr","∑");let op=match ch.as_str(){"∫"=>"\\int","∏"=>"\\prod",_=>"\\sum"};format!("{op}_{{{}}}^{{{}}}{{{}}}",child("sub",w),child("sup",w),child("e",w))},
            "d"=>format!("\\left{}{}\\right{}",attr("begChr","("),child("e",w),attr("endChr",")")),
            "m"=>{let rows=n.children().filter(|n|package::local(&n.name)=="mr").map(|r|r.children().filter(|n|package::local(&n.name)=="e").map(|c|go(&c,w,depth+1)).collect::<Vec<_>>().join(" & ")).collect::<Vec<_>>().join(" \\\\ ");format!("\\begin{{matrix}}{rows}\\end{{matrix}}")},
            "acc"=>{let ch=attr("chr","^");let op=match ch.as_str(){"^"|"̂"=>"\\hat","→"|"⃗"=>"\\vec","."|"̇"=>"\\dot","~"|"̃"=>"\\tilde",s=>s};format!("{op}{{{}}}",child("e",w))},
            k if k.ends_with("Pr")=>String::new(),
            "oMath"|"oMathPara"|"r"|"e"|"num"|"den"|"sub"|"sup"|"deg"|"mr"=>n.children().map(|c|go(&c,w,depth+1)).collect::<Vec<_>>().join(""),
            key=>{w.push(format!("OOXML math element not fully converted: {key}; rendered as plain text"));latex_escape(&n.text())},
        }
    }
    go(n,warnings,0)
}
pub fn run_inlines(n:&Element,rels:&BTreeMap<String,Relationship>,warnings:&mut Vec<String>)->Vec<Inline>{
    let mut out=vec![];
    for c in n.content(){let e=match c{Content::Text(_)=>continue,Content::Element(e)=>e};match package::local(&e.name){
        "t"=>out.push(Inline::Text(e.text())),"tab"=>out.push(Inline::Text("\t".into())),"br"|"cr"=>out.push(Inline::Break),
        "footnoteReference"|"endnoteReference"=>out.push(Inline::FootnoteRef(e.attr("id").into())),
        "r"=>{let props=e.child("rPr");let style=props.as_ref().map(parse_run_properties).unwrap_or_default();let children=run_inlines(&e,rels,warnings);let mut xs=if style.is_plain(){children}else{vec![Inline::Styled{style,children}]};if let Some(url)=props.and_then(|p|p.child("hlinkClick")).and_then(|h|rels.get(h.attr("id"))).map(|r|r.target.clone()){xs=vec![Inline::Link{url,children:xs}]}out.extend(xs)},
        "hyperlink"=>{let url=rels.get(e.attr("id")).map(|r|r.target.clone()).unwrap_or_else(||format!("#{}",e.attr("anchor")));out.push(Inline::Link{url,children:run_inlines(&e,rels,warnings)})},
        "oMath"|"oMathPara"=>out.push(Inline::MathInline(omml_to_latex(&e,warnings))),
        "rPr"|"pPr"|"tcPr"=>{},_=>out.extend(run_inlines(&e,rels,warnings)),
    }}out
}
pub fn parse_table_cell(n:&Element,ns:char,rels:&BTreeMap<String,Relationship>,warnings:&mut Vec<String>)->TableCell{
    let span=n.child("tcPr").and_then(|p|p.child("gridSpan")).and_then(|n|n.attr("val").parse::<usize>().ok()).or_else(||n.attr("gridSpan").parse::<usize>().ok()).unwrap_or(1).clamp(1,1000);
    let rowspan=if ns=='a'{n.attr("rowSpan").parse::<usize>().unwrap_or(1).clamp(1,1000)}else{1};
    TableCell{inlines:run_inlines(n,rels,warnings),colspan:span,rowspan,align:Align::Default}
}
pub fn table(n:&Element,ns:char,rels:&BTreeMap<String,Relationship>,warnings:&mut Vec<String>)->Table{
    let mut rows:Vec<Vec<TableCell>>=vec![];let mut active:HashMap<usize,(usize,usize)>=HashMap::new();
    for tr in n.children().filter(|n|package::local(&n.name)=="tr"){
        let ri=rows.len();let mut row=vec![];let mut col=0;
        for tc in tr.children().filter(|n|package::local(&n.name)=="tc"){
            let vm=tc.child("tcPr").and_then(|n|n.child("vMerge"));
            let continuation=vm.as_ref().map(|n|n.attr("val")!="restart").unwrap_or(false)||tc.attr("vMerge")=="1";
            let cell=parse_table_cell(&tc,ns,rels,warnings);
            if continuation{
                if let Some(&(r,c))=active.get(&col){if ns=='w'{if let Some(old)=rows.get_mut(r).and_then(|row|row.get_mut(c)){old.rowspan+=1}}}
                else if ns=='w'{warnings.push("orphan vertical merge continuation".into());row.push(cell.clone())}
            }else if tc.attr("hMerge")!="1"{
                if vm.is_some()||cell.rowspan>1{active.insert(col,(ri,row.len()));}else{active.remove(&col);}
                row.push(cell.clone());
            }
            // DrawingML has explicit continuation cells for horizontal merges.
            col+=if ns=='a'{1}else{cell.colspan};
        }
        rows.push(row);
    }
    let header=if rows.is_empty(){vec![]}else if ns=='w'{
        // P1-S02: first row is header iff every cell's first run is bold.
        let bold_first=row_bold(&rows[0]);
        if bold_first{rows.remove(0)}else{vec![]}
    }else{rows.remove(0)};
    Table{header,rows,..Table::default()}
}
fn row_bold(row:&[TableCell])->bool{!row.is_empty()&&row.iter().all(|c|matches!(c.inlines.first(),Some(Inline::Styled{style,..})if style.bold))}
pub fn images(n:&Element,pkg:&Package,part:&str,rels:&BTreeMap<String,Relationship>,ctx:&mut ExtractCtx<'_>)->Vec<Block>{
    let mut out=vec![];
    for blip in n.desc("blip"){
        if let Some(r)=rels.get(blip.attr("embed")){
            if !r.external{if let Some(path)=pkg.resolve(part,&r.target){if let Some(bytes)=pkg.get(&path){let asset=ctx.assets.add(bytes.to_vec(),"embedded OOXML image",None);out.push(Block::Image{asset,alt:None,caption:None});continue}}}
        }
        ctx.warnings.push("embedded OOXML image missing or unsafe".into());
    }out
}
