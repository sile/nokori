# RFC: `explain how-to-feedback` says how somebody outside reports

## Summary

This answers the second half of
[feedback-094641](../closed/20261122-feedback-094641.md).

`explain` gains a topic, `how-to-feedback`, that says what a person does when
they want to send a report to somebody else's backlog: the command, where the
words come from, where the file lands, and what happens to it there. The
point is that a developer can tell somebody outside the repository "use
`nokori feedback`; run `nokori explain how-to-feedback` for the flow" and be
understood. It is not the `feedback` kind explained among the kinds; it is the
flow of a role that the default flow does not describe.

## Motivation

An item template tells a writer what to put in the file once it exists. The
`feedback` template is the same: it has a `## Feedback` section and notes on
what belongs there. What it does not carry -- and no template does, since a
template is read after the file is created -- is how the file comes to exist
when the writer is not the developer of the backlog.

The default flow in `explain flow` is written for the developer working in
their own repository: `create` makes a file, `check` reads it, `close` moves
it. Sending a report to somebody else's repo is a different reader with a
different question. They do not have the repository, they do not run `create`,
and the rules that matter to them -- which directory the words are read from,
where the item lands, what the receiving repository does next -- are the ones
`flow` scatters around its default case: a paragraph in "Writing down what
somebody said", a note in `references`. A developer who wants to hand off that
flow has nothing to point at.

## Design

A new entry in `templates::TOPICS`, named `how-to-feedback`, printed by
`nokori explain how-to-feedback`. It describes one round of the flow, in the
tense a person outside the repository needs:

- the command, `nokori feedback <path>`, and that `<path>` lets them send to a
  different repository than the one they stand in;
- where the words come from: `-m`, `-f`, standard input, the same way `git
  commit` takes a message;
- the file name it writes, and why it has no slug and no title line: a summary
  is made by reading the words, which the run has not done;
- that nothing is committed, in either repository;
- what the receiving repository does: reads the words, decides what they
  become, and closes the feedback with the decision in `## Outcome`;
- that a duplicate is not damage here, and that a relation to another item may
  be added but is not demanded of somebody who hit a problem and moved on.

The topic closes by pointing at `nokori explain flow` for what happens on the
receiving side, and at `nokori explain references` for the form of a link.

Where the material is taken from is part of the design. The paragraphs in
`flow` under "Writing down what somebody said" that are about `feedback`
specifically move here. The parts that are about items in general -- the
relation to another item, when it is written, which sits in the same part of
`flow` -- stay where they are, because a developer creating any kind of item
needs them. `flow` keeps a one-line pointer: a reporter's flow is
`how-to-feedback`, so a developer reading `flow` knows where the other entry
is without that whole flow being reproduced in the middle of theirs.

### Naming

`explain` keeps its shape: the long-form topics (`overview`, `kinds`, `flow`,
`references`) are about the states, the kinds, and the relations, and they
read the same whatever command the reader arrived from. A command's own flow
is a different kind of page, and it is named for the command:
`how-to-<command>`. `feedback` was the first command with a flow that its user
is not the developer, which is why it is the first such topic; the rule is
written down here so that the next one is a name and not an argument.

## Alternatives

**A `feedback` topic among the kinds.** Read as "what a feedback kind is",
next to `kinds`, it answers the wrong question: the reader wants to send one,
not to know how kinds are counted. Keeping `kinds` about the kinds and naming
this page for the flow keeps both readings clean.

**Extend `flow` with a feedback section.** It mixes two readers into one page:
the developer who creates items and the person outside who sends a report.
Each would read past the other's material to find theirs, and the developer who
wants to hand off the flow has no short name to give.

**Name it `feedback-flow`.** Accurate, but it reads as a companion to `flow`
rather than as a page a non-developer can be sent to. `how-to-feedback` says
what the reader gets: how to do it.

**Do nothing.** The material is all in `flow` and `references` today. But the
use case is a pointer, and a pointer needs a short name; without one the
developer has to paraphrase the flow, which is what this is meant to replace.

## Open questions

- How much of the relation rule belongs in `how-to-feedback`? A report that
  cites nothing is normal, so the page says it is optional and points at
  `references` for the form, rather than reproducing the rule.
- Does `flow` keep the `feedback` mention in "Writing down what somebody said"
at all, or only the pointer? Settled while writing: the pointer, and nothing
that duplicates the new page.

## Outcome
