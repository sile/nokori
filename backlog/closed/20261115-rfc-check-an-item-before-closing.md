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

None. The success line names the file, so that the line has the same shape as
a refusal and a transcript of several runs reads without guessing. `check` does
not report a `## Outcome` written while the item is still open: an accepted RFC
stays open until its change lands, and a command about being written is not the
place to refuse a mistake about closing.

## Outcome

Accepted and implemented. `nokori check <file>` reads one item in
`backlog/open/` and refuses it for a `NOKORI_TODO:` comment left in a comment,
found by the same `find_marker` that `close` uses, and for a link that does not
resolve. It says nothing about `## Outcome`, about the working tree, or about a
link into `backlog/open/`, which are the three checks `close` makes that are
not open questions while the item is being written.

The marker check is now `check_markers` and the `## Outcome` check is
`check_outcome`, both called by `close` through `check_filled_in` and the first
of them by `check` as well, so the message a writer sees is the same sentence
whichever command they ran. The link check is not shared, and the reason is in
the design: `close` reads the file from where it is going and `check` from
where it is.

`create` now ends by naming the command, and `explain flow` has a section for
it.
