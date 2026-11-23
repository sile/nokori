//! What makes a path an item, and what a link to one is called.

use std::path::Path;

use crate::templates::{CLOSED_DIR, OPEN_DIR};

/// Says whether a path names an item: a file directly in `backlog/open/` or
/// `backlog/closed/` whose name is `YYYYMMDD-<kind>-<slug>.md`.
pub fn is_item_path(path: &Path) -> bool {
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    if parent != Path::new(OPEN_DIR) && parent != Path::new(CLOSED_DIR) {
        return false;
    }
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    let Some(stem) = name.strip_suffix(".md") else {
        return false;
    };
    let digits = stem.chars().take_while(char::is_ascii_digit).count();
    digits == 8 && stem[digits..].starts_with('-')
}

/// The label a line prints for an item: its file name without `.md`, which is
/// the text a link to it carries.
pub fn item_label(name: &str) -> String {
    link_name(name)
}

/// The name a link's target is written as when the link has to go: its text
/// without the path. The text of a link is the item's file name without
/// `.md` (see `nokori explain references`), so the name reads the same as the
/// link did.
pub fn link_name(target: &str) -> String {
    let path = Path::new(target);
    let file = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(target);
    file.strip_suffix(".md").unwrap_or(file).to_owned()
}
