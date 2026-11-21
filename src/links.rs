//! Reading the links a markdown file holds.
//!
//! A link is read only where a reader would see one. Text inside an HTML
//! comment is not rendered, and text inside a code fence or a code span is
//! rendered as code, so a `](` in any of them is not a link -- it is text
//! about a link, which no reader follows and which the item does not depend
//! on.

use std::path::{Component, Path, PathBuf};

/// A link found in a file: the target as it is written, and the line it is on
/// so that an error can say which one is wrong.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    /// The target as written, without the `[...]` and the parentheses.
    pub target: String,

    /// The 1-based line the link is on.
    pub line: usize,
}

/// Collects the links in `content`. Only inline links (`[text](target)`) with a
/// target that is a plain path are read, since that is the form a reference is
/// written in.
///
/// The example links the templates put in their comments are the case that
/// made the comment rule matter: they name no file, and an item created from a
/// template is refused for one until its comment is deleted.
pub fn links(content: &str) -> Vec<Link> {
    let mut found = Vec::new();
    let mut fence: Option<(char, usize)> = None;
    let mut in_comment = false;

    for (index, line) in content.split('\n').enumerate() {
        if let Some((marker, length)) = fence {
            if closes_fence(line, marker, length) {
                fence = None;
            }
            continue;
        }
        if let Some(opened) = opens_fence(line) {
            fence = Some(opened);
            continue;
        }

        let text = visible_text(line, &mut in_comment);
        for target in links_in(&text) {
            found.push(Link {
                target,
                line: index + 1,
            });
        }
    }
    found
}

/// Says whether a target is something other than a path in this repository: a
/// link with a scheme, or one that names only an anchor in the same file. The
/// two are skipped by the checks, since neither is a file that a move could
/// change the meaning of.
///
/// A scheme is read as the colon before any `/` (`https:`, `mailto:`), so that
/// a scheme is recognized by its form rather than by a list of the ones found
/// so far. A colon further along is part of a file name and is not a scheme.
pub fn is_external(target: &str) -> bool {
    if target.starts_with('#') || target.is_empty() {
        return true;
    }
    let head = target.split('/').next().unwrap_or_default();
    head.contains(':')
}

/// Reads a link target as a path from the repository root, dropping an anchor
/// and folding `.` and `..` away, so that the depth of the result says where
/// the file is rather than how the link was spelled.
///
/// A target is always a path from `from`, the directory the link is written
/// in. There is no target that is read from the root instead.
pub fn target_path(from: &Path, target: &str) -> PathBuf {
    let target = target.split('#').next().unwrap_or(target);
    normalize(&from.join(target))
}

/// Folds `.` and `..` away, so that two spellings of one path compare equal.
pub fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other),
        }
    }
    out
}

/// The parts of `line` that a reader would see as text: the line with every
/// HTML comment removed, and with the part of a comment that opened on an
/// earlier line removed as well. `in_comment` carries the state from one line
/// to the next, since a comment spans lines.
fn visible_text(line: &str, in_comment: &mut bool) -> String {
    let mut text = String::new();
    let mut rest = line;
    loop {
        if *in_comment {
            match rest.find("-->") {
                Some(end) => {
                    *in_comment = false;
                    rest = &rest[end + 3..];
                }
                None => return text,
            }
        } else {
            match rest.find("<!--") {
                Some(start) => {
                    text.push_str(&rest[..start]);
                    *in_comment = true;
                    rest = &rest[start + 4..];
                }
                None => {
                    text.push_str(rest);
                    return text;
                }
            }
        }
    }
}

/// The links in a line of visible text. Code spans are dropped by `prose_runs`
/// before the search, so a `](` between a pair of backticks is not found.
fn links_in(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for run in prose_runs(text) {
        let mut rest = run;
        while let Some(open) = rest.find("](") {
            let after = &rest[open + 2..];
            let Some(close) = after.find(')') else {
                break;
            };
            let target = after[..close].trim();
            if !target.is_empty() && !target.contains(['(', ' ', '\\']) {
                found.push(target.to_owned());
            }
            rest = &after[close + 1..];
        }
    }
    found
}

/// The parts of a line that are not code: the line split at its backticks, with
/// the part between a pair of them dropped. The parts before and after a code
/// span are kept apart rather than joined, since a `](` cannot span one: the
/// backticks would be part of its link text, and a link with backticks in its
/// text is not the form a reference is written in.
///
/// A span is closed by a run of backticks of the same length as the one that
/// opened it, so `` `x` `` and `` ``x`` `` are both one span. An unpaired run
/// leaves the rest of the line as code: the span ends at the end of the line.
fn prose_runs(text: &str) -> Vec<&str> {
    let mut runs = Vec::new();
    let mut open: Option<usize> = None;
    let mut start = 0;
    let mut at = 0;
    while at < text.len() {
        if !text[at..].starts_with('`') {
            at += text[at..].chars().next().map_or(1, char::len_utf8);
            continue;
        }
        let run = text[at..].chars().take_while(|c| *c == '`').count();
        match open {
            Some(length) if length == run => {
                open = None;
                start = at + run;
            }
            Some(_) => {}
            None => {
                if start < at {
                    runs.push(&text[start..at]);
                }
                open = Some(run);
            }
        }
        at += run;
    }
    if open.is_none() && start < text.len() {
        runs.push(&text[start..]);
    }
    runs
}

/// The character and length of the run that opens a code fence, if this line
/// starts one: three or more backticks or tildes, optionally indented.
fn opens_fence(line: &str) -> Option<(char, usize)> {
    let stripped = line.trim_start();
    let marker = stripped.chars().next().filter(|c| *c == '`' || *c == '~')?;
    let length = stripped.chars().take_while(|c| *c == marker).count();
    (length >= 3).then_some((marker, length))
}

/// Says whether this line closes a fence opened by a run of `length` `marker`s:
/// a line of the same character, at least as long, and nothing else.
fn closes_fence(line: &str, marker: char, length: usize) -> bool {
    let stripped = line.trim();
    let run = stripped.chars().take_while(|c| *c == marker).count();
    run >= length && stripped[run..].is_empty()
}
