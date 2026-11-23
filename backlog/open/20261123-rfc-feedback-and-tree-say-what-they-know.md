# RFC: feedback and tree say what they know

## Summary

Three things a run already knows never reach the file or the output: the
feedback item has no line saying what it is about, a line of `tree` does not
say whether its item is open or closed, and a walk from every open item is a
loop in a shell script rather than a command.

This RFC makes them reach it. The feedback template gains a `## Summary`,
which `nokori feedback` fills with the first line of the words. A closed item
is drawn in `tree` and in the `close` report with its state, so that a reader
of a settled item can see what came of it and what is still hanging off it.
And `tree` given no item walks every open one, each as a root.

## Motivation

A file named `20261122-feedback-163856.md` says nothing until it is opened.
The name is the day and the second because a slug is a summary, and this run
has not read the words -- but it has the words in hand, and the first line of
them is the summary it would have written. The cost is paid every time
somebody lists `backlog/open/`: to know what is waiting, they open each file.
Today's template makes it worse by cutting the title line as well, so the file
that most needs a first sentence is the one with none.

The same run that writes the file says so: `Created
backlog/open/20261122-feedback-163856.md`, three lines of what it belongs to,
and nothing about what was said. The tool that read the words is the one that
could have answered.

`tree` has a smaller version of it. A tree from a closed item, walked
inverted, is how a settled decision is read back -- what came of it, and what
was written about it since. Every line of that tree is an item whose state the
run knows, from the directory it read the item out of, and none of them says
it. The reader who cannot tell a closed item from an open one has to open the
file to find out whether the work under it is still running.

The missing walk is the one used most: what are all the open items, and what
does each need. Today that is a `for` loop over `backlog/open/*.md` calling
`nokori tree`, which is the scanning work the tool already does once per item.
`cargo tree` answers the same question with no argument, and a backlog with
nothing closed yet prints one item per line.

## Design

### `## Summary` in a feedback item

`templates/item_feedback.md` gains a `## Summary` section above `##
Feedback`, with the same `NOKORI_TODO` comment the other kinds have. It is a
section rather than a title line for the reason the title line is cut: a
heading is a phrase the writer of the file chose, and this run has only the
words. A section holds a sentence the reader scans a list for.

`nokori feedback` fills it as well as `## Feedback`: the first line of the
words that is a paragraph of its own -- not blank, not a heading, a list item,
a quote, or a fence -- with that line's leading and trailing space removed.
The words themselves are written under `## Feedback` unchanged, so nothing the
reporter said is lost or reworded. A run whose words are all of those things
leaves `## Summary` empty rather than guessing, and `check` then says so like
any other empty section.

`nokori create feedback` writes the same template as it does today, with the
comment in both sections, since a writer there has a phrase before the words.

### Closed items are marked in `tree`

`Item` gains `closed: bool`, set by the edge from the directory the file was
read out of. A line whose item is closed is drawn with `*` after the branch
and a space before the name:

```
20261122-rfc-tree-shows-one-direction
├── 20261122-rfc-link-text-keeps-the-date
└── * 20261115-rfc-say-when-a-link-is-written
```

`*` rather than a word, because it is one character on every terminal, it sits
where `cargo tree` puts its markers rather than in the name (so the name is
still the link text, unchanged), and it is easy to see in a column before
reading anything else. The root is marked like any other line: an inverted
tree from a closed item starts with a marked root, which is the fact the run
was asked about.

`nokori close` prints the same mark in its report of open items, which are
open ones and so unmarked; the shape is the one place it is written, so the
two prints stay alike.

### `tree` with no item walks the open ones

`nokori tree` with no `<ITEM>` prints every item in `backlog/open/`, sorted by
path, each as a root, then a blank line, then the next. The blank line is
what says where one tree ends and the next begins, since a root is drawn
without a branch.

`--invert` works with no item too, and reads as one question: every open
item, and what is pointed at it. That is the list of proposals, bugs, and
plans that already have something written about them.

The argument becomes optional (`[<ITEM>]`), and what it means to give it is
unchanged. A backlog with nothing open prints nothing and says so on standard
error rather than printing an empty screen with a success status.

The walk is still one call per root and not one joining walk: two open items
that share a closed item are two trees, because the roots are separate
questions. A heading per root is not printed either: the first line of a tree
is its root, and a heading would be a second name for it.

## Alternatives

**A title line instead of `## Summary`.** `# Feedback: <first line>` reads
better in a listing than a heading three lines down. It loses because it
repeats the words: the first line is in `## Feedback` as well, and `create`
keeps the title line it has, so the file would say the same sentence twice in
two shapes. A section is filled where every other kind has one, and the reader
who knows `bug` and `rfc` already knows where to look.

**A slug made from the words.** The file would be named
`20261123-feedback-panic-on-empty-input.md`, which is what a listing wants
most. It loses because a name is permanent and a summary is not: the words are
the reporter's first draft, the discussion that follows usually renames the
thing, and a slug that disagrees with the summary underneath it is worse than
a time. `create` has a slug because a writer chose it on purpose.

**`(closed)` in place of `* `.** It says the state in English rather than
leaving it to be learned. It loses on width and alignment: every line's name
shifts right by a column that is blank on most lines, and a tree from an open
item -- the common case -- pays for a mark it does not use. `*` is one
character and a column of its own.

**A separate `nokori open` command.** `open` lists the open items and
`tree` walks one; two names for the two questions. It loses because the
question is the same walk over the same graph, the same way `--invert` was
folded into `tree` rather than given a name. The list is `tree` with the
smallest possible argument set, not a command.

**Walk every item, open and closed, when no item is given.** It is the whole
graph at once, a truer `nokori tree`. It loses because the closed items would
be walked as roots and then again under the open ones, which is the whole
backlog printed on every run; the open ones are the question a run without an
argument is asking.

## Open questions

Whether `*` is the right mark. It is one character and it is not part of a
name, which is what the line needs; whether a closed item should also read as
closed in the root line of a forward tree from an open item is answered by the
rule "the edge knows, so the line says": yes.

Whether the blank line between roots is enough of a separator. A tree's last
line and the next root are different shapes already -- one is indented, the
other is not -- so the blank line is for the eye rather than the parser, and
one line is cheap.

Whether the summary belongs in `create feedback` too. It does: the template
is one file for both entries, and the comment says what the section is for.

## Outcome

