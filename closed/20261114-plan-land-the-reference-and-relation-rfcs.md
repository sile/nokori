# Plan: land the three RFCs on references and relations

## Goal

Carry out
[rfc-how-items-refer-to-each-other](closed/20261114-rfc-how-items-refer-to-each-other.md),
[rfc-never-cite-an-open-item-from-a-closed-one](closed/20261114-rfc-never-cite-an-open-item-from-a-closed-one.md),
and
[rfc-carry-the-relations-in-the-steps](closed/20261114-rfc-carry-the-relations-in-the-steps.md):
their rules are in the templates, and `close` refuses a file whose link points
into `backlog/` or at a file that does not exist. Checked by: `nokori explain
references` prints the rule, the templates carry the line and the example, the
plan template has no `Items` section, and `cargo test` covers both refusals.

## Steps

- [x] `templates/explain/references.md` exists and `nokori explain references`
      prints it; the topic is listed in `TOPICS` and in the error for an
      unknown topic. `tests/explain.rs` covers it.
- [x] `close` refuses a file holding a link into `backlog/`; `tests/close.rs`
      has a case for it and the error names the link and what to do.
- [x] `close` refuses a file holding a link that does not resolve, with the
      same kind of message.
- [x] The plan template has no `Items` section, and `Steps` and `Goal` carry a
      sentence each about where a relation goes, pointing at
      `nokori explain references`.
- [x] The `Outcome` comment of all four templates carries the line and the
      example for a reference, and the `Motivation` comment of `rfc` and the
      `Feedback` comment of `feedback` carry the same.
- [x] `nokori explain kinds` lists the sections of `plan` without `Items`.
- [x] `cargo fmt`, `clippy -D warnings` and `cargo test` pass.
- [x] The three RFCs are closed, after their `Outcome` sections are written.
- [x] The item held in
      [feedback-rename-plan-items-section](closed/20261114-feedback-rename-plan-items-section.md)
      is answered by this change or by a new item that names it.

## Notes

Assumed before the first step: the three RFCs are accepted. If one is turned
down, the steps that carry it out are cut rather than reshaped -- the others
do not depend on the reasoning, only on the link form.

The work is likely to get stuck in the link check. `close` reads a file and
resolves relative paths against the directory the file is in and the
directory it is going to; both are known before the move, so the check can be
written without moving anything first.

If the plan turns out to be too big, cut the last step. The closed feedback
item is answered either way, because the answer is the rule in the first two
RFCs, and the last step only records where it landed.

The plan itself is the first thing the new rule bit: its `Items` section named
three open RFCs, and every one of those links had to be rewritten to
`closed/` as they closed. The check refused this file until that was done,
which is the rule working on the item that wrote it.

## Outcome

Every step ran. The work is commit `994a457`, which adds `nokori explain
references`, the link check in `close`, and the template sentences, and drops
`Items` from the plan template.

The plan was written with `Items`, and that section is what pushed the first
change: the links it held had to be rewritten to `closed/` as each RFC closed,
which the new check refused until it was done. The section is gone from this
file now, because the template no longer has one -- the relations live in the
`Goal` and in the steps, where they belong.

Two things turned up that the RFCs did not foresee, and both are recorded in
their outcomes instead of being smoothed over here:

- The three RFCs cited each other, so none of them could close first under the
  new rule. The one that closed first had to name the other in words, and the
  other then linked it once it was closed. That is the rule doing what it says,
  and it cost one edit.
- The examples in the RFCs and templates used made-up file names, which the
  check refused. The fix was to point the examples at items that exist.

[feedback-rename-plan-items-section](closed/20261114-feedback-rename-plan-items-section.md)
is answered: the section was renamed first, and then dropped, as the third RFC
decided.
