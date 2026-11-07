bkl
===

- backlog management cli tool
- markdown file based
- commands:
  - bkl init
    - (optional) add exclude to cargo.toml if exists
    - create backlog/ dir and backlog/README.md
    - README.md niha saiteigen no naiyou nomiwo kijutusuru 
      - bkl version kousin no tokino maintenance cost wo sagerutame
    - kawarini nkl new de tukuru template file no naiyou wo self descriptive nis teioku
      - kaku section ni <!-- TODO: ... --> wo kaiteoite, sono section de yarubekikoto / kiwotukerubekikoto wo kaiteoku
      - riyousyaha, section no kisaiga owattara, sono TODO wo kesu (koremo TODO: ni tejun tosite kaiteoku)
  - bkl new (or create) TYPE
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
    - memo: free format no memo
      - jissaini sagyouni hairumaeniha, hokano item ni otosikon de kara torikumu noga suisyou sareru
      - goku keibina monoha reigai
    - TODO: hokano kubun ga hituyouka, ya ueno bunrui nado ga tekisetuka, ha kentousuru
  - bkl finish or close or settle or resolve else
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

- `bkl init` writes `backlog/README.md` too, but its content stays minimal.
- `pending/` is part of v1. An item has one of three states: `open`,
  `postponed`, `done`.


Open questions
--------------

### ITEM_TYPE (rfc, bug, plan, memo)

The reference repositories (kk, termnix, tuinix) use only `rfc` and `bug`.
`tuinix/backlog/README.md` states the criterion for adding a kind:

> When such items start to accumulate -- and the tell is having to write an
> RFC whose Summary already says what will be done -- add a third kind then.
> It would carry a two-value state (`open` / `done`) instead of the
> `draft`/`accepted`/`rejected` and `open`/`fixed`/`not-a-bug` vocabularies,
> because it records work rather than a decision.

Reduced to two points, a kind is either (a) a decision or (b) a piece of
work:

- (a) is what `rfc` and `bug` carry (`draft`/`accepted`/`rejected` and
  `open`/`fixed`/`not-a-bug`).
- (b) needs only `open` and `done`.

Against that criterion:

| kind | verdict       | reason |
| ---- | ------------- | ------ |
| rfc  | keep          | asks "what should the API be". States: `draft` / `accepted` / `rejected` / `postponed` |
| bug  | keep          | asks "is the contract violated". States: `open` / `fixed` / `not-a-bug` |
| plan | **keep**      | an umbrella for work split into phases. Not a decision but a record of work, and `open` / `done` is enough |
| memo | **postpone**  | free format. Neither a decision nor a piece of work, so no state can be defined. Whether it folds into an existing kind is a question for after some use |

Proposal: v1 has `rfc`, `bug`, and `plan`; `memo` is left out until there is
something to learn from.

Keeping `plan` has two effects:

- The `finish` section above ("move the file and change the status") reduces to
  a two-value `open` / `done` transition.
- The settle dispatch gains one more line and nothing else.

`memo` describes itself as something written before it is broken down into
other items. Moving it to `done/` therefore involves a judgement of some kind,
which makes it awkward to treat as a kind that carries a state.

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
- Pros: no dependencies, easy to test, a clear boundary for `bkl`.
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
- Cons: requires `gh`, and pushes `bkl` toward shelling out to git and gh.

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


