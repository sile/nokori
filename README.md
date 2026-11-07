nokori
===

- backlog management cli tool
- markdown file based
- commands:
  - nokori init
    - (optional) add exclude to cargo.toml if exists
    - create backlog/ dir and backlog/README.md
    - README.md niha saiteigen no naiyou nomiwo kijutusuru 
      - nokori version kousin no tokino maintenance cost wo sagerutame
    - kawarini nokori new de tukuru template file no naiyou wo self descriptive nis teioku
      - kaku section ni <!-- TODO: ... --> wo kaiteoite, sono section de yarubekikoto / kiwotukerubekikoto wo kaiteoku
      - riyousyaha, section no kisaiga owattara, sono TODO wo kesu (koremo TODO: ni tejun tosite kaiteoku)
  - nokori new (or create) TYPE
    - ITEM_TYPE=rfc,bug,plan,spike
    - rfc: sin kinou ya kyodou henkou no teian
      - jissou syousai ha hukumesuginai (soreha jissoujini kimereba iinode, jissouni tuiteha hontou ni hituyouna jouhounomiwo kisaisusuru)
      - rule
        - dekirudake self-contained na naiyounisuru (hokano repository tokakarano youkyuudemo, dekirudake ippanntekina youkyuu ni otosikonnde kisaisuru)
    - bug: bug syusei
    - plan: item no taiouga omoi baaiya, hukusuu no phase ni wakete torikumu baai(kottiha umbrella teki) niha, kono item wo tukuttemo ii
      - aruteido gutaitekina tejun ya nagarewo kisaisurutameno mono
      - tugino major release madeno milestone, tekina tukaiktademoii
      - sagyou wo tyuudan site atode saikai sitari, betuno hitoni hikitugitaitoki
      - suguni owaru tannki tekina sagyoudeha huyou
    - spike: tugino housin wo kimeru tameno tyousa / sisaku (kekka ha done/ ni nokosu)
    - (draft/: kind ga kimatteinai zattana memo ha koko ni oku. kind mo filename ni fukumenai)
  - nokori finish or close or settle or resolve else
    - file no idou to status henkou wo yarutameno command?
      - status ni awaseru nara 'accept' tokamo ari?
    - mada syousai ha humei (somosomo hituyouka douka, tokamo)
- status:
  - joutai ha file no iti basho de arawasu (draft/, backlog/, ongoing/, pending/, done/).
    file nai no Status field ha motanai
  - jissouni tyakusyu igo ha, kihontekini kaku ITEM no kizon no nakami ha syuuseisinai (tuiki ga omo)
- dir structure:
  - backlog/
    - README.md
    - draft/ (kind ga mada kimatteinai mono)
    - ongoing/ (tyakusyu sumi no mono)
    - pending/ (tyuudann)
    - done/ (kettyaku zumi)


Decided
-------

- `nokori init` writes `backlog/README.md` too, but its content stays minimal.
- An item's state is expressed by the directory it is in, never by a `Status`
  field inside the file. The directories are `draft/`, `backlog/` itself,
  `ongoing/`, `pending/`, and `done/`.
- v1 has four item kinds: `rfc`, `bug`, `plan`, and `spike`. There is no
  `memo` kind; a rough note is a file in `draft/` that has not been promoted to
  a kind yet.
- `draft/` holds notes that have no kind. Their filenames carry no kind either
  (`YYYYMMDD-<slug>.md`), because the kind is chosen when the note is promoted
  to `backlog/` and is not known before then.
- `ongoing/` is a claim: an item is there because someone has started on it, so
  nobody else picks it up. For an `rfc` it means *provisionally* accepted --
  acceptance is settled only when the change merges.
- `done/` does not fork by outcome. It holds every settled item, and for `rfc`
  alone the outcome is written into the filename
  (`YYYYMMDD-rfc-accepted-<slug>.md` / `YYYYMMDD-rfc-rejected-<slug>.md`).
  `bug`, `plan`, and `spike` keep their name unchanged when they move there.
- A `bug` that cannot be fixed for now goes to `pending/`. One that is decided
  never to be fixed is not a bug but a documented behavior: the behavior is
  written into the public docs and the item goes to `done/`, the same way a
  report that turns out not to be a bug does.


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

A rough note has no state to carry. Moving it to `done/` would claim something
was concluded when nothing was, and it would then sit there as a record of
nothing having happened.

Such a note lives in `draft/` instead. It is a file with no kind: it has not
been broken down far enough for the kind to be known, so both the location and
the filename say so (`YYYYMMDD-<slug>.md`). Promoting it is a move to
`backlog/` that adds the kind it turned out to be; a note that never matters is
deleted, and because it never reached `done/` nobody has to explain it.

This replaces a `memo` kind. A kind that cannot move would have been the only
exception in the state machine, and every reader of `nokori` would have to
learn why one kind behaves differently. Folding it into `draft/` keeps one
rule: an item is something that has a kind, and everything with a kind moves.

### Why the state is the directory, not a `Status` field

A field inside the file can disagree with where the file is. A file in `done/`
with `Status: draft` leaves the reader to decide which one is true, and it
takes a rule to say the directory wins -- a rule that has to be remembered
whenever a file is edited by hand.

Keeping the state in one place removes the question. `ls backlog/done/` and
the filename tell the whole story. The cost is that a state change is a move,
and that `done/` has to record the outcome somewhere; for `rfc` that record is
a word in the filename rather than a field.

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

### Scope of `finish`

Breaking down what `settle.sh` in the reference repositories does:

| step | what it does | needs `gh` | note |
| ---- | ------------ | ---------- | ---- |
| a | check that the working tree is clean | no | |
| b | read the merge commit and head branch for a PR number | **yes** | `gh pr view --json mergeCommit,headRefName` |
| c | switch to `main` and fast-forward it | no | git only |
| d | rewrite `Status` and append `## Outcome` | no | this is the substance |
| e | `git mv` into `done/` | no | |
| f | commit, push, delete the merged branch | no | push needs the network |

**Only step b depends on `gh`.** Without it, the rest of a-f is plain git and
plain file editing.

That narrows the design of `finish` to three options:

**A: local only (file operations)**

- Rewrite `Status`, append `## Outcome`, move into `done/`.
- No commit and no push.
- Pros: no dependencies, easy to test, a clear boundary for `nokori`.
- Cons: the tool cannot honour the "settle in a single commit" rule the
  reference repositories rely on.

**B: local plus git (no `gh`)**

- As A, plus `git mv`, the clean-tree check, and `git commit`.
- The PR line in `## Outcome` comes from an optional `--pr`; with no `--pr`,
  the line is omitted rather than written empty.
- Pros: no `gh`, yet the history-following and single-commit rules hold.
- Cons: the PR line is missing in environments that do not pass one.

**C: full `settle.sh` replacement (with `gh`)**

- As B, plus `gh pr view` for the merge commit and head branch, then push and
  branch deletion.
- Pros: replaces the manual script outright.
- Cons: requires `gh`, and pushes `nokori` toward shelling out to git and gh.

**Recommendation: B.**

Most of `settle.sh`'s complexity sits in b (the `gh` dependency) and f (push and
branch deletion). Removing them leaves the tool with a single responsibility:
move the file correctly and write the right body. `gh` support can be added
later as `finish --pr N`.

### What `finish` is called

- Whether to settle on `finish`, `close`, `settle`, or `resolve` is still
  undecided.
- The reference repositories use `settle.sh` throughout.
- `accept` was also considered, but it names the outcome of an RFC rather than
  the operation.


