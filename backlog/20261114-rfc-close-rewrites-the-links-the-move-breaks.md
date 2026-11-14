# RFC: `close` rewrites the links that the move would break

## Summary

`close` writes the link targets in the file it moves, so that each one
resolves from `closed/` instead of from `backlog/`. A link into `backlog/` is
still refused, and so is one that resolves nowhere: the first breaks the rule
that a closed item never cites an open one, and no rewrite can mend the
second. Every target `close` changes is printed, so that the commit records
why the file changed and not only that it moved.

## Motivation

A link is written from where the file is, so the same text means a different
file once the file moves. In `backlog/`,
[rfc-never-cite-an-open-item-from-a-closed-one](closed/20261114-rfc-never-cite-an-open-item-from-a-closed-one.md)
is the right target; in `closed/`, that text reads `closed/closed/20261114-rfc-never-cite-an-open-item-from-a-closed-one.md`,
which is nothing. The writer cannot be asked to get this right, because the
right answer changes at a moment the writer does not choose.

`close` decides that moment. It moves the file, and it checks the links
before moving; so a file whose links are checked from `backlog/` passes and is
then broken by the move `close` itself makes. That is not a hypothetical. The
three RFCs that introduced links were closed this way, and the first of them
to be read afterwards was pointing at a file that does not exist:
[rfc-how-items-refer-to-each-other](closed/20261114-rfc-how-items-refer-to-each-other.md)
still reads `closed/20261114-rfc-never-cite-an-open-item-from-a-closed-one.md`
from inside `closed/`. The section on references says that a broken link never
reaches `closed/`, and `close`, which is what makes that true, put it there.

The cost is not only the three files. A repository that follows the rules is
the only evidence that the rules can be followed, and a `closed/` that does
not resolve is that evidence failing. A reader who finds a broken link in the
record has to wonder which of the other rules are also held only in the
documentation.

## Design

`close` reads every link in the file from `closed/`, the directory the file
is going to, rather than from `backlog/`, the one it is in. A link whose
target no longer means the same file from there is written again as a path
relative to `closed/`, and the line is left otherwise as it was.

The rule for a new target is one line: the path of the file the link named,
seen from `closed/`, with the `closed/` it starts with dropped. A link to
[bug-x](closed/20261114-bug-x.md) becomes `20261114-bug-x.md`, because both
files then sit in the same directory. A link to `../closed/20261114-bug-x.md`
becomes the same thing: how the link was spelled does not decide what it
becomes, only what it names. The text of the link is not touched, so it keeps
the kind and slug it was written with.

Whether a target names an item is decided by the table in `nokori explain
references`: a target that starts with `backlog/` or `closed/` is read from the
repository root, and anything else from the directory the link sits in. Both
readings are needed because both are what the table asks the writer to type, so
the check has to read a target the same way the table does rather than pick
one reading and be wrong about the other.

Links that are not to an item are left alone. An external URL, a link to a
file outside `backlog/` and `closed/`, a link that was already broken before
the item was written: none of them is this command's business, and a rewrite
that reached them would be guessing at what the writer meant.

Two things still stop the close, and neither is rewritable. A link into
`backlog/` breaks the rule that a closed item never cites an open one, and
whether that link should become a citation or be removed is a question about
what the item says, not about where the file is. A link that resolves nowhere
has no file to point at; rewriting it would mean picking one.

Each changed target is printed to stdout, under the line that says what was
closed, with the line it is on and what it became. The move and the rewrites
are one commit whose message can say both, and the writer does not see the
change happen: a file that changed on its way to `closed/` should not surprise
whoever reads the diff.

Nothing else in the file changes. `close` writes only the link targets it
computed, not a reformatted copy of the file, so whitespace, line endings and
every line without a rewritten link stay byte for byte what they were.

A file that is already in `closed/` is never visited. `close` reads one file,
the one it moves, and the move is the only moment a swap is made; a broken link
that is already in `closed/` stays there. That is the general rule and not an
accident: a backlog states what was true at the time, and a later state is a
new item citing the old one, so sweeping the record every time a rule changes
is work that grows with the record and has no end. The three files this
proposal was written about are left as they are, and a repository that reads
them sees what those commits produced.

## Alternatives

**Targets from the repository root.** Write every target as a path from the
root, such as `closed/20261114-bug-x.md` or `backlog/20261114-rfc-x.md`. A
target like that means the same file from anywhere, so no move can break it
and `close` would have nothing to rewrite.

This is the one that looks obviously right, and it is the one to answer for.
It is not a link. Markdown resolves a target as a relative path, and the
tools that render it -- the web view of a repository, an editor's preview, the
terminal -- treat a target that starts with `/` or with a directory as relative
to where the file is. `closed/20261114-bug-x.md`, written in a file that is
itself in `closed/`, is read by all of them as `closed/closed/20261114-bug-x.md`.
The fix would be a target that nothing opens, chosen so that a checker agrees
with it; a reference nobody can follow is not worth checking.

**Refuse a link from an open item to a closed one.** Have `close` read the
links from `backlog/` as it does now and stop when one points into `closed/`,
so that no file enters `closed/` with a target the move will break.

This refuses the direction the references RFC allowed on purpose, and it does
not work even on its own terms. The item is checked before it moves, so the
writer would be told to change a link that is right where the writer is
standing, and told to change it to a form that does not resolve yet. There is
no moment at which the file is both correct and accepted.

**Have the writer fix the links after the move.** Move the file as it is,
print the links that no longer resolve, and let the writer run `close` again
or edit the file in `closed/`.

This asks the writer to do by hand what the computer just computed, and it
contradicts the rule that closes the item: a file in `closed/` is not edited
again. The one moment a closed item may be written is the move itself, which
is why the rewrite belongs there and not after it.

## Open questions

None. The rewrite is reported on stdout, below the line that says what was
closed. Read back, the two lines are one report -- what moved, and what had to
be respelled for it to move -- so they belong on the same stream. A warning on
stderr would say the file is in doubt, and it is not: the move ends with every
link working.

Writing this found a second fault, older than the one it was written about.
`close` resolved every target from `backlog/`, so it read `closed/x.md` as
`backlog/closed/x.md`, which is not an item, and passed the link without
looking at it. The link from an open item to a closed one was never checked:
the direction the references RFC introduced was the one nobody looked at. The
table now says which targets are read from the root -- those that start with
`backlog/` or `closed/` -- and the check reads them that way. Nothing in
`closed/` depended on the old reading, since a file there names its neighbours
by bare filename.

## Outcome

Accepted and implemented in `0a10bb3`. `close` reads each link from `closed/`,
rewrites the targets the move would change, and prints them under the line
that says what was closed.

Writing it found what the RFC was written about and could not see: the check
resolved every target from `backlog/`, so `closed/x.md` read as
`backlog/closed/x.md`, which is not an item, and was passed without being
looked at. The link from an open item to a closed one was the direction nobody
checked. Targets that start with `backlog/` or `closed/` are now read from the
root, and the table in `nokori explain references` says so.

The three files that prompted this are left as they are, with their broken
links: a closed item is not edited, so a link that was wrong when it landed is
part of the record of what produced it. That rule is now stated for everything
nokori keeps, in `README.md`, and not only for `closed/`.
