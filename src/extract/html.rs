use super::{package::{self, Content, Element}, ExtractCtx};
use crate::model::*;

pub fn extract_html(bytes: &[u8], meta: Meta, loader: &mut dyn FnMut(&str) -> Option<Vec<u8>>, ctx: &mut ExtractCtx<'_>) -> Document {
    extract(bytes, meta, loader, ctx, false)
}
pub(super) fn extract_epub_html(bytes: &[u8], meta: Meta, loader: &mut dyn FnMut(&str) -> Option<Vec<u8>>, ctx: &mut ExtractCtx<'_>) -> Document {
    extract(bytes, meta, loader, ctx, true)
}
fn entities(raw: &str) -> String {
    // One forward pass: replacement text cannot itself become a second entity.
    let table = [("nbsp","\u{a0}"),("mdash","—"),("ndash","–"),("lsquo","‘"),("rsquo","’"),("ldquo","“"),("rdquo","”"),("hellip","…"),("trade","™"),("reg","®"),("copy","©"),("deg","°"),("plusmn","±"),("frac12","½"),("bull","•"),("middot","·"),("euro","€"),("pound","£"),("yen","¥"),("sect","§"),("para","¶"),("dagger","†"),("Dagger","‡"),("laquo","«"),("raquo","»"),("times","×"),("divide","÷"),("infin","∞"),("minus","−"),("oelig","œ"),("scaron","š"),("Yuml","Ÿ"),("fnof","ƒ"),("circ","ˆ"),("tilde","˜"),("ensp"," "),("emsp"," "),("thinsp"," "),("zwnj","‌"),("zwj","‍"),("lrm","‎"),("rlm","‏"),("sbquo","‚"),("bdquo","„"),("permil","‰"),("lsaquo","‹"),("rsaquo","›")];
    let mut out = String::new(); let mut i = 0;
    while i < raw.len() {
        let tail = &raw[i..];
        if tail.starts_with('&') {
            if let Some(end) = tail.find(';').filter(|n| *n <= 32) {
                let name = &tail[1..end];
                if let Some((_, value)) = table.iter().find(|(key, _)| *key == name) { out.push_str(value); }
                else if matches!(name,"amp"|"lt"|"gt"|"quot"|"apos") || name.starts_with('#') { out.push_str(&tail[..=end]); }
                else { out.push('�'); }
                i += end + 1; continue;
            }
        }
        if let Some(c) = tail.chars().next() { if !c.is_control() || matches!(c,'\n'|'\t') { out.push(c); } i += c.len_utf8(); } else { break; }
    }
    out
}
fn extract(bytes: &[u8], meta: Meta, loader: &mut dyn FnMut(&str) -> Option<Vec<u8>>, ctx: &mut ExtractCtx<'_>, epub: bool) -> Document {
    let mut doc = Document { meta, ..Document::default() };
    let input = entities(&String::from_utf8_lossy(bytes));
    match package::parse(input.as_bytes(), true) {
        Ok((root, warnings)) => {
            doc.warnings.extend(warnings);
            if doc.meta.title.is_none() { doc.meta.title = root.desc("title").next().map(|n| n.text()); }
            let mut walker = Walker { loader, ctx, navigation: 0, epub, footnotes: vec![] };
            doc.blocks = walker.blocks(&root, 0);
            doc.footnotes = walker.footnotes;
            if walker.navigation > 0 { doc.warnings.push(format!("skipped {} navigation/interface sections", walker.navigation)); }
        },
        Err(error) => doc.warnings.push(error.to_string()),
    }
    doc
}
struct Walker<'a,'b,'c> {
    loader: &'a mut dyn FnMut(&str) -> Option<Vec<u8>>,
    ctx: &'b mut ExtractCtx<'c>, navigation: usize, epub: bool,
    footnotes: Vec<(String, Vec<Inline>)>,
}
fn whitespace(s: &str) -> String {
    let mut out = String::new(); let mut space = false;
    for c in s.chars() { if c.is_whitespace() { if !space { out.push(' '); } space = true; } else { out.push(c); space = false; } }
    out
}
fn trim_inlines(xs: &mut Vec<Inline>) {
    while matches!(xs.first(), Some(Inline::Text(s)) if s.trim().is_empty()) { xs.remove(0); }
    while matches!(xs.last(), Some(Inline::Text(s)) if s.trim().is_empty()) { xs.pop(); }
    if let Some(Inline::Text(s)) = xs.first_mut() { *s = s.trim_start().into(); }
    if let Some(Inline::Text(s)) = xs.last_mut() { *s = s.trim_end().into(); }
}
fn flush(xs: &mut Vec<Inline>, blocks: &mut Vec<Block>) {
    trim_inlines(xs);
    if !xs.is_empty() { blocks.push(Block::Paragraph(std::mem::take(xs))); }
}
impl Walker<'_, '_, '_> {
    fn skip(&mut self, e: &Element) -> bool {
        let tag = package::local(&e.name).to_ascii_lowercase();
        if self.epub && e.attr("type").split_whitespace().any(|s| s == "footnote") {
            let mut xs = self.inline_content(e);
            trim_inlines(&mut xs);
            let id = if e.attr("id").is_empty() { format!("note-{}", self.footnotes.len()+1) } else { e.attr("id").into() };
            self.footnotes.push((id, xs)); return true;
        }
        if matches!(tag.as_str(),"nav"|"header"|"footer"|"aside") { self.navigation += 1; return true; }
        matches!(tag.as_str(),"script"|"style"|"noscript"|"template"|"iframe"|"svg"|"canvas"|"form"|"button"|"select"|"textarea"|"input"|"dialog")
    }
    fn inline_content(&mut self, e: &Element) -> Vec<Inline> {
        let mut out = vec![];
        for c in e.content() { match c { Content::Text(s) => out.push(Inline::Text(whitespace(&s))), Content::Element(e) => out.extend(self.inline_element(&e)) } }
        out
    }
    fn inline_element(&mut self, e: &Element) -> Vec<Inline> {
        if self.skip(e) { return vec![]; }
        let tag = package::local(&e.name).to_ascii_lowercase();
        match tag.as_str() {
            "br" => vec![Inline::Break], "code" => vec![Inline::Code(e.text())],
            "img" => { let alt = (!e.attr("alt").is_empty()).then(|| e.attr("alt").to_owned());
                if let Some(bytes) = (self.loader)(e.attr("src")) {
                    let asset = self.ctx.assets.add(bytes, "HTML image", alt.clone()); vec![Inline::Image { asset, alt }]
                } else { self.ctx.warnings.push(format!("image missing or unreadable: {}",e.attr("src"))); vec![Inline::Link { children: vec![Inline::Text(alt.unwrap_or_else(|| "Image".into()))], url: e.attr("src").into() }] }
            },
            "a" => vec![Inline::Link { url: e.attr("href").into(), children: self.inline_content(e) }],
            "strong"|"b"|"em"|"i"|"u"|"s"|"strike"|"del"|"sup"|"sub" => vec![Inline::Styled {
                style: TextStyle { bold: matches!(tag.as_str(),"strong"|"b"), italic: matches!(tag.as_str(),"em"|"i"), underline: tag=="u", strike: matches!(tag.as_str(),"s"|"strike"|"del"), sup: tag=="sup", sub: tag=="sub" },
                children: self.inline_content(e),
            }],
            _ => self.inline_content(e),
        }
    }
    fn blocks(&mut self, root: &Element, level: usize) -> Vec<Block> {
        let mut out = vec![]; let mut pending = vec![];
        for content in root.content() {
            let e = match content { Content::Text(s) => { pending.push(Inline::Text(whitespace(&s))); continue; }, Content::Element(e) => e };
            if self.skip(&e) { continue; }
            let tag = package::local(&e.name).to_ascii_lowercase();
            match tag.as_str() {
                "head"|"title"|"meta"|"link" => {},
                "h1"|"h2"|"h3"|"h4"|"h5"|"h6" => { flush(&mut pending,&mut out); let mut xs=self.inline_content(&e); trim_inlines(&mut xs); out.push(Block::Heading { level:tag[1..].parse().unwrap_or(1), inlines:xs }); },
                "p" => { flush(&mut pending,&mut out); let mut xs=self.inline_content(&e); flush(&mut xs,&mut out); },
                "ul"|"ol" => {
                    flush(&mut pending,&mut out); let mut items=vec![];
                    for li in e.children().filter(|n|package::local(&n.name).eq_ignore_ascii_case("li")) { items.push(ListItem { level, blocks:self.blocks(&li,level+1) }); }
                    out.push(Block::List { ordered:tag=="ol", start:if tag=="ol" {Some(e.attr("start").parse().unwrap_or(1))}else{None}, items });
                },
                "blockquote" => { flush(&mut pending,&mut out); out.push(Block::Quote(self.blocks(&e,level))); },
                "pre" => { flush(&mut pending,&mut out); let language=e.child("code").and_then(|c|c.attr("class").split_whitespace().find_map(|s|s.strip_prefix("language-").map(str::to_owned))); let text=e.text(); let text=text.strip_prefix('\n').unwrap_or(&text); let text=text.strip_suffix('\n').unwrap_or(text).to_owned(); out.push(Block::CodeBlock { language,text }); },
                "hr" => { flush(&mut pending,&mut out); out.push(Block::Rule); },
                "img" => { flush(&mut pending,&mut out); out.extend(self.image(&e,None)); },
                "figure" => { flush(&mut pending,&mut out); let caption=e.desc("figcaption").next().map(|n|whitespace(&n.text()).trim().to_owned()); for image in e.desc("img") { out.extend(self.image(&image,caption.clone())); } },
                "table" => { flush(&mut pending,&mut out); let mut rows=vec![]; for tr in e.desc("tr") { let mut row=vec![]; for c in tr.children().filter(|c|matches!(package::local(&c.name).to_ascii_lowercase().as_str(),"td"|"th")) { let mut xs=self.inline_content(&c); trim_inlines(&mut xs); row.push(TableCell { inlines:xs, colspan:c.attr("colspan").parse::<usize>().unwrap_or(1).clamp(1,1000), rowspan:c.attr("rowspan").parse::<usize>().unwrap_or(1).clamp(1,1000), align:Align::Default }); } rows.push(row); } let header=if rows.is_empty(){vec![]}else{rows.remove(0)}; out.push(Block::Table(Table { header,rows,caption:e.child("caption").map(|n|n.text()),approximate:false })); },
                "html"|"body"|"article"|"main"|"section"|"div"|"li" => { flush(&mut pending,&mut out); out.extend(self.blocks(&e,level)); },
                _ => pending.extend(self.inline_element(&e)),
            }
        }
        flush(&mut pending,&mut out); out
    }
    fn image(&mut self,e:&Element,caption:Option<String>)->Vec<Block> {
        self.inline_element(e).into_iter().map(|i| match i { Inline::Image {asset,alt}=>Block::Image {asset,alt,caption:caption.clone()},i=>Block::Paragraph(vec![i]) }).collect()
    }
}
