An item refers to another item with a markdown link: the text is the other
item's kind and slug, and the target is the path to its file from the
file holding the link.

```markdown
[rfc-cache-the-parse](20261114-rfc-cache-the-parse.md)
```


Where the path points
=====================

The target depends on where the two items are, so it changes when an item
moves. This table is the whole rule:

| referring item | target item | target |
| -------------- | ----------- | ------ |
| `backlog/` | `backlog/` | `20261114-plan-split-the-lexer.md` |
| `backlog/` | `closed/` | `closed/20261114-plan-split-the-lexer.md` |
| `closed/` | `closed/` | `20261114-plan-split-the-lexer.md` |
| `closed/` | `backlog/` | not allowed; see below |

Write the path that is correct now. An item may cite an item that is still
open, and when that one closes the link breaks; it is repaired in the open
item that holds it, because `close` refuses a link that does not resolve.
A broken link therefore never reaches `closed/`.


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

`close` looks at one thing: a link in the file that points at an item,
meaning a `.md` file in `backlog/` or `closed/`. It refuses the move when
that link points into `backlog/` (see below) or at a file that does not
exist.

Nothing else is checked. A link to a URL, to a file in this repository that
is not an item, or in some other notation altogether is left alone, broken
or not: `close` does not take a position on it, and keeping such a link
correct is the same as keeping any other text in the file correct, which is
the writer's business.

A link to an item that does not exist yet is refused all the same, since the
target is missing. When the item that would settle this one is not written
yet, say what would settle it, and let that item link back from the day it
is created.


Closed never cites open
=======================

An item in `closed/` may link another item in `closed/`, and an item in
`backlog/` may link either, but a closed item never links an open one.
`close` refuses a file that points into `backlog/`, because the file is
about to be in `closed/` and the link would then be the forbidden direction.

What makes the rule worth having is that a closed item is not edited again.
A link it holds is permanent, so it must point at something that is also
permanent: an open item can still be renamed, and its outcome is not known
yet. The same holds for every section of a closed file, not only `Outcome`,
which is why `close` checks the whole file.

The case that looks like it needs an exception does not. When an item answers
an open proposal, the answer is not settled yet either, so it stays in
`backlog/` with its outcome unwritten; the item that finishes the work links
back at the end.
