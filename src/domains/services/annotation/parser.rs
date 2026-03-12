use crate::domains::services::annotation::raw_annotation::RawAnnotation;
use regex::Regex;

pub struct AnnotationParser;

pub struct ParseResult {
    pub annotations: Vec<RawAnnotation>,
    pub warnings: Vec<ParseWarning>,
}

#[derive(Debug)]
pub struct ParseWarning {
    pub source_file: String,
    pub line: usize,
    pub message: String,
    pub raw_text: String,
}

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
    pub fn parse(content: &str, source_file: &str) -> Result<ParseResult, ParseError> {
        let mut annotations = Vec::new();
        let warnings = Vec::new();

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
