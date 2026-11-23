# RFC: tree shows one direction

## Summary

`nokori tree` walks one way at a time. By default it follows the links an
item holds, and with `--invert` it follows the links that point at it -- the
same pair of views `cargo tree` offers with `-i`. The mark that says which
file holds a link goes away with the two-directional walk: a line's direction
is what the run was asked for, and the same for every line under it.

## Motivation

Today's output puts a `-` or a `<-` in front of every name, where `-` is a
link in the line above and `<-` is a link in the line itself. The marks are
right, and the output is still hard to read: two lines at the same depth mean
something different, and telling which is which means reading the mark before
reading the name. A reader who is scanning for what an item is about is
reading symbols instead.

Worse, the walk crosses back and forth. An item that cites this one sits
beside an item this one cites, under the same indentation, so the shape says
"these are the same relation" where the files say opposite things. The
relation is one thing and the direction is the other, and the output mixes
them in one line each.

What the reader has in hand decides which half they want. An open item asks
"what does this need?" -- the links it holds, to follow before closing. A
settled one asks "what came of this?" -- the links that point at it, which is
the half no file holds. Today's command prints both and lets the reader
filter by mark, and that filtering is the tool's job.

## Design

**`nokori tree <item>` follows the links the item holds.** Each line is the
target of a link in the line above it, and the walk goes outward until there
is nothing further.

**`nokori tree --invert <item>` follows the links that point at it.** Each
line is an item holding a link to the line above it. The walk goes inward,
and an item that nothing links to is printed alone.

**Both ways are the same walk, run once.** The `--invert` flag chooses the
set of neighbours that gets followed; everything else -- the indentation, the
cycle rule, the order, the repeat of an item reached by two paths -- is
unchanged. `cargo tree -i` is the same shape: a flag changes one step of an
algorithm whose output is otherwise identical.

**The output uses `cargo tree`'s branch drawing.** A line that has siblings
below it and more after it is drawn with `├── `, the last one with `└── `,
and the depth above it is carried by `│   ` where a line below is still
there:

```
20261122-rfc-link-text-keeps-the-date
├── 20261122-rfc-say-when-a-link-is-written
│   └── 20261122-bug-empty-input
└── 20261122-feedback-235217
```

**No mark, and no marker for the root.** The root is the first line and is
not drawn with a branch, as `cargo tree` prints the package it was given.
There is nothing in the output that says "link held here" or "link held
there", because the run said it once.

**The `cargo tree` conventions are the ones to inherit.** A reader who knows
why a repeated package in `cargo tree` is printed under both paths already
knows why an item is: the walk is a tree of paths, not of items. The
indentation and the branch characters are a small thing to learn, and they
are already learned by the reader who has run the other command.

## Alternatives

**Keep both directions and drop the marks.** A relation is a relation, so
print every related item once. It loses because the direction is what the
reader acts on: an item linked from here can be pointed at again, and one
that only links here has to be edited there. Without a mark the output
cannot say which, and without direction it is the same list `grep` gives.

**Two commands rather than one with a flag** -- `tree` and `tree-invert`. It
loses because the two are the same walk over the same files with one step
different, and a second name is a second thing to document and to remember
for a difference a flag says in five characters.

**Keep the current output and add `--invert` alongside.** Then the default
has marks and `--invert` does not, or both do. It loses because the default
is the one most readers meet, and a default that mixes direction is the thing
that makes it hard to read.

## Open questions

Whether `--invert` is the right name. It is `cargo tree`'s, and the operation
is the same one; a name of our own would have to explain what "invert"
already says to a reader of that command.

Whether the cycle still needs to be allowed for. Yes, and the rule does not
change: a cycle among closed items is possible, and the walk keeps the items
on the current path to stop at one. Under `--invert` the cycle is the same
cycle read backwards.

Whether the label keeps the date. That question is
[20261122-rfc-link-text-keeps-the-date](20261122-rfc-link-text-keeps-the-date.md)'s,
and this RFC reads whatever text that one settles on.

## Outcome

