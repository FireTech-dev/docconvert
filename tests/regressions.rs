mod common;
use docconvert::{config::*,convert,extract::{package,text},model::*,render};

#[test]
fn xml_views_survive_root_drop_without_a_child_tree(){
    let (root,_)=package::parse(b"<root><a id='one'>first<b>nested</b>end</a><a id='two'/></root>",false).unwrap();
    let first=root.desc("a").next().unwrap();drop(root);
    assert_eq!(first.attr("id"),"one");assert_eq!(first.text(),"firstnestedend");
    assert_eq!(first.child("b").unwrap().text(),"nested");
    assert_eq!(first.children().count(),1);
}
#[test]
fn xml_stream_order_and_cdata(){
    let (root,_)=package::parse(b"<a><x>1</x><b><x><![CDATA[2 < 3]]></x></b><x>4 &amp; 5</x></a>",false).unwrap();
    assert_eq!(root.desc("x").map(|n|n.text()).collect::<Vec<_>>(),vec!["1","2 < 3","4 & 5"]);
}
#[test]
fn xml_strict_rejects_mismatched_nesting(){assert!(package::parse(b"<a><b></a>",false).is_err());}
#[test]
fn html_tolerates_void_and_unclosed_tags(){
    let (root,_)=package::parse(b"<HTML><BODY><p>one<br>two<img src='x'></missing>",true).unwrap();
    assert_eq!(root.desc("br").count(),1);assert_eq!(root.desc("img").count(),1);assert_eq!(root.desc("p").next().unwrap().text(),"onetwo");
}
#[test]
fn external_entities_are_never_loaded(){
    let (root,warnings)=package::parse(br#"<!DOCTYPE r [<!ENTITY file SYSTEM "file:///not-permitted">]><r>&file;</r>"#,false).unwrap();
    assert!(!warnings.is_empty());assert!(!root.text().contains("not-permitted"));
}
#[test]
fn html_navigation_count_and_entity_single_pass(){
    let t=common::Temp::new();let p=t.write("article.html",b"<html><body><nav>remove</nav><header>remove</header><p>&amp;lt; &copy;</p><aside>remove</aside></body></html>");
    let d=common::extract(&p);assert!(d.warnings.iter().any(|w|w=="skipped 3 navigation/interface sections"));
    assert!(d.blocks.iter().any(|b|matches!(b,Block::Paragraph(xs) if inline_text(xs).contains("&lt; ©"))));
}
#[test]
fn inline_html_images_keep_text_order(){
    let t=common::Temp::new();t.write("image.png",&common::png());let p=t.write("inline.html",b"<html><body><p>before <img src='image.png' alt='middle'> after</p></body></html>");
    let d=common::extract(&p);let xs=d.blocks.iter().find_map(|b|if let Block::Paragraph(xs)=b{Some(xs)}else{None}).unwrap();
    assert!(matches!(&xs[1],Inline::Image{..}));assert_eq!(d.assets.len(),1);
}
#[test]
fn markdown_local_images_are_embedded_with_safe_paths(){
    let t=common::Temp::new();t.write("image.png",&common::png());let p=t.write("image.md",b"Before ![alt](image.png) after");
    let d=common::extract(&p);assert_eq!(d.assets.len(),1);assert!(d.warnings.iter().all(|w|w!="external/missing image not embedded"));
    let xs=d.blocks.iter().find_map(|b|if let Block::Paragraph(xs)=b{Some(xs)}else{None}).unwrap();assert!(xs.iter().any(|i|matches!(i,Inline::Image{..})));
}
#[test]
fn markdown_remote_or_traversing_images_remain_links(){
    let t=common::Temp::new();let p=t.write("image.md",b"![x](https://example.org/x.png) ![y](../../outside.png)");
    let d=common::extract(&p);assert!(d.assets.is_empty());assert_eq!(d.warnings.iter().filter(|w|w.as_str()=="external/missing image not embedded").count(),2);
}
#[test]
fn markdown_hard_breaks_and_code_runs(){
    let d=text::extract_markdown(b"first  \nsecond\n\n`` a`b ``",Meta::default());
    assert!(matches!(&d.blocks[0],Block::Paragraph(xs) if xs.iter().any(|i|matches!(i,Inline::Break))));
    assert!(matches!(&d.blocks[1],Block::Paragraph(xs) if xs.iter().any(|i|matches!(i,Inline::Code(s) if s=="a`b"))));
}
#[test]
fn markdown_mixed_nested_lists_keep_order(){
    let d=text::extract_markdown(b"- parent\n  1. child one\n  2. child two\n- next",Meta::default());
    let s=render::markdown::render_markdown(&d,false);
    assert!(s.contains("  1. child one"));assert!(s.contains("  2. child two"));assert!(s.find("child two").unwrap()<s.find("next").unwrap());
}
fn merged_table()->Table{Table{header:vec![TableCell{inlines:vec![Inline::Text("A".into())],rowspan:2,..TableCell::default()},TableCell{inlines:vec![Inline::Text("B".into())],..TableCell::default()}],rows:vec![vec![TableCell{inlines:vec![Inline::Text("C".into())],..TableCell::default()}]],..Table::default()}}
#[test]
fn vertical_merges_do_not_shift_later_cells(){let d=Document{blocks:vec![Block::Table(merged_table())],..Document::default()};assert!(render::markdown::render_markdown(&d,false).contains("|  | C |"));}
#[test]
fn table_code_pipes_are_escaped(){
    let t=Table{header:vec![TableCell{inlines:vec![Inline::Code("a|b".into())],..TableCell::default()}],..Table::default()};
    let d=Document{blocks:vec![Block::Table(t)],..Document::default()};assert!(render::markdown::render_markdown(&d,false).contains("a\\|b"));
}
#[test]
fn txt_long_cells_wrap_without_losing_characters(){
    let long="abcdefghijklmnopqrstuvwxyz0123456789TAIL";
    let table=Table{header:vec![TableCell{inlines:vec![Inline::Text("Value".into())],..TableCell::default()}],rows:vec![vec![TableCell{inlines:vec![Inline::Text(long.into())],..TableCell::default()}]],..Table::default()};
    let s=render::txt::render_txt(&Document{blocks:vec![Block::Table(table)],..Document::default()});
    assert!(s.contains("6789TAIL"));assert!(!s.contains('…'));assert!(s.lines().filter(|l|!l.starts_with("Value")&&!l.starts_with('-')).collect::<String>().contains(long));
}
#[test]
fn tts_minimal_placeholders_preserve_alt_and_caption(){
    let d=Document{blocks:vec![Block::Image{asset:0,alt:Some("Useful alt".into()),caption:Some("Useful caption".into())}],..Document::default()};
    let text=render::txt::render_txt_with_options(&d,true);assert!(text.contains("Useful alt"));assert!(text.contains("Useful caption"));assert!(!text.contains("[Image"));
}
#[test]
fn rtf_unclosed_picture_is_finalized(){let t=common::Temp::new();let d=common::extract(&t.write("image.rtf",br"{\rtf1 {\pict\pngblip 89504e470d0a1a0a"));assert_eq!(d.assets.len(),1);assert!(d.warnings.iter().any(|w|w.contains("unterminated")));}
#[test]
fn rtf_nested_picture_groups_do_not_duplicate_assets(){let t=common::Temp::new();let d=common::extract(&t.write("image.rtf",br"{\rtf1 {\pict\pngblip 89504e47{0d0a1a0a}}}"));assert_eq!(d.assets.len(),1);}
#[test]
fn docx_optional_part_errors_are_reported(){
    let bytes=common::zip(&[("[Content_Types].xml",b"<Types/>"),("word/document.xml",b"<w:document><w:body><w:p><w:r><w:t>kept</w:t></w:r></w:p></w:body></w:document>"),("word/styles.xml",b"<broken")]);
    let t=common::Temp::new();let d=common::extract(&t.write("a.docx",&bytes));assert!(d.warnings.iter().any(|w|w.contains("styles.xml")));assert!(d.text_len()>0);
}
#[test]
fn zip_case_collisions_are_rejected(){let bytes=common::zip(&[("one.xml",b"a"),("ONE.xml",b"b")]);assert!(package::Package::open(&bytes).is_err());}
#[test]
fn source_overwrite_stays_refused_after_renderer_changes(){let t=common::Temp::new();let p=t.write("a.md",b"source");let o=Options{format_choice:FormatChoice::Md,overwrite:true,..Options::default()};assert!(convert::convert_file(&p,&o).is_err());assert_eq!(std::fs::read(p).unwrap(),b"source");}

#[test]
fn epub_cross_chapter_footnote_reference(){
    let chapter=br##"<html><body><h1>Last chapter</h1><p>Cross reference <a href="b.xhtml#n">note</a></p></body></html>"##;
    let bytes=common::zip_patch(&common::epub(),&[("OEBPS/text/a.xhtml",chapter)]);
    let t=common::Temp::new();let d=common::extract(&t.write("book.epub",&bytes));
    let output=render::markdown::render_markdown(&d,false);assert_eq!(d.footnotes.len(),1);assert_eq!(output.matches("[^ch1-n]").count(),3);
}
#[test]
fn pptx_picture_table_chart_keep_shape_order(){
    let slide=br#"<p:sld xmlns:p="p" xmlns:a="a" xmlns:c="c" xmlns:r="r"><p:cSld><p:spTree><p:pic><p:blipFill><a:blip r:embed="image1"/></p:blipFill></p:pic><p:sp><p:txBody><a:p><a:r><a:t>After picture</a:t></a:r></a:p></p:txBody></p:sp><p:graphicFrame><a:tbl><a:tr><a:tc rowSpan="2"><a:txBody><a:p><a:r><a:t>Merged</a:t></a:r></a:p></a:txBody></a:tc><a:tc><a:txBody><a:p><a:r><a:t>Header B</a:t></a:r></a:p></a:txBody></a:tc></a:tr><a:tr><a:tc vMerge="1"/><a:tc><a:txBody><a:p><a:r><a:t>Body B</a:t></a:r></a:p></a:txBody></a:tc></a:tr></a:tbl></p:graphicFrame><p:graphicFrame><c:chart r:id="chart1"/></p:graphicFrame><p:sp><p:txBody><a:p><a:r><a:t>After chart</a:t></a:r></a:p></p:txBody></p:sp></p:spTree></p:cSld></p:sld>"#;
    let bytes=common::zip_patch(&common::pptx(),&[("ppt/slides/slide7.xml",slide)]);
    let t=common::Temp::new();let d=common::extract(&t.write("deck.pptx",&bytes));
    assert!(matches!(&d.blocks[0],Block::Image{..}));assert!(matches!(&d.blocks[1],Block::Paragraph(_)));assert!(matches!(&d.blocks[2],Block::Table(_)));assert!(matches!(&d.blocks[3],Block::Placeholder{..}));
    let text=render::markdown::render_markdown(&d,false);assert!(text.contains("|  | Body B |"));
}
#[test]
fn ods_repeated_metadata_is_expanded(){
    let content=br#"<office:document-content xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0" xmlns:table="urn:oasis:names:tc:opendocument:xmlns:table:1.0" xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0" xmlns:xlink="http://www.w3.org/1999/xlink"><office:body><office:spreadsheet><table:table table:name="Data"><table:table-row table:number-rows-repeated="2"><table:table-cell table:number-columns-repeated="2" office:value-type="float" office:value="3" table:formula="of:=1+2"><text:p><text:a xlink:href="https://example.org">3</text:a></text:p><office:annotation><text:p>Repeated comment</text:p></office:annotation></table:table-cell></table:table-row></table:table></office:spreadsheet></office:body></office:document-content>"#;
    let bytes=common::zip_patch(&common::ods(),&[("content.xml",content)]);let t=common::Temp::new();let d=common::extract(&t.write("book.ods",&bytes));let sheet=&d.workbook.as_ref().unwrap().sheets[0];
    assert_eq!(sheet.comments.len(),4);for rc in [(0,0),(0,1),(1,0),(1,1)]{assert_eq!(sheet.cells[&rc].formula.as_deref(),Some("of:=1+2"));assert_eq!(sheet.cells[&rc].hyperlink.as_deref(),Some("https://example.org"));}
}
#[test]
fn report_conflict_does_not_write_partial_output(){
    let t=common::Temp::new();let input=t.write("source.txt",b"new content");t.write("out/source.report.json",b"old report");
    let result=convert::convert_file(&input,&Options{format_choice:FormatChoice::Md,output:Some(t.0.join("out")),..Options::default()});
    assert!(result.is_err());assert!(!t.0.join("out/source.md").exists());assert_eq!(std::fs::read(t.0.join("out/source.report.json")).unwrap(),b"old report");
    assert!(!std::fs::read_dir(t.0.join("out")).unwrap().flatten().any(|e|e.file_name().to_string_lossy().starts_with(".docconvert-")));
}
#[test]
fn staged_overwrite_replaces_outputs_and_cleans_backups(){
    let t=common::Temp::new();let input=t.write("source.txt",b"new content");t.write("out/source.md",b"old output");t.write("out/source.report.json",b"old report");
    convert::convert_file(&input,&Options{format_choice:FormatChoice::Md,output:Some(t.0.join("out")),overwrite:true,..Options::default()}).unwrap();
    assert_eq!(std::fs::read(t.0.join("out/source.md")).unwrap(),b"new content\n");assert!(!std::fs::read_dir(t.0.join("out")).unwrap().flatten().any(|e|e.file_name().to_string_lossy().starts_with(".docconvert-")));
}
#[cfg(unix)]
#[test]
fn report_symlink_rejected_before_replacing_output(){
    let t=common::Temp::new();let input=t.write("source.txt",b"new content");t.write("out/source.md",b"old output");let protected=t.write("protected",b"do not modify");
    std::os::unix::fs::symlink(&protected,t.0.join("out/source.report.json")).unwrap();
    assert!(convert::convert_file(&input,&Options{format_choice:FormatChoice::Md,output:Some(t.0.join("out")),overwrite:true,..Options::default()}).is_err());
    assert_eq!(std::fs::read(t.0.join("out/source.md")).unwrap(),b"old output");assert_eq!(std::fs::read(protected).unwrap(),b"do not modify");
}

#[test]
fn all_remaining_omml_mapping_rules(){
    use docconvert::extract::ooxml;
    let cases=[
        ("<m:sSub><m:e><m:r><m:t>x</m:t></m:r></m:e><m:sub><m:r><m:t>i</m:t></m:r></m:sub></m:sSub>","{x}_{i}"),
        ("<m:sSubSup><m:e><m:r><m:t>x</m:t></m:r></m:e><m:sub><m:r><m:t>i</m:t></m:r></m:sub><m:sup><m:r><m:t>2</m:t></m:r></m:sup></m:sSubSup>","{x}_{i}^{2}"),
        ("<m:rad><m:deg><m:r><m:t>3</m:t></m:r></m:deg><m:e><m:r><m:t>x</m:t></m:r></m:e></m:rad>","\\sqrt[3]{x}"),
        ("<m:nary><m:naryPr><m:chr m:val='∑'/></m:naryPr><m:sub><m:r><m:t>1</m:t></m:r></m:sub><m:sup><m:r><m:t>n</m:t></m:r></m:sup><m:e><m:r><m:t>x</m:t></m:r></m:e></m:nary>","\\sum_{1}^{n}{x}"),
        ("<m:d><m:e><m:r><m:t>x</m:t></m:r></m:e></m:d>","\\left(x\\right)"),
        ("<m:acc><m:accPr><m:chr m:val='^'/></m:accPr><m:e><m:r><m:t>x</m:t></m:r></m:e></m:acc>","\\hat{x}"),
        ("<m:m><m:mr><m:e><m:r><m:t>1</m:t></m:r></m:e><m:e><m:r><m:t>2</m:t></m:r></m:e></m:mr><m:mr><m:e><m:r><m:t>3</m:t></m:r></m:e><m:e><m:r><m:t>4</m:t></m:r></m:e></m:mr></m:m>","\\begin{matrix}1 & 2 \\\\ 3 & 4\\end{matrix}"),
    ];
    for(xml,expected)in cases{let(root,_)=package::parse(xml.as_bytes(),false).unwrap();let mut warnings=vec![];let result=ooxml::omml_to_latex(&root.children().next().unwrap(),&mut warnings);assert_eq!(result,expected);assert!(warnings.is_empty());}
}
#[test]
fn omml_deep_nesting_stays_bounded(){
    use docconvert::extract::ooxml;
    // 60-deep stays under both package MAX_DEPTH (64) and the omml 64-guard:
    // converts without a nesting-limit warning (guard is defense-in-depth;
    // deeper input is rejected at parse time before omml runs).
    let depth=60;let xml=format!("{}<m:t>deep</m:t>{}","<m:oMath>".repeat(depth),"</m:oMath>".repeat(depth));
    let(root,_)=package::parse(xml.as_bytes(),false).unwrap();let mut warnings=vec![];
    let result=ooxml::omml_to_latex(&root.children().next().unwrap(),&mut warnings);
    assert!(result.contains("deep"));assert!(!warnings.iter().any(|w|w.contains("nesting limit")));
}
