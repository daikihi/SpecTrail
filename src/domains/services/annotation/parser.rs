/// [@st-code-domain-services-annotation-parser-file] layer: abstract, type: File, name: parser.rs
/// This file provides the parser for extracting raw annotations from source text using regular expressions.
use crate::domains::services::annotation::raw_annotation::RawAnnotation;
use regex::Regex;

/// [@st-code-domain-services-annotation-parser-annotation-parser] layer: abstract, type: Structure, name: AnnotationParser
/// Responsible for scanning file content and identifying annotation patterns.
pub struct AnnotationParser;

/// [@st-code-domain-services-annotation-parser-parse-result] layer: abstract, type: Structure, name: ParseResult
/// Holds the results of a parsing operation, including any extracted annotations and warnings.
pub struct ParseResult {
    pub annotations: Vec<RawAnnotation>,
    pub warnings: Vec<ParseWarning>,
}

/// [@st-code-domain-services-annotation-parser-parse-warning] layer: abstract, type: Structure, name: ParseWarning
/// Represents a non-fatal issue encountered during parsing, such as malformed syntax.
#[derive(Debug)]
pub struct ParseWarning {
    pub source_file: String,
    pub line: usize,
    pub message: String,
    pub raw_text: String,
}

/// [@st-code-domain-services-annotation-parser-parse-error] layer: abstract, type: Structure, name: ParseError
/// Represents a fatal error that prevents parsing from continuing.
#[derive(Debug)]
pub enum ParseError {
    IoError(std::io::Error),
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::IoError(err)
    }
}

impl AnnotationParser {
    /* Parses all annotations from the given content string.
     *
     * This method uses regular expressions to find annotation tags and extract their
     * metadata fields (layer, type, name, and optional links). */
    pub fn parse(content: &str, source_file: &str) -> Result<ParseResult, ParseError> {
        let mut annotations = Vec::new();
        let warnings = Vec::new();

    /* Regex for standard single-line annotations: [@id] layer: L, type: T, name: N, links: [L1, L2] */
        let re = Regex::new(r"\[@(?P<id>[^\]]+)\]\s*layer:\s*(?P<layer>[^,]+),\s*type:\s*(?P<type>[^,]+),\s*name:\s*(?P<name>[^,\n]+)(?:,\s*links:\s*\[(?P<links>[^\]]+)\])?").unwrap();

        for cap in re.captures_iter(content) {
            let id = cap["id"].trim().to_string();
            let layer = cap["layer"].trim().to_string();
            let annotation_type = cap["type"].trim().to_string();
            let name = cap["name"].trim().to_string();

            let mut links = Vec::new();
            if let Some(links_str) = cap.name("links").map(|m| m.as_str()) {
                for link_id in links_str.split(',') {
                    let link_id = link_id.trim();
                    if !link_id.is_empty() {
                        links.push(link_id.to_string());
                    }
                }
            }

            annotations.push(RawAnnotation {
                id,
                layer,
                annotation_type,
                name,
                links,
                source_file: source_file.to_string(),
            });
        }

        Ok(ParseResult {
            annotations,
            warnings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_annotation() {
        let content = "/// [@st-app] layer: meta, type: Philosophy, name: App";
        let result = AnnotationParser::parse(content, "src/main.rs").unwrap();
        assert_eq!(result.annotations.len(), 1);
        let anno = &result.annotations[0];
        assert_eq!(anno.id, "st-app");
        assert_eq!(anno.layer, "meta");
        assert_eq!(anno.annotation_type, "Philosophy");
        assert_eq!(anno.name, "App");
        assert_eq!(anno.source_file, "src/main.rs");
    }

    #[test]
    fn test_parse_with_links() {
        let content =
            "/// [@st-bar] layer: abstract, type: Page, name: Bar, links: [@st-foo, @st-baz]";
        let result = AnnotationParser::parse(content, "src/bar.rs").unwrap();
        assert_eq!(result.annotations.len(), 1);
        let anno = &result.annotations[0];
        assert_eq!(anno.links, vec!["@st-foo", "@st-baz"]);
    }
}
