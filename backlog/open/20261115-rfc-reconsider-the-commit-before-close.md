# RFC: reconsider the commit `close` requires before it moves a file

## Summary

`close` refuses a file with uncommitted changes, so closing an item is two
commits: the one that writes the `## Outcome`, and the one that moves the
file. The proposal is to weigh whether that is worth its cost -- one refusal,
one extra commit, and a file that has to be brought to a clean state before it
can be closed -- against what it buys.

This item asks the question rather than answering it. If the answer is that
the check earns its place, the item closes with the reason written down, and
the next reader who notices the two commits does not have to work it out
again. If it does not, the check comes out and the `Outcome` is committed with
the move.

## Motivation

The rule is in `check_committed` and in the help text: the commit that moves
an item records only the move. What it costs is paid each time an item
closes, and it is more than one line.

Closing an item is always two commits. The first writes `## Outcome`, because
`close` refuses an empty one, and the second moves the file, because `close`
refuses a file with changes left. Neither can be folded into the other: the
first has to exist before `close` will run, and the second is made by the user
afterwards. There is no path through the rule that ends in one commit.

Each of the two has a refusal that exists only because of the rule. Writing
the `Outcome` and running `close` in the same breath is refused for being
uncommitted, so the writer runs `close` twice: once to be told, once after the
commit. A file left dirty for an unrelated reason -- a typo fixed while
reading it -- is refused for a change that has nothing to do with the close,
and the message names the file rather than the change.

What it buys is stated in one line, and the item is to find out whether that
line is true. If `nokori close` is run by mistake on a file whose edits are
not saved to git, the check is what keeps the mistake from taking the edits to
`backlog/closed/`. The question is whether that is a case worth a rule, since
the same mistake on any other file in the repository is not one the tooling
guards against.

## Design

Nothing is built until the question is answered, so the design is the answer
and the change it implies.

**If the check stays**, the item closes with the reason in `## Outcome`,
recorded where a reader who tries to fold the two commits will look. The
reason has to be about the case the item is for -- an unsaved edit carried
into `backlog/closed/` -- and not the general claim that a commit should do
one thing, which the repository applies to other commits as well and which
`close` is not the place to enforce.

**If the check goes**, `check_committed` and its call leave `close`, and the
`Outcome` is committed with the move. Two things have to change in the same
commit, or the removal leaves the tool saying something untrue:

- `nokori explain flow` loses the fourth refusal and the sentence that says
  the close needs the change committed, since it would then need nothing
  committed;
- the `## Outcome` comment in all four templates, which says `nokori close`
  needs the change committed, says instead that the section is committed with
  the move.

Either way `close` keeps its other three refusals. What is questioned is this
one check, not the shape of the command.

## Alternatives

**Keep the rule as it is, and write the cost off as worth paying.** That is
the default if nothing is decided, and it is what the item was written to
test: the reason the rule gives -- an edit carried into `backlog/closed/` by a
mistaken close -- is real, and the two commits are the price of it. It loses
if that price is the thing people notice about the tool, since a rule whose
reason nobody has read is the one worked around with `git commit --amend`.

**Have `close` make the commit itself, with a `--commit` flag.** One command
would move and commit, and the two commits would become one. It loses because
the tool would then write the commit, which no other command does, and
because the `Outcome` still has to be committed before the command may run,
so the flag saves the second `close` and not the first commit.

## Open questions

Whether the two commits can be avoided while keeping the check. Expected
answer: no, since `close` cannot see an `Outcome` before the user writes it.
A flag that made the commit itself is argued against under Alternatives.

Whether a file that is dirty for a reason unrelated to the close should be
refused. Expected answer: no, and the check should ask whether the file would
change in the commit that moves it, rather than whether it is dirty at all.
That is a narrower question than the one this item is about, and it can be
asked whichever way the item settles.

## Outcome

<!-- NOKORI_TODO: the one section written later, when `nokori close` moves
this file. It stays empty while the RFC is open, this comment and all, and
the comment is deleted in the same edit that writes the section -- which is
the close, since `close` refuses a file whose outcome is still empty.

The sections above are written before the item moves: by the time `nokori
close` moves this file, they are written, their comments are gone (this one
with them), and the change is committed.

Those sections may be rewritten as long as the item is open, as often as the
understanding changes. What they may not become is a description of work
already done. `nokori close` needs the change committed, so whatever they say
at that point is what the moving commit records.

Say whether the RFC was accepted or rejected, and why. When it is accepted,
name the change that implements it, or the item that does, if the work happens
somewhere else.

To refer to another item, link it:
[bug-empty-input](../closed/20261114-bug-empty-input.md). Link only an item
that is already closed: never link one that is still open, not even as
background. The rule is about this section and not the file: a closed item
may not cite an open one, while an open item may cite either, so a section
above this one may link an open item. Run `nokori explain references` for the
paths and the rule.

An accepted RFC can still be overturned later. Say so in the later item and
cite this one; do not edit this file afterwards to follow it.

Once this section is written and committed, treat the file as closed: do not
edit any part of it again, and append nothing later. If something turns up
later, leave the file as it is and open a new item that cites this one and
acts on it.

Delete this comment in the same edit that writes the section, and commit both
together before the move. -->
