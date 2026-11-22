# RFC: `explain feedback` says how somebody outside sends one

## Summary

This answers the second half of
[feedback-094641](../closed/20261122-feedback-094641.md).

`explain` gains a topic, `feedback`, that says what a person does when they
want to send a report to somebody else's backlog: the command, where the
words come from, where the file lands, and what happens to it there. The
point is that a developer can tell somebody outside the repository "use
`nokori feedback`; run `nokori explain feedback` for the flow" and be
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
flow has nothing short to point at: `explain flow` is a page about the
developer's own items, and reading it for the reporter's flow means reading
past material written for somebody else.

## Design

A new entry in `templates::TOPICS`, named `feedback`, printed by `nokori
explain feedback`. It is a short page -- the reporter's first stretch of the
flow -- and it does not repeat the default flow around it:

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

The topic opens by saying which reader it is for -- somebody outside the
repository, sending a report to a backlog that is not theirs -- so that a
reader who arrived by name does not mistake it for the `feedback` kind. It
closes by pointing at `nokori explain flow` for what happens on the receiving
side, and at `nokori explain references` for the form of a link.

### What moves and what stays

The paragraphs in `flow` under "Writing down what somebody said" are about two
things. The command and the file it writes -- the first six paragraphs: the
invocation, `-m`/`-f`/standard input, the meaning of `<path>`, the file name
and why it has no slug, the receiving side, and the note that every other
command puts an item in this repository -- are the reporter's material, and
that is what the new topic carries. The rest of the section -- when a relation
is written, that a duplicate is noticed while looking, that an item standing
alone is ordinary -- is about items in general, and a developer creating any
kind needs it, so it stays in `flow`.

That split is why this is an entry page rather than a move. The topic does not
take over the section, and `flow` does not lose it: a developer reading `flow`
still finds the `feedback` command where it always was, and a reporter sent to
`explain feedback` finds the same command said from their side, with nothing
of the developer's own flow in it. `flow` keeps a one-line pointer so that a
developer reading it knows the other entry exists, and `kinds` keeps a one-line
pointer so that a reader who landed on the `feedback` kind knows where the
usage is.

### Naming

The topic is the noun `feedback`, next to `overview`, `kinds`, `flow`, and
`references`. A command's own flow could be named `how-to-<command>`, and that
is a rule worth deciding when a second such topic exists; with one, a name and
not a rule, `feedback` is the obvious one: it is the word a reader who wants
the flow would type, and it reads as the noun the list already holds.

## Alternatives

**A `feedback` section inside `kinds`.** Read next to the other kinds, it
answers the wrong question: the reader wants to send one, not to know how
kinds are counted. Keeping `kinds` about the kinds and giving the flow its own
topic keeps both readings clean, and the pointer in `kinds` covers the reader
who went there first.

**Name it `how-to-feedback`.** It says the same thing, but it is a rule --
`how-to-<command>` -- applied to the first instance of it, and there is no
second instance to tell whether the rule is right. `feedback` can be renamed
to that later if a second command's flow appears and the pattern holds; going
the other way is harder.

**Extend `flow` with a feedback section.** It mixes two readers into one page:
the developer who creates items and the person outside who sends a report.
Each would read past the other's material to find theirs, and the developer who
wants to hand off the flow has no short name to give. Moving the section wholesale
into a new topic has the same cost in reverse -- a developer reading `flow`
loses the command's usage.

**Do nothing.** The material is all in `flow` and `references` today. But the
use case is a pointer, and a pointer needs a short name; without one the
developer has to paraphrase the flow, which is what this is meant to replace.

## Open questions

- Settled while writing: the topic is an entry page, not a move. The
  reporter's material is in both, the developer's own flow stays in `flow`,
  and the two fit because the topic is deliberately short. A whole section
  moved here would leave `flow` describing a command it no longer mentions.
- Settled while writing: `flow` keeps the `feedback` material in
  "Writing down what somebody said" and gains a one-line pointer, rather than
  keeping a stub. The command's usage is not the developer's flow, so leaving
  it in `flow` costs the developer nothing.
- Settled while writing: the name is `feedback`, not `how-to-feedback`. The
  rule `how-to-<command>` waits for a second instance.

## Outcome

Implemented. `nokori explain feedback` prints `templates/explain_feedback.md`,
listed fifth in `TOPICS` and named in the `explain` help and the unknown-topic
error. It is the entry page the Open questions settled on: the reporter's
command, the words, the file, and that nothing is committed, said from the side
that sends it, opening with a line that says it is not the `feedback` kind.

`flow` keeps its "Writing down what somebody said" section and gains a one-line
pointer at the top; `kinds` gains a sentence at the end of the `feedback`
section. Neither paragraph moved, so the command's usage is in both places and
a developer reading `flow` loses nothing.

Two tests were added, `explain_prints_the_feedback_flow` and
`flow_still_mentions_feedback`, and the two that assert the topic list were
widened. The template was renamed from `how_to_feedback.md` to
`explain_feedback.md` to match the other topics' file names.
