# RFC: say when a link is written

## Summary

A link is a relation, and a relation is readable from either end: whoever
opens a file can follow the links in it and the files that link to it, so one
link between two items relates them both ways. One direction is enough, and
which one is for the writer to choose. What the rules do not say is *when* a
writer owes one, so the flow should say it -- an item is not finished until
the relation to what it answers is written, at one end or the other.

Mutual links between two open items are the normal state, since both files
are being written and both writers can see the other. What that leaves is the
close: a closed file may not link an open one, so the item closed first has
to lose its half. `close` should say what to do with that half rather than
refuse it -- the target is written as a name instead of a link, so the
closed file still records what it was about without holding a link that the
next rename would break. And when the other item does not link back, the
relation would then be nowhere, so `close` should say to add that link while
the other file is still open.

The rule is not a prohibition. An item with no relation is a normal item: a
`bug` fixed on sight, a piece of work nobody else asked for. It asks the
writer to look, and what to do with what the look finds is the writer's: a
relation that is not there is not one to invent, and a `feedback` item --
written by the person who hit something, at the moment they hit it -- may be
filed as it is, duplicate or not.

## Motivation

`nokori` keeps a record of pieces of work, and the pieces relate to each
other: a feedback item is answered by an RFC, the RFC is carried out by a
plan, the plan touches two bugs, and a bug that was fixed turns out later to
be the cause of another. The record keeps the items; what it does not keep is
the relations between them, and a backlog is then a pile of files a reader
has to search to find out which of them are about what.

The tool already says that a link is how an item refers to another, and
`close` already reads the links in the file it moves. What it does not do is
say when a link is owed. The rules about links are about their shape and
their directions -- which spellings are allowed, which way a closed file may
point -- and a rule about shape is not a rule about writing the link in the
first place: a reader of the rules learns what a link has to look like, and
nothing about when one is owed.

A ban is also the wrong tool for the half of the problem this RFC is about,
because a link is not directional in what it tells a reader. A file names its
target and its target is a file, so the whole backlog contains the reverse
relation of every link in it: whoever has both can follow the relation from
either end. A link written from a plan to the RFC it carries out is a
relation a reader of the RFC can find, by reading the RFC and then finding
the files that point at it.

What that means is that one direction is enough, and which one is a matter of
where the writer is. A link from a later item to an earlier one is written
when the later file is created, and the target is already there; the other
way round can be written as soon as the later item exists, which in the usual
flow is the moment it is created. The rule does not need to say which. It
needs to say that the relation is written somewhere, and at what point in the
work the writing happens -- which is what the flow has left unsaid.

The tool does not need to write either half. Both ends are ordinary edits to
ordinary files -- the later one while it is being written, the earlier one
by whoever is working on it -- and no command has to write them. A tool that
did would be editing a file its run was not about, and the case that would
save is one the writer faces once per new item.

The rule that already exists, and is not changed here, is the ban on
closed-to-open. A closed item is not edited, so a link it holds is permanent,
while the item it names can still be renamed or turned down; a closed file
naming an open one is a reference with a lifetime shorter than the record's.
The writable directions are the other three, and they are enough: a link
written at the later end is one a reader of the earlier file can follow back
from, and a link written at the earlier end is one a reader of the later file
can follow forward. The direction the ban forbids is the only one where the
writing file outlives the target, and it is not needed to hold a relation.

What is left unsaid is when the writing happens. The rules say a link between
two items exists if either end holds it, and nothing more: the flow does not
put the writing anywhere, so an item answering another links it from
`Motivation` or from `Outcome`, or does not, and no step says when.

The file that decides is the earlier one, and it has the shorter window. A
closed file cannot link an open one, so the earlier item's half can only be
written while it is still open -- and it is usually the item being worked on
at the moment the later one is created. The later end has no such deadline:
the item it names already exists, so the link can be written whenever that
file is next touched. Left to itself, that is after the close, when nobody is
touching it any more.

Looking for the item to link is also the point at which a duplicate is
noticed, and that is worth having: `create` writes a file from a slug and
reads nothing, so a slug that names an item already open produces a second
file about one thing. What the writer has to do to satisfy this rule -- think
of the items the new one is about and name the one it answers -- is the
search that would have found the earlier file.

