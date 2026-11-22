use super::{AnnotationFile, format_type, layer_name};
use crate::dto::ShowView;
use crate::show_response_adapter::ShowResponseView;
use serde_json::{Value, json};
use spec_trail::domains::models::abstract_annotation::{AbstractAnnotation, AbstractLink};
use spec_trail::domains::models::implementation::{ImplementationAnnotation, ImplementationLink};
use spec_trail::domains::models::meta::MetaAnnotation;
use spec_trail::domains::models::spec_detail::{SpecDetailAnnotation, SpecDetailLink};
use spec_trail::domains::models::trace::Trace;
use spec_trail::domains::services::annotation::scanner::ScanWarning;
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
                "kind": rw.kind.as_str(),
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
        "traces": view_model.traces.len(),
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

    let doc_files: Vec<_> = view_model
        .document_annotations
        .iter()
        .map(file_summary_to_json)
        .collect();
    let code_files: Vec<_> = view_model
        .code_annotations
        .iter()
        .map(file_summary_to_json)
        .collect();

    json!({
        "total_annotations": code_totals + doc_totals,
        "warnings": warnings_to_json(&view_model.warnings),
        "document_files": doc_files,
        "code_files": code_files,
    })
}

pub fn render_group(view_model: &ShowResponseView) -> Value {
    let mut tree: BTreeMap<&'static str, BTreeMap<String, Vec<Value>>> = BTreeMap::new();

    let mut add_item =
        |layer: &'static str, anno_type: String, id: &str, name: &str, file: &str| {
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
        .chain(
            view_model
                .code_annotations
                .iter()
                .map(|c| c as &dyn AnnotationFile),
        )
        .collect();

    for file in files {
        let f = file.source_file();
        for m in file.metas() {
            add_item(
                layer_name(m.layer),
                format_type(m.r#type.as_ref()),
                &m.id.0,
                &m.name.0,
                f,
            );
        }
        for a in file.abstracts() {
            add_item(
                layer_name(a.layer),
                format_type(a.r#type.as_ref()),
                &a.id.0,
                &a.name.0,
                f,
            );
        }
        for d in file.details() {
            add_item(
                layer_name(d.layer),
                format_type(d.r#type.as_ref()),
                &d.id.0,
                &d.name.0,
                f,
            );
        }
        for i in file.implementations() {
            add_item(
                layer_name(i.layer),
                format_type(i.r#type.as_ref()),
                &i.id.0,
                &i.name.0,
                f,
            );
        }
    }

    let mut output = json!(tree);
    if let Value::Object(ref mut map) = output {
        map.insert(
            "warnings".to_string(),
            json!(warnings_to_json(&view_model.warnings)),
        );
    }

    output
}

fn link_to_json(id: &str, layer: &str) -> Value {
    json!({
        "id": id,
        "layer": layer,
    })
}

fn meta_to_json(m: &MetaAnnotation) -> Value {
    let links: Vec<_> = m
        .links
        .iter()
        .map(|id| link_to_json(&id.0, "Meta"))
        .collect();
    json!({
        "id": m.id.0,
        "name": m.name.0,
        "type": format_type(m.r#type.as_ref()),
        "layer": layer_name(m.layer),
        "links": links,
        "line": m.line.as_usize(),
    })
}

fn abstract_to_json(a: &AbstractAnnotation) -> Value {
    let links: Vec<_> = a
        .links
        .iter()
        .map(|link| match link {
            AbstractLink::Meta(id) => link_to_json(&id.0, "Meta"),
            AbstractLink::Abstract(id) => link_to_json(&id.0, "Abstract"),
            AbstractLink::SpecDetail(id) => link_to_json(&id.0, "SpecDetail"),
        })
        .collect();
    json!({
        "id": a.id.0,
        "name": a.name.0,
        "type": format_type(a.r#type.as_ref()),
        "layer": layer_name(a.layer),
        "links": links,
        "line": a.line.as_usize(),
    })
}

fn detail_to_json(d: &SpecDetailAnnotation) -> Value {
    let links: Vec<_> = d
        .links
        .iter()
        .map(|link| match link {
            SpecDetailLink::Abstract(id) => link_to_json(&id.0, "Abstract"),
            SpecDetailLink::Implementation(id) => link_to_json(&id.0, "Implementation"),
        })
        .collect();
    json!({
        "id": d.id.0,
        "name": d.name.0,
        "type": format_type(d.r#type.as_ref()),
        "layer": layer_name(d.layer),
        "links": links,
        "line": d.line.as_usize(),
    })
}

fn implementation_to_json(i: &ImplementationAnnotation) -> Value {
    let links: Vec<_> = i
        .links
        .iter()
        .map(|link| match link {
            ImplementationLink::SpecDetail(id) => link_to_json(&id.0, "SpecDetail"),
            ImplementationLink::Abstract(id) => link_to_json(&id.0, "Abstract"),
        })
        .collect();
    json!({
        "id": i.id.0,
        "name": i.name.0,
        "type": format_type(i.r#type.as_ref()),
        "layer": layer_name(i.layer),
        "links": links,
        "artifact": i.artifact.0,
        "status": i.status.as_ref().map(|s| format!("{:?}", s)),
        "line": i.line.as_usize(),
    })
}

fn trace_to_json(t: &Trace) -> Value {
    let (src_space, dst_space) = t.direction.spaces();
    json!({
        "src_space": src_space,
        "src_file": t.src_file,
        "src_line": t.src.line().as_usize(),
        "dst_space": dst_space,
        "dst_file": t.dst_file,
        "dst_line": t.dst.line().as_usize(),
        "src_id": t.src.id(),
        "src_layer": layer_name(t.src.layer()),
        "dst_id": t.dst.id(),
        "dst_layer": layer_name(t.dst.layer()),
        "kind": t.kind.to_string(),
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
        "traces": view_model.traces.iter().map(trace_to_json).collect::<Vec<_>>(),
        "warnings": warnings_to_json(&view_model.warnings),
    })
}
