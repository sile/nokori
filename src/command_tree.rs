//! Walking the links between items.
//!
//! The walk is driven by calls from the edge: the items are read there, and
//! [`Tree::new`] is handed the set it read. Nothing here opens a directory.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::item::{is_item_path, state_label};
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

/// The walk from each item `tree` was given, as the text it prints.
///
/// Each item is printed under the line that reached it, drawn with `cargo
/// tree`'s branches: `├── ` for a line with more to follow at its depth, `└── `
/// for the last one, and `│   ` carrying the depth above where a line below is
/// still there.
///
/// A root is the first line of its walk and is drawn without a branch, as
/// `cargo tree` prints the package it was given. Nothing in the output says
/// which file holds the link that reached a line: the run said the direction
/// once, so every line under a root has it.
///
/// Every line starts with `open/` or `closed/`, which is the directory the
/// item is in. The root's line does too: a run whose first line says which
/// state it started from is one a reader can check against every line below.
///
/// An item is printed once below a given line however many links reach it, and
/// an item the run has already printed is printed with `(*)` and not walked
/// again. That is `cargo tree`'s rule, for `cargo tree`'s reason: the subtree
/// is under the first printing, and the mark says where to read it. The set of
/// items printed is what keeps the walk finite: a cycle is an item the walk has
/// just printed, so it is marked like any other and the walk ends there.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree {
    text: String,
}

impl Tree {
    /// Builds the walk from `file` over `items`, which is every item the edge
    /// read, keyed by [`Item::path`], following the direction asked for.
    pub fn new(file: &str, items: &[Item], direction: Direction) -> Result<Self, String> {
        Self::of(&[file], items, direction)
    }

    /// Builds one walk per file in `files`, in the order given, each separated
    /// from the next by a blank line.
    pub fn of(files: &[&str], items: &[Item], direction: Direction) -> Result<Self, String> {
        let mut roots = Vec::with_capacity(files.len());
        for file in files {
            let path = normalize(Path::new(file));
            if !is_item_path(&path) {
                return Err(format!(
                    r"`{file}` is not an item. `tree` walks the links between items, so it starts
from one in `{OPEN_DIR}/` or `{CLOSED_DIR}/`. Run `nokori explain overview` for what an item
is."
                ));
            }
            let Some(item) = items.iter().find(|item| item.path == path) else {
                return Err(format!(
                    r"`{file}` is not an item. Nothing under `{OPEN_DIR}/` or `{CLOSED_DIR}/` has that
name. Pass the path of a file in one of them."
                ));
            };
            roots.push(item);
        }
        Ok(tree_of(&roots, items, direction))
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

/// Prints the walk from each of `roots`, over `items`, following `direction`.
///
/// A root is printed without a branch and its tree follows under it. Each root
/// is a walk of its own: the printed set starts empty at every root, so an item
/// reached under the first is not `(*)` under the second. The roots are the
/// questions the run was asked, and a mark on one says it was shown under
/// another, which is a fact about that other root rather than about this one.
pub fn tree_of(roots: &[&Item], items: &[Item], direction: Direction) -> Tree {
    let mut text = String::new();
    for (index, root) in roots.iter().enumerate() {
        if index > 0 {
            text.push('\n');
        }
        text.push_str(&state_label(&root.path));
        text.push('\n');
        let mut printed = HashSet::from([root.path.clone()]);
        walk_children(
            root,
            items,
            direction,
            &mut String::new(),
            &mut printed,
            &mut text,
        );
    }
    Tree { text }
}

/// Prints the items one hop from `item`, and then theirs.
///
/// `prefix` is the branch drawing owed to the ancestors of `item`: `│   ` for
/// each level whose siblings still follow. `printed` is every item the walk has
/// shown, which is what tells a line to mark rather than walk.
fn walk_children(
    item: &Item,
    items: &[Item],
    direction: Direction,
    prefix: &mut String,
    printed: &mut HashSet<PathBuf>,
    out: &mut String,
) {
    let found = neighbours(item, items, direction);
    let count = found.len();
    for (index, next) in found.into_iter().enumerate() {
        let last = index + 1 == count;
        let branch = if last { "└── " } else { "├── " };
        let below = if last { "    " } else { "│   " };

        if !printed.insert(next.path.clone()) {
            // Shown already, under an earlier line. The entry is printed so
            // that the reader following this path finds where it goes, and its
            // subtree is not, since it is under the first printing. A cycle is
            // this same case -- the item is on the path because the walk has
            // just printed it -- so there is no second rule to ask.
            out.push_str(&format!(
                "{prefix}{branch}{} (*)\n",
                state_label(&next.path)
            ));
            continue;
        }

        out.push_str(&format!("{prefix}{branch}{}\n", state_label(&next.path)));
        let depth = prefix.len();
        prefix.push_str(below);
        walk_children(next, items, direction, prefix, printed, out);
        prefix.truncate(depth);
    }
}

/// The items one hop from `item` in the direction asked for, each once.
///
/// Following the links `item` holds, the order is the one the links are in its
/// file, since that is the order the writer wrote them in. Following the links
/// that point at `item`, the order is the one the files are read from the
/// directories.
///
/// An item more than one link reaches is returned once, at the place of the
/// first link: a line under a name is that item as its neighbour, and it is
/// one neighbour however many times the file names it.
///
/// A link is followed only where it reaches an item the edge read: a link to a
/// source file, a URL, or a heading is prose around the walk rather than part
/// of it. A self-link reaches the item itself, which the walk has printed, so
/// it is printed and stopped at like any other cycle.
fn neighbours<'a>(item: &'a Item, items: &'a [Item], direction: Direction) -> Vec<&'a Item> {
    let mut found: Vec<&Item> = Vec::new();
    let mut seen: HashSet<&Path> = HashSet::new();
    let mut add = |candidate: &'a Item| {
        if seen.insert(candidate.path.as_path()) {
            found.push(candidate);
        }
    };

    match direction {
        Direction::Holds => {
            for target in &item.links {
                if let Some(candidate) = items.iter().find(|candidate| &candidate.path == target) {
                    add(candidate);
                }
            }
        }
        Direction::PointsAt => {
            for other in items {
                if other.links.contains(&item.path) {
                    add(other);
                }
            }
        }
    }
    found
}
