# RFC: never cite an open item from a closed one

## Summary

An item that is closed may cite another item that is closed, but never one
that is still open. The allowed directions are closed-to-closed,
open-to-closed, and open-to-open; closed-to-open is the one to avoid.

What this buys is that a closed item's references never rot. A closed file
is not edited again, so a reference it holds is permanent, and a reference
into `backlog/` is a reference to something that can still be renamed or
deleted.

## Motivation

The `Outcome` comment says to name the change that carried the item out, "or
the item that does, if the work happens somewhere else". Following that
sentence, a `feedback` item closed today could name the `rfc` that answers
it -- and that `rfc` is a file in `backlog/` whose slug is not fixed.

This is not hypothetical. `20261114-feedback-rename-plan-items-section.md`
was closed naming a proposed `rfc`; by the time the `rfc` was written, what it
proposed had changed -- the section was renamed first and then dropped -- and
the name the closed item had used was never the name of any file. The closed
item cannot be corrected, because a closed item is not edited. The reference
is there, it looks like a reference, and it points at nothing.

Naming an open item is also a claim about what that item will become. A
proposal can be rejected, reshaped, or renamed after somebody reads it; a
closed item that says "answered by `rfc-split-the-lexer`" has committed to an
outcome it cannot see, in a file it will not revisit.

The other three directions do not have this problem, which is why the rule
is a direction and not a ban on citing items altogether. Open-to-closed cites
something finished and fixed. Open-to-open cites something the writer can
still fix, and should, if it moves. Closed-to-closed cites something that is
as permanent as the citing file.

## Design

One rule, in the comment under `Outcome` in all four templates:

> Name the change that carried this out, or the item that did if the work
> happened somewhere else -- but only if that item is already closed. Never
> cite an item that is still open.

The rule applies to references that carry information about the outcome: the
one line in `Outcome` that says where the work went. It is not a rule about
prose. A closed item may still say "this is what
`20261114-rfc-how-items-refer-to-each-other.md` proposed" as background, as
long as the sentence does not stand as the answer to "what happened".

When the work is not finished, there is nothing to cite: the item stays in
`backlog/` with its outcome unwritten, and the reference is written by
whoever closes the item that finished the work.

Where the rule appears is the place a writer will be when breaking it. The
`Outcome` comment is read at `close` time, which is the moment the file
becomes permanent; the `Sections:` line in `explain kinds` is not.

## Alternatives

**Ban citing items in `Outcome` altogether.** Simplest to state and to check,
and it loses the case the sentence exists for: two items close together when
one carries out the other, and a reader of the later one wants the earlier.
Closed-to-closed is the case with no downside, so banning it costs more than
the rule it replaces.

**Allow it and add a `check` command that reports dangling references.**
This keeps every direction writable and fixes the ones that actually break.
It only works if the check runs, which means a hook or a CI step in every
repository that uses nokori, and the reference it reports has no fix: the
file that holds it may not be edited. A check is the right tool for a
reference that can be corrected, and this is the one that cannot.

**Say it once in `backlog/README.md` instead of in every template.** One
place to change, and it is the file a reader of the directory opens first.
It is not the file a writer has open at `close` time, though, and `README.md`
is prose about the directory rather than a comment inside the kind being
written.

## Open questions

Should `close` refuse a closed-to-open reference when it can see one, now
that the rule exists? `close` would have to tell an item reference from any
other backticked string, which is a guess about the naming convention rather
than something the file format guarantees -- and the rule is about the
sentence's meaning, which `close` cannot read. A wrong refusal at `close`
time costs more than a wrong reference.

Does the same rule belong on the `Motivation` and `Notes` sections, where a
closed item might describe what an open item proposes? Left out of the design
above; background prose still names items, and the line between "background"
and "the answer" is what a reader has to judge anyway.

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

An accepted RFC can still be overturned later. Say so in the later item and
cite this one; do not edit this file afterwards to follow it.

Once this section is written and committed, treat the file as closed: do not
edit any part of it again, and append nothing later. If something turns up
later, leave the file as it is and open a new item that cites this one and
acts on it.

Delete this comment when you write the section. -->
