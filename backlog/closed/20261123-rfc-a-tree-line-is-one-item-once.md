# RFC: a tree line is one item, once, and says whether it is open

## Summary

`nokori tree` prints an item once per link that reaches it, so a file that
links the same item twice -- once in its body, once in its `## Outcome`, which
is the usual shape -- prints that item on two lines. It also walks an item
again every time a second path reaches it, where `cargo tree` stops a package
it has already printed and says so with `(*)`.

A line also says only the item's name, so a reader cannot see from the tree
whether what an open item needs is open too, and a tree of closed items looks
exactly like one of open items.

The command also takes one item, where the question most often asked is about
every open one.

This RFC does four things: an item print once per place it is reached, an
item already printed be said with `(*)` and walked no further, every line
carry the item's `open/` or `closed/` directory, and `tree` take any number of
items, with none meaning every item in `backlog/open/`.

## Motivation

```
nokori tree backlog/closed/20261122-rfc-tree-shows-one-direction.md
```

```
20261122-rfc-tree-shows-one-direction
├── 20261122-rfc-link-text-keeps-the-date
└── 20261122-rfc-link-text-keeps-the-date
```

That is today's output, and the rest of this RFC is about what it should be:
the two lines of one item, the name that does not say which directory it is
in, and the walk that had to ask for one item at a time.

Writing the RFC linked the same item twice on purpose -- once where the
proposal reads its name rule from, once in the Outcome as the item it was
settled by -- and the walk reads two links where there is one relation. Naming
the file a third time would print it a third time, and there is no reading of
the output under which the three lines say three things: a line under a name
is that item as its neighbour, and it is the same neighbour.

The cost is not only the duplicate line. The branch drawing has to choose
`├── ` for a line with a sibling below it, so the same item is printed with
`├── ` and again with `└── `, and a reader looking for which item is the last
neighbour of the one above reads a choice the walk did not make.

The other half is an item reached twice through different lines. `cargo tree`
gives it a line with `(*)` and stops: the entry is shown, and what is under it
is not, because it is under the first entry and the reader can find it there.
Today's walk prints the whole subtree again, and on a backlog where a few
items are cited from many places that is most of the output a second time.

The third thing a line does not say is where its item lives. The question
the tree is read for is whether an open item can be started, and an item can
be started when what it links is closed; a tree whose lines are all names is
a walk whose reader has to open each one to find out. On a backlog where the
closed items outnumber the open ones by a year's work, that is every line.

The missing walk is the one used most: what are all the open items, and what
does each need. Today that is a `for` loop over `backlog/open/*.md` calling
`nokori tree`, which is the scanning work the tool already does once per item.

## Design

### One line per item, below a given line

`neighbours` returns the items one hop away, and an item that more than one
link of the line above reaches is returned once. The order is the order the
first such link is in the file, so the line stays where the writer first wrote
it.

What counts as the same item is the path: `20250101-bug-x` and
`../open/20250101-bug-x` are one edge, since that is the resolution the walk
already does for every link.

### A line says the directory the item is in

Every line names its item as `open/<name>` or `closed/<name>`, including the
root:

```
open/20261122-rfc-show-the-relations-between-items
├── closed/20261122-rfc-tree-shows-one-direction
└── closed/20261122-rfc-link-text-keeps-the-date
```

The prefix is the directory, not the whole path: `backlog/` is the same on
every line of every run, and dropping it is what makes the prefix short
enough to read at the start of a line. Two words that a reader can act on --
this one is done, this one is not -- for seven characters.

It also makes an unambiguous line. A tree with deduplication and the mark has
lines that differ only in the item's name; a tree with a prefix has lines
that differ in the name and in what state the name is in, so a reader
scanning for the open items does not have to know which names are open.

### An item already printed is `(*)`, and is not walked again

The walk keeps the items it has printed. A line whose item is one of them is
printed with `(*)` after it, and nothing is printed under it:

```
open/20261122-rfc-show-the-relations-between-items
├── closed/20261122-rfc-tree-shows-one-direction
└── closed/20261122-rfc-link-text-keeps-the-date (*)
```

The mark says the run has shown this item already, and where. It is where
`cargo tree` puts it, for the reason `cargo tree` puts it there: the subtree
is under the first printing, and printing it twice would say the second path
is the one to read.

The root counts as printed, so a tree that comes back to it is `(*)` like any
other. An item reached twice from the same line is still one line, from the
rule above; the mark is for two different lines reaching it.

A line that is both open and printed reads `open/<name> (*)`: the prefix says
where the item is, the mark says where the run has shown it.

### A cycle is still a cycle

An item on the current path is still the case the walk stops at with its
note, and it is reached before the printed set is consulted: a cycle among
closed items is a thing to say, not a repeat of a name.

### `tree` takes any number of items

The argument becomes `[<ITEM>]...`. Given one or more, each is a root, printed
in the order given, one after the other. A blank line separates one root's
tree from the next, since a root is drawn without a branch and a blank line is
what tells where the tree above ended.

