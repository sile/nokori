# Plan: land the three RFCs on references and relations

## Goal

The three RFCs are accepted, their rules are in the templates, and `close`
refuses a file whose link points into `backlog/` or at a file that does not
exist. Checked by: `nokori explain references` prints the rule, the templates
carry the line and the example, the plan template has no `Items` section, and
`cargo test` covers both refusals.

## Items

- carries out
  [rfc-how-items-refer-to-each-other](20261114-rfc-how-items-refer-to-each-other.md);
  it has to be accepted first, since it decides the link form the other two
  use.
- carries out
  [rfc-never-cite-an-open-item-from-a-closed-one](20261114-rfc-never-cite-an-open-item-from-a-closed-one.md);
  its check is part of the same `close` change, and it depends on the link
  form above.
- carries out
  [rfc-carry-the-relations-in-the-steps](20261114-rfc-carry-the-relations-in-the-steps.md);
  it touches the same plan template that the first one changes, so it goes
  last and in the same commit.

All three are open, so the links above are written as `backlog/` paths and
will need rewriting to `closed/` before this plan closes. That is the rule in
the second RFC applied to itself.

## Steps

- [ ] `templates/explain/references.md` exists and `nokori explain references`
      prints it; the topic is listed in `EXPLAIN_TOPICS` and in the error for
      an unknown topic. `tests/explain.rs` covers it.
- [ ] `close` refuses a file holding a link into `backlog/`; `tests/close.rs`
      has a case for it and the error names the link and what to do.
- [ ] `close` refuses a file holding a link that does not resolve, with the
      same kind of message.
- [ ] The plan template has no `Items` section, and `Steps` and `Goal` carry a
      sentence each about where a relation goes, pointing at
      `nokori explain references`.
- [ ] The `Outcome` comment of all four templates carries the line and the
      example for a reference, and the `Motivation` comment of `feedback` and
      `rfc` carries the same.
- [ ] `nokori explain kinds` lists the sections of `plan` without `Items`.
- [ ] `cargo fmt`, `clippy -D warnings` and `cargo test` pass.
- [ ] The three RFCs are closed, after their `Outcome` sections are written.
- [ ] The item held in
      [feedback-rename-plan-items-section](../closed/20261114-feedback-rename-plan-items-section.md)
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

Say which steps ran, which did not, and why the ones that did not were left
out. Name the change that carried the plan out, or the item that did, when
the work happened somewhere else.

An unchecked box in a closed plan means the step was decided against, not
forgotten: `close` does not refuse a plan with steps left, so this section is
where the decision is recorded. A plan that stops partway through and should
be picked up again is not closed -- it stays in `backlog/` with the unchecked
boxes showing where it stopped.

Once this section is written and committed, treat the file as closed: do not
edit any part of it again, and append nothing later. If something turns up
later, leave the file as it is and open a new item that cites this one and
acts on it.

Delete this comment when you write the section. -->
