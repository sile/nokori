use std::path::Path;
use std::process::Command;

fn nokori(dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_nokori"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("failed to run nokori")
}

fn git(dir: &Path, args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("failed to run git");
    assert!(output.status.success(), "git {args:?}: {output:?}");
}

fn temp_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "nokori-test-close-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock before epoch")
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("failed to create temp dir");
    git(&dir, &["init", "--quiet"]);
    git(&dir, &["config", "user.email", "test@example.com"]);
    git(&dir, &["config", "user.name", "Test"]);
    dir
}

/// Sets up a repository with one item, written out and committed.
fn repo_with_item(name: &str, body: &str) -> (std::path::PathBuf, String) {
    let dir = temp_dir(name);
    assert!(nokori(&dir, &["init"]).status.success());
    let output = nokori(&dir, &["create", "bug", "done-thing"]);
    assert!(output.status.success(), "{output:?}");

    let name = std::fs::read_dir(dir.join("backlog"))
        .expect("failed to read backlog dir")
        .map(|e| e.expect("failed to read entry").file_name())
        .map(|n| n.to_string_lossy().into_owned())
        .find(|n| n != "README.md")
        .expect("create made no item");
    let item = format!("backlog/{name}");
    std::fs::write(dir.join(&item), body).expect("failed to write item");

    git(&dir, &["add", "."]);
    git(&dir, &["commit", "--quiet", "-m", "add item"]);
    (dir, item)
}

const FINISHED: &str =
    "# Bug: empty input panics\n\n## Summary\n\nIt panics.\n\n## Outcome\n\nFixed in abc1234.\n";

#[test]
fn close_moves_a_finished_item() {
    let (dir, item) = repo_with_item("moves", FINISHED);

    let output = nokori(&dir, &["close", &item]);
    assert!(output.status.success(), "{output:?}");
    assert!(!dir.join(&item).exists(), "item was left in backlog/");

    let closed = dir.join("closed").join(item.trim_start_matches("backlog/"));
    assert!(closed.is_file(), "{} is missing", closed.display());

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn close_refuses_an_item_with_a_todo() {
    let body =
        "# Bug: x\n\n## Summary\n\n<!-- TODO: say what is broken. -->\n\n## Outcome\n\nFixed.\n";
    let (dir, item) = repo_with_item("todo", body);

    let output = nokori(&dir, &["close", &item]);
    assert!(!output.status.success(), "close should have failed");
    assert!(dir.join(&item).exists(), "item was moved anyway");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("## Summary"), "{stderr}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn close_refuses_an_empty_outcome() {
    let body = "# Bug: x\n\n## Summary\n\nIt panics.\n\n## Outcome\n\n";
    let (dir, item) = repo_with_item("empty-outcome", body);

    let output = nokori(&dir, &["close", &item]);
    assert!(!output.status.success(), "close should have failed");
    assert!(dir.join(&item).exists(), "item was moved anyway");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn close_refuses_an_uncommitted_edit() {
    let (dir, item) = repo_with_item("dirty", FINISHED);
    std::fs::write(dir.join(&item), "# Bug: x\n\n## Outcome\n\nEdited.\n")
        .expect("failed to edit item");

    let output = nokori(&dir, &["close", &item]);
    assert!(!output.status.success(), "close should have failed");
    assert!(dir.join(&item).exists(), "item was moved anyway");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn close_refuses_an_item_already_closed() {
    let (dir, item) = repo_with_item("twice", FINISHED);
    assert!(nokori(&dir, &["close", &item]).status.success());

    let output = nokori(&dir, &["close", &item]);
    assert!(!output.status.success(), "closing twice should fail");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("already closed"), "{stderr}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn close_refuses_a_path_outside_backlog() {
    let (dir, item) = repo_with_item("outside", FINISHED);

    let output = nokori(&dir, &["close", "README.md"]);
    assert!(!output.status.success(), "close should have failed");
    assert!(dir.join(&item).exists(), "the item was moved anyway");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}
