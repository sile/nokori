# RFC: A cycle is an item already printed

## Summary

`nokori tree` loses its second rule. An item the walk has already printed is
printed with `(*)` and not walked again, and a cycle is one of those: the item
is on the path because the walk has just printed it. So the walk keeps one
set, not two, and the sentence it prints under a circle goes away -- the cycle
stops at the same mark as a repeat.

This answers [20261123-rfc-a-tree-line-is-one-item-once](../closed/20261123-rfc-a-tree-line-is-one-item-once.md),
which chose the opposite and gave its reason in `Open questions` and again in
`Outcome`.

## Motivation

The rule that was written down is that a cycle gets a note rather than a mark,
because a cycle is about the path and a mark is about what was printed. That
distinction is not there. The path is a list of items the walk has printed:
the root goes in before the first line, and a line goes in before its subtree
is walked. An item on the path has been printed, every time, by definition --
so narrowing the question to the path asks a subset of the question the mark
already asks, and the case the note catches is one the mark would catch too.

What the second rule costs is paid on every read of the code. `walk_children`
keeps two containers, and the one for the path is asked first and separately,
with a comment saying that this is before the printed set because a circle is
a thing to say rather than a repeat of a name. A reader who wants to know when
the walk stops has to hold both, work out that one is contained in the other,
and then decide which of the two the branch they are reading was written for.

The other cost is the sentence itself: two more lines in the output, and a
second shape of ending for a reader to know. An output already reads a line
that ends in `(*)` as one to look for above; a line that is followed by a
parenthetical is read as one to look for above too, once the reader has been
taught that it means nearly the same thing.

It also puts a claim in the output that is not true. The sentence says the
walk stops there; what it means is that the walk stops wherever the item was
printed first. A reader who takes it at its word goes looking for a circle
that is only a circle if they start from the line they are on.

## Design

The walk keeps the printed set and nothing else. For each item one hop from
`item`:

1. if the item is in the printed set, print its line with `(*)` after it and
   do not walk it;
2. otherwise, print its line, add it to the printed set, and walk it.

The root is in the printed set before the first line, so a walk that comes
back to the root marks it like any other item.

What goes: the `path` list, the check `path.contains(&next.path)`, and the
two-line sentence `(already on the path: the links run in a circle, so the walk
stops here)`.

What a circle prints, before and after, for two closed items that cite each
other:

```
closed/20261122-rfc-a                     closed/20261122-rfc-a
└── closed/20261122-rfc-b                  └── closed/20261122-rfc-b
    └── closed/20261122-rfc-a                 └── closed/20261122-rfc-a (*)
        (already on the path: the links
         run in a circle, so the walk stops
         here)
```

`explain references` loses the paragraph in "Reading the relations back" that
says "A cycle -- possible between two closed items -- stops the walk where it
meets it and says so." The paragraph that already says an item the run has
printed is printed with `(*)` covers the cycle too, and is the only one left.

Nothing else changes: the direction, the order, the branches, the prefix on
every line, and the roots resetting the printed set between them are all
untouched. A cycle was reachable only among closed items, and it still is --
the ban that keeps an open item out of a circle is `close`'s, not `tree`'s.

## Alternatives

**Keep the note and drop `(*)` instead.** One rule either way, so the cost of
two sets goes; a repeat and a cycle would then both be marked the same way.
It loses because `(*)` is the mark the output is read with, in this tool and
in `cargo tree`, and reusing its slot for a note that needs two lines under a
name makes every line longer to make one rare line clearer. The mark that says
less is the one for the common case.

**One set, but say in the output that this one was a cycle.** Print `(*)`
and, when the item was on the path, add the sentence after the line. It loses
because it keeps both outputs and drops neither, so a reader still has two
endings to learn; and the sentence is still the claim from `Motivation` that
is nearly true, now sitting under a mark that says the opposite.

**Keep it as it is, and accept the second set as the price of a true
sentence.** This is what [20261123-rfc-a-tree-line-is-one-item-once](../closed/20261123-rfc-a-tree-line-is-one-item-once.md)
chose. It loses because the sentence is not true: the walk stops because the
item was printed before, and being on the path is only one of the ways that
happens. A rule that costs a set and a branch to say something the other rule
already says has to say something the other rule does not, and it does not.

That RFC weighed two sets against one set with a note, and the note is what it
chose. It did not weigh one set against one set with a note, which is what is
asked here, and the paragraph above is about the sentence rather than the set,
so it does not settle this either. What it fixes is what a cycle read like
while the second set was there, and that reading is what this RFC changes.

## Open questions

Whether the output says anywhere that the item marked was reached by going
around in a circle, since a reader who sees only `(*)` may look above and find
the circle rather than a line. Expected answer: it does not. Where an item is
printed first is where the reader is told to read it, and a line found above
explains itself; a reader who wants to know whether they are in a circle can
already see that the line they are on is above.

Whether the mark on a root reached from its own tree is enough. Today
the root is in the printed set before the first line, so a tree that comes
back to its root marks it. That is the same case and is unchanged.

## Outcome

<!-- NOKORI_TODO: the one section written later, when `nokori close` moves
this file. It stays empty while the RFC is open, this comment and all, and
the comment is deleted in the same edit that writes the section -- which is
the close, since `close` refuses a file whose outcome is still empty.

The sections above are written before the item moves: by the time `nokori
close` moves this file, they are written and their comments are gone.

Those sections may be rewritten as long as the item is open, as often as the
understanding changes. What they may not become is a description of work
already done.

Say whether the RFC was accepted or rejected, and why. When it is accepted,
name the change that implements it, or the item that does, if the work happens
somewhere else.

To refer to another item, link it:
[20261114-bug-empty-input](../closed/20261114-bug-empty-input.md). Link only an item
that is already closed: never link one that is still open, not even as
background. The rule is about this section and not the file: a closed item
may not cite an open one, while an open item may cite either, so a section
above this one may link an open item. Run `nokori explain references` for the
paths and the rule.

An accepted RFC can still be overturned later. Say so in the later item and
cite this one; do not edit this file afterwards to follow it.

Once this section is written, treat the file as closed: do not
edit any part of it again, and append nothing later. If something turns up
later, leave the file as it is and open a new item that cites this one and
acts on it.

Delete this comment in the same edit that writes the section, and commit both
together with the move. -->
