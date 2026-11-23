//! The names a run produces, and the text a template becomes.
//!
//! Nothing here reads a clock or a file: a run hands in the [`Clock`] it read,
//! and gets back the name to write under `backlog/open/`.

use std::path::PathBuf;

use crate::templates::{
    FEEDBACK_KIND, FEEDBACK_SECTION, FEEDBACK_TITLE_PREFIX, KINDS, OPEN_DIR, TODO_MARKER,
};

/// The day and the time, read in one go so that a name made of both is of one
/// moment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clock {
    /// The date as `YYYYMMDD`.
    pub date: String,

    /// The time as `HHMMSS`.
    pub time: String,
}

/// Says whether `slug` can go into a file name unquoted.
pub fn is_valid_slug(slug: &str) -> bool {
    let allowed = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_';
    !slug.is_empty() && slug.chars().all(allowed)
}

/// The item `create` would write for `kind` and `slug`, once `clock` says which
/// day it is, or the message saying what is wrong with `kind` or `slug`.
///
/// The path is returned without being looked at on disk: whether an item of
/// that name is already there is the caller's to find out, since only the
/// caller knows the directory it would be written in.
pub fn item_name(clock: &Clock, kind: &str, slug: &str) -> Result<PathBuf, String> {
    if !KINDS.iter().any(|(k, _)| *k == kind) {
        let known: Vec<&str> = KINDS.iter().map(|(k, _)| *k).collect();
        return Err(format!(
            "`{kind}` is not a known kind. The kinds are: {}.",
            known.join(", ")
        ));
    }

    if !is_valid_slug(slug) {
        return Err(format!(
            r"`{slug}` cannot be used in a filename. Use lowercase letters, digits,
and `-` or `_`, so that the file name needs no quoting."
        ));
    }

    let name = format!("{}-{kind}-{slug}.md", clock.date);
    Ok(std::path::Path::new(OPEN_DIR).join(name))
}

/// The name `feedback` writes under `backlog/open/`: the day and the second,
/// with no slug, since a slug is a summary and the run has not read the words
/// yet.
pub fn feedback_name(clock: &Clock) -> PathBuf {
    let name = format!("{}-{FEEDBACK_KIND}-{}.md", clock.date, clock.time);
    std::path::Path::new(OPEN_DIR).join(name)
}

/// The feedback item as it is written: the template without its title line,
/// with the words in place of the comment in `## Feedback`.
///
/// Both edits are made here rather than in the template because `create`
/// writes the same template with its title line kept.
pub fn feedback_content(template: &str, words: &str) -> Result<String, String> {
    let mut out = String::new();
    let mut dropped_title = false;
    let mut body = template;
    if let Some(rest) = body.strip_prefix(FEEDBACK_TITLE_PREFIX) {
        let end = rest.find('\n').map_or(rest.len(), |i| i + 1);
        body = &rest[end.min(rest.len())..];
        dropped_title = true;
    }
    if !dropped_title {
        return Err(format!(
            r"the `{FEEDBACK_KIND}` template does not start with `{FEEDBACK_TITLE_PREFIX}`,
so the title line could not be dropped. This is a bug in nokori."
        ));
    }

    let heading = format!("{FEEDBACK_SECTION}\n");
    let Some(at) = body.find(&heading) else {
        return Err(format!(
            r"the `{FEEDBACK_KIND}` template has no `{FEEDBACK_SECTION}` section, so the words
have nowhere to go. This is a bug in nokori."
        ));
    };
    let (before, after_heading) = body.split_at(at + heading.len());
    out.push_str(&before_without_leading_blank(before));
    out.push_str(words);
    out.push('\n');

    // What follows the words is the rest of the template with the comment in
    // this section taken out. The comment is the answer the words just gave.
    let rest = drop_section_comment(after_heading);
    out.push_str(&rest);
    Ok(out)
}

/// Removes the blank line the title line's removal left above the first
/// heading.
fn before_without_leading_blank(before: &str) -> String {
    let trimmed = before.trim_start_matches('\n');
    if trimmed.is_empty() {
        String::new()
    } else {
        let mut out = trimmed.to_owned();
        out.push('\n');
        out
    }
}

/// Removes the leading `<!-- NOKORI_TODO: ... -->` comment, which is the one
/// asking for the section the words now fill.
fn drop_section_comment(rest: &str) -> String {
    let trimmed = rest.trim_start_matches('\n');
    let Some(body) = trimmed.strip_prefix("<!--") else {
        return trimmed.to_owned();
    };
    if !body.contains(TODO_MARKER) {
        return trimmed.to_owned();
    }
    let Some(close) = body.find("-->") else {
        return trimmed.to_owned();
    };
    let after = &body[close + 3..];
    let after = after.trim_start_matches('\n');
    let mut out = String::from("\n");
    out.push_str(after);
    out
}

/// An item read for the walk: the file it is in, the label a line prints, and
/// the items it links.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    /// The path of the file, normalized.
    pub path: PathBuf,

    /// The label a line prints: the file name without `.md`.
    pub name: String,

    /// The items this one links, as normalized paths.
    pub links: Vec<PathBuf>,
}
