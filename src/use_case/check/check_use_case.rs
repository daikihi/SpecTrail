use crate::config::SpecTrailConfig;
use crate::domains::services::annotation::scanner::AnnotationScanner;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct CheckUseCaseRequestDto {
    pub strict: bool,
    pub warn_unimplemented: bool,
    pub config_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckSeverity {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub struct CheckIssue {
    pub severity: CheckSeverity,
    pub issue_type: String,
    pub file: String,
    pub line: usize,
    pub spec_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct CheckUseCaseResponseDto {
    pub errors_count: usize,
    pub warnings_count: usize,
    pub issues: Vec<CheckIssue>,
}

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

        let scan_result = AnnotationScanner::scan(
            Path::new(&config.source.head),
            &config.source.extension,
            Path::new(&config.document.head),
            &config.document.extension,
        );

        let mut issues = Vec::new();

        // Convert ScanWarnings into check issues for now
        for warning in scan_result.warnings {
            match warning {
                crate::domains::services::annotation::scanner::ScanWarning::Parse(pw) => {
                    issues.push(CheckIssue {
                        severity: CheckSeverity::Error,
                        issue_type: "invalid_annotation_format".to_string(),
                        file: pw.source_file,
                        line: pw.line,
                        spec_id: None,
                        message: pw.message,
                    });
                }
                crate::domains::services::annotation::scanner::ScanWarning::Resolve(rw) => {
                    issues.push(CheckIssue {
                        severity: CheckSeverity::Error,
                        issue_type: "orphaned_annotation".to_string(),
                        file: rw.source_annotation_id.clone(),
                        line: 0,
                        spec_id: Some(rw.source_annotation_id),
                        message: rw.message,
                    });
                }
            }
        }

        let errors_count = issues.iter().filter(|i| i.severity == CheckSeverity::Error).count();
        let warnings_count = issues.iter().filter(|i| i.severity == CheckSeverity::Warning).count();

        Ok(CheckUseCaseResponseDto {
            errors_count,
            warnings_count,
            issues,
        })
    }
}
