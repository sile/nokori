//! Deciding whether an open item can be closed, and what the move names.
//!
//! The core reads the text and the links it holds; whether a link resolves is
//! the edge's answer, handed in as [`CloseSource::exists`]. Performing the move
//! is the edge's too: the core returns the [`CloseDecision`] that says what to
//! move where, and its own [`Closing`] report says what was done.

use std::path::{Path, PathBuf};

use crate::check::check_marker;
use crate::item::link_name;
use crate::links::{Link, is_external, links, normalize, target_path};
use crate::marker::{close_hint, marker_message};
use crate::model::Item;
use crate::templates::{BACKLOG_DIR, CLOSED_DIR, OPEN_DIR, OUTCOME_SECTION};

/// An item as the edge found it, for the questions a close asks about the
/// paths its links name.
pub struct CloseSource<'a> {
    /// The path as it was given, used in the messages rather than for I/O.
    pub file: &'a str,

    /// Whether that path is a file.
    pub is_file: bool,

    /// The text of the file.
    pub content: &'a str,

    /// Says whether a resolved path exists, so that a link is refused when the
    /// file it will name once moved is not there.
    pub exists: &'a dyn Fn(&Path) -> bool,

    /// Reads another item's text, or returns `None` when it cannot be read.
    /// Only the one file a refused link names is read, never the directory.
    pub read: &'a dyn Fn(&Path) -> Option<String>,
}

/// The close a run decided on: the name to move to, and the source it came
/// from. Performing the move is the edge's: the decision says what to move
/// where, and the run that made it opens the directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloseDecision {
    source: PathBuf,
    destination: PathBuf,
}

impl CloseDecision {
    /// The file the item is in now.
    pub fn source(&self) -> &Path {
        &self.source
    }

    /// The file the item is moved to, under `backlog/closed/`.
    pub fn destination(&self) -> &Path {
        &self.destination
    }
}

/// Decides whether the item `source` names can be closed, and under what name.
///
/// Everything that would make the move wrong is refused here, before the move,
/// and for the same reason each time: a closed file is not edited again, so a
/// problem created by the move is one nobody can repair afterwards.
pub fn close_item(source: CloseSource<'_>) -> Result<CloseDecision, String> {
    let file = source.file;
    if !source.is_file {
        return Err(not_a_file(file, &source));
    }

    let src = Path::new(file);
    if src.parent() != Some(Path::new(OPEN_DIR)) {
        return Err(not_open(file, src));
    }

    let content = source.content;
    check_filled_in(file, content)?;
    let name = src
        .file_name()
        .expect("a path under the open directory is a file");
    let dst = Path::new(CLOSED_DIR).join(name);
    // Checked before the move, and from both places: a link that names one
    // file in `open/` and another in `closed/` would be broken by the move,
    // and a closed file is not edited to repair it afterwards.
    check_links(file, src, &dst, content, &source)?;

    if (source.exists)(&dst) {
        return Err(format!(
            r"`{}` already exists, so closing this item would replace the record of
another one. Rename one of the two files by hand and run `nokori close`
again.",
            dst.display()
        ));
    }

    Ok(CloseDecision {
        source: src.to_path_buf(),
        destination: dst,
    })
}

/// The items in `backlog/open/` whose links name `target`, sorted by path.
///
/// The move breaks these even though nothing in the file being closed is
/// wrong: an open item at a neighbour's path names the file as
/// `20250101-bug-x.md`, which reaches nothing once the target is in
/// `backlog/closed/`. The list is a report, not a refusal -- an open item may
/// cite an open one, and repairing it is the other item's business -- so it is
/// gathered after the decision, from the items the edge already read for
/// `tree`.
///
/// `items` is every item the edge read, keyed by [`Item::path`], and each
/// item's `links` are already resolved and normalized. An item counts when it
/// is itself in `backlog/open/` and one of its links equals `target`; a link
/// that already points at `target`'s closed path is not this item's problem
/// and is not returned.
pub fn open_items_linking(target: &Path, items: &[Item]) -> Vec<PathBuf> {
    let target = normalize(target);
    let mut found: Vec<PathBuf> = items
        .iter()
        .filter(|item| item.path.starts_with(OPEN_DIR) && item.path != target)
        .filter(|item| item.links.contains(&target))
        .map(|item| item.path.clone())
        .collect();
    found.sort();
    found
}

