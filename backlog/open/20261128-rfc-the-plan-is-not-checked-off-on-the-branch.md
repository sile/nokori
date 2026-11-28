# RFC: The plan is not checked off on the branch

## Summary

A `plan` is no longer ticked where the work happens. The checkboxes stay as
they are written, and how far the work has got is kept out of the file until
the plan is closed. `nokori close` starts refusing a plan whose steps are not
all ticked, so that a plan in `backlog/open/` is one that is not finished, and
the `## Outcome` says which steps ran.

What is lost is resuming from the file: a reader can no longer open a plan
and see where to start. That was never a thing the file could do well enough
to be worth what it costs, and it is the kind of thing a working copy does
better.

## Motivation

A plan is ticked off as the work happens, and the work happens on branches.
So the file changes on every branch that carries part of the plan, in the
same lines, and the branches meet in the same checkboxes.

A plan big enough to have parallel work is where this shows. Two branches
take two parts of the steps, and each ticks its own box because that is the
instruction; the requests then conflict on `## Steps`, and the conflict is
not about the code. Nobody resolving it can tell which side is right, since
both are: the box is ticked on each branch because the step is done on each
branch. The merge turns into a pile of checkbox edits, and the plan that
comes out says nothing about which parts have landed together.

Ticking later does not fix it, only moves it. A step that is done on a branch
but not merged is not done, so ticking it after the merge puts the edit on
the main line, where a second plan branching before it conflicts anyway.

What the file is being asked to do is track the progress of the work, and
progress is a property of a working copy, not of a record. Git already knows
which commits are on which branch; a file that repeats this in checkboxes is
a second copy that is wrong whenever the two are apart, which is most of the
time on a branch.

## Design

**The steps stay checkboxes.** A step is still written as `- [ ]`, and the
list is still the order the work goes in. What changes is who ticks them: the
writer of the plan, when it is closed, if at all.

The template keeps the box and loses the instruction to tick it as the step
finishes. The `## Steps` comment says the boxes are ticked when the plan is
closed, and that the progress of the work is kept somewhere else, in as much
detail as the work needs.

**`close` refuses a plan with an unticked step.** This is the rule that makes
the box mean something again: an unticked box in `backlog/open/` says the
plan is not finished, and one in `backlog/closed/` says the step was decided
against, which the `## Outcome` says why. Without the refusal the two would
be the same, and a closed plan with unticked boxes would be one whose steps
might or might not have run.

`close` reads `- [ ]` and `- [x]` in `## Steps` and nothing else. A plan with
steps left is not closed, and a plan that is not going to be finished is
closed by saying so in `## Outcome` first -- the decision to stop is written
in the section that records decisions, not made by leaving boxes alone.

**The `## Outcome` says which steps ran.** It already says so, and it is now
the only place: a plan that is finished says all the steps ran, one that was
cut short says which and why.

**How far the work has got is kept where the work is.** A working copy is the
natural place, and so is a note in the plan's own branch, or nothing at all
where the plan is small enough to hold in mind. The flow does not say which:
a record of what is being considered and what was decided is not a tracker,
and the shape of a project's branches is its own business.

## Alternatives

**Leave it as it is and let the branches conflict.** The conflict is
resolvable, and a plan with parallel branches is not the common case. It
loses because the conflict is unresolvable in the way that matters: both
sides are right, so the person resolving picks one and the record loses the
other. Repeating that per merge is a cost paid every time to keep a copy of
git's state in a file.

**Tick the boxes after the merge, on the main line.** One edit per branch,
where the branches do not meet. It loses because it is the same edit as the
first, moved: a plan that branched before the merge conflicts on the line
that was added, so the conflict is rarer and not gone. It also needs a writer
who knows which steps merged, which is the reading the box was supposed to
save.

**Have `nokori` read git and tick the boxes.** The tool knows which commits
have landed, so the step could carry the commit that does it. It loses
because `nokori` does not read git anywhere else, and a checkbox made from a
commit hash is a claim about the code that the plan is not in a position to
check -- the step could be half done, or done differently, and the hash would
still be there.

**Drop the checkboxes and write the steps as a list.** Simpler, and the
conflict goes away with the boxes. It loses because a box is still worth
having: the closed plan is more readable for saying what did and did not run,
and the refusal in `close` gives the box a meaning that a plain line does not
have.

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
