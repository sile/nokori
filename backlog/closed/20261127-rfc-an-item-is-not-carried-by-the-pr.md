# RFC: An item is not carried by the pull request

## Summary

`nokori explain flow` gains a paragraph saying that the item is committed on
its own, apart from the change it proposes, and that an accepted RFC is
closed on the default branch after the pull request implementing it has
merged. Nothing in the tool changes.

The rule already exists, one directory away: the `Summary` comment in `rfc`
says never to mention `backlog/` in a pull request. What is missing is the
consequence -- the natural reading of "never mention it" is about the text of
the request, and a file added by the same request is not text. A backlog is
the developer's record and stays out of the published part; a pull request is
one of the places the published part is.

## Motivation

A backlog is the developer's own record. It says what is being considered,
what was turned down and why, and who said what, and none of that is written
for a reader outside the work. Keeping it out of the published part is the
point.

The same directory is not an argument against that. `backlog/` often sits
next to `src/` because it is convenient while the work is going on, not
because it belongs with the code, and the two may be split later. So a
directory that happens to be under the same root as the code is one place the
record is kept, not a part of the code.

Where the two are apart, the reason is plain: the backlog may be private
while the code is not, so a mention of it in the code is a mention in public,
and somebody reading the code may not be able to open what it points at.

Where the two are together, the pull request makes the same case. A review
reads the change and the change only. An item carries what the work came
from, which the person doing the work needs and the reviewer does not, and
the two are read differently and at different times. Put in the same diff,
the item is noise in the review and the code is noise in the record.

The flow points away from this, which is why it needs saying. An accepted RFC
stays open until its change lands, so an RFC and its implementation are
worked on together, on one branch, one sitting after the other. What the flow
does not say is where the line falls, and a writer doing both will put both
in the same place unless told otherwise. That is what happened here: two
branches in a row carried the item and the change together, one pull request
took in three RFCs of which one was implemented, and it had to be rebuilt
from a single commit.

## Design

The `Closing` section of `nokori explain flow` gains a paragraph after the
one about an accepted RFC. The paragraph says three things, in this order:

- the item is committed apart from the work it proposes, because a pull
  request is read for what it changes and the item is not part of that;
- an accepted RFC is closed on the default branch, after the change that
  implements it has merged, so that the `## Outcome` names a commit that
  exists there;
- the same holds for the other kinds -- the item goes in on its own, and a
  `plan` that was carried out while open is closed once the steps are done.

The paragraph is prose and not a rule `close` enforces, for the same reason
the paragraph before it is: `close` reads the file and cannot see a branch.

## Alternatives

Leave it out and say the flow says enough. It does not: the RFC staying open
until its change lands is the reason the two end up together, and reading it
as permission to put both on one branch is the mistake, not an unusual one.

Ban it only where the backlog is a separate repository. There, the reason is
least avoidable, so the rule looks like it belongs to that arrangement. But a
backlog next to the code is one that is kept there for the work's sake, not
one that the code may reach into, and the review argues against carrying it
in either case. A rule that is the same wherever the directory sits is the one
a writer can hold in mind.

Add the sentence to the `Summary` comment of the `rfc` template only. That is
where the rule against `backlog/` in a pull request already sits, so it looks
like the right home. But the same mistake is available to a `plan` and to a
`bug` -- any item whose change arrives as a pull request -- and a per-kind
comment repeats the same paragraph in four files. `flow` is where the shape
of the commands already lives, and this is a statement about when a command
is run.

Have `create` check the branch, or `close` refuse a file with a newer commit
on the branch. Neither is possible without reading git, which no command
does, and the second would refuse the ordinary case of closing an item the
same sitting it was written.

## Open questions

None.

## Outcome

Accepted. The tool does not change; `nokori explain flow` gains the paragraph.

**What was hard was telling two rules apart.** The obvious sentence -- an
accepted RFC is closed on the default branch -- reads as an instruction about
where the item is kept, and that is not a rule anybody can follow or check:
`close` reads a file and never a branch. What the flow can say is where the
commit goes: the item and the change are committed apart, sharing a commit is
the concession a project with no branches makes, and closing comes after the
merge because the `## Outcome` names a commit that has to exist. Which branch
the item is kept on is the project's choice.

**It stays prose.** Nothing inspects a branch or a diff.

**Done.** The paragraph is in `flow`, and the sections of `bug`, `feedback`,
and `rfc` lost a sentence that was only true of an RFC. `plan` is out of the
paragraph altogether: a plan is checked off where the work happens, which
rules out keeping it off the branch, and two repositories make that impossible
outright -- how a plan is carried is its own item. `overview` now grounds the
rule in what a backlog is: the work's own record, next to the code because
that is convenient while the work goes on -- so a backlog in a repository of
its own is the same case and not an exception.
