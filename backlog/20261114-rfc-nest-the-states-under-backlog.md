# RFC: Nest both states under `backlog/`

## Summary

An open item lives in `backlog/open/` and a closed one in `backlog/closed/`,
instead of the two being `backlog/` and `closed/` at the root of the
repository. Both states are then one directory, `backlog/`, which can be
moved as a whole, and neither is a neighbour of the source tree.

Because the two directories are at the same depth, a relative link from one
to the other keeps its meaning when an item moves: `../closed/x.md` read from
`backlog/open/` names the same file as `../closed/x.md` read from
`backlog/closed/`. That removes the reason `close` had to rewrite link
targets on the way, and the reason there was a root-relative kind of target.
`close` then only checks that the links still resolve, which it can do for
any link in the file rather than only for the ones that point at an item.

## Motivation

`close` rewrites link targets. It does so because a target that is right in
`backlog/` can be wrong in `closed/`: a file in `backlog/` that writes
`closed/x.md` is right where it stands, and the same text read from `closed/`
means `closed/closed/x.md`. The rewrite exists to paper over that, and it
went wrong the first time it ran, leaving broken links in the files it had
just moved.

That kind of repair is what the tool should not be doing at all. Moving a
file is a thing a reader can check by looking at the diff; a rewrite is a
second change hiding in the same commit, and it is the change nobody looks
at. It also forced a rule into `explain references` -- a target beginning
with `backlog/` or `closed/` is read from the root -- that exists only to
make the rewrite predictable, and that a reader has to hold in mind while
writing every link.

There is also a cost to keeping the states apart. `backlog/` and `closed/`
sit at the root of the repository, next to `src/`. A repository that keeps
its backlog at the root has to allow for two directories, and one of them,
`closed/`, is created late and is easy to forget in a packaging list or a
`.gitignore` written before it existed. One directory that can be moved as a
whole is simpler to describe.

## Design

**The directories.** `backlog/` holds two directories, `open/` and `closed/`,
and one file, `README.md`. An item is created in `backlog/open/` and closed by
moving it to `backlog/closed/`. `init` writes all three, so that neither state
is a directory that appears later than the others.

**The READMEs.** Each of the three is short: what the directory holds, and
which `nokori explain` topic says the rest. They say nothing about how nokori
works beyond that, because a file written once by `init` cannot follow a
change of policy. Whatever advice used to sit in `backlog/README.md` -- that
an item is a record rather than a specification, that the source never points
back at the backlog -- moves to `nokori explain flow`, which is shipped inside
the binary and is therefore always the current text.

**The commands.** The two commands keep their names and their arguments; only
the paths they write change:

| command | writes |
| ------- | ------ |
| `create <kind> <slug>` | `backlog/open/YYYYMMDD-<kind>-<slug>.md` |
| `close <file>` | `backlog/closed/<file>` |

**The link rule.** A target is a path relative to the file holding the link,
and nothing else. The rule that a target beginning with `backlog/` or
`closed/` is read from the root goes away with the rewrite it existed for.

| referring item | target item | target |
| -------------- | ----------- | ------ |
| `backlog/open/` | `backlog/open/` | `20261114-bug-y.md` |
| `backlog/open/` | `backlog/closed/` | `../closed/20261114-bug-y.md` |
| `backlog/closed/` | `backlog/closed/` | `20261114-bug-y.md` |
| `backlog/closed/` | `backlog/open/` | not allowed |

**What `close` checks.** Before moving a file, `close` resolves every link in
it twice: once from `backlog/open/`, where the file is, and once from
`backlog/closed/`, where it is going. If a target names a different file from
the two places, or does not exist from the second, `close` refuses the move
and says which line is wrong. It changes nothing in the file.

A link that is not a path -- one with a scheme, such as `https://` or
`mailto:`, and one that begins with `#` -- is skipped. So is an empty target.
Everything else is treated as a path, whether or not it points at an item:
with no rewrite there is nothing to gain from telling the two apart, and a
check that covers every path in the file is a check a reader can rely on
without remembering its scope.

Given the two directories are at the same depth, the comparison comes out
equal for every relative link that names something outside `backlog/open/`
and `backlog/closed/`. What it catches is a bare filename: `bug-y.md`
resolves inside `backlog/open/` now and inside `backlog/closed/` after the
move, which is the mistake the rule about not citing an open item already
forbids. The check states it in the one way a machine can see, so that a
link the writer forgot to change is refused rather than discovered later.

## Alternatives

**Keep `backlog/` and `closed/` at the root, and fix the rewrite.** This is
the smallest change: the bug that wrote broken links into `closed/` was a
resolved-from-the-wrong-place mistake, and it has been fixed. The rewrite
would then stay, along with the root-relative kind of target it needs, and
so would the rule a reader has to remember while writing a link. The rewrite
is also the part of `close` that is hardest to get right for no visible gain:
the file is moved, and something else in it changed, and the reader of the
diff has to decide whether that second change is right.

**`closed/` under `backlog/`, but keep the root-relative targets.** A target
that begins with `backlog/` or `closed/` could still be honoured by reading
it from the repository root. That keeps a rewrite in `close`, since a file
in `backlog/open/` writing `closed/x.md` means `backlog/closed/x.md`, which
is not what the same text means from `backlog/closed/`. The rewrite and the
special kind of target stand or fall together, and both are only there
because the two states used to sit at depths that differed from the
repository root.

**A state field instead of a directory.** Already ruled out: a field can
disagree with where the file is, and the directory cannot.

## Open questions

Does `backlog/open/` make `ls backlog/` confusing? No: it lists two
directories and a README, which is what the directory now holds. The item
that used to be the first thing `ls backlog/` showed is one level down, where
`ls backlog/open/` shows it.

Should `init` write `backlog/closed/README.md` for a repository that never
closes anything? Yes. The alternative is a directory that appears on its
first use, which leaves `init` producing two of the three directories and a
first-time reader unable to tell which one is missing.

What happens to the six items already in `closed/`? They move to
`backlog/closed/` without their contents being changed, as the move is made
by hand rather than by a command. The links in them were written for the
depth they were at, so three of them now point one level too deep. They are
left that way: the past is not rewritten, and what produced them is in git.

## Outcome

<!-- NOKORI_TODO: the one section written later, when `nokori close` moves
this file. Leave it empty until then, comment and all.

The sections above are written before the item moves: by the time `nokori
close` moves this file, they are written, their comments are gone, and the
change is committed.

Those sections may be rewritten as long as the item is open, as often as the
understanding changes. What they may not become is a description of work
already done. `nokori close` needs the change committed, so whatever they say
at that point is what the moving commit records.

Say whether the RFC was accepted or rejected, and why. When it is accepted,
name the change that implements it, or the item that does, if the work happens
somewhere else.

To refer to another item, link it:
[bug-empty-input](closed/20261114-bug-empty-input.md). Link only an item
that is already closed: never link one that is still open, not even as
background. Run `nokori explain references` for the paths and the rule.

An accepted RFC can still be overturned later. Say so in the later item and
cite this one; do not edit this file afterwards to follow it.

Once this section is written and committed, treat the file as closed: do not
edit any part of it again, and append nothing later. If something turns up
later, leave the file as it is and open a new item that cites this one and
acts on it.

Delete this comment when you write the section. -->
