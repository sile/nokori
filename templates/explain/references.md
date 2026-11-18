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

The first of the three is not only refused: the message says what to write
instead, which is a name (below), and whether the other file links back
already. That file is read for the second half and never written.

A link that is not a path is skipped: one with a scheme, such as `https://`
or `mailto:`, and one that names only an anchor in this file. Everything else
is read as a path, whether or not it points at an item. There is no narrower
scope to remember: a check that covers every path in the file is one a writer
can rely on without knowing where its edge is.

A link is read only where a reader would see one. Text inside an HTML comment
is not rendered, and text inside a code fence or a code span is code rather
than prose, so a `](` in any of those is text about a link and not a link.
That is where the example in a template's comment sits, and it is why a
freshly created item passes `check` without its comments being deleted first.

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
`backlog/open/` with its outcome unwritten; the item that finishes the work
links back at the end.


A name where a link will not do
===============================

A closed file may not hold a link into `backlog/open/`, and one case puts it
there every time: two items that answer each other are written while both are
open, and the one closed first has a link to a file that is still open. What
it writes instead is the target's link text, as a name in a code span:

    the decision was made in `rfc-cache-the-parse`

Link text is the kind and slug, so the name reads the same as the link did.
The code span is what tells a reader it is a name and not a bare word. A
reader who wants the file finds it from the directory the two share.

A name stays readable where a link would not. A closed file is not edited, so
a link in it is permanent while the item it names can still be renamed: a
stale name reads as something written earlier, a stale link points at nothing
in a file nobody will repair.

Nothing is checked about this. A name holds no `](`, which is all `close`
looks for, so a writer who leaves the sentence as it was can close the item
-- and so can one who writes the name without the backticks. It is a form to
reach for when the link cannot stay, not a rule.


Reading the relations back
==========================

    nokori tree backlog/open/<item>.md

Prints the item and the items reachable from it, one per line, nested two
spaces per hop. The mark before a line says which file holds the link that
reached it:

    rfc-show-the-relations-between-items
      - plan-the-reading-commands
        <- rfc-show-the-relations-between-items
      <- feedback-slugs-are-hard-to-type

`-` is a link in the line above, `<-` is a link in the line itself. A
relation written at both ends is reached twice, once per link, since two
links is what is in the files.

An item reached by two paths is printed under each, the way `cargo tree`
prints a shared dependency. There is no repeat mark and no `--depth`: both are
ways of cutting a tree that has grown too long, and the cut is a change to
make when one does. A cycle -- possible between two closed items -- stops the
walk where it meets it and says so.

A link to anything that is not an item is skipped: a path into the source
tree, a URL, a heading. What the command reads is what `close` reads, so the
graph it prints is the one the rules are about.
