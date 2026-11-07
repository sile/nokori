nokori
===

- backlog management cli tool
- markdown file based


Decided
-------

- `nokori init` writes `backlog/README.md` too, but its content stays minimal,
  so that keeping it current as `nokori` changes costs little.
- The templates are self-descriptive instead. `nokori new <kind>` writes an
  item whose sections each carry a `<!-- TODO: ... -->` saying what belongs
  there and what to keep in mind; the author deletes the comment once the
  section says what it should. That instruction is itself a `<!-- TODO: -->`
  in the template.
- An item's state is expressed by the directory it is in, never by a `Status`
  field inside the file. The directories are `draft/`, `backlog/` itself,
  `ongoing/`, `pending/`, and `done/`.
- v1 has four item kinds: `rfc`, `bug`, `plan`, and `spike`. There is no
  `memo` kind.
- `draft/` means *not ready to be started*. It is not a place for notes of a
  particular shape; it is one of the states, and an item can sit there with a
  complete body if the work still cannot begin. Every file, in `draft/`
  included, carries a kind in its filename (`YYYYMMDD-<kind>-<slug>.md`).
- The kind in `draft/` is a guess and may be wrong. Changing it means deleting
  the file and creating it again with the new kind; history is git's problem,
  so `nokori` offers no rename.
- `draft/` and `pending/` both mean "not being worked on right now", but they
  are different states: `draft/` is work not yet accepted as worth doing or
  ready to begin, `pending/` is work that was open and is now interrupted.
- `ongoing/` is a claim: an item is there because someone has started on it, so
  nobody else picks it up. For an `rfc` it means *provisionally* accepted --
  acceptance is settled only when the change merges.
- `done/` does not fork by outcome. It holds every settled item, and for `rfc`
  alone the outcome is written into the filename
  (`YYYYMMDD-rfc-accepted-<slug>.md` / `YYYYMMDD-rfc-rejected-<slug>.md`).
  `bug`, `plan`, and `spike` keep their name unchanged when they move there.
- An item may be deleted without trace only while it is in `draft/`. Once it
  has been open, deleting it is not an option: it must go to `done/` first, so
  that ending it leaves a record. The test is whether the item was ever
  accepted as work worth doing.
- Files in `done/` are not edited afterwards; corrections and additions go in
  as appended notes. It is the same implicit rule as out-of-flow operations in
  git-flow: doing something outside the flow by hand is the user's own
  responsibility, and does not need to be spelled out.
- Moving an item into `done/` is local only: the tool rewrites the file and
  moves it. It does not commit, push, or otherwise change anything through
  git.
- A `bug` that cannot be fixed for now goes to `pending/`. One that is decided
  never to be fixed is not a bug but a documented behavior: the behavior is
  written into the public docs and the item goes to `done/`, the same way a
  report that turns out not to be a bug does.
- The commands are one per destination directory:

  | command | moves to | from |
  | ------- | -------- | ---- |
  | `new <kind> <slug>` | `backlog/` | -- |
  | `new <kind> <slug> --draft` | `draft/` | -- |
  | `open <file>` | `backlog/` | `draft/`, `pending/` |
  | `start <file>` | `ongoing/` | `backlog/` |
  | `suspend <file>` | `pending/` | `backlog/`, `ongoing/` |
  | `finish <file>` | `done/` | `backlog/`, `ongoing/`, `pending/` |

  `open` covers both `draft/` -> `backlog/` and `pending/` -> `backlog/`
  because the destination is the same. `new` writes to `backlog/` by default;
  `--draft` is the exception, because an item that is worth recording but not
  yet worth starting is the unusual case.
- Commands may read git state, but never change it: no commit, no push, no
  network. A move checks that the file it touches is committed, the way
  `cargo publish` refuses a dirty tree, but only for that file rather than the
  whole repository. `nokori` assumes it runs inside a git repository.
- A command reports what to do next when it finishes. `init` prints how to
  create the first item and does not go on to explain the kinds: the generated
  file already shows what each kind's sections expect.
- Reference material that does not follow from a command lives under
  `explain <topic>`. The first two topics are `kinds` (the kinds, plus how to
  choose between them) and `flow` (the state transitions and the commands that
  cause them).


Rationale
---------

### Decisions versus pieces of work

The reference repositories (kk, termnix, tuinix) use only `rfc` and `bug`.
`tuinix/backlog/README.md` states the criterion for adding a kind:

> When such items start to accumulate -- and the tell is having to write an
> RFC whose Summary already says what will be done -- add a third kind then.
> It would carry a two-value state (`open` / `done`) instead of the
> `draft`/`accepted`/`rejected` and `open`/`fixed`/`not-a-bug` vocabularies,
> because it records work rather than a decision.

That criterion separates kinds into (a) a decision and (b) a piece of work.
`rfc` and `bug` are decisions. `plan` and `spike` are both pieces of work.
Every kind walks the same directories (`backlog/` -> `ongoing/` -> `done/`,
with `pending/` and `draft/` on the side); only what the move to `done/`
records differs, and for `rfc` that record lives in the filename.

### `plan` versus `spike`

Two kinds sharing one state machine need a rule that tells them apart, or the
choice is made fresh every time.

| | `plan` | `spike` |
| --- | ------ | ------- |
| question | how to get there | is it possible, and what happens if we try |
| deliverable | the work, in phases | what was learned |
| what `done` means | the work is finished | the question is answered |
| what follows | nothing | an `rfc`, a `plan`, or nothing at all |

