# Bug: <!-- TODO: one line naming what is broken -->

## Summary

<!-- TODO: one paragraph. Say it before the item moves: `nokori start`,
`nokori suspend`, and `nokori finish` all expect the paragraph to be written,
this comment gone, and the change committed.

A bug is a decision that today's behavior breaks its own documented contract,
not a proposal for a different one. Give the smallest input that shows it.

An item may point at the code, but the code never points back: never mention
`backlog/` in the code, in a commit message, or in a pull request.

Delete this comment once the paragraph is here. -->

## Reproduction

<!-- TODO: the smallest exact input and the exact calls that show it.

The report is self-contained when a reader can act on it without opening
anything else. Pull in what it needs from the calling code; leave out what it
does not.

Prefer a concrete value -- a command, a key sequence, a specific file -- over
a description. The exact calls matter more than the context: the fix can read
the implementation, but nobody can reproduce what was never written down.

Delete this comment once the section is here. -->

## Observed behavior

<!-- TODO: what actually happens, and the invariant it breaks.

Quote the test or branch rather than paraphrasing it. A quote stays true when
the code changes; a paraphrase quietly drifts away from it.

Knowing where it goes wrong is a help to the fix, not a requirement: say
where if you know, and leave the rest to the fix.

Delete this comment once the section is here. -->

## Expected behavior

<!-- TODO: what the documented contract says should happen instead.

Cite the rustdoc, invariant, or type that the observed behavior breaks. The
citation is what makes this a bug report rather than a preference.

Delete this comment once the section is here. -->

## Notes

<!-- TODO: what helps the fix, without turning this into a design document.

Write it in the present tense: this is a claim about the work ahead, not a
report of what was done.

Worth saying here: related cases with the same cause, why the simpler-looking
fix is wrong, where the fix belongs, and whether the ambiguity is big enough
to need an RFC instead.

Delete this comment once the section says what it should, or the whole
section if there is nothing to add. -->

## Outcome

<!-- TODO: the one section written later, when `nokori finish` moves this
file. Leave it empty until then, comment and all.

Say what actually happened, and name the change that fixed it; link the
commit or pull request when there is one.

When the fix differs from what the sections above expected, say so here rather
than editing them: those sections record what was thought while the bug was
open, and rewriting them erases the very difference that is worth knowing.

Delete this comment when you write the section. -->
