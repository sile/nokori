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

<!-- NOKORI_TODO: the one section written later, when `nokori close` moves
this file. It stays empty while the bug is open, this comment and all, and
the comment is deleted in the same edit that writes the section -- which is
the close, since `close` refuses a file whose outcome is still empty.

The sections above are written before the item moves: by the time `nokori
close` moves this file, they are written, their comments are gone (this one
with them), and the change is committed.

Those sections may be rewritten as long as the item is open, as often as the
understanding changes. What they may not become is a description of work
already done. `nokori close` needs the change committed, so whatever they say
at that point is what the moving commit records.

Say what actually happened, and name the change that fixed it; link the
commit or pull request when there is one.

To refer to another item, link it:
[bug-empty-input](../closed/20261114-bug-empty-input.md). Link only an item
that is already closed: never link one that is still open, not even as
background. The rule is about this section and not the file: a closed item
may not cite an open one, while an open item may cite either, so a section
above this one may link an open item. Run `nokori explain references` for the
paths and the rule.

When the fix differs from what the sections above expected, say so here rather
than editing them: those sections record what was thought while the bug was
open, and rewriting them erases the very difference that is worth knowing.

Once this section is written and committed, treat the file as closed: do not
edit any part of it again, and append nothing later. If something turns up
later -- most of all, if this turns out not to have been a bug -- leave the
file as it is and open a new item (an RFC or another bug) that cites this one
and acts on it.

Delete this comment in the same edit that writes the section, and commit both
together before the move. -->
