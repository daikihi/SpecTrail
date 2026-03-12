pub mod scenario_a {
    use SpecTrail::domains::services::annotation::parser::AnnotationParser;
    use SpecTrail::domains::services::annotation::resolver::AnnotationResolver;

    #[test]
    fn test_scenario_a_malformed_annotation() {
        // Given ソースファイルに `[@st-foo] layer: INVALID, type: Rule, name: Foo` が含まれている
        let content = "[@st-foo] layer: INVALID, type: Rule, name: Foo";
        let parse_result = AnnotationParser::parse(content, "src/foo.rs").unwrap();

        // When 解決を実行する
        let resolve_result = AnnotationResolver::resolve(parse_result.annotations);

        // Then アノテーション `@st-foo` は出力に表示されない
        assert_eq!(resolve_result.annotations.len(), 0);

        // And 警告が生成される
        assert_eq!(resolve_result.warnings.len(), 1);
        assert!(
            resolve_result.warnings[0]
                .message
                .contains("Unknown layer 'INVALID'")
        );
    }
}

pub mod scenario_b {
    use SpecTrail::domains::services::annotation::parser::AnnotationParser;
    use SpecTrail::domains::services::annotation::resolver::{
        AnnotationResolver, ResolvedAnnotation,
    };

    #[test]
    fn test_scenario_b_broken_link() {
        // Given アノテーション `@st-bar` が `links: [@st-does-not-exist]` を持っている
        let content =
            "[@st-bar] layer: abstract, type: Page, name: Bar, links: [@st-does-not-exist]";
        let parse_result = AnnotationParser::parse(content, "src/bar.rs").unwrap();

        // When 解決を実行する
        let resolve_result = AnnotationResolver::resolve(parse_result.annotations);

        // Then `@st-bar` は出力に表示される
        assert_eq!(resolve_result.annotations.len(), 1);

        if let ResolvedAnnotation::Abstract(abs, _) = &resolve_result.annotations[0] {
            // And links は空（解決できなかったため除外される設計）
            assert_eq!(abs.links.len(), 0);
        } else {
            panic!("Expected Abstract annotation");
        }

        // And 警告が生成される
        assert_eq!(resolve_result.warnings.len(), 1);
        assert!(
            resolve_result.warnings[0]
                .message
                .contains("Link target '@st-does-not-exist' not found")
        );
    }
}
