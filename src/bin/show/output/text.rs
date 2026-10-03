use super::{format_type, layer_name, AnnotationFile};
use crate::dto::ShowView;
use crate::show_response_adapter::ShowResponseView;
use spec_trail::domains::services::annotation::scanner::ScanWarning;
use std::collections::BTreeMap;

pub fn render(view_model: &ShowResponseView) {
    print_warnings(&view_model.warnings);
    match view_model.view {
        ShowView::Summary => render_summary(view_model),
        ShowView::List => render_list(view_model),
        ShowView::Group => render_group(view_model),
        ShowView::Detail => render_detail(view_model),
    }
}

pub fn print_warnings(warnings: &[ScanWarning]) {
    for warning in warnings {
        match warning {
            ScanWarning::Parse(pw) => {
                eprintln!(
                    "WARNING [parse] {}:{}: {} (raw: '{}')",
                    pw.source_file, pw.line, pw.message, pw.raw_text
                );
            }
            ScanWarning::Resolve(rw) => {
                if rw.line.as_usize() > 0 {
                    eprintln!("WARNING [resolve] {}:{}: {}", rw.source_file, rw.line, rw.message);
                } else {
                    eprintln!("WARNING [resolve] {}", rw.message);
                }
            }
        }
    }
}

fn count_annotations<T: AnnotationFile>(files: &[T]) -> usize {
    files.iter().map(|f| f.total_count()).sum()
}

pub fn render_summary(view_model: &ShowResponseView) {
    let code_totals = count_annotations(&view_model.code_annotations);
    let document_totals = count_annotations(&view_model.document_annotations);

    println!("Show summary");
    println!("  code files: {}", view_model.code_annotations.len());
    println!("  code annotations: {}", code_totals);
    println!("  document files: {}", view_model.document_annotations.len());
    println!("  document annotations: {}", document_totals);
    println!("  warnings: {}", view_model.warnings.len());
}

fn print_file_list_section<T: AnnotationFile>(files: &[T], source_label: &str) {
    let mut items: Vec<_> = files.iter().collect();
    items.sort_by(|a, b| a.source_file().cmp(b.source_file()));

    for (index, file) in items.into_iter().enumerate() {
        println!("[{}] {}", index + 1, file.source_file());
        println!("    metas: {}", file.metas().len());
        println!("    abstracts: {}", file.abstracts().len());
        println!("    details: {}", file.details().len());
        println!("    implementations: {}", file.implementations().len());
        println!("    source: {}", source_label);
        println!();
    }
}

pub fn render_list(view_model: &ShowResponseView) {
    let code_totals = count_annotations(&view_model.code_annotations);
    let doc_totals = count_annotations(&view_model.document_annotations);

    println!("Found {} annotations", code_totals + doc_totals);
    println!("Warnings: {}", view_model.warnings.len());
    println!();

    print_file_list_section(&view_model.document_annotations, "document");
    print_file_list_section(&view_model.code_annotations, "code");
}

pub fn render_group(view_model: &ShowResponseView) {
    struct GroupItem<'a> {
        id: &'a str,
        name: &'a str,
        file: &'a str,
    }

    let mut tree: BTreeMap<&'static str, BTreeMap<String, Vec<GroupItem>>> = BTreeMap::new();

    let files: Vec<&dyn AnnotationFile> = view_model
        .document_annotations
        .iter()
        .map(|d| d as &dyn AnnotationFile)
        .chain(view_model.code_annotations.iter().map(|c| c as &dyn AnnotationFile))
        .collect();

    for file in files {
        let f = file.source_file();
        for m in file.metas() {
            tree.entry(layer_name(m.layer))
                .or_default()
                .entry(format_type(m.r#type.as_ref()))
                .or_default()
                .push(GroupItem { id: &m.id.0, name: &m.name.0, file: f });
        }
        for a in file.abstracts() {
            tree.entry(layer_name(a.layer))
                .or_default()
                .entry(format_type(a.r#type.as_ref()))
                .or_default()
                .push(GroupItem { id: &a.id.0, name: &a.name.0, file: f });
        }
        for d in file.details() {
            tree.entry(layer_name(d.layer))
                .or_default()
                .entry(format_type(d.r#type.as_ref()))
                .or_default()
                .push(GroupItem { id: &d.id.0, name: &d.name.0, file: f });
        }
        for i in file.implementations() {
            tree.entry(layer_name(i.layer))
                .or_default()
                .entry(format_type(i.r#type.as_ref()))
                .or_default()
                .push(GroupItem { id: &i.id.0, name: &i.name.0, file: f });
        }
    }

    for (layer, types) in tree {
        println!("Layer: {}", layer);
        for (anno_type, list) in types {
            println!("  Type: {}", anno_type);
            for item in list {
                println!("    - @{}: {} ({})", item.id, item.name, item.file);
            }
        }
        println!();
    }
}

fn print_file_detail(file: &impl AnnotationFile) {
    println!("File: {}", file.source_file());
    for m in file.metas() {
        println!("  [@{}] line: {}, Layer: Meta, Type: {:?}, Name: {}", m.id.0, m.line, m.r#type, m.name.0);
    }
    for a in file.abstracts() {
        println!("  [@{}] line: {}, Layer: Abstract, Type: {:?}, Name: {}", a.id.0, a.line, a.r#type, a.name.0);
    }
    for d in file.details() {
        println!("  [@{}] line: {}, Layer: SpecDetail, Type: {:?}, Name: {}", d.id.0, d.line, d.r#type, d.name.0);
    }
    for i in file.implementations() {
        println!(
            "  [@{}] line: {}, Layer: Implementation, Type: {:?}, Name: {}, Status: {:?}",
            i.id.0, i.line, i.r#type, i.name.0, i.status
        );
    }
    println!();
}

pub fn render_detail(view_model: &ShowResponseView) {
    println!("=== Document Annotations ===");
    for doc in &view_model.document_annotations {
        print_file_detail(doc);
    }

    println!("=== Code Annotations ===");
    for code in &view_model.code_annotations {
        print_file_detail(code);
    }
}
