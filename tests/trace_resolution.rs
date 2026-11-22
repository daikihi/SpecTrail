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
    assert_eq!(traces.len(), 1);
    assert_eq!(traces[0]["src_id"], "T");
    assert_eq!(traces[0]["src_layer"], "Implementation");
    assert_eq!(traces[0]["dst_id"], "T");
    assert_eq!(traces[0]["dst_layer"], "Meta");

    let check = cli_json(
        env!("CARGO_BIN_EXE_check"),
        CONFIG,
        &["--strict", "--warn-unimplemented"],
    );
    let issues = check["issues"].as_array().unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0]["type"], "unimplemented_spec");
    assert_eq!(issues[0]["spec_id"], "D");
}
