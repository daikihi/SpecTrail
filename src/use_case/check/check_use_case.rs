use super::dto::{CheckIssue, CheckSeverity, CheckUseCaseRequestDto, CheckUseCaseResponseDto};
use crate::config::SpecTrailConfig;
use crate::domains::models::annotation::code_annotation::CodeAnnotation;
use crate::domains::models::annotation::document_annotation::DocumentAnnotation;
use crate::domains::models::trace::TraceDirection;
use crate::domains::services::annotation::scanner::{AnnotationScanner, ScanWarning};
use crate::domains::services::annotation::trace_resolver::TraceResolver;
use std::collections::HashSet;
use std::path::Path;

fn scan_warning_to_issue(warning: ScanWarning) -> CheckIssue {
    match warning {
        ScanWarning::Parse(pw) => CheckIssue {
            severity: CheckSeverity::Error,
            issue_type: "invalid_annotation_format".to_string(),
            file: pw.source_file,
            line: pw.line.as_usize(),
            spec_id: None,
            message: pw.message,
        },
        ScanWarning::Resolve(rw) => CheckIssue {
            severity: CheckSeverity::Error,
            issue_type: rw.kind.as_str().to_string(),
            file: rw.source_file,
            line: rw.line.as_usize(),
            spec_id: Some(rw.source_annotation_id),
            message: rw.message,
        },
    }
}

fn find_unimplemented_spec_issues(
    documents: &[DocumentAnnotation],
    codes: &[CodeAnnotation],
) -> Vec<CheckIssue> {
    let traces = TraceResolver::resolve(documents, codes);

    let traced_documents: HashSet<_> = traces
        .iter()
        .map(|trace| match trace.direction {
            TraceDirection::CodeToDocument => (
                trace.dst_file.as_str(),
                trace.dst.line(),
                trace.dst.layer(),
                trace.dst.id(),
            ),
            TraceDirection::DocumentToCode => (
                trace.src_file.as_str(),
                trace.src.line(),
                trace.src.layer(),
                trace.src.id(),
            ),
        })
        .collect();

    let mut issues = Vec::new();
    for doc in documents {
        let specs = doc
            .abstracts
            .iter()
            .map(|a| (&a.id.0, &a.name.0, a.layer, a.line))
            .chain(
                doc.details
                    .iter()
                    .map(|d| (&d.id.0, &d.name.0, d.layer, d.line)),
            )
            .chain(
                doc.implementations
                    .iter()
                    .map(|i| (&i.id.0, &i.name.0, i.layer, i.line)),
            );
        for (id, name, layer, line) in specs {
            if !traced_documents.contains(&(doc.source_file.as_str(), line, layer, id.as_str())) {
                issues.push(CheckIssue {
                    severity: CheckSeverity::Warning,
                    issue_type: "unimplemented_spec".to_string(),
                    file: doc.source_file.clone(),
                    line: line.as_usize(),
                    spec_id: Some(id.clone()),
                    message: format!(
                        "Unimplemented Spec: '@{id}' ({name}) is not referenced in code"
                    ),
                });
            }
        }
    }

    issues
}

fn severity_rank(severity: &CheckSeverity) -> u8 {
    match severity {
        CheckSeverity::Error => 0,
        CheckSeverity::Warning => 1,
    }
}

/// [@st-code-use-case-check-check-use-case] layer: abstract, type: Structure, name: CheckUseCase, links: [@st-manual-spec-cli-check-command]
#[derive(Default)]
pub struct CheckUseCase;

