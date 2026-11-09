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
        "nokori-test-create-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock before epoch")
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

fn today() -> String {
    let output = Command::new("date")
        .arg("+%Y%m%d")
        .output()
        .expect("failed to run date");
    String::from_utf8(output.stdout)
        .expect("date printed non-UTF-8")
        .trim()
        .to_owned()
}

fn items(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir.join("backlog"))
        .expect("failed to read backlog dir")
        .map(|e| e.expect("failed to read entry").file_name())
        .map(|n| n.to_string_lossy().into_owned())
        .filter(|n| n != "README.md")
        .collect();
    names.sort();
    names
}

#[test]
fn create_writes_the_bug_template() {
    let dir = temp_dir("bug");
    assert!(nokori(&dir, &["init"]).status.success());

    let output = nokori(&dir, &["create", "bug", "empty-input"]);
    assert!(output.status.success(), "{output:?}");

    let expected = format!("{}-bug-empty-input.md", today());
    assert_eq!(items(&dir), vec![expected.clone()]);

    let content = std::fs::read_to_string(dir.join("backlog").join(&expected))
        .expect("failed to read created item");
    assert!(content.starts_with("# Bug:"), "{content}");
    assert!(content.contains("## Reproduction"), "{content}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn create_refuses_an_existing_file() {
    let dir = temp_dir("exists");
    assert!(nokori(&dir, &["init"]).status.success());
    assert!(nokori(&dir, &["create", "bug", "twice"]).status.success());

    let output = nokori(&dir, &["create", "bug", "twice"]);
    assert!(!output.status.success(), "create should have failed");
    assert_eq!(items(&dir).len(), 1);

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn create_refuses_an_unknown_kind() {
    let dir = temp_dir("kind");
    assert!(nokori(&dir, &["init"]).status.success());

    let output = nokori(&dir, &["create", "rfc", "x"]);
    assert!(!output.status.success(), "create should have failed");
    assert!(items(&dir).is_empty());

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn create_refuses_a_slug_that_is_not_a_filename() {
    let dir = temp_dir("slug");
    assert!(nokori(&dir, &["init"]).status.success());

    let output = nokori(&dir, &["create", "bug", "a b"]);
    assert!(!output.status.success(), "create should have failed");
    assert!(items(&dir).is_empty());

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn create_fails_without_a_backlog() {
    let dir = temp_dir("no-backlog");

    let output = nokori(&dir, &["create", "bug", "x"]);
    assert!(!output.status.success(), "create should have failed");
    assert!(!dir.join("backlog").exists(), "backlog/ was created");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}
