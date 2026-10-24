use super::dto::{CheckIssue, CheckSeverity, CheckUseCaseRequestDto, CheckUseCaseResponseDto};
use crate::config::SpecTrailConfig;
use crate::domains::services::annotation::scanner::AnnotationScanner;
use std::path::Path;

fn scan_warning_to_issue(
    warning: crate::domains::services::annotation::scanner::ScanWarning,
) -> CheckIssue {
    match warning {
        crate::domains::services::annotation::scanner::ScanWarning::Parse(pw) => CheckIssue {
            severity: CheckSeverity::Error,
            issue_type: "invalid_annotation_format".to_string(),
            file: pw.source_file,
            line: pw.line.as_usize(),
            spec_id: None,
            message: pw.message,
        },
        crate::domains::services::annotation::scanner::ScanWarning::Resolve(rw) => {
            let issue_type = if rw.message.starts_with("Link target") {
                "broken_link"
            } else {
                "orphaned_annotation"
            };
            CheckIssue {
                severity: CheckSeverity::Error,
                issue_type: issue_type.to_string(),
                file: rw.source_file,
                line: rw.line.as_usize(),
                spec_id: Some(rw.source_annotation_id),
                message: rw.message,
            }
        }
    }
}

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
        );

        let issues: Vec<CheckIssue> = scan_result
            .warnings
            .into_iter()
            .map(scan_warning_to_issue)
            .collect();

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
    use crate::domains::models::line_number::LineNumber;
    use crate::domains::services::annotation::parser::ParseWarning;
    use crate::domains::services::annotation::resolver::ResolveWarning;
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
    }

    #[test]
    fn test_broken_link_resolve_warning_mapped_to_broken_link() {
        let warning = ScanWarning::Resolve(ResolveWarning {
            source_annotation_id: "st-src".to_string(),
            source_file: "src/foo.rs".to_string(),
            line: LineNumber::new(12),
            message: "Link target '@st-missing' not found (referenced from @st-src)".to_string(),
        });

        let issue = scan_warning_to_issue(warning);
        assert_eq!(issue.issue_type, "broken_link");
        assert_eq!(issue.severity, CheckSeverity::Error);
        assert_eq!(issue.file, "src/foo.rs");
        assert_eq!(issue.line, 12);
        assert_eq!(issue.spec_id, Some("st-src".to_string()));
        assert_eq!(
            issue.message,
            "Link target '@st-missing' not found (referenced from @st-src)"
        );
    }

    #[test]
    fn test_other_resolve_warning_preserves_orphaned_annotation() {
        let warning = ScanWarning::Resolve(ResolveWarning {
            source_annotation_id: "st-orphan".to_string(),
            source_file: "src/bar.rs".to_string(),
            line: LineNumber::new(42),
            message: "Orphaned annotation without links".to_string(),
        });

        let issue = scan_warning_to_issue(warning);
        assert_eq!(issue.issue_type, "orphaned_annotation");
        assert_eq!(issue.severity, CheckSeverity::Error);
        assert_eq!(issue.file, "src/bar.rs");
        assert_eq!(issue.line, 42);
        assert_eq!(issue.spec_id, Some("st-orphan".to_string()));
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
}
