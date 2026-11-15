# Feedback: "settled" does not say whether an accepted RFC may close before it is built

## Feedback

I wrote two RFCs, the direction for both had been agreed, and then I tried to
close them. `nokori close` refused because `## Outcome` was still empty, so I
wrote the `Outcome` sections myself and closed both. That was the wrong thing
to do, but the reason I thought it was allowed is in `nokori explain flow`:

> An item is closed when it is settled, not when the work is finished. A `bug`
> that turns out not to be a bug, an `rfc` that is turned down, and a `plan`
> whose steps were decided against are all closed, with the reason in
> `## Outcome`.

Every example in that sentence is a decision not to do the work: not a bug,
turned down, decided against. Nothing there is a proposal that was accepted and
is waiting to be built, so I read "settled" as "the decision is made" and
closed an accepted RFC with nothing implemented. The person reviewing it
pointed out that this does not work: a closed file is never edited, so if the
implementation changes a detail of the design -- which it does -- the closed
record says something that is no longer true and cannot be corrected.

What I want is for that paragraph to say what happens to an accepted proposal
that nobody has built yet. Three answers would each be clear, and I do not know
which one is meant:

- an accepted RFC may be closed before its implementation, because closing
  records the decision and a plan carries the work;
- an accepted RFC stays open until the implementation lands, because the design
  is not settled while it is still being read against the code;
- either is fine, but `close` should say which one the backlog expects, so the
  next person does not have to infer it from an example that does not cover it.

I have followed the second reading since then and left the two RFCs in
`backlog/open/`. What is worth fixing is that I had to guess: the section that
explains when to close does not answer the case that made me close too early.

## Outcome

<!-- NOKORI_TODO: the one section written later, when `nokori close` moves
this file. Leave it empty until then, comment and all.

Two things are recorded here, and both are needed: what was decided, and what
came of it. The decision is written when it is made, even though the file is
not closed until the result is known, because the reason for a decision is hard
to recover later and is exactly what a reader of a closed file asks for.

Accepted, reshaped, declined, and held -- say which, and why. A declined
request needs its reason as much as an accepted one: the next person with the
same complaint will look for this file, and "no" without a reason sends them to
file it again.

Then say what came of it. A new item usually carries the work, so name that
item and let it stand for the change; whether it is a bug that reports the
program misbehaving, an RFC that argues the new shape, or a plan that says how
the work runs is decided then, by what the response turned out to be. When the
response was to do nothing, say that instead: an `Outcome` that says nothing
about the result is read as work nobody got to, not as a decision to leave it.

Once this section is written and committed, treat the file as closed: do not
edit any part of it again, and append nothing later. If something turns up
later, leave the file as it is and open a new item that cites this one and
acts on it.

To refer to another item, link it:
[bug-empty-input](../closed/20261114-bug-empty-input.md). Link only an item
that is already closed: never link one that is still open, not even as
background. Run `nokori explain references` for the paths and the rule.

Delete this comment when you write the section. -->
