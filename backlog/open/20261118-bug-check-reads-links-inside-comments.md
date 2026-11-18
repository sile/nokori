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

<!-- NOKORI_TODO: the one section written later, when `nokori close` moves
this file. It stays empty while the bug is open, this comment and all, and
the comment is deleted in the same edit that writes the section -- which is
the close, since `close` refuses a file whose outcome is still empty.

The sections above are written before the item moves: by the time `nokori
close` moves this file, they are written and their comments are gone.

Those sections may be rewritten as long as the item is open, as often as the
understanding changes. What they may not become is a description of work
already done.

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

Once this section is written, treat the file as closed: do not
edit any part of it again, and append nothing later. If something turns up
later -- most of all, if this turns out not to have been a bug -- leave the
file as it is and open a new item (an RFC or another bug) that cites this one
and acts on it.

Delete this comment in the same edit that writes the section, and commit both
together with the move. -->
