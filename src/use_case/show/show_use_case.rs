/// [@st-code-use-case-show-show-use-case-file] layer: abstract, type: File, name: show_use_case.rs
use crate::domains::models::annotation::code_annotation::CodeAnnotation;
use crate::domains::models::annotation::document_annotation::DocumentAnnotation;
use crate::domains::services::annotation::scanner::AnnotationScanner;
use log::info;

/// [@st-code-use-case-show-show-use-case-request-dto] layer: abstract, type: Structure, name: ShowUseCaseRequestDto
#[derive(Debug)]
pub struct ShowUseCaseRequestDto {
    pub mode: String,
    pub target: String,
    pub scope: Option<String>,
}

/// [@st-code-use-case-show-show-use-case-response-dto] layer: abstract, type: Structure, name: ShowUseCaseResponseDto
#[derive(Debug)]
pub struct ShowUseCaseResponseDto {
    pub document_annotations: Vec<DocumentAnnotation>,
    pub code_annotations: Vec<CodeAnnotation>,
}

/// [@st-code-use-case-show-show-use-case] layer: abstract, type: Structure, name: ShowUseCase
pub struct ShowUseCase;

impl ShowUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(
        &self,
        request: ShowUseCaseRequestDto,
    ) -> Result<ShowUseCaseResponseDto, Box<dyn std::error::Error>> {
        info!("Executing ShowUseCase with request: {:?}", request);
 
        let mut code_annotations: Vec<CodeAnnotation> = Vec::new();
        let mut document_annotations: Vec<DocumentAnnotation> = Vec::new();
 
        // 1. Get annotations from code (scan under src/)
        if request.target == "all" || request.target == "code" {
            code_annotations = AnnotationScanner::scan_code("src");
        }
 
        // 2. Get annotations from documents (scan under specify_manual/)
        if request.target == "all" || request.target == "document" {
            document_annotations = AnnotationScanner::scan_documents("specify_manual");
        }
 
        // Reporting
        self.report_annotations(&code_annotations, &document_annotations);

        Ok(ShowUseCaseResponseDto {
            document_annotations,
            code_annotations,
        })
    }

    fn report_annotations(
        &self,
        code_annotations: &[CodeAnnotation],
        document_annotations: &[DocumentAnnotation],
    ) {
        info!("--- Annotation Report ---");
        info!("Total Code Annotations: {}", code_annotations.len());
        for (i, anno) in code_annotations.iter().enumerate() {
            info!("  Code Anno [{}]:", i);
            if !anno.metas.is_empty() {
                for meta in &anno.metas {
                    let link_ids: Vec<String> = meta.links.iter().map(|l| l.id.0.clone()).collect();
                    info!(
                        "    [Meta] id: {}, name: {}, type: {:?}, links: {:?}",
                        meta.id.0, meta.name.0, meta.r#type, link_ids
                    );
                }
            }
            if !anno.abstracts.is_empty() {
                for abs in &anno.abstracts {
                    let link_ids: Vec<String> = abs.links.iter().map(|l| l.id.0.clone()).collect();
                    info!(
                        "    [Abstract] id: {}, name: {}, type: {:?}, links: {:?}",
                        abs.id.0, abs.name.0, abs.r#type, link_ids
                    );
                }
            }
            if !anno.details.is_empty() {
                for detail in &anno.details {
                    let link_ids: Vec<String> = detail
                        .links
                        .iter()
                        .map(|l| match l {
                            crate::domains::models::spec_detail::SpecDetailLink::Abstract(a) => {
                                a.id.0.clone()
                            }
                            crate::domains::models::spec_detail::SpecDetailLink::Implementation(
                                i,
                            ) => i.id.0.clone(),
                        })
                        .collect();
                    info!(
                        "    [Detail] id: {}, name: {}, type: {:?}, links: {:?}",
                        detail.id.0, detail.name.0, detail.r#type, link_ids
                    );
                }
            }
            if !anno.implementations.is_empty() {
                for impl_anno in &anno.implementations {
                    let link_ids: Vec<String> = impl_anno.links.iter().map(|l| match l {
                        crate::domains::models::implementation::ImplementationLink::Abstract(a) => a.id.0.clone(),
                        crate::domains::models::implementation::ImplementationLink::SpecDetail(s) => s.id.0.clone(),
                    }).collect();
                    info!(
                        "    [Implementation] id: {}, name: {}, type: {:?}, artifact: {}, links: {:?}",
                        impl_anno.id.0,
                        impl_anno.name.0,
                        impl_anno.r#type,
                        impl_anno.artifact.0,
                        link_ids
                    );
                }
            }
        }

        info!("Total Document Annotations: {}", document_annotations.len());
        for (i, anno) in document_annotations.iter().enumerate() {
            info!("  Document Anno [{}]:", i);
            if !anno.metas.is_empty() {
                for meta in &anno.metas {
                    let link_ids: Vec<String> = meta.links.iter().map(|l| l.id.0.clone()).collect();
                    info!(
                        "    [Meta] id: {}, name: {}, type: {:?}, links: {:?}",
                        meta.id.0, meta.name.0, meta.r#type, link_ids
                    );
                }
            }
            if !anno.abstracts.is_empty() {
                for abs in &anno.abstracts {
                    let link_ids: Vec<String> = abs.links.iter().map(|l| l.id.0.clone()).collect();
                    info!(
                        "    [Abstract] id: {}, name: {}, type: {:?}, links: {:?}",
                        abs.id.0, abs.name.0, abs.r#type, link_ids
                    );
                }
            }
            if !anno.details.is_empty() {
                for detail in &anno.details {
                    let link_ids: Vec<String> = detail
                        .links
                        .iter()
                        .map(|l| match l {
                            crate::domains::models::spec_detail::SpecDetailLink::Abstract(a) => {
                                a.id.0.clone()
                            }
                            crate::domains::models::spec_detail::SpecDetailLink::Implementation(
                                i,
                            ) => i.id.0.clone(),
                        })
                        .collect();
                    info!(
                        "    [Detail] id: {}, name: {}, type: {:?}, links: {:?}",
                        detail.id.0, detail.name.0, detail.r#type, link_ids
                    );
                }
            }
            if !anno.implementations.is_empty() {
                for impl_anno in &anno.implementations {
                    let link_ids: Vec<String> = impl_anno.links.iter().map(|l| match l {
                        crate::domains::models::implementation::ImplementationLink::Abstract(a) => a.id.0.clone(),
                        crate::domains::models::implementation::ImplementationLink::SpecDetail(s) => s.id.0.clone(),
                    }).collect();
                    info!(
                        "    [Implementation] id: {}, name: {}, type: {:?}, artifact: {}, links: {:?}",
                        impl_anno.id.0,
                        impl_anno.name.0,
                        impl_anno.r#type,
                        impl_anno.artifact.0,
                        link_ids
                    );
                }
            }
        }
        info!("-------------------------");
    }
}
