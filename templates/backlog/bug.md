# Bug: <!-- TODO: one line naming what is broken -->

## Summary

<!-- TODO: one paragraph saying what is broken and the smallest input that
shows it.

A bug is a report that today's behavior breaks its own documented contract,
not a proposal for a different one.

The item may point at the source code, but the source code never points back:
never mention `backlog/` in the source code, in a commit message, or in a pull
request. A bug report is something you read and act on here, and it stops
working as one if the code it describes has to be read for it to make sense.

Delete this comment once the paragraph is here. -->

## Reproduction

<!-- TODO: the smallest exact input and the exact calls that show it.

Include the context this needs and leave out the context it does not: the
report is self-contained when a reader can reproduce it without opening
anything else. Being complete is not the same as being exhaustive -- a
snippet that pulls in the whole calling module copies code nobody asked to
read, and it goes stale as soon as that code changes.

Prefer a concrete value -- a command, a key sequence, a specific file -- over
a description.

Delete this comment once the section is here. -->

## Observed behavior

<!-- TODO: what actually happens, and the invariant it breaks.

Quote the test, branch, or assertion that the observed behavior contradicts
rather than paraphrasing it: the quote points at where to look, and the
text around it says what the contrast is.

Knowing where it goes wrong is a help to the fix, not a requirement: say
where if you know, and leave the rest to the fix.

Delete this comment once the section is here. -->

## Expected behavior

<!-- TODO: what the documented contract says should happen instead.

Cite the rustdoc, invariant, or type that promises it. The citation is what
makes this a bug report rather than a preference.

Delete this comment once the section is here. -->

## Notes

<!-- TODO: what helps the fix, without turning this into a design document.

Write it in the present tense: this is a claim about the work ahead, not a
report of what was done.

Worth saying here: related cases with the same cause, why the simpler-looking
fix is wrong, where the fix belongs, and whether the question is big enough to
need an RFC instead.

Delete this comment once the section says what it should, or the whole
section if there is nothing to add. -->

## Outcome

<!-- TODO: the one section written later, when `nokori close` moves this
file. Leave it empty until then, comment and all.

The sections above are written before the item moves: by the time `nokori
close` moves this file, they are written, their comments are gone, and the
change is committed.

Those sections may be rewritten as long as the item is open, as often as the
understanding changes. What they may not become is a description of work
already done. `nokori close` needs the change committed, so whatever they say
at that point is what the moving commit records.

Say what actually happened, and name the change that fixed it; link the
commit or pull request when there is one.

When the fix differs from what the sections above expected, say so here rather
than editing them: those sections record what was thought while the bug was
open, and rewriting them erases the very difference that is worth knowing.

Once this section is written and committed, treat the file as closed: do not
edit any part of it again, and append nothing later. If something turns up
later -- most of all, if this turns out not to have been a bug -- leave the
file as it is and open a new item (an RFC or another bug) that cites this one
and acts on it.

Delete this comment when you write the section. -->
