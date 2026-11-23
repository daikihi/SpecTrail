/// [@st-code-domain-services-annotation-parser-file] layer: abstract, type: File, name: parser.rs
/// This file provides the parser for extracting raw annotations from source text.
use crate::domains::models::line_number::LineNumber;
use crate::domains::services::annotation::raw_annotation::RawAnnotation;

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

fn parse_fence_marker(trimmed_line: &str) -> Option<(char, usize, &str)> {
    let first = trimmed_line.chars().next()?;
    if first != '`' && first != '~' {
        return None;
    }
    let count = trimmed_line.chars().take_while(|&c| c == first).count();
    if count >= 3 {
        Some((first, count, &trimmed_line[count..]))
    } else {
        None
    }
}

fn extract_annotation_candidate(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    let body = if let Some(rest) = trimmed.strip_prefix("///") {
        rest.trim_start()
    } else {
        trimmed
    };
    if body.starts_with("[@") {
        Some(body)
    } else {
        None
    }
}

fn split_fields(input: &str) -> Result<Vec<&str>, &'static str> {
    let mut parts = Vec::new();
    let mut bracket_depth: usize = 0;
    let mut in_quotes = false;
    let mut escaped = false;
    let mut start = 0;

    for (idx, ch) in input.char_indices() {
        if in_quotes {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_quotes = false;
            }
            continue;
        }

        match ch {
            '"' => in_quotes = true,
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            ',' if bracket_depth == 0 => {
                parts.push(&input[start..idx]);
                start = idx + 1;
            }
            _ => {}
        }
    }

    if in_quotes {
        return Err("unterminated quoted string");
    }

    parts.push(&input[start..]);
    Ok(parts)
}

fn unquote_value(raw: &str) -> Result<String, &'static str> {
    if !raw.starts_with('"') {
        return Ok(raw.to_string());
    }
    if raw.len() < 2 || !raw.ends_with('"') {
        return Err("malformed quoted value");
    }

    let inner = &raw[1..raw.len() - 1];
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('"') => out.push('"'),
                Some('\\') => out.push('\\'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => return Err("dangling escape in quoted value"),
            }
        } else if ch == '"' {
            return Err("unexpected unescaped quote inside value");
        } else {
            out.push(ch);
        }
    }
    Ok(out)
}

fn push_parse_warning(
    warnings: &mut Vec<ParseWarning>,
    source_file: &str,
    line: LineNumber,
    raw_line: &str,
    message: String,
) {
    warnings.push(ParseWarning {
        source_file: source_file.to_string(),
        line,
        message,
        raw_text: raw_line.to_string(),
    });
}

