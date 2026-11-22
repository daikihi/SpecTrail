use crate::dto::ShowFormat;
use crate::show_response_adapter::ShowResponseView;
use spec_trail::domains::models::abstract_annotation::AbstractAnnotation;
use spec_trail::domains::models::annotation::code_annotation::CodeAnnotation;
use spec_trail::domains::models::annotation::document_annotation::DocumentAnnotation;
use spec_trail::domains::models::implementation::ImplementationAnnotation;
use spec_trail::domains::models::layer::Layer;
use spec_trail::domains::models::meta::MetaAnnotation;
use spec_trail::domains::models::spec_detail::SpecDetailAnnotation;

mod json;
mod text;

/// Trait abstracting over DocumentAnnotation and CodeAnnotation file containers.
pub trait AnnotationFile {
    fn source_file(&self) -> &str;
    fn metas(&self) -> &[MetaAnnotation];
    fn abstracts(&self) -> &[AbstractAnnotation];
    fn details(&self) -> &[SpecDetailAnnotation];
    fn implementations(&self) -> &[ImplementationAnnotation];

    fn total_count(&self) -> usize {
        self.metas().len()
            + self.abstracts().len()
            + self.details().len()
            + self.implementations().len()
    }
}

impl AnnotationFile for DocumentAnnotation {
    fn source_file(&self) -> &str {
        &self.source_file
    }
    fn metas(&self) -> &[MetaAnnotation] {
        &self.metas
    }
    fn abstracts(&self) -> &[AbstractAnnotation] {
        &self.abstracts
    }
    fn details(&self) -> &[SpecDetailAnnotation] {
        &self.details
    }
    fn implementations(&self) -> &[ImplementationAnnotation] {
        &self.implementations
    }
}

impl AnnotationFile for CodeAnnotation {
    fn source_file(&self) -> &str {
        &self.source_file
    }
    fn metas(&self) -> &[MetaAnnotation] {
        &self.metas
    }
    fn abstracts(&self) -> &[AbstractAnnotation] {
        &self.abstracts
    }
    fn details(&self) -> &[SpecDetailAnnotation] {
        &self.details
    }
    fn implementations(&self) -> &[ImplementationAnnotation] {
        &self.implementations
    }
}

pub fn layer_name(layer: Layer) -> &'static str {
    match layer {
        Layer::Meta => "Meta",
        Layer::Abstract => "Abstract",
        Layer::SpecDetail => "SpecDetail",
        Layer::Implementation => "Implementation",
    }
}

pub fn format_type<T: std::fmt::Debug>(t: Option<&T>) -> String {
    t.map(|v| format!("{:?}", v))
        .unwrap_or_else(|| "unknown".to_string())
}

