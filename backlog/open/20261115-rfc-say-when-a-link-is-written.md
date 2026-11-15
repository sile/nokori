# RFC: say when a link is written

## Summary

A link is a relation, and a relation is readable from either end: whoever
opens a file can follow the links in it and the files that link to it, so one
link between two items relates them both ways. One direction is enough, and
which one is for the writer to choose. What the rules do not say is *when* a
writer owes one, so the flow should say it -- an item is not finished until
the relation to what it answers is written, at one end or the other -- and
`close` should say so when the item it is about to move has no relation at
all, from either side.

The rule is not a prohibition. An item with no link is a normal item: a `bug`
fixed on sight, a piece of work nobody else asked for. It asks the writer to
look, and what to do with what the look finds is the writer's: a relation that
is not there is not one to invent, and a `feedback` item -- written by the
person who hit something, at the moment they hit it -- may be filed as it is,
duplicate or not.

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

The item that comes out with no link at all is the one nobody looks at again:
nothing points at it, so it is found only by opening the directory. A step in
the flow is what prevents it, and a report at the close is what catches the
one the step missed -- one line, where the file is still open and its writer
is standing.

## Design

**The rule is one-directional, and only the direction is left open.** An item
and the item it relates to are linked in at least one of the three allowed
directions, and which one is the writer's. The usual one is later-to-earlier,
since the writer of the later item is the one creating the file and the
target already exists.

What the change adds is not a new restriction on what an item may be. It is
a step in the flow -- the relation is written before the item is closed, not
at some point nobody names -- and a report in `close` for the item that comes
out with neither end written. An item with no relation is a normal item and
closes like any other; the report is a line, not a refusal, and writes
nothing.

**The ban on closed-to-open is as it is.** It is not part of this change and
the check stays where it is. It does mean that the earlier file can only get
its link while it is open, since a closed file cannot link the item that
settles it -- so the relation is written from the open item, which is also
where the writer is while the answer is being written. Writing it is ordinary
work on an open item, and the alternatives argue about whether `create`
should do it in the same run as the new file.

**The step has a moment, and it is the close.** `explain flow` gains the step:
an item that settles another is not finished until the relation is written,
at one end or the other. The later end is written while the new item is
being written; the earlier end is written then too, since the file that would
have to take it may close before the new one does. Both are ordinary edits to
ordinary files, and in the usual case both happen in the same sitting.

**An item with no link at all is reported.** `close` gains a check that says
so when the file it is about to move holds no link to another item and no
other item holds a link to it. It is a report and not a refusal: an item may
legitimately stand alone -- a `bug` found and fixed without any item being
involved -- and the tool cannot tell that from an item whose relation was
forgotten. What the report does is make the forgotten one visible at the
moment the file stops being edited.

Nothing is filled in to make the report go away, and no relation is to be
invented for it. The reason an item has no link is that there is no item to
link -- if there were, the rule would have had the writer link it -- so the
report is answered by saying so in the `Outcome`, or by leaving it: a report
is not a refusal, and the move goes ahead either way.

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

The relation is read from both sides, so the check reads the whole open
directory for links that point at the closing file as well as the file
itself. That is the one place where a `close` has to look at files other than
the one it moves; it reads them and writes nothing, and the open directory is
small enough that reading it is what a writer would do by hand to answer the
same question.

A link from a file in `backlog/closed/` is read too, since a relation written
at the earlier end is written while that item is open and the file may have
closed since. What the check asks is whether the relation exists in either
direction, not whether the file it is about holds the link itself.

**What a link is, for this check.** Only a link to an item in this backlog
counts: a target that resolves to a markdown file under `backlog/open/` or
`backlog/closed/`. A link to `src/main.rs`, to a URL, or to a heading counts
for nothing, and an item whose only links are of that kind stands alone. That
is narrower than the scope `close` already checks for resolution, and it is
narrow on purpose: the two questions are different -- "does this target
exist" is about the shape of the file, "is this item related to anything" is
about the graph.

The report says so and the move goes ahead. There is no flag to silence it,
since it is one line on the one item out of many that has no relation, and a
flag would be a thing to remember for an output nobody minds reading.

