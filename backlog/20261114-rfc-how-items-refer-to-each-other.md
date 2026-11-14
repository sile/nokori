# RFC: refer to an item by a link to its file

## Summary

An item refers to another item with a markdown link whose text is the file
name and whose target is the path to that file from the referring file's own
place: `[20261114-rfc-how-items-refer-to-each-other.md](20261114-rfc-how-items-refer-to-each-other.md)`.

A link is what makes the rule in
[rfc-never-cite-an-open-item-from-a-closed-one](20261114-rfc-never-cite-an-open-item-from-a-closed-one.md)
checkable. That
rule forbids a closed item from citing an open one, and with a name alone
`close` could only guess at what a name meant; with a link, `close` can see
where it points and refuse the file until it is fixed.

The cost is that a link carries the path of the file it points at, and
`close` moves files, so a link written while both items are open breaks when
one of them closes. That cost is paid before the item moves: `close` refuses
a file holding a link that does not resolve, so a broken link cannot reach
`closed/`. It is fixed in the open item that holds it, not left behind.

The rule itself lives in `nokori explain references`, and each template's
comment carries a line and an example pointing there.

## Motivation

A slug is not a name. Every item's file name starts with the date it was
created and the kind, and the slug is only the tail: `plan-split-the-lexer`
is the slug of `20261114-plan-split-the-lexer.md`. Taken alone the slug is
ambiguous -- two items created on different days may have the same slug, and
nothing in `plan-split-the-lexer` says whether it is that plan or an RFC
from last year with the same words.

A reader who has the slug and the repository can usually find the file, and
that is the case the current rule was written for. The case it misses is a
reader with no repository to search: a pasted quotation, a commit message
reviewed out of context, or a discussion where one person says "see
split-the-lexer" and the others have to be told which file that is.

The file name in the link's text settles both. It is unique within the
repository and it is what `ls` and tab completion offer, so a reader who has
it can open the file without a search, and a reader who has the whole file
in front of them can follow the link instead.

A link is also checkable, which a name in backticks is not. A rule that can
only be checked by reading has to be written so that a human can judge it at
`close` time; a link has a target that a program can resolve, so the rule
about which direction a reference may point can be enforced rather than
remembered. That is why the link is not just a convenience here -- it is what
makes the neighbouring
[rfc-never-cite-an-open-item-from-a-closed-one](20261114-rfc-never-cite-an-open-item-from-a-closed-one.md)
enforceable.

## Design

A reference is a markdown link:

```markdown
[rfc-how-items-refer-to-each-other](20261114-rfc-how-items-refer-to-each-other.md)
```

The text is the item's kind and slug, without the date. The target is the path
to that file from the directory of the file holding the link, which depends
on where the two items are:

| referring item | target item | target |
| --- | --- | --- |
| `backlog/` | `backlog/` | `20261114-plan-split-the-lexer.md` |
| `backlog/` | `closed/` | `closed/20261114-plan-split-the-lexer.md` |
| `closed/` | `closed/` | `20261114-plan-split-the-lexer.md` |
| `closed/` | `backlog/` | forbidden; see
  [rfc-never-cite-an-open-item-from-a-closed-one](20261114-rfc-never-cite-an-open-item-from-a-closed-one.md) |

Write the path that is correct now. When an item named by an open one closes,
the link breaks, and it is repaired in the open item that holds it -- which
happens before that item itself closes, because `close` refuses a link that
does not resolve.

No URL and no `[[...]]`. A URL points outside the repository and needs a
network to follow; `[[...]]` is not markdown, so nothing renders it and no
tool can check it.

The date is left out of the text because it carries nothing a reader needs:
when an item was created is not why it is being referred to, and both the
target and `ls` give it. The kind stays, because it is what tells a reader
whether the thing referred to is a proposal, a piece of work, or a report --
which is most of what the sentence around the link is saying.

