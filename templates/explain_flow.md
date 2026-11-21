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


Writing down what somebody said
===============================

    nokori feedback <path>

The words come the way `git commit` takes a message: `-m` for text on the
command line, `-f` for a file, and standard input when neither is given.

`<path>` is the directory holding the backlog the item belongs to -- `.` for
this repository, or the root of another one. That is what makes this command
different from `create`: the person who met the problem is often not in the
repository the report belongs to, and the run happens where the words come
from rather than where the item lives.

The file is `backlog/open/YYYYMMDD-feedback-<HHMMSS>.md`. The name is the day
and the second it was written, with no slug: a slug is a summary, and a
summary is made by reading the words, which this run has not done. There is no
title line either, for the same reason. What the run knows is that somebody
said something, and when.

The rest of the flow happens in the repository it landed in. That repository
is the one that reads the words, decides what they turn into, and closes the
feedback with the decision in `## Outcome`. Nothing is committed by the run,
in either repository.

Putting an item in another repository is the one thing every other command
does not do, so it is one command rather than an option on `create`: an
option that is left out is silent, and the item lands in the wrong
repository without a word.

An item that settles another is not finished until the relation is written,
at one end or the other. One link holds it, since whoever has both files can
follow it from either side, so which end is the writer's. The usual one is
later to earlier: the writer of the new item is the one creating the file and
the target already exists.

When to write it matters. An open item may cite an open one, but a closed item
may not, so the half written in the item that closes first has only until that
close to be written. That is usually the item being worked on when the new one
is created, which is the same sitting -- both halves are ordinary edits to
ordinary files, and no command writes them for you.

Looking for the item to cite is also where a duplicate is noticed, which
`create` cannot do: it writes a file from a slug and reads nothing. What this
asks for is the search that would have found the earlier file.

It is not a ban on items that stand alone. A `bug` fixed on sight and work
nobody asked for are ordinary items, and a relation that is not there is not
one to invent -- what the rule asks is that you look. A `feedback` item is
the case where the look is optional: it is written by the person who hit
something, at the moment they hit it, and a duplicate is not damage there.


Checking what is written
========================

    nokori check backlog/open/<file>

Says whether the work is written down: a `NOKORI_TODO:` comment left in the
file stops it, and so does a link that does not resolve. Nothing else does.

It leaves out the sections only the close writes. `## Outcome` is *expected*
to be empty while the item is open, and its comment stays in place for the
whole of that time, so reporting it would report every item that anybody is
working on. A link into `backlog/open/` is allowed -- the rule against it is
about a file that has moved.

When nothing is reported, the item is as far as it can be without the close,
and the output says so.

What it is for is the stretch between `create` and `close`, when the file is
rewritten over one sitting or several and nothing else looks at it. `create`
names the command in the output that wrote the file, so it is found from the
run that made the work rather than from the help.

Running it is not a step in the flow, and `close` does not want it to have
been run: `close` makes the checks itself, and a file moved without a
`check` is closed the same way. The two differ in what they leave out, since
the close is where `## Outcome` is written and the point of a check is to be
run before that.


Closing
=======

    nokori close backlog/open/<file>

Moves the item to `backlog/closed/`. `close` refuses the move in three cases,
and each one is something to fix rather than to work around -- the first is
the one `check` reports:

- a `NOKORI_TODO:` comment is left, which means a section is not written;
- `## Outcome` is missing or empty, so there is nothing recording how it
  settled;
- a link in the file would not resolve once it is in `backlog/closed/`, or
  would name an item still in `backlog/open/`, so the closed file would hold a
  link that is wrong and cannot be fixed. The second of those is the normal
  state while two items answer each other, and the message says what to write
  instead: the target's name in a code span where the link is, plus the link to
  add if the other file does not already hold it. `nokori explain references`
  has the form.

Nothing is asked of git. Whether the file is committed is not `close`'s
question: an item is written in the same sitting it is closed in, and a rule
that made that two commits would be paid on every item to guard the one run
that closed a file by mistake.

The link check is the subject of `nokori explain references`, which says
which links it looks at and which it leaves alone.

`close` moves the file and changes nothing in it. Committing that, with the
`## Outcome` that was written just before it, is the user's step and the last
one, because a file in `backlog/closed/` is not edited afterwards. A link that
was already wrong when the file reached `backlog/closed/` stays as it is, for
the same reason.

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
