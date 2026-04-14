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

/// Entry point for the `show` CLI: parses command-line arguments, executes the Show use case, and prints any warnings and a compact summary of results.
///
/// # Examples
///
/// ```no_run
/// // Run the compiled binary from a shell:
/// // cargo run --bin show -- <mode> <target> [scope]
/// ```
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

/// Prints scan warnings to standard error in a human-readable form.
///
/// Parse warnings are printed as:
/// `WARNING [parse] <source_file>: <message> (raw: '<raw_text>')`
/// Resolve warnings are printed as:
/// `WARNING [resolve] <message>`
///
/// # Arguments
///
/// * `warnings` - Slice of `ScanWarning` values to print.
///
/// # Examples
///
/// ```
/// // Assuming `ScanWarning` is in scope:
/// // print_warnings(&[] as &[crate::ScanWarning]);
/// print_warnings(&[]);
/// ```
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

/// Prints a compact summary of the show use-case response counts to stdout.
///
/// The summary includes counts of code files, code annotations, document files,
/// document annotations, and total warnings from the provided response.
///
/// # Examples
///
/// ```
/// // Construct a minimal response with empty collections (types assumed in scope).
/// let response = ShowUseCaseResponseDto {
///     code_annotations: Vec::new(),
///     document_annotations: Vec::new(),
///     warnings: Vec::new(),
/// };
/// print_response_summary(&response);
/// ```
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

/// Counts the total number of annotations contained in the given code-file annotations.
///
/// The count is the sum, for each `CodeAnnotation`, of its `metas`, `abstracts`,
/// `details`, and `implementations` collections.
///
/// # Examples
///
/// ```
/// // Assuming CodeAnnotation has a simple constructor for tests; replace with
/// // real constructors in actual code.
/// use spec_trail_domain::CodeAnnotation;
///
/// let annotations: Vec<CodeAnnotation> = vec![];
/// assert_eq!(crate::count_code_annotations(&annotations), 0);
/// ```
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

/// Counts all annotations contained in the given document annotations slice.
///
/// Returns the total number of `metas`, `abstracts`, `details`, and `implementations` across every `DocumentAnnotation` in `annotations`.
///
/// # Returns
///
/// `usize` total count of document-level annotation items.
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