Each root begins a walk of its own: the printed set starts empty at every
root, so an item reached under the first root is not `(*)` under the second.
The roots are separate questions, and a mark on a root's own line would be a
fact about another root rather than about it.

Given none, the roots are every item in `backlog/open/`, sorted by path, and
the run says what it is doing before it does it, since it was not asked for
those items:

```
No item was given, so every open item is walked.
open/20261123-bug-panic-on-empty-input
...

open/20261123-rfc-a-tree-line-is-one-item-once
...
```

The line goes to standard output, because it is the run saying what the output
is. `--invert` works the same way, and reads as one question: every open item,
and what is pointed at it.

When nothing is open, the run prints the line and then says so on standard
error, and exits 0: an empty `backlog/open/` is a backlog with nothing open,
not an error.

```
No item was given, so every open item is walked.
No item is open: `backlog/open/` has nothing in it but its README.
```

## Alternatives

**A separate `nokori open` command.** `open` lists the open items and `tree`
walks one; two names for the two questions. It loses because the question is
the same walk over the same graph, the same way `--invert` was folded into
`tree` rather than given a name. The list is `tree` with the smallest possible
argument set, not a command.

**A `--all` flag rather than no argument.** It is more explicit, and it says
what it means. It loses on the shape of the command: `cargo tree` with no
argument walks the package it was run in, `ls` with no argument lists the
directory it was run in, and a command whose argument is optional reads as
"show me the one you would show me". The line that says what was walked is
what keeps the default from being silent.

**Deduplicate the whole tree by cutting, as a shape rule.** Print each item
once in the run, wherever it is reached from, and nothing anywhere else. It
loses for the reason `cargo tree` prints a shared dependency under both
parents with `(*)`: which of several lines reached an item is what the reader
is following, and the second line is where a reader looking at that line goes
on to the first printing.

**Do the deduplication where the links are read.** The edge already drops
links that name nothing, so it could drop the repeats too, before `Item` is
built. It loses because the repeat is the walk's question and not the
reading's: `close` asks which open items name the file it is moving, and that
question has nothing to do with how many lines a tree prints.

**Mark the closed items rather than prefix every line.** `closed/<name>` and
`<name>` for open, or a `[closed]` column at the end. It is quieter, and on a
tree that is mostly closed it is a lot quieter. It loses on the shape a reader
learns: an unmarked line would mean open by the absence of a mark, and the one
thing a reader does with this tree is look for which items are still open. A
rule that says both states out loud is the one a reader can check every line
against.

**Keep the whole `backlog/open/<name>` path.** It is what the argument takes,
so a line could be pasted into `nokori tree` as it is. It loses because the
line is already indented and already has a branch, and `backlog/` is on every
line of every run without ever telling a reader anything: the prefix is read
to learn the state, and the state is one directory deep.

**Walk every item, open and closed, when no item is given.** It is the whole
graph at once, a truer `nokori tree`. It loses because the closed items would
be walked as roots and then again under the open ones, which is the whole
backlog printed on every run; the open ones are the question a run without an
argument is asking.

## Open questions

Whether the same rule should reach `nokori close`'s report of open items that
link the one being closed. It does not repeat today for the same reason it
could: it lists the items themselves, and an item appears once however often
it links the target.

Whether a cycle should also be `(*)`. No: a cycle is about the path and a
`(*)` is about what was printed, and following a cycle would print it forever
where following a printed item would not. Two causes, two lines.

Whether the blank line between roots is enough of a separator. A tree's last
line and the next root are different shapes already -- one is indented, the
other is not -- so the blank line is for the eye rather than the parser, and
one line is cheap.

Whether an item in a directory other than `open/` and `closed/` is possible.
Today the tree reads `backlog/open/` and `backlog/closed/` and nothing else,
so the prefix has two values. If a third directory ever exists the line needs
a third word, and the name rule is the one to check then, not now.

## Outcome

Settled by commit `e2a50b0`, in `src/command_tree.rs`, `src/item.rs`,
`src/lib.rs`, and `src/main.rs`.

All four things are in. `neighbours` returns an item once however many links of
the line above reach it, at the place of the first link. Every line, the root
included, is `open/<name>` or `closed/<name>`. An item the walk has printed is
printed with `(*)` and not walked again, and the set starts empty at every
root. `tree` takes `[<ITEM>]...`; with none, every file in `backlog/open/` is a
root, sorted by path, after the line saying so.

The two things this RFC left open and the code settled:

- A cycle is still a note after the line, not a `(*)`: the path is asked
  before the printed set, so a circle among closed items reads as a circle.
- Inverting the walk with no argument is one question still -- every open item,
  and what points at it -- and the roots reset the printed set between them.

What was not done, and is not part of this RFC: the version stays `0.0.2`.
The output of `tree` changed shape, so a `0.1.0` is a bump worth making on its
own line rather than in the commit that taught the tree a new one.

The example that opened the Motivation now prints one line:

```
closed/20261122-rfc-tree-shows-one-direction
└── closed/20261122-rfc-link-text-keeps-the-date
```
