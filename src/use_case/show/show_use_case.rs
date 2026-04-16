use crate::config::SpecTrailConfig;
/// [@st-code-use-case-show-show-use-case-file] layer: abstract, type: File, name: show_use_case.rs
use crate::domains::models::annotation::code_annotation::CodeAnnotation;
use crate::domains::models::annotation::document_annotation::DocumentAnnotation;
use crate::domains::services::annotation::scanner::{AnnotationScanner, ScanWarning};
use log::info;
use std::path::Path;

/// [@st-code-use-case-show-show-use-case-request-dto] layer: abstract, type: Structure, name: ShowUseCaseRequestDto
#[derive(Debug)]
pub struct ShowUseCaseRequestDto {
    pub mode: String,
    pub target: String,
    pub scope: Option<String>,
    /// [@st-code-use-case-show-show-use-case-request-dto-config-path] layer: abstract, type: Structure, name: config_path
    pub config_path: Option<String>,
}

/// [@st-code-use-case-show-show-use-case-response-dto] layer: abstract, type: Structure, name: ShowUseCaseResponseDto
#[derive(Debug)]
pub struct ShowUseCaseResponseDto {
    pub document_annotations: Vec<DocumentAnnotation>,
    pub code_annotations: Vec<CodeAnnotation>,
    pub warnings: Vec<ScanWarning>,
}

/// [@st-code-use-case-show-show-use-case] layer: abstract, type: Structure, name: ShowUseCase
pub struct ShowUseCase;

#[cfg(test)]
mod tests {
    use super::*;

    fn make_request(mode: &str, target: &str, scope: Option<&str>) -> ShowUseCaseRequestDto {
        ShowUseCaseRequestDto {
            mode: mode.to_string(),
            target: target.to_string(),
            scope: scope.map(|s| s.to_string()),
        }
    }

    #[test]
    fn execute_fails_for_search_mode() {
        let uc = ShowUseCase::new();
        let result = uc.execute(make_request("search", "all", None));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "show --mode search is not implemented yet"
        );
    }

    #[test]
    fn execute_fails_for_group_target() {
        let uc = ShowUseCase::new();
        let result = uc.execute(make_request("list", "group", None));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "show --target group is not implemented yet"
        );
    }

    #[test]
    fn execute_fails_when_scope_is_present_with_non_search_mode() {
        let uc = ShowUseCase::new();
        let result = uc.execute(make_request("list", "all", Some("@st-foo")));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "--scope is only supported with --mode search"
        );
    }

    #[test]
    fn execute_search_mode_is_checked_before_scope_validation() {
        // When mode=search AND scope is given, the search-mode error should fire first
        let uc = ShowUseCase::new();
        let result = uc.execute(make_request("search", "all", Some("@st-foo")));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "show --mode search is not implemented yet"
        );
    }

    #[test]
    fn execute_succeeds_with_list_mode_and_code_target() {
        let uc = ShowUseCase::new();
        let result = uc.execute(make_request("list", "code", None));
        assert!(result.is_ok());
    }

    #[test]
    fn execute_succeeds_with_list_mode_and_document_target() {
        let uc = ShowUseCase::new();
        let result = uc.execute(make_request("list", "document", None));
        assert!(result.is_ok());
    }

    #[test]
    fn execute_succeeds_with_list_mode_and_all_target() {
        let uc = ShowUseCase::new();
        let result = uc.execute(make_request("list", "all", None));
        assert!(result.is_ok());
        let response = result.unwrap();
        // Warnings collection should exist (may be empty or non-empty depending on scan)
        let _ = response.warnings;
    }
}

impl ShowUseCase {
    pub fn new() -> Self {
        Self
    }

    /// Executes the show use case: scans configured code and document paths according to the request
    /// and returns found annotations and any scan warnings.
    ///
    /// This validates the request options and returns an error for unsupported combinations:
    /// - mode == "search" -> error "show --mode search is not implemented yet"
    /// - target == "group" -> error "show --target group is not implemented yet"
    /// - scope is Some -> error "--scope is only supported with --mode search"
    ///
    /// On success, loads configuration from "src/config/config.toml", selects input paths based on
    /// `request.target`, runs the annotation scanner, reports the results via `report_annotations`,
    /// and returns a `ShowUseCaseResponseDto` containing document annotations, code annotations, and warnings.
    ///
    /// # Examples
    ///
    /// ```
    /// let uc = ShowUseCase::new();
    /// let req = ShowUseCaseRequestDto { mode: "search".into(), target: "all".into(), scope: None };
    /// assert!(uc.execute(req).is_err()); // "search" mode is not implemented
    /// ```
    pub fn execute(
        &self,
        request: ShowUseCaseRequestDto,
    ) -> Result<ShowUseCaseResponseDto, Box<dyn std::error::Error>> {
        info!("Executing ShowUseCase with request: {:?}", request);

        if request.mode == "search" {
            return Err("show --mode search is not implemented yet".into());
        }

        if request.target == "group" {
            return Err("show --target group is not implemented yet".into());
        }

        if request.scope.is_some() {
            return Err("--scope is only supported with --mode search".into());
        }

        let config_path = request
            .config_path
            .unwrap_or_else(|| String::from("src/config/default.toml"));
        let config = SpecTrailConfig::from_file(config_path)?;

        let code_path = if request.target == "all" || request.target == "code" {
            Path::new(&config.source.head)
        } else {
            Path::new("")
        };

        let doc_path = if request.target == "all" || request.target == "document" {
            Path::new(&config.document.head)
        } else {
            Path::new("")
        };

        let scan_result = AnnotationScanner::scan(
            code_path,
            &config.document.extension,
            doc_path,
            &config.source.extension,
        );

        let code_annotations = scan_result.code_annotations;
        let document_annotations = scan_result.document_annotations;
        let warnings = scan_result.warnings;

        // Reporting
        self.report_annotations(&code_annotations, &document_annotations);

        Ok(ShowUseCaseResponseDto {
            document_annotations,
            code_annotations,
            warnings,
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