**The documentation is reorganized around it.** `explain references` spends
most of its words on the shapes a link may take, and what it does not say is
what a link is for: a relation that is readable from either end, one link
enough to hold it, and the one thing the writer has to get right -- that the
target resolves. `explain flow` gains the step: an item that settles another
is not finished until the relation between the two is written, in one
direction at least, and the earlier end has to be written while it is still
open -- with `feedback` as the one kind where the step is a suggestion rather
than a step. Each template's `Outcome` comment keeps the line about how to
write a link, and the `Summary` of each gains a line for the relation to the
item this one answers; the `feedback` one says the line may be left out.

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
loses for the reason the report is not a refusal: the tool cannot tell a
relation that is absent from one the writer could not name, and the kind that
this matters most for, `feedback`, is the one where the search is the least
reasonable thing to ask.

**Report the orphan from a command of its own, instead of `close`.** A
`nokori check` that reads the open directory and lists the items with no
relation, over the whole backlog at once. It loses because the moment the
orphan is fixable is the close: after the file is in `backlog/closed/` it is
not edited, and a report on a backlog of many items is read once and acted on
later. The report belongs where the file can still be changed, which is one
line in `close`.

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

**Have `close` add the forward link to the older file.** The writer closes
the newer item, and `close` adds the link to the item it settles. It loses because it makes a
`close` edit a second file, which is the kind of change this tool has already
decided against -- a move is visible in the diff, and a rewrite hidden in the
same commit is not. It also cannot know which older item to link, since a
`close` sees one file.

**Nothing in the tool, only in the documentation.** Say in `explain flow`
that the relation may be written in either direction, that one link holds it,
and at what point in the work it is written. It loses because the rule is then
a thing a writer has to remember at the moment they stop rereading the file,
and the tool's own reason for existing is that every command reports what to
do next. A report does not make the rule, but it is the sentence in the place
the writer is standing.

**Say it in `explain flow` alone, and have `close` report nothing.** The flow
is where the step belongs, and an item that stands alone is not an error. It
loses because the flow is read when the flow is learned, not at the moment a
file is closed, and the writer who forgot the link is the one who does not
know to look.

## Open questions

Whether the report is worth a line of output at all, given that an item
without a relation is normal. Expected answer: yes, as one line and not a
refusal -- the case it is for is the relation the writer meant to write and
did not, and a writer who did not mean one reads the line and moves on.

Whether `feedback` is exempt from the report as well as from the rule, so that
an orphan `feedback` closes with nothing said. Expected answer: no, report it
like any other -- the writer of the close is the maintainer, who has read the
record, and a silent `feedback` would hide the one item whose relation the
tool could have helped add.

Whether a `feedback` item that does find a related earlier one should cite it,
or leave the citing to the maintainer. Expected answer: cite it if it is
handy, and do not go looking; the template says as much and no more, since a
rule would put the search back.

Whether `close` reporting an orphan is a check or the beginning of the `check`
command proposed in
[rfc-check-an-item-before-closing](20261115-rfc-check-an-item-before-closing.md).
That item reads one file before the close; an orphan is read from two sides,
and the second side is the directory. Expected answer: the report is in
`close`, since it is about the state a close produces, and `check` is about a
file still being written.

Whether the line for the relation belongs in `Summary` or in a section of its
own. Under `Summary` it is read before anything else and is short; a section
of its own would make it structure, and the items that have no relation would
carry an empty section. Expected answer: `Summary`, one line, and only when
there is an item to name.

## Outcome

<!-- NOKORI_TODO: the one section written later, when `nokori close` moves
this file. It stays empty while the RFC is open, this comment and all, and
the comment is deleted in the same edit that writes the section -- which is
the close, since `close` refuses a file whose outcome is still empty.

The sections above are written before the item moves: by the time `nokori
close` moves this file, they are written, their comments are gone (this one
with them), and the change is committed.

Those sections may be rewritten as long as the item is open, as often as the
understanding changes. What they may not become is a description of work
already done. `nokori close` needs the change committed, so whatever they say
at that point is what the moving commit records.

Say whether the RFC was accepted or rejected, and why. When it is accepted,
name the change that implements it, or the item that does, if the work happens
somewhere else.

To refer to another item, link it:
[bug-empty-input](../closed/20261114-bug-empty-input.md). Link only an item
that is already closed: never link one that is still open, not even as
background. The rule is about this section and not the file: a closed item
may not cite an open one, while an open item may cite either, so a section
above this one may link an open item. Run `nokori explain references` for the
paths and the rule.

An accepted RFC can still be overturned later. Say so in the later item and
cite this one; do not edit this file afterwards to follow it.

Once this section is written and committed, treat the file as closed: do not
edit any part of it again, and append nothing later. If something turns up
later, leave the file as it is and open a new item that cites this one and
acts on it.

Delete this comment in the same edit that writes the section, and commit both
together before the move. -->
