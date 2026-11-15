# Bug: `close` reads the marker wherever it appears, not only in a comment

## Summary

`check_filled_in` finds the marker with a plain search over the whole file, so
an item that writes the marker in its own prose reads as unfinished work. Such
an item can never be closed: writing the sections does not help, because the
match is in a section that is already written, and deleting the mention is the
only way out.

The smallest input is an item whose body says what the marker is. The RFC that
changed this very comment was one, and it had to be moved to `backlog/closed/`
by hand.

## Reproduction

A file that has been fully written -- every template comment deleted,
including the one in `## Outcome` -- whose `## Summary` contains this line:

    ...and `close` refuses a file that has any NOKIRO_TODO left.

written with the marker's real spelling, colon and all, inside backticks.
Running `nokori close` on that file refuses it and reports the line in
`## Summary` as the comment it stopped on.

## Observed behavior

`close` refuses the file, and the message names the section that is already
written:

    The comment is in `## Summary`, so that section is not written yet.

That sentence is false about the file. The section is written; the marker in
it is prose, not a comment.

The invariant it breaks is the one written above the marker's constant in
`src/main.rs`, which says why the marker is spelled out rather than being a
bare `TODO`:

    The marker is a whole word of its own rather than a `TODO` a comment
    happens to start with, so that an item may write about a `TODO` in the
    code it reports on without `close` reading that as unfinished work.

The intent is that an item may write *about* the marker. The check reads any
occurrence, so the intent holds for `TODO:` and fails for the marker itself.

## Expected behavior

`close` stops on a marker the author left behind as a comment, and not on one
written inside a sentence. The doc comment on the marker's constant is what
promises the second half, and the templates are what say where a real marker
sits: each one is written as an HTML comment and nothing else.

The section `close` names in its message is also expected to be true, since it
is there to send the writer to the part of the file that is unfinished.

## Notes

The fix belongs in `check_filled_in`, and it is to search inside HTML comments
rather than over the whole file. A marker is a comment in every template, so
searching the comments finds every real one and skips the prose.

`links()` in the same file sets the precedent for how much to care: it reads
every line, including lines in a code fence, and the doc comment there accepts
that as the cheaper side of the trade. The same reasoning applies here in the
other direction -- scoping the search to comments is a few lines, and a
comment is the only place the templates put a marker.

A file that writes the marker inside its own comment -- an item explaining the
syntax, say -- is still refused. That case has no fix short of a marker the
writer has to escape, and it is rare enough that handling it by hand costs
less than the escape would. The case this report is about is the other one:
prose in a section that is already written.

Related case with the same cause: the error says which section the marker is
in by walking back to the last `## ` heading above the match. With the search
scoped to comments that stays right, since a marker is always inside a
section.

## Outcome

Fixed in `bf408a4` ("Search the comments for a marker, not the whole file").
`find_marker` walks the HTML comments and searches only their contents, and
`check_filled_in` uses it in place of the search over the whole file.

The fix went where the notes said it belonged. A comment that is never closed
is searched to the end of the file, which the notes did not cover: a file whose
comment is left open is commented out from there on as far as a reader is
concerned, so a marker in it is as unfinished as one in a closed comment.

Confirmed against the case that prompted the report: the RFC that had to be
moved to `backlog/closed/` by hand closes on its own once its file is put back
in `backlog/open/`, and a freshly created item is still refused on the marker
its template left in the title line.
