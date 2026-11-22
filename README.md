nokori
======

[![Crates.io](https://img.shields.io/crates/v/nokori.svg)](https://crates.io/crates/nokori)
[![Actions Status](https://github.com/sile/nokori/workflows/CI/badge.svg)](https://github.com/sile/nokori/actions)
![License](https://img.shields.io/crates/l/nokori)

nokori is Japanese for "what is left" (残り).

It is a backlog management CLI tool, with the backlog kept as markdown
files.

A backlog of items, each a file under `backlog/open/` or `backlog/closed/`.
The usual tools work on them -- `grep` to search, your editor to read and
write.

There is nothing to learn before you start: each command tells you what to
run next, and `nokori init` is the first.


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
