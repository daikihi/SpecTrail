/// [@st-code-domain-services-annotation-parser-file] layer: abstract, type: File, name: parser.rs
/// This file provides the parser for extracting raw annotations from source text using regular expressions.
use crate::domains::models::line_number::LineNumber;
use crate::domains::services::annotation::raw_annotation::RawAnnotation;
use regex::Regex;
use std::sync::OnceLock;

/// [@st-code-domain-services-annotation-parser-annotation-parser] layer: abstract, type: Structure, name: AnnotationParser, links: [@st-manual-spec-annotation-parser, @st-manual-spec-parser-code-annotations, @st-manual-spec-parser-spec-docs]
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
#[derive(Debug, Clone)]
pub struct ParseWarning {
    pub source_file: String,
    pub line: LineNumber,
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

fn annotation_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"\[@(?P<id>[^\]]+)\]\s*layer:\s*(?P<layer>[^,\n]+),\s*type:\s*(?P<type>[^,\n]+),\s*name:\s*(?P<name>[^,\n]+)(?P<rest>[^\n]*)",
        )
        .expect("annotation regex must be valid")
    })
}

fn parse_optional_fields(rest: &str) -> (Vec<String>, Option<String>, Option<String>) {
    let mut links = Vec::new();
    let mut artifact = None;
    let mut status = None;

    let mut remaining = rest.trim_start();
    while let Some(after_comma) = remaining.strip_prefix(',') {
        let field = after_comma.trim_start();
        if let Some(after_key) = field.strip_prefix("links:") {
            let after_key = after_key.trim_start();
            if let Some(after_bracket) = after_key.strip_prefix('[') {
                if let Some(end_idx) = after_bracket.find(']') {
                    let links_str = &after_bracket[..end_idx];
                    for link_id in links_str.split(',') {
                        let link_id = link_id.trim();
                        if !link_id.is_empty() {
                            links.push(link_id.to_string());
                        }
                    }
                    remaining = after_bracket[end_idx + 1..].trim_start();
                    continue;
                }
            }
            break;
        } else if let Some(after_key) = field.strip_prefix("status:") {
            let (value, next) = split_next_field(after_key);
            if !value.is_empty() {
                status = Some(value.to_string());
            }
            remaining = next;
        } else if let Some(after_key) = field.strip_prefix("artifact:") {
            let (value, next) = split_next_field(after_key);
            if !value.is_empty() {
                artifact = Some(value.to_string());
            }
            remaining = next;
        } else {
            let (_, next) = split_next_field(field);
            remaining = next;
        }
    }

    (links, artifact, status)
}

fn split_next_field(s: &str) -> (&str, &str) {
    match s.find(',') {
        Some(idx) => (s[..idx].trim(), &s[idx..]),
        None => (s.trim(), ""),
    }
}

impl AnnotationParser {
    /* Parses all annotations from the given content string.
     *
     * This method uses regular expressions to find annotation tags and extract their
     * metadata fields (layer, type, name, and optional links, artifact, and status). */
    pub fn parse(content: &str, source_file: &str) -> Result<ParseResult, ParseError> {
        let mut annotations = Vec::new();
        let warnings = Vec::new();

        let re = annotation_regex();

        let mut previous_end = 0;
        let mut line_idx = 1;
        for cap in re.captures_iter(content) {
            let full_match = cap.get(0).unwrap();
            line_idx += content[previous_end..full_match.start()]
                .chars()
                .filter(|&c| c == '\n')
                .count();
            previous_end = full_match.start();
            let line = LineNumber::new(line_idx);

            let id = cap["id"].trim().to_string();
            let layer = cap["layer"].trim().to_string();
            let annotation_type = cap["type"].trim().to_string();
            let name = cap["name"].trim().to_string();

            let rest = cap.name("rest").map(|m| m.as_str()).unwrap_or("");
            let (links, artifact, status) = parse_optional_fields(rest);

            annotations.push(RawAnnotation {
                id,
                layer,
                annotation_type,
                name,
                links,
                artifact,
                status,
                source_file: source_file.to_string(),
                line,
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
        let content = "/// [@_fixture_bar] layer: abstract, type: Page, name: Bar, links: [@_fixture_foo, @_fixture_baz]";
        let result = AnnotationParser::parse(content, "src/bar.rs").unwrap();
        assert_eq!(result.annotations.len(), 1);
        let anno = &result.annotations[0];
        assert_eq!(anno.links, vec!["@_fixture_foo", "@_fixture_baz"]);
        assert_eq!(anno.line, LineNumber::new(1));
    }

    #[test]
    fn test_parse_multiline_line_number() {
        let content =
            "line 1\nline 2\n/// [@st-line3] layer: meta, type: Philosophy, name: Line3\nline 4";
        let result = AnnotationParser::parse(content, "src/test.rs").unwrap();
        assert_eq!(result.annotations.len(), 1);
        assert_eq!(result.annotations[0].line, LineNumber::new(3));
    }

    #[test]
    fn test_parse_implementation_status_and_artifact_in_any_order() {
        let content = concat!(
            "/// [@_fixture_impl1] layer: implementation, type: DomainEntity, name: User, links: [@_fixture_spec], status: Completed, artifact: src/domain/user.rs\n",
            "/// [@_fixture_impl2] layer: implementation, type: DaoRepository, name: UserRepo, artifact: src/repo/user.rs, status: InProgress, links: [@_fixture_spec]\n",
            "/// [@_fixture_impl3] layer: implementation, type: Structure, name: Plain\n"
        );
        let result = AnnotationParser::parse(content, "src/impls.rs").unwrap();
        assert_eq!(result.annotations.len(), 3);

        let first = &result.annotations[0];
        assert_eq!(first.links, vec!["@_fixture_spec"]);
        assert_eq!(first.status.as_deref(), Some("Completed"));
        assert_eq!(first.artifact.as_deref(), Some("src/domain/user.rs"));

        let second = &result.annotations[1];
        assert_eq!(second.links, vec!["@_fixture_spec"]);
        assert_eq!(second.status.as_deref(), Some("InProgress"));
        assert_eq!(second.artifact.as_deref(), Some("src/repo/user.rs"));

        let third = &result.annotations[2];
        assert!(third.links.is_empty());
        assert_eq!(third.status, None);
        assert_eq!(third.artifact, None);
    }
}
