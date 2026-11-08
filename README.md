nokori
===

- backlog management cli tool
- markdown file based

What this tool is
-----------------

`nokori` cannot enforce much of what it recommends. It keeps no hidden files
and no index: the backlog is ordinary directories and ordinary markdown files,
which anyone can edit, move, or delete by hand, and there is hardly any check
that could fail.

What it offers is one good way to keep a backlog, written down as a flow of
commands. The rules are recommendations and following them is the user's own
responsibility -- the same deal as git-flow, where the flow is a convention
and the tool only makes it convenient.

The tool exists at all, rather than as a page of documentation, so that the
flow can be walked without knowing it in advance. Every command reports what
to do next, so `nokori --help`, then `nokori init`, then `nokori new` each lead
on to the following step, and the whole shape is learned by running them in
order.


Decided
-------

- `nokori init` writes `backlog/README.md` too, but its content stays minimal,
  so that keeping it current as `nokori` changes costs little.
- The templates are self-descriptive instead. `nokori new <kind>` writes an
  item whose sections each carry a `<!-- TODO: ... -->` saying what belongs
  there and what to keep in mind; the author deletes the comment once the
  section says what it should. That instruction is itself a `<!-- TODO: -->`
  in the template.
- An item stands on its own. Its sections carry whatever a reader needs to
  judge it, and the sections that are always there are the ones every kind
  must answer: for a `bug`, the reproduction, the observed behavior, and the
  expected behavior. The rule that an item may cite the code but the code
  never cites an item is written into the template, because a reader of an
  item may never open `backlog/README.md`, and because that file stays fixed
  after `init` and is kept minimal. Advice that does not fit an item goes in
  `nokori explain <topic>`, not in the README.
- A `bug` item keeps an `## Outcome` section from the start, empty until
  `nokori finish` writes it. A file that is complete on its own is worth more
  than one that matches the settled files in a reference repository, so the
  section is in the template rather than appended at the end.
- `## Outcome` is the only section that stays empty while the item is open.
  Until `finish` the item is a report of a problem; after it, the sections
  above `## Outcome` still say what was thought while the item was open,
  including what it got wrong. A fix that differs from what they predicted is
  described in `## Outcome`, not by rewriting them.
- There is no section for a later addition. A template offering one would read
  as advice to amend a settled item, and `nokori` recommends no such flow: a
  file in `done/` is left as it was written. Appending to it by hand is the
  user's own business, the same as any operation outside the flow.
- An item's state is expressed by the directory it is in, never by a `Status`
  field inside the file. There are two: `backlog/` itself, and `done/`.
- v1 has four item kinds: `rfc`, `bug`, `plan`, and `spike`. There is no
  `memo` kind.
- Every file carries a kind in its filename (`YYYYMMDD-<kind>-<slug>.md`). The
  kind is part of the name rather than a state, so it stays put while the item
  is open; changing it means deleting the file and creating it again with the
  new kind. History is git's problem, so `nokori` offers no rename.
- `backlog/` holds every item that is not settled yet, whether it is being
  worked on right now or not. There is no directory for "started" and none for
  "not ready to start": an item that is waiting on something says so in its
  body, where a reader looks for the reason anyway.
- `done/` does not fork by outcome. It holds every settled item, and for `rfc`
  alone the outcome is written into the filename
  (`YYYYMMDD-rfc-accepted-<slug>.md` / `YYYYMMDD-rfc-rejected-<slug>.md`).
  `bug`, `plan`, and `spike` keep their name unchanged when they move there.
- An item is ended by moving it to `done/`, never by deleting it, so that
  ending it leaves a record. Deleting a file by hand is the user's own
  business, the same as any operation outside the flow.
- Files in `done/` are not edited afterwards; corrections and additions go in
  as appended notes. It is the same implicit rule as out-of-flow operations in
  git-flow: doing something outside the flow by hand is the user's own
  responsibility, and does not need to be spelled out.
- Moving an item into `done/` is local only: the tool rewrites the file and
  moves it. It does not commit, push, or otherwise change anything through
  git.
- A `bug` that cannot be fixed for now stays in `backlog/`, with the reason it
  is stuck written in its body. One that is decided never to be fixed is not a
  bug but a documented behavior: the behavior is written into the public docs
  and the item goes to `done/`, the same way a report that turns out not to be
  a bug does.
- The commands are one per destination directory:

  | command | moves to | from |
  | ------- | -------- | ---- |
  | `new <kind> <slug>` | `backlog/` | -- |
  | `finish <file>` | `done/` | `backlog/` |

  Two states leave little room for the table to grow, and that is the point:
  every state costs the reader a word to learn, and gives two files in
  different states a way to say the same thing.
- Commands may read git state, but never change it: no commit, no push, no
  network. A move checks that the file it touches is committed, the way
  `cargo publish` refuses a dirty tree, but only for that file rather than the
  whole repository. `nokori` assumes it runs inside a git repository.
- A command reports what to do next when it finishes. `init` prints how to
  create the first item and does not go on to explain the kinds: the generated
  file already shows what each kind's sections expect.
- Reference material that does not follow from a command lives under
  `explain <topic>`. The first two topics are `kinds` (the kinds, plus how to
  choose between them) and `flow` (how an item is opened and settled).
