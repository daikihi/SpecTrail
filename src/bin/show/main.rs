/// [@st-code-bin-show-main-file] layer: abstract, type: File, name: main.rs
mod dto;

use SpecTrail::domains::models::annotation::code_annotation::CodeAnnotation;
use SpecTrail::domains::models::annotation::document_annotation::DocumentAnnotation;
use SpecTrail::domains::services::annotation::scanner::ScanWarning;
use SpecTrail::use_case::show::show_use_case::{
    ShowUseCase, ShowUseCaseRequestDto, ShowUseCaseResponseDto,
};
use dto::ShowRequestDto;
use std::env;
use std::process;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    let request_dto: ShowRequestDto = match ShowRequestDto::from_args(&args) {
        Ok(request_dto) => request_dto,
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    };

    /* Convert to UseCase Request DTO */
    let use_case_request = ShowUseCaseRequestDto {
        mode: request_dto.mode.to_string(),
        target: request_dto.target.to_string(),
        scope: request_dto.scope.clone(),
    };

    let use_case: ShowUseCase = ShowUseCase::new();
    let response: Result<ShowUseCaseResponseDto, Box<dyn std::error::Error>> =
        use_case.execute(use_case_request);

    match response {
        Ok(response) => {
            print_warnings(&response.warnings);
            print_response_summary(&response);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    }
}

fn print_warnings(warnings: &[ScanWarning]) {
    for warning in warnings {
        match warning {
            ScanWarning::Parse(pw) => {
                eprintln!(
                    "WARNING [parse] {}: {} (raw: '{}')",
                    pw.source_file, pw.message, pw.raw_text
                );
            }
            ScanWarning::Resolve(rw) => {
                eprintln!("WARNING [resolve] {}", rw.message);
            }
        }
    }
}

fn print_response_summary(response: &ShowUseCaseResponseDto) {
    let code_totals = count_code_annotations(&response.code_annotations);
    let document_totals = count_document_annotations(&response.document_annotations);

    println!("Show summary");
    println!("  code files: {}", response.code_annotations.len());
    println!("  code annotations: {}", code_totals);
    println!("  document files: {}", response.document_annotations.len());
    println!("  document annotations: {}", document_totals);
    println!("  warnings: {}", response.warnings.len());
}

fn count_code_annotations(annotations: &[CodeAnnotation]) -> usize {
    annotations
        .iter()
        .map(|annotation| {
            annotation.metas.len()
                + annotation.abstracts.len()
                + annotation.details.len()
                + annotation.implementations.len()
        })
        .sum()
}

fn count_document_annotations(annotations: &[DocumentAnnotation]) -> usize {
    annotations
        .iter()
        .map(|annotation| {
            annotation.metas.len()
                + annotation.abstracts.len()
                + annotation.details.len()
                + annotation.implementations.len()
        })
        .sum()
}
