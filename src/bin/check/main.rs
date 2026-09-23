mod dto;

use dto::{CheckFormat, CheckRequestDto};
use SpecTrail::use_case::check::{
    CheckIssue, CheckSeverity, CheckUseCase, CheckUseCaseRequestDto,
};
use serde_json::json;
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
            render_output(&res.issues, res.errors_count, res.warnings_count, request_dto.format);

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
            let warn_label = if warnings_count == 1 { "warning" } else { "warnings" };
            println!("Found {} {}, {} {}.", errors_count, err_label, warnings_count, warn_label);
        }
        CheckFormat::Github => {
            for issue in issues {
                let cmd = match issue.severity {
                    CheckSeverity::Error => "error",
                    CheckSeverity::Warning => "warning",
                };
                if issue.line > 0 {
                    println!("::{} file={},line={}::{}", cmd, issue.file, issue.line, issue.message);
                } else {
                    println!("::{} file={}::{}", cmd, issue.file, issue.message);
                }
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
