//! End-to-end CLI tests.
//!
//! These tests drive the `lumaui` binary against scratch projects built into
//! per-test temp directories. They exist to keep the operator-visible surface
//! aligned with the parser/semantic/backend stages.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

fn cli_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_lumaui-cli"))
}

fn unique_dir(prefix: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let pid = std::process::id();
    let mut p = std::env::temp_dir();
    p.push(format!("lumaui-cli-test-{prefix}-{pid}-{n}"));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).expect("create temp dir");
    p
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(cli_binary())
        .args(args)
        .output()
        .expect("invoke lumaui-cli")
}

fn normalize_generated_prefix(input: &str) -> String {
    input.replace("minimal_", "lumaui_")
}

#[test]
fn init_then_doctor_validate_build_succeeds_end_to_end() {
    let dir = unique_dir("init-build");
    let project = dir.join("proj");

    let out = run(&["init", project.to_str().unwrap(), "--name", "demo"]);
    assert!(out.status.success(), "init failed: {:?}", out);

    // starter project must include lumaui.toml + ui sources.
    assert!(project.join("lumaui.toml").exists());
    assert!(project.join("ui/screens/main.lui").exists());
    assert!(project.join("ui/styles/theme.lus").exists());

    let out = run(&["doctor", project.to_str().unwrap()]);
    assert!(out.status.success(), "doctor failed: {:?}", out);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("project_name=demo"), "stderr: {stderr}");

    let out = run(&["validate", project.to_str().unwrap()]);
    assert!(out.status.success(), "validate failed: {:?}", out);

    let out = run(&["build", project.to_str().unwrap()]);
    assert!(out.status.success(), "build failed: {:?}", out);
    let header = project.join("generated/ui/screens/main_gen.h");
    let source = project.join("generated/ui/screens/main_gen.c");
    assert!(header.exists(), "missing {}", header.display());
    assert!(source.exists(), "missing {}", source.display());

    let body = fs::read_to_string(&source).unwrap();
    assert!(body.contains("lumaui_screen_main_create"));
    assert!(body.contains("lumaui_event_open_settings"));
    assert!(body.contains("lumaui-region: compiler-owned begin"));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn build_minimal_example_matches_snapshot_for_home_screen() {
    let mut workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    workspace_root.pop();
    let example = workspace_root.join("examples/minimal");

    let out = Command::new(cli_binary())
        .args(["build", example.to_str().unwrap()])
        .output()
        .expect("invoke lumaui-cli");
    assert!(out.status.success(), "build failed: {:?}", out);

    let generated_source = example.join("generated/ui/screens/home_gen.c");
    let generated_header = example.join("generated/ui/screens/home_gen.h");
    let source = fs::read_to_string(&generated_source).expect("generated source missing");
    let header = fs::read_to_string(&generated_header).expect("generated header missing");

    let expected_source = fs::read_to_string(workspace_root.join("tests/snapshots/minimal_screen.c"))
        .expect("expected source snapshot missing");
    let expected_header = fs::read_to_string(workspace_root.join("tests/snapshots/minimal_screen.h"))
        .expect("expected header snapshot missing");

    assert_eq!(
        normalize_generated_prefix(&source),
        expected_source,
        "frontend-generated source drifted from the canonical backend snapshot"
    );
    assert_eq!(
        normalize_generated_prefix(&header),
        expected_header,
        "frontend-generated header drifted from the canonical backend snapshot"
    );
}

#[test]
fn validate_rejects_unsupported_widget_fixture() {
    let mut workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    workspace_root.pop();

    let dir = unique_dir("invalid-widget");
    let project = dir.join("proj");
    fs::create_dir_all(project.join("ui/screens")).unwrap();
    fs::create_dir_all(project.join("ui/styles")).unwrap();
    fs::write(
        project.join("lumaui.toml"),
        "project_name = \"invalid\"\nlvgl_version = \"9.x\"\n",
    )
    .unwrap();
    fs::copy(
        workspace_root.join("tests/fixtures/unsupported_widget.lui"),
        project.join("ui/screens/main.lui"),
    )
    .unwrap();

    let out = run(&["validate", project.to_str().unwrap()]);
    assert!(!out.status.success(), "validate should fail");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("unsupported widget"),
        "stderr should mention unsupported widget: {stderr}"
    );

    let _ = fs::remove_dir_all(&dir);
}