/// The message for a `FILE` that is not there. What to say depends on whether
/// the item was already closed, which the answer is handed in for.
fn not_a_file(file: &str, source: &CloseSource<'_>) -> String {
    let closed = match Path::new(file).file_name() {
        Some(name) => Path::new(CLOSED_DIR).join(name),
        None => PathBuf::new(),
    };
    let hint = if !closed.as_os_str().is_empty() && (source.exists)(&closed) {
        format!(
            r" It is already closed, as `{}`: a closed item is left as it was
written, so there is nothing for `close` to do.",
            closed.display()
        )
    } else {
        format!(
            r" Pass the item to close as a path, such as
`nokori close {BACKLOG_DIR}/20250101-bug-panic-on-empty-input.md`."
        )
    };
    format!("`{file}` is not a file, so there is no item to close.{hint}")
}

/// The message for a file that is not in `backlog/open/`, which is either a
/// name without its directory or a file that is not open at all.
fn not_open(file: &str, src: &Path) -> String {
    let hint = if src.components().count() == 1 {
        format!(" The item is probably `{OPEN_DIR}/{file}`.")
    } else {
        format!(
            r" A file that is not in `{OPEN_DIR}/` is not open, and one that is
already closed is left as it was written."
        )
    };
    format!(
        r"`{file}` is not an item in `{OPEN_DIR}/`, so `close` is not the command
for it.{hint}"
    )
}

/// Checks that the item is ready to be closed: no `TODO` left anywhere, and the
/// `## Outcome` section filled in.
pub fn check_filled_in(file: &str, content: &str) -> Result<(), String> {
    check_markers(file, content)?;
    check_outcome(file, content)
}

/// The marker check as `close` makes it: every section, `## Outcome` included.
fn check_markers(file: &str, content: &str) -> Result<(), String> {
    check_marker(content, |_| true).map_or(Ok(()), |(todo, line)| {
        let last = close_hint();
        Err(marker_message(file, content, todo, line, &last))
    })
}

/// The half of `check_filled_in` that only `close` makes: the section that
/// records how the item settled has to be there, and has to say something.
fn check_outcome(file: &str, content: &str) -> Result<(), String> {
    let outcome = content
        .lines()
        .position(|line| line.trim_end() == OUTCOME_SECTION);
    let Some(start) = outcome else {
        return Err(format!(
            r"`{file}` has no `{OUTCOME_SECTION}` section. An item records what was
concluded, so a file without one cannot be closed. Add the section, write
what happened, and commit the file."
        ));
    };
    let body: String = content
        .lines()
        .skip(start + 1)
        .collect::<Vec<_>>()
        .join("\n");
    if body.trim().is_empty() {
        return Err(format!(
            r"`{OUTCOME_SECTION}` in `{file}` is empty, and closing records the outcome
there. Say what actually happened and name the change that settled the item
(the commit or pull request, when there is one), then commit the file."
        ));
    }

    Ok(())
}