impl CheckUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(
        &self,
        request: CheckUseCaseRequestDto,
    ) -> Result<CheckUseCaseResponseDto, Box<dyn std::error::Error>> {
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

        let mut issues: Vec<CheckIssue> = scan_result
            .warnings
            .into_iter()
            .map(scan_warning_to_issue)
            .collect();

        if request.warn_unimplemented {
            issues.extend(find_unimplemented_spec_issues(
                &scan_result.document_annotations,
                &scan_result.code_annotations,
            ));
        }

        issues.sort_by(|a, b| {
            severity_rank(&a.severity)
                .cmp(&severity_rank(&b.severity))
                .then_with(|| a.file.cmp(&b.file))
                .then_with(|| a.line.cmp(&b.line))
                .then_with(|| a.issue_type.cmp(&b.issue_type))
                .then_with(|| a.spec_id.cmp(&b.spec_id))
        });

        let errors_count = issues
            .iter()
            .filter(|i| i.severity == CheckSeverity::Error)
            .count();
        let warnings_count = issues
            .iter()
            .filter(|i| i.severity == CheckSeverity::Warning)
            .count();

        Ok(CheckUseCaseResponseDto {
            errors_count,
            warnings_count,
            issues,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domains::models::abstract_annotation::{
        AbstractAnnotation, AbstractAnnotationId, AbstractLink, AbstractName,
    };
    use crate::domains::models::layer::Layer;
    use crate::domains::models::line_number::LineNumber;
    use crate::domains::models::meta::{MetaAnnotation, MetaAnnotationId, MetaName};
    use crate::domains::models::spec_detail::{
        SpecDetailAnnotation, SpecDetailAnnotationId, SpecDetailName,
    };
    use crate::domains::services::annotation::parser::ParseWarning;
    use crate::domains::services::annotation::resolver::{ResolveWarning, ResolveWarningKind};
    use crate::domains::services::annotation::scanner::ScanWarning;

    #[test]
    fn test_execute_succeeds_with_default_config() {
        let uc = CheckUseCase::new();
        let result = uc.execute(CheckUseCaseRequestDto {
            strict: false,
            warn_unimplemented: false,
            config_path: None,
        });
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(
            response.errors_count, 0,
            "expected no check errors in repository, got: {:?}",
            response.issues
        );
        assert_eq!(
            response.warnings_count, 0,
            "expected no check warnings in repository, got: {:?}",
            response.issues
        );
        assert!(response.issues.is_empty());
    }

    #[test]
    fn test_execute_with_warn_unimplemented_reports_only_warnings() {
        let uc = CheckUseCase::new();
        let response = uc
            .execute(CheckUseCaseRequestDto {
                strict: false,
                warn_unimplemented: true,
                config_path: None,
            })
            .expect("check with --warn-unimplemented should succeed");

        assert_eq!(response.errors_count, 0);
        assert!(response.warnings_count > 0);
        assert_eq!(response.warnings_count, response.issues.len());
        assert!(response.issues.iter().all(|issue| {
            issue.severity == CheckSeverity::Warning && issue.issue_type == "unimplemented_spec"
        }));
        // Referenced spec IDs must not appear as unimplemented warnings
        assert!(
            !response
                .issues
                .iter()
                .any(|i| i.spec_id.as_deref() == Some("st-manual-spec-cli-show-command"))
        );
    }

    #[test]
    fn test_resolve_warning_kinds_map_to_distinct_issue_types() {
        let cases = [
            (ResolveWarningKind::BrokenLink, "broken_link"),
            (ResolveWarningKind::UnknownLayer, "unknown_layer"),
            (ResolveWarningKind::UnknownType, "unknown_type"),
            (ResolveWarningKind::UnknownStatus, "unknown_status"),
            (
                ResolveWarningKind::UnsupportedLinkLayer,
                "unsupported_link_layer",
            ),
        ];

        for (kind, expected_type) in cases {
            let warning = ScanWarning::Resolve(ResolveWarning {
                kind,
                source_annotation_id: "st-src".to_string(),
                source_file: "src/foo.rs".to_string(),
                line: LineNumber::new(12),
                message: "sample warning".to_string(),
            });

            let issue = scan_warning_to_issue(warning);
            assert_eq!(issue.issue_type, expected_type);
            assert_eq!(issue.severity, CheckSeverity::Error);
            assert_eq!(issue.file, "src/foo.rs");
            assert_eq!(issue.line, 12);
            assert_eq!(issue.spec_id, Some("st-src".to_string()));
        }
    }

    #[test]
    fn test_find_unimplemented_spec_issues_ignores_meta_and_traced_specs() {
        let doc = DocumentAnnotation {
            source_file: "specify_manual/spec.md".to_string(),
            metas: vec![MetaAnnotation {
                id: MetaAnnotationId("st-meta-only".to_string()),
                name: MetaName("Meta Only".to_string()),
                r#type: None,
                layer: Layer::Meta,
                links: vec![],
                line: LineNumber::new(1),
            }],
            abstracts: vec![AbstractAnnotation {
                id: AbstractAnnotationId("st-abs-unimplemented".to_string()),
                name: AbstractName("Unimplemented Abstract".to_string()),
                r#type: None,
                layer: Layer::Abstract,
                links: vec![],
                line: LineNumber::new(10),
            }],
            details: vec![
                SpecDetailAnnotation {
                    id: SpecDetailAnnotationId("st-detail-implemented".to_string()),
                    name: SpecDetailName("Implemented Detail".to_string()),
                    r#type: None,
                    layer: Layer::SpecDetail,
                    links: vec![],
                    line: LineNumber::new(20),
                },
                SpecDetailAnnotation {
                    id: SpecDetailAnnotationId("st-detail-missing".to_string()),
                    name: SpecDetailName("Missing Detail".to_string()),
                    r#type: None,
                    layer: Layer::SpecDetail,
                    links: vec![],
                    line: LineNumber::new(30),
                },
            ],
            implementations: vec![],
        };

        let code = CodeAnnotation {
            source_file: "src/impl.rs".to_string(),
            metas: vec![],
            abstracts: vec![AbstractAnnotation {
                id: AbstractAnnotationId("st-code-impl".to_string()),
                name: AbstractName("Impl".to_string()),
                r#type: None,
                layer: Layer::Abstract,
                links: vec![AbstractLink::SpecDetail(SpecDetailAnnotationId(
                    "st-detail-implemented".to_string(),
                ))],
                line: LineNumber::new(5),
            }],
            details: vec![],
            implementations: vec![],
        };

        let issues = find_unimplemented_spec_issues(&[doc], &[code]);
        assert_eq!(issues.len(), 2);
        let ids: Vec<_> = issues
            .iter()
            .filter_map(|i| i.spec_id.as_deref())
            .collect();
        assert_eq!(ids, vec!["st-abs-unimplemented", "st-detail-missing"]);
        assert!(issues.iter().all(|i| i.severity == CheckSeverity::Warning));
        assert!(issues.iter().all(|i| i.issue_type == "unimplemented_spec"));
    }

    #[test]
    fn test_parse_warning_mapped_to_invalid_annotation_format() {
        let warning = ScanWarning::Parse(ParseWarning {
            source_file: "src/baz.rs".to_string(),
            line: LineNumber::new(5),
            message: "Failed to parse line".to_string(),
            raw_text: "[@bad]".to_string(),
        });

        let issue = scan_warning_to_issue(warning);
        assert_eq!(issue.issue_type, "invalid_annotation_format");
        assert_eq!(issue.severity, CheckSeverity::Error);
        assert_eq!(issue.file, "src/baz.rs");
        assert_eq!(issue.line, 5);
        assert_eq!(issue.spec_id, None);
    }

    #[test]
    fn test_execute_fails_when_configured_path_does_not_exist() {
        let temp_dir = std::env::temp_dir();
        let config_file = temp_dir.join("spectrail_nonexistent_dir_config.toml");
        std::fs::write(
            &config_file,
            r#"
[source]
head = "nonexistent_source_xyz"
extension = ".rs"
[document]
head = "src/"
extension = ".rs"
[annotation]
prefix = "@st-"
"#,
        )
        .unwrap();

        let uc = CheckUseCase::new();
        let result = uc.execute(CheckUseCaseRequestDto {
            strict: false,
            warn_unimplemented: false,
            config_path: Some(config_file.to_str().unwrap().to_string()),
        });

        let _ = std::fs::remove_file(&config_file);

        assert!(result.is_err());
    }
}
