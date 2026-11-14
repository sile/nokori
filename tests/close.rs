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

/// Creates a second item, so that there is something for a link to point at.
fn add_item(dir: &Path, name: &str) {
    let path = dir.join("backlog").join(name);
    std::fs::write(&path, format!("# Bug: {name}\n\n## Outcome\n\nFixed.\n"))
        .expect("failed to write item");
    git(dir, &["add", "."]);
    git(dir, &["commit", "--quiet", "-m", "add second item"]);
}

#[test]
fn close_refuses_a_link_into_backlog() {
    // The link is in `Outcome`, and the section above it is left bare: a link
    // in any section is refused, and the line the error names says which one
    // was read.
    let body = "# Bug: x\n\n## Summary\n\nIt was declined.\n\n## Outcome\n\nDeclined; see\n[bug-done-thing](20250101-bug-done-thing.md).\n";
    let (dir, item) = repo_with_item("link-open", body);
    add_item(&dir, "20250101-bug-done-thing.md");

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
fn close_rewrites_a_link_to_a_closed_item() {
    // Written in `backlog/`, the target `closed/x.md` names the closed item;
    // moved into `closed/`, the same text would name `closed/closed/x.md`. The
    // move respells it, which is the one write a closed file gets.
    let body = "# Bug: x\n\n## Summary\n\nAnswers\n[bug-done-thing](closed/20250101-bug-done-thing.md).\n\n## Outcome\n\nFixed in abc1234.\n";
    let (dir, item) = repo_with_item("link-closed", body);
    std::fs::create_dir_all(dir.join("closed")).expect("failed to create closed/");
    std::fs::write(
        dir.join("closed/20250101-bug-done-thing.md"),
        "# Bug: done-thing\n\n## Outcome\n\nFixed.\n",
    )
    .expect("failed to write closed item");
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "--quiet", "-m", "close another item"]);

    let output = nokori(&dir, &["close", &item]);
    assert!(output.status.success(), "{output:?}");
    assert!(!dir.join(&item).exists(), "the item was left in backlog/");

    let closed = dir.join("closed").join(item.trim_start_matches("backlog/"));
    let written = std::fs::read_to_string(&closed).expect("failed to read closed item");
    assert!(
        written.contains("[bug-done-thing](20250101-bug-done-thing.md)"),
        "{written}"
    );
    // The target is all that changed: the rest of the file is byte for byte
    // what it was, so the diff of the moving commit shows the link and nothing
    // else.
    assert_eq!(written, body.replace("closed/20250101", "20250101"));

    // The writer does not see the file change, so the rewrite is reported.
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("`closed/20250101-bug-done-thing.md` -> `20250101-bug-done-thing.md`"),
        "{stdout}"
    );
    assert!(stdout.contains("line 6"), "{stdout}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn close_rewrites_a_link_however_it_was_spelled() {
    // `../closed/x.md` names the same file from `closed/` as `closed/x.md` names
    // from `backlog/`, but it is not how a file in `closed/` names its
    // neighbour, so it is respelled as well.
    let body = "# Bug: x\n\n## Summary\n\nAnswers\n[bug-done-thing](../closed/20250101-bug-done-thing.md).\n\n## Outcome\n\nFixed in abc1234.\n";
    let (dir, item) = repo_with_item("link-parent", body);
    std::fs::create_dir_all(dir.join("closed")).expect("failed to create closed/");
    std::fs::write(
        dir.join("closed/20250101-bug-done-thing.md"),
        "# Bug: done-thing\n\n## Outcome\n\nFixed.\n",
    )
    .expect("failed to write closed item");
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "--quiet", "-m", "close another item"]);

    let output = nokori(&dir, &["close", &item]);
    assert!(output.status.success(), "{output:?}");

    let closed = dir.join("closed").join(item.trim_start_matches("backlog/"));
    let written = std::fs::read_to_string(&closed).expect("failed to read closed item");
    assert!(
        written.contains("[bug-done-thing](20250101-bug-done-thing.md)"),
        "{written}"
    );

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn close_does_not_rewrite_what_is_already_in_closed() {
    // A closed item is not edited, so a link that was wrong when it landed is
    // read as the record of what that `close` produced.
    let dir = temp_dir("no-sweep");
    std::fs::create_dir_all(dir.join("backlog")).expect("failed to create backlog/");
    std::fs::create_dir_all(dir.join("closed")).expect("failed to create closed/");
    std::fs::write(dir.join("backlog/README.md"), "# Backlog\n").expect("failed to write");
    let broken =
        "# Bug: old\n\n## Outcome\n\nSee\n[bug-nowhere](closed/closed/20250101-bug-nowhere.md).\n";
    std::fs::write(dir.join("closed/20240101-bug-old.md"), broken)
        .expect("failed to write closed item");
    let body = "# Bug: x\n\n## Summary\n\nDone.\n\n## Outcome\n\nFixed.\n";
    std::fs::write(dir.join("backlog/20250101-bug-new.md"), body).expect("failed to write item");
    git(&dir, &["add", "."]);
    git(&dir, &["commit", "--quiet", "-m", "setup"]);

    let output = nokori(&dir, &["close", "backlog/20250101-bug-new.md"]);
    assert!(output.status.success(), "{output:?}");

    let untouched = std::fs::read_to_string(dir.join("closed/20240101-bug-old.md"))
        .expect("failed to read closed item");
    assert_eq!(untouched, broken, "a closed item was rewritten");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn close_refuses_a_link_that_does_not_resolve() {
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
fn close_ignores_a_link_that_is_not_an_item() {
    // `close` checks a link to an item and nothing else: a link to a file of
    // the repository, or out of it, is the writer's business.
    let body = "# Bug: x\n\n## Summary\n\nSee [src](src/main.rs) and\n[the docs](https://example.com/none).\n\n## Outcome\n\nFixed in abc1234.\n";
    let (dir, item) = repo_with_item("link-other", body);

    let output = nokori(&dir, &["close", &item]);
    assert!(output.status.success(), "{output:?}");

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
