# RFC: Add an `overview` topic to `explain`

## Summary

`nokori explain overview` prints what the tool is for and what it does not do:
the backlog is plain files, the rules are conventions it cannot enforce, and
what it offers is one way to keep a backlog rather than a mechanism. The other
topics say what to do; this one says what the thing is, so that a reader who
started from `nokori --help` meets the premise before the instructions.

## Motivation

The premise of the tool is written in exactly one place: the `What this tool
is` section of `README.md`. That file is a crates.io page and a GitHub page.
It is not part of the binary, so a reader who installed the crate and ran
`nokori --help` has not seen it, and `cargo install nokori` does not leave it
anywhere on disk.

What the three topics say presumes that premise. `kinds` says a `bug` is
closed when it is settled rather than when it is fixed; `flow` says a file in
`closed/` is never edited; `references` says a link that was already broken
stays broken. A reader who meets those rules without the premise reads them
as arbitrary, because the premise is what makes them follow: nothing is
enforced, so the rules are only worth stating if they are worth following.

The gap is not one of discovery, either. Nothing in `nokori --help`, in the
output of `init`, or in an unknown-topic error mentions `README.md`, so the
reader has no reason to go looking for the file that would explain what they
are following.

## Design

A new file, `templates/explain/overview.md`, is included with `include_str!`
and listed in `TOPICS` as `overview`. `nokori explain` with no topic prints
the list of topics; `overview` goes first in that list, since it is the one to
read first.

The text has three parts.

**What the backlog is.** Ordinary directories and ordinary markdown files,
little more than `backlog/`, `backlog/open/`, and `backlog/closed/`. No index,
no hidden file, nothing that can only be read through this tool. Anything can
be edited, moved, or deleted by hand, and doing so is not working around
`nokori` but using the same files it uses.

**What the tool is for.** It is not there to enforce the flow, and it cannot:
there is hardly any check that could fail. What it offers is one way to keep a
backlog, written down as a flow of commands, and the rules are
recommendations that the user follows on their own responsibility -- the same
deal as git-flow. It exists as a program rather than as a page of prose so
that the flow can be walked without knowing it in advance: every command says
what to do next, and the shape is learned by running them in order.

**What that asks of the reader.** Because nothing is enforced, a rule that
looks arbitrary is worth following only if the reason holds for this backlog.
`kinds`, `flow`, and `references` give those reasons, and each is about a
decision that has a cost as well as a benefit: two states rather than four, a
closed item rather than an edited one, a link rather than a name.

`flow` gives up its `What an item is` section to `overview`. That section says
an item is a record rather than a specification, and that the source never
points back at the backlog -- both are properties of what the tool is, and
leaving them in `flow` would put the premise after the first instruction that
depends on it. `flow` keeps the two mistakes it ends with, since those are
about a command rather than about the tool.

## Alternatives

**Point `--help` at `README.md`.** A line saying where the premise is written
would be shorter than a new topic. It loses because the path is right for
nobody in particular: this repository's `README.md` is not what the reader
has. The published crate ships one, but it is in the source distribution only,
so where it sits depends on how the crate was installed and it may be nowhere
at all. `explain` travels in the binary, so it is right for every reader
without a path to get wrong.

**Say the premise in each command's `--help`.** Then it arrives where the
reader already is instead of being asked for. It loses because the text is
the same for all of them, and copied into four places it will drift in three.
It would also be the longest part of `create --help` and `close --help`,
pushing what those flags do below something that is answered once.

**Fold it into `flow` instead of adding a topic.** `flow` is what a reader
runs next anyway, so nothing new has to be learned to find it. It loses
because `flow` is about one item's life, and a section on what the tool does
not enforce does not follow from a command. A reader who comes back to `flow`
for the signature of `close` would step past it every time. It would also
leave `What an item is` where it is, which is the section that belongs beside
the premise rather than after the first instruction.

## Open questions

Whether `README.md` keeps its `What this tool is` section once `explain
overview` exists. The two would say the same thing, and a README that repeats
the binary costs a reader the job of keeping them in step. It is not settled
here because the README is the page crates.io shows and `explain` is not, so
the section does a job the topic cannot; what is open is whether that is
enough to keep both.

Whether the unknown-topic error should still point at `nokori --help` now that
`overview` is first in the list. Expected answer: yes. That error covers the
mistake of taking `explain` for usage help, and a reader who wanted a flag is
not helped by the premise either.

## Outcome

Accepted, and implemented in the commit `3870e14` ("Add an overview topic, and
move what an item is into it").

The text has the three parts the design asked for, and `flow` gave up its
`What an item is` section, which is now under the same heading in `overview`
and is the reason a reader meets the premise before the first instruction
that depends on it.

Both open questions were settled during the work, and neither went as the
item expected.

**Whether `README.md` keeps its `What this tool is` section.** It keeps it.
The section is what crates.io shows and `explain` is not, so the two reach
different readers and the duplication buys something the topic cannot. The
worry that they drift is real but small: the topic carries the flow and the
reasons, which change, while the README section says only what the tool is
not, which does not.

**Whether the unknown-topic error still points at `nokori --help`.** It does,
as expected, so that half of the question was answered as the item predicted.
What the work turned up is a different gap the item did not ask about:
`nokori explain` with no topic refused for a missing argument, so the list the
design says it prints did not exist. It prints the list now, `overview`
first, since a reader who knows no topic name is exactly the one who needs to
be shown them.