The exemption is not a licence to skip that search. A relation that is hard
to write is one thing, and a relation that is not there is another, and only
the second is exempt.

The other end of the flow is the close, and it is where the rule costs
something. Two open items may link each other, and both are right to: neither
knows which will be closed first. The item closed first then has a link to a
file that is still open, which a closed file may not hold. Until now the only
answer was to refuse the close, which leaves the writer to work out for
themselves what to do with the half -- usually at the end of the work, which
is the worst moment to be handed a puzzle.

What that half is for is worth separating from what it is. The relation does
not need the link: the other item's link holds it from the other side, or,
when there is none, still can be written while that file is open. What the
closed file needs is a note of what it was about -- which is what a reader of
a settled item asks after, having found it by some other route -- and a name
in prose does that without going stale. A name that has since been renamed is
a name a reader can tell is old; a link that has since been renamed is one
that fails to resolve, in a file nobody will edit again.

## Design

**The rule is one-directional, and only the direction is left open.** An item
and the item it relates to are linked in at least one of the three allowed
directions, and which one is the writer's. The usual one is later-to-earlier,
since the writer of the later item is the one creating the file and the
target already exists.

What the change adds is not a new restriction on what an item may be. It is
a step in the flow -- the relation is written before the item is closed, not
at some point nobody names -- and, at the close, a sentence telling the writer
what to do with the half that can no longer be a link. Nothing is reported
for the item that has no relation at all: such an item is rare enough that a
line of output for it costs more than it saves, and the writer who meant to
link something has the step.

**The ban on closed-to-open is narrowed, not lifted.** It still holds for
links: a closed file may not hold a link into `backlog/open/`. What changes is
that a target may also be written as a name, and a name is not a link. The
check keeps looking for `](`, so a name is not something it has to learn to
except -- it is a thing the writer can already put in a file.

**The name form is recommended, not enforced.** The target is written as its
link text alone, in a code span, where the link was. With a link:

    the shape argued in [rfc-how-items-refer-to-each-other]
    (../closed/20261114-rfc-how-items-refer-to-each-other.md)

With a name, and the file it names moved to `backlog/closed/`:

    the shape argued in `rfc-how-items-refer-to-each-other`

Link text is already kind and slug without the date (`explain references`
says so), so the name reads the same as the link did, and a reader who wants
the file can resolve it from the directory the two share. The code span is
what tells a reader it is a name and not a bare word in the prose. None of
this is checked: `close` does not require it, `check` says nothing about it,
and a writer who leaves the sentence as it was -- or writes the name without
the backticks -- can close the item.

What it buys is the one case the ban exists for. A closed file is not edited,
so a link in it is permanent, while the item it names can still be renamed.
A name that has since gone stale is a sentence that reads as old; a link that
has since gone stale is a file nobody will repair. So the form is recommended
for the same reason the ban is there, and it is recommended rather than
required because a rule here would be a rule about prose, which is the one
thing the tool does not own.

**`close` guides in two cases, and writes nothing.** When the file being
moved holds a link that resolves into `backlog/open/`, `close` stops as it
does today, and the message says what to do instead: write the target as a
name where the link is. It does not rewrite the file. The file being moved is
not necessarily committed, and a command that edits what it was asked to move
is a command whose output is a diff nobody read.

**The other item decides the second half of the message.** `close` follows
the one link it found, to the one file it names, and reads it. If that file
holds a link back, the relation survives the name -- the other end holds it,
the other file is still open, and the message stops after the first sentence.
If it does not, the relation would exist nowhere once the link is a name, so
the message says to add the link in the other file while it is still open, and
names the file.

That is the second file `close` reads (the first is the one it moves), and it
is read, never written. One file and not the whole `backlog/open/`: the
question is about these two items, and a link from a third one says nothing
about it. `explain flow` says nothing of this -- what a reader of the flow
needs is the rule, not which file the command opens to apply it.

**`check` is silent about all of this.** A link into `backlog/open/` is
ordinary while the item is open, and mutual links are the normal state, so
reporting one before the close would report the work in progress. What `check`
reports is about the file as it stands, and as it stands the link resolves.

