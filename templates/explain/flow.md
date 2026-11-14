An item is created in `backlog/` and closed by moving it to `closed/`. Those
are the only two states, and the directory a file is in is the state.


Opening
=======

    nokori create <kind> <slug>

Writes `backlog/YYYYMMDD-<kind>-<slug>.md` from the template for that kind.
Every section starts with a `<!-- NOKORI_TODO: ... -->` comment saying what
belongs there; the file is written by replacing each comment with the section
it asks for and deleting the comment.

An item may stay in `backlog/` for as long as it needs to. `backlog/` means
everything not closed, whether or not anybody is working on it right now.


Closing
=======

    nokori close backlog/<file>

Moves the item to `closed/`, and creates `closed/` the first time. `close`
refuses the move in four cases, and each one is something to fix rather than
to work around:

- a `NOKORI_TODO:` comment is left, which means a section is not written;
- `## Outcome` is missing or empty, so there is nothing recording how it
  settled;
- a link points at an item in `backlog/`, or at a file that is not there, so
  the closed file would hold a link that is wrong and cannot be fixed;
- the file has changes that are not committed, so the commit that moves it
  would also be the commit that edits it.

The link check is the subject of `nokori explain references`, which says
which links it looks at and which it leaves alone.

`close` moves the file and touches nothing else. Committing the move is the
user's step, and is the last one: a file in `closed/` is not edited afterwards.

An item is closed when it is settled, not when the work is finished. A `bug`
that turns out not to be a bug, an `rfc` that is turned down, and a `plan`
whose steps were decided against are all closed, with the reason in
`## Outcome`.


Stopping without closing
========================

An item that is not settled stays in `backlog/` with the reason written in its
body. For a `plan`, that is the normal way to stop and resume later: the
unchecked steps say where the work stopped, so no other record is needed.

A `bug` that cannot be fixed for now, and an `rfc` whose open questions are
waiting on something, are the same: they stay open, and the body says why.


A file the command does not take
================================

Running `close` on a file already in `closed/` is an ordinary mistake, and the
error says only that the move is not allowed. What the states mean is above.

A closed item is left as it was written, so there is nothing for `close` to
do with it and nothing to fix. If something has changed since it closed, open
a new item that cites the closed one, rather than editing the record.

Passing a bare file name to `close` is also common: the item is probably
`backlog/<file>`, and `close` wants the path it can read.
