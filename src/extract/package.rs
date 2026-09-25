//! Bounded ZIP access and pull-based XML views. No DOM or child-node tree is retained.
use crate::error::{ConvertError, Result};
use quick_xml::events::Event;
use std::{collections::BTreeMap, io::{Cursor, Read}, ops::Range, sync::Arc};

pub const MAX_ENTRY: usize = 64 * 1024 * 1024;
pub const MAX_TOTAL: usize = 200 * 1024 * 1024;
pub const MAX_DEPTH: usize = 64;
pub const MAX_ELEMENTS: usize = 500_000;

pub struct Package { pub entries: BTreeMap<String, Vec<u8>> }
impl Package {
    pub fn open(bytes: &[u8]) -> Result<Self> {
        let mut zip = zip::ZipArchive::new(Cursor::new(bytes))
            .map_err(|_| ConvertError::Corrupt("incomplete or damaged ZIP archive".into()))?;
        if zip.len() > 10_000 { return Err(ConvertError::Corrupt("ZIP entry count exceeds limit".into())); }
        let mut entries = BTreeMap::new();
        let mut folded = std::collections::HashSet::new();
        let mut total = 0usize;
        for i in 0..zip.len() {
            let member = zip.by_index(i).map_err(|_| ConvertError::Corrupt("unreadable ZIP entry".into()))?;
            if member.is_dir() { continue; }
            let name = normalize("", member.name()).ok_or_else(|| ConvertError::Corrupt("unsafe ZIP entry path".into()))?;
            if !folded.insert(name.to_ascii_lowercase()) { return Err(ConvertError::Corrupt("duplicate ZIP entry".into())); }
            if member.size() > MAX_ENTRY as u64 { return Err(ConvertError::Corrupt("ZIP entry exceeds decompressed-size limit".into())); }
            let mut data = Vec::new();
            member.take((MAX_ENTRY + 1) as u64).read_to_end(&mut data)
                .map_err(|_| ConvertError::Corrupt("unreadable ZIP entry data".into()))?;
            total = total.saturating_add(data.len());
            if data.len() > MAX_ENTRY || total > MAX_TOTAL { return Err(ConvertError::Corrupt("ZIP exceeds decompressed-size limit".into())); }
            entries.insert(name, data);
        }
        Ok(Self { entries })
    }
    pub fn get(&self, name: &str) -> Option<&[u8]> {
        self.entries.get(name).or_else(|| self.entries.iter().find(|(k, _)| k.eq_ignore_ascii_case(name)).map(|(_, v)| v)).map(Vec::as_slice)
    }
    pub fn xml(&self, name: &str) -> Result<Element> {
        parse(self.get(name).ok_or_else(|| ConvertError::Corrupt("required package part missing".into()))?, false).map(|v| v.0)
    }
    pub fn optional_xml(&self, name: &str, warnings: &mut Vec<String>) -> Option<Element> {
        self.get(name)?;
        match self.xml(name) {
            Ok(part) => Some(part),
            Err(_) => { warnings.push(format!("optional package part malformed: {}", name.rsplit('/').next().unwrap_or("part"))); None }
        }
    }
    pub fn rels(&self, part: &str) -> BTreeMap<String, Relationship> {
        let (dir, file) = part.rsplit_once('/').unwrap_or(("", part));
        let path = if dir.is_empty() { format!("_rels/{file}.rels") } else { format!("{dir}/_rels/{file}.rels") };
        let mut out = BTreeMap::new();
        if let Ok(root) = self.xml(&path) {
            for n in root.desc("Relationship") {
                out.insert(n.attr("Id").into(), Relationship {
                    target: n.attr("Target").into(), kind: n.attr("Type").into(), external: n.attr("TargetMode") == "External",
                });
            }
        }
        out
    }
    pub fn resolve(&self, part: &str, target: &str) -> Option<String> {
        normalize(part.rsplit_once('/').map(|(d, _)| d).unwrap_or(""), target)
    }
}
#[derive(Debug, Clone)]
pub struct Relationship { pub target: String, pub kind: String, pub external: bool }
pub fn normalize(base: &str, target: &str) -> Option<String> {
    if target.contains(['\\', ':', '\0']) || target.starts_with('/') { return None; }
    let mut parts: Vec<&str> = base.split('/').filter(|s| !s.is_empty()).collect();
    for p in target.split('#').next()?.split('/') {
        match p { "" | "." => {}, ".." => { parts.pop()?; }, p => parts.push(p) }
    }
    Some(parts.join("/"))
}
pub fn local(name: &str) -> &str { name.rsplit(':').next().unwrap_or(name) }