**The step has a moment, and it is the close.** `explain flow` gains the step:
an item that settles another is not finished until the relation is written,
at one end or the other. The later end is written while the new item is
being written; the earlier end is written then too, since the file that would
have to take it may close before the new one does. Both are ordinary edits to
ordinary files, and in the usual case both happen in the same sitting.

What the flow says about the close changes by one clause: the third refusal
is about a link, and a target written as a name is not one. That is the whole
of it -- the flow states the rule and the two places it is applied, and the
file `close` reads to apply it is left out.

**The link is recommended for `feedback`, not expected.** A `feedback` item is
written by the person who hit something, at the moment they hit it, and the
search is a step between them and getting the words down. When it is skipped,
nothing is lost that this tool is responsible for: the item is a report of
what one person ran into, a second report of the same thing is not an error,
and the maintainer who reads it can link it to whatever exists. The same goes
the other way -- a `feedback` item claiming no relation does not make the
earlier complaint about the same thing invisible, and a later item that
answers both cites both.

What that buys is that the backlog is not steered by whatever an outside
writer happened to find. `feedback` arrives from people who have not read the
record, and an item that had to cite an earlier one would be read as an
answer to it; a duplicate carries no such claim. The `Summary` of a `feedback`
template says the link is optional for this kind, so the writer knows the
search is theirs to skip.

**The documentation is reorganized around it.** `explain references` spends
most of its words on the shapes a link may take, and what it does not say is
what a link is for: a relation that is readable from either end, one link
enough to hold it, and the one thing the writer has to get right -- that the
target resolves. It gains a section on the name form: what it looks like, why
a closed file takes one where a link will not do, and that it is a
recommendation and not a rule. The step goes into `explain flow`, with
`feedback` as the one kind where it is a suggestion rather than a step. Each
template's `Outcome` comment keeps the line about how to write a link, and the
`Summary` of each gains a line for the relation to the item this one answers;
the `feedback` one says the line may be left out.

**The close message says what to do, not what is wrong.** Two messages, one
per case, both of them naming the file and the line:

    `backlog/open/a.md` links to an item that is still open:
      line 12: `rfc-b`

    A closed item is not edited, so a link it holds breaks the day its
    target is renamed. Write the target as a name instead, where the link
    is:

      `rfc-b`

and, when the other file does not link back, one more paragraph naming that
file and saying to add the link there while it is still open. The first
sentence is the refusal; the rest is the way out of it.

## Alternatives

**Go all the way and drop the ban on closed-to-open.** A relation is a
relation, and a closed item linking an open one says something true: this is
what came of me, before it is done. It loses because a closed file cannot
follow the item it names. The link is permanent where it is written and the
target is not, so a rename on the other side leaves the record holding a
reference that points at nothing, and the record is not edited to repair it.
The relation is writable the other way round, from the open file, so the ban
costs nothing a reader needs.

**Require the link in both directions.** Every relation written twice, so
that neither file has to be searched for. It loses because a link is read
from the target as well as from the source: the second copy adds nothing a
reader could not get by following the link from the other end, and it doubles
the links that go stale when a file is renamed, and it makes the step twice as
big at the moment the writer is already writing a new file. What the rule
needs is that the relation exists in the files, not that it appears twice.

**Ask every item for a link, `feedback` included.** One rule with no
kind-shaped hole in it. It loses because the writer of a `feedback` item is
the least likely to have read the record and the least likely to find the
item to cite, and what the search finds at that moment is a relation the
outside writer is claiming on their own reading -- a later item answering
both is the cheaper place to make that claim. A duplicate is not damage here:
the kind exists to carry what a person ran into, and two reports of one thing
are two reports.

**Require the relation to be found before the item is written.** Make the
search the first step of `create`, so a duplicate cannot be filed at all. It
loses because the tool cannot tell a relation that is absent from one the
writer could not name, and the kind that this matters most for, `feedback`,
is the one where the search is the least reasonable thing to ask.

**Report every item that closes with no relation at all.** A line in `close`
for the item that has neither a link out nor a link in. It loses because such
an item is rare and the report is paid for by all the others: an item standing
alone is a normal shape -- a `bug` fixed on sight, work nobody asked for -- and
the tool cannot tell it from the relation the writer forgot. A sentence on
every close that nobody acts on is the line a reader learns to skip, which
costs more than the one item it would have caught.

