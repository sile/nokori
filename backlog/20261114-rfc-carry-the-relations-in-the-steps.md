# RFC: carry the relations in the steps

The plan template loses its `Items` section. An item this plan waits on,
carries out, or runs beside is named in the step where it matters, or in the
Goal when it is a fact about the whole plan, and a plan that touches nothing
else simply has no such sentence.

What changes is not only where the relations are written but what a reader
can rely on: with the section gone there is no list that can hold three bare
slugs and pass as a section, because there is no list.

## Motivation

The `Items` section holds one line per item, and what a writer puts on the
line is the slug:

```
rfc-cache-the-parse
plan-split-the-lexer
bug-lexer-drops-eof
```

Three items are named and no relation is stated. Does the plan wait for the
RFC, or carry it out? Is the bug a reason to do either? The comment under the
section explains what belongs there, but the comment is read after the
heading, and the heading is what a reader uses to decide whether to read the
section at all.

The section is optional, and a plan that relates to nothing is expected to
delete it. That is the common case -- most plans are one piece of work in one
place -- so the template asks for the same edit every time, and asks the
writer to decide "is there anything to put here" before the plan has been
written far enough to know. An optional section that is usually empty spends
the writer's attention to no end.

What the section is trying to say is an ordering: this has to be settled
first, that waits, these two can go in either order. Order is already the
subject of `Steps`, which is a numbered list of when, and the relations are
what the order is made of. Kept in a separate section, the order is stated in
one place and carried out in another, and the writer says it twice or says it
in the wrong one.

## Design

The `## Items` section is deleted from the template, with its comment. In its
place, two rules about sentences that already have somewhere to go:

**A relation that belongs to one step goes in that step.**

```markdown
- [ ] Wait for `20261114-rfc-how-items-refer-to-each-other.md` to be accepted,
      then apply the same file name rule to the templates.
- [ ] Split the lexer out of `parse.rs`, in parallel with
      `plan-cache-the-parse`; neither has to wait for the other.
```

**A relation that is true of the plan as a whole goes in the Goal.**

```markdown
## Goal

Carry out `20261114-rfc-how-items-refer-to-each-other.md`: every reference in
`templates/` is written as a file name, not a path.
```

Both carry the relation in the writer's words, with no vocabulary to pick
from. A step may name the item at the front or the back of the sentence,
whichever reads better.

The comment under `Steps` gains one sentence saying that an item another step
waits on is named there, so that the rule is where a writer will be when
writing one. The `Sections:` line in `explain kinds` loses `Items`.

Items are named by file name, as decided in
`20261114-rfc-how-items-refer-to-each-other.md`; the two RFCs land together,
and that one is the one that decides the spelling.

## Alternatives

**Keep the section, rename it `Related items`, and make each line say the
relation.** This fixes the bare list, and it is what the motivation asks for
most directly. It does not fix the optional-section problem: the heading still
sits there as a question, still gets deleted most of the time, and the
ordering is still stated in a place other than the one that carries it out.
It buys a better-shaped list for a section that should not exist.

**Keep the section, rename it, and leave the lines alone.** The smallest
change of all, and it fixes only the half of the problem that the heading's
name causes. Three bare names under a better heading are still three bare
names.

**Fix the vocabulary.** Write down the relations a plan may have -- `carries
out`, `waits on`, `runs in parallel`, `is replaced by` -- and have writers
pick one. The result reads evenly, and the list can be checked. It also makes
the section a field to fill in: a writer with a relation that is not on the
list either squeezes it into the nearest word or leaves the line bare, and
the relations worth writing are exactly the ones nobody thought to list.

## Open questions

Does a step with a relation in it still read as one line of work when the
relation is long? The second example above runs to three lines inside one
checkbox; if steps in practice grow past three, the rule is what needs
narrowing, not the template.

A relation that belongs to every step ("this plan assumes the RFC is
accepted") has no step to go in unless it is repeated. Written once in the
Goal it covers the plan, but a writer may not think of the Goal as a place
for a condition. Whether the `Goal` comment should say so is left open.

## Outcome

<!-- NOKORI_TODO: the one section written later, when `nokori close` moves
this file. Leave it empty until then, comment and all.

The sections above are written before the item moves: by the time `nokori
close` moves this file, they are written, their comments are gone, and the
change is committed.

Those sections may be rewritten as long as the item is open, as often as the
understanding changes. What they may not become is a description of work
already done. `nokori close` needs the change committed, so whatever they say
at that point is what the moving commit records.

Say whether the RFC was accepted or rejected, and why. When it is accepted,
name the change that implements it, or the item that does, if the work happens
somewhere else.

An accepted RFC can still be overturned later. Say so in the later item and
cite this one; do not edit this file afterwards to follow it.

Once this section is written and committed, treat the file as closed: do not
edit any part of it again, and append nothing later. If something turns up
later, leave the file as it is and open a new item that cites this one and
acts on it.

Delete this comment when you write the section. -->