/// Entry point for rendering the ShowResponseView according to requested view and format.
pub fn render(view_model: &ShowResponseView) {
    match view_model.format {
        ShowFormat::Text => text::render(view_model),
        ShowFormat::Json => json::render(view_model),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::{ShowFormat, ShowView};
    use spec_trail::domains::models::layer::Layer;
    use spec_trail::domains::models::line_number::LineNumber;
    use spec_trail::domains::models::meta::{MetaAnnotation, MetaAnnotationId, MetaName};

    #[test]
    fn test_render_summary_does_not_panic() {
        let view_model = ShowResponseView {
            document_annotations: vec![],
            code_annotations: vec![],
            traces: vec![],
            warnings: vec![],
            view: ShowView::Summary,
            format: ShowFormat::Text,
        };
        render(&view_model);
    }

    #[test]
    fn test_render_json_group_structure() {
        let meta = MetaAnnotation {
            id: MetaAnnotationId("st-meta-test".to_string()),
            name: MetaName("Meta Test".to_string()),
            r#type: None,
            layer: Layer::Meta,
            links: vec![],
            line: LineNumber(1),
        };
        let doc = DocumentAnnotation {
            source_file: "test.md".to_string(),
            metas: vec![meta],
            abstracts: vec![],
            details: vec![],
            implementations: vec![],
        };
        let view_model = ShowResponseView {
            document_annotations: vec![doc],
            code_annotations: vec![],
            traces: vec![],
            warnings: vec![],
            view: ShowView::Group,
            format: ShowFormat::Json,
        };
        let json_val = json::render_group(&view_model);
        assert!(json_val.get("Meta").is_some());
    }

    #[test]
    fn test_render_json_detail_structure() {
        let meta = MetaAnnotation {
            id: MetaAnnotationId("st-meta-test".to_string()),
            name: MetaName("Meta Test".to_string()),
            r#type: None,
            layer: Layer::Meta,
            links: vec![MetaAnnotationId("st-meta-other".to_string())],
            line: LineNumber(1),
        };
        let doc = DocumentAnnotation {
            source_file: "test.md".to_string(),
            metas: vec![meta],
            abstracts: vec![],
            details: vec![],
            implementations: vec![],
        };
        let view_model = ShowResponseView {
            document_annotations: vec![doc],
            code_annotations: vec![],
            traces: vec![],
            warnings: vec![],
            view: ShowView::Detail,
            format: ShowFormat::Json,
        };
        let json_val = json::render_detail(&view_model);
        assert!(json_val.get("document_annotations").is_some());
        assert!(json_val.get("code_annotations").is_some());
        assert!(json_val.get("traces").is_some());
        let meta_links = &json_val["document_annotations"][0]["metas"][0]["links"];
        assert_eq!(meta_links[0]["id"], "st-meta-other");
        assert_eq!(meta_links[0]["layer"], "Meta");
    }

    #[test]
    fn test_render_json_group_includes_warnings() {
        use spec_trail::domains::services::annotation::parser::ParseWarning;
        use spec_trail::domains::services::annotation::scanner::ScanWarning;

        let view_model = ShowResponseView {
            document_annotations: vec![],
            code_annotations: vec![],
            traces: vec![],
            warnings: vec![ScanWarning::Parse(ParseWarning {
                source_file: "test.rs".to_string(),
                line: LineNumber(5),
                message: "syntax error".to_string(),
                raw_text: "@bad".to_string(),
            })],
            view: ShowView::Group,
            format: ShowFormat::Json,
        };
        let json_val = json::render_group(&view_model);
        let warnings = json_val
            .get("warnings")
            .expect("warnings should be present in group output");
        assert_eq!(warnings.as_array().unwrap().len(), 1);
        assert_eq!(warnings[0]["type"], "parse");
        assert_eq!(warnings[0]["message"], "syntax error");
    }

    #[test]
    fn test_render_json_detail_implementation_fields() {
        use spec_trail::domains::models::abstract_annotation::AbstractAnnotationId;
        use spec_trail::domains::models::annotation::code_annotation::CodeAnnotation;
        use spec_trail::domains::models::implementation::{
            ImplementationAnnotation, ImplementationAnnotationId, ImplementationArtifact,
            ImplementationLink, ImplementationSpecName, ImplementationStatus,
        };

        let impl_annotation = ImplementationAnnotation {
            id: ImplementationAnnotationId("st-impl-test".to_string()),
            name: ImplementationSpecName("Impl Test".to_string()),
            r#type: None,
            layer: Layer::Implementation,
            links: vec![ImplementationLink::Abstract(AbstractAnnotationId(
                "st-abs-target".to_string(),
            ))],
            artifact: ImplementationArtifact("src/foo.rs".to_string()),
            status: Some(ImplementationStatus::Completed),
            line: LineNumber(10),
        };
        let code = CodeAnnotation {
            source_file: "src/foo.rs".to_string(),
            metas: vec![],
            abstracts: vec![],
            details: vec![],
            implementations: vec![impl_annotation],
        };
        let view_model = ShowResponseView {
            document_annotations: vec![],
            code_annotations: vec![code],
            traces: vec![],
            warnings: vec![],
            view: ShowView::Detail,
            format: ShowFormat::Json,
        };
        let json_val = json::render_detail(&view_model);
        let impls = &json_val["code_annotations"][0]["implementations"][0];
        assert_eq!(impls["artifact"], "src/foo.rs");
        assert_eq!(impls["status"], "Completed");
        assert_eq!(impls["links"][0]["id"], "st-abs-target");
        assert_eq!(impls["links"][0]["layer"], "Abstract");
    }

    #[test]
    fn test_render_text_detail_implementation_artifact() {
        use spec_trail::domains::models::abstract_annotation::AbstractAnnotationId;
        use spec_trail::domains::models::implementation::{
            ImplementationAnnotation, ImplementationAnnotationId, ImplementationArtifact,
            ImplementationLink, ImplementationSpecName,
        };

        let impl_annotation = ImplementationAnnotation {
            id: ImplementationAnnotationId("st-impl-text".to_string()),
            name: ImplementationSpecName("Impl Text".to_string()),
            r#type: None,
            layer: Layer::Implementation,
            links: vec![ImplementationLink::Abstract(AbstractAnnotationId(
                "st-abs-target".to_string(),
            ))],
            artifact: ImplementationArtifact("src/bar.rs".to_string()),
            status: None,
            line: LineNumber(3),
        };

        let line = text::format_implementation_line(&impl_annotation);
        assert!(
            line.contains("Artifact: src/bar.rs"),
            "expected 'Artifact: src/bar.rs' in formatted line, got: {line:?}"
        );
        assert!(
            line.contains("Links: [@st-abs-target (Abstract)]"),
            "expected formatted links in line, got: {line:?}"
        );
    }

    #[test]
    fn test_render_detail_and_summary_include_traces() {
        use spec_trail::domains::models::abstract_annotation::{
            AbstractAnnotation, AbstractAnnotationId, AbstractLink, AbstractName,
        };
        use spec_trail::domains::models::spec_detail::{
            SpecDetailAnnotation, SpecDetailAnnotationId, SpecDetailName,
        };
        use spec_trail::domains::models::trace::{Trace, TraceDestination, TraceKind, TraceSource};

        let code_abs = AbstractAnnotation {
            id: AbstractAnnotationId("st-code-show".to_string()),
            name: AbstractName("ShowUseCase".to_string()),
            r#type: None,
            layer: Layer::Abstract,
            links: vec![AbstractLink::SpecDetail(SpecDetailAnnotationId(
                "st-doc-show".to_string(),
            ))],
            line: LineNumber(1),
        };
        let doc_detail = SpecDetailAnnotation {
            id: SpecDetailAnnotationId("st-doc-show".to_string()),
            name: SpecDetailName("show-command".to_string()),
            r#type: None,
            layer: Layer::SpecDetail,
            links: vec![],
            line: LineNumber(10),
        };
        let trace = Trace {
            src: TraceSource::Abstract(code_abs),
            dst: TraceDestination::SpecDetail(doc_detail),
            kind: TraceKind::Implements,
        };

        let trace_line = text::format_trace_line(&trace);
        assert_eq!(
            trace_line,
            "  [@st-code-show] (Abstract) --Implements--> [@st-doc-show] (SpecDetail)"
        );

        let view_model = ShowResponseView {
            document_annotations: vec![],
            code_annotations: vec![],
            traces: vec![trace],
            warnings: vec![],
            view: ShowView::Detail,
            format: ShowFormat::Json,
        };

        let summary_json = json::render_summary(&view_model);
        assert_eq!(summary_json["traces"], 1);

        let detail_json = json::render_detail(&view_model);
        assert_eq!(detail_json["traces"].as_array().unwrap().len(), 1);
        assert_eq!(detail_json["traces"][0]["src_id"], "st-code-show");
        assert_eq!(detail_json["traces"][0]["dst_id"], "st-doc-show");
        assert_eq!(detail_json["traces"][0]["kind"], "Implements");
    }
}
