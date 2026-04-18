/// [@st-code-domain-services-annotation-scanner-file] layer: abstract, type: File, name: scanner.rs
/// This file provides the AnnotationScanner, which orchestrates the parsing and resolution of annotations across multiple files and directories.
use crate::domains::models::annotation::code_annotation::CodeAnnotation;
use crate::domains::models::annotation::document_annotation::DocumentAnnotation;
use crate::domains::services::annotation::parser::{AnnotationParser, ParseWarning};
use crate::domains::services::annotation::raw_annotation::RawAnnotation;
use crate::domains::services::annotation::resolver::{
    AnnotationResolver, ResolveWarning, ResolvedAnnotation,
};
use std::fs;
use std::path::Path;

/// [@st-code-domain-services-annotation-scanner-scan-result] layer: abstract, type: Structure, name: ScanResult
/// Holds the final results of a scan, including categorized annotations and all accumulated warnings.
pub struct ScanResult {
    pub code_annotations: Vec<CodeAnnotation>,
    pub document_annotations: Vec<DocumentAnnotation>,
    pub warnings: Vec<ScanWarning>,
}

/// [@st-code-domain-services-annotation-scanner-scan-warning] layer: abstract, type: Structure, name: ScanWarning
/// Wraps warnings from either the parsing or resolution stages.
#[derive(Debug)]
pub enum ScanWarning {
    Parse(ParseWarning),
    Resolve(ResolveWarning),
}

/// [@st-code-domain-services-annotation-scanner-annotation-scanner] layer: abstract, type: Structure, name: AnnotationScanner
/// Orchestrates the end-to-end scanning process: file traversal, parsing, resolution, and categorization.
pub struct AnnotationScanner;

impl AnnotationScanner {
/* Performs a full scan of code and document directories.
     *
     * This method traverses both directories, parses all matching files for annotations,
     * resolves links between them, and then categorizes the results based on their source path. */
    pub fn scan<P: AsRef<Path>>(
        code_path: P,
        code_ext: &str,
        doc_path: P,
        doc_ext: &str,
    ) -> ScanResult {
        let mut raw_annotations = Vec::new();
        let mut warnings = Vec::new();

        /* 1. Scan code files */
        let (mut code_raw, code_parse_warnings) = Self::collect_raw(code_path.as_ref(), code_ext);
        raw_annotations.append(&mut code_raw);
        warnings.extend(code_parse_warnings.into_iter().map(ScanWarning::Parse));

        /* 2. Scan document files */
        let (mut doc_raw, doc_parse_warnings) = Self::collect_raw(doc_path.as_ref(), doc_ext);
        raw_annotations.append(&mut doc_raw);
        warnings.extend(doc_parse_warnings.into_iter().map(ScanWarning::Parse));

        /* 3. Resolve links */
        let resolve_result = AnnotationResolver::resolve(raw_annotations);
        warnings.extend(
            resolve_result
                .warnings
                .into_iter()
                .map(ScanWarning::Resolve),
        );

        /* 4. Categorize by file path */
        let mut code_annotations = Vec::new();
        let mut document_annotations = Vec::new();

        let code_path_str = code_path.as_ref().to_str().unwrap_or("");
        let doc_path_str = doc_path.as_ref().to_str().unwrap_or("");

        /* Use maps to aggregate annotations by file path for both code and documents. */
        let mut code_map: std::collections::HashMap<String, CodeAnnotation> =
            std::collections::HashMap::new();
        let mut doc_map: std::collections::HashMap<String, DocumentAnnotation> =
            std::collections::HashMap::new();

        for resolved in resolve_result.annotations {
            let source_file = resolved.source_file().to_string();

            if !code_path_str.is_empty() && source_file.starts_with(code_path_str) {
                let container = code_map
                    .entry(source_file.clone())
                    .or_insert_with(|| CodeAnnotation {
                        source_file: source_file.clone(),
                        metas: vec![],
                        abstracts: vec![],
                        details: vec![],
                        implementations: vec![],
                    });
                match resolved {
                    ResolvedAnnotation::Meta(a, _) => container.metas.push(a),
                    ResolvedAnnotation::Abstract(a, _) => container.abstracts.push(a),
                    ResolvedAnnotation::SpecDetail(a, _) => container.details.push(a),
                    ResolvedAnnotation::Implementation(a, _) => container.implementations.push(a),
                }
            } else if !doc_path_str.is_empty() && source_file.starts_with(doc_path_str) {
                let container = doc_map
                    .entry(source_file.clone())
                    .or_insert_with(|| DocumentAnnotation {
                        source_file: source_file.clone(),
                        metas: vec![],
                        abstracts: vec![],
                        details: vec![],
                        implementations: vec![],
                    });
                match resolved {
                    ResolvedAnnotation::Meta(a, _) => container.metas.push(a),
                    ResolvedAnnotation::Abstract(a, _) => container.abstracts.push(a),
                    ResolvedAnnotation::SpecDetail(a, _) => container.details.push(a),
                    ResolvedAnnotation::Implementation(a, _) => container.implementations.push(a),
                }
            }
        }

        code_annotations.extend(code_map.into_values());
        document_annotations.extend(doc_map.into_values());

        ScanResult {
            code_annotations,
            document_annotations,
            warnings,
        }
    }

    /* Recursively collects raw annotations from all files matching the extension in the given path. */
    fn collect_raw(path: &Path, extension: &str) -> (Vec<RawAnnotation>, Vec<ParseWarning>) {
        let mut all_raw = Vec::new();
        let mut all_warnings = Vec::new();

        if path.as_os_str().is_empty() {
            return (all_raw, all_warnings);
        }

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let child_path = entry.path();
                if child_path.is_dir() {
                    if child_path.file_name().and_then(|s| s.to_str()) == Some("target") {
                        continue;
                    }
                    let (raw, warnings) = Self::collect_raw(&child_path, extension);
                    all_raw.extend(raw);
                    all_warnings.extend(warnings);
                } else if child_path.extension().and_then(|s| s.to_str())
                    == Some(extension.trim_start_matches('.'))
                {
                    if let Ok(content) = fs::read_to_string(&child_path) {
                        let path_str = child_path.to_str().unwrap_or("");
                        match AnnotationParser::parse(&content, path_str) {
                            Ok(result) => {
                                all_raw.extend(result.annotations);
                                all_warnings.extend(result.warnings);
                            }
                            Err(_) => {}
                        }
                    }
                }
            }
        }

        (all_raw, all_warnings)
    }

    /* Backward compatibility methods */
    pub fn scan_code<P: AsRef<Path>>(path: P, extension: &str) -> Vec<CodeAnnotation> {
        let result = Self::scan(path.as_ref(), extension, Path::new(""), "");
        result.code_annotations
    }

    pub fn scan_documents<P: AsRef<Path>>(path: P, extension: &str) -> Vec<DocumentAnnotation> {
        let result = Self::scan(Path::new(""), "", path.as_ref(), extension);
        result.document_annotations
    }
}