/// An immutable view into one shared XML byte buffer, not an object-model node.
/// Child access starts a fresh streaming cursor. There are no parent/child links,
/// retained event lists, recursive containers, or lazily cached subtrees.
#[derive(Debug, Clone)]
pub struct Element {
    pub name: String,
    pub attrs: BTreeMap<String, String>,
    source: Arc<[u8]>,
    inner: Range<usize>,
}
#[derive(Debug, Clone)]
pub enum Content { Text(String), Element(Element) }
impl Element {
    pub fn attr(&self, key: &str) -> &str {
        self.attrs.get(key).or_else(|| self.attrs.iter().find(|(k, _)| local(k) == key).map(|(_, v)| v)).map(String::as_str).unwrap_or("")
    }
    pub fn content(&self) -> ContentIter { ContentIter { source: self.source.clone(), cursor: self.inner.start, end: self.inner.end } }
    pub fn children(&self) -> impl Iterator<Item = Element> { self.content().filter_map(|c| match c { Content::Element(e) => Some(e), _ => None }) }
    pub fn child(&self, name: &str) -> Option<Element> { self.children().find(|e| local(&e.name).eq_ignore_ascii_case(name)) }
    /// Tag-filtered streaming traversal. Only the caller's selected views are retained.
    pub fn desc(&self, name: &str) -> Descendants { Descendants { stack: vec![self.content()], name: name.to_owned() } }
    pub fn text(&self) -> String {
        let mut reader = quick_xml::Reader::from_reader(&self.source[self.inner.clone()]);
        let mut out = String::new();
        loop { match reader.read_event() {
            Ok(Event::Text(e)) => out.push_str(&unescape(e.as_ref())),
            Ok(Event::CData(e)) => out.push_str(&String::from_utf8_lossy(e.as_ref())),
            Ok(Event::GeneralRef(e)) => out.push_str(&reference(e.as_ref())),
            Ok(Event::Eof) | Err(_) => break,
            _ => {},
        }}
        out
    }
}
pub struct Descendants { stack: Vec<ContentIter>, name: String }
impl Iterator for Descendants {
    type Item = Element;
    fn next(&mut self) -> Option<Element> {
        while let Some(iter) = self.stack.last_mut() {
            match iter.next() {
                Some(Content::Element(e)) => {
                    self.stack.push(e.content());
                    if local(&e.name).eq_ignore_ascii_case(&self.name) { return Some(e); }
                },
                Some(_) => {},
                None => { self.stack.pop(); },
            }
        }
        None
    }
}
pub struct ContentIter { source: Arc<[u8]>, cursor: usize, end: usize }
impl Iterator for ContentIter {
    type Item = Content;
    fn next(&mut self) -> Option<Content> {
        while self.cursor < self.end {
            let base = self.cursor;
            let mut reader = quick_xml::Reader::from_reader(&self.source[base..self.end]);
            let event = reader.read_event().ok()?;
            let after = base + reader.buffer_position() as usize;
            match event {
                Event::Text(e) => { let text = unescape(e.as_ref()); self.cursor = after; return Some(Content::Text(text)); },
                Event::CData(e) => { let text = String::from_utf8_lossy(e.as_ref()).into_owned(); self.cursor = after; return Some(Content::Text(text)); },
                Event::GeneralRef(e) => { let text = reference(e.as_ref()); self.cursor = after; return Some(Content::Text(text)); },
                Event::Empty(e) => {
                    let (name, attrs) = tag(&e); self.cursor = after;
                    return Some(Content::Element(Element { name, attrs, source: self.source.clone(), inner: after..after }));
                },
                Event::Start(e) => {
                    let (name, attrs) = tag(&e);
                    let start = after; let mut depth = 1usize;
                    loop {
                        let before = base + reader.buffer_position() as usize;
                        match reader.read_event().ok()? {
                            Event::Start(_) => depth += 1,
                            Event::End(_) => {
                                depth -= 1;
                                if depth == 0 {
                                    self.cursor = base + reader.buffer_position() as usize;
                                    return Some(Content::Element(Element { name, attrs, source: self.source.clone(), inner: start..before }));
                                }
                            },
                            Event::Eof => { self.cursor = self.end; return None; },
                            _ => {},
                        }
                    }
                },
                Event::Eof => { self.cursor = self.end; return None; },
                _ => { self.cursor = after; },
            }
        }
        None
    }
}
fn unescape(bytes: &[u8]) -> String {
    let s = String::from_utf8_lossy(bytes);
    quick_xml::escape::unescape(&s).map(|s| s.into_owned()).unwrap_or_else(|_| s.into_owned())
}
fn reference(bytes: &[u8]) -> String {
    let s = format!("&{};", String::from_utf8_lossy(bytes));
    quick_xml::escape::unescape(&s).map(|s| s.into_owned()).unwrap_or_else(|_| "�".into())
}
fn tag(e: &quick_xml::events::BytesStart<'_>) -> (String, BTreeMap<String, String>) {
    let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
    let mut attrs = BTreeMap::new();
    for a in e.attributes().with_checks(false).flatten() {
        attrs.insert(String::from_utf8_lossy(a.key.as_ref()).into_owned(), unescape(a.value.as_ref()));
    }
    (name, attrs)
}
fn void(name: &str) -> bool {
    matches!(local(name).to_ascii_lowercase().as_str(), "br"|"img"|"hr"|"meta"|"link"|"input"|"source"|"wbr"|"area"|"base"|"col"|"embed"|"param"|"track")
}
/// Validate with bounded streaming state; tolerant HTML is normalized by streaming
/// writes (void tags, mismatched ends, EOF closures). External entities are never read.
/// Retains at most source bytes, output bytes and a MAX_DEPTH stack of tag names.
pub fn parse(bytes: &[u8], tolerant: bool) -> Result<(Element, Vec<String>)> {
    if bytes.len() > MAX_ENTRY { return Err(ConvertError::Corrupt("XML part exceeds limit".into())); }
    let mut reader = quick_xml::Reader::from_reader(bytes);
    reader.config_mut().check_end_names = false;
    let mut writer = quick_xml::Writer::new(Vec::new());
    let mut stack: Vec<String> = vec![]; let mut count = 0usize; let mut warnings = vec![];
    loop {
        let event = match reader.read_event() {
            Ok(e) => e,
            Err(_) if tolerant => { warnings.push("HTML truncated at parse error: malformed markup".into()); break; },
            Err(_) => return Err(ConvertError::Xml("malformed XML".into())),
        };
        match event {
            Event::Start(e) => {
                count += 1;
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                if tolerant && void(&name) { writer.write_event(Event::Empty(e))?; }
                else {
                    if stack.len() >= MAX_DEPTH { return Err(ConvertError::Corrupt("XML complexity limit exceeded".into())); }
                    stack.push(name); writer.write_event(Event::Start(e))?;
                }
            },
            Event::Empty(e) => { count += 1; writer.write_event(Event::Empty(e))?; },
            Event::End(e) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                if tolerant {
                    if let Some(pos) = stack.iter().rposition(|n| n.eq_ignore_ascii_case(&name)) {
                        while stack.len() > pos {
                            if let Some(name) = stack.pop() { writer.write_event(Event::End(quick_xml::events::BytesEnd::new(name)))?; }
                        }
                    }
                } else {
                    if stack.pop().as_deref() != Some(name.as_str()) { return Err(ConvertError::Xml("unexpected closing tag".into())); }
                    writer.write_event(Event::End(e))?;
                }
            },
            Event::DocType(_) => warnings.push("document type declaration ignored; external entities never loaded".into()),
            Event::Decl(_) | Event::PI(_) => {},
            Event::GeneralRef(e) => {
                let decoded = reference(e.as_ref());
                writer.write_event(Event::Text(quick_xml::events::BytesText::new(&decoded)))?;
            },
            Event::Eof => break,
            event => { writer.write_event(event)?; },
        }
        if count > MAX_ELEMENTS || writer.get_ref().len() > MAX_ENTRY { return Err(ConvertError::Corrupt("XML complexity limit exceeded".into())); }
    }
    if !tolerant && !stack.is_empty() { return Err(ConvertError::Xml("unclosed XML element".into())); }
    while let Some(name) = stack.pop() { writer.write_event(Event::End(quick_xml::events::BytesEnd::new(name)))?; }
    let data = writer.into_inner();
    if data.len() > MAX_ENTRY { return Err(ConvertError::Corrupt("XML part exceeds limit".into())); }
    let end = data.len();
    Ok((Element { name: String::new(), attrs: BTreeMap::new(), source: Arc::from(data), inner: 0..end }, warnings))
}
pub fn metadata(pkg: &Package, meta: &mut crate::model::Meta) {
    if let Ok(n) = pkg.xml("docProps/core.xml") {
        for (key, dst) in [("title", &mut meta.title), ("creator", &mut meta.author), ("subject", &mut meta.subject), ("description", &mut meta.description), ("created", &mut meta.created), ("modified", &mut meta.modified), ("language", &mut meta.language)] {
            *dst = n.desc(key).next().map(|n| n.text());
        }
    }
}
pub fn read_capped(path: &std::path::Path, max: u64) -> Result<Vec<u8>> {
    let file = std::fs::File::open(path)?;
    let mut bytes = Vec::new();
    file.take(max.saturating_add(1)).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > max { return Err(ConvertError::Unsupported("file exceeds --max-size".into())); }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::read_capped;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    fn unique_path() -> std::path::PathBuf {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("docconvert-read-capped-{}-{n}", std::process::id()))
    }
    #[test]
    fn read_capped_rejects_over_max() {
        let path = unique_path();
        std::fs::write(&path, b"abc").unwrap();
        let err = read_capped(&path, 2).unwrap_err();
        assert!(matches!(&err, crate::error::ConvertError::Unsupported(m) if m == "file exceeds --max-size"));
        assert_eq!(err.to_string(), "unsupported format: file exceeds --max-size");
        let _ = std::fs::remove_file(&path);
    }
    #[test]
    fn read_capped_accepts_at_max() {
        let path = unique_path();
        std::fs::write(&path, b"ab").unwrap();
        let bytes = read_capped(&path, 2).unwrap();
        assert_eq!(bytes, b"ab");
        let _ = std::fs::remove_file(&path);
    }
}
