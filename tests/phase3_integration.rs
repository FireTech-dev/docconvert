mod common;
use docconvert::{config::*,convert,model::*,render};
#[test]fn slide_order_and_per_slide_relationships(){let t=common::Temp::new();let d=common::extract(&t.write("slides.pptx",&common::pptx()));assert_eq!(d.meta.slide_count,Some(2));assert_eq!(d.assets.len(),1);let s=render::markdown::render_markdown(&d,false);assert!(s.find("First slide").unwrap()<s.find("Last slide").unwrap());assert!(!s.contains("Private speaker note"));assert!(d.blocks.iter().any(|b|matches!(b,Block::Boundary(Boundary::Slide{n:2,title:Some(_)}))))}
#[test]fn speaker_notes_opt_in(){let t=common::Temp::new();let p=t.write("slides.pptx",&common::pptx());let opts=Options{include_notes:true,format_choice:FormatChoice::Md,output:Some(t.0.join("out")),..Options::default()};let r=convert::convert_file(&p,&opts).unwrap();assert_eq!(r.slides,2);let s=std::fs::read_to_string(t.0.join("out/slides.md")).unwrap();assert!(s.contains("Private speaker note"));assert!(t.0.join("out/assets/slides/image-001.png").exists())}
#[test]fn pptx_cli_preview(){let t=common::Temp::new();let p=t.write("slides.pptx",&common::pptx());let r=std::process::Command::new(env!("CARGO_BIN_EXE_docconvert")).arg("--preview").arg(p).output().unwrap();assert!(r.status.success());assert!(String::from_utf8_lossy(&r.stdout).contains("Slides:      2"))}
#[test]fn slide_table_and_chart(){// P3-S02/S03: table slide survives with header; chart slide → asset-less Placeholder + inventory warning.
let t=common::Temp::new();
let slide2=br#"<p:sld xmlns:p="p" xmlns:a="a" xmlns:r="r" xmlns:c="c"><p:cSld><p:spTree><p:sp><p:nvSpPr><p:nvPr><p:ph type="title"/></p:nvPr></p:nvSpPr><p:txBody><a:p><a:r><a:t>Last slide</a:t></a:r></a:p></p:txBody></p:sp><p:graphicFrame><a:tbl><a:tr><a:tc><a:txBody><a:p><a:r><a:t>H</a:t></a:r></a:p></a:txBody></a:tc><a:tc><a:txBody><a:p><a:r><a:t>V</a:t></a:r></a:p></a:txBody></a:tc></a:tr><a:tr><a:tc><a:txBody><a:p><a:r><a:t>a</a:t></a:r></a:p></a:txBody></a:tc><a:tc><a:txBody><a:p><a:r><a:t>b</a:t></a:r></a:p></a:txBody></a:tc></a:tr></a:tbl><c:chart id="ch"/></p:graphicFrame></p:spTree></p:cSld></p:sld>"#.as_slice();
let rels=br#"<Relationships><Relationship Id="ch" Target="../charts/chart1.xml" Type="chart"/></Relationships>"#.as_slice();
let chart=br#"<c:chartSpace xmlns:c="c"><c:chart><c:title><a:t xmlns:a="a">Deck Chart</a:t></c:title></c:chart></c:chartSpace>"#.as_slice();
let bytes=common::zip_patch(&common::pptx(),&[("ppt/slides/slide2.xml",slide2),("ppt/slides/_rels/slide2.xml.rels",rels),("ppt/charts/chart1.xml",chart)]);
let d=common::extract(&t.write("deck.pptx",&bytes));
assert_eq!(d.meta.slide_count,Some(2));
assert!(d.blocks.iter().any(|b|matches!(b,Block::Boundary(Boundary::Slide{n:2,title:Some(_)}))));
let table=d.blocks.iter().find_map(|b|if let Block::Table(t)=b{Some(t)}else{None}).unwrap();
assert_eq!(table.header.len(),2);assert_eq!(table.rows.len(),1);
assert!(d.blocks.iter().any(|b|matches!(b,Block::Placeholder{kind:PlaceholderKind::Chart,label:Some(_),asset:None})));
assert!(d.warnings.iter().any(|s|s.contains("chart inventory")))}
