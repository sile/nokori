nokori
======

[![Crates.io](https://img.shields.io/crates/v/nokori.svg)](https://crates.io/crates/nokori)
[![Actions Status](https://github.com/sile/nokori/workflows/CI/badge.svg)](https://github.com/sile/nokori/actions)
![License](https://img.shields.io/crates/l/nokori)

A backlog management CLI tool, with the backlog kept as markdown files.

A backlog of items, kept as ordinary text files. Nothing extra is added to
the filesystem, so searching the backlog is `grep`, reading it is your
editor. The `nokori` commands tell you how to use them as you go, so you can
start without reading anything first -- `nokori init` is the first one.

nokori is Japanese for "what is left" (残り).


What it manages
---------------

An item is one file, and its kind is part of its name.

- `rfc` -- a proposal, and the decision on it.
- `bug` -- a problem, and what was done about it.
- `plan` -- a piece of work, and its progress.
- `feedback` -- words from somebody else, kept as they arrived. The run
  records what was said, not a summary of it.

An item is *open* while it is in `backlog/open/`, and *closed* once it has
been moved to `backlog/closed/`. Those are the only two states, and the
directory a file is in is the state; the file itself has no status field.


Installation
------------

```console
$ cargo install nokori

$ nokori --help
```


Overview
--------

`nokori init` creates this, and nothing else:

```
backlog/
  README.md
  open/
    README.md
  closed/
    README.md
```

An item lives in `backlog/open/` as `YYYYMMDD-<kind>-<slug>.md`, and `nokori
close` moves it to `backlog/closed/`. Everything under `backlog/` can be read
by any editor, and edited by hand.


Getting started
---------------

```console
$ nokori init

$ nokori create bug panic-on-empty-input

$ $EDITOR backlog/open/20250101-bug-panic-on-empty-input.md

$ nokori check backlog/open/20250101-bug-panic-on-empty-input.md

$ nokori close backlog/open/20250101-bug-panic-on-empty-input.md
```

The file `create` writes has a `<!-- NOKORI_TODO: ... -->` in every section,
saying what belongs there; you write the section and delete the comment.
`create` also shows how to run `check`, which tells you which comments are
left. `close` refuses to move the file while any remain, or while
`## Outcome` is still empty; once both are written, it moves the file to
`backlog/closed/` without changing a byte of it.

When somebody outside the repository sends a report, `nokori feedback <path>`
writes it down as it was said.

Reference material that does not follow from a command lives under `nokori
explain <topic>`: `overview`, `kinds`, `flow`, `feedback`, and `references`.
