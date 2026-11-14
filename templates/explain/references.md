An item refers to another item with a markdown link: the text is the other
item's kind and slug, and the target is the path to its file from the
file holding the link.

```markdown
[rfc-cache-the-parse](20261114-rfc-cache-the-parse.md)
```


Where the path points
=====================

The target depends on where the two items are. An open item is in
`backlog/open/` and a closed one is in `backlog/closed/`, and the two
directories are at the same depth, so a target that reaches out of its own
directory keeps its meaning when an item moves from one to the other. This
table is the whole rule:

| referring item | target item | target |
| -------------- | ----------- | ------ |
| `backlog/open/` | `backlog/open/` | `20261114-plan-split-the-lexer.md` |
| `backlog/open/` | `backlog/closed/` | `../closed/20261114-plan-split-the-lexer.md` |
| `backlog/closed/` | `backlog/closed/` | `20261114-plan-split-the-lexer.md` |
| `backlog/closed/` | `backlog/open/` | not allowed; see below |

A target is always a path from the directory of the item that holds the
link, never from the root of the repository. That is why the first and third
rows are a bare filename -- those items are neighbours -- and why the second
row, which reaches out of its own directory, starts with `../`.

Write the path that is correct now. An item may cite an item that is still
open, and when that one closes the link breaks; that is why `close` refuses a
link that does not resolve, so the repair happens in the open item that holds
it before the item closes.

The writer never has to spell a target for a place the file is not in yet: a
file in `backlog/open/` that links `../closed/x.md` is right where it stands,
and the same text is right once it is in `backlog/closed/`. `close` does not
rewrite targets, so the table above is what a link reads in the file, before
the move as well as after.

A broken link therefore does not reach `backlog/closed/` by a move. What is
already there stays: a closed item is not edited, so a link that was broken
when it landed is read as the record of what that `close` produced.


What the text says
==================

The text is the kind and the slug, without the date: `rfc-cache-the-parse`,
not `20261114-rfc-cache-the-parse.md` and not `cache-the-parse`.

The kind is kept because it is most of what the sentence around the link is
saying -- whether the thing cited is a proposal, a report, or a piece of
work. The date is left out because it carries nothing a reader needs to know
why the item is being cited, and both `ls` and the target give it.

The text is only a label: the target is the reference. Two items may share a
kind and slug on different days, and nothing is lost, because the link still
points at one file.


What close checks
=================

`close` reads every link in the file it is about to move, and refuses the
move in three cases:

- the link names an item in `backlog/open/`, which is the forbidden
direction (see below);
- the link does not resolve once the file is in `backlog/closed/`;
- the link names one file from `backlog/open/` and another from
`backlog/closed/`, which the move would hide.

It changes nothing in the file. What it checks is what the file will read as
once it is closed, since that is the only state the file is ever read in
afterwards, and a link that is wrong then could not be repaired.

A link that is not a path is skipped: one with a scheme, such as `https://`
or `mailto:`, and one that names only an anchor in this file. Everything else
is read as a path, whether or not it points at an item. There is no narrower
scope to remember: a check that covers every path in the file is one a writer
can rely on without knowing where its edge is.

A link to a file in this repository that does not exist is refused all the
same, since the target is missing. When the item that would settle this one is
not written yet, say what would settle it, and let that item link back from
the day it is created.


Closed never cites open
=======================

An item in `backlog/closed/` may link another item in `backlog/closed/`, and
an item in `backlog/open/` may link either, but a closed item never links an
open one. `close` refuses a file that points into `backlog/open/`, because the
file is about to be in `backlog/closed/` and the link would then be the
forbidden direction.

What makes the rule worth having is that a closed item is not edited again.
A link it holds is permanent, so it must point at something that is also
permanent: an open item can still be renamed, and its outcome is not known
yet. The same holds for every section of a closed file, not only `Outcome`,
which is why `close` checks the whole file.

The case that looks like it needs an exception does not. When an item answers
an open proposal, the answer is not settled yet either, so it stays in
`backlog/` with its outcome unwritten; the item that finishes the work links
back at the end.
