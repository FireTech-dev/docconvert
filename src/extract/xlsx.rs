use super::{package::{self,Package},ExtractCtx};
use crate::{error::{ConvertError,Result},model::*};
use calamine::{Data,Reader};
use std::{collections::HashMap,io::Cursor};
pub fn extract_xlsx(bytes:&[u8],ctx:&mut ExtractCtx<'_>)->Result<Document>{read_workbook(bytes,ctx)}
pub(super) fn read_workbook(bytes:&[u8],ctx:&mut ExtractCtx<'_>)->Result<Document>{
 let pkg=if bytes.starts_with(b"PK"){Some(Package::open(bytes)?)}else{None};
 // Preflight actual cell coordinates (not the declared dimension) before calamine allocates ranges.
 if let Some(p)=&pkg{for(name,b)in &p.entries{if name.starts_with("xl/worksheets/")&&name.ends_with(".xml"){let(root,_)=package::parse(b,false)?;let mut min=(u32::MAX,u32::MAX);let mut max=(0,0);for c in root.desc("c"){if let Some((r,col))=super::parse_ref(c.attr("r")){min.0=min.0.min(r);min.1=min.1.min(col);max.0=max.0.max(r);max.1=max.1.max(col)}}if min.0!=u32::MAX&&(max.0 as u64-min.0 as u64+1).saturating_mul(max.1 as u64-min.1 as u64+1)>2_000_000{return Err(ConvertError::Unsupported("worksheet used range exceeds cell limit".into()))}}}}
 if let Some(p)=&pkg{if p.get("mimetype").map(|b|b==b"application/vnd.oasis.opendocument.spreadsheet").unwrap_or(false){let tree=p.xml("content.xml")?;let mut expanded=0u64;for row in tree.desc("table-row"){let nr=row.attr("number-rows-repeated").parse::<u64>().unwrap_or(1).max(1);let mut cols=0u64;for cell in row.children(){if matches!(package::local(&cell.name),"table-cell"|"covered-table-cell"){cols=cols.saturating_add(cell.attr("number-columns-repeated").parse::<u64>().unwrap_or(1).max(1));}}expanded=expanded.saturating_add(nr.saturating_mul(cols));if expanded>2_000_000{return Err(ConvertError::Unsupported("worksheet repeated cells exceed cell limit".into()))}}}}
 let mut reader=calamine::open_workbook_auto_from_rs(Cursor::new(bytes.to_vec())).map_err(|_|ConvertError::Corrupt("workbook could not be opened".into()))?;
 let names=reader.sheet_names();let mut d=Document::default();d.meta.worksheet_count=Some(names.len());let mut sheets=vec![];
 for name in names{let range=reader.worksheet_range(&name).map_err(|_|ConvertError::Corrupt("worksheet could not be read".into()))?;let start=range.start().unwrap_or((0,0));let mut sheet=Sheet{name:name.clone(),..Sheet::default()};
 if range.get_size().0.saturating_mul(range.get_size().1)>2_000_000{return Err(ConvertError::Unsupported("worksheet used range exceeds cell limit".into()))}
 for(r,c,value)in range.used_cells(){let row=start.0+r as u32;let col=start.1+c as u32;let (display,kind)=match value{Data::Empty=>continue,Data::String(s)=>(s.clone(),CellKind::Text),Data::Int(n)=>(n.to_string(),CellKind::Number),Data::Float(n)=>(n.to_string(),CellKind::Number),Data::Bool(b)=>((if *b{"TRUE"}else{"FALSE"}).into(),CellKind::Bool),Data::Error(e)=>(format!("{e:?}"),CellKind::Error),Data::DateTime(dt)=>{let(y,m,day,h,min,s,ms)=dt.to_ymd_hms_milli();let text=if h==0&&min==0&&s==0&&ms==0{format!("{y:04}-{m:02}-{day:02}")}else if ms==0{format!("{y:04}-{m:02}-{day:02}T{h:02}:{min:02}:{s:02}")}else{format!("{y:04}-{m:02}-{day:02}T{h:02}:{min:02}:{s:02}.{ms:03}")};(text,CellKind::Date)},Data::DateTimeIso(s)=>(s.clone(),CellKind::Date),Data::DurationIso(s)=>(s.clone(),CellKind::Text)};
 sheet.max_row=sheet.max_row.max(row);sheet.max_col=sheet.max_col.max(col);sheet.cells.insert((row,col),Cell{reference:super::cell_ref(row,col),display,kind,..Cell::default()});}
 if ctx.options.preserve_formulas{match reader.worksheet_formula(&name){Ok(formulas)=>{let origin=formulas.start().unwrap_or((0,0));for(r,c,f)in formulas.used_cells(){if f.is_empty(){continue}let rc=(origin.0+r as u32,origin.1+c as u32);let cell=sheet.cells.entry(rc).or_insert_with(||Cell{reference:super::cell_ref(rc.0,rc.1),..Cell::default()});cell.formula=Some(f.clone());sheet.max_row=sheet.max_row.max(rc.0);sheet.max_col=sheet.max_col.max(rc.1)}},Err(_)=>ctx.warnings.push("formula text unavailable; cached values preserved".into())}}
 sheets.push(sheet);
 }
 if let Some(pkg)=&pkg{if pkg.get("xl/workbook.xml").is_some(){xlsx_fidelity(pkg,&mut sheets,ctx)?;package::metadata(pkg,&mut d.meta)}else if pkg.get("content.xml").is_some(){ods_fidelity(pkg,&mut sheets,ctx)?}}
 else{ctx.warnings.push("legacy XLS: formula text, merges, comments, hyperlinks, hidden-sheet detection and chart inventory may be unavailable; cached values preserved".into());}
 d.workbook=Some(Workbook{title:d.meta.title.clone(),sheets});super::lower_workbook(&mut d,ctx.options);Ok(d)
}
/// P2-S01: Excel 1900-system serial (with the 1900 leap-year bug) → ISO 8601.
/// `None` for out-of-range values; the caller keeps the raw display then.
pub(super) fn serial_to_iso(serial:f64)->Option<String>{
 if !serial.is_finite()||serial<0.0||serial>2958465.0{return None}
 let days=serial.floor() as i64;
 // 25569 is the serial of 1970-01-01 for serials past the phantom 1900-02-29;
 // serials 1..61 predate it and need one day less removed (serial 60, Excel's
 // nonexistent 1900-02-29, renders as 1900-03-01 — the closest real date).
 let mut unix_days=if days<61{days-25568}else{days-25569};
 let mut secs=((serial-serial.floor())*86400.0).round() as i64;
 if secs>=86400{unix_days+=1;secs=0}
 let (y,m,d)=civil_from_days(unix_days)?;
 if secs<=0{Some(format!("{y:04}-{m:02}-{d:02}"))}
 else{Some(format!("{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}",secs/3600,(secs%3600)/60,secs%60))}
}
fn civil_from_days(z:i64)->Option<(i32,u32,u32)>{
 // Howard Hinnant's civil_from_days; valid across the whole Excel range.
 let z=z.checked_add(719468)?;let era=z.div_euclid(146097);let doe=z.rem_euclid(146097);
 let yoe=(doe-doe/1460+doe/36524-doe/146796)/365;let y=yoe+era*400;
 let doy=doe-(365*yoe+yoe/4-yoe/100);let mp=(5*doy+2)/153;
 let d=(doy-(153*mp+2)/5+1) as u32;let m=if mp<10{mp+3}else{mp-9} as u32;
 Some(((if m<=2{y+1}else{y}) as i32,m,d))
}
pub(super) fn is_date_format(fmt:&str)->bool{
    // P2-S02: number-format → CellKind::Date. Strip Excel [...] conditions and
    // "..." literals, then look for date/time tokens. `%` never counts as date.
    if fmt.contains('%'){return false}
    let lower=fmt.to_lowercase();let mut clean=String::new();let mut bracket=false;let mut quote=false;
    for c in lower.chars(){match c{'['=>bracket=true,']'=>bracket=false,'"'=>quote=!quote,_=>if !bracket&&!quote{clean.push(c)}}}
    ["yyyy","yy","mm","dd","m/d","d/m","d.","h:mm","mm:ss","yyyy-mm","hh:"].iter().any(|t|clean.contains(t))
}
fn xlsx_fidelity(pkg:&Package,sheets:&mut [Sheet],ctx:&mut ExtractCtx<'_>)->Result<()>{let root=pkg.xml("xl/workbook.xml")?;let rels=pkg.rels("xl/workbook.xml");let mut formats=HashMap::new();let mut styles=vec![];
 if let Ok(root)=pkg.xml("xl/styles.xml"){for n in root.desc("numFmt"){formats.insert(n.attr("numFmtId").to_owned(),n.attr("formatCode").to_owned());}if let Some(xfs)=root.desc("cellXfs").next(){for xf in xfs.children(){let id=xf.attr("numFmtId");let code=formats.get(id).cloned().unwrap_or_else(||match id{"9"=>"0%".into(),"10"=>"0.00%".into(),"14"=>"m/d/yyyy".into(),"22"=>"m/d/yyyy h:mm".into(),_=>String::new()});styles.push(code)}}}
 for entry in root.desc("sheet"){let Some(sheet)=sheets.iter_mut().find(|s|s.name==entry.attr("name"))else{continue};sheet.hidden=matches!(entry.attr("state"),"hidden"|"veryHidden");let Some(rel)=rels.get(entry.attr("id"))else{continue};let Some(part)=pkg.resolve("xl/workbook.xml",&rel.target)else{continue};let tree=pkg.xml(&part)?;let sheet_rels=pkg.rels(&part);
  for n in tree.desc("mergeCell"){let r=n.attr("ref");if !r.is_empty()&&!sheet.merged.iter().any(|m|m==r){sheet.merged.push(r.into())}}
 for n in tree.desc("hyperlink"){let url=if !n.attr("location").is_empty(){format!("#{}",n.attr("location"))}else{sheet_rels.get(n.attr("id")).map(|r|r.target.clone()).unwrap_or_default()};if url.is_empty(){continue}if let Some(rc)=super::parse_ref(n.attr("ref").split(':').next().unwrap_or("")){let reference=super::cell_ref(rc.0,rc.1);sheet.cells.entry(rc).or_insert_with(||Cell{reference,..Cell::default()}).hyperlink=Some(url)}}
  for n in tree.desc("c"){if let Some(cell)=super::parse_ref(n.attr("r")).and_then(|rc|sheet.cells.get_mut(&rc)){if let Some(fmt)=n.attr("s").parse::<usize>().ok().and_then(|i|styles.get(i)).filter(|s|!s.is_empty()){cell.number_format=Some(fmt.clone());if fmt.contains('%'){if let Ok(v)=cell.display.parse::<f64>(){let p=v*100.0;cell.display=if p.is_finite()&&p.fract()==0.0&&p.abs()<9e15{format!("{}%",p as i64)}else{format!("{p}%")}}}else if matches!(cell.kind,CellKind::Number)&&is_date_format(fmt){cell.kind=CellKind::Date;if let Ok(v)=cell.display.parse::<f64>(){if let Some(iso)=serial_to_iso(v){cell.display=iso}}}}if ctx.options.preserve_formulas{if let Some(f)=n.child("f"){let text=f.text();if !text.is_empty(){cell.formula=Some(format!("={text}"))}}}}}
 for rel in sheet_rels.values(){if rel.external{continue}let Some(path)=pkg.resolve(&part,&rel.target)else{continue};if rel.kind.ends_with("/comments")&&ctx.options.include_comments{if let Ok(root)=pkg.xml(&path){for c in root.desc("comment"){sheet.comments.push((c.attr("ref").into(),c.desc("t").map(|n|n.text()).collect::<Vec<_>>().join("")))}}}
 if rel.kind.ends_with("/drawing"){if let Ok(drawing)=pkg.xml(&path){let drawing_rels=pkg.rels(&path);for chart in drawing.desc("chart"){let title=drawing_rels.get(chart.attr("id")).and_then(|r|pkg.resolve(&path,&r.target)).and_then(|p|pkg.xml(&p).ok()).and_then(|n|n.desc("title").next().map(|n|n.text()));sheet.charts.push(title);ctx.warnings.push("chart inventory preserved; chart rasterization is not supported".into());}}}}
 }Ok(())}
fn ods_fidelity(pkg:&Package,sheets:&mut [Sheet],ctx:&mut ExtractCtx<'_>)->Result<()>{
    let root=pkg.xml("content.xml")?;
    let extra=pkg.optional_xml("styles.xml",ctx.warnings);
    let mut visibility=HashMap::new();
    for source in std::iter::once(&root).chain(extra.as_ref()){
        for style in source.desc("style"){
            if let Some(props)=style.child("table-properties"){visibility.insert(style.attr("name").to_owned(),props.attr("display")!="false");}
        }
    }
    for table in root.desc("table"){
        let Some(sheet)=sheets.iter_mut().find(|s|s.name==table.attr("name"))else{continue};
        sheet.hidden=!visibility.get(table.attr("style-name")).copied().unwrap_or(true);
        let mut row=0u32;
        for record in table.desc("table-row"){
            let repeats=record.attr("number-rows-repeated").parse::<u32>().unwrap_or(1).max(1);
            let mut col=0u32;
            for data in record.children(){
                if !matches!(package::local(&data.name),"table-cell"|"covered-table-cell"){continue}
                let count=data.attr("number-columns-repeated").parse::<u32>().unwrap_or(1).max(1);
                let formula=(!data.attr("formula").is_empty()).then(||data.attr("formula").to_owned());
                let url=data.desc("a").next().map(|a|a.attr("href").to_owned());
                let comment=data.desc("annotation").next().map(|n|n.text());
                let colspan=data.attr("number-columns-spanned").parse::<u32>().unwrap_or(1).max(1);
                let rowspan=data.attr("number-rows-spanned").parse::<u32>().unwrap_or(1).max(1);
                // Preflight above bounds the total expanded row/cell product before Calamine opens it.
                for dr in 0..repeats{for dc in 0..count{
                    let rc=(row.saturating_add(dr),col.saturating_add(dc));
                    if let Some(cell)=sheet.cells.get_mut(&rc){
                        if ctx.options.preserve_formulas&&formula.is_some(){cell.formula=formula.clone()}
                        if url.is_some(){cell.hyperlink=url.clone()}
                    }
                    if ctx.options.include_comments{if let Some(text)=&comment{sheet.comments.push((super::cell_ref(rc.0,rc.1),text.clone()))}}
                    if colspan>1||rowspan>1{let m=format!("{}:{}",super::cell_ref(rc.0,rc.1),super::cell_ref(rc.0.saturating_add(rowspan-1),rc.1.saturating_add(colspan-1)));if !sheet.merged.contains(&m){sheet.merged.push(m)}}
                }}
                col=col.saturating_add(count);
            }
            row=row.saturating_add(repeats);
        }
        for object in table.desc("object"){
            if let Some(path)=package::normalize("",object.attr("href")){
                let part=format!("{}/content.xml",path.trim_end_matches('/'));
                if let Ok(chart)=pkg.xml(&part){if chart.desc("chart").next().is_some(){sheet.charts.push(chart.desc("title").next().map(|n|n.text()));ctx.warnings.push("chart inventory preserved; chart rasterization is not supported".into());continue}}
            }
            ctx.warnings.push("ODF embedded object could not be classified; no rendered image available".into());
        }
    }
    if ctx.options.preserve_formulas&&sheets.iter().flat_map(|s|s.cells.values()).any(|c|c.formula.as_ref().map(|s|s.starts_with("of:")).unwrap_or(false)){ctx.warnings.push("ODF formula syntax preserved as-is; not translated to Excel syntax".into())}
    Ok(())
}
#[cfg(test)]mod date_format_tests{use super::{is_date_format,serial_to_iso};#[test]fn date_tokens(){assert!(is_date_format("m/d/yyyy"));assert!(is_date_format("yyyy-mm-dd"));assert!(is_date_format("[Red]yyyy-mm-dd"));assert!(is_date_format("m/d/yyyy h:mm"));assert!(!is_date_format("0%"));assert!(!is_date_format("0.00%"));assert!(!is_date_format("General"));assert!(!is_date_format("0.00"))}
#[test]fn serial_to_iso_known_values(){assert_eq!(serial_to_iso(1.0).as_deref(),Some("1900-01-01"));assert_eq!(serial_to_iso(59.0).as_deref(),Some("1900-02-28"));assert_eq!(serial_to_iso(61.0).as_deref(),Some("1900-03-01"));assert_eq!(serial_to_iso(44927.0).as_deref(),Some("2023-01-01"));assert_eq!(serial_to_iso(44927.5).as_deref(),Some("2023-01-01T12:00:00"));assert_eq!(serial_to_iso(-1.0),None);assert_eq!(serial_to_iso(f64::NAN),None);assert_eq!(serial_to_iso(9999999.0),None)}}
