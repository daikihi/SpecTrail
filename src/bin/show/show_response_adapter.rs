use SpecTrail::domains::models::annotation::code_annotation::CodeAnnotation;
use SpecTrail::domains::models::annotation::document_annotation::DocumentAnnotation;
use SpecTrail::domains::services::annotation::scanner::ScanWarning;
use SpecTrail::use_case::show::show_use_case::ShowUseCaseResponseDto;

#[derive(Debug)]
pub struct ShowResponseView {
    pub document_annotations: Vec<DocumentAnnotation>,
    pub code_annotations: Vec<CodeAnnotation>,
    pub warnings: Vec<ScanWarning>,
    pub view: crate::dto::ShowView,
    pub format: crate::dto::ShowFormat,
}

pub fn adapt_response(
    response: &ShowUseCaseResponseDto,
    view: crate::dto::ShowView,
    format: crate::dto::ShowFormat,
) -> ShowResponseView {
    ShowResponseView {
        document_annotations: response.document_annotations.clone(),
        code_annotations: response.code_annotations.clone(),
        warnings: response.warnings.clone(),
        view,
        format,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use SpecTrail::domains::models::abstract_annotation::{
        AbstractAnnotation, AbstractAnnotationId, AbstractName,
    };
    use SpecTrail::domains::models::implementation::{
        ImplementationAnnotation, ImplementationArtifact, ImplementationLink,
        ImplementationSpecName,
    };
    use SpecTrail::domains::models::layer::Layer;
    use SpecTrail::domains::models::meta::{MetaAnnotation, MetaAnnotationId, MetaName};
    use SpecTrail::domains::models::spec_detail::{
        SpecDetailAnnotation, SpecDetailAnnotationId, SpecDetailLink, SpecDetailName,
    };
    use SpecTrail::domains::services::annotation::parser::ParseWarning;
    use SpecTrail::domains::services::annotation::resolver::ResolveWarning;

    fn make_meta() -> MetaAnnotation {
        MetaAnnotation {
            id: MetaAnnotationId("m".to_string()),
            name: MetaName("Meta".to_string()),
            r#type: None,
            layer: Layer::Meta,
            links: vec![],
        }
    }

    fn make_abstract() -> AbstractAnnotation {
        AbstractAnnotation {
            id: AbstractAnnotationId("a".to_string()),
            name: AbstractName("Abstract".to_string()),
            r#type: None,
            layer: Layer::Abstract,
            links: vec![],
        }
    }

    fn make_detail() -> SpecDetailAnnotation {
        SpecDetailAnnotation {
            id: SpecDetailAnnotationId("d".to_string()),
            name: SpecDetailName("Detail".to_string()),
            r#type: None,
            layer: Layer::SpecDetail,
            links: vec![SpecDetailLink::Abstract(Box::new(make_abstract()))],
        }
    }

    fn make_implementation() -> ImplementationAnnotation {
        ImplementationAnnotation {
            id: SpecTrail::domains::models::implementation::ImplementationAnnotationId(
                "i".to_string(),
            ),
            name: ImplementationSpecName("Impl".to_string()),
            r#type: None,
            layer: Layer::Implementation,
            links: vec![ImplementationLink::Abstract(Box::new(make_abstract()))],
            artifact: ImplementationArtifact("artifact".to_string()),
            status: None,
        }
    }

    #[test]
    fn adapts_response_to_view_model() {
        let response = ShowUseCaseResponseDto {
            document_annotations: vec![DocumentAnnotation {
                source_file: "docs/a.md".to_string(),
                metas: vec![make_meta()],
                abstracts: vec![make_abstract()],
                details: vec![make_detail()],
                implementations: vec![make_implementation()],
            }],
            code_annotations: vec![CodeAnnotation {
                source_file: "src/a.rs".to_string(),
                metas: vec![make_meta()],
                abstracts: vec![make_abstract()],
                details: vec![make_detail()],
                implementations: vec![make_implementation()],
            }],
            warnings: vec![],
        };

        let view = adapt_response(
            &response,
            crate::dto::ShowView::List,
            crate::dto::ShowFormat::Text,
        );

        assert_eq!(view.document_annotations.len(), 1);
        assert_eq!(view.code_annotations.len(), 1);
        assert_eq!(view.view, crate::dto::ShowView::List);
        assert_eq!(view.format, crate::dto::ShowFormat::Text);
        assert!(view.warnings.is_empty());
    }

    #[test]
    fn preserves_warnings_in_view_model() {
        let response = ShowUseCaseResponseDto {
            document_annotations: vec![],
            code_annotations: vec![],
            warnings: vec![ScanWarning::Parse(ParseWarning {
                source_file: "src/main.rs".to_string(),
                line: 12,
                message: "broken".to_string(),
                raw_text: "@bad".to_string(),
            })],
        };

        let view = adapt_response(
            &response,
            crate::dto::ShowView::Summary,
            crate::dto::ShowFormat::Json,
        );

        assert_eq!(view.warnings.len(), 1);
        assert_eq!(view.view, crate::dto::ShowView::Summary);
        assert_eq!(view.format, crate::dto::ShowFormat::Json);
    }
}
