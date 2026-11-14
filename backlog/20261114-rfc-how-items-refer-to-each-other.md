# RFC: refer to an item by the name of its file

## Summary

An item refers to another item by the name of its file, without the
directory: `20261114-rfc-how-items-refer-to-each-other.md`. Today the only
reference that is anywhere written down is the plan template's `Items`
section, which asks for the slug alone, and a slug names an item only if you
already know which date it was created on and which kind it is.

This settles the written form. It does not move the files: the reason a path
is not written down is that `close` moves the file, and the answer to that is
to write the name that survives the move, not to keep the file in place.

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

The file name settles both. It is unique within the repository, it survives
`close`, and it is what `ls` and tab completion offer, so a reader who has it
can open the file without a search.

## Design

An item's name is its file name without the directory:
`20261114-plan-split-the-lexer.md`. A reference to an item is that name.

- Write the whole name, date and kind included. `split-the-lexer` is not a
  reference; it is a fragment the reader has to resolve.
- Write the name, not the path. `backlog/20261114-plan-split-the-lexer.md`
  and `closed/20261114-plan-split-the-lexer.md` stop pointing at the file
the moment `close` moves it; the name alone points at it in both states.
- Do not link it. No markdown link, no URL, no `[[...]]`. The name is enough
  for a reader to find the file, and a link would have to be rewritten by
  hand in every item that points at a file that moved.

`close` does not check references. A reference that names a file that does
not exist is not an error, because the file may be about to be created by
another item that is still open; a check belongs in a lint command that
reports what it finds, not in the command that moves a file.

The plan template's `Items` section says "by slug" today, and it changes to
say "by file name" with the same words. The RFC and bug templates gain no
section for references: a reference goes where it belongs in prose, and a
section would invite a list of links that reads as structure.

## Alternatives

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

**Add a `rename` command so a changed slug does not break references.** A
slug that changes after the file was written is a real cost, and `rename`
would make it one command instead of `git mv` plus an edit of the title.
But under the design here a changed slug changes the file name, and the
name is what references are written with, so the references break either
way. `rename` would make the edit of the one file easier and would not make
the references any safer, which is what the request was about.

## Open questions

Does the table in `backlog/README.md`, or anything else `init` writes, need
to say how to name an item? A reader meets the rule in `explain flow`
today; if `init` says it too, there are two places to keep in step.

Is the rule about links worth stating? A reader who writes a markdown link
has not broken anything yet, and the rule says what to do instead. The
alternative is to leave it out and let the first broken link teach it.

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
