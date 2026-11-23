//! `nokori`'s Sans I/O core.
//!
//! This crate holds everything that does not touch the file system, a process,
//! or a clock: the shape of the backlog ([`OPEN_DIR`], [`CLOSED_DIR`]), the
//! templates an item is written from ([`KINDS`], [`TOPICS`]), the readings of
//! a markdown file that the commands are decided on ([`links`], [`find_marker`]),
//! and the decisions each command makes once its inputs are in hand.
//!
//! The modules are named for what they hold: a module named after a command,
//! with a `command_` prefix (`command_check`, `command_close`,
//! `command_tree`), holds what that one command decides, and reads from the
//! rest; a module without the prefix is shared -- `marker` is the undone work
//! the check and the close both look for, `links` is what `check`, `close`,
//! `tree`, and `item` all read, and `templates` is the text every command
//! writes or prints.
//!
//! What the core does not do is perform I/O. A command that reads a file is
//! handed the text, and one that writes it is handed back the text to write:
//! [`check_item`] takes [`ItemSource`], [`close_item`] takes [`CloseSource`]
//! and returns a [`CloseDecision`], and [`Tree::new`] takes the items the edge
//! read. The binary in `main.rs` is the only place that opens a file, runs a
//! process, or prints, and the messages the core returns are plain [`String`]s
//! for it to report.

#![warn(missing_docs)]
#![forbid(unsafe_code)]

mod command_check;
mod command_close;
mod command_tree;
mod item;
mod links;
mod marker;
mod model;
mod templates;

pub use command_check::{ItemSource, check_item};
pub use command_close::{
    CloseDecision, CloseSource, check_filled_in, close_item, open_items_linking,
};
pub use command_tree::{Direction, Tree};
pub use item::{is_item_path, item_label, link_name};
pub use links::{Link, is_external, links, normalize, target_path};
pub use marker::find_marker;
pub use model::{Clock, Item, feedback_content, feedback_name, is_valid_slug, item_name};
pub use templates::{
    BACKLOG_DIR, BACKLOG_README_TEMPLATE, CLOSED_DIR, CLOSED_README_TEMPLATE, KINDS, OPEN_DIR,
    OPEN_README_TEMPLATE, OUTCOME_SECTION, README_FILES, TODO_MARKER, TOPICS, feedback_template,
    template_for, topic,
};
