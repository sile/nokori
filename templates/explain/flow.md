An item is created in `backlog/open/` and closed by moving it to
`backlog/closed/`. Those are the only two states, and the directory a file is
in is the state. Both are under `backlog/`, so the backlog is one directory,
and neither sits next to the source tree.


Opening
=======

    nokori create <kind> <slug>

Writes `backlog/open/YYYYMMDD-<kind>-<slug>.md` from the template for that
kind. Every section starts with a `<!-- NOKORI_TODO: ... -->` comment saying
what belongs there; the file is written by replacing each comment with the
section it asks for and deleting the comment.

An item may stay in `backlog/open/` for as long as it needs to. The open
directory means everything not closed, whether or not anybody is working on
it right now.


Closing
=======

    nokori close backlog/open/<file>

Moves the item to `backlog/closed/`. `close` refuses the move in four cases,
and each one is something to fix rather than to work around:

- a `NOKORI_TODO:` comment is left, which means a section is not written;
- `## Outcome` is missing or empty, so there is nothing recording how it
  settled;
- a link in the file would not resolve once it is in `backlog/closed/`, or
  would name an item still in `backlog/open/`, so the closed file would hold a
  link that is wrong and cannot be fixed;
- the file has changes that are not committed, so the commit that moves it
  would also be the commit that edits it.

The link check is the subject of `nokori explain references`, which says
which links it looks at and which it leaves alone.

`close` moves the file and changes nothing in it. Committing the move is the
user's step, and it is the last one, because a file in `backlog/closed/` is
not edited afterwards. A link that was already wrong when the file reached
`backlog/closed/` stays as it is, for the same reason.

An item is closed when it is settled, not when the work is finished. A `bug`
that turns out not to be a bug, an `rfc` that is turned down, and a `plan`
whose steps were decided against are all closed, with the reason in
`## Outcome`.

An `rfc` that is accepted is the case those examples do not cover, and it
stays open until the change it names has landed. The decision is settled when
the RFC is accepted, so it could be argued that the file is ready to close
then. It is not, because an accepted RFC is still being read against the code
while the work is carried out: a detail of the design often turns out wrong,
and sometimes the whole change is dropped. Each of those is an edit to the
proposal, and a closed file is never edited. So the RFC closes when the
implementation lands, and its `## Outcome` names the change that implemented
it.

Nothing here is a rule `close` enforces. It reads the file, not whether the
change it names has landed, so an accepted RFC closed ahead of its
implementation is refused only if the file itself is unfinished -- an empty
`## Outcome`, or a link that the move would break. What keeps such an RFC open
is the habit this section describes, and the reason for it: a design that is
edited after it closed leaves a record that says something no longer true.


Stopping without closing
========================

An item that is not settled stays in `backlog/open/` with the reason written
in its body. For a `plan`, that is the normal way to stop and resume later:
the unchecked steps say where the work stopped, so no other record is needed.

A `bug` that cannot be fixed for now, and an `rfc` whose open questions are
waiting on something, are the same: they stay open, and the body says why.


A file the command does not take
================================

Running `close` on a file already in `backlog/closed/` is an ordinary
mistake, and the error says only that the move is not allowed. What the states
mean is above.

A closed item is left as it was written, so there is nothing for `close` to
do with it and nothing to fix. If something has changed since it closed, open
a new item that cites the closed one, rather than editing the record.

Passing a bare file name to `close` is also common: the item is probably
`backlog/open/<file>`, and `close` wants the path it can read.
