# RFC: a tree line is one item, once

## Summary

`nokori tree` prints an item once per link that reaches it, so a file that
links the same item twice -- once in its body, once in its `## Outcome`, which
is the usual shape -- prints that item on two lines. It also walks an item
again every time a second path reaches it, where `cargo tree` stops a package
it has already printed and says so with `(*)`.

The command also takes one item, where the question most often asked is about
every open one.

This RFC does three things: an item print once per place it is reached, an
item already printed be said with `(*)` and walked no further, and `tree` take
any number of items, with none meaning every item in `backlog/open/`.

## Motivation

```
nokori tree backlog/closed/20261122-rfc-tree-shows-one-direction.md
```

```
20261122-rfc-tree-shows-one-direction
├── 20261122-rfc-link-text-keeps-the-date
└── 20261122-rfc-link-text-keeps-the-date
```

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

### An item already printed is `(*)`, and is not walked again

The walk keeps the items it has printed. A line whose item is one of them is
printed with `(*)` after it, and nothing is printed under it:

```
20261122-rfc-show-the-relations-between-items
├── 20261122-rfc-tree-shows-one-direction
└── 20261122-rfc-link-text-keeps-the-date (*)
```

The mark says the run has shown this item already, and where. It is where
`cargo tree` puts it, for the reason `cargo tree` puts it there: the subtree
is under the first printing, and printing it twice would say the second path
is the one to read.

The root counts as printed, so a tree that comes back to it is `(*)` like any
other. An item reached twice from the same line is still one line, from the
rule above; the mark is for two different lines reaching it.

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
20261123-bug-panic-on-empty-input
...

20261123-rfc-a-tree-line-is-one-item-once
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

## Outcome
