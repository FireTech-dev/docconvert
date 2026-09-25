use crate::model::*;
fn inline(xs:&[Inline],minimal:bool)->String{
    let mut out=String::new();
    for x in xs{out.push_str(&match x{
        Inline::Link{children,url}=>{let text=inline(children,minimal);if &text==url{text}else{format!("{text} ({url})")}},
        Inline::Styled{children,..}=>inline(children,minimal),
        Inline::Image{alt,..}=>if minimal{alt.clone().unwrap_or_default()}else{alt.as_ref().map(|s|format!("[Image: {s}]")).unwrap_or_else(||"[Image]".into())},
        _=>inline_text(std::slice::from_ref(x)),
    })}out
}
fn wrap(s:&str,width:usize)->Vec<String>{
    if s.is_empty(){return vec![String::new()]}
    let mut lines=vec![];
    for line in s.split('\n'){
        let mut part=String::new();let mut count=0;
        for ch in line.chars(){if count==width{lines.push(std::mem::take(&mut part));count=0}part.push(ch);count+=1}
        lines.push(part);
    }lines
}
fn table(t:&Table,minimal:bool)->String{
    let cells=super::expanded_rows(t);
    let rows:Vec<Vec<String>>=cells.iter().map(|r|r.iter().map(|c|inline(&c.inlines,minimal)).collect()).collect();
    let columns=rows.first().map(Vec::len).unwrap_or(0);
    let widths:Vec<usize>=(0..columns).map(|c|rows.iter().map(|r|r[c].lines().map(|s|s.chars().count()).max().unwrap_or(0)).max().unwrap_or(0).clamp(1,32)).collect();
    let numeric:Vec<bool>=(0..columns).map(|c|{let values:Vec<_>=rows.iter().skip(1).map(|r|r[c].trim()).filter(|s|!s.is_empty()).collect();!values.is_empty()&&values.iter().all(|s|s.parse::<f64>().is_ok())}).collect();
    let mut out=vec![];
    for(i,row)in rows.iter().enumerate(){
        let wrapped:Vec<_>=row.iter().enumerate().map(|(c,s)|wrap(s,widths[c])).collect();
        let height=wrapped.iter().map(Vec::len).max().unwrap_or(1);
        for line in 0..height{
            out.push((0..columns).map(|c|{let s=wrapped[c].get(line).map(String::as_str).unwrap_or("");let padding=" ".repeat(widths[c].saturating_sub(s.chars().count()));if numeric[c]&&i>0{format!("{padding}{s}")}else{format!("{s}{padding}")}}).collect::<Vec<_>>().join("  ").trim_end().to_owned());
        }
        if i==0{out.push(widths.iter().map(|w|"-".repeat(*w)).collect::<Vec<_>>().join("  "))}
    }
    if let Some(caption)=&t.caption{out.push(caption.clone())}out.join("\n")
}
fn blocks(bs:&[Block],minimal:bool)->String{
    let mut out=String::new();
    for b in bs{let text=match b{
        Block::Heading{level,inlines}=>{let text=inline(inlines,minimal).to_uppercase();format!("{text}\n{}",if *level==1{"="}else{"-"}.repeat(text.chars().count()))},
        Block::Paragraph(xs)=>inline(xs,minimal),
        Block::List{ordered,start,items}=>items.iter().enumerate().map(|(i,item)|{
            let marker=if *ordered{format!("{}.",start.unwrap_or(1)+i as u64)}else{"-".into()};
            let text=blocks(&item.blocks,minimal);let indent=" ".repeat(item.level.min(64)*2);let mut lines=text.trim_end().lines();let mut out=format!("{indent}{marker} {}",lines.next().unwrap_or("").trim_start());
            for line in lines{let leading=line.len()-line.trim_start().len();let nested=leading>=indent.len()+2&&(line.trim_start().starts_with("- ")||line.trim_start().chars().next().map(|c|c.is_ascii_digit()).unwrap_or(false));if nested{out.push_str(&format!("\n{line}"))}else{out.push_str(&format!("\n{indent}  {line}"))}}out
        }).collect::<Vec<_>>().join("\n"),
        Block::Table(t)=>table(t,minimal),
        Block::CodeBlock{text,..}|Block::MathBlock(text)=>text.lines().map(|s|format!("    {s}")).collect::<Vec<_>>().join("\n"),
        Block::Quote(bs)=>blocks(bs,minimal).trim_end().lines().map(|s|format!("    {s}")).collect::<Vec<_>>().join("\n"),
        Block::Rule=>"---".into(),
        Block::Image{alt,caption,..}=>{
            let mut s=if minimal{alt.clone().unwrap_or_default()}else{alt.as_ref().map(|s|format!("[Image: {s}]")).unwrap_or_else(||"[Image]".into())};
            if let Some(c)=caption{if !s.is_empty(){s.push('\n')}s.push_str(c)}s
        },
        Block::Placeholder{kind,label,asset}=>if minimal{label.clone().unwrap_or_else(||kind.label().into())}else{format!("[{}{} — {}]",kind.label(),label.as_ref().map(|s|format!(": {s}")).unwrap_or_default(),if asset.is_some(){"preserved as an external asset"}else{"no rendered asset available"})},
        Block::Boundary(b)=>match b{Boundary::Page(n)=>format!("---- Page {n} ----"),Boundary::Slide{n,title}=>format!("---- Slide {n}: {} ----",title.as_deref().unwrap_or("")),Boundary::Chapter(t)=>format!("{}\n{}","=".repeat(60),t.as_deref().unwrap_or("Chapter")),Boundary::Worksheet(t)=>format!("{}\n{t}","=".repeat(60))},
        Block::FormulaList(fs)=>format!("FORMULAS\n{}",fs.iter().map(|(r,f)|format!("{r} {}",if f.starts_with('='){f.clone()}else{format!("= {f}")})).collect::<Vec<_>>().join("\n")),
    };if !text.is_empty(){out.push_str(&text);out.push_str("\n\n")}}
    out
}
pub fn render_txt(doc:&Document)->String{render_txt_with_options(doc,false)}
pub fn render_txt_with_options(doc:&Document,minimal:bool)->String{
    let mut out=blocks(&doc.blocks,minimal);
    for(id,xs)in &doc.footnotes{out.push_str(&format!("[{id}] {}\n",inline(xs,minimal)))}
    while out.ends_with('\n'){out.pop();}out.push('\n');out
}
