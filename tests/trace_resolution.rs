use serde_json::Value;
use std::process::Command;

const CONFIG: &str = "tests/resources/trace_layers/config.toml";

fn cli_json(binary: &str, config: &str, args: &[&str]) -> Value {
    let output = Command::new(binary)
        .args(["--format", "json", "--config", config])
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn explicit_links_require_the_resolved_layer_but_matching_ids_can_cross_layers() {
    let show = cli_json(
        env!("CARGO_BIN_EXE_show"),
        CONFIG,
        &["--target", "all", "--view", "detail"],
    );
    let traces = show["traces"].as_array().unwrap();
    assert_eq!(traces.len(), 2);
    let shared_id = traces.iter().find(|t| t["src_id"] == "T").unwrap();
    assert_eq!(shared_id["src_layer"], "Implementation");
    assert_eq!(shared_id["dst_id"], "T");
    assert_eq!(shared_id["dst_layer"], "Meta");

    let explicit = traces.iter().find(|t| t["src_id"] == "D").unwrap();
    assert_eq!(explicit["src_line"], 3);
    assert_eq!(explicit["dst_id"], "C");

    let check = cli_json(
        env!("CARGO_BIN_EXE_check"),
        CONFIG,
        &["--strict", "--warn-unimplemented"],
    );
    let issues = check["issues"].as_array().unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0]["type"], "unimplemented_spec");
    assert_eq!(issues[0]["spec_id"], "D");
    assert_eq!(issues[0]["line"], 2);
}

#[test]
fn sample_keeps_both_implementations_for_each_document_location() {
    let show = cli_json(
        env!("CARGO_BIN_EXE_show"),
        "src/config/simple_sample.toml",
        &["--target", "all", "--view", "detail"],
    );
    let edges: Vec<_> = show["traces"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|t| t["dst_id"] == "ss-simple-sample" && t["kind"] == "Implements")
        .map(|t| {
            assert_eq!(t["src_space"], "code");
            assert_eq!(t["dst_space"], "document");
            assert_eq!(t["src_line"], 1);
            assert_eq!(t["dst_line"], 3);
            let root = "samples/rust/simple_sample/";
            (
                t["src_file"].as_str().unwrap().strip_prefix(root).unwrap(),
                t["dst_file"].as_str().unwrap().strip_prefix(root).unwrap(),
            )
        })
        .collect();
    assert_eq!(
        edges,
        [
            ("src/application.rs", "README.md"),
            ("src/application.rs", "spec.md"),
            ("src/domain.rs", "README.md"),
            ("src/domain.rs", "spec.md"),
        ]
    );
}
