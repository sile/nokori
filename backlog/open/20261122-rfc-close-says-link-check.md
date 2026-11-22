# RFC: `close` says in its output that the link check passed

## Summary

This answers
[feedback-094641](../closed/20261122-feedback-094641.md).

`close` reads every link in the file before it moves it, and refuses the move
when one would break. When the move goes ahead, the run prints a line saying
the link check passed, before the `Closed <path>` line. The run also names the
open items that link the one being closed, since those links are broken by the
move and the run already has the list.

Neither half blocks the move. The check refuses a link that is wrong in the
file being closed, because that file is not edited afterwards; a link held by
another open item is that item's to repair, so the print names the items and
the move goes ahead.

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

The check in the file being closed is only half of what the move does to
links. A link from an open item to the one being closed is written as
`20261122-foo.md` while the target is a neighbour in `backlog/open/`, and the
move makes it point at nothing: the target is now in `backlog/closed/`, and
the link needed `../closed/20261122-foo.md`. That link is not in the file
being closed, so `close` neither refuses it nor says anything about it today.

The writer is the one who would notice, and the run is the one that can tell
them: it walks the open items to know what is there, and the same walk answers
which of them link the file it is closing. Refusing the move would be wrong
-- an open item citing an open one is allowed, and the writer of that other
item may be in another sitting -- but staying silent sends them to find the
broken link by running `check` on every open item, which is the scanning work
the print would have done for them.

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

### The open items that link this one

After the `Closed <path>` line, the run names every open item whose links
resolve to the file that just moved. The paths are printed, one per line, the
way `close` prints paths elsewhere:

```
Closed backlog/closed/20261122-rfc-close-says-link-check.md

Two open items link it and would break. Update them:
  backlog/open/20261122-rfc-title-one.md
  backlog/open/20261122-rfc-title-two.md

Commit the move, and leave the file as it is from then on:
...
```

When no open item links it, nothing is added and the output is what it is
today. The move happens either way: the message is a report, not a refusal,
and the run does not wait for the links to be fixed.

The list is the one the graph already holds. The core keeps the decision and
the report as one value; the edge gathers the items -- it is already gathering
them for `tree` -- and hands the ones that link the moved file to the report.
What the core does with them stays pure: whether an item links a path is a
question about the items the edge passed in, not a question that reads a
directory.

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

**Refuse the move when an open item links it.** The move is not wrong, and
neither is the link: an open item may cite an open one, and the two are only
wrong together once the target has moved. Refusing would also make one
writer's close depend on another writer's edits, which is the coupling the
"one link holds the relation" rule exists to avoid.

**Leave it to `nokori tree`, or to running `check` on every open item.** That
is what a writer who suspects it does today. The list is the scanning work,
and the run has it for free; printing it removes the step rather than
renaming it.

**Make the check a separate, visible step.** `close --check-only` or a `check
--links` flag would let the writer run the check twice and read it once. That
is more surface for the same reassurance, and it moves the check out of the
run that is about to act on it, which is where the mistake of a stale check
would come from.

## Open questions

- Does `close` have another refusal whose message would read oddly beside a
  "links checked" line? The TODO check and the empty-outcome check both run
  before the link check, so no line is printed when either fires.
- Should the list include an open item that links back only through the name
  form in a code span? That is not a link, so it is not in the graph and not
  in the list; the item holds no link to break. Leaning to no.
- Does the walk pay off when the backlog is large? `tree` already walks every
  item on every run, so the cost is the one the tool already accepts; the
  question is whether `close` should share the walk's function or keep its
  own. Settled while writing.

## Outcome
