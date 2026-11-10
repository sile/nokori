# Plan: <!-- NOKORI_TODO: one line naming the work this plan carries out -->

## Goal

<!-- NOKORI_TODO: the state that says this plan is finished, in a form a
reader can check.

A plan is a piece of work, not a proposal: the question of whether to do it
was settled before the file was created (usually by an RFC), and what is left
is how, in what order, and how far. So the goal is the end, not the
argument for the end.

Write it so that "finished" is a fact rather than a feeling. "The parser no
longer allocates per token" can be checked; "the parser is faster" cannot.

An implementation detail that is too small for an RFC belongs here, in the
plan, not in the RFC: a proposal is argued on its shape, and the steps that
follow from it are what this file is for.

Delete this comment once the paragraph is here. -->

## Items

<!-- NOKORI_TODO: (optional) the other items this plan is carried out
through, one per line, by slug.

A plan is often the one place where work that spans several items is kept in
order: this RFC has to be accepted first, that bug waits on it, this other
plan runs in parallel. Naming them here is what makes the order visible
without opening each file.

Refer to them by slug only -- `rfc-cache-the-parse`, not a path. An item
moves to `closed/` when it settles, and a path would stop pointing at the
file the moment it did.

A plan that touches no other item is a normal plan: when there is nothing to
link, delete this section along with its comment.

Delete this comment once the list is here. -->

## Steps

<!-- NOKORI_TODO: the steps, in order, as checkboxes -- one per line, each
saying what is done and what is true when it is.

- [ ] Split the lexer out of `parse.rs`; the crate builds with both modules.
- [ ] Run the old and new parsers over `tests/fixtures` and compare output.

Write a step so that its checkbox carries a meaning: an unchecked box says
the work is not done, not that nobody got to it today. A step that is too big
to be one checkbox is two steps.

Tick a box as the step finishes, before writing the next one. That is the
whole of resume-from-here: there is no separate section saying where the work
stopped, because the unchecked boxes say it.

Delete this comment once the steps are here. -->

## Notes

<!-- NOKORI_TODO: (optional) what the steps need in order to be carried
out, without turning this into a design document.

Worth saying here: what is assumed to be true before the first step, where
the work is likely to get stuck, and which step is the one to cut if the plan
turns out to be too big.

The design itself is not repeated here. If the shape of the change is still
open, that is an RFC waiting to be written, not a plan with a long Notes
section.

Delete this comment once the section says what it should, or the whole
section if there is nothing to add. -->

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

Say which steps ran, which did not, and why the ones that did not were left
out. Name the change that carried the plan out, or the item that did, when
the work happened somewhere else.

An unchecked box in a closed plan means the step was decided against, not
forgotten: `close` does not refuse a plan with steps left, so this section is
where the decision is recorded. A plan that stops partway through and should
be picked up again is not closed -- it stays in `backlog/` with the unchecked
boxes showing where it stopped.

Once this section is written and committed, treat the file as closed: do not
edit any part of it again, and append nothing later. If something turns up
later, leave the file as it is and open a new item that cites this one and
acts on it.

Delete this comment when you write the section. -->
