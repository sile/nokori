# RFC: The branch does not tick the plan

## Summary

The checkboxes stay, and a step is ticked when it is done -- but not by the
branch that does it. A working branch carries a step, or a few steps, and the
branch is what the step is done on, not what decides it is done. So the tick
goes somewhere other than the branch: on the main line, with the work already
merged, or by whoever is reading the plan at the time.

`nokori` itself does not change, and no command reads a branch. What changes
is what `explain flow` says about carrying a `plan`, which is now the same as
what it says about any other item: the file's edits are committed apart from
the work.

## Motivation

A working branch does not correspond to a plan as a whole. It carries one
step, or a few, or a part of one that was cut in two while it was being done.
The plan is bigger than any branch that serves it, and the steps that are not
in that branch are not failed or skipped by it -- they are simply not what the
branch is about.

So a branch cannot say whether a step is done. What it knows is what it
changed, and what it changed is one way of doing a step: the others are on
other branches, or not written yet. The step is done when the plan says so,
which is a judgement about the plan, made with the plan in view.

Ticking on the branch is what makes the two shapes collide. A plan with two
branches taking two parts of it gets each box ticked twice, on the same line,
and the requests conflict on `## Steps` -- a conflict nobody can resolve,
because each side is right about the branch it is on. That is the example, and
the severe one, but the same thing is smaller elsewhere: a branch that carries
one step still carries a tick for a plan it has only glanced at, and a second
branch off the same plan disagrees with it about a line neither was asked
about.

Keeping the file out of the branch is the same rule the flow already has for
every item, and it is the reason the rule holds here too: an item is what the
work came from, read by the person doing the work and not by the reviewer of
the change.

## Design

**The template does not change.** A step is still `- [ ]`, still written so
that the box carries a meaning, and still ticked when the step finishes. What
is said around it is where the tick goes, and that is not the branch: the
`## Steps` comment says the tick belongs to the plan, and the branch that
carried the step does not decide it.

**`explain flow` gains a `plan` of its own.** The `Closing` section already
says the item and the change are committed apart. It gets a paragraph for
`plan`: a plan is carried out over several branches, and none of them carries
the plan; the file goes in on its own, and the boxes are ticked where the plan
is, in a commit that is not the change.

**The order of ticking is left open.** Ticking when the step is merged and
ticking when somebody next reads the plan are both the same shape -- a tick
that is not on the branch that did the work. Which one a project uses depends
on how often somebody is looking at the plan, and saying so here would be a
rule `nokori` cannot check and nobody needs.

**Nothing in the tool changes.** `close` still does not look at the boxes, and
an unchecked box in a closed plan still means the step was decided against.
The change is one paragraph of prose and one comment.

## Alternatives

**Tick on the branch, as now, and accept the conflict.** It loses because the
conflict is the one kind that cannot be resolved by reading it: both sides are
right, so whoever resolves it picks a side and the plan loses the other. It
also asks a branch to answer a question about the whole plan, which it cannot.

**Stop ticking altogether; the `## Outcome` says what ran.** Simple, and the
conflict goes away with the ticks. It loses because the box is the plan's own
record of where it stands, and a reader of an open plan is exactly who wants
it. What is wrong is where the tick is committed, not that it exists.

**Have `nokori` read git and tick the boxes.** The tool knows which commits
have landed, so a step could carry the commit that does it. It loses because
`nokori` reads git nowhere else, and a tick made from a commit hash is a claim
about the code that the plan cannot check -- the step could be half done, or
done differently, and the hash would be there anyway.

**Tick after the merge, on the main line, as the rule for every kind.** The
closest to the design, and it loses only on being too strong: the shape that
matters is that the tick is not on the branch that did the work, and requiring
one particular line makes a rule out of an arrangement.

## Open questions

None.

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
