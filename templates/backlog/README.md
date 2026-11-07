# backlog

This directory holds the work items of this repository that are not finished
yet: proposals, bug reports, and other pieces of work. It is managed with
[`nokori`](https://github.com/sile/nokori), a markdown-file-based backlog tool.

## What this directory is for

An item here is a **record of work or a proposal**, not a specification. The
authoritative specification is the code and `README.md` of the crate; those
must make sense without this directory. The rule is one-way: `src/` and
`README.md` may be cited from an item, but they must never point back here.

## Flow

A new item is created with `nokori new` (command names are provisional) and
lives directly under `backlog/` while it is open. When the work is done, the
item is settled with `nokori finish` (provisional) and moved out of the way.
The exact command names and item kinds are defined by `nokori` itself; run
`nokori --help` for the current set, so that this file does not have to be
updated every time the tool changes.

## TODO

- Add `exclude = ["backlog/"]` to this repository's `Cargo.toml` so that the
  directory is left out of the published crate.
