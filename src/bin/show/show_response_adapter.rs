use SpecTrail::domains::models::annotation::code_annotation::CodeAnnotation;
use SpecTrail::domains::models::annotation::document_annotation::DocumentAnnotation;
use SpecTrail::domains::services::annotation::scanner::ScanWarning;
use SpecTrail::use_case::show::show_use_case::ShowUseCaseResponseDto;

#[derive(Debug, PartialEq, Eq)]
pub struct ShowResponseView {
    pub stdout: Vec<String>,
    pub stderr: Vec<String>,
}

pub fn adapt_response(response: &ShowUseCaseResponseDto) -> ShowResponseView {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    let code_annotations = count_code_annotations(&response.code_annotations);
    let document_annotations = count_document_annotations(&response.document_annotations);
    let total_annotations = code_annotations + document_annotations;

    stdout.push(format!("Found {} annotations", total_annotations));
    stdout.push(format!("Warnings: {}", response.warnings.len()));
    stdout.push(String::new());

    append_file_section(&mut stdout, &response.document_annotations, "document");
    append_file_section(&mut stdout, &response.code_annotations, "code");

    for warning in &response.warnings {
        stderr.push(format_warning(warning));
    }

    ShowResponseView { stdout, stderr }
}

fn append_file_section(stdout: &mut Vec<String>, annotations: &[impl AnnotationGroupView], label: &str) {
    let mut items: Vec<_> = annotations.iter().collect();
    items.sort_by(|a, b| a.source_file().cmp(b.source_file()));

    for (index, annotation) in items.into_iter().enumerate() {
        stdout.push(format!(
            "[{}] {}",
            index + 1,
            annotation.source_file()
        ));
        stdout.push(format!(
            "    metas: {}",
            annotation.meta_count()
        ));
        stdout.push(format!(
            "    abstracts: {}",
            annotation.abstract_count()
        ));
        stdout.push(format!(
            "    details: {}",
            annotation.detail_count()
        ));
        stdout.push(format!(
            "    implementations: {}",
            annotation.implementation_count()
        ));
        stdout.push(format!("    source: {}", label));
        stdout.push(String::new());
    }
}

fn format_warning(warning: &ScanWarning) -> String {
    match warning {
        ScanWarning::Parse(pw) => format!(
            "warning: skipped unknown annotation at {}:{}",
            pw.source_file, pw.line
        ),
        ScanWarning::Resolve(rw) => format!("warning: {}", rw.message),
    }
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

trait AnnotationGroupView {
    fn source_file(&self) -> &str;
    fn meta_count(&self) -> usize;
    fn abstract_count(&self) -> usize;
    fn detail_count(&self) -> usize;
    fn implementation_count(&self) -> usize;
}

impl AnnotationGroupView for CodeAnnotation {
    fn source_file(&self) -> &str {
        &self.source_file
    }

    fn meta_count(&self) -> usize {
        self.metas.len()
    }

    fn abstract_count(&self) -> usize {
        self.abstracts.len()
    }

    fn detail_count(&self) -> usize {
        self.details.len()
    }

    fn implementation_count(&self) -> usize {
        self.implementations.len()
    }
}

impl AnnotationGroupView for DocumentAnnotation {
    fn source_file(&self) -> &str {
        &self.source_file
    }

    fn meta_count(&self) -> usize {
        self.metas.len()
    }

    fn abstract_count(&self) -> usize {
        self.abstracts.len()
    }

    fn detail_count(&self) -> usize {
        self.details.len()
    }

    fn implementation_count(&self) -> usize {
        self.implementations.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use SpecTrail::domains::models::abstract_annotation::{
        AbstractAnnotation, AbstractAnnotationId, AbstractName,
    };
    use SpecTrail::domains::models::implementation::{
        ImplementationAnnotation, ImplementationArtifact, ImplementationLink,
        ImplementationSpecName,
    };
    use SpecTrail::domains::models::layer::Layer;
    use SpecTrail::domains::models::meta::{MetaAnnotation, MetaAnnotationId, MetaName};
    use SpecTrail::domains::models::spec_detail::{
        SpecDetailAnnotation, SpecDetailAnnotationId, SpecDetailLink, SpecDetailName,
    };
    use SpecTrail::domains::services::annotation::parser::ParseWarning;
    use SpecTrail::domains::services::annotation::resolver::ResolveWarning;

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
            links: vec![SpecDetailLink::Abstract(Box::new(make_abstract()))],
        }
    }

    fn make_implementation() -> ImplementationAnnotation {
        ImplementationAnnotation {
            id: SpecTrail::domains::models::implementation::ImplementationAnnotationId(
                "i".to_string(),
            ),
            name: ImplementationSpecName("Impl".to_string()),
            r#type: None,
            layer: Layer::Implementation,
            links: vec![ImplementationLink::Abstract(Box::new(make_abstract()))],
            artifact: ImplementationArtifact("artifact".to_string()),
            status: None,
        }
    }

    #[test]
    fn renders_summary_and_file_sections_into_stdout_lines() {
        let response = ShowUseCaseResponseDto {
            document_annotations: vec![DocumentAnnotation {
                source_file: "docs/a.md".to_string(),
                metas: vec![make_meta()],
                abstracts: vec![make_abstract()],
                details: vec![make_detail()],
                implementations: vec![make_implementation()],
            }],
            code_annotations: vec![CodeAnnotation {
                source_file: "src/a.rs".to_string(),
                metas: vec![make_meta()],
                abstracts: vec![make_abstract()],
                details: vec![make_detail()],
                implementations: vec![make_implementation()],
            }],
            warnings: vec![],
        };

        let view = adapt_response(&response);

        assert_eq!(
            view.stdout,
            vec![
                "Found 8 annotations".to_string(),
                "Warnings: 0".to_string(),
                "".to_string(),
                "[1] docs/a.md".to_string(),
                "    metas: 1".to_string(),
                "    abstracts: 1".to_string(),
                "    details: 1".to_string(),
                "    implementations: 1".to_string(),
                "    source: document".to_string(),
                "".to_string(),
                "[1] src/a.rs".to_string(),
                "    metas: 1".to_string(),
                "    abstracts: 1".to_string(),
                "    details: 1".to_string(),
                "    implementations: 1".to_string(),
                "    source: code".to_string(),
                "".to_string(),
            ]
        );
        assert!(view.stderr.is_empty());
    }

    #[test]
    fn separates_warning_lines_into_stderr() {
        let response = ShowUseCaseResponseDto {
            document_annotations: vec![],
            code_annotations: vec![],
            warnings: vec![
                ScanWarning::Parse(ParseWarning {
                    source_file: "src/main.rs".to_string(),
                    line: 12,
                    message: "broken".to_string(),
                    raw_text: "@bad".to_string(),
                }),
                ScanWarning::Resolve(ResolveWarning {
                    source_annotation_id: "x".to_string(),
                    message: "missing link".to_string(),
                }),
            ],
        };

        let view = adapt_response(&response);

        assert_eq!(view.stderr.len(), 2);
        assert_eq!(
            view.stderr[0],
            "warning: skipped unknown annotation at src/main.rs:12".to_string()
        );
        assert_eq!(view.stderr[1], "warning: missing link".to_string());
    }
}
