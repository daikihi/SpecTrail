use crate::config::SpecTrailConfig;
/// [@st-code-use-case-show-show-use-case-file] layer: abstract, type: File, name: show_use_case.rs, links: [@st-manual-spec-cli-show-command]
use crate::domains::models::annotation::code_annotation::CodeAnnotation;
use crate::domains::models::annotation::document_annotation::DocumentAnnotation;
use crate::domains::models::trace::Trace;
use crate::domains::services::annotation::scanner::{AnnotationScanner, ScanWarning};
use crate::domains::services::annotation::trace_resolver::TraceResolver;
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
    pub traces: Vec<Trace>,
    pub warnings: Vec<ScanWarning>,
}

/// [@st-code-use-case-show-show-use-case] layer: abstract, type: Structure, name: ShowUseCase, links: [@st-manual-spec-cli-show-command, @st-manual-usecase-show-config-split]
pub struct ShowUseCase;

#[cfg(test)]
mod tests {
    use super::*;

    fn make_request(mode: &str, target: &str, scope: Option<&str>) -> ShowUseCaseRequestDto {
        ShowUseCaseRequestDto {
            mode: mode.to_string(),
            target: target.to_string(),
            scope: scope.map(|s| s.to_string()),
            config_path: None,
        }
    }

    #[test]
    fn execute_succeeds_with_list_mode_and_all_target() {
        let uc = ShowUseCase::new();
        let result = uc.execute(make_request("list", "all", None));
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.code_annotations.is_empty());
        assert!(!response.document_annotations.is_empty());
        assert!(
            response
                .code_annotations
                .iter()
                .all(|a| a.source_file.starts_with("src/") && a.source_file.ends_with(".rs"))
        );
        assert!(
            response
                .document_annotations
                .iter()
                .all(|a| a.source_file.starts_with("specify_manual/")
                    && a.source_file.ends_with(".md"))
        );
        assert!(
            !response.traces.is_empty(),
            "expected at least one resolved trace between code and document annotations"
        );
        // Warnings collection should exist (may be empty or non-empty depending on scan)
        let _ = response.warnings;
    }

    #[test]
    fn execute_with_code_target_returns_only_rust_source_annotations() {
        let uc = ShowUseCase::new();
        let response = uc
            .execute(make_request("list", "code", None))
            .expect("code target scan should succeed");

        assert!(!response.code_annotations.is_empty());
        assert!(response.document_annotations.is_empty());
        assert!(
            !response.traces.is_empty(),
            "expected cross-space traces to be resolved even when target is code"
        );
        assert!(
            response.warnings.is_empty(),
            "expected no false broken-link warnings when target is code, got: {:?}",
            response.warnings
        );
        assert!(
            response
                .code_annotations
                .iter()
                .all(|a| a.source_file.starts_with("src/") && a.source_file.ends_with(".rs"))
        );
    }

    #[test]
    fn execute_with_document_target_returns_only_markdown_document_annotations() {
        let uc = ShowUseCase::new();
        let response = uc
            .execute(make_request("list", "document", None))
            .expect("document target scan should succeed");

        assert!(response.code_annotations.is_empty());
        assert!(!response.document_annotations.is_empty());
        assert!(
            !response.traces.is_empty(),
            "expected cross-space traces to be resolved even when target is document"
        );
        assert!(
            response.warnings.is_empty(),
            "expected no warnings when target is document, got: {:?}",
            response.warnings
        );
        assert!(
            response
                .document_annotations
                .iter()
                .all(|a| a.source_file.starts_with("specify_manual/")
                    && a.source_file.ends_with(".md"))
        );
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
    /// use spec_trail::use_case::show::show_use_case::{ShowUseCase, ShowUseCaseRequestDto};
    /// let uc = ShowUseCase::new();
    /// let req = ShowUseCaseRequestDto {
    ///     mode: "search".into(),
    ///     target: "all".into(),
    ///     scope: None,
    ///     config_path: None,
    /// };
    /// assert!(uc.execute(req).is_err()); // "search" mode is not implemented
    /// ```
    pub fn execute(
        &self,
        request: ShowUseCaseRequestDto,
    ) -> Result<ShowUseCaseResponseDto, Box<dyn std::error::Error>> {
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

        let ignored_prefixes = config.scanner.effective_ignored_prefixes();
        let scan_result = AnnotationScanner::scan_with_ignored_prefixes(
            Path::new(&config.source.head),
            &config.source.extension,
            Path::new(&config.document.head),
            &config.document.extension,
            &ignored_prefixes,
        )?;

        let traces = TraceResolver::resolve(
            &scan_result.document_annotations,
            &scan_result.code_annotations,
        );

        let include_code = request.target == "all" || request.target == "code";
        let include_document = request.target == "all" || request.target == "document";

        let code_annotations = if include_code {
            scan_result.code_annotations
        } else {
            Vec::new()
        };
        let document_annotations = if include_document {
            scan_result.document_annotations
        } else {
            Vec::new()
        };
        let warnings = if include_code && include_document {
            scan_result.warnings
        } else {
            scan_result
                .warnings
                .into_iter()
                .filter(|warning| {
                    let source_file = match warning {
                        ScanWarning::Parse(pw) => pw.source_file.as_str(),
                        ScanWarning::Resolve(rw) => rw.source_file.as_str(),
                    };
                    (include_code && source_file.starts_with(&config.source.head))
                        || (include_document && source_file.starts_with(&config.document.head))
                })
                .collect()
        };

        Ok(ShowUseCaseResponseDto {
            document_annotations,
            code_annotations,
            traces,
            warnings,
        })
    }
}
