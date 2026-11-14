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

/// Creates an item and returns its contents, checking what every template has
/// to look like: the sections `close` relies on are there, and every marker is
/// the one `close` looks for.
fn create_and_read(name: &str, kind: &str, slug: &str, title: &str, sections: &[&str]) -> String {
    let dir = temp_dir(name);
    assert!(nokori(&dir, &["init"]).status.success());

    let output = nokori(&dir, &["create", kind, slug]);
    assert!(output.status.success(), "{output:?}");

    let expected = format!("{}-{kind}-{slug}.md", today());
    assert_eq!(items(&dir), vec![expected.clone()]);

    let content = std::fs::read_to_string(dir.join("backlog").join(&expected))
        .expect("failed to read created item");
    assert!(content.starts_with(title), "{content}");
    for section in sections {
        assert!(content.contains(section), "{section} in {content}");
    }
    assert!(content.contains("## Outcome"), "{content}");
    assert!(content.contains("NOKORI_TODO:"), "{content}");
    assert!(!content.contains("<!-- TODO:"), "{content}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
    content
}

#[test]
fn create_writes_the_bug_template() {
    create_and_read(
        "bug",
        "bug",
        "empty-input",
        "# Bug:",
        &[
            "## Summary",
            "## Reproduction",
            "## Observed behavior",
            "## Expected behavior",
            "## Notes",
        ],
    );
}

#[test]
fn create_writes_the_rfc_template() {
    create_and_read(
        "rfc",
        "rfc",
        "cache-the-parse",
        "# RFC:",
        &[
            "## Summary",
            "## Motivation",
            "## Design",
            "## Alternatives",
            "## Open questions",
        ],
    );
}

#[test]
fn create_writes_the_plan_template() {
    let content = create_and_read(
        "plan",
        "plan",
        "split-the-lexer",
        "# Plan:",
        &["## Goal", "## Items", "## Steps", "## Notes"],
    );
    // The steps are checkboxes, and the template shows what one looks like:
    // without an example the format would be left for the author to guess.
    assert!(content.contains("- [ ] "), "{content}");
}

#[test]
fn create_writes_the_feedback_template() {
    create_and_read(
        "feedback",
        "feedback",
        "faster-listing",
        "# Feedback:",
        &["## What I want"],
    );
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

    let output = nokori(&dir, &["create", "memo", "x"]);
    assert!(!output.status.success(), "create should have failed");
    assert!(items(&dir).is_empty());
    // The error has to say which kinds there are, since the reader guessed
    // wrong and `create` has no other way to tell them.
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("rfc, bug, plan, feedback"), "{stderr}");

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
