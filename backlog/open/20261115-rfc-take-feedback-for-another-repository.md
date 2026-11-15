# RFC: Take feedback for another repository

## Summary

A command that takes the words of a person who is not in this repository and
writes them down as it is said: `feedback <path> <text>`. It writes
`backlog/open/YYYYMMDD-feedback-<slug>.md` with the words copied into the
`## Feedback` section, the slug made from them, and that is all. There is no
kind to choose and no slug to invent, because the person saying it is not
writing an item -- they are saying what happened, and the tool is where the
item comes from.

## Motivation

A `feedback` item is the words of somebody who met a problem, and that person
is often not in the repository the item belongs to. The tool is built for the
repository it runs in: every command takes a path under `backlog/`, and the
directory is the repository. So the words of a person outside cannot be
written down from where they are, and are carried by hand instead -- copied
into a file made by `create` in the repository they happen to be in, and moved
to the right one afterwards.

What that costs is not only the `mv`. `create` asks for a kind and a slug
before it can write anything, and both are the item's, not the reporter's. The
kind is a judgement about what to do with the words -- an RFC that argues the
shape, a bug that reproduces them, a plan that carries the work -- and the
reporter is in no position to make it, while the writer is not ready to make
it either, since the words are the thing to be read first. The slug is worse:
it is the whole summary of what the person said, decided in seconds, by
someone who has just been told it.

The feedback kind exists to keep a complaint cheap to write down, and these
two costs are at the moment it is written. Every other item is written by the
person whose item it is, who has the summary in hand because they thought of
it. A `feedback` is written for somebody else, and asking for the same two
things asks the writer to have already decided what the item is -- which is
the reading that has not happened yet.

## Design

    nokori feedback <path> <text>

`<path>` is the directory to write the item into: `.` for this repository, or
the root of another one, which is the directory that holds its `backlog/`.
`<text>` is what the person said, in their words, on the command line.

**What it writes.** `backlog/open/YYYYMMDD-feedback-<slug>.md`, from the
`feedback` template, with `<text>` placed under `## Feedback`. The title line
and the slug both come from `<text>`: the summary of an item is what it says,
and the words are the only summary there is.

**The slug is mechanical.** The first words of `<text>`, lowercased, with a
run of characters that are not a letter or a digit cut down to one `-`, and a
bound on how long it may be. It is the text and not a reading of it, so the
same words make the same slug, and nobody chooses anything. A slug that comes
out empty, because `<text>` was punctuation or a script the rule has nothing
left of, takes a fallback and the run says so.

**Where the rest of the item comes from.** Nowhere: the file is written with
its `## Feedback` filled in and every other section as the template has it, so
the comments that ask for the `Outcome` are still there and the item is plainly
unfinished. `nokori close` refuses it for the same reason it refuses any other
item whose sections are unwritten, which is what keeps it in `backlog/open/`
until somebody has read it.

**The other repository.** `<path>` is resolved the way a directory is, so
`../other` and `/home/x/other` both work, and the item is written under
`<path>/backlog/open/`. It is an error if `<path>/backlog/README.md` is
missing, and the error says to run `nokori init` there: `init` writes three
files, and when to make them is the other repository's decision. Nothing is
read from the backlog in this repository, so the command behaves the same in a
repository that has no backlog at all.

**What it does not do.** It does not choose a kind, write a `bug`, an `rfc`, or
a `plan`, or edit the words it was given. It does not commit, in either
repository. It does not open the file. It does not ask a question, so that it
can be run on the command line with the words somebody just said.

**Reading the words back.** A file made this way is a `feedback` item like any
other, and what happens to it next is the flow: read it, decide what to do,
write the item that does it, then close the feedback with the decision in
`## Outcome`. `explain flow` gains a paragraph saying that when what to do is
not yet known, the next item is made with `create` in that repository, or in
this one -- which is what the kind and the slug in `create` are for.

## Alternatives

**`create-external <kind> <slug> --into <dir>`.** The command in the help of
`create`, writing any kind into another repository with the kind and the slug
chosen where the command runs. It loses because it keeps both costs and adds a
third. The kind still has to be chosen before the words are read, the slug
still has to be invented, and the destination now has to be named every time
-- an option that does nothing for this repository and is the whole point for
another one.

**`create --into <dir>`, or a `--root` on every command.** One option naming
the repository, for `create` or for all of them. It loses for the same reason
and one more: an option that is left out is silent, and the item lands in the
repository the command ran in, which is the one place it was not meant to go.
`close` is the worse half of it -- closing an item moves a file the user is
looking at, in the repository that user is in, so an option that names a
repository would let `close` move an item in one the user is not in.

**Have `create feedback` ask for the words when the slug is left out.** Keep
one command, and read the section from the command line when it is given.
It loses because the choice of what to ask for is the command's, and `create`
with a section on the command line is a second way to write a file that every
kind would then be expected to have. The words being the whole item is true of
`feedback` and of nothing else.

**Keep it out of the tool, and let the reader of the words file the item.**
The words are carried by hand today, and a person who receives them can file
the `feedback` with `create` in the repository it belongs to. It loses because
the person who wrote the words is the one who is there when they are said, and
the hand is what is being removed: the words travel through a chat or a mail,
and the summary is made by whoever receives them.

## Open questions

Whether `<text>` is the whole of what goes into `## Feedback`, with no way to
pass a file or a longer text. The case the command is for is a sentence or a
paragraph said by somebody, which fits on a command line; a long report is not
what `feedback` is for, and an item that is mostly a pasted document is one to
write with `create` after reading it.

Whether the title line is the first sentence of `<text>` or the whole of it.
A title is read in a list of items and a section is read in a file, so the two
want different lengths, and cutting the text into a sentence is a rule about
the text that the tool would have to get right.

The order of `<path>` and `<text>`. `<path>` first keeps the text last, where
a shell hands over a quoted string without trouble; `<text>` first reads as
"write this down here". Expected answer: `<path>` first, since the words of a
person may begin with anything.

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

To refer to another item, link it:
[bug-empty-input](../closed/20261114-bug-empty-input.md). Link only an item
that is already closed: never link one that is still open, not even as
background. Run `nokori explain references` for the paths and the rule.

An accepted RFC can still be overturned later. Say so in the later item and
cite this one; do not edit this file afterwards to follow it.

Once this section is written and committed, treat the file as closed: do not
edit any part of it again, and append nothing later. If something turns up
later, leave the file as it is and open a new item that cites this one and
acts on it.

Delete this comment when you write the section. -->
