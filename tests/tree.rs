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
    assert_eq!(printed, "open/20250101-bug-alone\n");

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
    assert_eq!(
        printed,
        "open/20250102-bug-later\n└── closed/20250101-bug-earlier\n"
    );

    // The other direction is the other half of the same relation: the closed
    // item is pointed at by the open one, and holds no link itself.
    let printed = inverted(&dir, "backlog/closed/20250101-bug-earlier.md");
    assert_eq!(
        printed,
        "closed/20250101-bug-earlier\n└── open/20250102-bug-later\n"
    );

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
        "open/20250101-bug-start\n├── open/20250102-bug-a\n│   └── open/20250104-bug-shared\n└── open/20250103-bug-b\n"
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
    // `bug-shared` is reached from `bug-a` and again from `bug-b`: it is
    // printed under both, since which path reached it is what the reader is
    // following, and the second one is `(*)` because its subtree is under the
    // first.
    assert_eq!(
        printed,
        "open/20250101-bug-start\n\
         ├── open/20250102-bug-a\n\
         │   └── open/20250104-bug-shared\n\
         └── open/20250103-bug-b\n\
         \x20   └── open/20250104-bug-shared (*)\n"
    );

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn tree_prints_an_item_once_below_one_line() {
    // The usual shape of an RFC: the item it reads its rule from is linked in
    // the body and again in `## Outcome`, so one relation is written twice.
    let dir = repo(
        "one-line",
        &[
            (
                "closed/20250102-rfc-later.md",
                "[earlier](20250101-rfc-earlier.md) and again [earlier](20250101-rfc-earlier.md)",
            ),
            ("closed/20250101-rfc-earlier.md", "earlier"),
        ],
    );

    let printed = tree(&dir, "backlog/closed/20250102-rfc-later.md");
    assert_eq!(
        printed,
        "closed/20250102-rfc-later\n└── closed/20250101-rfc-earlier\n"
    );

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn tree_marks_an_item_it_has_shown_and_walks_no_further() {
    // `bug-shared` has a link of its own; the second line for it says it was
    // shown already and does not print the subtree a second time.
    let dir = repo(
        "shown",
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
            (
                "open/20250104-bug-shared.md",
                "[bug-below](20250105-bug-below.md)",
            ),
            ("open/20250105-bug-below.md", "below"),
        ],
    );

    let printed = tree(&dir, "backlog/open/20250101-bug-start.md");
    assert_eq!(
        printed,
        "open/20250101-bug-start\n\
         ├── open/20250102-bug-a\n\
         │   └── open/20250104-bug-shared\n\
         │       └── open/20250105-bug-below\n\
         └── open/20250103-bug-b\n\
         \x20   └── open/20250104-bug-shared (*)\n"
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
    assert!(
        printed.starts_with("closed/20250101-bug-one\n"),
        "{printed}"
    );
    // A cycle is said rather than marked: the item is on the path, not printed
    // under an earlier line, and the two are different causes.
    assert!(!printed.contains("(*)"), "{printed}");

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
    assert_eq!(printed, "open/20250101-bug-links\n");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn tree_walks_every_item_it_is_given_in_the_order_given() {
    let dir = repo(
        "many",
        &[
            ("open/20250102-bug-second.md", "nothing"),
            ("open/20250101-bug-first.md", "nothing"),
        ],
    );

    let output = nokori(
        &dir,
        &[
            "tree",
            "backlog/open/20250102-bug-second.md",
            "backlog/open/20250101-bug-first.md",
        ],
    );
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "open/20250102-bug-second\n\nopen/20250101-bug-first\n"
    );

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn tree_starts_each_root_with_nothing_shown() {
    // The roots are separate questions, so the item reached under the first is
    // not `(*)` under the second.
    let dir = repo(
        "roots",
        &[
            ("open/20250101-bug-start.md", "[bug-a](20250102-bug-a.md)"),
            ("open/20250102-bug-a.md", "nothing"),
        ],
    );

    let output = nokori(
        &dir,
        &[
            "tree",
            "backlog/open/20250101-bug-start.md",
            "backlog/open/20250102-bug-a.md",
        ],
    );
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "open/20250101-bug-start\n└── open/20250102-bug-a\n\nopen/20250102-bug-a\n"
    );

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn tree_with_no_item_walks_every_open_item() {
    let dir = repo(
        "all-open",
        &[
            ("open/20250102-bug-second.md", "nothing"),
            ("open/20250101-bug-first.md", "nothing"),
            ("closed/20250101-bug-done.md", "nothing"),
        ],
    );

    let output = nokori(&dir, &["tree"]);
    assert!(output.status.success(), "{output:?}");
    // The line says why these items were walked, and the closed one is not a
    // root: the missing argument asks about what is open.
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "No item was given, so every open item is walked.\n\n\
         open/20250101-bug-first\n\n\
         open/20250102-bug-second\n"
    );

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn tree_with_no_item_and_nothing_open_says_so_and_succeeds() {
    let dir = repo(
        "nothing-open",
        &[("closed/20250101-bug-done.md", "nothing")],
    );

    let output = nokori(&dir, &["tree"]);
    // A backlog with nothing open is a backlog, not an error.
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "No item was given, so every open item is walked.\n"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No item is open"), "{stderr}");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

// A deep chain of links is where a recursive walk runs out of stack. The walk
// is recursive, so this says how deep it gets: a year's worth of items is
// deeper than a backlog is likely to be, and shallow enough to be quick.
#[test]
fn tree_walks_a_long_chain() {
    let depth = 500;
    let mut items: Vec<(String, String)> = Vec::with_capacity(depth);
    for day in 0..depth {
        let name = format!("2025{day:04}-bug-x");
        let body = if day + 1 < depth {
            format!("[next](2025{:04}-bug-x.md)", day + 1)
        } else {
            "nothing".to_owned()
        };
        items.push((format!("open/{name}.md"), body));
    }
    let borrowed: Vec<(&str, &str)> = items
        .iter()
        .map(|(name, body)| (name.as_str(), body.as_str()))
        .collect();

    let dir = repo("long-chain", &borrowed);
    // One line per item: the root, then one per hop below it.
    let printed = tree(&dir, "backlog/open/20250000-bug-x.md");
    assert_eq!(printed.lines().count(), depth, "{}", printed.len());

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
