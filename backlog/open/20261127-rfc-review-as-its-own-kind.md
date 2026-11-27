# RFC: a review is an item of its own kind

## Summary

A `review` becomes a kind of item. What a reviewer says about a change before
it lands -- what they looked at, what they found, what they would have done
differently -- gets a file of its own, instead of being said in the pull
request, in the commit, or in a `feedback` item that happens to mention it.

The shape is the proposal: whether `review` is a kind at all, where the
sections fall, and how it relates to the change it reviews. The wording of
every section is the work that follows, so what is asked here is a template
that can be tried and reshaped, not one that has to be right first.

## Motivation

Review is a step in the flow, and it is the one step the tool says nothing
about. `create`, `check`, and `close` cover the item from the moment it is
written to the moment it settles; what happens between the work being done
and it landing is a pull request, which is reviewed in a place that is not
this one and does not keep what was said.

It reads like `feedback`, and that is why the question is worth asking. Both
are written by somebody other than whoever did the work, and both keep the
reason rather than the decision. What separates them is the object: feedback
is about the program, and it is answered by changing the program or declining
and saying why; a review is about one change, and it is answered by that
change being edited or merged, or by the review being the reason it was not. Feedback can sit in the backlog for a year; a
review is written against a change that is still open, and it is stale the
moment the change lands.

A review has a shape that feedback does not. A reviewer is asked to look, so
the file says what was looked at and what was found, and a reviewer who found
nothing says so rather than leaving the file out. That is the part a kind
would make clearer than a paragraph in a pull request, and it is what the
sections are for.

## Design

`review` is added to `KINDS`, and `nokori create review <slug>` writes
`backlog/open/YYYYMMDD-review-<slug>.md` from a new template. No command
changes otherwise: `check`, `close`, and the link rules treat it like any
other item.

The sections the template starts with, to be argued about and reshaped:

- `## Summary` -- the change reviewed, named so that a reader knows what the
  file is about without opening anything else;
- `## What I looked at` -- the parts of the change the review covers, and what
  was left out;
- `## What I found` -- what the reviewer saw, as they would say it;
- `## Notes` -- anything else, including what is expected of the change back;
- `## Outcome` -- written at the close, saying how the change answered the
  review.

The change under review is linked, not copied: the item cites the pull request
or the commit it was written against, and the code never cites the item back.
`nokori explain kinds` gains a `review` entry and a paragraph in "Choosing
between the kinds"; `explain` may also want a topic of its own for the review
side of the flow, if the sections turn out to need more than a template's
comments to explain.

## Alternatives

**Review in a pull request, where the change already is.** It is where the
diff is, it is where the author and the reviewer already meet, and it costs
no new kind. It loses less than it first looks: what the backlog is for is
the reasoning, and a review comment is reasoning. What it does not have is a
place in the flow -- a review that opened a question nobody answered leaves
the pull request and is gone, while an item is closed with an `## Outcome`
that says how it was answered.

**`feedback` carries it.** The two are close enough that a paragraph in
`explain kinds` could say when the subject is a change rather than the
program. It loses on the form: feedback's section is deliberately shapeless,
which is what lets a person write it in their own words, and a review is
written by somebody who was asked to look and is expected to say what they
looked at. A kind whose whole reason is a shape should not be a kind whose
rule is that there is no shape.

**A `plan` with the review in a step.** The step can be ticked when the
review is done, and the file is already the place for work that spans more
than a sitting. It loses because a plan says what will be done, and a review
says what was found: a step whose body is the finding is a kind with a
heading the wrong way round.

## Open questions

Whether the sections above are the right ones. The three argued for here are
`What I looked at` and `What I found`, and a `Summary` that names the change;
whether `Notes` earns its place is the first thing to try. Each section wants
a comment in the template saying what belongs, the way the other kinds do, so
some of this is writing rather than deciding.

Whether a review may cite an open item. It reviews a change that is still
being made, so the thing it is about is often an open `rfc` or `plan`, and
naming it is the whole point. That is allowed today, since only a closed item
is barred from citing an open one -- but a review is closed while the change
it reviews is often still open, which is the shape the rule was written to
prevent. Expected answer: the rule stays as it is, and a review whose change
has not landed is either kept open until it does, like an accepted RFC, or
closed as a record of a review of a change that no longer exists.

Whether `nokori` wants a command for the review side of the flow, in the way
`nokori feedback` exists for reports from outside. Expected answer: not yet.
A review is written from inside the repository by somebody who already has
the template in front of them, which is `create`.

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
