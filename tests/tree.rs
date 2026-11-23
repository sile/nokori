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
        "nokori-test-tree-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock before epoch")
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

/// Sets up a backlog with the items it is given: each one is a name and a body,
/// written into `backlog/open/` unless it starts with `closed/`.
fn repo(name: &str, items: &[(&str, &str)]) -> std::path::PathBuf {
    let dir = temp_dir(name);
    assert!(nokori(&dir, &["init"]).status.success());
    for (file, body) in items {
        let path = dir.join("backlog").join(file);
        std::fs::create_dir_all(path.parent().expect("a path has a parent"))
            .expect("failed to create the item dir");
        std::fs::write(&path, format!("# Bug: x\n\n## Summary\n\n{body}\n"))
            .expect("failed to write item");
    }
    dir
}

fn tree(dir: &Path, start: &str) -> String {
    let output = nokori(dir, &["tree", start]);
    assert!(output.status.success(), "{output:?}");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn inverted(dir: &Path, start: &str) -> String {
    let output = nokori(dir, &["tree", "--invert", start]);
    assert!(output.status.success(), "{output:?}");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn tree_prints_an_item_with_nothing_around_it_alone() {
    let dir = repo("alone", &[("open/20250101-bug-alone.md", "nothing")]);

    let printed = tree(&dir, "backlog/open/20250101-bug-alone.md");
    assert_eq!(printed, "20250101-bug-alone\n");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn tree_follows_the_links_the_item_holds_by_default() {
    // The open item links the closed one. By default the walk leaves from the
    // end that holds the link, so the closed item is reached and its own links
    // are followed after it -- and it links nothing.
    let dir = repo(
        "holds",
        &[
            (
                "open/20250102-bug-later.md",
                "came from [bug-earlier](../closed/20250101-bug-earlier.md)",
            ),
            ("closed/20250101-bug-earlier.md", "earlier"),
        ],
    );

    let printed = tree(&dir, "backlog/open/20250102-bug-later.md");
    assert_eq!(printed, "20250102-bug-later\n└── 20250101-bug-earlier\n");

    // The other direction is the other half of the same relation: the closed
    // item is pointed at by the open one, and holds no link itself.
    let printed = inverted(&dir, "backlog/closed/20250101-bug-earlier.md");
    assert_eq!(printed, "20250101-bug-earlier\n└── 20250102-bug-later\n");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn tree_branches_a_line_that_has_more_below_it() {
    // `cargo tree`'s drawing: `├── ` while a sibling follows, `└── ` for the
    // last one, and `│   ` carrying the depth above where a line below is
    // still there.
    let dir = repo(
        "branches",
        &[
            (
                "open/20250101-bug-start.md",
                "[bug-a](20250102-bug-a.md) and [bug-b](20250103-bug-b.md)",
            ),
            (
                "open/20250102-bug-a.md",
                "[bug-shared](20250104-bug-shared.md)",
            ),
            ("open/20250103-bug-b.md", "nothing"),
            ("open/20250104-bug-shared.md", "shared"),
        ],
    );

    let printed = tree(&dir, "backlog/open/20250101-bug-start.md");
    assert_eq!(
        printed,
        "20250101-bug-start\n├── 20250102-bug-a\n│   └── 20250104-bug-shared\n└── 20250103-bug-b\n"
    );

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn tree_does_not_mark_the_direction_per_line() {
    // The run said the direction once, so no line says which file holds the
    // link that reached it. The mark was the thing this replaced.
    let dir = repo(
        "no-marks",
        &[
            ("open/20250101-bug-start.md", "[bug-a](20250102-bug-a.md)"),
            ("open/20250102-bug-a.md", "nothing"),
        ],
    );

    let printed = tree(&dir, "backlog/open/20250101-bug-start.md");
    assert!(!printed.contains("<- "), "{printed}");
    assert!(!printed.contains(" - "), "{printed}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn tree_prints_an_item_under_each_path_that_reaches_it() {
    let dir = repo(
        "twice",
        &[
            (
                "open/20250101-bug-start.md",
                "[bug-a](20250102-bug-a.md) and [bug-b](20250103-bug-b.md)",
            ),
            (
                "open/20250102-bug-a.md",
                "[bug-shared](20250104-bug-shared.md)",
            ),
            (
                "open/20250103-bug-b.md",
                "[bug-shared](20250104-bug-shared.md)",
            ),
            ("open/20250104-bug-shared.md", "shared"),
        ],
    );

    let printed = tree(&dir, "backlog/open/20250101-bug-start.md");
    // `bug-shared` is reached from `bug-a` and again from `bug-b`, so it is
    // printed under both rather than once. The first hop under the start is
    // each of the two, and each of those reaches `bug-shared` in turn.
    assert!(
        printed
            .starts_with("20250101-bug-start\n├── 20250102-bug-a\n│   └── 20250104-bug-shared\n"),
        "{printed}"
    );
    assert!(
        printed.contains("\n└── 20250103-bug-b\n    └── 20250104-bug-shared\n"),
        "{printed}"
    );

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn tree_stops_at_a_cycle_and_says_so() {
    // Two closed items citing each other pass `close`'s checks when they are
    // committed in the same run, so the walk has to survive one.
    let dir = repo(
        "cycle",
        &[
            (
                "closed/20250101-bug-one.md",
                "[bug-two](20250102-bug-two.md)",
            ),
            (
                "closed/20250102-bug-two.md",
                "[bug-one](20250101-bug-one.md)",
            ),
        ],
    );

    let printed = tree(&dir, "backlog/closed/20250101-bug-one.md");
    assert!(printed.contains("already on the path"), "{printed}");
    // The walk ends, which is the thing being tested: a hang would not return.
    assert!(printed.starts_with("20250101-bug-one\n"), "{printed}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn tree_skips_a_link_that_is_not_an_item() {
    let dir = repo(
        "not-item",
        &[(
            "open/20250101-bug-links.md",
            "[code](../src/main.rs) and [web](https://example.com)",
        )],
    );

    let printed = tree(&dir, "backlog/open/20250101-bug-links.md");
    assert_eq!(printed, "20250101-bug-links\n");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn tree_refuses_a_file_that_is_not_an_item() {
    let dir = repo("not-a-file", &[]);

    let output = nokori(&dir, &["tree", "backlog/README.md"]);
    assert!(!output.status.success(), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("is not an item"), "{stderr}");

    let output = nokori(&dir, &["tree", "backlog/open/nope.md"]);
    assert!(!output.status.success(), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("is not a file"), "{stderr}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}