fn parse_candidate_line(
    candidate: &str,
    raw_line: &str,
    source_file: &str,
    line: LineNumber,
    warnings: &mut Vec<ParseWarning>,
) -> Option<RawAnnotation> {
    let after_open = candidate.strip_prefix("[@")?;
    let Some(close_idx) = after_open.find(']') else {
        push_parse_warning(
            warnings,
            source_file,
            line,
            raw_line,
            "Malformed annotation: missing closing ']' in tag".to_string(),
        );
        return None;
    };

    let id = after_open[..close_idx].trim();
    if id.is_empty() {
        push_parse_warning(
            warnings,
            source_file,
            line,
            raw_line,
            "Malformed annotation: empty annotation ID".to_string(),
        );
        return None;
    }

    let fields_str = after_open[close_idx + 1..].trim();
    let mut layer: Option<String> = None;
    let mut annotation_type: Option<String> = None;
    let mut name: Option<String> = None;
    let mut links: Vec<String> = Vec::new();
    let mut links_seen = false;
    let mut artifact: Option<String> = None;
    let mut status: Option<String> = None;

    if !fields_str.is_empty() {
        let segments = match split_fields(fields_str) {
            Ok(segs) => segs,
            Err(reason) => {
                push_parse_warning(
                    warnings,
                    source_file,
                    line,
                    raw_line,
                    format!("Malformed annotation @{}: {}", id, reason),
                );
                return None;
            }
        };

        for segment in segments {
            let trimmed = segment.trim();
            if trimmed.is_empty() {
                push_parse_warning(
                    warnings,
                    source_file,
                    line,
                    raw_line,
                    format!("Malformed annotation @{}: empty field segment", id),
                );
                continue;
            }

            let Some((raw_key, raw_val)) = trimmed.split_once(':') else {
                push_parse_warning(
                    warnings,
                    source_file,
                    line,
                    raw_line,
                    format!(
                        "Malformed field '{}' in @{}: expected 'key: value'",
                        trimmed, id
                    ),
                );
                continue;
            };

            let key = raw_key.trim();
            let val = raw_val.trim();

            match key {
                "layer" => {
                    if layer.is_some() {
                        push_parse_warning(
                            warnings,
                            source_file,
                            line,
                            raw_line,
                            format!("Duplicate field 'layer' in @{}", id),
                        );
                    } else if val.is_empty() {
                        push_parse_warning(
                            warnings,
                            source_file,
                            line,
                            raw_line,
                            format!("Empty value for required field 'layer' in @{}", id),
                        );
                    } else {
                        layer = Some(val.to_string());
                    }
                }
                "type" => {
                    if annotation_type.is_some() {
                        push_parse_warning(
                            warnings,
                            source_file,
                            line,
                            raw_line,
                            format!("Duplicate field 'type' in @{}", id),
                        );
                    } else if val.is_empty() {
                        push_parse_warning(
                            warnings,
                            source_file,
                            line,
                            raw_line,
                            format!("Empty value for required field 'type' in @{}", id),
                        );
                    } else {
                        annotation_type = Some(val.to_string());
                    }
                }
                "name" => {
                    if name.is_some() {
                        push_parse_warning(
                            warnings,
                            source_file,
                            line,
                            raw_line,
                            format!("Duplicate field 'name' in @{}", id),
                        );
                    } else if val.is_empty() {
                        push_parse_warning(
                            warnings,
                            source_file,
                            line,
                            raw_line,
                            format!("Empty value for required field 'name' in @{}", id),
                        );
                    } else {
                        match unquote_value(val) {
                            Ok(parsed) if parsed.trim().is_empty() => {
                                push_parse_warning(
                                    warnings,
                                    source_file,
                                    line,
                                    raw_line,
                                    format!("Empty value for required field 'name' in @{}", id),
                                );
                            }
                            Ok(parsed) => name = Some(parsed),
                            Err(reason) => {
                                push_parse_warning(
                                    warnings,
                                    source_file,
                                    line,
                                    raw_line,
                                    format!("Malformed field 'name' in @{}: {}", id, reason),
                                );
                            }
                        }
                    }
                }
                "links" => {
                    if links_seen {
                        push_parse_warning(
                            warnings,
                            source_file,
                            line,
                            raw_line,
                            format!("Duplicate field 'links' in @{}", id),
                        );
                    } else {
                        links_seen = true;
                        if let Some(inner) = val.strip_prefix('[').and_then(|s| s.strip_suffix(']'))
                        {
                            for link_id in inner.split(',') {
                                let link_id = link_id.trim();
                                if !link_id.is_empty() {
                                    links.push(link_id.to_string());
                                }
                            }
                        } else {
                            push_parse_warning(
                                warnings,
                                source_file,
                                line,
                                raw_line,
                                format!(
                                    "Malformed 'links' field '{}' in @{}: expected '[...]' list",
                                    val, id
                                ),
                            );
                        }
                    }
                }
                "status" => {
                    if status.is_some() {
                        push_parse_warning(
                            warnings,
                            source_file,
                            line,
                            raw_line,
                            format!("Duplicate field 'status' in @{}", id),
                        );
                    } else if val.is_empty() {
                        push_parse_warning(
                            warnings,
                            source_file,
                            line,
                            raw_line,
                            format!("Empty value for field 'status' in @{}", id),
                        );
                    } else {
                        status = Some(val.to_string());
                    }
                }
                "artifact" => {
                    if artifact.is_some() {
                        push_parse_warning(
                            warnings,
                            source_file,
                            line,
                            raw_line,
                            format!("Duplicate field 'artifact' in @{}", id),
                        );
                    } else if val.is_empty() {
                        push_parse_warning(
                            warnings,
                            source_file,
                            line,
                            raw_line,
                            format!("Empty value for field 'artifact' in @{}", id),
                        );
                    } else {
                        match unquote_value(val) {
                            Ok(parsed) if parsed.trim().is_empty() => {
                                push_parse_warning(
                                    warnings,
                                    source_file,
                                    line,
                                    raw_line,
                                    format!("Empty value for field 'artifact' in @{}", id),
                                );
                            }
                            Ok(parsed) => artifact = Some(parsed),
                            Err(reason) => {
                                push_parse_warning(
                                    warnings,
                                    source_file,
                                    line,
                                    raw_line,
                                    format!("Malformed field 'artifact' in @{}: {}", id, reason),
                                );
                            }
                        }
                    }
                }
                _ => {
                    push_parse_warning(
                        warnings,
                        source_file,
                        line,
                        raw_line,
                        format!("Unknown annotation field '{}' in @{}", key, id),
                    );
                }
            }
        }
    }

    let mut missing = Vec::new();
    if layer.is_none() {
        missing.push("layer");
    }
    if annotation_type.is_none() {
        missing.push("type");
    }
    if name.is_none() {
        missing.push("name");
    }

    if !missing.is_empty() {
        push_parse_warning(
            warnings,
            source_file,
            line,
            raw_line,
            format!(
                "Missing required field(s) [{}] in @{}",
                missing.join(", "),
                id
            ),
        );
        return None;
    }

    Some(RawAnnotation {
        id: id.to_string(),
        layer: layer.unwrap(),
        annotation_type: annotation_type.unwrap(),
        name: name.unwrap(),
        links,
        artifact,
        status,
        source_file: source_file.to_string(),
        line,
    })
}

