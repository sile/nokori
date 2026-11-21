//! Finding the work a template left undone, and saying which section it is in.

use crate::templates::{OUTCOME_SECTION, TODO_MARKER};

/// Finds the byte offset of the first marker left in a comment, or `None` when
/// every section is written.
///
/// Only the inside of an HTML comment is searched. The marker is a comment in
/// every template, and an item is allowed to write *about* it -- a bug report
/// on this check is one -- so a marker in a sentence is prose and not work left
/// undone. Searching the comments is what tells the two apart without asking
/// the writer to escape anything.
///
/// A comment that is never closed is searched to the end of the file, since the
/// rest of such a file is commented out as far as a reader is concerned, and a
/// marker in it is as unfinished as one in a closed comment.
pub fn find_marker(content: &str) -> Option<usize> {
    let mut rest = 0;
    while let Some(open) = content[rest..].find("<!--") {
        let body = rest + open + 4;
        let (end, next) = match content[body..].find("-->") {
            Some(close) => (body + close, body + close + 3),
            None => (content.len(), content.len()),
        };
        if let Some(found) = content[body..end].find(TODO_MARKER) {
            return Some(body + found);
        }
        rest = next;
    }
    None
}

/// The `## ` heading above `at`, or `None` when there is none above it.
pub fn section_of(content: &str, at: usize) -> Option<&str> {
    content[..at]
        .lines()
        .rev()
        .find(|line| line.starts_with("## "))
}

/// The message for a marker left at `todo`, on `line`: what is wrong, and what
/// `last` adds for the command that found it. The section the marker is in is
/// named when there is one, so the writer is told where to look rather than
/// only that something is left.
pub fn marker_message(file: &str, content: &str, todo: usize, line: usize, last: &str) -> String {
    let message = format!(
        r"`{file}` still has a `{TODO_MARKER}` comment (line {line}), so it is not
finished. Each comment says what its section needs; write that and delete the
comment.{last}"
    );
    match section_of(content, todo) {
        Some(heading) => format!(
            "{message}\n\nThe comment is in `{heading}`, so that section is not written yet."
        ),
        None => message,
    }
}

/// The hint `close` adds to a marker message, about the sections it reports.
pub fn close_hint() -> String {
    format!(
        " If a section has nothing to say, delete its comment along with\n\
         the heading -- every section but `{OUTCOME_SECTION}`, which has to stay\n\
         even when it is empty."
    )
}

/// The hint `check` adds to a marker message, about the section it leaves out.
pub fn check_hint() -> String {
    format!(
        " `{OUTCOME_SECTION}` is not reported here: its comment stays until the\n\
         item is closed, so it is still there while the work is going on. When\n\
         every other section is written, the item can be closed, and the close\n\
         is where that section is written."
    )
}
