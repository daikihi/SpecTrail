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
    t.map(|v| format!("{:?}", v)).unwrap_or_else(|| "unknown".to_string())
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
            warnings: vec![],
            view: ShowView::Detail,
            format: ShowFormat::Json,
        };
        let json_val = json::render_detail(&view_model);
        assert!(json_val.get("document_annotations").is_some());
        assert!(json_val.get("code_annotations").is_some());
    }
}
