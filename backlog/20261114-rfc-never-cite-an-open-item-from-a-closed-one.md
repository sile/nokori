# RFC: never cite an open item from a closed one

## Summary

An item that is closed may cite another item that is closed, but never one
that is still open. The allowed directions are closed-to-closed,
open-to-closed, and open-to-open; closed-to-open is the one to avoid.

This holds for the whole file, not only for `Outcome`: a closed item is not
edited anywhere, so a reference in `Motivation` rots as surely as one in the
outcome. What this buys is that a closed item's references never rot. A
closed file is not edited again, so a reference it holds is permanent, and a
reference into `backlog/` is a reference to something that can still be
renamed or deleted.

The rule is checkable, which is most of why it is worth writing down. A
reference is a markdown link (see
[rfc-how-items-refer-to-each-other](20261114-rfc-how-items-refer-to-each-other.md)),
so `close` can see that a file in `closed/` points into `backlog/` and refuse
the move. The rule is not left to a writer to remember at the moment the file
stops being theirs.

## Motivation

The `Outcome` comment says to name the change that carried the item out, "or
the item that does, if the work happens somewhere else". Following that
sentence, a `feedback` item closed today could link the `rfc` that answers
it -- and that `rfc` is a file in `backlog/` whose slug is not fixed.

This is not hypothetical. The feedback item on renaming the plan template's
`Items` section was closed naming a proposed `rfc`; by the time the `rfc` was
written, what it proposed had changed -- the section was renamed first and
then dropped -- and the name the closed item had used was never the name of
any file. The closed item cannot be corrected, because a closed item is not
edited. The reference is there, it looks like a reference, and it points at
nothing.

With a link, the same mistake would have been caught. A link to a file that
does not exist does not resolve, and a link is exactly what `close` can look
at.

Naming an open item is also a claim about what that item will become. A
proposal can be rejected, reshaped, or renamed after somebody reads it; a
closed item that links to a plan still in `backlog/` has committed to an
outcome it cannot see, in a file it will not revisit. The same is true of a
link mentioned in passing while explaining where the idea came from, which is
why the rule is about the file and not about the one section where an answer
is expected.

The other three directions do not have this problem, which is why the rule
is a direction and not a ban on citing items altogether. Open-to-closed cites
something finished and fixed. Open-to-open cites something the writer can
still fix, and should, if it moves. Closed-to-closed cites something that is
as permanent as the citing file.

## Design

One rule, in the comment under `Outcome` in all four templates:

> Name the change that carried this out, or link the item that did if the
> work happened somewhere else -- but only if that item is already closed.
> Never link an item that is still open, not even as background.

The rule is about the file, not about `Outcome` alone: every section of a
closed item is frozen, so every section can hold a reference that goes stale.
`Outcome` is where the rule is written because it is the section written
last, but it is not the only section that outlives the writer's chance to fix
it.

The rule has no exception for a reference that is only being explained. An
exception reads as a smaller rule and is a larger one: "background" and "the
answer" are not two kinds of sentence but two readings of the same one, so
every writer would be deciding which of the two their sentence is, at the
moment they are writing the last thing the file will ever say. And a reader
of the closed file sees a link and has no way to tell which reading the
writer intended.

For `close` the exception would be worse than for a reader. A link is
resolvable, so the check is one line of code; "this link is only background"
is a judgment about prose that no program can make. The rule is written so
that it can be enforced by the command that makes it permanent.

Nothing is lost by having no exception. The two cases that look like they
need one do not:

- **An item answering a proposal that is still open.** The item is not
  settled yet, so it is not closed: it stays in `backlog/` with its outcome
  unwritten, and the reference is written by whoever closes the item that
  finished the work, pointing back at this one.
- **An item explaining what an open item proposes.** Say what the proposal
  is, without the name. `An RFC was floated to split the lexer` says
  everything the reader needs and holds nothing that can go stale. If the
  name is what matters to the reader, the item is not closed.

Where the rule appears is the place a writer will be when breaking it. The
`Outcome` comment is read at `close` time, which is the moment the file
becomes permanent; the `Sections:` line in `explain kinds` is not.

## Alternatives

**Ban linking items in `Outcome` altogether.** Simplest to state and to
check, and it loses the case the sentence exists for: two items close
together when one carries out the other, and a reader of the later one wants
the earlier. Closed-to-closed is the case with no downside, so banning it
costs more than the rule it replaces.

**Leave it to a later `check` command that reports dangling links.** This
keeps every direction writable and reports the ones that break. It fixes
nothing, and the one thing that breaks worst cannot be fixed at all: a
closed file is not edited, so the report says "this file has a bad link"
about a file that may not be touched. A check is the right tool for a
problem that still has a fix, and this is the one that does not -- which is
why the check goes in `close`, before the file stops being fixable.

**Say it once in `backlog/README.md` instead of in every template.** One
place to change, and it is the file a reader of the directory opens first.
It is not the file a writer has open at `close` time, though, and `README.md`
is prose about the directory rather than a comment inside the kind being
written.

## Open questions

None.

The check is settled and is part of the design rather than left to a later
command: `close` refuses a file whose link points into `backlog/`, because
the file is about to be in `closed/` and that link would be the forbidden
direction. The same check refuses a link that does not resolve at all, which
covers the link that broke when the item it named closed.

The one case that looked like it needed an exception -- proposing that a
future item will answer this one -- is answered by the direction of the rule
rather than by an exception: the reference is written by the item that
answers it, on the day it is created, so there is never a moment when an open
item has to be named from a closing one.

## Outcome

<!-- NOKORI_TODO: the one section written later, when `nokori close` moves
this file. Leave it empty until then, comment and all.

The sections above are written before the item moves: by the time `nokori
close` moves this file, they are written, their comments are gone, and the
change is committed.

Those sections may be rewritten as long as the item is open, as often as the
understanding changes. What they may not become is a description of work
already done. `nokori close` needs the change committed, so whatever they say
at that point is what the moving commit records.

Say whether the RFC was accepted or rejected, and why. When it is accepted,
name the change that implements it, or the item that does, if the work happens
somewhere else.

An accepted RFC can still be overturned later. Say so in the later item and
cite this one; do not edit this file afterwards to follow it.

Once this section is written and committed, treat the file as closed: do not
edit any part of it again, and append nothing later. If something turns up
later, leave the file as it is and open a new item that cites this one and
acts on it.

Delete this comment when you write the section. -->
