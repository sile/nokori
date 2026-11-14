use std::path::Path;
use std::process::Command;

fn nokori(dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_nokori"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("failed to run nokori")
}

fn temp_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "nokori-test-explain-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock before epoch")
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

#[test]
fn explain_prints_the_kinds() {
    let dir = temp_dir("kinds");

    let output = nokori(&dir, &["explain", "kinds"]);
    assert!(output.status.success(), "{output:?}");

    let stdout = String::from_utf8_lossy(&output.stdout);
    // One heading per kind `create` accepts, so the two lists cannot drift.
    let headings: Vec<&str> = stdout
        .lines()
        .filter(|line| !line.starts_with(' '))
        .collect();
    for kind in ["rfc", "bug", "plan", "feedback"] {
        assert!(headings.contains(&kind), "{kind} in {stdout}");
    }

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn explain_prints_the_flow() {
    let dir = temp_dir("flow");

    let output = nokori(&dir, &["explain", "flow"]);
    assert!(output.status.success(), "{output:?}");

    let stdout = String::from_utf8_lossy(&output.stdout);
    // The flow is about the two directories and the two commands, which is
    // what the reader arrives with a question about.
    for term in ["backlog/", "closed/", "nokori create", "nokori close"] {
        assert!(stdout.contains(term), "{term} in {stdout}");
    }

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn explain_prints_the_references() {
    let dir = temp_dir("references");

    let output = nokori(&dir, &["explain", "references"]);
    assert!(output.status.success(), "{output:?}");

    let stdout = String::from_utf8_lossy(&output.stdout);
    // The four cases of the table, which is what a writer has to get right and
    // what `close` refuses a file for getting wrong.
    for term in [
        "closed/20261114-plan-split-the-lexer.md",
        "not allowed",
        "What close checks",
    ] {
        assert!(stdout.contains(term), "{term} in {stdout}");
    }

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn explain_runs_without_a_backlog() {
    // A reader who has not run `init` is exactly who needs the flow explained.
    let dir = temp_dir("no-backlog");

    let output = nokori(&dir, &["explain", "flow"]);
    assert!(output.status.success(), "{output:?}");
    assert!(!dir.join("backlog").exists(), "backlog/ was created");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn explain_refuses_an_unknown_topic() {
    let dir = temp_dir("topic");

    let output = nokori(&dir, &["explain", "nope"]);
    assert!(!output.status.success(), "explain should have failed");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("kinds, flow, references"), "{stderr}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}
