# Bug: `check` refuses a link inside an HTML comment

## Summary

`check` reads a link inside an HTML comment as a link that has to resolve,
and refuses the item when it does not. An HTML comment is not rendered, so a
link in one is not a link; where `close` already looks only inside comments
for its marker, `check` looks at text that a comment covers the same way it
looks at prose.

The item is reported as unfinished for a link the writer cannot act on, and
the refusal names a line the writer can only fix by deleting a comment that
is not meant to be deleted until the section is written. The templates put
example links in exactly those comments, so a freshly created item is refused
for a link that was written by `create`.

## Reproduction

Create an item and close nothing, then make every comment go away but one:
the `## Outcome` comment, which stays until the close by design. Delete the
other comments as written, leaving that one as `create` wrote it.

    $ nokori create bug example
    Created backlog/open/20261118-bug-example.md
    ...
    $ nokori check backlog/open/20261118-bug-example.md

`create` writes no link into `## Outcome`; the templates do. So the case needs
no editing at all beyond deleting the other comments: the example link is there
in the file `create` just wrote, and it names
`../closed/20261114-bug-empty-input.md`, which is not a file.

## Observed behavior

Once the other comments are deleted, `check` fails -- and the failure is about
the link, not about anything left to write:

    $ nokori check backlog/open/20261118-bug-example.md
    Error: `backlog/open/20261118-bug-example.md` links
    `../closed/20261114-bug-empty-input.md` (line 91), which is not a file.

The line it names is inside the `<!-- ... -->` that opens `## Outcome`. The
marker in that same comment is skipped -- `check_markers_outside_outcome`
leaves `## Outcome` out -- so the comment is known to `check` and skipped for
one rule while being read for another.

The invariant it breaks is the one `links` states: a link is a reference
between items, written in the text of an item. A comment is not the text.
`check_links` and `check_links_now` both read every link `links` returns, and
`links` walks the lines of the file with no notion of a comment, so a link
that a renderer never shows is checked as though a reader would follow it.

## Expected behavior

`check` should report what is not written in the item, and a link inside a
comment is not written in the item. The comment says as much itself: it is
there to be deleted, and everything in it goes with it when the section is
written.

The same reasoning already governs the marker. `find_marker` reads only the
inside of comments, and `check_markers_outside_outcome` skips the one comment
that is expected to stay, so a comment's contents count for the section they
sit in and not as text. A link is the other half of the same distinction, and
should be read from the text outside comments only.

## Notes

The fix belongs in `links`, which both commands read through, so that `check`
and `close` agree about which links exist. A fix in `check_links_now` alone
would leave `close` refusing a link that `check` passed, which is the worse
of the two disagreements: the writer would be told the item is written and
then refused for a link inside a comment.

`links` currently says it reads a link inside a code fence too, and accepts
that on the grounds that a fence showing a link is rare. The two are one case
and not two: neither a comment nor a code span is rendered as a link, and a
fence in an item is not rare at all -- `explain` topics and the templates
both show commands in one. Reading text that is not prose is the cause here,
and a fix that covers comments alone leaves it in place for fences.

What a reader can act on is a link in prose. A link in prose is the only one
whose target the item depends on, and it is the only one `close` can tell is
wrong once the file has moved.

## Outcome

Fixed in `eefc0ba` by the fix the notes asked for, in `links` rather than in
`check_links_now`: `check` and `close` read links through `links`, so putting
it there fixed both at once and neither can now refuse a file the other passes.

The file is scanned line by line, and a link is read only from text a reader
would see. An open comment is carried across lines, so a comment that spans
half the file hides all of it. A fence is entered on a run of three or more
backticks or tildes and left on a run of the same character at least as long.
A code span is removed before the search, by matching backtick runs of equal
length, so `` `x` `` and `` ``x`` `` are both code.

Three tests hold it: that a link in a comment is not read, that a link in a
fence and one in a span are not read, and that `close` moves a file that has
all three. `explain references` now says where a link is read from, since the
answer is no longer "anywhere in the file".