impl AnnotationParser {
    /* Parses all annotations from the given content string.
     *
     * Scans line-by-line, skips Markdown fenced code blocks and non-annotation lines
     * (such as inline code examples or string literals), parses key-value fields in any order,
     * and collects ParseWarnings for malformed annotation lines. */
    pub fn parse(content: &str, source_file: &str) -> Result<ParseResult, ParseError> {
        let mut annotations = Vec::new();
        let mut warnings = Vec::new();
        let mut active_fence: Option<(char, usize)> = None;

        for (idx, raw_line) in content.lines().enumerate() {
            let line_text = raw_line.trim_end_matches('\r');
            let trimmed = line_text.trim_start();

            if let Some((fence_char, fence_len)) = active_fence {
                if let Some((ch, count, rest)) = parse_fence_marker(trimmed)
                    && ch == fence_char
                    && count >= fence_len
                    && rest.trim().is_empty()
                {
                    active_fence = None;
                }
                continue;
            }

            if let Some((ch, count, _)) = parse_fence_marker(trimmed) {
                active_fence = Some((ch, count));
                continue;
            }

            let Some(candidate) = extract_annotation_candidate(line_text) else {
                continue;
            };

            let line = LineNumber::new(idx + 1);
            if let Some(raw_annotation) =
                parse_candidate_line(candidate, line_text, source_file, line, &mut warnings)
            {
                annotations.push(raw_annotation);
            }
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
            "/// [@_fixture_bar] layer: abstract, type: Page, name: Bar, links: [@_fixture_foo, @_fixture_baz]";
        let result = AnnotationParser::parse(content, "src/bar.rs").unwrap();
        assert_eq!(result.annotations.len(), 1);
        let anno = &result.annotations[0];
        assert_eq!(anno.links, vec!["@_fixture_foo", "@_fixture_baz"]);
        assert_eq!(anno.line, LineNumber::new(1));
    }

    #[test]
    fn test_parse_multiline_line_number() {
        let content = "line 1\nline 2\n/// [@st-line3] layer: meta, type: Philosophy, name: Line3\nline 4";
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

    #[test]
    fn test_ignores_annotations_inside_markdown_fenced_code_blocks() {
        let content = concat!(
            "/// [@st-before] layer: meta, type: Philosophy, name: Before\n",
            "```rust\n",
            "/// [@st-inside-backticks] layer: abstract, type: Structure, name: InsideBackticks\n",
            "```\n",
            "~~~\n",
            "/// [@st-inside-tildes] layer: abstract, type: Structure, name: InsideTildes\n",
            "~~~\n",
            "/// [@st-after] layer: abstract, type: Structure, name: After\n"
        );
        let result = AnnotationParser::parse(content, "docs/spec.md").unwrap();
        let ids: Vec<&str> = result.annotations.iter().map(|a| a.id.as_str()).collect();
        assert_eq!(ids, vec!["st-before", "st-after"]);
        assert_eq!(result.annotations[0].line, LineNumber::new(1));
        assert_eq!(result.annotations[1].line, LineNumber::new(8));
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_ignores_longer_nested_fence_until_matching_close() {
        let content = concat!(
            "````markdown\n",
            "```\n",
            "/// [@st-still-inside] layer: meta, type: Rule, name: StillInside\n",
            "```\n",
            "````\n",
            "/// [@st-outside] layer: meta, type: Rule, name: Outside\n"
        );
        let result = AnnotationParser::parse(content, "docs/spec.md").unwrap();
        let ids: Vec<&str> = result.annotations.iter().map(|a| a.id.as_str()).collect();
        assert_eq!(ids, vec!["st-outside"]);
        assert_eq!(result.annotations[0].line, LineNumber::new(6));
    }

    #[test]
    fn test_ignores_inline_code_and_string_literals_and_prose() {
        let content = concat!(
            "let s = \"/// [@st-in-string] layer: meta, type: Philosophy, name: InString\";\n",
            "- Example: `/// [@st-in-inline-code] layer: abstract, type: Structure, name: InInline`\n",
            "/// Mentioning [@st-in-prose] layer: abstract, type: Structure, name: InProse\n",
            "    /// [@st-indented] layer: abstract, type: Structure, name: Indented\n",
            "[@st-bare] layer: meta, type: Rule, name: Bare\n"
        );
        let result = AnnotationParser::parse(content, "src/example.rs").unwrap();
        let ids: Vec<&str> = result.annotations.iter().map(|a| a.id.as_str()).collect();
        assert_eq!(ids, vec!["st-indented", "st-bare"]);
        assert_eq!(result.annotations[0].line, LineNumber::new(4));
        assert_eq!(result.annotations[1].line, LineNumber::new(5));
    }

    #[test]
    fn test_handles_crlf_line_endings_and_empty_input() {
        let empty = AnnotationParser::parse("", "src/empty.rs").unwrap();
        assert!(empty.annotations.is_empty());
        assert!(empty.warnings.is_empty());

        let crlf = "line1\r\n/// [@st-crlf] layer: abstract, type: Structure, name: Crlf\r\n";
        let result = AnnotationParser::parse(crlf, "src/crlf.rs").unwrap();
        assert_eq!(result.annotations.len(), 1);
        assert_eq!(result.annotations[0].id, "st-crlf");
        assert_eq!(result.annotations[0].name, "Crlf");
        assert_eq!(result.annotations[0].line, LineNumber::new(2));
    }

    #[test]
    fn test_parses_required_and_optional_fields_in_arbitrary_order() {
        let content = concat!(
            "/// [@st-reordered] name: Reordered, type: Func, layer: spec-detail\n",
            "/// [@st-full] status: Planned, links: [@a, @b], artifact: src/x.rs, name: Full, type: DomainEntity, layer: implementation\n",
            "/// [@st-empty-links] layer: abstract, type: Structure, name: EmptyLinks, links: []\n"
        );
        let result = AnnotationParser::parse(content, "src/order.rs").unwrap();
        assert!(result.warnings.is_empty(), "got warnings: {:?}", result.warnings);
        assert_eq!(result.annotations.len(), 3);

        assert_eq!(result.annotations[0].id, "st-reordered");
        assert_eq!(result.annotations[0].layer, "spec-detail");
        assert_eq!(result.annotations[0].annotation_type, "Func");
        assert_eq!(result.annotations[0].name, "Reordered");

        assert_eq!(result.annotations[1].id, "st-full");
        assert_eq!(result.annotations[1].layer, "implementation");
        assert_eq!(result.annotations[1].annotation_type, "DomainEntity");
        assert_eq!(result.annotations[1].name, "Full");
        assert_eq!(result.annotations[1].links, vec!["@a", "@b"]);
        assert_eq!(result.annotations[1].status.as_deref(), Some("Planned"));
        assert_eq!(result.annotations[1].artifact.as_deref(), Some("src/x.rs"));

        assert_eq!(result.annotations[2].id, "st-empty-links");
        assert!(result.annotations[2].links.is_empty());
    }

    #[test]
    fn test_emits_warning_for_missing_closing_bracket_or_empty_id() {
        let content = concat!(
            "/// [@st-unclosed layer: meta, type: Rule, name: Unclosed\n",
            "/// [@] layer: meta, type: Rule, name: EmptyId\n",
            "/// [@   ] layer: meta, type: Rule, name: WhitespaceId\n"
        );
        let result = AnnotationParser::parse(content, "src/bad_tags.rs").unwrap();
        assert!(result.annotations.is_empty());
        assert_eq!(result.warnings.len(), 3);

        assert_eq!(result.warnings[0].line, LineNumber::new(1));
        assert!(result.warnings[0].message.contains("missing closing ']'"));
        assert_eq!(
            result.warnings[0].raw_text,
            "/// [@st-unclosed layer: meta, type: Rule, name: Unclosed"
        );

        assert_eq!(result.warnings[1].line, LineNumber::new(2));
        assert!(result.warnings[1].message.contains("empty annotation ID"));

        assert_eq!(result.warnings[2].line, LineNumber::new(3));
        assert!(result.warnings[2].message.contains("empty annotation ID"));
    }

    #[test]
    fn test_emits_warning_for_missing_or_empty_required_fields() {
        let content = concat!(
            "/// [@st-no-name] layer: abstract, type: Structure\n",
            "/// [@st-no-layer] type: Structure, name: NoLayer\n",
            "/// [@st-no-type] layer: abstract, name: NoType\n",
            "/// [@st-no-fields]\n",
            "/// [@st-empty-name] layer: abstract, type: Structure, name: \n"
        );
        let result = AnnotationParser::parse(content, "src/missing.rs").unwrap();
        assert!(result.annotations.is_empty());

        // Line 1: missing [name]
        assert!(result.warnings.iter().any(|w| {
            w.line == LineNumber::new(1) && w.message.contains("Missing required field(s) [name]")
        }));
        // Line 2: missing [layer]
        assert!(result.warnings.iter().any(|w| {
            w.line == LineNumber::new(2) && w.message.contains("Missing required field(s) [layer]")
        }));
        // Line 3: missing [type]
        assert!(result.warnings.iter().any(|w| {
            w.line == LineNumber::new(3) && w.message.contains("Missing required field(s) [type]")
        }));
        // Line 4: missing all three
        assert!(result.warnings.iter().any(|w| {
            w.line == LineNumber::new(4)
                && w.message
                    .contains("Missing required field(s) [layer, type, name]")
        }));
        // Line 5: empty value for name + missing required field
        assert!(result.warnings.iter().any(|w| {
            w.line == LineNumber::new(5)
                && w.message.contains("Empty value for required field 'name'")
        }));
    }

    #[test]
    fn test_emits_warning_for_malformed_links_unknown_keys_duplicates_and_bad_tokens() {
        let content = concat!(
            "/// [@st-bad-links1] layer: abstract, type: Structure, name: L1, links: [@a, @b\n",
            "/// [@st-bad-links2] layer: abstract, type: Structure, name: L2, links: @a\n",
            "/// [@st-typo] layer: implementation, type: Structure, name: Typo, statu: Completed\n",
            "/// [@st-dup] layer: meta, layer: abstract, type: Rule, name: Dup\n",
            "/// [@st-no-colon] layer: meta, broken_field, type: Rule, name: NoColon\n",
            "/// [@st-trailing-comma] layer: meta, type: Rule, name: Trailing,\n",
            "/// [@st-empty-opt] layer: implementation, type: Structure, name: EmptyOpt, status: , artifact: \n"
        );
        let result = AnnotationParser::parse(content, "src/warnings.rs").unwrap();
        // All 7 lines still have valid layer, type, and name so annotations are recovered
        assert_eq!(result.annotations.len(), 7);

        assert!(result.warnings.iter().any(|w| {
            w.line == LineNumber::new(1) && w.message.contains("Malformed 'links' field")
        }));
        assert!(result.warnings.iter().any(|w| {
            w.line == LineNumber::new(2) && w.message.contains("Malformed 'links' field")
        }));
        assert!(result.warnings.iter().any(|w| {
            w.line == LineNumber::new(3) && w.message.contains("Unknown annotation field 'statu'")
        }));
        assert!(result.warnings.iter().any(|w| {
            w.line == LineNumber::new(4) && w.message.contains("Duplicate field 'layer'")
        }));
        assert!(result.warnings.iter().any(|w| {
            w.line == LineNumber::new(5) && w.message.contains("expected 'key: value'")
        }));
        assert!(result.warnings.iter().any(|w| {
            w.line == LineNumber::new(6) && w.message.contains("empty field segment")
        }));
        assert!(result.warnings.iter().any(|w| {
            w.line == LineNumber::new(7) && w.message.contains("Empty value for field 'status'")
        }));
        assert!(result.warnings.iter().any(|w| {
            w.line == LineNumber::new(7) && w.message.contains("Empty value for field 'artifact'")
        }));
    }

    #[test]
    fn test_recovers_valid_annotations_when_mixed_with_malformed_lines() {
        let content = concat!(
            "/// [@st-ok-1] layer: meta, type: Philosophy, name: Ok1\n",
            "/// [@st-bad] layer: abstract\n",
            "/// [@st-ok-2] name: Ok2, layer: abstract, type: Structure\n"
        );
        let result = AnnotationParser::parse(content, "src/mixed.rs").unwrap();
        let ids: Vec<&str> = result.annotations.iter().map(|a| a.id.as_str()).collect();
        assert_eq!(ids, vec!["st-ok-1", "st-ok-2"]);
        assert_eq!(result.annotations[0].line, LineNumber::new(1));
        assert_eq!(result.annotations[1].line, LineNumber::new(3));
        assert_eq!(result.warnings.len(), 1);
        assert_eq!(result.warnings[0].line, LineNumber::new(2));
        assert_eq!(result.warnings[0].source_file, "src/mixed.rs");
    }

    #[test]
    fn test_parses_quoted_values_with_commas_and_escapes() {
        let content = concat!(
            "/// [@st-comma] layer: abstract, type: Structure, name: \"Auth, Session, and Tokens\", links: [@st-x]\n",
            "/// [@st-escaped] layer: implementation, type: DomainEntity, name: \"Quote \\\"Hero\\\"\", artifact: \"src/a, b/hero.rs\", status: Completed\n"
        );
        let result = AnnotationParser::parse(content, "src/quoted.rs").unwrap();
        assert!(result.warnings.is_empty(), "got warnings: {:?}", result.warnings);
        assert_eq!(result.annotations.len(), 2);

        assert_eq!(result.annotations[0].name, "Auth, Session, and Tokens");
        assert_eq!(result.annotations[0].links, vec!["@st-x"]);

        assert_eq!(result.annotations[1].name, "Quote \"Hero\"");
        assert_eq!(
            result.annotations[1].artifact.as_deref(),
            Some("src/a, b/hero.rs")
        );
        assert_eq!(result.annotations[1].status.as_deref(), Some("Completed"));
    }

    #[test]
    fn test_emits_warning_for_unterminated_or_empty_quoted_values() {
        let content = concat!(
            "/// [@st-unterm] layer: abstract, type: Structure, name: \"Unclosed\n",
            "/// [@st-empty-quote] layer: abstract, type: Structure, name: \"   \"\n",
            "/// [@st-trailing-quote] layer: abstract, type: Structure, name: \"Foo\" bar\n"
        );
        let result = AnnotationParser::parse(content, "src/bad_quotes.rs").unwrap();
        assert!(result.annotations.is_empty());

        assert!(result.warnings.iter().any(|w| {
            w.line == LineNumber::new(1) && w.message.contains("unterminated quoted string")
        }));
        assert!(result.warnings.iter().any(|w| {
            w.line == LineNumber::new(2)
                && w.message.contains("Empty value for required field 'name'")
        }));
        assert!(result.warnings.iter().any(|w| {
            w.line == LineNumber::new(3) && w.message.contains("Malformed field 'name'")
        }));
    }
}
