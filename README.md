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
    - ITEM_TYPE=rfc,bug,plan,memo
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
    - memo: free format no memo (joutai wo motanai)
      - jissaini sagyouni hairumaeniha, hokano item ni otosikon de kara torikumu noga suisyou sareru
      - goku keibina monoha reigai
  - nokori finish or close or settle or resolve else
    - file no idou to status henkou wo yarutameno command?
      - status ni awaseru nara 'accept' tokamo ari?
    - mada syousai ha humei (somosomo hituyouka douka, tokamo)
- status:
  - TODO
  - ITEM_TYPE goto ni kotonaru status wo motu?
  - jissouni tyakusyu igo ha, kihontekini kaku ITEM no kizon no nakami ha syuuseisinai (tuiki ga omo)
- dir structure:
  - backlog/
    - README.md
    - done/ (if need)
    - pending/ (if need)


Decided
-------

- `nokori init` writes `backlog/README.md` too, but its content stays minimal.
- `pending/` is part of v1. An item has one of three states: `open`,
  `postponed`, `done`.
- v1 has three item kinds that carry a state: `rfc`, `bug`, and `plan`.
- `memo` is a fourth kind that carries **no state**. It lives directly under
  `backlog/` with a `memo-` filename prefix and is never moved to `pending/`
  or `done/`; `finish` leaves it alone. A memo worth keeping beyond that is
  promoted by hand into a proper item (`rfc`, `bug`, or `plan`), because a
  memo records something noticed rather than work that can be concluded.


Rationale
---------

### How `memo` differs from the other kinds

The reference repositories (kk, termnix, tuinix) use only `rfc` and `bug`.
`tuinix/backlog/README.md` states the criterion for adding a kind:

> When such items start to accumulate -- and the tell is having to write an
> RFC whose Summary already says what will be done -- add a third kind then.
> It would carry a two-value state (`open` / `done`) instead of the
> `draft`/`accepted`/`rejected` and `open`/`fixed`/`not-a-bug` vocabularies,
> because it records work rather than a decision.

That criterion separates kinds into (a) a decision and (b) a piece of work.
`rfc`, `bug`, and `plan` all fit, and `plan` is the one added; its two-value
`open` / `done` transition is the whole of what `finish` does for it.

`memo` is the exception: it is neither a decision nor a piece of work, but
something noticed and written down before it is broken down. That is why it
carries no state. Giving it `open` / `done` would mean moving it to `done/`
when nothing was concluded, and it would then sit there with no record of
anything having happened. A memo that turns out to matter is promoted by hand
into an `rfc`, `bug`, or `plan`; one that does not simply stays where it is.

So `memo` lives directly under `backlog/` like an open item, told apart by its
filename prefix. The rule that nothing but items live in `backlog/` still
holds: a memo is an item, just one that does not move.

### Naming prefix for `memo`

Every kind is carried by a filename prefix (`YYYYMMDD-<kind>-<slug>.md`), so
`memo` needs one too: `YYYYMMDD-memo-<slug>.md`.

The alternative was a separate `backlog/memo/` directory. That would break the
rule that the prefix, not the directory, carries the kind, and add a third
directory next to `pending/` and `done/` whose meaning ("not a state") does not
fit the other two ("a state"). The prefix keeps the tree shallow and the rule
intact.


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


