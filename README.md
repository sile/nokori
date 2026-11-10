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
to do next, so `nokori --help`, then `nokori init`, then `nokori create` each
lead on to the following step, and the whole shape is learned by running them
in order.


Decided
-------

- `nokori init` writes `backlog/README.md` too, but its content stays minimal,
  so that keeping it current as `nokori` changes costs little.
- The templates are self-descriptive instead. `nokori create <kind>` writes an
  item whose sections each carry a `<!-- NOKORI_TODO: ... -->` saying what
  belongs there and what to keep in mind; the author deletes the comment once
  the section says what it should. That instruction is itself a
  `<!-- NOKORI_TODO: -->` in the template.
- The marker is the word `NOKORI_TODO:` rather than a plain `TODO:`, so that
  `close` can find the comments a template left without matching a `TODO` the
  author wrote about the code. What `close` checks is not "the word TODO is
  gone" but "every comment written by `create` is gone", and only a marker
  the templates own can tell the two apart.
- An item stands on its own. Its sections carry whatever a reader needs to
  judge it, and the sections that are always there are the ones every kind
  must answer: for a `bug`, the reproduction, the observed behavior, and the
  expected behavior. The rule that an item may cite the code but the code
  never cites an item is written into the template, because a reader of an
  item may never open `backlog/README.md`, and because that file stays fixed
  after `init` and is kept minimal. Advice that does not fit an item goes in
  `nokori explain <topic>`, not in the README.
- A `bug` item keeps an `## Outcome` section from the start, empty until
  `nokori close` writes it. A file that is complete on its own is worth more
  than one that matches the closed files in a reference repository, so the
  section is in the template rather than appended at the end.
- `## Outcome` is the only section that stays empty while the item is open.
  Until `close` the item is a report of a problem; after it, the sections
  above `## Outcome` still say what was thought while the item was open,
  including what it got wrong. A fix that differs from what they predicted is
  described in `## Outcome`, not by rewriting them.
- There is no section for a later addition. A template offering one would read
  as advice to amend a closed item, and `nokori` recommends no such flow: a
  file in `closed/` is left as it was written. Appending to it by hand is the
  user's own business, the same as any operation outside the flow.
- An item's state is expressed by the directory it is in, never by a `Status`
  field inside the file. There are two: `backlog/` itself, and `closed/`.
- v1 has three item kinds: `rfc`, `bug`, and `plan`. There is no `memo`
  kind.
- Every file carries a kind in its filename (`YYYYMMDD-<kind>-<slug>.md`). The
  kind is part of the name rather than a state, so it stays put while the item
  is open; changing it means deleting the file and creating it again with the
  new kind. History is git's problem, so `nokori` offers no rename.
- `backlog/` holds every item that is not closed yet, whether it is being
  worked on right now or not. There is no directory for "started" and none for
  "not ready to start": an item that is waiting on something says so in its
  body, where a reader looks for the reason anyway.
- `closed/` does not fork by outcome: it holds every closed item, and no kind
  writes its outcome into the filename. A file keeps the name it was created
  with, so the outcome is read from the body, never from the path. An `rfc`
  that was accepted can be overturned by a later one, and the earlier file is
  not renamed to follow.
- An item is closed by moving it to `closed/`, never by deleting it, so that
  closing it leaves a record. Deleting a file by hand is the user's own
  business, the same as any operation outside the flow.
- Files in `closed/` are not edited afterwards; corrections and additions go in
  as appended notes. It is the same implicit rule as out-of-flow operations in
  git-flow: doing something outside the flow by hand is the user's own
  responsibility, and does not need to be spelled out.
- Moving an item into `closed/` is local only: the tool moves the file and
  touches nothing else. It does not commit, push, or otherwise change anything
  through git.
- `close` refuses an item that still has a `TODO` comment, or whose `##
  Outcome` is missing or empty, and the error names the section that is not
  written yet. The templates already say what each section needs, so the check
  only adds the one thing the file cannot say about itself: that a comment
  left behind would be read in `closed/` as part of the record.
- `closed/` is not created by `init`. It appears when the first item is moved
  into it, so a repository that never closed anything has no directory whose
  contents nothing has produced yet. `create` treats a missing `backlog/` as
  "run `init` first"; the two directories are otherwise set up the first time
  one is needed.
- A `bug` that cannot be fixed for now stays in `backlog/`, with the reason it
  is stuck written in its body. One that is decided never to be fixed is not a
  bug but a documented behavior: the behavior is written into the public docs
  and the item goes to `closed/`, the same way a report that turns out not to
  be a bug does.
- The commands are one per destination directory:

  | command | state after | directory | from |
  | ------- | ----------- | --------- | ---- |
  | `create <kind> <slug>` | open | `backlog/` | -- |
  | `close <file>` | closed | `closed/` | `backlog/` |

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
  choose between them) and `flow` (how an item is opened and closed).
