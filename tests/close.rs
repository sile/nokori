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

    let name = std::fs::read_dir(dir.join("backlog/open"))
        .expect("failed to read the open dir")
        .map(|e| e.expect("failed to read entry").file_name())
        .map(|n| n.to_string_lossy().into_owned())
        .find(|n| n != "README.md")
        .expect("create made no item");
    let item = format!("backlog/open/{name}");
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

    let closed = dir
        .join("backlog/closed")
        .join(item.trim_start_matches("backlog/open/"));
    assert!(closed.is_file(), "{} is missing", closed.display());

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn close_refuses_an_item_with_a_todo() {
    let body = "# Bug: x\n\n## Summary\n\n<!-- NOKORI_TODO: say what is broken. -->\n\n## Outcome\n\nFixed.\n";
    let (dir, item) = repo_with_item("todo", body);

    let output = nokori(&dir, &["close", &item]);
    assert!(!output.status.success(), "close should have failed");
    assert!(dir.join(&item).exists(), "item was moved anyway");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("## Summary"), "{stderr}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn close_ignores_a_todo_written_in_the_body() {
    // The marker is `NOKORI_TODO:` so that a report may name a `TODO` in the
    // code without the item reading as unfinished.
    let body = "# Bug: x\n\n## Summary\n\nAn old `TODO:` was left in `src/lib.rs`,\nand nothing reads it as unfinished work.\n\n## Outcome\n\nFixed in abc1234.\n";
    let (dir, item) = repo_with_item("body-todo", body);

    let output = nokori(&dir, &["close", &item]);
    assert!(output.status.success(), "{output:?}");

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

/// Creates a second open item, so that there is something for a link to point
/// at, and says how the item being closed names it: a neighbour is a bare file
/// name.
fn add_open_item(dir: &Path, name: &str) {
    let path = dir.join("backlog/open").join(name);
    std::fs::write(&path, format!("# Bug: {name}\n\n## Outcome\n\nFixed.\n"))
        .expect("failed to write item");
    git(dir, &["add", "."]);
    git(dir, &["commit", "--quiet", "-m", "add second item"]);
}

/// Creates a second item that is already closed, so that a link to it keeps
/// resolving after the move.
fn add_closed_item(dir: &Path, name: &str) {
    let path = dir.join("backlog/closed").join(name);
    std::fs::write(&path, format!("# Bug: {name}\n\n## Outcome\n\nFixed.\n"))
        .expect("failed to write closed item");
    git(dir, &["add", "."]);
    git(dir, &["commit", "--quiet", "-m", "close another item"]);
}

#[test]
fn close_refuses_a_link_into_open() {
    // The link is in `Outcome`, and the section above it is left bare: a link
    // in any section is refused, and the line the error names says which one
    // was read. The target is written the way a neighbour is named, so it
    // would resolve after the move as well; the rule about citing an open item
    // is what refuses it, not the move.
    let body = "# Bug: x\n\n## Summary\n\nIt was declined.\n\n## Outcome\n\nDeclined; see\n[bug-done-thing](20250101-bug-done-thing.md).\n";
    let (dir, item) = repo_with_item("link-open", body);
    add_open_item(&dir, "20250101-bug-done-thing.md");

    let output = nokori(&dir, &["close", &item]);
    assert!(!output.status.success(), "close should have failed");
    assert!(dir.join(&item).exists(), "item was moved anyway");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("A closed item never cites an open one"),
        "{stderr}"
    );
    // The line the error names is the one the link is on, so that a writer
    // with several links knows which of them to fix.
    let line = body
        .lines()
        .position(|line| line.contains("]("))
        .map(|line| line + 1)
        .expect("the body has no link");
    assert!(
        stderr.contains(&format!("line {line}")),
        "line {line} in {stderr}"
    );

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn close_keeps_a_link_to_a_closed_item() {
    // `../closed/x.md` names the closed item from the open directory, and the
    // same text names it from the closed one: the two directories are at the
    // same depth, so the move does not touch the target.
    let body = "# Bug: x\n\n## Summary\n\nAnswers\n[bug-done-thing](../closed/20250101-bug-done-thing.md).\n\n## Outcome\n\nFixed in abc1234.\n";
    let (dir, item) = repo_with_item("link-closed", body);
    add_closed_item(&dir, "20250101-bug-done-thing.md");

    let output = nokori(&dir, &["close", &item]);
    assert!(output.status.success(), "{output:?}");
    assert!(
        !dir.join(&item).exists(),
        "the item was left in backlog/open/"
    );

    let closed = dir
        .join("backlog/closed")
        .join(item.trim_start_matches("backlog/open/"));
    let written = std::fs::read_to_string(&closed).expect("failed to read closed item");
    // Nothing in the file changed: the moving commit records the move alone.
    assert_eq!(written, body, "close edited the file it moved");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn close_refuses_a_target_that_means_two_files() {
    // `closed/x.md` read from `backlog/open/` is the closed item, and read from
    // `backlog/closed/` it is `backlog/closed/closed/x.md`. The move would
    // change what the link says, so it is refused rather than repaired.
    let body = "# Bug: x\n\n## Summary\n\nAnswers\n[bug-done-thing](closed/20250101-bug-done-thing.md).\n\n## Outcome\n\nFixed in abc1234.\n";
    let (dir, item) = repo_with_item("link-twice", body);
    add_closed_item(&dir, "20250101-bug-done-thing.md");

    let output = nokori(&dir, &["close", &item]);
    assert!(!output.status.success(), "close should have failed");
    assert!(dir.join(&item).exists(), "the item was moved anyway");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("line 6"), "{stderr}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn close_does_not_touch_what_is_already_in_closed() {
    // A closed item is not edited, so a link that was wrong when it landed is
    // read as the record of what that `close` produced.
    let dir = temp_dir("no-sweep");
    std::fs::create_dir_all(dir.join("backlog/open")).expect("failed to create backlog/open/");
    std::fs::create_dir_all(dir.join("backlog/closed")).expect("failed to create backlog/closed/");
    std::fs::write(dir.join("backlog/README.md"), "# Backlog\n").expect("failed to write");
    std::fs::write(dir.join("backlog/open/README.md"), "# Open\n").expect("failed to write");
    std::fs::write(dir.join("backlog/closed/README.md"), "# Closed\n").expect("failed to write");
    let broken =
        "# Bug: old\n\n## Outcome\n\nSee\n[bug-nowhere](closed/20250101-bug-nowhere.md).\n";
    std::fs::write(dir.join("backlog/closed/20240101-bug-old.md"), broken)
        .expect("failed to write closed item");
    let body = "# Bug: x\n\n## Summary\n\nDone.\n\n## Outcome\n\nFixed.\n";
    std::fs::write(dir.join("backlog/open/20250101-bug-new.md"), body)
        .expect("failed to write item");
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "--quiet", "-m", "setup"]);

    let output = nokori(&dir, &["close", "backlog/open/20250101-bug-new.md"]);
    assert!(output.status.success(), "{output:?}");

    let untouched = std::fs::read_to_string(dir.join("backlog/closed/20240101-bug-old.md"))
        .expect("failed to read closed item");
    assert_eq!(untouched, broken, "a closed item was rewritten");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn close_refuses_a_link_that_does_not_resolve() {
    // A bare file name resolves inside `backlog/open/` before the move and
    // inside `backlog/closed/` after it, which is the neighbour that should be
    // there once it closes -- but nothing is there, so the target is missing.
    let body = "# Bug: x\n\n## Summary\n\nAnswers\n[bug-never-written](20250101-bug-never-written.md).\n\n## Outcome\n\nFixed in abc1234.\n";
    let (dir, item) = repo_with_item("link-broken", body);

    let output = nokori(&dir, &["close", &item]);
    assert!(!output.status.success(), "close should have failed");
    assert!(dir.join(&item).exists(), "item was moved anyway");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("does not resolve"), "{stderr}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn close_checks_a_path_that_is_not_an_item() {
    // The scope is every path in the file, not only the ones that point at an
    // item: a check with a narrower scope is one the writer has to remember,
    // and there is nothing to gain from the narrower one now that `close` does
    // not rewrite targets. `../src/main.rs` reaches out of both directories at
    // the same depth, so it is kept.
    let body = "# Bug: x\n\n## Summary\n\nSee [src](../../src/main.rs) and\n[the docs](https://example.com/none).\n\n## Outcome\n\nFixed in abc1234.\n";
    let (dir, item) = repo_with_item("link-other", body);
    std::fs::create_dir_all(dir.join("src")).expect("failed to create src/");
    std::fs::write(dir.join("src/main.rs"), "fn main() {}\n").expect("failed to write src");
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "--quiet", "-m", "add src"]);

    let output = nokori(&dir, &["close", &item]);
    assert!(output.status.success(), "{output:?}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn close_refuses_a_link_to_a_file_that_is_not_there() {
    // The same refusal covers a path that is not an item: the target is
    // missing, and a closed file is not edited to repair it.
    let body = "# Bug: x\n\n## Summary\n\nSee [src](../../src/main.rs).\n\n## Outcome\n\nFixed in abc1234.\n";
    let (dir, item) = repo_with_item("link-missing", body);

    let output = nokori(&dir, &["close", &item]);
    assert!(!output.status.success(), "close should have failed");
    assert!(dir.join(&item).exists(), "the item was moved anyway");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn close_refuses_a_path_outside_open() {
    let (dir, item) = repo_with_item("outside", FINISHED);

    let output = nokori(&dir, &["close", "README.md"]);
    assert!(!output.status.success(), "close should have failed");
    assert!(dir.join(&item).exists(), "the item was moved anyway");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}
