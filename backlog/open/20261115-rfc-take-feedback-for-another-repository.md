# RFC: Take feedback for another repository

## Summary

A command that takes the words of a person who is not in this repository and
writes them down as it is said: `feedback <path>`, with the words on the
command line, in a file, or on standard input. It writes
`backlog/open/YYYYMMDD-feedback-<HHMMSS>.md` with the words copied into the
`## Feedback` section and nothing else. There is no kind to choose, no slug to
invent, and no title to write: the name is the day and the second, because the
person saying it is not writing an item -- they are saying what happened, and
the tool is where the item comes from.

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

    nokori feedback <path> [--message <text> | --file <file>]

`<path>` is the directory to write the item into: `.` for this repository, or
the root of another one, which is the directory that holds its `backlog/`. It
is always given.

The person's words come from one of three places, in the shape `git commit`
has for a message:

- `--message <text>`, `-m <text>`: the words, on the command line;
- `--file <file>`, `-f <file>`: the words, read from a file;
- neither: the words, read from standard input.

`-m` and `-f` together are an error rather than one of the two being taken:
which one was meant is not something the command can tell, and it is a run
that would be written up wrong rather than not at all. So is `-m ""`: an
empty `## Feedback` is an item with nothing in it, and the run that writes one
is the run that should have stopped.

**Why the words are not the second argument.** They were, and it made the
common case the hard one: the words of a person who met a problem are a
paragraph, a list of steps, or an error message, and a shell argument is not
where those live. The words are in a chat window or an editor, and the shell
is between them and the file -- `-m` puts quoting in the way, `-f` puts a
rebuild of what was already written in the way, and standard input puts one
keystroke in the way (`|`, or Ctrl-D). Which of the three fits is the
writer's, and the default is the one that takes any of them.

**Reading from a terminal.** With neither option, standard input is where the
words come from, and the shell will wait for them -- which, at a terminal,
looks like the command hanging. So when standard input is a terminal, the
command writes one line to standard error saying it is waiting and that Ctrl-D
ends the words, then reads until the end. When it is not a terminal, nothing
is written: a pipe is somebody who is already sending, and a line saying so
would be part of the output of whatever is being piped.

This is the first thing `nokori` writes that is not an answer to a command,
and it is written for the one case where the silence would be mistaken for a
hang. It goes to standard error, so that redirecting standard output still
gets only the path of the file written.

**What it writes.** `backlog/open/YYYYMMDD-feedback-<HHMMSS>.md`, from the
`feedback` template, with the words placed under `## Feedback`. The name has no
slug: the item is named by the day and the second it was written.

**The title line goes, and the comment in `## Feedback` with it.** The words
are the whole item, so there is nothing to put above them: a title made out of
the first line is the first line of the section repeated an inch above itself,
sixty characters shorter and missing the rest. The template has a title line
because `create feedback` is run by somebody who knows what the item is about
and can say it in a phrase before writing the section; this run knows only
what the person said.

The `NOKORI_TODO` comment inside `## Feedback` is replaced by the words, and so
is the one in the title line -- by nothing, since the line it sits on goes too.
What is left is an item whose section is written and whose `## Outcome`
comment is still there. Without that the file would be refused by `nokori
check` for a comment the command itself left, and the writer would be told to
write what they had just said.

**The words are copied as they are**, leading and trailing whitespace around
the whole of them aside. A file read with `-f` is the words and nothing more:
no note goes into the item saying which file they came from, since the person
who said them is the author of the item and the file they were kept in is an
accident of how they travelled.

**The words must be something.** The empty text is refused, however it
arrived: an empty `## Feedback` is an item with nothing in it, and it is read
back as a file nobody wrote rather than a report that said nothing. So `-m ""`,
a `-f` naming a file with only whitespace in it, and a run that was given no
standard input and sent none are the same error, worded as the same thing.

**The name is the day and the time.** `YYYYMMDD-feedback-<HHMMSS>.md`, with the
time the run wrote the file. Two items written in the same second are the one
collision there is, and it is refused with the message `create` gives for a
name already taken -- wait a second and run it again.

What the name would have been is a slug, and a slug is not here. The writer of
a `feedback` has just been told something and is writing it down; a name they
chose is a summary they made before reading the words, and the summary is the
reading. A name the command makes out of the text is worse than that: the rule
would cut the words down to whatever it can spell, so a report that opens with
a sentence about the tool gets a name about the tool, and one that opens with
where the person was gets a name about that -- a bias in the name that nobody
chose and that the person who reads the file has to ignore.

The slug belongs to the item the feedback turns into. A name says what a thing
is so that a reader can find it in a list, and the thing worth naming is what
was decided, which is not known until somebody has read the words. That is the
`rfc`, the `bug`, or the `plan` written in answer, and naming those is the work
of whoever writes them.

So a file made this way is named for when it arrived and begins with what was
said. Nothing in it claims to know what the item is about, because the run did
not know -- which is also the honest order of the flow: the feedback is a
message, and a message is read before it is named.

**Where the rest of the item comes from.** Nowhere: `## Feedback` is what the
run has, and every other section is as the template has it. The comment asking
for the `Outcome` is still there and the item is plainly unfinished. `nokori
close` refuses it for the same reason it refuses any other item whose sections
are unwritten, which is what keeps it in `backlog/open/` until somebody has
read it.

