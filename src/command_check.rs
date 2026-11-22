//! Deciding whether an open item is written yet.
//!
//! The core reads the text it is given and nothing else. Two of the three
//! questions `check` asks are about the file system -- is this a file, and
//! does a link resolve -- so the answers are handed in as [`ItemSource`], and
//! the run that answered them is the one that opened the directory.

use std::path::{Path, PathBuf};

use crate::links::{is_external, links, target_path};
use crate::marker::{check_hint, find_marker, marker_message, section_of};
use crate::templates::{CLOSED_DIR, OPEN_DIR, OUTCOME_SECTION};

/// An item as the edge found it: the text, and the one question about the path
/// that the core cannot answer without the file system.
pub struct ItemSource<'a> {
    /// The path as it was given, used in the messages rather than for I/O.
    pub file: &'a str,

    /// Whether that path is a file. A path that is not is refused before its
    /// text would be read.
    pub is_file: bool,

    /// The text of the file.
    pub content: &'a str,

    /// Says whether a resolved path exists. Every path the core resolves is
    /// passed here rather than checked itself, so the edge stays the only
    /// reader.
    pub exists: &'a dyn Fn(&Path) -> bool,
}

/// Refuses the two things that are wrong while an item is open: a `TODO` left
/// in a comment outside `## Outcome`, and a link that does not resolve.
///
/// The checks `close` makes that are not here are the point of the split.
/// `## Outcome` is *expected* to be empty, a file with uncommitted changes is
/// the normal state of something being written, and the link into
/// `backlog/open/` is allowed -- that rule is about a file that has moved.
///
/// The link check is not the same as `close`'s either: `close` reads the file
/// from where it is going, and this reads it from where it is. Where it is
/// going depends on how it is spelled, and a file being written is allowed to
/// be unfinished in ways the move would settle, so what is reported is what is
/// wrong now.
pub fn check_item(source: ItemSource<'_>) -> Result<(), String> {
    let file = source.file;
    if !source.is_file {
        return Err(format!(
            r"`{file}` is not a file, so there is no item to check. Pass the item as
a path, such as `nokori check {OPEN_DIR}/20250101-bug-panic-on-empty-input.md`."
        ));
    }

    let src = Path::new(file);
    if src.parent() != Some(Path::new(OPEN_DIR)) {
        let hint = if src.components().count() == 1 {
            format!(" The item is probably `{OPEN_DIR}/{file}`.")
        } else {
            format!(
                r" There is nothing to check about an item in `{CLOSED_DIR}/`: it is
written and settled, and it is not edited again."
            )
        };
        return Err(format!(
            r"`{file}` is not an item in `{OPEN_DIR}/`, so `check` is not the command
for it.{hint}"
        ));
    }

    let content = source.content;
    check_markers_outside_outcome(file, content)?;
    check_links_now(source)
}

/// The half of the marker check that is about a file still being written: the
/// `TODO` comments, and nothing about `## Outcome`, which is empty until the
/// close.
///
/// A comment is left in `## Outcome` for the whole of the working time, so its
/// being there says nothing about whether the work is done.
pub(crate) fn check_markers_outside_outcome(file: &str, content: &str) -> Result<(), String> {
    check_marker(content, |heading| heading != OUTCOME_SECTION).map_or(Ok(()), |(todo, line)| {
        let last = check_hint();
        Err(marker_message(file, content, todo, line, &last))
    })
}

/// The first marker left in a comment whose section `in_section` accepts, as
/// the offset of the marker and the line it is on.
pub(crate) fn check_marker(
    content: &str,
    in_section: impl Fn(&str) -> bool,
) -> Option<(usize, usize)> {
    let mut from = 0;
    while let Some(found) = find_marker(&content[from..]) {
        let todo = from + found;
        let line = content[..todo].lines().count();
        let heading = section_of(content, todo).unwrap_or("");
        if in_section(heading) {
            return Some((todo, line));
        }
        // Resume after the comment the marker was in, so that the next search
        // looks at the next comment rather than at this one again.
        from = content[todo..]
            .find("-->")
            .map_or(content.len(), |end| todo + end + 3);
    }
    None
}

/// Checks the links in a file where it is now. Nothing is refused for the
/// direction or for what the move would do to it, since neither is a problem
/// yet: a link into `backlog/open/` is allowed from `backlog/open/`, and a
/// target that only becomes wrong when the file moves is one to fix before the
/// move, which is the next command.
fn check_links_now(source: ItemSource<'_>) -> Result<(), String> {
    let file = source.file;
    let here = Path::new(file).parent().unwrap_or_else(|| Path::new("."));
    for link in links(source.content) {
        if is_external(&link.target) {
            continue;
        }
        let path: PathBuf = target_path(here, &link.target);
        if !(source.exists)(&path) {
            return Err(format!(
                r"`{file}` links `{}` (line {}), which is not a file. A link to a file in
this repository is read as a path, so it has to resolve for the item to be
written.

If the file it names was renamed, point the link at the new name. If it was
never written, say what it will say instead. The paths are in `nokori explain
references`.",
                link.target, link.line
            ));
        }
    }

    Ok(())
}