A text of the slug alone would read even better, and is what a link makes
possible: the target is unique, so nothing is lost while the link is a link.
It is the link coming apart that loses it -- a paste into a plain-text
channel, a `git grep` output, a quotation in a review -- and what is lost is
the kind, not the identity. Two items with the same kind and slug on
different days would collide as text; that is accepted, since the text is
only a label and the target is the reference.

`close` checks links and refuses the move when one of them points at a file
that does not exist. A link to an item that is not created yet is therefore
not writable before the item exists: write what would settle the item, and
let the item that settles it link back.

The rule is written in three places, for three different moments.

`nokori explain references` says it in full: the link form, the table of
targets, the two things not to write, and why the close check exists. It is
the one place to change when the rule changes.

Each template's comment says it in a line, with one example and a pointer:

```
To refer to another item, link it:
[rfc-how-items-refer-to-each-other](20261114-rfc-how-items-refer-to-each-other.md).
Run `nokori explain references` for where the path points from each
section.
```

That is in the comment of the section a reference is most often written in:
`Outcome` for a closed item, `Motivation` for the `feedback` or `rfc` a
change answers. A writer meets the rule in the file they have open, in the
section they are filling in, without having been told to go read something
first -- which is what a template is for. The detail is not repeated there,
because a rule copied into four templates is four rules.

No template gains a section for references. A section would invite a list of
links, and a list reads as structure rather than as the sentence a reference
belongs to.

## Alternatives

**Write the file name without linking it.** The name alone is enough for a
reader with the repository, it never goes stale, and no item ever has to be
edited because another one moved. It also makes the rule in
[rfc-never-cite-an-open-item-from-a-closed-one](20261114-rfc-never-cite-an-open-item-from-a-closed-one.md)
a matter of judgement: `close` sees a backticked string and cannot tell
whether it names
an item in `backlog/`, because the string carries no location. The rule would
then be one a writer has to remember at the moment they stop rereading the
file, which is the worst moment to rely on memory.

**Keep referring by slug.** This is what the plan template asks for now, and
it is shorter to write. It leaves the ambiguity above in place, and it leaves
the reader to search -- which is the case the change is for.

**Keep the files in place and refer by path.** Under this, `backlog/` would
hold every item and the state would be something else: a `closed/` directory
of symlinks pointing into `backlog/`, or a line in the file rather than a
move. A path would then never move, so writing one would be safe.

It trades one kind of fragility for another. `git` tracks a symlink as a
path string, not as a link, so a reader who clones on a system without
symlinks gets a tree that looks wrong; and every command that walks the
backlog would have to know that the directories are views rather than
truth. The state being where the file is, is what makes `nokori` small: the
state is visible in `ls` without a symlink to follow, and `close` is one
`rename`.

**Add a `rename` command so a changed slug does not break links.** A slug
that changes after the file was written is a real cost, and `rename` would
make it one command instead of `git mv` plus an edit of the title and of
every link that points at the file. It is the one alternative that gets
stronger under links rather than weaker: with a name alone, `rename` could
not fix the references at all, because it could not tell a reference from any
other string.

The reason not to do it is size, not disagreement. A slug is fixed at
`create` time and later items link to the name they can see, so a rename
means editing the links in other open items, which `close` already forces
somebody to do when they break. A command that did it in one step would save
that editing on the day somebody renames something -- and `git mv` plus a
hand edit is what it replaces, which is a rare operation being made shorter.

## Open questions

None. Two questions were open when this was written and both are answered
above: `backlog/README.md` says nothing about references, because a rule in
two places is a rule that falls out of step and `README.md` is not the file a
writer has open; and the rule about links is stated, in the templates, where
the writer is.

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

Say whether the RFC was accepted or rejected, and why. When it is accepted,
name the change that implements it, or the item that does, if the work happens
somewhere else.

An accepted RFC can still be overturned later. Say so in the later item and
cite this one; do not edit this file afterwards to follow it.

Once this section is written and committed, treat the file as closed: do not
edit any part of it again, and append nothing later. If something turns up
later, leave the file as it is and open a new item that cites this one and
acts on it.

Delete this comment when you write the section. -->