- `explain` also covers the cases a reader arrives with, and the commonest of
  those is a file in a state the command does not take. Running `close` on a
  file already in `closed/` is an ordinary mistake, and the message a command
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
`rfc` and `bug` are decisions, and `plan` is the piece of work. Every kind
walks the same directories (`backlog/` -> `closed/`); only what the move to
`closed/` records differs. That record is in the body for every kind: a
filename has to stay stable, while the outcome it would carry can change
under it.

### Why there is no `spike` kind

A `spike` is a question asked by trying something: is this possible, and what
happens if we do. Separating it from `plan` needs a rule that says which of
the two an item is, because both are pieces of work sharing one state machine,
and the rule has to be applied afresh every time.

The rule that was tried -- is doing nothing an acceptable answer? -- is not
reliable enough to lean on. A `plan` whose first phase is an experiment can
legitimately end in "do not build it", and a `spike` that finds a path forward
simply becomes the start of one. The author has to guess which kind the work
will turn out to be before doing the work that would tell them.

So there is one kind for work, and the shape of the work is a matter for its
body: a plan whose first phase is an investigation says so, and the `##
Outcome` records what was learned whether the phases ran or the answer was no.
Kinds should be few enough that choosing one is not itself a judgement call;
the criterion quoted above says as much, since it adds a kind only when items
of that kind accumulate on their own.

### Why there is no `memo` kind

A `memo` would have been a kind that is never closed. That is the only
property it needs, and it is not a property of a kind: an item written down
before it is worth doing is an ordinary item that happens to be idle. Keeping
`memo` would have made it the one kind with its own rules, and every reader of
`nokori` would have had to learn why. One rule is worth more: an item is
something with a kind, and everything is closed the same way.

### Why the state is the directory, not a `Status` field

A field inside the file can disagree with where the file is. A file in
`closed/` with `Status: open` leaves the reader to decide which one is true,
and it takes a rule to say the directory wins -- a rule that has to be
remembered whenever a file is edited by hand.

Keeping the state in one place removes the question. `ls backlog/` and `ls
closed/` tell the whole story. The cost is that a state change is a move, and
that `closed/` has to record the outcome in the body rather than in the path.

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
`closed/`, and the record of deciding not to do it never appears.

So an item is open or closed, and `backlog/` means what it says: everything
not closed, whether or not anybody is working on it. If a state later turns
out to earn its place, adding one is easy; removing one costs the user a
rename of every file in it, which is why the smaller set goes first.

### Why `closed/` is not edited

A file in `closed/` records what was concluded at a particular time. Rewriting
it would make the record show what is known now instead, and the reader could
no longer tell which parts were the original judgement. Corrections and
additions are appended for that reason: the earlier text stays legible and the
revision is dated by git.

### Why `close` and not `finish`

`finish` says the work is over. For a `bug` that turns out not to be a bug, and
for an `rfc` that is turned down, no work was finished -- the item merely
stopped being open, which is what `close` says and all it says. It also names
the state the item is left in, so the command and the directory cannot drift
apart.

### Why `create` and not `open` or `new`

`open` would have paired with `close`, but it already means "show this file"
in most editors and browsers, and `nokori open <kind> <slug>` would read as an
attempt to open something that is not there yet. `new` is unambiguous but pairs
with nothing; `create` is what `gh issue create` and `gh pr create` call the
same step, so a reader who has used `gh` already knows the word.

The pair `create`/`close` is not symmetric, and that is accepted: only the
closing command has to agree with a state the item ends up in, because that is
the state the directory names. Nothing about creating an item is a state.

### Why a move checks that the file is clean

A move and a content change in one commit make the history unreadable: the
commit that says "close this item" also rewrites what the item said, and the
rewrite is what a later reader wants to see on its own. `cargo publish`
refuses a dirty tree for the same reason, and the check here is narrower only
because an item is not the whole crate -- what matters is the file being
moved, not the working tree around it.

The check reads `git status --porcelain -- <file>` for that one path, and an
untracked file counts as dirty: `create` followed by `close` before the first
commit is refused, which is the flow described above working as intended. What
the check needs is not "the file is clean" but "there is a commit holding what
this file says", and an untracked file has none.

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
and elsewhere: which kind to use, what `backlog/` holds, what `close` does to
a file. Printing all of that would make the one output nobody
re-reads long. So `init` stops at the next step, and the reference material is
reached through `explain` from wherever it is needed.

### Why the outcome stays out of the filename

An outcome is not final just because an item reached `closed/`. An `rfc` that
was
accepted can be overturned by a later one, and a `bug` closed as `not-a-bug`
can turn out to be the user's own mistake. A filename that carried the outcome
would have to be renamed each time, and a rename breaks every reference to it
at once; an `rfc` accepted early and implemented later would also have to spend
time under a name that guesses its own future.

So the outcome is written once, in the body, and left as it was: the later item
says that it supersedes the earlier one. This is the same rule that keeps the
kind in the filename but out of the body -- a name is for finding a file, and
what a file means is for reading it.
