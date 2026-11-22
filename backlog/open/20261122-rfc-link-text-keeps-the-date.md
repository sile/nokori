# RFC: link text keeps the date

## Summary

A link to an item is written with the item's whole file name as its text,
date included: `[20261122-bug-empty-input](20261122-bug-empty-input.md)`
rather than `[bug-empty-input](20261122-bug-empty-input.md)`. The same text
is what `close` writes where a link has to become a name, so the name in a
closed file and the link in an open one read alike.

## Motivation

A link's text is the kind and the slug, and for three of the four kinds that
is enough: `rfc-cache-the-parse` names the item, and a reader who wants the
file has the date in the path. There is one kind where the text names
nothing: `feedback` has no slug, and its file is named by the second it
arrived, so the text of a link to one is `feedback-235217`. That is a time
and not a subject. A reader who meets it inline has no way to tell what the
item is about without following it, and two feedbacks from the same minute
are told apart only by following each.

There is a second cost, smaller and paid on every kind: the text drops the
date, which is the one part of the name that says when the thing happened.
Two items of the same kind and slug cannot both exist, so the date is not
needed to tell them apart -- but a reader deciding whether a link is still
current is asking when, and the answer is in the path and not in the text.

Dates in the text are also what the names in a closed file would carry, if
the same text is used there: `close` turns a link into a name by taking the
text, so what a closed file says and what the open one said would agree
exactly, and a reader who copies one into the other does not have to decide
which form is right.

## Design

**The text of a link to an item is its file name without `.md`.**
`[20261122-rfc-cache-the-parse](20261122-rfc-cache-the-parse.md)`,
`[20261122-feedback-235217](20261122-feedback-235217.md)`. The label
`nokori explain references` describes is the same string, so the rule has one
statement and not two.

**The name in a closed file is that same text.** `close` writes a name where
a link into `backlog/open/` has to go, and the name it writes is the text of
the link it replaces. Since the text keeps the date, the name does too, and
the two forms -- link and name -- are the same characters with and without
the target.

**`nokori tree` prints the same label**, since it prints the text a link to
the item carries, and that text now has the date in it.

**What a link's text has to be is unchanged.** Nothing checks the text of a
link: `close` looks for `](` and for a target that resolves, and a reader who
writes a name without the date is not stopped. The rule is a form to write,
as it is today, and the date is a part of the form.

## Alternatives

**Keep the kind and slug, and add a slug to `feedback`.** Then every kind has
a label that says what the item is about, and the date stays out of the text.
It loses because a slug is a summary, and a summary is made by reading the
words: the run that writes a feedback has not read them, and a person
reporting a bug should not be asked to summarize it before it is filed. The
date and time are what the run knows.

**Put the date in front of the text rather than in it.**
`[20261122 bug-empty-input](...)`. It loses because the label is then a
string no file has: a reader who copies it into a name writes something that
is not the item's name, and `close`, which writes the text as the name, would
write a name that does not read as one.

**Leave the text alone and have `tree` print the date.** The command that
lists the relations could print the whole file name while links keep the
kind and slug. It loses because the two would then disagree: the reader of a
tree is looking for the item a link in a file points at, and two spellings of
one item is a thing to be matched by eye in every case.

**Make the date optional by kind** -- `feedback` keeps it and the rest do
not. It loses because the rule would have one exception and the exception is
the one case a reader has to remember. A rule with no exceptions is what the
text is for.

## Open questions

Whether a link to a closed item is written with the same date as an open one.
Yes: the text is the file name, and a closed file keeps the name it had. The
directory in the target is what says where the file is, and it is already in
the link.

Whether the date is the full `YYYYMMDD`, or shortened since it is repeated.
Full: it is the name, and a name that is shortened is a name that has to be
read as a rule rather than as a file.

## Outcome

