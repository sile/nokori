//! The text `nokori` writes, and the few names that decide where it goes.
//!
//! Everything here is `include_str!` output or a literal, so the module as a
//! whole is free of I/O: the file names below are the paths a README is written
//! to, relative to the repository root, and writing them is the binary's
//! business.

/// The directory the backlog is kept in, and the two directories under it that
/// hold the two states. Both states are below `backlog/` rather than beside it,
/// so that the backlog is one directory to move and neither state sits next to
/// the source tree. They are also at the same depth, which is what lets a
/// relative link read the same from either one.
pub const BACKLOG_DIR: &str = "backlog";

/// The directory holding an open item, relative to the repository root.
pub const OPEN_DIR: &str = "backlog/open";

/// The directory holding a closed item, relative to the repository root.
pub const CLOSED_DIR: &str = "backlog/closed";

/// The README `init` writes into `backlog/`.
pub const BACKLOG_README_TEMPLATE: &str = include_str!("../templates/backlog/README.md");

/// The README `init` writes into `backlog/open/`.
pub const OPEN_README_TEMPLATE: &str = include_str!("../templates/backlog/open/README.md");

/// The README `init` writes into `backlog/closed/`.
pub const CLOSED_README_TEMPLATE: &str = include_str!("../templates/backlog/closed/README.md");

/// What `init` writes, each with the template for its content.
pub const README_FILES: &[(&str, &str)] = &[
    ("backlog/README.md", BACKLOG_README_TEMPLATE),
    ("backlog/open/README.md", OPEN_README_TEMPLATE),
    ("backlog/closed/README.md", CLOSED_README_TEMPLATE),
];

/// The section `close` writes into, and the marker a template leaves in its
/// sections. The marker is a whole word of its own rather than a `TODO` a
/// comment happens to start with, so that an item may write about a `TODO` in
/// the code it reports on without `close` reading that as unfinished work.
pub const OUTCOME_SECTION: &str = "## Outcome";

/// The marker a template leaves in a section whose work is not done.
pub const TODO_MARKER: &str = "NOKORI_TODO:";

/// The kinds `create` accepts, each with the template written into the file.
pub const KINDS: &[(&str, &str)] = &[
    ("rfc", include_str!("../templates/backlog/rfc.md")),
    ("bug", include_str!("../templates/backlog/bug.md")),
    ("plan", include_str!("../templates/backlog/plan.md")),
    ("feedback", include_str!("../templates/backlog/feedback.md")),
];

/// The kind `feedback` writes.
pub const FEEDBACK_KIND: &str = "feedback";

/// The title line `feedback` cuts out whole: the words are the item, and a
/// title made of their first line is that line repeated a little above itself.
/// `create` keeps the line, because a writer there knows the phrase before the
/// words.
pub const FEEDBACK_TITLE_PREFIX: &str = "# Feedback:";

/// The section `feedback` puts the words in.
pub const FEEDBACK_SECTION: &str = "## Feedback";

/// The topics `explain` prints, each with the text. They carry the material a
/// command cannot: the rules about the states and the kinds, which read the
/// same whatever command the reader arrived from. `overview` comes first
/// because it is the one to read first: the other three state rules that only
/// make sense once the reader knows nothing here is enforced.
pub const TOPICS: &[(&str, &str)] = &[
    ("overview", include_str!("../templates/explain/overview.md")),
    ("kinds", include_str!("../templates/explain/kinds.md")),
    ("flow", include_str!("../templates/explain/flow.md")),
    (
        "references",
        include_str!("../templates/explain/references.md"),
    ),
];

/// The template for `kind`, or `None` when this build has no such kind.
pub fn template_for(kind: &str) -> Option<&'static str> {
    KINDS.iter().find(|(k, _)| *k == kind).map(|(_, t)| *t)
}

/// The text for `topic`, or `None` when there is no such topic.
pub fn topic(name: &str) -> Option<&'static str> {
    TOPICS
        .iter()
        .find(|(t, _)| *t == name)
        .map(|(_, text)| *text)
}

/// The template `feedback` writes from.
///
/// Returns `Err` rather than panicking, since a build whose `KINDS` has no
/// `feedback` entry is a bug in nokori and the run still has a message to
/// print.
pub fn feedback_template() -> Result<&'static str, String> {
    template_for(FEEDBACK_KIND).ok_or_else(|| {
        format!(
            r"this build has no `{FEEDBACK_KIND}` template to write from, which is a bug
in nokori rather than something to fix here."
        )
    })
}
