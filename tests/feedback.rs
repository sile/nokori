use std::path::Path;
use std::process::{Command, Stdio};

fn nokori(dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_nokori"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("failed to run nokori")
}

fn nokori_with_stdin(dir: &Path, args: &[&str], stdin: &str) -> std::process::Output {
    use std::io::Write;

    let mut child = Command::new(env!("CARGO_BIN_EXE_nokori"))
        .args(args)
        .current_dir(dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to run nokori");
    child
        .stdin
        .as_mut()
        .expect("no stdin")
        .write_all(stdin.as_bytes())
        .expect("failed to write stdin");
    child.wait_with_output().expect("failed to wait for nokori")
}

fn temp_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "nokori-test-feedback-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock before epoch")
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

/// A directory holding a repository with a backlog, and nothing else.
fn repo_with_backlog(name: &str) -> std::path::PathBuf {
    let dir = temp_dir(name);
    let out = nokori(&dir, &["init"]);
    assert!(out.status.success(), "init failed: {out:?}");
    dir
}

fn written(dir: &Path) -> (String, String) {
    let open = dir.join("backlog/open");
    let mut names: Vec<String> = std::fs::read_dir(&open)
        .expect("failed to read the open dir")
        .map(|e| e.expect("failed to read entry").file_name())
        .map(|n| n.to_string_lossy().into_owned())
        .filter(|n| n != "README.md")
        .collect();
    names.sort();
    assert_eq!(names.len(), 1, "expected one item, found {names:?}");
    let content = std::fs::read_to_string(open.join(&names[0])).expect("failed to read the item");
    (names.remove(0), content)
}

#[test]
fn feedback_writes_the_words_into_the_named_repository() {
    let here = temp_dir("from");
    let there = repo_with_backlog("to");

    let out = nokori(
        &here,
        &[
            "feedback",
            there.to_str().expect("non-UTF-8 path"),
            "-m",
            "the search box loses my typing",
        ],
    );
    assert!(out.status.success(), "feedback failed: {out:?}");

    let (name, content) = written(&there);
    assert!(
        name.contains("-feedback-") && name.ends_with(".md"),
        "the name should carry the kind: {name}"
    );
    assert!(
        content.contains("the search box loses my typing"),
        "{content}"
    );
    assert!(
        !content.contains("NOKORI_TODO: what you think"),
        "{content}"
    );
}

#[test]
fn feedback_leaves_the_outcome_comment_and_no_title_line() {
    let there = repo_with_backlog("shape");
    let out = nokori(
        &there,
        &["feedback", ".", "-m", "something worth writing down"],
    );
    assert!(out.status.success(), "feedback failed: {out:?}");

    let (name, content) = written(&there);
    assert!(!content.contains("# Feedback:"), "no title line: {content}");
    assert!(content.starts_with("## Feedback\n"), "{content}");
    assert!(content.contains("## Outcome"), "{content}");
    assert!(name.contains("-feedback-"), "{name}");

    // The `Outcome` comment is still there, so the item is plainly unfinished.
    assert!(
        content.contains("NOKORI_TODO: the one section"),
        "{content}"
    );
}

#[test]
fn feedback_reads_the_words_from_a_file() {
    let there = repo_with_backlog("file");
    let words = there.join("words.md");
    std::fs::write(&words, "  from a file\n\n").expect("failed to write the words");

    let out = nokori(
        &there,
        &[
            "feedback",
            ".",
            "--file",
            words.to_str().expect("non-UTF-8 path"),
        ],
    );
    assert!(out.status.success(), "feedback failed: {out:?}");

    let (_, content) = written(&there);
    assert!(content.contains("\nfrom a file\n"), "trimmed: {content}");
}

#[test]
fn feedback_reads_the_words_from_standard_input() {
    let there = repo_with_backlog("stdin");
    let out = nokori_with_stdin(&there, &["feedback", "."], "said on a pipe\n");
    assert!(out.status.success(), "feedback failed: {out:?}");

    // Nothing is written for a pipe: that would land in whatever is reading.
    assert!(
        out.stderr.is_empty(),
        "stderr should be empty for a pipe: {out:?}"
    );

    let (_, content) = written(&there);
    assert!(content.contains("said on a pipe"), "{content}");
}

#[test]
fn feedback_refuses_empty_words_however_they_arrived() {
    let there = repo_with_backlog("empty");
    let blank = there.join("blank.md");
    std::fs::write(&blank, "   \n\n").expect("failed to write the blank file");

    for (args, stdin) in [
        (vec!["feedback", ".", "-m", ""], None),
        (
            vec!["feedback", ".", "-f", blank.to_str().expect("non-UTF-8")],
            None,
        ),
        (vec!["feedback", "."], Some("\n")),
    ] {
        let out = match stdin {
            Some(text) => nokori_with_stdin(&there, &args, text),
            None => nokori(&there, &args),
        };
        assert!(!out.status.success(), "expected a refusal for {args:?}");
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            stderr.contains("the words are empty"),
            "one error for all three: {stderr}"
        );
    }

    assert!(
        written_or_none(&there).is_none(),
        "nothing should have been written"
    );
}

fn written_or_none(dir: &Path) -> Option<String> {
    let open = dir.join("backlog/open");
    std::fs::read_dir(&open).ok()?.find_map(|e| {
        let name = e.ok()?.file_name().to_string_lossy().into_owned();
        (name != "README.md").then_some(name)
    })
}

#[test]
fn feedback_refuses_message_and_file_together() {
    let there = repo_with_backlog("both");
    let words = there.join("words.md");
    std::fs::write(&words, "words").expect("failed to write the words");

    let out = nokori(
        &there,
        &[
            "feedback",
            ".",
            "-m",
            "one",
            "-f",
            words.to_str().expect("non-UTF-8"),
        ],
    );
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("cannot be used together"), "{stderr}");
}

#[test]
fn feedback_says_which_way_the_path_is_wrong() {
    let there = temp_dir("paths");

    let out = nokori(&there, &["feedback", "./nowhere", "-m", "words"]);
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("is not a directory"),
        "{out:?}"
    );

    let plain = there.join("plain");
    std::fs::create_dir_all(&plain).expect("failed to make the plain dir");
    let out = nokori(
        &there,
        &[
            "feedback",
            plain.to_str().expect("non-UTF-8"),
            "-m",
            "words",
        ],
    );
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("is not set up"),
        "{out:?}"
    );

    // A backlog whose `open/` was never made is a different answer again.
    let old = there.join("old");
    std::fs::create_dir_all(old.join("backlog")).expect("failed to make the old backlog");
    std::fs::write(old.join("backlog/README.md"), "# Backlog\n")
        .expect("failed to write the README");
    let out = nokori(
        &there,
        &["feedback", old.to_str().expect("non-UTF-8"), "-m", "words"],
    );
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("not the one this version"),
        "{out:?}"
    );
}

#[test]
fn feedback_writes_an_item_that_check_accepts() {
    let there = repo_with_backlog("check");
    let out = nokori(&there, &["feedback", ".", "-m", "a written down report"]);
    assert!(out.status.success(), "feedback failed: {out:?}");

    let (name, _) = written(&there);
    let file = format!("backlog/open/{name}");
    let out = nokori(&there, &["check", &file]);
    assert!(
        out.status.success(),
        "check should accept it: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}
