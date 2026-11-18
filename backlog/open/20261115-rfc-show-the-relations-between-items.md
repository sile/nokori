# RFC: show the relations between items

## Summary

A command that takes one item and prints the items reachable from it, as
`cargo tree` prints the packages a package depends on: one item per line,
nested under the item that reached it, with the direction of the link that
got there -- which file holds it. The graph is acyclic, since a closed item
may not link an open one, so the walk terminates by itself; an item reached
by more than one path is printed under each, as `cargo tree` does.

## Motivation

The links between items are in the files, and nothing reads them. Reading
them is left to the writer, by opening the directory and grepping for the
slug: a link is a filename with a date in it, and the text that names the
target is a label that the target has to be matched against. What that costs
is paid every time an item is picked up: an item that answers another, or is
answered by one, does not say so on its own, and the item that is not
mentioned by anything is invisible without the search.

What makes the search worth avoiding is the direction. A relation between two
items is one link, written at one end, and which end it is written at is the
writer's choice -- so a reader who finds no link in the file has found
nothing, and the relation may still be there, written from the other side.
The file says what it points at and never what points at it. The half the
file does not hold is the one a reader wants to see: on a settled item, what
came of it, and on an open one, what it is an answer to.

There is a second cost, paid once per writer instead of once per reader: the
run that lists the relations is one the writer has to remind themselves to
make. [rfc-say-when-a-link-is-written](20261115-rfc-say-when-a-link-is-written.md)
puts the moment for it in the flow, and the step is a search the writer does
by hand -- open the directory, grep for the slug, check the date prefix on
what comes back. A tool that creates and moves the files already knows where
they are, and makes the writer search anyway.

What a reader does with the relations once they are found is walk them: this
RFC came from that feedback, was carried out by that plan, which touched
those bugs. The walk is what makes a set of items a history rather than a
pile, and it is the part that cannot be done by opening one file -- the next
hop out of an item is another item's links, and the hop after that is a
third file to open.

## Design

**One item, named on the command line.**

    nokori tree <item>

`<item>` is a path to an item file in `backlog/open/` or `backlog/closed/`,
spelled as `close` takes it. The command prints that item and everything
reachable from it, and exits. There is no argument form that prints the whole
backlog: the walk starts from an item, and a listing of every item is a
different question with a different answer (see Alternatives).

**Each line is an item, nested under the line that reached it.** The order
under a line is the order the links are in, and a link is followed as one
item of the walk -- so the same item can appear deeper and again further
down.

```
rfc-show-the-relations-between-items
- plan-the-reading-commands
  - rfc-say-when-a-link-is-written
- feedback-slugs-are-hard-to-type
- bug-empty-input
  - plan-the-reading-commands
```

The first line is the item named, by its kind and slug -- the text a link to
it carries. Every line below it is an item one hop further, two spaces per
hop.

**The mark before an item says which file holds the link that reached it.**
`-` is a link in the item on the line above: open that file and find it.
`<-` is a link in the item on this line, pointing back at the one above:
open this file and find it. A relation written at both ends is reached twice,
once per link, since two links is what is in the files.

So a line is read as a sentence -- "this item, reached from a link in the one
above" or "reached from a link in itself" -- and the reader knows which file
to edit without opening either.

**The direction is the fact, and the ban is why it matters.** A relation has
no direction of its own -- two items are about each other -- but the link does:
one of the two files holds it and the other does not. A closed item may not
link an open one, so a relation whose only link is in the closed file can
never be written from the open side, while one whose only link is in the open
file has to be written before that file closes. The two are the same edge and
different work, and the mark is the difference.

**There is no cycle to build, and the walk still has to survive one.** The
ban keeps a cycle out of `backlog/open/`: a closed item may not link an open
one, and every cycle that includes an open item would need that edge. Inside
`backlog/closed/` the links are allowed both ways, so a cycle of two closed
items is possible in principle -- neither link can be written after the other
file closes, so the pair would have to be committed in the same run, which is
not something a writer does on purpose.

The walk follows links in both directions, so it would not end if one existed.
What the implementation keeps is the items on the current path: an item that
repeats on the path is a cycle, and the walk stops and says so. That costs a
set the depth of the walk, and it is not the visited set of the alternative
below -- an item reached twice by different paths is still printed twice.

**An item reached by two paths is printed under both**, as `cargo tree`
prints a shared dependency. Deduplicating would say which path was taken
instead, and that is a rule about the graph the files do not have. The shape
of the output is then a tree of the paths, not a tree of the items.

There is no repeat mark and no `--depth`. Both are ways of cutting a tree
that has grown too long to read, and neither is needed by a backlog whose
tree fits on a screen. Three conventions -- the indent, the mark, and the
shape of the names -- is what a reader learns; a fourth is what a real output
would ask for, in an item citing this one.

