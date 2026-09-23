use super::{format_type, layer_name, AnnotationFile};
use crate::dto::ShowView;
use crate::show_response_adapter::ShowResponseView;
use SpecTrail::domains::models::abstract_annotation::AbstractAnnotation;
use SpecTrail::domains::models::implementation::ImplementationAnnotation;
use SpecTrail::domains::models::meta::MetaAnnotation;
use SpecTrail::domains::models::spec_detail::SpecDetailAnnotation;
use SpecTrail::domains::services::annotation::scanner::ScanWarning;
use serde_json::{json, Value};
use std::collections::BTreeMap;

pub fn render(view_model: &ShowResponseView) {
    let json_value = match view_model.view {
        ShowView::Summary => render_summary(view_model),
        ShowView::List => render_list(view_model),
        ShowView::Group => render_group(view_model),
        ShowView::Detail => render_detail(view_model),
    };

    if let Ok(serialized) = serde_json::to_string_pretty(&json_value) {
        println!("{}", serialized);
    }
}

pub fn warnings_to_json(warnings: &[ScanWarning]) -> Vec<Value> {
    warnings
        .iter()
        .map(|w| match w {
            ScanWarning::Parse(pw) => json!({
                "type": "parse",
                "source_file": pw.source_file,
                "line": pw.line.as_usize(),
                "message": pw.message,
                "raw_text": pw.raw_text,
            }),
            ScanWarning::Resolve(rw) => json!({
                "type": "resolve",
                "source_annotation_id": rw.source_annotation_id,
                "source_file": rw.source_file,
                "line": rw.line.as_usize(),
                "message": rw.message,
            }),
        })
        .collect()
}

fn count_annotations<T: AnnotationFile>(files: &[T]) -> usize {
    files.iter().map(|f| f.total_count()).sum()
}

pub fn render_summary(view_model: &ShowResponseView) -> Value {
    let code_totals = count_annotations(&view_model.code_annotations);
    let document_totals = count_annotations(&view_model.document_annotations);

    json!({
        "code_files": view_model.code_annotations.len(),
        "code_annotations": code_totals,
        "document_files": view_model.document_annotations.len(),
        "document_annotations": document_totals,
        "warnings": warnings_to_json(&view_model.warnings),
    })
}

fn file_summary_to_json(file: &impl AnnotationFile) -> Value {
    json!({
        "source_file": file.source_file(),
        "metas": file.metas().len(),
        "abstracts": file.abstracts().len(),
        "details": file.details().len(),
        "implementations": file.implementations().len(),
    })
}

pub fn render_list(view_model: &ShowResponseView) -> Value {
    let code_totals = count_annotations(&view_model.code_annotations);
    let doc_totals = count_annotations(&view_model.document_annotations);

    let doc_files: Vec<_> = view_model.document_annotations.iter().map(file_summary_to_json).collect();
    let code_files: Vec<_> = view_model.code_annotations.iter().map(file_summary_to_json).collect();

    json!({
        "total_annotations": code_totals + doc_totals,
        "warnings": warnings_to_json(&view_model.warnings),
        "document_files": doc_files,
        "code_files": code_files,
    })
}

pub fn render_group(view_model: &ShowResponseView) -> Value {
    let mut tree: BTreeMap<&'static str, BTreeMap<String, Vec<Value>>> = BTreeMap::new();

    let mut add_item = |layer: &'static str, anno_type: String, id: &str, name: &str, file: &str| {
        tree.entry(layer)
            .or_default()
            .entry(anno_type)
            .or_default()
            .push(json!({
                "id": format!("@{}", id),
                "name": name,
                "source_file": file,
            }));
    };

    let files: Vec<&dyn AnnotationFile> = view_model
        .document_annotations
        .iter()
        .map(|d| d as &dyn AnnotationFile)
        .chain(view_model.code_annotations.iter().map(|c| c as &dyn AnnotationFile))
        .collect();

    for file in files {
        let f = file.source_file();
        for m in file.metas() {
            add_item(layer_name(m.layer), format_type(m.r#type.as_ref()), &m.id.0, &m.name.0, f);
        }
        for a in file.abstracts() {
            add_item(layer_name(a.layer), format_type(a.r#type.as_ref()), &a.id.0, &a.name.0, f);
        }
        for d in file.details() {
            add_item(layer_name(d.layer), format_type(d.r#type.as_ref()), &d.id.0, &d.name.0, f);
        }
        for i in file.implementations() {
            add_item(layer_name(i.layer), format_type(i.r#type.as_ref()), &i.id.0, &i.name.0, f);
        }
    }

    json!(tree)
}

fn meta_to_json(m: &MetaAnnotation) -> Value {
    json!({
        "id": m.id.0,
        "name": m.name.0,
        "type": format_type(m.r#type.as_ref()),
        "layer": layer_name(m.layer),
        "line": m.line.as_usize(),
    })
}

fn abstract_to_json(a: &AbstractAnnotation) -> Value {
    json!({
        "id": a.id.0,
        "name": a.name.0,
        "type": format_type(a.r#type.as_ref()),
        "layer": layer_name(a.layer),
        "line": a.line.as_usize(),
    })
}

fn detail_to_json(d: &SpecDetailAnnotation) -> Value {
    json!({
        "id": d.id.0,
        "name": d.name.0,
        "type": format_type(d.r#type.as_ref()),
        "layer": layer_name(d.layer),
        "line": d.line.as_usize(),
    })
}

fn implementation_to_json(i: &ImplementationAnnotation) -> Value {
    json!({
        "id": i.id.0,
        "name": i.name.0,
        "type": format_type(i.r#type.as_ref()),
        "layer": layer_name(i.layer),
        "artifact": i.artifact.0,
        "line": i.line.as_usize(),
    })
}

fn file_detail_to_json(file: &impl AnnotationFile) -> Value {
    json!({
        "source_file": file.source_file(),
        "metas": file.metas().iter().map(meta_to_json).collect::<Vec<_>>(),
        "abstracts": file.abstracts().iter().map(abstract_to_json).collect::<Vec<_>>(),
        "details": file.details().iter().map(detail_to_json).collect::<Vec<_>>(),
        "implementations": file.implementations().iter().map(implementation_to_json).collect::<Vec<_>>(),
    })
}

pub fn render_detail(view_model: &ShowResponseView) -> Value {
    json!({
        "document_annotations": view_model.document_annotations.iter().map(file_detail_to_json).collect::<Vec<_>>(),
        "code_annotations": view_model.code_annotations.iter().map(file_detail_to_json).collect::<Vec<_>>(),
        "warnings": warnings_to_json(&view_model.warnings),
    })
}
