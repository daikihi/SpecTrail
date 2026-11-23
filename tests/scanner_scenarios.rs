pub mod scenario_a {
    use spec_trail::domains::services::annotation::parser::AnnotationParser;
    use spec_trail::domains::services::annotation::resolver::AnnotationResolver;

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
    use spec_trail::domains::services::annotation::parser::AnnotationParser;
    use spec_trail::domains::services::annotation::resolver::{
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

pub mod scenario_c {
    use spec_trail::use_case::check::{CheckSeverity, CheckUseCase, CheckUseCaseRequestDto};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_scenario_c_malformed_syntax_reported_by_check_use_case() {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("spectrail_scenario_c_{}", nanos));
        let src_dir = root.join("src");
        let doc_dir = root.join("docs");
        fs::create_dir_all(&src_dir).unwrap();
        fs::create_dir_all(&doc_dir).unwrap();

        fs::write(
            src_dir.join("lib.rs"),
            concat!(
                "/// [@st-good] layer: abstract, type: Structure, name: Good\n",
                "/// [@st-missing-name] layer: abstract, type: Structure\n",
                "/// [@st-typo] layer: implementation, type: Structure, name: Typo, statu: Completed\n",
                "let ignored = \"/// [@st-in-string] layer: abstract\";\n"
            ),
        )
        .unwrap();

        fs::write(
            doc_dir.join("spec.md"),
            concat!(
                "/// [@st-doc-ok] layer: meta, type: Philosophy, name: \"Spec, Model\"\n",
                "```rust\n",
                "/// [@st-in-fence] layer: abstract\n",
                "```\n"
            ),
        )
        .unwrap();

        let config_path = root.join("config.toml");
        fs::write(
            &config_path,
            format!(
                "[source]\nhead = \"{}\"\nextension = \".rs\"\n[document]\nhead = \"{}\"\nextension = \".md\"\n[annotation]\nprefix = \"@st-\"\n",
                src_dir.display(),
                doc_dir.display()
            ),
        )
        .unwrap();

        let uc = CheckUseCase::new();
        let response = uc
            .execute(CheckUseCaseRequestDto {
                strict: true,
                warn_unimplemented: false,
                config_path: Some(config_path.to_str().unwrap().to_string()),
            })
            .expect("check use case should execute cleanly");

        let _ = fs::remove_dir_all(&root);

        assert_eq!(response.errors_count, 2, "issues: {:?}", response.issues);
        assert_eq!(response.warnings_count, 0);
        assert!(response.issues.iter().all(|i| {
            i.severity == CheckSeverity::Error && i.issue_type == "invalid_annotation_format"
        }));
        assert_eq!(response.issues[0].line, 2);
        assert!(response.issues[0].message.contains("Missing required field(s) [name]"));
        assert_eq!(response.issues[1].line, 3);
        assert!(response.issues[1].message.contains("Unknown annotation field 'statu'"));
    }
}

pub mod scenario_d {
    use spec_trail::domains::models::implementation::{ImplementationStatus, ImplementationType};
    use spec_trail::domains::services::annotation::scanner::AnnotationScanner;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_scenario_d_reordered_quoted_implementation_annotation_end_to_end() {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("spectrail_scenario_d_{}", nanos));
        let src_dir = root.join("src");
        let doc_dir = root.join("docs");
        fs::create_dir_all(&src_dir).unwrap();
        fs::create_dir_all(&doc_dir).unwrap();

        fs::write(
            doc_dir.join("spec.md"),
            "/// [@st-spec-auth] name: \"Login, Logout, and Refresh\", type: Func, layer: spec-detail\n",
        )
        .unwrap();

        fs::write(
            src_dir.join("auth.rs"),
            "/// [@st-impl-auth] status: Completed, links: [@st-spec-auth], artifact: \"src/auth, v2.rs\", name: \"Auth Handler, v2\", type: DomainEntity, layer: implementation\n",
        )
        .unwrap();

        let scan = AnnotationScanner::scan(&src_dir, ".rs", &doc_dir, ".md")
            .expect("scanner should succeed");

        let _ = fs::remove_dir_all(&root);

        assert!(scan.warnings.is_empty(), "warnings: {:?}", scan.warnings);
        assert_eq!(scan.document_annotations.len(), 1);
        assert_eq!(
            scan.document_annotations[0].details[0].name.0,
            "Login, Logout, and Refresh"
        );

        assert_eq!(scan.code_annotations.len(), 1);
        let impl_anno = &scan.code_annotations[0].implementations[0];
        assert_eq!(impl_anno.name.0, "Auth Handler, v2");
        assert_eq!(impl_anno.r#type, Some(ImplementationType::DomainEntity));
        assert_eq!(impl_anno.status, Some(ImplementationStatus::Completed));
        assert_eq!(impl_anno.artifact.0, "src/auth, v2.rs");
        assert_eq!(impl_anno.links.len(), 1);
    }
}
