//! Walking the links between items.
//!
//! The walk is driven by calls from the edge: the items are read there, and
//! [`Tree::new`] is handed the set it read. Nothing here opens a directory.

use std::path::{Path, PathBuf};

use crate::item::is_item_path;
use crate::links::normalize;
use crate::model::Item;
use crate::templates::{CLOSED_DIR, OPEN_DIR};

/// Which half of a relation the walk follows.
///
/// The graph is the same either way; what the direction chooses is which end
/// of a link a step leaves from. `cargo tree -i` is the same flag for the same
/// reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// The links the item holds, walked outward.
    Holds,

    /// The links that point at the item, walked inward.
    PointsAt,
}

/// The walk from one item, as the text `tree` prints.
///
/// Each item is printed under the line that reached it, drawn with `cargo
/// tree`'s branches: `├── ` for a line with more to follow at its depth, `└── `
/// for the last one, and `│   ` carrying the depth above where a line below is
/// still there.
///
/// The root is the first line and is drawn without a branch, as `cargo tree`
/// prints the package it was given. Nothing in the output says which file holds
/// the link that reached a line: the run said the direction once, so every line
/// under the root has it.
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
    /// read, keyed by [`Item::path`], following the direction asked for.
    pub fn new(file: &str, items: &[Item], direction: Direction) -> Result<Self, String> {
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
        let mut prefix = String::new();
        walk_children(first, items, direction, &mut prefix, &mut path, &mut text);
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

/// Prints the items one hop from `item`, and then theirs.
///
/// `prefix` is the branch drawing owed to the ancestors of `item`: `│   ` for
/// each level whose siblings still follow. `path` is the items on the way down
/// to `item`, which is what tells a cycle from an item shared by two paths.
pub fn walk_children(
    item: &Item,
    items: &[Item],
    direction: Direction,
    prefix: &mut String,
    path: &mut Vec<PathBuf>,
    out: &mut String,
) {
    let found = neighbours(item, items, direction);
    let count = found.len();
    for (index, next) in found.into_iter().enumerate() {
        let last = index + 1 == count;
        let branch = if last { "└── " } else { "├── " };
        out.push_str(&format!("{prefix}{branch}{}\n", next.name));

        if path.contains(&next.path) {
            // A cycle: the item is already on this path, so following it again
            // would print it forever. This is the only way the walk could not
            // end, and it is said here rather than refused before printing.
            let below = if last { "    " } else { "│   " };
            out.push_str(&format!(
                "{prefix}{below}(already on the path: the links run in a circle, so the walk stops \
                 here)\n"
            ));
            continue;
        }
        path.push(next.path.clone());
        let depth = prefix.len();
        prefix.push_str(if last { "    " } else { "│   " });
        walk_children(next, items, direction, prefix, path, out);
        prefix.truncate(depth);
        path.pop();
    }
}

/// The items one hop from `item` in the direction asked for.
///
/// Following the links `item` holds, the order is the one the links are in its
/// file, since that is the order the writer wrote them in. Following the links
/// that point at `item`, the order is the one the files are read from the
/// directories.
///
/// A link is followed only where it reaches an item the edge read: a link to a
/// source file, a URL, or a heading is prose around the walk rather than part
/// of it. A self-link reaches the item itself, which is on the path already, so
/// it is printed and stopped at like any other cycle.
fn neighbours<'a>(item: &'a Item, items: &'a [Item], direction: Direction) -> Vec<&'a Item> {
    match direction {
        Direction::Holds => item
            .links
            .iter()
            .filter_map(|target| items.iter().find(|candidate| &candidate.path == target))
            .collect(),
        Direction::PointsAt => items
            .iter()
            .filter(|other| other.links.contains(&item.path))
            .collect(),
    }
}
