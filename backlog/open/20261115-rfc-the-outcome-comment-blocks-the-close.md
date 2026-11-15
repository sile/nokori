# RFC: Stop the `Outcome` comment from blocking the close

## Summary

The `Outcome` comment in every template says to keep it "until then, comment
and all", and `close` refuses a file that has any `NOKORI_TODO:` left. A file
that follows the template exactly therefore cannot be closed. The change is to
say in the comment when it goes: it stays while the item is open, and it is
gone by the time the file closes, because `close` is the step where the
section is written and the comment is deleted with it.

## Motivation

The two instructions are each clear, and together they cannot both be
followed. The template says of `## Outcome`: "Leave it empty until then,
comment and all." `close` says of any `NOKORI_TODO:`: "still has a
`NOKORI_TODO:` comment, so it is not finished." The comment the template tells
the author to keep is the one `close` stops on.

Following the template as written therefore makes an item that cannot be
closed, and the way out is to break the template: delete the comment before
closing, which is the opposite of "comment and all". A reader who does this
learns the rule wrong, and a reader who does not is stopped at the last step
with an error that says the section is not written yet, which it is. The rule
the reader is left with is worse than either instruction alone: that the
comment may be kept or deleted, and nothing says which.

The run-in is not a corner case. Every item passes through it: it is the
state `create` leaves the file in, and the state the first commit records. The
first time an item was written by following the template, the close was
refused, and the error named `## Outcome` as the unwritten section.

The same gap shows up once more in the error `close` prints. It ends with
"If a section has nothing to say, delete its comment along with the heading",
and deleting the `## Outcome` heading is the one section for which that is not
allowed: the next check in `close` refuses a file with no `## Outcome`. So the
error sends the writer to a fix that leads to a second refusal.

## Design

The comment in the `## Outcome` section of all four templates
(`bug`, `feedback`, `plan`, `rfc`) says when it goes, and one error message in
`close` changes with it. Nothing else changes.

No other section moves: a writer deletes those comments as they write the
sections, and they are gone long before the item is a candidate for closing.
Only `## Outcome` has a comment that outlives the first day, and so only it
needs to say when it goes.

**The comment stays while the item is open.** It is the only place that says
the section is written at the close: the author of an item and the person
carrying it out are often not the same, and what tells the second one what is
expected of them is the file as `create` wrote it. A section that is empty
with no comment reads as a section with nothing to say, and the convention --
record the outcome -- would not reach the reader at all.

**It is gone by the time the file closes.** The opening sentence changes from
"Leave it empty until then, comment and all" to say both halves: the section
stays empty while the item is open, and the comment is deleted when the
section is written -- which is the close, because `close` refuses a file whose
`## Outcome` is empty. That is the whole of the change: one sentence restated,
and the two later sentences that contradict it ("their comments are gone" in
the middle paragraph, "Delete this comment when you write the section" at the
end) brought into line. The instruction that carries the convention and the
state `close` requires no longer disagree.

**The section it leaves is empty.** The comment goes with no placeholder in
its place. The template already leaves `## Outcome` empty, which is what
`close` checks for, and an empty section is what a file that has not closed
yet should say. A mark saying "this emptiness is intended" would have to be
told apart from the text that replaces it, and would make `close` treat a
comment as content everywhere and as nothing here.

**The error `close` prints is narrowed.** The line about deleting a heading
with its comment now says which sections it applies to: every section but
`## Outcome`, which `close` needs to find. This is one sentence in the
`NOKORI_TODO:` refusal, not a new failure mode. Without it, the error sends
the writer to a fix that leads to a second refusal.

`close` does not change. It already refuses an `## Outcome` with nothing in
it, and that is the state the corrected template leaves behind: the writer
deletes the comment as part of writing the section, and commits both together
before the move, as `close` already requires.

## Alternatives

**Let `close` ignore the comment in `## Outcome`.** The check could skip one
comment in a known place instead of changing the template, which is the
smallest change of all. It loses because the exception is invisible from the
template: the writer keeps the comment, `close` does not stop, and the file
arrives in `closed/` carrying a `NOKORI_TODO:` that the next reader finds.
The templates are what says a comment is to be deleted, so the fix belongs
where the instruction is.

**Delete the `## Outcome` section from the templates and have `close` require
writing it.** The section would be made at closing time, and `close` would
refuse a file without it, so nothing would sit empty. It loses because the
template stops showing the whole file at once: the section an item is judged
by at the end -- what was decided, in what words -- would be invisible until
that end. The same reason the section is in the template rather than appended:
a file that is complete on its own is worth more than one that needs a piece
added to finish it.

**Keep the instruction and print a warning from `create` about the one comment
that has to go.** It loses because a warning that contradicts the file it just
wrote is not a warning but a second instruction, and the reader has to work out
which of the two the file means. The file is what the person carrying the item
out reads, long after the warning has scrolled away, so the one place the
convention has to be right is the file.

**Give the `Outcome` comment a marker of its own, so that `close` reads it as
something other than unfinished work.** It loses even though it is the
smallest change of all -- `close` would not change at all: the file would
arrive in `backlog/closed/` with a comment still in it, and every other closed
section has none. A reader who learns "a closed file has no comments left"
would find one, in the section they came for, and look for a second rule to
explain it. Two markers are two rules to remember; the rule `close` has now is
one.

**Leave a placeholder in the empty section, so that the convention sits in a
comment the writer keeps.** A sentence saying "written at the close" would
stay in place of the instructions, and the emptiness would read as intended.
It loses because a comment in the section `close` requires to be empty is the
one thing `close` cannot tell apart from text: either it reads the comment as
content and an empty section passes with nothing in it, or it ignores comments
there and the one section is read by a different rule than the rest.

## Open questions

None.

The comment that stays -- the one in `## Outcome` while the item is open --
needs no answer later. It is deleted in the same edit that writes the section,
so the file is committed as closed with nothing left in it; `close` already
refuses a file with uncommitted changes.

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
[bug-empty-input](../closed/20261114-bug-empty-input.md). Link only an item
that is already closed: never link one that is still open, not even as
background. Run `nokori explain references` for the paths and the rule.

An accepted RFC can still be overturned later. Say so in the later item and
cite this one; do not edit this file afterwards to follow it.

Once this section is written and committed, treat the file as closed: do not
edit any part of it again, and append nothing later. If something turns up
later, leave the file as it is and open a new item that cites this one and
acts on it.

Delete this comment when you write the section. -->