- `explain` also covers the cases a reader arrives with, and the commonest of
  those is a file in a state the command does not take. Running `finish` on a
  file already in `done/` is an ordinary mistake, and the message a command
  prints for it says only that the move is not allowed; the explanation of
  what the states mean, and what to do with the file instead, belongs in a
  topic of its own.


Rationale
---------

### Decisions versus pieces of work

The reference repositories (kk, termnix, tuinix) use only `rfc` and `bug`.
`tuinix/backlog/README.md` states the criterion for adding a kind:

> When such items start to accumulate -- and the tell is having to write an
> RFC whose Summary already says what will be done -- add a third kind then.
> It would carry a two-value state (`open` / `done`) instead of the
> `accepted`/`rejected` and `fixed`/`not-a-bug` vocabularies, because it
> records work rather than a decision.

That criterion separates kinds into (a) a decision and (b) a piece of work.
`rfc` and `bug` are decisions. `plan` and `spike` are both pieces of work.
Every kind walks the same directories (`backlog/` -> `done/`); only what the
move to `done/` records differs, and for `rfc` that record lives in the
filename.

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
needs, and it is not a property of a kind: an item written down before it is
worth doing is an ordinary item that happens to be idle. Keeping `memo` would
have made it the one kind with its own rules, and every reader of `nokori`
would have had to learn why. One rule is worth more: an item is something with
a kind, and everything is settled the same way.

### Why the state is the directory, not a `Status` field

A field inside the file can disagree with where the file is. A file in `done/`
with `Status: open` leaves the reader to decide which one is true, and it
takes a rule to say the directory wins -- a rule that has to be remembered
whenever a file is edited by hand.

Keeping the state in one place removes the question. `ls backlog/` and `ls
backlog/done/` tell the whole story. The cost is that a state change is a
move, and that `done/` has to record the outcome somewhere; for `rfc` that
record is a word in the filename rather than a field.

### Why there are only two states

The obvious additions are a place for work that has been started and one for
work that is waiting. Both name something real, and both lose more than they
give.

A "started" directory cannot keep anybody from starting the same item twice:
`nokori` has no index and no lock, so the move is a claim, not a guarantee.
What it does do is give an item a third place to sit, and every extra place is
another way for two files to say the same thing.

A "waiting" directory is worse, because it is where items go to be forgotten.
An item that is stuck is precisely the one that has to stay in view, and the
place to say why it is stuck is its own body. With the directory in place, an
item nobody intends to do any more collects there instead of ending in
`done/`, and the record of deciding not to do it never appears.

So an item is open or settled, and `backlog/` means what it says: everything
not settled, whether or not anybody is working on it. If a state later turns
out to earn its place, adding one is easy; removing one costs the user a
rename of every file in it, which is why the smaller set goes first.

### Why `done/` is not edited

A file in `done/` records what was concluded at a particular time. Rewriting it
would make the record show what is known now instead, and the reader could no
longer tell which parts were the original judgement. Corrections and additions
are appended for that reason: the earlier text stays legible and the revision
is dated by git.

### Why `finish` and not `settle`

The first design had four moves and named them all by what they did to the
file, which is why `settle` lost to `finish` then: next to `open`, `start`,
and `suspend`, a word describing the conclusion was the odd one out. With
those gone, `settle` would say more accurately what happens.

It stays `finish` because the reader starts from a piece of work and asks to
be done with it. `settle` also settles a question, and a `bug` that turns out
not to be a bug is settled without any work being finished.

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

### Why a disallowed move is explained in `explain`, not in the error

A command that refuses a file can only say what it will not do: the file is
not in a state this move takes. What the reader needs next is a rule about the
states themselves -- which command takes this file, and why the state it is in
does not accept this one -- and that answer is the same paragraph whatever the
command that refused it. Printing it from each command would write the same
text several times and make the refusal long; leaving it in the error alone
would send the reader to `--help` for a fact that is not about usage.

Putting it in `explain` keeps the error to one sentence and gives the longer
answer a home. It is also where the reader already goes for material that does
not follow from a command, so nothing new has to be learned to find it.

### Why `init` does not explain the kinds

`init` runs once per repository, and most of what it could say is needed later
and elsewhere: which kind to use, what `backlog/` holds, what `finish` does to
a file. Printing all of that would make the one output nobody
re-reads long. So `init` stops at the next step, and the reference material is
reached through `explain` from wherever it is needed.

### Why an `rfc` outcome is written when it merges

An `rfc` spends time between "accepted" and "merged", and during that time it
can still be turned down: trying the implementation may show the design does
not hold. So the filename gets its `accepted` or `rejected` word when the item
moves to `done/`, not when somebody agrees to it. Until then the item sits in
`backlog/` like any other, and whoever is implementing it says so in the body.


Open questions
--------------

### Whether an untracked file counts as dirty

A move refuses to run while the file it touches has uncommitted changes, but
an untracked file is a different case: `git status` reports it as dirty, yet
there is no earlier content for the move to bury.

The flow it blocks is narrow -- `new` writes an untracked file, so `finish`
run before that file's first commit would fail under the strict reading --
which is why this is left to the implementation rather than settled here.

