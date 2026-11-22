# RFC: `close` says in its output that the link check passed

## Summary

This answers
[feedback-094641](../closed/20261122-feedback-094641.md).

`close` reads every link in the file before it moves it, and refuses the move
when one would break. When the move goes ahead, the run prints a line saying
the link check passed, before the `Closed <path>` line. Nothing about the
check or the move changes; what changes is that the run accounts for a puzzle
the writer was already solving by hand.

## Motivation

A writer who has just run `nokori check` and been told the file is written has
no way to know, from the `close` output, that the move also checks links. The
`check` command says whether the file is written; it does not say what `close`
will look at, and it cannot, because it deliberately leaves `## Outcome`
alone and so cannot see the file as it will be once closed. The two commands
ask different questions, and only one of them says so.

The result is duplicate work: the writer re-reads the links by hand after a
`check` that already passed, or opens the file again before the close, because
nothing in either output promised that a broken link would stop the move. The
reassurance the writer is looking for is a fact the `close` run already has --
it is the run deciding it -- and one line puts it where the writer is looking.

## Design

`run_close` prints one line before the existing `Closed <path>` line, in the
same shape as it:

```
links checked; the move is safe
Closed backlog/closed/20261122-rfc-close-says-link-check.md
```

Exact wording is settled while writing it, against the output of the other
commands; what is fixed here is that the line comes from the same run that did
the check, and that it sits above the move rather than below, since it is a
statement about what the run found before acting.

When the check fails, nothing is added: the existing error already names the
broken link and says what to write instead, and a line saying the check ran
would be the least of what the writer needs to read there.

## Alternatives

**Change `check` to say it.** `check` cannot see what `close` sees, since it
leaves `## Outcome` alone on purpose: a file with an empty outcome is one
`check` is written to pass and `close` is written to refuse. Making `check`
report on links as they will be after the close would either duplicate the
close's rule in a command that cannot decide it, or force the writer to
imagine the closed file. The run that does the check is the one that can say
it passed.

**Say nothing and document it.** `nokori explain flow` already says `close`
checks links. The writer who asks for reassurance while running the command is
not reading the explanation at that moment; the place to answer is the output,
and one line is the cost.

**Make the check a separate, visible step.** `close --check-only` or a `check
--links` flag would let the writer run the check twice and read it once. That
is more surface for the same reassurance, and it moves the check out of the
run that is about to act on it, which is where the mistake of a stale check
would come from.

## Open questions

- Does `close` have another refusal whose message would read oddly beside a
  "links checked" line? The TODO check and the empty-outcome check both run
  before the link check, so no line is printed when either fires.

## Outcome
