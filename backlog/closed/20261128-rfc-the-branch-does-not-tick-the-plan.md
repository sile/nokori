# RFC: The branch does not tick the plan

## Summary

The checkboxes stay, and a step is still ticked when it is done -- but not by
the branch that does it. A working branch carries a step, or a few steps, and
the branch is what the step is done on, not what decides it is done. So the
tick goes where the plan is, in a commit of its own, apart from the change
that did the work; when it is written is left to whoever does it.

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

**The steps keep their boxes.** A step is still `- [ ]`, still written so that
the box carries a meaning, and still ticked when it is done. What changes is
the sentence around it: where the tick goes, and what it is not.

**The `## Steps` comment says only what the box belongs to.** That the box is
the plan's, and that a branch does not decide it: a branch carries a step or a
few, or part of one, and knows what it changed rather than whether the step is
finished. The comment stops there.

**The order of ticking is not said.** When the tick is written -- as the step
is merged, when somebody next picks the plan up, on a branch of the plan's own
-- is a matter of how the work is arranged, and a backlog that is kept apart
from the code is exactly the arrangement that cannot know it. Naming one order
would make a rule out of somebody's habit, and make the other shapes look
wrong. What has to hold is only that the tick is not on the branch that did the
work; everything else is left to the writer.

**`explain flow` gains a `plan` of its own.** The `Closing` section already
says the item and the change are committed apart. It gets a paragraph for
`plan`, saying the same thing in the plan's own terms: a plan is carried out
over more branches than one, and none of them carries the plan, so the file
goes in on its own and the boxes are ticked where the plan is. As with the
comment, it says where and not when.

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

**Tick after the merge, on the main line, and say so.** The closest to the
design, and it loses on the same point the design avoids: merging is one way
to finish a step, not the only one, and naming it makes a rule out of a
particular arrangement of branches. Where the tick goes is the thing worth
saying; when it is written is the project's.

**Leave the timing out and nothing else.** Say nothing at all, and let the
writer work it out from the rest of the flow. It loses because the sentence
that is there now says the opposite -- tick as the step finishes, before
writing the next one -- and a writer who follows it ticks on the branch, which
is the whole problem. Something has to be said to replace it, and what is said
is where the tick goes.

## Open questions

None.

## Outcome

Accepted, and the tool does not change. The `## Steps` comment and the
`plan` paragraph of `explain flow` are in.

**What was hard was keeping the text to one thing.** The first draft had the
box left unticked while the plan was open, with `close` refusing a plan whose
steps were unfinished. That is a different decision -- one about what the
box means -- and it followed from the same mistake the conflict comes from:
treating the box as a report on the work rather than as the plan's own
record. The box stays, and a step is still ticked when it is done.

**The timing is not said, and that was the second correction.** Naming it --
as the step merges, when somebody next picks the plan up -- makes one
arrangement into a rule and the others into mistakes. The premise is already
that the backlog and the work are apart, so how the branches are run is
precisely what this cannot know. The text says where the tick goes and
stops.

What is left to a project is not a gap to be filled in later: where the
tick goes is the part that had to be said, and the rest is what each project
already knows about itself.
