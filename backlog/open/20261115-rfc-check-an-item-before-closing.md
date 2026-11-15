# RFC: let `check` say whether an item is written

## Summary

Add `nokori check <file>`, which reads an item in `backlog/open/` and reports
what is not written yet: a `NOKORI_TODO:` comment still in it, and a link that
does not resolve. It says nothing about `## Outcome`, which stays empty until
the close, and nothing about the working tree. `create` ends by pointing at
it, so the first thing a writer is told after the file appears is the command
that says whether they are done.

## Motivation

The moment an item is written is not the moment it closes. A file is created,
the sections are filled in over one sitting or several, and it is committed
somewhere in there. The close comes later, when the change that settles the
item has landed, and by then the file has been rewritten several times.

Nothing in the tool serves that middle stretch. `create` writes the file and
says what to do with it, and the next command that looks at a file at all is
`close`, which asks a different question -- whether the item may be moved --
at the end. In between, the only way to tell whether the sections are written
is to look, and the way to look without reading the file again is to search
it:

    grep -n NOKORI_TODO backlog/open/20261115-*.md

That search is wrong as it stands, since it finds the marker in prose and in
unclosed comments, which is exactly what the close check stopped doing in
`bf408a4`. So the habit either reports a file as unfinished after it is not,
or has to be as careful as `find_marker` to be trusted, and neither is a thing
to ask a writer to keep doing by hand.

What is being checked is also narrower than what `close` checks, and that
matters. A file whose links are still broken, or whose `## Outcome` is still
empty, is in the state it should be in; a hand-rolled `grep` cannot say so,
and `close` refuses those two cases as well. A writer who runs `close` to
find out gets a refusal about something they were not asking.

## Design

    nokori check backlog/open/20261115-rfc-<slug>.md

One file, named the way `close` names it: a path under `backlog/open/`. A file
elsewhere is refused with the same reasoning `close` gives, since there is
nothing to check about an item that has already closed. Checking the whole
directory is not what this command is for -- the writer asks about the file
they have open -- and it would make the command the one that reports on items
nobody is working on.

Two checks run, in this order, and the first failure stops the run as it does
in `close`:

- a `NOKORI_TODO:` left in a comment, found by `find_marker`;
- every link resolves, found by the same `links` and `target_path` that
  `check_links` uses.

The checks `close` makes that are not here are the point of the command:
`## Outcome` is *expected* to be empty, and a file with uncommitted changes is
the normal state for something being written. One link rule does not carry
over either: closed-to-open is about a file that has moved, and a link from an
open item to an open item is allowed.

What is reported is what `close` would say about the same file, minus the
parts that only apply at the close:

    backlog/open/20261115-rfc-x.md still has a NOKORI_TODO: comment (line 12)

    The comment is in `## Design`, so that section is not written yet.

and, when nothing is left:

    backlog/open/20261115-rfc-x.md is written

The message goes to stdout on success and stderr on failure, with exit status
zero and non-zero, so that the command can be used without reading the output.
The wording is a second copy of part of `close`'s message, and that is the
cost of the command: the two are read by the same writer in the same sitting,
and a message that says something different about the same file is worse than
a duplicated one.

`create` gains one line to its closing output, after the one about the
comments:

    Run `nokori check <file>` to see what is left to write.

That line is where the command is found. `nokori --help` lists it, but nobody
reads the help of a tool whose first command already told them what to do
next, and `create` is the command that put the file there.

## Alternatives

**Have `close` report on a file without moving it** (a `--check` flag). It
loses because the two questions are asked at different times and answered
with different verdicts: `close --check` on a file that is still being
written says it is not ready, which is true and not what the writer asked.
The flag would also have to be remembered, and a flag on a command whose name
says "close" is not one a writer reaches for while the item is still open.

**Do nothing, and let the writers keep the `grep`.** It loses on the case the
`grep` cannot get right without being rewritten: the marker in prose and the
marker in a comment are told apart by `find_marker`, and a hand-rolled search
that does not know that either over-reports or has to be maintained by hand.
The tool already holds the rule that tells them apart, and this is the command
that hands it to the writer at the moment they need it.

**Check every open item at once, with no argument.** It loses because it
answers a question nobody asked: a backlog of items in various states will
always report some of them as unwritten, and a writer who has to find their
own file in the output learns to ignore the output. The one file the writer is
holding is the whole subject.

**Print nothing on success, in the manner of `fmt` and `check` in other
tools.** It loses because the success message is the answer the writer came
for: they asked whether the file is written, and a silent exit leaves them to
read the exit status to find out. Nothing is more common than the file being
written, so the line that says so is not noise -- it is the usual result.

**Have `create` write the check into the file** -- a line in the placeholder
text saying to run `nokori check` -- instead of adding a line of output. It
loses because that line is one more thing to delete before the close, and the
comment it sits in is already asking for the work the command reports on.

## Open questions

Whether the success line should name the file or only say that it is written.
Expected answer: name it, so that the line has the same shape as the failure
message and a transcript of several runs reads without guessing.

Whether `check` should also report a `## Outcome` written while the item is
still open. Expected answer: no. An accepted RFC stays open until its change
lands, so an `Outcome` written early is the mistake `nokori explain flow`
already describes, and a command about being written is not the place to
refuse a mistake about closing.

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
