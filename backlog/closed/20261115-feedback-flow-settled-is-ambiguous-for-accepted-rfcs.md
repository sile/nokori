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

Accepted, as the second of the three answers the report offered: an accepted
`rfc` stays open until its implementation lands.

Stated a little wider than the report did. An RFC whose acceptance is the whole
of the work has no change to wait for, so the rule is written as "until the
change it names has landed" rather than "until the implementation is complete",
and the report's second reading is the ordinary case of that.

The reason is the one the report gave: a closed file is never edited, so a
design that still changes while the work is carried out would leave a record
saying something no longer true. The reason went into the text, since a rule
with no reason behind it is the one that gets broken next time.

The change is the commit `902fcc8` ("Say that an accepted RFC stays open until
it is built"), which adds the paragraph to `nokori explain flow` under `Closing`,
and a test that fails if the paragraph is dropped.

The report also offered a third answer, that `close` should say which reading
the backlog expects. It does not, and the new paragraph says why: `close` reads
the file and not the repository, so it cannot tell whether a named change has
landed, and a check it cannot make is not one to promise. What keeps an
accepted RFC open is the habit and the reason for it.
