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
        "nokori-test-check-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock before epoch")
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

/// Sets up a backlog with one item, written out with `body`. Nothing is
/// committed: an item being written is not committed yet, and `check` says
/// nothing about the working tree.
fn repo_with_item(name: &str, body: &str) -> (std::path::PathBuf, String) {
    let dir = temp_dir(name);
    assert!(nokori(&dir, &["init"]).status.success());
    assert!(
        nokori(&dir, &["create", "bug", "half-written"])
            .status
            .success()
    );

    let name = std::fs::read_dir(dir.join("backlog/open"))
        .expect("failed to read the open dir")
        .map(|e| e.expect("failed to read entry").file_name())
        .map(|n| n.to_string_lossy().into_owned())
        .find(|n| n != "README.md")
        .expect("create made no item");
    let item = format!("backlog/open/{name}");
    std::fs::write(dir.join(&item), body).expect("failed to write item");
    (dir, item)
}

/// An item that has every section written but no `## Outcome`, which is what
/// an open item looks like: the outcome is written at the close.
const WRITTEN: &str = "# Bug: empty input panics\n\n## Summary\n\nIt panics.\n\n## Outcome\n\n";

/// The same, with the `## Outcome` comment still in place. That is how an item
/// looks for most of its life: the comment is only deleted in the edit that
/// writes the outcome, which is the close.
const WRITTEN_WITH_OUTCOME_COMMENT: &str = "# Bug: empty input panics\n\n## Summary\n\nIt panics.\n\n## Outcome\n\n<!-- NOKORI_TODO: say what happened, when the item is closed. -->\n";

#[test]
fn check_says_a_written_item_is_written() {
    let (dir, item) = repo_with_item("written", WRITTEN);

    let output = nokori(&dir, &["check", &item]);
    assert!(output.status.success(), "{output:?}");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("is written"), "{stdout}");
    assert!(stdout.contains("nokori close"), "{stdout}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn check_does_not_report_the_outcome_comment() {
    let (dir, item) = repo_with_item("outcome-comment", WRITTEN_WITH_OUTCOME_COMMENT);

    let output = nokori(&dir, &["check", &item]);
    assert!(output.status.success(), "{output:?}");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("can be closed"),
        "a written item should be told it can be closed: {stdout}"
    );

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn check_reports_a_todo_and_names_the_section() {
    let body =
        "# Bug: x\n\n## Summary\n\n<!-- NOKORI_TODO: say what is broken. -->\n\n## Outcome\n\n";
    let (dir, item) = repo_with_item("todo", body);

    let output = nokori(&dir, &["check", &item]);
    assert!(!output.status.success(), "check should have failed");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("NOKORI_TODO:"), "{stderr}");
    assert!(stderr.contains("## Summary"), "{stderr}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn check_ignores_a_marker_written_in_prose() {
    let body = "# Bug: x\n\n## Summary\n\n`close` refuses a file that has any `NOKORI_TODO:` left.\n\n## Outcome\n\n";
    let (dir, item) = repo_with_item("prose", body);

    let output = nokori(&dir, &["check", &item]);
    assert!(output.status.success(), "{output:?}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn check_reports_a_todo_before_the_outcome_comment() {
    // Both comments are there; the one to report is the one that is work.
    let body = "# Bug: x\n\n## Summary\n\n<!-- NOKORI_TODO: say what is broken. -->\n\n\
                ## Outcome\n\n<!-- NOKORI_TODO: say what happened. -->\n";
    let (dir, item) = repo_with_item("todo-before-outcome", body);

    let output = nokori(&dir, &["check", &item]);
    assert!(!output.status.success(), "check should have failed");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("The comment is in `## Summary`"),
        "{stderr}"
    );
    assert!(
        !stderr.contains("The comment is in `## Outcome`"),
        "the outcome is not reported: {stderr}"
    );

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn check_reports_a_link_that_does_not_resolve() {
    let body = "# Bug: x\n\n## Summary\n\nAnswers\n[bug-never-written](20250101-bug-never-written.md).\n\n## Outcome\n\n";
    let (dir, item) = repo_with_item("broken-link", body);

    let output = nokori(&dir, &["check", &item]);
    assert!(!output.status.success(), "check should have failed");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("line 6"), "{stderr}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn check_allows_a_link_into_open() {
    // The direction is only forbidden for a file in `backlog/closed/`, and the
    // item is not there yet.
    let (dir, item) = repo_with_item("link-open", WRITTEN);
    let neighbour = dir.join("backlog/open/20250101-bug-still-open.md");
    std::fs::write(&neighbour, "# Bug: y\n").expect("failed to write neighbour");
    std::fs::write(
        dir.join(&item),
        "# Bug: x\n\n## Summary\n\nAnswers\n[bug-still-open](20250101-bug-still-open.md).\n\n## Outcome\n\n",
    )
    .expect("failed to write item");

    let output = nokori(&dir, &["check", &item]);
    assert!(output.status.success(), "{output:?}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn check_refuses_a_path_outside_open() {
    let (dir, item) = repo_with_item("outside", WRITTEN);

    let output = nokori(&dir, &["check", "README.md"]);
    assert!(!output.status.success(), "check should have failed");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("backlog/open/"), "{stderr}");
    assert!(dir.join(&item).is_file(), "the item was moved");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}
