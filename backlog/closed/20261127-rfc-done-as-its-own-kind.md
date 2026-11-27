# RFC: a done is an item that starts closed

## Summary

A `done` becomes a kind of item, and it is written already closed: `nokori
done` writes the file into `backlog/closed/` in one run, with the same
command growing a dedicated name the way `nokori feedback` has one.

It is the record of work that did not have an item while it was being done --
finished, and written down afterwards so that the next reader can find out
that it happened. A kind and a command are asked for here; whether the kind
earns its place over the alternatives is open, and the template is the thing
to settle first.

Rejected in `## Outcome`: a `done` is a `memo` with the state already
decided, and a kind that is never settled has no place in a backlog.

## Motivation

Some work never had an item, and the backlog has no shape for it. A `plan` is
written before the work so that the steps can be followed; a `bug` and an
`rfc` are written before the change so that it can be argued about. Work that
was finished in one sitting, with nobody to consult, has nothing to write
before it and still leaves the next reader asking why a thing is the way it
is.

Today that ends up in the commit message, which is where it fits least: a
commit says what changed, in a diff a reader can already see, and the reason
that does not fit in a diff has nowhere to go. The commit message is also the
one record that cannot be closed, revised, or cited -- a later item that
cites it cites a hash whose text is fixed by the moment it was made.

A backlog that only holds what was decided in advance is a backlog that
starts too late. The reason for a change is often known only when it is done,
and a record written at that point is worth as much to a reader a year later
as a plan written at the start is to the person carrying it out.

Why a command of its own matters more than the kind: `create` then writing
then `close` is three steps, and the first and the last are the tool's. A
record that is made after the fact is made where the work ended, and asking
for the path, then the file, then the close, is the friction that turns it
into a commit message.

## Design

`done` is added to `KINDS`, and `nokori done <slug>` writes
`backlog/closed/YYYYMMDD-done-<slug>.md` in one run. It is the only command
that writes into `backlog/closed/`, and the only one that writes an item and
closes it without the file ever being in `backlog/open/`.

`backlog/closed/` is created if it is not there, the same way `close` makes it
when the first item is closed.

The sections, as a template to be argued about:

- `## Summary` -- what was done, in one paragraph;
- `## Why` -- the reason, in the present tense, the way `Motivation` is
  written for an RFC;
- `## What was left out` -- what a reader would expect to be part of this and
  is not, and why not;
- `## Outcome` -- written in the same run, since the item is closed from the
  start.

The name is a slug like `create`'s, not the words of a person like
`nokori feedback`'s: the words going in are the writer's, and a writer who
can name the file can name a slug. The date is the day the run is made, which
is the day it is written and not necessarily the day the work was finished --
the template says that the `Summary` is where the date that matters goes if it
is another one.

`done` is written by whoever did the work, which is not what `nokori feedback`
is for: that one is the person who met the problem, reporting to the backlog
that is not theirs. A `done` is the doer's record of their own work and
belongs to the backlog it happened in, so it takes no path -- it writes into
the `backlog/` it is run beside, like `create`.

## Alternatives

**A `memo` kind instead: an item that was never planned and is not a
proposal.** The same file, reached from the other end -- a note of something
that was done, with no decision behind it and nothing to close. It is the
kind the backlog has been missing longest, since a note that never closes is
still an item. It loses, or wins, on what a `done` is for: `done` says a
thing is over and the reader can stop looking, where `memo` says only that it
was written down. Neither is taken up: an item that is never settled sits in
`backlog/open/` for good, and a backlog holding what will never be closed is
not what this one is for.

**`nokori done` writes the file and nothing else, with no kind.** The command
is the whole point, and the kind is what needs defending. It loses because
the directory is the state and the state is `closed/`, so a file there that
nothing knows how to write is a file `check` and `close` would have to grow a
rule for. If the kind goes, the command goes with it.

**Write the item with `create`, fill it in, close it.** No new kind, no new
command. It loses on the friction the Motivation is about: three runs for a
record that is made after the fact, in the middle of the sitting where the
work just ended.

**Write it in the commit message.** It is already there and costs nothing.
It loses because the commit is the record of the diff, not of the reason, and
the reason is the part a later reader cannot reconstruct. A commit message is
also fixed at the moment of the commit, where an item can at least be shaped
before it is closed.

## Open questions

Whether a kind earns its place over a `memo` kind, which would hold the same
file from the other end. Expected answer: this is the decision to make first,
and the alternatives section is where it is argued -- a `done` is `memo` with
the state already decided.

Whether `## What was left out` is worth a section. The other kinds have no
place to say what was not done, and a reader who expected something is the
one who cares. Expected answer: try it, and delete the section if nothing
lands there.

Whether the file goes to `backlog/closed/` with one run or two. Writing it
straight there is what makes the command worth having, and it is also the
only path that skips the close -- so a `done` whose sections are unwritten
would reach `backlog/closed/` without `close` ever refusing it. Expected
answer: the run refuses to write an empty file rather than writing one and
coming back, so the refusal `close` makes is made by `done` instead, on the
file it is about to write. This turned out to be the cost that decided the
RFC: the one run that makes the command worth having is the one run that
none of `close`'s checks are made in.

Whether `## Outcome` is a section at all. It is the section written at the
close, and there is no close here. Expected answer: keep it, since a `done`
is closed and an `Outcome` is what the other kinds mean by that, but write it
as the place the reason for closing would have gone if there had been an
earlier item to close against.

## Outcome

Rejected.

A `done` is a `memo` with the state already decided, and the alternative it
rests on was turned down with it: an item is written to be settled, and a
kind that is never settled sits in `backlog/open/` for good. A backlog that
fills with what will never close is not the one this tool is for, so the answer
to the question this RFC left open is that neither kind earns its place.

What that leaves is already here. Work that was never planned can be written
up afterwards with `plan`: the sections it wants are a goal and what was
done, `close` takes it as any other item, and nothing about the file says it
was written after the work rather than before. The one thing a `done` would
have added is the file never entering `backlog/open/`, and it buys that by
repeating every check `close` makes on the file it is about to write.
