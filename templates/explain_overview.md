`nokori` is a way to keep a backlog in markdown files. This topic says what
that is and what it does not do, so that the rules in `kinds`, `flow`, and
`references` arrive as consequences rather than as a list to memorize.


What the backlog is
===================

The backlog is ordinary directories and ordinary markdown files: `backlog/`,
`backlog/open/`, `backlog/closed/`, and a file per item. There is no index, no
hidden file, and nothing that can only be read through this tool. Anything in
`backlog/` can be edited, moved, or deleted by hand, and doing so works around
nothing -- those are the same files this tool reads and writes.

`nokori init` writes the three READMEs and nothing else. `create` writes a
template, `check` reads one back and says what is still unwritten outside the
section the close writes, and `close` moves a file. Every other way of using a backlog -- a manager's list, a review
queue, a filter over the open items -- is reading the files yourself, which is
what the tool is doing too.


What the tool is for
====================

It is not there to enforce the flow, and it cannot: there is hardly any check
that could fail. What it offers is one way to keep a backlog, written down as
a flow of commands, and the rules are recommendations that the user follows on
their own responsibility -- the same deal as git-flow, where the flow is a
convention and the tool only makes it convenient.

It exists as a program rather than as a page of prose so that the flow can be
walked without knowing it in advance. Every command says what to do next, so
`nokori --help`, then `nokori init`, then `nokori create` each lead on to the
following step, and the shape is learned by running them in order.


What an item is
===============

An item is a record of work or a proposal, not a specification. The
authoritative specification is the code and `README.md` of the crate; those
must make sense without the backlog. The rule is one-way: an item may cite
`src/` and `README.md`, but neither of them may point back at an item.

Putting it there would tie the code to a file that is closed and never edited
again, and it would make the code unreadable to somebody who does not have the
backlog. So a `TODO` in the code that points at an item belongs in the code as
a plain sentence, and the item is where the reasoning lives. The same holds
for a commit message and a pull request.


What that asks of the reader
============================

Because nothing is enforced, a rule here is worth following only if its reason
holds for this backlog. The other topics give those reasons, and each is about
a decision that has a cost as well as a benefit:

- `kinds` -- two states rather than four, so that the state is the directory
  and never a field that can disagree with it;
- `flow` -- a closed item rather than an edited one, so that a record means
  what it said when it was settled;
- `references` -- a link rather than a name, so that a citation can be
  followed and breaks visibly when it is wrong.

A rule that is followed without its reason is the one that gets broken the
first time it looks inconvenient, so a reader who finds one of them arbitrary
should read the topic that argues it before deciding to do something else.