**`create feedback` and `feedback` write two shapes of the same kind.**
`create feedback <slug>` writes `YYYYMMDD-feedback-<slug>.md` with a title
line; `feedback` writes `YYYYMMDD-feedback-<HHMMSS>.md` without one. That is
not an inconsistency to iron out: the two are for different writers. `create`
is run by somebody who has decided what the item is and can say it in a phrase;
`feedback` is run by somebody copying down what another person just said, who
has the words and nothing above them.

What the two files have in common is what the tool reads: a name with a date,
a kind, and something after it, and two sections that `close` and `tree` read
the same way. Nothing parses the part after `-<kind>-`, and nothing needs a
title line, so both are items and neither is second class.

Whether `create feedback` should exist at all is a question this RFC leaves
alone. It is the one way to write feedback into this repository, and the words
of somebody outside are the case `feedback` adds. If having two ways turns out
to be the confusion it might be, the answer is to drop the older one -- but
that is a decision to make by using this command, not by guessing at it now.

**The other repository.** `<path>` is resolved the way a directory is, so
`../other` and `/home/x/other` both work, and the item is written under
`<path>/backlog/open/`. Nothing is read from the backlog in this repository,
so the command behaves the same in a repository that has no backlog at all.

The path is the one thing the writer cannot be helped with -- where the other
repository is, only somebody who knows does -- so what is left is that every
way of getting it wrong says which way it was. There are three, and each gets
its own message with its own next step:

- the directory does not exist: the message quotes it as the command read it,
  which is the common mistake of a relative path resolved from another
  directory, and says nothing about `nokori`;
- the directory exists and `<path>/backlog/README.md` does not: the backlog is
  not set up there, and the message says to run `nokori init` in `<path>` --
  `init` writes three files, and when to make them is the other repository's
  decision;
- the directory is there and its backlog is set up, but there is no
  `backlog/open/`: the message names the two and says the layout is not the
  one this version writes, rather than reporting a missing file.

The help says that `<path>` is the directory that holds `backlog/` and not the
`backlog/` itself, which is the near miss the third message is written for.

**What it does not do.** It does not choose a kind, write a `bug`, an `rfc`, or
a `plan`, or edit the words it was given. It does not commit, in either
repository. It does not open the file. It does not ask a question: a prompt is
something somebody has to answer before the words are written down, and the
words are the one thing the run already has.

**Reading the words back.** A file made this way is a `feedback` item like any
other, and what happens to it next is the flow: read it, decide what to do,
write the item that does it, then close the feedback with the decision in
`## Outcome`. `explain flow` gains a paragraph saying that an item for another
repository, or words that are not an item yet, are written with `feedback`,
and that the rest of the flow happens in the repository the item landed in.

**The file is the other repository's from the moment it is written.** The run
that made it took no part in reading it, so it does not close it either: the
`## Outcome` says what was decided about the words, and that is a decision the
repository the item landed in makes. So the writer's part ends with the file
on disk and uncommitted, and the sentence the other repository needs -- that
there is a new item, and who said it -- is the one thing this command cannot
put anywhere. It is said in the same place the words came from.

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

**Make `<path>` optional, defaulting to this repository.** Type the command,
and leave the path out when the item is for here. It loses because it is the
wrong default for the one case the command exists for. A path left out is a
file written in the repository the command ran in, and nothing about the run
says a destination was left out -- the same silence that losing `--into`
costs, arrived at by another route. An item for this repository is what
`create` is for: a kind, a slug, and a section to fill in. So a `feedback` with
no path is either the wrong command or the wrong input, and both are better
stopped than written.

**Make a slug out of the words.** The first words, lowercased with the runs of
other characters cut down to `-`, so the file is named after what it says. It
loses because the rule decides what the item looks like in a list from the
first sentence the person happened to say. A report that opens with a sentence
about the tool gets a name about the tool; one that opens with where the
person was gets a name about that; one that opens with the steps gets a name
about the steps. None of those is wrong, and none was chosen -- the bias comes
from the rule, and the reader has to look past the name to find out what the
item is. A name that says only *when* claims nothing about the content, which
is the claim it can keep.

There is a second reason. A slug made from the words is still a name in the
list, and a good name is a comprehension of what was said -- which is the
reading the tool is trying to postpone, not to fake. The file is read first,
and the item that comes out of the reading is the one worth naming.

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

**An editor, opened when there is nothing on the command line.** The words are
written in the editor `git commit` would open, so they can be as long as they
need and nothing is quoted. It loses because `create` does not open one either:
the tool writes a file with a template in it and leaves the editing to whoever
comes back later, and a run that blocks on an editor is a run that cannot be
put in a shell pipeline or done in the middle of another command. Standard
input takes the same text without taking the terminal.

## Open questions

Whether the file has a title line. The template has one, and the first line of
the words could fill it, so the item would read like the others in a list.
Resolved: no title line. A title made from the first line is the first line of
`## Feedback` repeated just above itself, cut short and without the rest, and
it says nothing the section does not. What the rule would have to decide --
where the first line ends, whether a full stop is part of it, how long it may
be -- is a reading of the words that belongs to whoever reads them, not to the
run that copies them. A reader who wants a name for the item gives it when the
feedback turns into one.

Whether `-f -` reads standard input, as `git commit -F -` does. With no option
at all already meaning standard input, it is a second spelling of the same
thing, and one more branch for no case that is not already covered. Expected
answer: no, `-f` takes a file name.

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
