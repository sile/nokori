//! `nokori`'s Sans I/O core.
//!
//! This crate holds everything that does not touch the file system, a process,
//! or a clock: the shape of the backlog ([`OPEN_DIR`], [`CLOSED_DIR`]), the
//! templates an item is written from ([`KINDS`], [`TOPICS`]), the readings of
//! a markdown file that the commands are decided on ([`links`], [`find_marker`]),
//! and the decisions each command makes once its inputs are in hand.
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

mod check;
mod close;
mod item;
mod links;
mod marker;
mod model;
mod templates;
mod tree;

pub use check::{ItemSource, check_item};
pub use close::{CloseDecision, CloseSource, check_filled_in, close_item};
pub use item::{is_item_path, item_label, link_name};
pub use links::{Link, is_external, links, normalize, target_path};
pub use marker::find_marker;
pub use model::{Clock, Item, feedback_content, feedback_name, is_valid_slug, item_name};
pub use templates::{
    BACKLOG_DIR, BACKLOG_README_TEMPLATE, CLOSED_DIR, CLOSED_README_TEMPLATE, KINDS, OPEN_DIR,
    OPEN_README_TEMPLATE, OUTCOME_SECTION, README_FILES, TODO_MARKER, TOPICS, feedback_template,
    template_for, topic,
};
pub use tree::Tree;
