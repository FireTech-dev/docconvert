use crate::model::*;
pub fn extract_txt(bytes:&[u8],mut meta:Meta)->Document{meta.source_format=Format::Txt;let s=String::from_utf8_lossy(bytes).replace("\r\n","\n").replace('\r',"\n");let blocks=s.split("\n\n").filter(|s|!s.trim().is_empty()).map(|s|if rule(s.trim()){Block::Rule}else{Block::Paragraph(vec![Inline::Text(s.trim().into())])}).collect();Document{meta,blocks,..Document::default()}}
fn rule(s:&str)->bool{let c=s.chars().filter(|c|!c.is_whitespace()).collect::<String>();c.len()>=3&&["-","*","_","="].iter().any(|m|c.chars().all(|x|m.starts_with(x)))}
pub type ImageLoader<'a> = dyn FnMut(&str, &str) -> Option<usize> + 'a;
pub fn extract_markdown(bytes:&[u8],meta:Meta)->Document{extract_markdown_with_loader(bytes,meta,&mut |_,_|None)}
pub fn extract_markdown_with_loader(bytes:&[u8],mut meta:Meta,loader:&mut ImageLoader<'_>)->Document{meta.source_format=Format::Markdown;let s=String::from_utf8_lossy(bytes).replace("\r\n","\n").replace('\r',"\n");let mut d=Document{meta,..Document::default()};d.blocks=parse_blocks(&s,&mut d.footnotes,&mut d.warnings,0,loader);d}
pub fn parse_inlines(s:&str,warnings:&mut Vec<String>)->Vec<Inline>{inline(s,warnings,0,&mut |_,_|None)}
fn parsed(s:&str,warnings:&mut Vec<String>,loader:&mut ImageLoader<'_>)->Vec<Inline>{inline(s,warnings,0,loader)}
fn inline(s:&str,warnings:&mut Vec<String>,depth:usize,loader:&mut ImageLoader<'_>)->Vec<Inline>{
 if depth>=64{warnings.push("inline nesting limit reached; preserved as text".into());return vec![Inline::Text(s.into())]}
 let mut out=vec![];let mut pos=0;let mut plain=String::new();
 while pos<s.len(){let rest=&s[pos..];let mut found:Option<(Inline,usize)>=None;
 if rest.starts_with('\\'){if let Some(c)=rest[1..].chars().next(){if c.is_ascii_punctuation(){plain.push(c);pos+=1+c.len_utf8();continue}if c=='\n'{if !plain.is_empty(){out.push(Inline::Text(std::mem::take(&mut plain)))}out.push(Inline::Break);pos+=2;continue}}}
 if rest.starts_with('\n')&&plain.ends_with("  "){plain.truncate(plain.trim_end_matches(' ').len());if !plain.is_empty(){out.push(Inline::Text(std::mem::take(&mut plain)))}out.push(Inline::Break);pos+=1;continue}
 if rest.starts_with('`'){let run=rest.bytes().take_while(|b|*b==b'`').count();let fence="`".repeat(run);let mut offset=run;while offset<rest.len(){if let Some(next)=rest[offset..].find(&fence){let end=offset+next;let after=end+run;if !rest[after..].starts_with('`')&&!rest[..end].ends_with('`'){let raw=rest[run..end].replace('\n'," ");let value=if raw.starts_with(' ')&&raw.ends_with(' ')&&!raw.chars().all(|c|c==' '){raw[1..raw.len()-1].into()}else{raw};found=Some((Inline::Code(value),after));break}offset=after;}else{break}}}
 if rest.starts_with("[^" ){if let Some(end)=rest.find(']'){found=Some((Inline::FootnoteRef(rest[2..end].into()),end+1))}}
 if found.is_none()&&(rest.starts_with('[')||rest.starts_with("![")){let image=rest.starts_with('!');let start=if image{2}else{1};if let Some(mid)=rest[start..].find("](").map(|i|i+start){if let Some(end)=rest[mid+2..].find(')').map(|i|i+mid+2){let label=&rest[start..mid];let url=rest[mid+2..end].trim().to_owned();let value=if image{if let Some(asset)=loader(&url,label){Inline::Image{asset,alt:Some(label.into())}}else{warnings.push("external/missing image not embedded".into());Inline::Link{children:inline(label,warnings,depth+1,loader),url}}}else{Inline::Link{children:inline(label,warnings,depth+1,loader),url}};found=Some((value,end+1));}}}
 if found.is_none(){for mark in ["***","**","~~","*","_","$"]{if rest.starts_with(mark){if let Some(end)=rest[mark.len()..].find(mark).map(|i|i+mark.len()){let inner=&rest[mark.len()..end];let value=match mark{"`"=>Inline::Code(inner.into()),"$"=>Inline::MathInline(inner.into()),_=>Inline::Styled{style:TextStyle{bold:mark.contains("**"),italic:mark=="***"||mark=="*"||mark=="_",strike:mark=="~~",..TextStyle::default()},children:inline(inner,warnings,depth+1,loader)}};found=Some((value,end+mark.len()));break}}}}
 if let Some((v,n))=found{if !plain.is_empty(){out.push(Inline::Text(std::mem::take(&mut plain)))}out.push(v);pos+=n}else if let Some(c)=rest.chars().next(){plain.push(c);pos+=c.len_utf8()}else{break}
 }if !plain.is_empty(){out.push(Inline::Text(plain))}out
}
fn list_marker(line:&str)->Option<(bool,u64,usize,&str)>{let t=line.trim_start();let level=(line.len()-t.len())/2;if t.starts_with("- ")||t.starts_with("* ")||t.starts_with("+ "){return Some((false,1,level,&t[2..]))}let n=t.chars().take_while(char::is_ascii_digit).count();if n>0&&(t[n..].starts_with(". ")||t[n..].starts_with(") ")){return Some((true,t[..n].parse().ok()?,level,&t[n+2..]))}None}
fn split_cells(s:&str)->Vec<String>{let t=s.trim().trim_matches('|');let mut out=vec![];let mut cell=String::new();let mut escape=false;for c in t.chars(){if escape{if c!='|'{cell.push('\\')}cell.push(c);escape=false}else if c=='\\'{escape=true}else if c=='|'{out.push(cell.trim().into());cell.clear()}else{cell.push(c)}}if escape{cell.push('\\')}out.push(cell.trim().into());out}
fn parse_blocks(s:&str,footnotes:&mut Vec<(String,Vec<Inline>)>,warnings:&mut Vec<String>,depth:usize,loader:&mut ImageLoader<'_>)->Vec<Block>{
 if depth>=64{warnings.push("block nesting limit reached".into());return vec![Block::Paragraph(vec![Inline::Text(s.into())])]}
 let lines:Vec<&str>=s.lines().collect();let mut out=vec![];let mut i=0;
 while i<lines.len(){let l=lines[i];let t=l.trim();if t.is_empty(){i+=1;continue}
 if let Some(end)=t.find("]:").filter(|_|t.starts_with("[^")){footnotes.push((t[2..end].into(),parsed(t[end+2..].trim(),warnings,loader)));i+=1;continue}
 if t.starts_with("```")||t.starts_with("~~~"){let c=t.chars().next().unwrap_or('`');let n=t.chars().take_while(|x|*x==c).count();let close=(i+1..lines.len()).find(|j|{let x=lines[*j].trim();x.chars().take_while(|v|*v==c).count()>=n&&x.chars().all(|v|v==c)});if let Some(j)=close{out.push(Block::CodeBlock{language:if t[n..].trim().is_empty(){None}else{Some(t[n..].trim().into())},text:lines[i+1..j].join("\n")});i=j+1;continue}else{warnings.push("unclosed fence; preserved as paragraph".into());out.push(Block::Paragraph(vec![Inline::Text(lines[i..].join("\n"))]));break}}
 if t.starts_with("$$") {let tail=&t[2..];if let Some(end)=tail.find("$$"){out.push(Block::MathBlock(tail[..end].into()));i+=1;continue}if let Some(end)=(i+1..lines.len()).find(|j|lines[*j].trim()=="$$"){out.push(Block::MathBlock(lines[i+1..end].join("\n")));i=end+1;continue}}
 let h=t.chars().take_while(|c|*c=='#').count();if (1..=6).contains(&h)&&t[h..].starts_with(' '){out.push(Block::Heading{level:h,inlines:parsed(t[h..].trim(),warnings,loader)});i+=1;continue}
 if rule(t){out.push(Block::Rule);i+=1;continue}
 if t.starts_with('>'){let start=i;while i<lines.len()&&lines[i].trim_start().starts_with('>'){i+=1}let inner=lines[start..i].iter().map(|l|l.trim_start().strip_prefix('>').unwrap_or(l).strip_prefix(' ').unwrap_or(l.trim_start().trim_start_matches('>'))).collect::<Vec<_>>().join("\n");out.push(Block::Quote(parse_blocks(&inner,footnotes,warnings,depth+1,loader)));continue}
 if let Some((_,_,base,_))=list_marker(l){let start=i;while i<lines.len()&&!lines[i].trim().is_empty(){if list_marker(lines[i]).is_some()||lines[i].starts_with("  "){i+=1}else{break}}let mut cursor=0;out.extend(parse_lists(&lines[start..i],&mut cursor,base,warnings,loader));continue}
 if i+1<lines.len()&&l.contains('|'){let sep=split_cells(lines[i+1]);if !sep.is_empty()&&sep.iter().all(|s|s.trim_matches(':').len()>=3&&s.trim_matches(':').chars().all(|c|c=='-')){
 let aligns:Vec<Align>=sep.iter().map(|s|match(s.starts_with(':'),s.ends_with(':')){(true,true)=>Align::Center,(true,false)=>Align::Left,(false,true)=>Align::Right,_=>Align::Default}).collect();let mut row=|line:&str|split_cells(line).into_iter().enumerate().map(|(j,s)|TableCell{inlines:parsed(&s,warnings,loader),align:aligns.get(j).copied().unwrap_or_default(),..TableCell::default()}).collect::<Vec<_>>();let header=row(l);i+=2;let mut rows=vec![];while i<lines.len()&&lines[i].contains('|')&&!lines[i].trim().is_empty(){rows.push(row(lines[i]));i+=1}out.push(Block::Table(Table{header,rows,..Table::default()}));continue}}
 if l.starts_with("    "){let start=i;while i<lines.len()&&(lines[i].starts_with("    ")||lines[i].is_empty()){i+=1}out.push(Block::CodeBlock{language:None,text:lines[start..i].iter().map(|l|l.strip_prefix("    ").unwrap_or(l)).collect::<Vec<_>>().join("\n")});continue}
 let mut para=vec![l.trim_start()];i+=1;while i<lines.len()&&!lines[i].trim().is_empty()&&!lines[i].trim().starts_with(['#','>','`'])&&list_marker(lines[i]).is_none()&&!rule(lines[i].trim()){para.push(lines[i].trim_start());i+=1}out.push(Block::Paragraph(parsed(&para.join("\n"),warnings,loader)));
 }out
}

fn parse_lists(lines:&[&str],cursor:&mut usize,level:usize,warnings:&mut Vec<String>,loader:&mut ImageLoader<'_>)->Vec<Block>{
    if level>=64{warnings.push("list nesting limit reached; preserved as text".into());let text=lines[*cursor..].join("\n");*cursor=lines.len();return vec![Block::Paragraph(vec![Inline::Text(text)])]}
    let mut output=vec![];
    while *cursor<lines.len(){
        let Some((ordered,start,found,_))=list_marker(lines[*cursor])else{break};
        if found<level{break}
        let mut items:Vec<ListItem>=vec![];
        while *cursor<lines.len(){
            let line=lines[*cursor];
            if let Some((kind,_,depth,text))=list_marker(line){
                if depth<level||depth==level&&kind!=ordered{break}
                if depth>level{
                    let nested=parse_lists(lines,cursor,depth,warnings,loader);
                    if let Some(item)=items.last_mut(){item.blocks.extend(nested)}else{items.push(ListItem{level,blocks:nested})}
                    continue;
                }
                items.push(ListItem{level,blocks:vec![Block::Paragraph(parsed(text,warnings,loader))]});*cursor+=1;
            }else{
                if let Some(item)=items.last_mut(){let xs=parsed(line.trim_start(),warnings,loader);if let Some(Block::Paragraph(previous))=item.blocks.last_mut(){previous.push(Inline::Text(" ".into()));previous.extend(xs)}else{item.blocks.push(Block::Paragraph(xs))}}
                *cursor+=1;
            }
        }
        output.push(Block::List{ordered,start:ordered.then_some(start),items});
    }
    output
}