**An item with no relation is printed alone**, its own name and nothing
under it. That is the answer to the question it was asked.

**The order is the files' own.** The links in an item come in the order they
appear in its file; the items that link *to* it come in the order the files
are read from the directory. Nothing is sorted by name: the order of the
links in the file is the order the writer wrote them in, and that is a small
piece of what the file says.

**Read from the files, nothing cached.** The command reads `backlog/open/`
and `backlog/closed/` on every run, since a link from a closed item is a
relation like any other. It writes nothing. The backlog is a set of text
files committed once per change, and a cache would be a second copy that is
right until somebody edits a file.

**Links that are not items are not relations.** A link to `src/main.rs`, to a
URL, or to a heading is skipped, and so is a link to a file in this
repository that is not an item. The command shows the item graph, and a link
to a source file in a `Summary` is part of the prose, not part of the graph.

The same `links()` reader `close` uses, with the same scope: what counts is a
target that is a path to a file, and an item is one whose name answers to
`YYYYMMDD-<kind>-<slug>.md`.

## Alternatives

**With no argument, print every open item.** The command would be a view of
the whole backlog, as `cargo tree` is a view of the whole dependency graph.
It loses because the output stops being readable as the backlog grows, and
because it answers a question nobody asks: the reader has an item in hand, and
what they want is the items next to it. The whole-backlog view also has to
choose an order that the files do not give it, and rule the output by a
convention a reader has to learn before reading it.

**Print the relation and not the direction.** The other end of a relation is
an item related to this one, and either file may hold the link. It loses
because the direction is what the reader acts on. A relation written only
from the closed side is a relation that can never be written from the open
side once it closes, and a reader who sees a bare name cannot tell whether
the half is there. The marks are the difference between a file to edit and a
file to leave alone.

**One hop, and nothing below it.** The item and its neighbours, each on its
own line, and a second hop only if the reader asks again. It loses because
the reader is walking the graph anyway, and a command that walks one step
leaves the rest to a shell loop -- which is the same hand-reading, with the
order lost. The path is what carries the meaning here: this RFC came from
that feedback, was carried out by that plan, which touched those bugs. A tree
shows the path; one hop shows two names.

**A generated index in a file.** Walk the backlog and write the relations
into a markdown file, regenerated on demand or committed. It loses because
the index is a second copy of what the files already say, and a copy is the
thing that goes out of step: it is correct until somebody edits a link and
runs nothing. The links are in the files, so the reading side is a command
that reads them on the spot, with nothing written down.

**Have `close` print the relations of the item it moves.** The moment the
file stops being edited is the moment its relations are read for the last
time, so `close` could print them along with the move. It loses because it
answers the question only once, at the end, and never for an item that is
still being worked on -- which is when a writer is deciding what to link. It
also makes `close`'s output a report, in a run whose one job is to move a
file and say what stopped it.

## Open questions

Whether the command is named `tree`. Resolved: `tree`. The name is
`cargo tree`'s and so is the shape of the output, and what the command does
is walk one graph -- that it is the item graph and not the build is said by
the argument, which is an item. A second name would have to say what kind of
graph, and nothing shorter than the argument does.

Whether `-` and `<-` are the marks to use. Resolved: `-` and `<-`. Both are
plain ASCII, both sit after the indent without pushing the name right, and
the line is read from its indentation first and its mark second. A word would
compete with the name for the reader's eye, and the output is already a list
of names.

Whether the walk drops into a cycle if one is there, or refuses to print at
all. Resolved: keep a set of the items on the current path only, and refuse
the walk when one repeats. That is not the visited set that deduplicates, and
it turns a hang into a sentence. A cycle in `backlog/closed/` is possible --
two items that cite each other and are committed in the same run pass
`close`'s checks -- and it is not a state a writer creates on purpose, so the
cost of the set is paid only by the run that meets one.

Whether the walk repeats an item reached by two paths, as `cargo tree` does,
or prints it once under the first path and marks the repeat as such
(`*` in `cargo tree`). Resolved: print it under each. The repeat is what says
the item is shared, and the mark is a second rule to read on top of the
indentation and the two marks -- three conventions for an output that is not
yet long enough to need them. If a real backlog prints too long, the mark is
a change to make on its own, citing this one.

Whether the item's own line carries the path as well. Resolved: kind and slug
only. The command was given the path, so a reader who is about to edit a
relation has it in the run that printed the tree; the line says which item,
the way a link's text does, and the other commands name items the same way.

Whether the depth is bounded by a flag (`--depth`, as `cargo tree` has).
Resolved: not yet. The walk is the point, and the length of a real backlog is
the thing that would ask for the cut; adding it before there is one guesses
at a default. When the output is long enough to want it, that is a new item
citing this one.

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
