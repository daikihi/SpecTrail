mod dto;

use dto::{CheckFormat, CheckRequestDto};
use serde_json::json;
use spec_trail::use_case::check::{
    CheckIssue, CheckSeverity, CheckUseCase, CheckUseCaseRequestDto,
};
use std::env;
use std::process;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    let request_dto = match CheckRequestDto::from_args(&args) {
        Ok(dto) => dto,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    let use_case = CheckUseCase::new();
    let response = use_case.execute(CheckUseCaseRequestDto {
        strict: request_dto.strict,
        warn_unimplemented: request_dto.warn_unimplemented,
        config_path: request_dto.config_path,
    });

    match response {
        Ok(res) => {
            render_output(
                &res.issues,
                res.errors_count,
                res.warnings_count,
                request_dto.format,
            );

            if request_dto.strict && res.errors_count > 0 {
                process::exit(1);
            } else {
                process::exit(0);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}

fn render_output(
    issues: &[CheckIssue],
    errors_count: usize,
    warnings_count: usize,
    format: CheckFormat,
) {
    match format {
        CheckFormat::Text => {
            for issue in issues {
                let tag = match issue.severity {
                    CheckSeverity::Error => "[ERROR]",
                    CheckSeverity::Warning => "[WARN] ",
                };
                if issue.line > 0 {
                    println!("{} {}:{} - {}", tag, issue.file, issue.line, issue.message);
                } else {
                    println!("{} {} - {}", tag, issue.file, issue.message);
                }
            }
            let err_label = if errors_count == 1 { "error" } else { "errors" };
            let warn_label = if warnings_count == 1 {
                "warning"
            } else {
                "warnings"
            };
            println!(
                "Found {} {}, {} {}.",
                errors_count, err_label, warnings_count, warn_label
            );
        }
        CheckFormat::Github => {
            for issue in issues {
                println!("{}", format_github_issue(issue));
            }
        }
        CheckFormat::Json => {
            let issues_json: Vec<_> = issues
                .iter()
                .map(|i| {
                    let sev = match i.severity {
                        CheckSeverity::Error => "error",
                        CheckSeverity::Warning => "warning",
                    };
                    json!({
                        "severity": sev,
                        "type": i.issue_type,
                        "file": i.file,
                        "line": i.line,
                        "spec_id": i.spec_id,
                        "message": i.message,
                    })
                })
                .collect();

            let output = json!({
                "summary": {
                    "errors": errors_count,
                    "warnings": warnings_count,
                },
                "issues": issues_json,
            });

            if let Ok(serialized) = serde_json::to_string_pretty(&output) {
                println!("{}", serialized);
            }
        }
    }
}

fn escape_github_property(s: &str) -> String {
    s.replace('%', "%25")
        .replace('\r', "%0D")
        .replace('\n', "%0A")
        .replace(':', "%3A")
        .replace(',', "%2C")
}

fn escape_github_data(s: &str) -> String {
    s.replace('%', "%25")
        .replace('\r', "%0D")
        .replace('\n', "%0A")
}

fn format_github_issue(issue: &CheckIssue) -> String {
    let cmd = match issue.severity {
        CheckSeverity::Error => "error",
        CheckSeverity::Warning => "warning",
    };
    let file = escape_github_property(&issue.file);
    let message = escape_github_data(&issue.message);
    if issue.line > 0 {
        format!("::{} file={},line={}::{}", cmd, file, issue.line, message)
    } else {
        format!("::{} file={}::{}", cmd, file, message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_github_property() {
        assert_eq!(
            escape_github_property("file:name,with%percent\r\n"),
            "file%3Aname%2Cwith%25percent%0D%0A"
        );
    }

    #[test]
    fn test_escape_github_data() {
        assert_eq!(
            escape_github_data("Line 1:\nLine 2, with 100% certainty\r\n"),
            "Line 1:%0ALine 2, with 100%25 certainty%0D%0A"
        );
    }

    #[test]
    fn test_format_github_issue_with_line() {
        let issue = CheckIssue {
            severity: CheckSeverity::Error,
            issue_type: "broken_link".to_string(),
            file: "src/foo:bar,test.rs".to_string(),
            line: 42,
            spec_id: Some("st-id".to_string()),
            message: "Link target '%target%' not found:\nline 2".to_string(),
        };
        assert_eq!(
            format_github_issue(&issue),
            "::error file=src/foo%3Abar%2Ctest.rs,line=42::Link target '%25target%25' not found:%0Aline 2"
        );
    }

    #[test]
    fn test_format_github_issue_without_line() {
        let issue = CheckIssue {
            severity: CheckSeverity::Warning,
            issue_type: "orphaned_annotation".to_string(),
            file: "src/orphan.rs".to_string(),
            line: 0,
            spec_id: None,
            message: "Orphaned annotation".to_string(),
        };
        assert_eq!(
            format_github_issue(&issue),
            "::warning file=src/orphan.rs::Orphaned annotation"
        );
    }
}
