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

#[cfg(test)]
mod tests {
    use super::*;
    use SpecTrail::domains::models::abstract_annotation::{
        AbstractAnnotation, AbstractAnnotationId, AbstractName,
    };
    use SpecTrail::domains::models::implementation::{
        ImplementationAnnotation, ImplementationAnnotationId, ImplementationArtifact,
        ImplementationSpecName,
    };
    use SpecTrail::domains::models::layer::Layer;
    use SpecTrail::domains::models::meta::{MetaAnnotation, MetaAnnotationId, MetaName};
    use SpecTrail::domains::models::spec_detail::{
        SpecDetailAnnotation, SpecDetailAnnotationId, SpecDetailName,
    };

    fn make_meta() -> MetaAnnotation {
        MetaAnnotation {
            id: MetaAnnotationId("m".to_string()),
            name: MetaName("Meta".to_string()),
            r#type: None,
            layer: Layer::Meta,
            links: vec![],
        }
    }

    fn make_abstract() -> AbstractAnnotation {
        AbstractAnnotation {
            id: AbstractAnnotationId("a".to_string()),
            name: AbstractName("Abstract".to_string()),
            r#type: None,
            layer: Layer::Abstract,
            links: vec![],
        }
    }

    fn make_detail() -> SpecDetailAnnotation {
        SpecDetailAnnotation {
            id: SpecDetailAnnotationId("d".to_string()),
            name: SpecDetailName("Detail".to_string()),
            r#type: None,
            layer: Layer::SpecDetail,
            links: vec![],
        }
    }

    fn make_implementation() -> ImplementationAnnotation {
        ImplementationAnnotation {
            id: ImplementationAnnotationId("i".to_string()),
            name: ImplementationSpecName("Impl".to_string()),
            r#type: None,
            layer: Layer::Implementation,
            links: vec![],
            artifact: ImplementationArtifact("artifact".to_string()),
            status: None,
        }
    }

    fn make_code_annotation(
        metas: usize,
        abstracts: usize,
        details: usize,
        impls: usize,
    ) -> CodeAnnotation {
        CodeAnnotation {
            metas: (0..metas).map(|_| make_meta()).collect(),
            abstracts: (0..abstracts).map(|_| make_abstract()).collect(),
            details: (0..details).map(|_| make_detail()).collect(),
            implementations: (0..impls).map(|_| make_implementation()).collect(),
        }
    }

    fn make_document_annotation(
        metas: usize,
        abstracts: usize,
        details: usize,
        impls: usize,
    ) -> DocumentAnnotation {
        DocumentAnnotation {
            metas: (0..metas).map(|_| make_meta()).collect(),
            abstracts: (0..abstracts).map(|_| make_abstract()).collect(),
            details: (0..details).map(|_| make_detail()).collect(),
            implementations: (0..impls).map(|_| make_implementation()).collect(),
        }
    }

    #[test]
    fn count_code_annotations_returns_zero_for_empty_slice() {
        assert_eq!(count_code_annotations(&[]), 0);
    }

    #[test]
    fn count_code_annotations_sums_all_annotation_types() {
        let annotation = make_code_annotation(2, 3, 1, 4);
        assert_eq!(count_code_annotations(&[annotation]), 10);
    }

    #[test]
    fn count_code_annotations_sums_across_multiple_files() {
        let a = make_code_annotation(1, 0, 0, 0);
        let b = make_code_annotation(0, 2, 1, 0);
        let c = make_code_annotation(0, 0, 0, 3);
        assert_eq!(count_code_annotations(&[a, b, c]), 7);
    }

    #[test]
    fn count_code_annotations_with_all_empty_collections_returns_zero() {
        let annotation = make_code_annotation(0, 0, 0, 0);
        assert_eq!(count_code_annotations(&[annotation]), 0);
    }

    #[test]
    fn count_document_annotations_returns_zero_for_empty_slice() {
        assert_eq!(count_document_annotations(&[]), 0);
    }

    #[test]
    fn count_document_annotations_sums_all_annotation_types() {
        let annotation = make_document_annotation(1, 2, 3, 4);
        assert_eq!(count_document_annotations(&[annotation]), 10);
    }

    #[test]
    fn count_document_annotations_sums_across_multiple_files() {
        let a = make_document_annotation(2, 0, 0, 0);
        let b = make_document_annotation(0, 0, 3, 0);
        assert_eq!(count_document_annotations(&[a, b]), 5);
    }
}