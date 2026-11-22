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
fn explain_says_when_an_accepted_rfc_closes() {
    let dir = temp_dir("accepted-rfc");

    let output = nokori(&dir, &["explain", "flow"]);
    assert!(output.status.success(), "{output:?}");

    // The examples in the paragraph above it are all decisions not to do the
    // work, so an accepted RFC is the case a reader has to be told about.
    let stdout = String::from_utf8_lossy(&output.stdout);
    let start = stdout
        .find("An `rfc` that is accepted")
        .unwrap_or_else(|| panic!("no paragraph on accepted RFCs in {stdout}"));
    let paragraph = &stdout[start..];
    let paragraph = &paragraph[..paragraph.find("\n\n").unwrap_or(paragraph.len())];
    assert!(
        paragraph.contains("stays open until the change it names has landed"),
        "{paragraph}"
    );

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
    // The whole list, in the order `explain` prints it: a reader who mistyped a
    // topic is being told what to type instead, and `overview` is the one to
    // read first. Anchoring on the list rather than on a name keeps a topic
    // added to `TOPICS` from passing this test without appearing here.
    assert!(
        stderr.contains("The topics are: overview, kinds, flow, references, feedback."),
        "{stderr}"
    );

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn explain_with_no_topic_lists_them() {
    // The reader who has just installed the crate knows no topic names, so the
    // bare command is the one they are most likely to run first.
    let dir = temp_dir("no-topic");

    let output = nokori(&dir, &["explain"]);
    assert!(output.status.success(), "{output:?}");

    let stdout = String::from_utf8_lossy(&output.stdout);
    for name in ["overview", "kinds", "flow", "references", "feedback"] {
        assert!(stdout.contains(name), "{name} in {stdout}");
    }
    // `overview` is the one to read first, so it is the one printed first.
    let first = stdout
        .lines()
        .find(|line| line.starts_with("- "))
        .expect("no topic listed");
    assert_eq!(first, "- overview", "{stdout}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn explain_prints_the_feedback_flow() {
    let dir = temp_dir("feedback");

    let output = nokori(&dir, &["explain", "feedback"]);
    assert!(output.status.success(), "{output:?}");

    let stdout = String::from_utf8_lossy(&output.stdout);
    // The reporter's side: the command, where the words come from, the file it
    // writes, and that nothing is committed. It is not the `feedback` kind,
    // which `kinds` explains, and the page says so.
    for term in [
        "nokori feedback",
        "standard input",
        "YYYYMMDD-feedback",
        "Nothing is committed",
        "not the `feedback` kind",
    ] {
        assert!(stdout.contains(term), "{term} in {stdout}");
    }

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn flow_still_mentions_feedback() {
    // The reporter's page is an entry, not a move: a developer reading the
    // flow still finds the command where it always was, with a pointer to the
    // other page.
    let dir = temp_dir("flow-feedback");

    let output = nokori(&dir, &["explain", "flow"]);
    assert!(output.status.success(), "{output:?}");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nokori feedback"), "{stdout}");
    assert!(stdout.contains("nokori explain feedback"), "{stdout}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn explain_prints_the_overview() {
    let dir = temp_dir("overview");

    let output = nokori(&dir, &["explain", "overview"]);
    assert!(output.status.success(), "{output:?}");

    let stdout = String::from_utf8_lossy(&output.stdout);
    // The premise the other three topics presume: what the backlog is, what the
    // tool is for, and that nothing here is enforced.
    for term in ["What the backlog is", "What the tool is for", "enforce"] {
        assert!(stdout.contains(term), "{term} in {stdout}");
    }

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn flow_names_check() {
    // `check` is not a step in the flow, and its section says so: what the
    // reader has to find is the command, which `create` prints too.
    let dir = temp_dir("flow-check");

    let output = nokori(&dir, &["explain", "flow"]);
    assert!(output.status.success(), "{output:?}");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nokori check"), "{stdout}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn flow_no_longer_says_what_an_item_is() {
    // That section moved to `overview`, which is where the premise belongs:
    // `flow` is about one item's life, and the rule that an item is a record
    // rather than a specification is not a fact about a command.
    let dir = temp_dir("flow-moved");

    let output = nokori(&dir, &["explain", "flow"]);
    assert!(output.status.success(), "{output:?}");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("What an item is"), "{stdout}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}
