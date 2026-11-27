nokori
======

[![Crates.io](https://img.shields.io/crates/v/nokori.svg)](https://crates.io/crates/nokori)
[![Actions Status](https://github.com/sile/nokori/workflows/CI/badge.svg)](https://github.com/sile/nokori/actions)
![License](https://img.shields.io/crates/l/nokori)

nokori is Japanese for "what is left" (残り).

It is a backlog management CLI tool, with the backlog kept as markdown
files.

A backlog of items, each a file under `backlog/open/` or `backlog/closed/`.
Ordinary markdown files, so the tools you already have keep working: `grep`
to search, your editor to read and write, `git` to keep the history. Use the
ones you want, and none of them if you do not.

There is nothing to learn before you start: each command tells you what to
run next, and `nokori init` is the first.


Installation
------------

```shell
cargo install nokori
nokori --help
```


Basic usage
-----------

```shell
# Set up backlog/ directory.
nokori init

# Create an item file from the template for its kind.
nokori create <kind> <slug>

# Write the sections, then fill in the outcome once the work is done.
$EDITOR <item_file>

# Close the item.
nokori close <item_file>
```


Backlog items
-------------

An item is one file, and its kind is part of its name:
`backlog/open/YYYYMMDD-<kind>-<slug>.md` while it is open, and the same
name in `backlog/closed/` once it is closed.

- `rfc` -- a proposal, and the decision on it.
- `bug` -- a problem, and what was done about it.
- `plan` -- a piece of work, and its progress.
- `feedback` -- words from somebody else, kept as they arrived.

An item is either *open* or *closed*, and the directory it is in says which.
There is no other state and no status field in the file; `nokori close` is
what moves one from the first to the second.


Reference
---------

More about each part is under `nokori explain <topic>`:

- `overview` -- what a backlog is, and what the tool does not do.
- `kinds` -- the kinds, and how to choose between them.
- `flow` -- how an item is opened, checked while it is written, and closed.
- `references` -- how one item cites another.
- `feedback` -- how a `feedback` is written down.
