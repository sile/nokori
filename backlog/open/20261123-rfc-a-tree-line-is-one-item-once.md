# RFC: a tree line is one item, once

## Summary

`nokori tree` prints an item once per link that reaches it, so a file that
links the same item twice -- once in its body, once in its `## Outcome`, which
is the usual shape -- prints that item on two lines. The walk is a walk of the
item graph, and the second line is the same node arrived at by the same edge.

This RFC makes an item print once per place it is reached: the links of a
file are read as a list, the duplicates among them are one edge when they name
the same item, and no line below a name repeats it.

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
neighbour of the one above reads a choice the walk did not make. `--invert`
has the same line twice from the other end, one per link that points at it.

The mark that used to say which half of a relation a line was at is what
taught the walk to keep the link rather than the item: two links held by one
file are two marks. The marks are gone, and what is left is a line per item.

## Design

**One line per item, below a given line.** `neighbours` returns the items one
hop away, and an item that more than one link of the line above reaches is
returned once. The order is the order the first such link is in the file, so
the line stays where the writer first wrote it.

**What counts as the same item is the path.** Two links are one edge when
they resolve to the same item, `20250101-bug-x` and
`../open/20250101-bug-x` alike, since that is the resolution the walk already
does for every link.

**The rule is per line, not per tree.** An item reached from two directions
-- two different items above it -- is still printed under both, as it is
today: those are two neighbours of two lines, and deduplicating across the
whole tree would be a different change, the one `cargo tree` declines and for
the same reason. What stops is the same line repeating, not the same item.

**The cycle rule does not change.** The items on the current path are what
stops a cycle, so a self-link and a two-item cycle read as they do today: the
item is printed and the walk stops under it with the note.

**`--invert` gets the rule by being the same walk.** The neighbours are
gathered from a different end, and an open item that links the one above it
in two places is one neighbour there too.

## Alternatives

**Deduplicate the whole tree.** Print each item once in the run, wherever it
is reached from. It loses for the reason `cargo tree` prints a shared
dependency under both parents: which of several lines reached an item is what
the reader is following, and the first of them is not a more truthful one. An
item shared by two open items is reached twice, truly, and cutting it to one
line says one of them reaches it.

**Do it where the links are read.** The edge already drops links that name
nothing, so it could drop the repeats too, before `Item` is built. It loses
because it is the wrong question for the wrong caller: `close` asks which open
items name the file it is moving, and an item that names it twice is one item
in that answer as well, so the rule belongs to the walk that prints, not to
the reading that both share.

**Leave it, and say the walk is a path tree like `cargo tree`.** `cargo tree`
does print a package twice when two dependency edges reach it. It loses
because those are two edges between four different packages, and here they
are two edges from the same file to the same file's neighbour: there is no
second path, only the same path written twice.

## Open questions

Whether the same rule should reach `nokori close`'s report of open items that
link the one being closed. It does not repeat today for the same reason it
could: it lists the items themselves, and an item appears once however often
it links the target.

Whether a repeat should be printed nowhere or said in a note. Nowhere: a note
under every line that two links reached is noise about the file's writing,
and the reader who wrote the links knows they wrote two.

## Outcome