/// Checks the links in a file about to move from `open/` to `closed/`. Each
/// one is read from where the file is going, and two things are refused: a
/// target that names an item still in `open/`, and a target that does not
/// resolve.
///
/// Both are about the file as it will be read, not as it is read now. A closed
/// file is not edited, so a target that is wrong once the move is done is one
/// nobody can repair. The rule that a closed item never cites an open one is
/// checked here for the same reason, and the move is what would break it: a
/// file in `closed/` writing `../open/x.md` is exactly such a citation.
///
/// Every link is read as a path, whether or not it names an item, since a
/// check with a narrower scope is one the writer has to remember. Links that
/// are not paths are skipped: one with a scheme (`https://`, `mailto:`) and
/// one that only names an anchor in this file.
fn check_links(
    file: &str,
    src: &Path,
    dst: &Path,
    content: &str,
    source: &CloseSource<'_>,
) -> Result<(), String> {
    let here = src.parent().unwrap_or_else(|| Path::new("."));
    let there = dst.parent().unwrap_or_else(|| Path::new("."));

    for link in links(content) {
        if is_external(&link.target) {
            continue;
        }

        let now = target_path(here, &link.target);
        let then = target_path(there, &link.target);

        // The item it names is still open, so the closed file would cite an
        // open one: the item can still be renamed or turned down, and a closed
        // file is not edited to follow it. Both places are read, because a
        // bare file name reaches an open item where the file is now and
        // nothing at all where it is going, and the open item is the more
        // useful thing to say.
        let cites_open = |path: &Path| path.starts_with(OPEN_DIR) && (source.exists)(path);
        if cites_open(&now) || cites_open(&then) {
            let named = if (source.exists)(&then) { then } else { now };
            return Err(cites_open_message(file, &link, &named, source));
        }

        if !(source.exists)(&then) {
            return Err(format!(
                r"`{file}` links `{}` (line {}), which is not a file. A close refuses
a link that does not resolve, because a closed item is not edited again and the
link could never be repaired there.

If the item it named was closed, point the link at its file in `{CLOSED_DIR}/`.
If it was renamed, point it at the new name. If it was never written, say what
it was going to do instead. The paths are in `nokori explain references`.",
                link.target, link.line
            ));
        }
    }

    Ok(())
}

/// The message for a link that names an item still in `backlog/open/`. What to
/// do with it depends on the other file: a link is stored at either end, so
/// when that file links back the relation survives the link becoming a name,
/// and the message stops after saying to write the name. When it does not, the
/// relation would be nowhere once the link is a name, so the message says to
/// add the link there while that file is still open.
///
/// The other file is read, never written. It is the one file the link names and
/// not the whole of `backlog/open/`: the question is about these two items, and
/// a link from a third one says nothing about it.
fn cites_open_message(file: &str, link: &Link, target: &Path, source: &CloseSource<'_>) -> String {
    let head = format!(
        r"`{file}` links `{}` (line {}), which names an item that is still open.
A closed item never cites an open one: the item it names can still be renamed
or turned down, and this file will not be edited again to follow it.

The relation does not need the link, though -- one link holds it from either
end -- so what a closed file needs is a name to read, not a target to follow.",
        link.target, link.line
    );
    let name = link_name(&link.target);
    let fix = format!(
        r"Write the target as a name instead, where the link is:

  `{name}`"
    );

    let other = target.display();
    if links_back(target, file, source) {
        return format!(
            "{head}\n\n{fix}\n\n`{other}` links back, so the relation is held from that end and\nthis name is all this file owes."
        );
    }
    format!(
        r"{head}

{fix}

`{other}` does not link back, so that name would be the only thing recording
the relation and a name is not followable. Add the link there while that file
is still open, at either end, and close this one afterwards. The rule is in
`nokori explain references`."
    )
}

/// Says whether `other` holds a link to `file`. Both are read as paths, and
/// every link in `other` is resolved from the directory it is in, so a link
/// written the way `explain references` says is found whichever end it is at.
///
/// A file that cannot be read links back to nothing: the writer has the other
/// file to look at, and the message then says the safe thing.
fn links_back(other: &Path, file: &str, source: &CloseSource<'_>) -> bool {
    let Some(content) = (source.read)(other) else {
        return false;
    };
    let here = other.parent().unwrap_or_else(|| Path::new("."));
    let me = Path::new(file);
    links(&content)
        .iter()
        .any(|link| !is_external(&link.target) && target_path(here, &link.target) == me)
}
