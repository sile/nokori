# backlog

This directory holds the work items of this repository that are not closed
yet: proposals, bug reports, and other pieces of work. It is managed with
[`nokori`](https://github.com/sile/nokori), a markdown-file-based backlog tool.

## What this directory is for

An item here is a **record of work or a proposal**, not a specification. The
authoritative specification is the code and `README.md` of the crate; those
must make sense without this directory. The rule is one-way: `src/` and
`README.md` may be cited from an item, but they must never point back here.

## Flow

A new item is created with `nokori create` and lives directly under `backlog/`
while it is open. When it is settled, the item is closed with `nokori close`
and moved to `closed/`. The exact command names and item kinds are defined by
`nokori` itself; run `nokori --help` to list them.
