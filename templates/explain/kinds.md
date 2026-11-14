rfc
===

A proposal for a different contract: something should be decided, and the
RFC says what, why, and what was considered instead.

Write one when the answer is not yet a matter of course -- when a reader could
reasonably want something else. If the Summary still has to explain the
problem rather than the change, that is a `bug` with a note about the fix.

Sections: Summary, Motivation, Design, Alternatives, Open questions.


bug
===

A report that today's behavior breaks its own documented contract. Not a
proposal: what should happen is already written down somewhere, and the item
points at the contradiction.

Write one when something is wrong in a way that people will hit again. A
mistake made once and fixed on the spot is a commit, not an item.

Sections: Summary, Reproduction, Observed behavior, Expected behavior, Notes.


plan
====

A plan for work whose worth is already settled: what to do, in what order, and
how far it has got. The order and the progress are the point; the question of
whether to do it should have been answered before the file was created.

Write one when the work spans several items, when one piece has to wait for
another, when it will be picked up and put down more than once, or when it is
large enough that losing track of the steps is a real risk. It is also where
implementation detail too small for an RFC goes, so that the RFC can stay a
proposal about shape.

Sections: Goal, Items (optional), Steps, Notes (optional).


feedback
========

A complaint kept from where it was made: someone met something in the program
that got in their way, and this file holds what they wanted instead and why.

The difference from `bug` is not that nothing is wrong. A `bug` says the
program did something other than what it says it does, and the report is
checked by reproducing it. Feedback says the program did what it does, and
what it does is worse than what the person needed. Nothing is reproduced and
nothing is fixed, so the response is to change what the program does, or to
decline and say why.

Write one when the complaint should stay in the words of the person who has
it. A request that a maintainer would have filed as a `bug` or an `rfc` in
their own voice is still feedback when the reason for it is the part worth
keeping -- what got in the way, from the side it got in the way of. It is
not a rule about who opens the file: a maintainer may open one on behalf of a
user, and a user may open a `bug` or an `rfc`.

Sections: What I want.


Choosing between the kinds
==========================

The three kinds are one decision and two pieces of work. `rfc` and `bug` are
the decisions: one says what the contract should become, the other says the
contract is already broken. `plan` is the work, whether or not an item
records why it is being done. `feedback` is the reason, kept until a decision
is made about it, and it is closed by that decision rather than by work.

Choosing between `rfc` and `bug` is a question about the answer: is what
should happen already written down (a `bug`), or is it the thing being decided
(an `rfc`)?

Choosing `plan` is a different question, about the shape of the work rather
than its subject: a change small enough to finish in one sitting is an `rfc`
or a `bug` whose fix is a commit. When the work outgrows that, the plan is
written, and it may cite the `rfc` it carries out or stand on its own.

`feedback` is chosen by where the complaint is written from, not by what
should happen: it is the file for the reason, kept from the side that met it.
When the same request is filed as an `rfc`, the reason is summarized in the
Motivation by whoever argues for the change; when it is filed as feedback,
the reason is the file, and the proposal, if there is one, comes later and
cites it.

Every kind is closed the same way, and the kind does not change while the item
is open. If the kind was wrong, the file is deleted and created again.


Kinds that are not there
========================

There is no `spike` kind. A spike is a question asked by trying something, and
separating it from `plan` would need a rule that says which of the two an item
is -- a rule applied afresh each time, before doing the work that would tell
you the answer. A plan whose first step is an experiment says so in its steps,
and its Outcome records what was learned whether the steps ran or the answer
was no.

There is no `memo` kind either. A note that never closes is still an item; an
idea not yet worth doing is an ordinary item that happens to be idle.
