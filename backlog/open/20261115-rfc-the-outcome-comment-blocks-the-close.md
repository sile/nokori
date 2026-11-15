# RFC: Stop the `Outcome` comment from blocking the close

## Summary

The `Outcome` comment in every template is written as a comment to keep until
the item closes, and `close` refuses a file that has any `NOKORI_TODO:` left.
A file that follows the template exactly therefore cannot be closed. The
change is to make the instruction say the one thing both rules need: the
comment is deleted when the section is written, as every other comment is,
and written instead is a line saying that the section is filled in when the
item closes.

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
with an error that says the section is not written yet, which it is.

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
(`bug`, `feedback`, `plan`, `rfc`) changes in two ways, and one error message
in `close` changes with it.

**It is deleted rather than kept.** The opening paragraph no longer says to
leave it, and the closing line is the same as the other sections': the comment
goes when the section is written. That is the whole of the change to the
template -- one paragraph shorter, and no new text.

**The section it leaves is empty.** No placeholder takes the comment's place.
The template already leaves `## Outcome` empty, which is what `close` checks
for, and an empty section is what a file that has not closed yet should say.
A mark saying "this emptiness is intended" would have to be told apart from
the text that replaces it, and would make `close` ignore a comment it treats
as content everywhere else.

**The error `close` prints is narrowed.** The line about deleting a heading
with its comment now says which sections it applies to: every section but
`## Outcome`, which `close` needs to find. This is one sentence in the
`NOKORI_TODO:` refusal, not a new failure mode.

`close` does not change. It already refuses an `## Outcome` with nothing in
it, and that is the state the corrected template leaves behind: the writer
gets there by deleting the comment while the item is open, and fills the
section when it closes.

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
which of the two the file means. A comment that is to be kept in one place and
deleted in another teaches a rule that is not true.

**Leave a placeholder in the empty section.** The comment deleted would be
replaced by another saying the section is written at closing time, so that the
emptiness reads as intended. It loses because emptiness is already what the
template means and what `close` checks, and the placeholder would be the one
comment `close` had to read as content everywhere and as nothing here.

## Open questions

None.

The placeholder that was weighed and left out -- a comment in the empty
section saying it is written at closing time -- needs no answer later. It
buys nothing the template does not already have, and it would cost `close` a
special case.

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
