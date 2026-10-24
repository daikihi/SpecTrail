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
#[derive(Debug, Clone)]
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
    ) -> std::io::Result<ScanResult> {
        Self::scan_with_ignored_prefixes(code_path, code_ext, doc_path, doc_ext, &["_fixture_"])
    }

    /* Performs a full scan of code and document directories with custom ignored prefixes. */
    pub fn scan_with_ignored_prefixes<P: AsRef<Path>, S: AsRef<str>>(
        code_path: P,
        code_ext: &str,
        doc_path: P,
        doc_ext: &str,
        ignored_prefixes: &[S],
    ) -> std::io::Result<ScanResult> {
        let mut raw_annotations = Vec::new();
        let mut warnings = Vec::new();

        /* 1. Scan code files */
        let (mut code_raw, code_parse_warnings) = Self::collect_raw(code_path.as_ref(), code_ext)?;
        raw_annotations.append(&mut code_raw);
        warnings.extend(code_parse_warnings.into_iter().map(ScanWarning::Parse));

        /* 2. Scan document files */
        let (mut doc_raw, doc_parse_warnings) = Self::collect_raw(doc_path.as_ref(), doc_ext)?;
        raw_annotations.append(&mut doc_raw);
        warnings.extend(doc_parse_warnings.into_iter().map(ScanWarning::Parse));

        /* 3. Resolve links */
        let resolve_result =
            AnnotationResolver::resolve_with_ignored_prefixes(raw_annotations, ignored_prefixes);
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

        Ok(ScanResult {
            code_annotations,
            document_annotations,
            warnings,
        })
    }

    /* Recursively collects raw annotations from all files matching the extension in the given path. */
    fn collect_raw(
        path: &Path,
        extension: &str,
    ) -> std::io::Result<(Vec<RawAnnotation>, Vec<ParseWarning>)> {
        let mut all_raw = Vec::new();
        let mut all_warnings = Vec::new();

        if path.as_os_str().is_empty() {
            return Ok((all_raw, all_warnings));
        }

        let entries = fs::read_dir(path)?;
        for entry in entries {
            let entry = entry?;
            let child_path = entry.path();
            if child_path.is_dir() {
                if child_path.file_name().and_then(|s| s.to_str()) == Some("target") {
                    continue;
                }
                let (raw, warnings) = Self::collect_raw(&child_path, extension)?;
                all_raw.extend(raw);
                all_warnings.extend(warnings);
            } else if child_path.extension().and_then(|s| s.to_str())
                == Some(extension.trim_start_matches('.'))
            {
                let content = fs::read_to_string(&child_path)?;
                let path_str = child_path.to_str().unwrap_or("");
                if let Ok(result) = AnnotationParser::parse(&content, path_str) {
                    all_raw.extend(result.annotations);
                    all_warnings.extend(result.warnings);
                }
            }
        }

        Ok((all_raw, all_warnings))
    }

    /* Backward compatibility methods */
    pub fn scan_code<P: AsRef<Path>>(
        path: P,
        extension: &str,
    ) -> std::io::Result<Vec<CodeAnnotation>> {
        let result = Self::scan(path.as_ref(), extension, Path::new(""), "")?;
        Ok(result.code_annotations)
    }

    pub fn scan_documents<P: AsRef<Path>>(
        path: P,
        extension: &str,
    ) -> std::io::Result<Vec<DocumentAnnotation>> {
        let result = Self::scan(Path::new(""), "", path.as_ref(), extension)?;
        Ok(result.document_annotations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_collect_raw_fails_on_nonexistent_directory() {
        let non_existent = Path::new("nonexistent_directory_xyz");
        let result = AnnotationScanner::collect_raw(non_existent, "rs");
        assert!(result.is_err());
    }

    #[test]
    fn test_scan_fails_when_code_path_does_not_exist() {
        let non_existent = Path::new("nonexistent_directory_xyz");
        let result = AnnotationScanner::scan(non_existent, "rs", Path::new(""), "");
        assert!(result.is_err());
    }

    #[test]
    fn test_scan_fails_when_doc_path_does_not_exist() {
        let non_existent = Path::new("nonexistent_directory_xyz");
        let result = AnnotationScanner::scan(Path::new(""), "", non_existent, "md");
        assert!(result.is_err());
    }

    #[test]
    fn test_collect_raw_succeeds_on_empty_path() {
        let empty = Path::new("");
        let result = AnnotationScanner::collect_raw(empty, "rs");
        assert!(result.is_ok());
        let (raw, warnings) = result.unwrap();
        assert!(raw.is_empty());
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_collect_raw_propagates_file_read_error() {
        let temp_dir = std::env::temp_dir().join("spectrail_test_invalid_utf8");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let file_path = temp_dir.join("invalid.rs");
        // Write invalid UTF-8 bytes to the file
        let mut file = File::create(&file_path).unwrap();
        file.write_all(&[0xFF, 0xFE, 0xFD]).unwrap();
        drop(file);

        let result = AnnotationScanner::collect_raw(&temp_dir, "rs");
        let _ = fs::remove_dir_all(&temp_dir);

        assert!(result.is_err());
    }
}
