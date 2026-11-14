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
[rfc-how-items-refer-to-each-other](closed/20261114-rfc-how-items-refer-to-each-other.md)
[rfc-never-cite-an-open-item-from-a-closed-one](closed/20261114-rfc-never-cite-an-open-item-from-a-closed-one.md)
[feedback-rename-plan-items-section](closed/20261114-feedback-rename-plan-items-section.md)
```

Three items are named and no relation is stated. Does the plan wait for the
RFC, or carry it out? Is the feedback a reason to do either? The comment under the
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
- [ ] Apply
      [rfc-how-items-refer-to-each-other](closed/20261114-rfc-how-items-refer-to-each-other.md)
      to the templates, after the rule it decides is settled.
- [ ] Settle
      [rfc-never-cite-an-open-item-from-a-closed-one](closed/20261114-rfc-never-cite-an-open-item-from-a-closed-one.md)
      in parallel with it; neither has to wait for the other.
```

**A relation that is true of the plan as a whole goes in the Goal.**

```markdown
## Goal

Carry out
[rfc-how-items-refer-to-each-other](closed/20261114-rfc-how-items-refer-to-each-other.md):
every reference in `templates/` is a link to the file.
```

Both carry the relation in the writer's words, with no vocabulary to pick
from. A step may name the item at the front or the back of the sentence,
whichever reads better.

The comment under `Steps` gains two sentences: one saying that an item
another step waits on is linked in that step, and one pointing at
`nokori explain references` for how to write the link. The rule is then in
the file a writer has open, and the detail stays in the one place it is kept.
The `Sections:` line in `explain kinds` loses `Items`.

References are links, as decided in
[rfc-how-items-refer-to-each-other](closed/20261114-rfc-how-items-refer-to-each-other.md);
that one decides the spelling, and this one only decides where they go.

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

Both were settled when the template was written. The `Goal` comment says a
relation true of the whole plan goes there, so the writer is told where a
condition goes rather than left to think of it. How long a step may get is
left to the writer, and the template shows a relation in a step that runs to
three lines: if steps grow past that in practice, the rule is what gets
narrowed, and that is a new item rather than an answer here.

## Outcome

Accepted. The `Items` section is gone from the plan template, and the two
rules took its place: a relation that belongs to one step is written in that
step, and one that holds for the whole plan is written in `Goal`.

The `Steps` comment gained the sentence and the three-line example, and the
`Goal` comment gained the sentence about a whole-plan relation. `nokori
explain kinds` lists the plan's sections without `Items`.

Accepted together with
[rfc-how-items-refer-to-each-other](closed/20261114-rfc-how-items-refer-to-each-other.md)
and
[rfc-never-cite-an-open-item-from-a-closed-one](closed/20261114-rfc-never-cite-an-open-item-from-a-closed-one.md),
in one change: the link form is what a relation is written in, so this file
had nothing to add about it.

This is also where
[feedback-rename-plan-items-section](closed/20261114-feedback-rename-plan-items-section.md)
was answered, and that answer stands as it was written: the section was
renamed first, and then dropped, as this RFC decided.