The test is whether **doing nothing is an acceptable answer**. A `spike` exists
to settle the next step, so "the answer is no, do not build it" closes it
legitimately, with that finding written in the file. When the work is
going to happen either way, the investigation is a first phase of a `plan`,
not a `spike`; splitting it out would leave a `spike` that cannot end in
anything but a `plan`.

A finished `spike` is worth keeping in `done/` for the same reason any other
item is: the finding is what a later reader needs in order to understand why
the next item says what it says. A spike whose result was thrown away is
usually a spike whose result will be rediscovered.

### Why there is no `memo` kind

A `memo` would have been a kind that never moves. That is the only property it
needs, and it is a property of a state rather than of a kind: something written
down before it is worth doing. `draft/` already is that state, so a note is an
ordinary item that happens to be in `draft/`, and it leaves by the same door as
any other: `open` to begin, or deletion if it never mattered.

Keeping `memo` would have made it the one kind with its own state machine, and
every reader of `nokori` would have had to learn why. Folding it into `draft/`
keeps one rule: an item is something with a kind, and everything with a kind
moves.

### Why the state is the directory, not a `Status` field

A field inside the file can disagree with where the file is. A file in `done/`
with `Status: draft` leaves the reader to decide which one is true, and it
takes a rule to say the directory wins -- a rule that has to be remembered
whenever a file is edited by hand.

Keeping the state in one place removes the question. `ls backlog/done/` and
the filename tell the whole story. The cost is that a state change is a move,
and that `done/` has to record the outcome somewhere; for `rfc` that record is
a word in the filename rather than a field.

### Why only `draft/` may be deleted

An item in `draft/` has not been accepted as work worth doing yet. Nobody has
committed to it, so dropping it concludes nothing and leaves nothing to
record. An item in `backlog/`, `ongoing/`, or `pending/` has been accepted, so
its end is a conclusion and belongs in `done/` -- a `bug` that turns out not to
be a bug, a `plan` that is abandoned. That single test, *was it ever open*,
is what makes `draft/` the one state where deletion is silent.

### Why `done/` is not edited

A file in `done/` records what was concluded at a particular time. Rewriting it
would make the record show what is known now instead, and the reader could no
longer tell which parts were the original judgement. Corrections and additions
are appended for that reason: the earlier text stays legible and the revision
is dated by git.

### Why `suspend`, and why the commands name the move

`suspend` was chosen over `pause`, `defer`, `block`, and `park` for two
reasons. It pairs with the directory it produces: a suspended item is a
pending one. And it does not promise a duration -- `pause` suggests a short
interruption and `defer` a postponement, but what actually keeps an item in
`pending/` is a condition, not a length of time. The condition itself belongs
in the body, where the reader will look for it.

The commands name the move rather than the outcome, which is why `settle` was
not used for `done/`: sitting next to `open`, `start`, and `suspend`, a word
that describes what was concluded would be the odd one out.

### Why a move checks that the file is clean

A move and a content change in one commit make the history unreadable: the
commit that says "settle this item" also rewrites what the item said, and the
rewrite is what a later reader wants to see on its own. `cargo publish`
refuses a dirty tree for the same reason, and the check here is narrower only
because an item is not the whole crate -- what matters is the file being
moved, not the working tree around it.

### Why `explain` and not `doc`

A command that moves files is a verb, and `doc` is not one: in `cargo doc` it
means "build the documentation". A reader who saw `nokori doc kinds` would
expect documentation to be written or opened, not printed. `explain` says what
happens, and it has the same sense as `cargo --explain`.

`help` was the other candidate, following `git help <concept>`, but it would
sit next to `--help` and leave the reader to work out which one to use.

### Why `init` does not explain the kinds

`init` runs once per repository, and most of what it could say is needed later
and elsewhere: which kind to use, what a `draft/` item is for, how `suspend`
differs from `finish`. Printing all of that would make the one output nobody
re-reads long. So `init` stops at the next step, and the reference material is
reached through `explain` from wherever it is needed.

### Why `open` serves two moves

`draft/` -> `backlog/` and `pending/` -> `backlog/` land in the same place, so
the same command covers both. The alternative -- `open` and `resume` -- would
name the same outcome twice, and the reader would have to learn which applies
before running either.

### Why `ongoing/` exists

An `rfc` spends time between "accepted" and "merged". During that time it can
still be turned down: trying the implementation may show the design does not
hold, and the item goes to `done/` as `rejected`. So `accepted` is only
provisional until the change merges.

`ongoing/` is what makes that provisional state visible. An item there has
been started -- an `rfc` by a provisional acceptance, a `bug` by a fix in
progress -- and nobody else should pick it up. Without the directory, an
accepted `rfc` would look exactly like one nobody had started on, and two
people could begin the same work.


Open questions
--------------

### Whether an untracked file counts as dirty

A move refuses to run while the file it touches has uncommitted changes, but
an untracked file is a different case: `git status` reports it as dirty, yet
there is no earlier content for the move to bury. `new --draft` followed by
`open` without a commit in between would fail under the strict reading.

The flow it blocks is narrow -- `new` writes to `backlog/` by default, so the
only item that could be opened straight from untracked is one created with
`--draft` and then promoted at once -- which is why this is left to the
implementation rather than settled here.

