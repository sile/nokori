//! Walking the links between items.
//!
//! The walk is driven by calls from the edge: the items are read there, and
//! [`Tree::new`] is handed the set it read. Nothing here opens a directory.

use std::path::{Path, PathBuf};

use crate::item::is_item_path;
use crate::links::normalize;
use crate::model::Item;
use crate::templates::{CLOSED_DIR, OPEN_DIR};

/// The walk from one item, as the text `tree` prints. Each item is printed
/// under the line that reached it, indented two spaces per hop, with a mark
/// saying which file holds the link that reached it: `-` for a link in the line
/// above, `<-` for a link in the line itself.
///
/// An item reached by two paths is printed under both, since deduplicating
/// would say which path was taken instead. What keeps the walk finite is the
/// set of items on the current path: a repeat there is a cycle, and the walk
/// stops and says so rather than hanging.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree {
    text: String,
}

impl Tree {
    /// Builds the walk from `file` over `items`, which is every item the edge
    /// read, keyed by [`Item::path`].
    pub fn new(file: &str, items: &[Item]) -> Result<Self, String> {
        let start = Path::new(file);
        let start = normalize(start);
        if !is_item_path(&start) {
            return Err(format!(
                r"`{file}` is not an item. `tree` walks the links between items, so it starts
from one in `{OPEN_DIR}/` or `{CLOSED_DIR}/`. Run `nokori explain overview` for what an item
is."
            ));
        }

        let Some(first) = items.iter().find(|item| item.path == start) else {
            return Err(format!(
                r"`{file}` is not an item. Nothing under `{OPEN_DIR}/` or `{CLOSED_DIR}/` has that
name. Pass the path of a file in one of them."
            ));
        };

        let mut text = String::new();
        text.push_str(&first.name);
        text.push('\n');
        let mut path = vec![first.path.clone()];
        walk_children(first, items, 1, &mut path, &mut text);
        Ok(Self { text })
    }

    /// The walk as the text `tree` prints.
    pub fn as_str(&self) -> &str {
        &self.text
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.text)
    }
}

/// Prints the items related to `item`, one hop out, and then theirs.
///
/// `path` is the items on the way down to `item`, which is what tells a cycle
/// from an item shared by two paths.
pub fn walk_children(
    item: &Item,
    items: &[Item],
    depth: usize,
    path: &mut Vec<PathBuf>,
    out: &mut String,
) {
    for (next, forward) in neighbours(item, items) {
        let indent = "  ".repeat(depth);
        let mark = if forward { "- " } else { "<- " };
        out.push_str(&format!("{indent}{mark}{}\n", next.name));

        if path.contains(&next.path) {
            // A cycle: the item is already on this path, so following it again
            // would print it forever. This is the only way the walk could not
            // end, and it is said here rather than refused before printing.
            out.push_str(&format!(
                "{}  (already on the path: the links run in a circle, so the walk stops here)\n",
                "  ".repeat(depth)
            ));
            continue;
        }
        path.push(next.path.clone());
        walk_children(next, items, depth + 1, path, out);
        path.pop();
    }
}

/// The items one hop from `item`, each with a mark saying where the link is:
/// `true` when the link is in `item` and points at the other one, `false` when
/// the link is in the other one and points back at `item`.
///
/// The ones `item` links come first, in the order the links are in its file,
/// since that is the order the writer wrote them in. The ones that link to it
/// follow, in the order the files are read from the directory.
fn neighbours<'a>(item: &'a Item, items: &'a [Item]) -> Vec<(&'a Item, bool)> {
    let mut found: Vec<(&Item, bool)> = Vec::new();
    for target in &item.links {
        if let Some(other) = items.iter().find(|candidate| &candidate.path == target) {
            found.push((other, true));
        }
    }
    for other in items {
        if other.path == item.path {
            continue;
        }
        if other.links.contains(&item.path) {
            found.push((other, false));
        }
    }
    found
}