**Have `close` stop on a link into `backlog/open/` and say nothing more.** The
refusal is right -- the closed file would hold a link it cannot be repaired
for -- but the writer is left at the end of the work with a link and no way
out that the tool has named. Either they work out that a name will do, or
they delete the sentence. The message is where the rule is applied, so it is
the place to say what to write instead.

**Have `close` rewrite the link into a name itself.** One command that moves
the file and fixes the one thing that would break it. It loses because the
file being moved is not necessarily committed, and a command that edits what
it was asked to move writes a change nobody reviewed -- the same reason `close`
changes nothing else in the file it moves.

**A command that writes the relation into the older file.** `nokori link
<new> <old>`, or an option on `create` that adds the line to the slot it
cites, so that the writer names the pair once and both halves are written. It
loses because the second file is not the tool's to edit. A file under
`backlog/open/` may be being written by somebody at the moment the relation
is made -- the item this one settles is usually the one still being worked
on -- and two writers on one file is a conflict the writer would have to
resolve, in a run that was about creating a different file. The half belongs
in an ordinary edit to the older item, at the point the writer has both files
in hand, and that is also the commit in which the relation is visible.

**Have `close` add the missing link to the other file.** The writer closes
the item, `close` sees that the other file has no link back, and adds one
while it reads it. It loses because it makes a `close` edit a second file,
and the second file is one somebody else may be working in at that moment.
The name plus a sentence saying where to add the link leaves the edit where
it belongs, with the writer who has both files in hand.

**Nothing in the tool, only in the documentation.** Say in `explain flow`
that the relation may be written in either direction, that one link holds it,
and that the item closed first writes the name instead of the link. It loses
because the writer meets the problem at the close, not while reading the flow,
and a link that no longer resolves is what tells them at that moment -- the
message is the only place they are standing.

**Say it in `explain flow` alone, and have `close` keep refusing.** The flow
is where the rules belong, and a link into `backlog/open/` is already refused
with a message naming the line. It loses because the refusal says what is
wrong and not what to write, and the writer is at the end of their work with
the file in front of them -- the one moment where one more sentence saves the
search through `explain references`.

## Open questions

Whether `close` should follow the link it stops on before writing the message,
or say the same thing whether or not the other file links back. Resolved by
the implementation: it follows it. The message then says whether the half is
already held, which is the difference between a writer who does nothing and a
writer who adds a link that was not needed. One file, read and not written.

Whether the message should name the line in the other file to add the link
at, or only the file. Resolved: the file. Which section of another item
answers this one is the writer's reading of it, and a line number in a file
they are about to open says less than the file name does.

Whether a `feedback` item that does find a related earlier one should cite it,
or leave the citing to the maintainer. Resolved: cite it if it is handy, and
do not go looking; the template says as much and no more, since a rule would
put the search back.

Whether `check` should say anything about a link into `backlog/open/`, since
that is the state the close will stop on. Resolved: no. Mutual links between
open items are the normal state, so `check` would report the work in progress
on nearly every item that has a relation at all, which is the same reason
`check` leaves out the `## Outcome` comment.

Whether the line for the relation belongs in `Summary` or in a section of its
own. Resolved: `Summary`, one line, and only when there is an item to name. A
section of its own would make it structure, and the items that have no
relation would carry an empty section. `plan` is the exception that falls out
of its own shape: it has `Goal` where the other kinds have `Summary`, so the
line goes there and the relation that belongs to one step goes in that step.

## Outcome

Accepted, and implemented in `f61fb9c`. `close` now stops on a link into
`backlog/open/` with a message that says what to write instead -- the target's
name in a code span, where the link is -- and reads the one file the link
names to say whether the relation is held from that end or has to be added
there. `explain references` gained the name as a section of its own,
`explain flow` the step that writes the relation and the moment it is owed,
and the `rfc` and `bug` templates a line in `Summary`; `feedback` says the
search is optional.

Two things came out of writing it that the proposal did not have. A name made
of the file's name loses the date with it, so `20261118-bug-x.md` becomes
`bug-x` and not `-bug-x`; the first version of the message got that wrong.
And the line for the relation goes in `plan`'s `Goal`, since that is where a
plan says what it is for -- the kind has no `Summary`.
