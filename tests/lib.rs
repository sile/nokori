//! Tests of the core as a caller of it sees it: no files, no processes, no
//! clock. Everything the edge would do is a closure written here.

use std::path::{Path, PathBuf};

/// Every path is a file, which is what a link check needs when the question is
/// about the link and not about the file system.
fn everything(_: &Path) -> bool {
    true
}

/// Nothing is a file.
fn nothing(_: &Path) -> bool {
    false
}

/// Only the paths named are files.
fn only<'a>(paths: &'a [&'a str]) -> impl Fn(&Path) -> bool + 'a {
    move |p: &Path| paths.iter().any(|q| Path::new(q) == p)
}

/// No other item can be read.
fn unreadable(_: &Path) -> Option<String> {
    None
}

const WRITTEN: &str = "# Bug: x\n\n## Summary\n\nIt panics.\n\n## Outcome\n\nIt was fixed.\n";

#[test]
fn a_written_item_is_closed_under_its_own_name() {
    let decision = nokori::close_item(nokori::CloseSource {
        file: "backlog/open/20250101-bug-x.md",
        is_file: true,
        content: WRITTEN,
        exists: &nothing,
        read: &unreadable,
    })
    .expect("a written item should close");

    assert_eq!(
        decision.source(),
        Path::new("backlog/open/20250101-bug-x.md")
    );
    assert_eq!(
        decision.destination(),
        Path::new("backlog/closed/20250101-bug-x.md")
    );
}

#[test]
fn an_empty_outcome_stops_the_close() {
    let content = "# Bug: x\n\n## Summary\n\nIt panics.\n\n## Outcome\n\n";
    let error = nokori::close_item(nokori::CloseSource {
        file: "backlog/open/20250101-bug-x.md",
        is_file: true,
        content,
        exists: &nothing,
        read: &unreadable,
    })
    .expect_err("an empty outcome should be refused");

    assert!(error.contains("is empty"), "{error}");
}

#[test]
fn a_link_that_does_not_resolve_from_the_closed_dir_stops_the_close() {
    // The link resolves where the file is and would not where it is going,
    // which is the case the move is checked for.
    let content = "# Bug: x\n\n## Summary\n\nSee [y](20250101-bug-y.md).\n\n## Outcome\n\nDone.\n";
    let exists = only(&["backlog/open/20250101-bug-y.md"]);
    let error = nokori::close_item(nokori::CloseSource {
        file: "backlog/open/20250101-bug-x.md",
        is_file: true,
        content,
        exists: &exists,
        read: &unreadable,
    })
    .expect_err("a link into open should be refused");

    assert!(error.contains("never cites an open one"), "{error}");
}

#[test]
fn the_same_link_is_allowed_when_the_other_file_links_back() {
    // One link at either end holds the relation, so the closed file owes a
    // name and nothing more.
    let content = "# Bug: x\n\n## Summary\n\nSee [y](20250101-bug-y.md).\n\n## Outcome\n\nDone.\n";
    let other = "backlog/open/20250101-bug-y.md";
    let files = [other];
    let exists = only(&files);
    let read = |p: &Path| (p == Path::new(other)).then(|| "[x](20250101-bug-x.md)\n".to_owned());
    let error = nokori::close_item(nokori::CloseSource {
        file: "backlog/open/20250101-bug-x.md",
        is_file: true,
        content,
        exists: &exists,
        read: &read,
    })
    .expect_err("a link into open is still refused");

    assert!(error.contains("links back"), "{error}");
    assert!(!error.contains("does not link back"), "{error}");
}

#[test]
fn check_accepts_an_item_whose_links_resolve() {
    let content = "# Bug: x\n\n## Summary\n\nSee [y](20250101-bug-y.md).\n\n## Outcome\n\n";
    let exists = only(&["backlog/open/20250101-bug-y.md"]);
    nokori::check_item(nokori::ItemSource {
        file: "backlog/open/20250101-bug-x.md",
        is_file: true,
        content,
        exists: &exists,
    })
    .expect("a written item with a resolving link should pass");
}

#[test]
fn check_leaves_the_outcome_comment_alone() {
    // The comment is in `## Outcome` and stays until the close, so it is not
    // work left undone.
    let content = "# Bug: x\n\n## Summary\n\nIt panics.\n\n## Outcome\n\n\
                   <!-- NOKORI_TODO: say what happened. -->\n";
    nokori::check_item(nokori::ItemSource {
        file: "backlog/open/20250101-bug-x.md",
        is_file: true,
        content,
        exists: &everything,
    })
    .expect("the outcome comment is not reported by check");
}

#[test]
fn the_same_comment_stops_the_close() {
    let content = "# Bug: x\n\n## Summary\n\nIt panics.\n\n## Outcome\n\n\
                   <!-- NOKORI_TODO: say what happened. -->\n";
    let error = nokori::close_item(nokori::CloseSource {
        file: "backlog/open/20250101-bug-x.md",
        is_file: true,
        content,
        exists: &nothing,
        read: &unreadable,
    })
    .expect_err("close reports every section");

    assert!(error.contains("NOKORI_TODO:"), "{error}");
}

fn item(path: &str, links: &[&str]) -> nokori::Item {
    nokori::Item {
        path: PathBuf::from(path),
        name: nokori::item_label(path),
        links: links.iter().map(PathBuf::from).collect(),
    }
}

#[test]
fn the_walk_marks_which_file_holds_the_link() {
    let items = vec![
        item(
            "backlog/open/20250102-bug-later.md",
            &["backlog/closed/20250101-bug-earlier.md"],
        ),
        item("backlog/closed/20250101-bug-earlier.md", &[]),
    ];
    let tree = nokori::Tree::new("backlog/open/20250102-bug-later.md", &items)
        .expect("the item is in the set");

    assert_eq!(
        tree.as_str(),
        "bug-later\n  - bug-earlier\n    <- bug-later\n      (already on the path: \
         the links run in a circle, so the walk stops here)\n"
    );
}

#[test]
fn the_walk_refuses_an_item_that_is_not_in_the_set() {
    let error = nokori::Tree::new("backlog/open/20250101-bug-x.md", &[])
        .expect_err("nothing was read, so there is nothing to walk");

    assert!(error.contains("is not an item"), "{error}");
}

#[test]
fn an_item_name_is_the_day_the_kind_and_the_slug() {
    let clock = nokori::Clock {
        date: "20250101".to_owned(),
        time: "120000".to_owned(),
    };

    assert_eq!(
        nokori::item_name(&clock, "bug", "x-y").expect("a valid name"),
        Path::new("backlog/open/20250101-bug-x-y.md")
    );
    assert!(nokori::item_name(&clock, "nope", "x").is_err());
    assert!(nokori::item_name(&clock, "bug", "Not A Slug").is_err());
}

#[test]
fn a_feedback_name_carries_no_slug() {
    let clock = nokori::Clock {
        date: "20250101".to_owned(),
        time: "120000".to_owned(),
    };

    assert_eq!(
        nokori::feedback_name(&clock),
        Path::new("backlog/open/20250101-feedback-120000.md")
    );
}

#[test]
fn a_feedback_item_drops_the_title_and_fills_the_section() {
    let template = nokori::feedback_template().expect("feedback is a kind");
    let content = nokori::feedback_content(template, "It is awkward.").expect("the template fits");

    assert!(!content.starts_with("# Feedback:"), "{content}");
    assert!(
        content.contains("## Feedback\n\nIt is awkward.\n"),
        "{content}"
    );
    // The comment that asked for the section is the answer the words gave.
    assert!(
        !content.contains("<!-- NOKORI_TODO: what you think"),
        "{content}"
    );
    // The outcome comment is still there: it is written at the close.
    assert!(content.contains(nokori::OUTCOME_SECTION), "{content}");
}

#[test]
fn links_inside_comments_and_code_are_not_links() {
    let content = "see [a](a.md)\n\
                   <!-- [b](b.md) -->\n\
                   `[c](c.md)`\n\
                   ```\n\
                   [d](d.md)\n\
                   ```\n\
                   [e](e.md)\n";
    let found: Vec<String> = nokori::links(content)
        .into_iter()
        .map(|l| l.target)
        .collect();

    assert_eq!(found, vec!["a.md".to_owned(), "e.md".to_owned()]);
}

#[test]
fn a_top_level_heading_does_not_count_as_a_section() {
    let content = "# Bug: <!-- NOKORI_TODO: name it -->\n\n## Summary\n\nx\n";
    let at = nokori::find_marker(content).expect("the marker is there");

    // The marker is in the title, which is above any section, so the message
    // has no section to name.
    assert!(nokori::find_marker(&content[..at]).is_none());
}

#[test]
fn a_topic_is_found_by_its_name() {
    assert!(nokori::topic("overview").is_some());
    assert!(nokori::topic("nope").is_none());
    assert_eq!(nokori::TOPICS[0].0, "overview");
}
