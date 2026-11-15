# RFC: Create an item for another repository

## Summary

A second command, `create-external` (name provisional), writes an item into
another repository's backlog: `create-external <kind> <slug> --into <dir>`.
Where `create` always writes into the backlog of the repository it runs in,
the new command names the repository to write into and creates nothing here.
The two are separate commands rather than one command with an option, so that
the ordinary case has nothing extra to read and forgetting the option cannot
put an item in the wrong repository.

## Motivation

An item can come from somebody who is not in the repository it belongs to.
`feedback` exists for that case: it keeps the words of the person who met the
problem, and that person often met it in a repository that is not the one they
are working in. So the item is written from elsewhere, and the tool offers no
way to write it.

What happens instead is that `create` writes the item wherever it is run, and
the file is moved by hand afterwards. That works, but nothing in the run says
that the file is in the wrong place until the writer notices it themselves, or
until the commit is made in the wrong repository.

The case is not rare enough to leave to a hand `mv`. The first `feedback` item
written for another repository by hand went into the repository the command
ran in, and the output did not say so: the mismatch was noticed only after
the file had been read back.

## Design

    nokori create-external <kind> <slug> --into <dir>

`<dir>` is the root of the other repository, the directory that holds its
`backlog/`. The item is written to `<dir>/backlog/open/YYYYMMDD-<kind>-<slug>.md`
by the same template `create` would use, and nothing is written under the
repository the command runs in.

The name and the spelling of the option are provisional; both are decided in
the open questions below, and the rest of this design does not depend on
either.

**What it shares with `create`.** The template is the same, chosen by kind. The
date comes from the same place. The slug is validated the same way, and the
refusal for a bad slug reads the same. The output that follows the write is the
same, with one difference: the steps it prints -- fill in the comments, commit,
then close -- name `<dir>` as the repository they are done in, because that is
where the file is and where the commit goes.

**What it does differently.** The destination is named rather than assumed, so
`create-external` never writes into the repository it runs in. It is an error
if `<dir>/backlog/README.md` is missing, and the error says to run `nokori init`
there rather than offering to create the backlog itself: `init` writes three
files and the choice of when to make them is the other repository's.

**Where `close` runs.** In the other repository, unchanged. `create-external`
writes a file that is an ordinary open item from the moment it lands, and
`close` reaches it the same way it reaches any other. Nothing about the check
`close` makes has to change, because the file is in a backlog with a
`closed/` beside it by the time it can be closed.

**What it does not do.** It does not commit, in either repository; no command
does. It does not create the other repository's backlog. It does not check
that `<dir>` is a git repository, which `create` does not check either.

**How it is found.** `create-external` is listed in `nokori --help` beside
`create`, so it is visible without being asked for. `explain flow` gains a
short paragraph after the `create` section saying that an item for another
repository is written with `create-external`, and that the rest of the flow
happens there. The name is what the reader has to know, and a help listing is
enough to carry it.

## Alternatives

**`create --into <dir>`.** One command with an option, which is the obvious
shape for a difference that is only where the file goes. It loses because the
wrong run is silent: without the option the item lands in the repository the
command is in, which is the one place it was not meant to go, and nothing
about the output says a destination was left out. A separate command makes
"into another repository" something the writer chose, so the run without it is
right by construction.

**A prompt, or a check that the destination was meant.** `create` could ask
where to write when it can tell something is off, or refuse when the current
repository is not the one being worked in. It loses because there is no signal
to go on: a `feedback` item for another repository looks exactly like one for
this one from the command's side. A prompt adds a question to every ordinary
run to pay for an uncommon one.

**A `--root` on every command, so the whole flow can be run in another
repository.** `nokori --root ../other create ...` and then `close` with the
same flag. It loses because `close` is not something run from outside: it
moves a file the user is looking at, in the repository that user is in, and an
option that names a repository would make `close` able to move an item in a
repository the user is not in. The asymmetry is the point -- writing an item
from elsewhere is a thing that happens, closing one from elsewhere is not.

**A command per kind, `feedback-create` and so on.** It loses because the
kind is not what makes the item external. A `bug` found in this repository by
somebody trying the program is the same case, and naming the kind in the
command would ask for a command per kind rather than one per destination.

## Open questions

The name. `create-external` says what it is and sits beside `create` in the
help listing, but `external` names the side rather than the destination. The
other shapes weighed were `create-for <dir>`, `import`, and `report`.
`report` collides with what a `bug` already is, and `import` reads as taking an
existing file in. Expect `create-external` unless a name is found that says
where the item goes without a second word.

The argument order. `create-external <kind> <slug> --into <dir>` keeps the
argument order of `create` and appends the destination, at the cost of the
destination reading as an afterthought when it is the whole difference.
`create-external <dir> <kind> <slug>` puts it first, at the cost of two
commands that take their arguments in different orders. Expected answer: keep
the order of `create`, since a reader who knows one command should not have to
remember which of the two reverses.

Whether `--into` takes the repository root or the backlog directory. The root
is what a person has in mind when they say "the other repository", and
`<dir>/backlog/` is what gets written to. Expected answer: the root, so that
no reader has to know the directory layout to name a destination.

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
