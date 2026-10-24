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
