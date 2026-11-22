This is the flow of a person outside the repository: somebody who met a
problem in this program and wants to report it to a backlog that is not
theirs. It is not the `feedback` kind -- that is in `nokori explain kinds`
-- but the stretch of flow that kind exists for, said from the side that
sends it.


Sending a report
================

    nokori feedback <path>

`<path>` is the directory holding the backlog the report belongs to: `.`
when you stand in the repository that owns it, the root of another one when
the backlog is somewhere else. That is what separates this from every other
command here -- the person who met the problem is often not in the repository
the report belongs to, and the run happens where the words come from rather
than where the item lives.

The words come the way `git commit` takes a message: `-m` for text on the
command line, `-f` for a file, and standard input when neither is given.


What the run writes
===================

The file is `backlog/open/YYYYMMDD-feedback-<HHMMSS>.md`. The name is the day
and the second it was written, with no slug: a slug is a summary, and a
summary is made by reading the words, which this run has not done. There is no
title line either, for the same reason. What the run knows is that somebody
said something, and when.

Nothing is committed by the run, in either repository.


What happens after it lands
===========================

The rest of the flow happens in the repository it landed in. That repository
is the one that reads the words, decides what they turn into, and closes the
feedback with the decision in `## Outcome`. Run `nokori explain flow` for
what happens on that side.

Looking for an earlier report is optional here, unlike the other kinds: this
is what you ran into, and a second report of the same thing is not damage.
A link you happen to know is worth writing, and `nokori explain references`
has the form; no search is owed.
