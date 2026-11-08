# Bug: <!-- TODO: one line naming what is broken -->

## Summary

<!-- TODO: one paragraph. What is broken, and the smallest input that shows
it. Keep it short: a bug is a decision that today's behavior breaks its own
documented contract, not a proposal for a different one. Write the report so
it stands on its own -- someone who reads only this file should be able to
judge it. Delete this comment once the paragraph is here. -->

## Reproduction

<!-- TODO: the smallest exact input and the exact calls that show it, so
someone can reproduce it without reading the implementation. Prefer a concrete
value (a command, a key sequence, a specific file) over a description. Pull in
what the report needs from the calling code, and leave out what it does not:
the report is self-contained when a reader can act on it without opening
anything else. Delete this comment once it is here. -->

```text
<exact input>
<exact call(s)>
<observed output>
```

## Observed behavior

<!-- TODO: what actually happens. Name the invariant that is broken, and
quote the test or branch rather than paraphrasing it. If you already know
where the code goes wrong, say where; that is a help to the fix, not a
requirement for the report. Delete this comment once it is here. -->

## Expected behavior

<!-- TODO: what the documented contract says should happen instead. Cite the
rustdoc, invariant, or type that the observed behavior breaks. Delete this
comment once it is here. -->

## Notes

<!-- TODO: anything that helps the fix without turning this into a design
document: related cases with the same cause, why the simpler-looking fix is
wrong, whether the ambiguity needs an RFC, and where the fix belongs. Write it
in the present tense while the item is open: it is a claim about the work
ahead, not a report of what was done. Delete this comment once the section
says what it should, or the whole section if there is nothing to add. -->

## Outcome

<!-- TODO: written when `nokori finish` settles the bug, and empty until
then. Say what actually happened and name the change that fixed it. When the
fix differs from what the sections above expected, say so here rather than
editing them: they record what was thought while the bug was open. -->

<!-- An item may point at the code, but the code must never point back at an
item: cite a source path from here, but never mention `backlog/` in the code,
the README, a commit message, or a pull request. -->
