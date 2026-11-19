#![forbid(unsafe_code)]

const BACKLOG_README_TEMPLATE: &str = include_str!("../templates/backlog/README.md");
const OPEN_README_TEMPLATE: &str = include_str!("../templates/backlog/open/README.md");
const CLOSED_README_TEMPLATE: &str = include_str!("../templates/backlog/closed/README.md");

/// The directory the backlog is kept in, and the two directories under it that
/// hold the two states. Both states are below `backlog/` rather than beside it,
/// so that the backlog is one directory to move and neither state sits next to
/// the source tree. They are also at the same depth, which is what lets a
/// relative link read the same from either one.
const BACKLOG_DIR: &str = "backlog";
const OPEN_DIR: &str = "backlog/open";
const CLOSED_DIR: &str = "backlog/closed";

/// What `init` writes, each with the template for its content.
const README_FILES: &[(&str, &str)] = &[
    ("backlog/README.md", BACKLOG_README_TEMPLATE),
    ("backlog/open/README.md", OPEN_README_TEMPLATE),
    ("backlog/closed/README.md", CLOSED_README_TEMPLATE),
];

/// The section `close` writes into, and the marker a template leaves in its
/// sections. The marker is a whole word of its own rather than a `TODO` a
/// comment happens to start with, so that an item may write about a `TODO` in
/// the code it reports on without `close` reading that as unfinished work.
const OUTCOME_SECTION: &str = "## Outcome";
const TODO_MARKER: &str = "NOKORI_TODO:";

/// The kinds `create` accepts, each with the template written into the file.
const KINDS: &[(&str, &str)] = &[
    ("rfc", include_str!("../templates/backlog/rfc.md")),
    ("bug", include_str!("../templates/backlog/bug.md")),
    ("plan", include_str!("../templates/backlog/plan.md")),
    ("feedback", include_str!("../templates/backlog/feedback.md")),
];

/// The kind `feedback` writes, and the two markers in its template that the
/// words replace rather than an answer to.
///
/// The title line is cut out whole: the words are the item, and a title made
/// of their first line is that line repeated a little above itself. `create`
/// keeps the line, because a writer there knows the phrase before the words.
const FEEDBACK_KIND: &str = "feedback";
const FEEDBACK_TITLE_PREFIX: &str = "# Feedback:";
const FEEDBACK_SECTION: &str = "## Feedback";

/// The topics `explain` prints, each with the text. They carry the material a
/// command cannot: the rules about the states and the kinds, which read the
/// same whatever command the reader arrived from. `overview` comes first
/// because it is the one to read first: the other three state rules that only
/// make sense once the reader knows nothing here is enforced.
const TOPICS: &[(&str, &str)] = &[
    ("overview", include_str!("../templates/explain/overview.md")),
    ("kinds", include_str!("../templates/explain/kinds.md")),
    ("flow", include_str!("../templates/explain/flow.md")),
    (
        "references",
        include_str!("../templates/explain/references.md"),
    ),
];

fn main() -> noargs::Result<()> {
    let mut args = noargs::raw_args();
    args.metadata_mut().app_name = env!("CARGO_PKG_NAME");
    args.metadata_mut().app_description = env!("CARGO_PKG_DESCRIPTION");

    if noargs::VERSION_FLAG.take(&mut args).is_present() {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    noargs::HELP_FLAG.take_help(&mut args);

    let _ = try_run_init(&mut args)?
        || try_run_create(&mut args)?
        || try_run_feedback(&mut args)?
        || try_run_check(&mut args)?
        || try_run_close(&mut args)?
        || try_run_tree(&mut args)?
        || try_run_explain(&mut args)?;

    if let Some(help) = args.finish()? {
        print!("{help}");
    }

    Ok(())
}

fn try_run_init(args: &mut noargs::RawArgs) -> noargs::Result<bool> {
    if !noargs::cmd("init")
        .doc("Set up a backlog in this repository")
        .take(args)
        .is_present()
    {
        return Ok(false);
    }

    if args.metadata().help_mode {
        return Ok(true);
    }

    init_backlog()?;

    for (path, _) in README_FILES {
        println!("Created {path}");
    }
    println!();
    println!("Next, create an item:");
    println!();
    println!("  nokori create <kind> <slug>");
    println!();
    println!("Run `nokori create --help` to see the usage and");
    println!("`nokori explain kinds`");
    println!("to choose a kind.");

    Ok(true)
}

fn try_run_create(args: &mut noargs::RawArgs) -> noargs::Result<bool> {
    if !noargs::cmd("create")
        .doc("Create a new item")
        .take(args)
        .is_present()
    {
        return Ok(false);
    }

    let kind: String = noargs::arg("<KIND>")
        .doc("Kind of item to create (`nokori explain kinds` lists them)")
        .example("bug")
        .take(args)
        .then(|a| a.value().parse())?;
    let slug: String = noargs::arg("<SLUG>")
        .doc("Short name for the item, used in its filename")
        .example("panic-on-empty-input")
        .take(args)
        .then(|a| a.value().parse())?;

    if args.metadata().help_mode {
        return Ok(true);
    }

    let path = create_item(&kind, &slug)?;
    let file = path.display();

    println!("Created {file}");
    println!();
    println!("Next, do the work and write it down: replace each `{TODO_MARKER}`");
    println!("comment with what it asks for, and delete the comment.");
    println!();
    println!("Run `nokori check {file}` to see what is left.");
    println!();
    println!("When nothing is, say what happened in `{OUTCOME_SECTION}` and close");
    println!("the item with `nokori close {file}`.");

    Ok(true)
}

fn try_run_feedback(args: &mut noargs::RawArgs) -> noargs::Result<bool> {
    if !noargs::cmd("feedback")
        .doc("Write down what somebody said, as a feedback item")
        .take(args)
        .is_present()
    {
        return Ok(false);
    }

    let path: String = noargs::arg("<PATH>")
        .doc("Repository to write the item in, given as the directory holding `backlog/`")
        .example("../other")
        .take(args)
        .then(|a| a.value().parse())?;
    let message: Option<String> = noargs::opt("message")
        .short('m')
        .doc("Take the words from this argument, or give `--file` or standard input")
        .take(args)
        .present_and_then(|a| a.value().parse())?;
    let source: Option<String> = noargs::opt("file")
        .short('f')
        .doc("Take the words from this file")
        .take(args)
        .present_and_then(|a| a.value().parse())?;

    if args.metadata().help_mode {
        return Ok(true);
    }

    let words = feedback_words(message, source)?;
    let path = write_feedback(std::path::Path::new(&path), &words)?;
    let file = path.display();

    println!("Created {file}");
    println!();
    println!("The item belongs to the repository it was written into, so it is read");
    println!("and closed there. Nothing is committed and the file was not opened.");

    Ok(true)
}

/// Reads the words from the one place they were given, and refuses the run
/// that would write an empty `## Feedback`.
///
/// The empty text is one error however it arrived, since the run that writes
/// it is the run that had nothing to say, and which flag was typed is not the
/// part to tell the writer about.
fn feedback_words(message: Option<String>, source: Option<String>) -> Result<String, String> {
    if message.is_some() && source.is_some() {
        return Err(
            r"`--message` and `--file` cannot be used together, since which one was meant
is not something this command can tell. Give the words one way."
                .to_owned(),
        );
    }

    let words = match (message, source) {
        (Some(words), _) => words,
        (None, Some(source)) => std::fs::read_to_string(&source).map_err(|e| {
            format!(
                r"failed to read the words from `{source}`: {e}

Check the path, or leave `--file` out to type the words instead."
            )
        })?,
        (None, None) => read_stdin()?,
    };

    let words = words.trim();
    if words.is_empty() {
        return Err(
            r"the words are empty, so the item would have nothing in it. Say what the
person said: with `--message`, with `--file`, or on standard input."
                .to_owned(),
        );
    }
    Ok(words.to_owned())
}

/// Reads the words from standard input, saying so first when it is a terminal.
///
/// At a terminal the read looks like a hang, and one line on standard error is
/// what takes the silence away. A pipe is somebody already sending, so nothing
/// is written for it -- a line there would land in the output of whatever is
/// being piped.
fn read_stdin() -> Result<String, String> {
    use std::io::{IsTerminal, Read};

    let stdin = std::io::stdin();
    if stdin.is_terminal() {
        eprintln!("Reading the words from standard input. End them with Ctrl-D.");
    }
    let mut words = String::new();
    stdin
        .lock()
        .read_to_string(&mut words)
        .map_err(|e| format!("failed to read the words from standard input: {e}"))?;
    Ok(words)
}

/// Writes `backlog/open/YYYYMMDD-feedback-<HHMMSS>.md` under `root`, and
/// returns the path.
///
/// The name is the day and the second, with no slug: a slug is a summary, and
/// this run has not read the words yet. The title line comes out of the file
/// for the same reason, since its content would be the first line of the
/// section a little higher up.
fn write_feedback(root: &std::path::Path, words: &str) -> Result<std::path::PathBuf, String> {
    let Some((_, template)) = KINDS.iter().find(|(k, _)| *k == FEEDBACK_KIND) else {
        return Err(format!(
            r"this build has no `{FEEDBACK_KIND}` template to write from, which is a bug
in nokori rather than something to fix here."
        ));
    };

    check_backlog_at(root)?;

    let clock = now()?;
    let name = format!("{}-{FEEDBACK_KIND}-{}.md", clock.date, clock.time);
    let path = root.join(OPEN_DIR).join(name);
    if path.exists() {
        return Err(format!(
            r"`{}` already exists, so an item was written in this second already.
Wait a second and run the command again; if it was this run, nothing was lost.",
            path.display()
        ));
    }

    let content = feedback_content(template, words)?;
    std::fs::write(&path, content)
        .map_err(|e| format!("failed to write `{}`: {e}", path.display()))?;
    Ok(path)
}

/// Says which way `root` is not a repository with a backlog, with a next step
/// for each.
///
/// The path is the one thing the command cannot help with -- where another
/// repository is, only somebody who knows does -- so every way of getting it
/// wrong says which way it was.
fn check_backlog_at(root: &std::path::Path) -> Result<(), String> {
    if !root.is_dir() {
        return Err(format!(
            r"`{}` is not a directory, so there is no repository to write the item in.
Give the path of the directory that holds that repository's `{BACKLOG_DIR}/`, as
this command read it -- a relative path is read from where you ran `nokori`.",
            root.display()
        ));
    }

    if !root.join(BACKLOG_DIR).join("README.md").is_file() {
        return Err(format!(
            r"`{}` has no `{BACKLOG_DIR}/README.md`, so its backlog is not set up.

Run `nokori init` in that directory, or give the path of a repository that has
one. Whether to make a backlog there is that repository's decision, so this
command does not make it.",
            root.display()
        ));
    }

    if !root.join(OPEN_DIR).is_dir() {
        return Err(format!(
            r"`{}` has a `{BACKLOG_DIR}/README.md` but no `{OPEN_DIR}/`, so its layout is
not the one this version of nokori writes.

`{OPEN_DIR}/` is where an open item goes. If `{BACKLOG_DIR}/` was made by an older
nokori, `nokori --version` here will say which one this run is.",
            root.display()
        ));
    }

    Ok(())
}

/// The item as it is written: the template without its title line, with the
/// words in place of the comment in `## Feedback`.
///
/// Both edits are made here rather than in the template because `create`
/// writes the same template with its title line kept.
fn feedback_content(template: &str, words: &str) -> Result<String, String> {
    let mut out = String::new();
    let mut dropped_title = false;
    let mut body = template;
    if let Some(rest) = body.strip_prefix(FEEDBACK_TITLE_PREFIX) {
        let end = rest.find('\n').map_or(rest.len(), |i| i + 1);
        body = &rest[end.min(rest.len())..];
        dropped_title = true;
    }
    if !dropped_title {
        return Err(format!(
            r"the `{FEEDBACK_KIND}` template does not start with `{FEEDBACK_TITLE_PREFIX}`,
so the title line could not be dropped. This is a bug in nokori."
        ));
    }

    let heading = format!("{FEEDBACK_SECTION}\n");
    let Some(at) = body.find(&heading) else {
        return Err(format!(
            r"the `{FEEDBACK_KIND}` template has no `{FEEDBACK_SECTION}` section, so the words
have nowhere to go. This is a bug in nokori."
        ));
    };
    let (before, after_heading) = body.split_at(at + heading.len());
    out.push_str(&before_without_leading_blank(before));
    out.push_str(words);
    out.push('\n');

    // What follows the words is the rest of the template with the comment in
    // this section taken out. The comment is the answer the words just gave.
    let rest = drop_section_comment(after_heading);
    out.push_str(&rest);
    Ok(out)
}

/// Removes the blank line the title line's removal left above the first
/// heading.
fn before_without_leading_blank(before: &str) -> String {
    let trimmed = before.trim_start_matches('\n');
    if trimmed.is_empty() {
        String::new()
    } else {
        let mut out = trimmed.to_owned();
        out.push('\n');
        out
    }
}

/// Removes the leading `<!-- NOKORI_TODO: ... -->` comment, which is the one
/// asking for the section the words now fill.
fn drop_section_comment(rest: &str) -> String {
    let trimmed = rest.trim_start_matches('\n');
    let Some(body) = trimmed.strip_prefix("<!--") else {
        return trimmed.to_owned();
    };
    if !body.contains(TODO_MARKER) {
        return trimmed.to_owned();
    }
    let Some(close) = body.find("-->") else {
        return trimmed.to_owned();
    };
    let after = &body[close + 3..];
    let after = after.trim_start_matches('\n');
    let mut out = String::from("\n");
    out.push_str(after);
    out
}

fn try_run_check(args: &mut noargs::RawArgs) -> noargs::Result<bool> {
    if !noargs::cmd("check")
        .doc("Say whether an open item is written yet")
        .take(args)
        .is_present()
    {
        return Ok(false);
    }

    let file: String = noargs::arg("<FILE>")
        .doc("Item to check, given as its path")
        .example("backlog/open/20250101-bug-panic-on-empty-input.md")
        .take(args)
        .then(|a| a.value().parse())?;

    if args.metadata().help_mode {
        return Ok(true);
    }

    check_item(&file)?;

    println!("{file} is written, so it can be closed once `{OUTCOME_SECTION}`");
    println!("says what happened:");
    println!();
    println!("  nokori close {file}");

    Ok(true)
}

fn try_run_close(args: &mut noargs::RawArgs) -> noargs::Result<bool> {
    if !noargs::cmd("close")
        .doc("Close an item")
        .take(args)
        .is_present()
    {
        return Ok(false);
    }

    let file: String = noargs::arg("<FILE>")
        .doc("Item to close, given as its path")
        .example("backlog/open/20250101-bug-panic-on-empty-input.md")
        .take(args)
        .then(|a| a.value().parse())?;

    if args.metadata().help_mode {
        return Ok(true);
    }

    let closing = close_item(&file)?;

    println!("Closed {}", closing.path.display());
    println!();
    println!("Commit the move, and leave the file as it is from then on:");
    println!("`{CLOSED_DIR}/` is a record, so later work cites it rather than editing it.");

    Ok(true)
}

fn try_run_tree(args: &mut noargs::RawArgs) -> noargs::Result<bool> {
    if !noargs::cmd("tree")
        .doc("Print the items reachable from one item")
        .take(args)
        .is_present()
    {
        return Ok(false);
    }

    let file: String = noargs::arg("<ITEM>")
        .doc("Item to start from, given as its path")
        .example("backlog/open/20250101-rfc-panic-on-empty-input.md")
        .take(args)
        .then(|a| a.value().parse())?;

    if args.metadata().help_mode {
        return Ok(true);
    }

    let tree = walk_tree(&file)?;
    print!("{tree}");

    Ok(true)
}

/// Reads every item under `backlog/open/` and `backlog/closed/`, with the links
/// each one holds, keyed by the path the links are resolved against.
///
/// Nothing is cached between runs: a link from a closed item is a relation like
/// any other, and a copy of the graph would be right until somebody edited a
/// file.
fn read_items() -> Result<Vec<Item>, String> {
    let mut items = Vec::new();
    for dir in [OPEN_DIR, CLOSED_DIR] {
        let entries = match std::fs::read_dir(dir) {
            Ok(entries) => entries,
            // A directory that is not there holds no item, and a backlog with
            // nothing closed yet has no `closed/`. That is a backlog with
            // fewer relations in it, not a reason to refuse the walk.
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => return Err(format!("failed to read `{dir}`: {e}")),
        };
        for entry in entries {
            let entry = entry.map_err(|e| format!("failed to read `{dir}`: {e}"))?;
            let name = entry.file_name().to_string_lossy().into_owned();
            if name == "README.md"
                || !std::path::Path::new(&name)
                    .extension()
                    .is_some_and(|e| e == "md")
            {
                continue;
            }
            let path = std::path::Path::new(dir).join(&name);
            let content = std::fs::read_to_string(&path)
                .map_err(|e| format!("failed to read `{}`: {e}", path.display()))?;
            let here = path.parent().unwrap_or_else(|| std::path::Path::new("."));
            let links = links(&content)
                .into_iter()
                .filter(|link| !is_external(&link.target))
                .map(|link| target_path(here, &link.target))
                .filter(|target| is_item_path(target))
                .collect();
            items.push(Item {
                path: normalize(&path),
                name: item_label(&name),
                links,
            });
        }
    }
    Ok(items)
}

/// An item read for the walk: the file it is in, the label a line prints, and
/// the items it links.
struct Item {
    path: std::path::PathBuf,
    name: String,
    links: Vec<std::path::PathBuf>,
}

/// The walk from one item, as the text `tree` prints. Each item is printed
/// under the line that reached it, indented two spaces per hop, with a mark
/// saying which file holds the link that reached it: `-` for a link in the line
/// above, `<-` for a link in the line itself.
///
/// An item reached by two paths is printed under both, since deduplicating
/// would say which path was taken instead. What keeps the walk finite is the
/// set of items on the current path: a repeat there is a cycle, and the walk
/// stops and says so rather than hanging.
fn walk_tree(file: &str) -> Result<String, String> {
    let start = std::path::Path::new(file);
    if !start.is_file() {
        return Err(format!(
            r"`{file}` is not a file, so there is no item to walk from. Pass the item as a
path, such as `nokori tree {OPEN_DIR}/20250101-bug-panic-on-empty-input.md`."
        ));
    }
    if !is_item_path(&normalize(start)) {
        return Err(format!(
            r"`{file}` is not an item. `tree` walks the links between items, so it starts
from one in `{OPEN_DIR}/` or `{CLOSED_DIR}/`. Run `nokori explain overview` for what an item
is."
        ));
    }

    let items = read_items()?;
    let start = normalize(start);
    let Some(first) = items.iter().find(|item| item.path == start) else {
        return Err(format!(
            r"`{file}` is not an item. Nothing under `{OPEN_DIR}/` or `{CLOSED_DIR}/` has that
name. Pass the path of a file in one of them."
        ));
    };

    let mut out = String::new();
    out.push_str(&first.name);
    out.push('\n');
    let mut path = vec![first.path.clone()];
    walk_children(first, &items, 1, &mut path, &mut out);
    Ok(out)
}

/// Prints the items related to `item`, one hop out, and then theirs.
fn walk_children(
    item: &Item,
    items: &[Item],
    depth: usize,
    path: &mut Vec<std::path::PathBuf>,
    out: &mut String,
) {
    for (next, forward) in neighbours(item, items) {
        let indent = "  ".repeat(depth);
        let mark = if forward { "- " } else { "<- " };
        out.push_str(&format!("{indent}{mark}{}\n", next.name));

        if path.contains(&next.path) {
            // A cycle: the item is already on this path, so following it again
            // would print it forever. This is the only way the walk could not
            // end, and it is said here rather than refused before printing.
            out.push_str(&format!(
                "{}  (already on the path: the links run in a circle, so the walk stops here)\n",
                "  ".repeat(depth)
            ));
            continue;
        }
        path.push(next.path.clone());
        walk_children(next, items, depth + 1, path, out);
        path.pop();
    }
}

/// The items one hop from `item`, each with a mark saying where the link is:
/// `true` when the link is in `item` and points at the other one, `false` when
/// the link is in the other one and points back at `item`.
///
/// The ones `item` links come first, in the order the links are in its file,
/// since that is the order the writer wrote them in. The ones that link to it
/// follow, in the order the files are read from the directory.
fn neighbours<'a>(item: &'a Item, items: &'a [Item]) -> Vec<(&'a Item, bool)> {
    let mut found: Vec<(&Item, bool)> = Vec::new();
    for target in &item.links {
        if let Some(other) = items.iter().find(|candidate| &candidate.path == target) {
            found.push((other, true));
        }
    }
    for other in items {
        if other.path == item.path {
            continue;
        }
        if other.links.contains(&item.path) {
            found.push((other, false));
        }
    }
    found
}

/// Says whether a path names an item: a file directly in `backlog/open/` or
/// `backlog/closed/` whose name is `YYYYMMDD-<kind>-<slug>.md`.
fn is_item_path(path: &std::path::Path) -> bool {
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new(""));
    if parent != std::path::Path::new(OPEN_DIR) && parent != std::path::Path::new(CLOSED_DIR) {
        return false;
    }
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    let Some(stem) = name.strip_suffix(".md") else {
        return false;
    };
    let digits = stem.chars().take_while(char::is_ascii_digit).count();
    digits == 8 && stem[digits..].starts_with('-')
}

/// The label a line prints for an item: the kind and slug, which is the text a
/// link to it carries.
fn item_label(name: &str) -> String {
    link_name(name)
}

/// Folds `.` and `..` away, so that two spellings of one path compare equal.
fn normalize(path: &std::path::Path) -> std::path::PathBuf {
    let mut out = std::path::PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                out.pop();
            }
            other => out.push(other),
        }
    }
    out
}

fn try_run_explain(args: &mut noargs::RawArgs) -> noargs::Result<bool> {
    if !noargs::cmd("explain")
        .doc("Print the rules behind the commands (see `<TOPIC>`)")
        .take(args)
        .is_present()
    {
        return Ok(false);
    }

    let topic = noargs::arg("[<TOPIC>]")
        .doc("Topic to explain (`overview`, `kinds`, `flow`, or `references`)")
        .example("kinds")
        .take(args)
        .present();
    let topic: Option<String> = topic.map(|a| a.value().parse()).transpose()?;

    if args.metadata().help_mode {
        return Ok(true);
    }

    // With no topic, the list itself is the answer: it is short enough to print
    // whole, and `overview` comes first so that a reader who asked for nothing
    // in particular is handed the one to read first.
    let Some(topic) = topic else {
        println!("The topics are:");
        println!();
        for (name, _) in TOPICS {
            println!("- {name}");
        }
        println!();
        println!("Run `nokori explain <topic>` for one of them.");
        return Ok(true);
    };

    let Some((_, text)) = TOPICS.iter().find(|(t, _)| *t == topic) else {
        let known: Vec<&str> = TOPICS.iter().map(|(t, _)| *t).collect();
        return Err(format!(
            r"`{topic}` is not a topic. The topics are: {}.

A topic is the material behind the commands, not the usage of one: run
`nokori --help` for the commands, and `nokori <command> --help` for one of
them.",
            known.join(", ")
        )
        .into());
    };

    print!("{text}");

    Ok(true)
}

fn create_item(kind: &str, slug: &str) -> Result<std::path::PathBuf, String> {
    let Some((_, template)) = KINDS.iter().find(|(k, _)| *k == kind) else {
        let known: Vec<&str> = KINDS.iter().map(|(k, _)| *k).collect();
        return Err(format!(
            "`{kind}` is not a known kind. The kinds are: {}.",
            known.join(", ")
        ));
    };

    if !is_valid_slug(slug) {
        return Err(format!(
            r"`{slug}` cannot be used in a filename. Use lowercase letters, digits,
and `-` or `_`, so that the file name needs no quoting."
        ));
    }

    if !std::path::Path::new(BACKLOG_DIR)
        .join("README.md")
        .is_file()
    {
        return Err(format!(
            r"`{BACKLOG_DIR}/README.md` does not exist, so this repository has no
backlog yet. Run `nokori init` first."
        ));
    }

    let date = today()?;
    let name = format!("{date}-{kind}-{slug}.md");
    let path = std::path::Path::new(OPEN_DIR).join(name);
    if path.exists() {
        return Err(format!(
            r"`{}` already exists, so an item with that kind and slug was created
today. Pick another slug, or edit the existing file.",
            path.display()
        ));
    }

    std::fs::write(&path, template)
        .map_err(|e| format!("failed to write `{}`: {e}", path.display()))?;
    Ok(path)
}

/// Reads an item that is still being written and refuses the two things that
/// are wrong while it is open: a `TODO` left in a comment, and a link that
/// does not resolve.
///
/// The checks `close` makes that are not here are the point of the command.
/// `## Outcome` is *expected* to be empty, a file with uncommitted changes is
/// the normal state of something being written, and the link into
/// `backlog/open/` is allowed -- that rule is about a file that has moved.
///
/// The one check that is not the same as `close`'s is the link: `close` reads
/// the file from where it is going, and this reads it from where it is. Where
/// it is going depends on how it is spelled, and a file being written is
/// allowed to be unfinished in ways the move would settle, so what is
/// reported is what is wrong now.
fn check_item(file: &str) -> Result<(), String> {
    let src = std::path::Path::new(file);
    if !src.is_file() {
        return Err(format!(
            r"`{file}` is not a file, so there is no item to check. Pass the item as
a path, such as `nokori check {OPEN_DIR}/20250101-bug-panic-on-empty-input.md`."
        ));
    }

    if src.parent() != Some(std::path::Path::new(OPEN_DIR)) {
        let hint = if src.components().count() == 1 {
            format!(" The item is probably `{OPEN_DIR}/{file}`.")
        } else {
            format!(
                r" There is nothing to check about an item in `{CLOSED_DIR}/`: it is
written and settled, and it is not edited again."
            )
        };
        return Err(format!(
            r"`{file}` is not an item in `{OPEN_DIR}/`, so `check` is not the command
for it.{hint}"
        ));
    }

    let content =
        std::fs::read_to_string(src).map_err(|e| format!("failed to read `{file}`: {e}"))?;
    check_markers_outside_outcome(file, &content)?;
    check_links_now(file, src, &content)
}

/// The half of `check_filled_in` that is about a file still being written: the
/// `TODO` comments, and nothing about `## Outcome`, which is empty until the
/// close.
fn check_markers(file: &str, content: &str) -> Result<(), String> {
    check_marker(content, |_| true).map_or(Ok(()), |(todo, line)| {
        let last = format!(
            " If a section has nothing to say, delete its comment along with\n\
             the heading -- every section but `{OUTCOME_SECTION}`, which has to stay\n\
             even when it is empty."
        );
        Err(marker_message(file, content, todo, line, &last))
    })
}

/// `check_markers` for a file that is still being worked on: the same, without
/// the sections the close writes. A comment is left in those for the whole of
/// the working time, so its being there says nothing about whether the work is
/// done.
fn check_markers_outside_outcome(file: &str, content: &str) -> Result<(), String> {
    check_marker(content, |heading| heading != OUTCOME_SECTION).map_or(Ok(()), |(todo, line)| {
        let last = format!(
            " `{OUTCOME_SECTION}` is not reported here: its comment stays until the\n\
             item is closed, so it is still there while the work is going on. When\n\
             every other section is written, the item can be closed, and the close\n\
             is where that section is written."
        );
        Err(marker_message(file, content, todo, line, &last))
    })
}

/// The first marker left in a comment whose section `in_section` accepts, as
/// the offset of the marker and the line it is on.
fn check_marker(content: &str, in_section: impl Fn(&str) -> bool) -> Option<(usize, usize)> {
    let mut from = 0;
    while let Some(found) = find_marker(&content[from..]) {
        let todo = from + found;
        let line = content[..todo].lines().count();
        let heading = content[..todo]
            .lines()
            .rev()
            .find(|line| line.starts_with("## "))
            .unwrap_or("");
        if in_section(heading) {
            return Some((todo, line));
        }
        // Resume after the comment the marker was in, so that the next search
        // looks at the next comment rather than at this one again.
        from = content[todo..]
            .find("-->")
            .map_or(content.len(), |end| todo + end + 3);
    }
    None
}

fn marker_message(file: &str, content: &str, todo: usize, line: usize, last: &str) -> String {
    let message = format!(
        r"`{file}` still has a `{TODO_MARKER}` comment (line {line}), so it is not
finished. Each comment says what its section needs; write that and delete the
comment.{last}"
    );
    match content[..todo]
        .lines()
        .rev()
        .find(|line| line.starts_with("## "))
    {
        Some(heading) => format!(
            "{message}\n\nThe comment is in `{heading}`, so that section is not written yet."
        ),
        None => message,
    }
}

/// Checks the links in a file where it is now. Nothing is refused for the
/// direction or for what the move would do to it, since neither is a problem
/// yet: a link into `backlog/open/` is allowed from `backlog/open/`, and a
/// target that only becomes wrong when the file moves is one to fix before the
/// move, which is the next command.
fn check_links_now(file: &str, src: &std::path::Path, content: &str) -> Result<(), String> {
    let here = src.parent().unwrap_or_else(|| std::path::Path::new("."));
    for link in links(content) {
        if is_external(&link.target) {
            continue;
        }
        let path = target_path(here, &link.target);
        if !path.exists() {
            return Err(format!(
                r"`{file}` links `{}` (line {}), which is not a file. A link to a file in
this repository is read as a path, so it has to resolve for the item to be
written.

If the file it names was renamed, point the link at the new name. If it was
never written, say what it will say instead. The paths are in `nokori explain
references`.",
                link.target, link.line
            ));
        }
    }

    Ok(())
}

fn close_item(file: &str) -> Result<Closing, String> {
    let src = std::path::Path::new(file);
    let name = src.file_name().map(|name| name.to_owned());
    if !src.is_file() {
        let closed = match name {
            Some(name) => std::path::Path::new(CLOSED_DIR).join(name),
            None => std::path::PathBuf::new(),
        };
        let hint = if closed.is_file() {
            format!(
                r" It is already closed, as `{}`: a closed item is left as it was
written, so there is nothing for `close` to do.",
                closed.display()
            )
        } else {
            format!(
                r" Pass the item to close as a path, such as
`nokori close {BACKLOG_DIR}/20250101-bug-panic-on-empty-input.md`."
            )
        };
        return Err(format!(
            "`{file}` is not a file, so there is no item to close.{hint}"
        ));
    }

    if src.parent() != Some(std::path::Path::new(OPEN_DIR)) {
        let hint = if src.components().count() == 1 {
            format!(" The item is probably `{OPEN_DIR}/{file}`.")
        } else {
            format!(
                r" A file that is not in `{OPEN_DIR}/` is not open, and one that is
already closed is left as it was written."
            )
        };
        return Err(format!(
            r"`{file}` is not an item in `{OPEN_DIR}/`, so `close` is not the command
for it.{hint}"
        ));
    }

    let content =
        std::fs::read_to_string(src).map_err(|e| format!("failed to read `{file}`: {e}"))?;
    check_filled_in(file, &content)?;
    let name = name.expect("a path under the open directory is a file");
    let dst = std::path::Path::new(CLOSED_DIR).join(&name);
    // Checked before the move, and from both places: a link that names one
    // file in `open/` and another in `closed/` would be broken by the move,
    // and a closed file is not edited to repair it afterwards.
    check_links(file, src, &dst, &content)?;

    if dst.exists() {
        return Err(format!(
            r"`{}` already exists, so closing this item would replace the record of
another one. Rename one of the two files by hand and run `nokori close`
again.",
            dst.display()
        ));
    }

    std::fs::create_dir_all(CLOSED_DIR)
        .map_err(|e| format!("failed to create `{CLOSED_DIR}`: {e}"))?;
    std::fs::rename(src, &dst)
        .map_err(|e| format!("failed to move `{file}` to `{}`: {e}", dst.display()))?;
    Ok(Closing { path: dst })
}

/// Checks that the item is ready to be closed: no `TODO` left anywhere, and the
/// `## Outcome` section filled in.
fn check_filled_in(file: &str, content: &str) -> Result<(), String> {
    check_markers(file, content)?;
    check_outcome(file, content)
}

/// The half of `check_filled_in` that only `close` makes: the section that
/// records how the item settled has to be there, and has to say something.
fn check_outcome(file: &str, content: &str) -> Result<(), String> {
    let outcome = content
        .lines()
        .position(|line| line.trim_end() == OUTCOME_SECTION);
    let Some(start) = outcome else {
        return Err(format!(
            r"`{file}` has no `{OUTCOME_SECTION}` section. An item records what was
concluded, so a file without one cannot be closed. Add the section, write
what happened, and commit the file."
        ));
    };
    let body: String = content
        .lines()
        .skip(start + 1)
        .collect::<Vec<_>>()
        .join("\n");
    if body.trim().is_empty() {
        return Err(format!(
            r"`{OUTCOME_SECTION}` in `{file}` is empty, and closing records the outcome
there. Say what actually happened and name the change that settled the item
(the commit or pull request, when there is one), then commit the file."
        ));
    }

    Ok(())
}

/// Finds the byte offset of the first marker left in a comment, or `None` when
/// every section is written.
///
/// Only the inside of an HTML comment is searched. The marker is a comment in
/// every template, and an item is allowed to write *about* it -- a bug report
/// on this check is one -- so a marker in a sentence is prose and not work left
/// undone. Searching the comments is what tells the two apart without asking
/// the writer to escape anything.
///
/// A comment that is never closed is searched to the end of the file, since the
/// rest of such a file is commented out as far as a reader is concerned, and a
/// marker in it is as unfinished as one in a closed comment.
fn find_marker(content: &str) -> Option<usize> {
    let mut rest = 0;
    while let Some(open) = content[rest..].find("<!--") {
        let body = rest + open + 4;
        let (end, next) = match content[body..].find("-->") {
            Some(close) => (body + close, body + close + 3),
            None => (content.len(), content.len()),
        };
        if let Some(found) = content[body..end].find(TODO_MARKER) {
            return Some(body + found);
        }
        rest = next;
    }
    None
}

/// A link found in a file: the target as it is written, and the line it is on
/// so that the error can say which one is wrong.
struct Link {
    target: String,
    line: usize,
}

/// Checks the links in a file about to move from `open/` to `closed/`. Each
/// one is read from where the file is going, and two things are refused: a
/// target that names an item still in `open/`, and a target that does not
/// resolve.
///
/// Both are about the file as it will be read, not as it is read now. A closed
/// file is not edited, so a target that is wrong once the move is done is one
/// nobody can repair. The rule that a closed item never cites an open one is
/// checked here for the same reason, and the move is what would break it: a
/// file in `closed/` writing `../open/x.md` is exactly such a citation.
///
/// Every link is read as a path, whether or not it names an item, since a
/// check with a narrower scope is one the writer has to remember. Links that
/// are not paths are skipped: one with a scheme (`https://`, `mailto:`) and
/// one that only names an anchor in this file.
fn check_links(
    file: &str,
    src: &std::path::Path,
    dst: &std::path::Path,
    content: &str,
) -> Result<(), String> {
    let here = src.parent().unwrap_or_else(|| std::path::Path::new("."));
    let there = dst.parent().unwrap_or_else(|| std::path::Path::new("."));

    for link in links(content) {
        if is_external(&link.target) {
            continue;
        }

        let now = target_path(here, &link.target);
        let then = target_path(there, &link.target);

        // The item it names is still open, so the closed file would cite an
        // open one: the item can still be renamed or turned down, and a closed
        // file is not edited to follow it. Both places are read, because a
        // bare file name reaches an open item where the file is now and
        // nothing at all where it is going, and the open item is the more
        // useful thing to say.
        let cites_open = |path: &std::path::Path| path.starts_with(OPEN_DIR) && path.is_file();
        if cites_open(&now) || cites_open(&then) {
            let named = if then.is_file() { then } else { now };
            return Err(cites_open_message(file, &link, &named));
        }

        if !then.exists() {
            return Err(format!(
                r"`{file}` links `{}` (line {}), which is not a file. A close refuses
a link that does not resolve, because a closed item is not edited again and the
link could never be repaired there.

If the item it named was closed, point the link at its file in `{CLOSED_DIR}/`.
If it was renamed, point it at the new name. If it was never written, say what
it was going to do instead. The paths are in `nokori explain references`.",
                link.target, link.line
            ));
        }
    }

    Ok(())
}

/// The message for a link that names an item still in `backlog/open/`. What to
/// do with it depends on the other file: a link is stored at either end, so
/// when that file links back the relation survives the link becoming a name,
/// and the message stops after saying to write the name. When it does not, the
/// relation would be nowhere once the link is a name, so the message says to
/// add the link there while that file is still open.
///
/// The other file is read, never written. It is the one file the link names and
/// not the whole of `backlog/open/`: the question is about these two items, and
/// a link from a third one says nothing about it.
fn cites_open_message(file: &str, link: &Link, target: &std::path::Path) -> String {
    let head = format!(
        r"`{file}` links `{}` (line {}), which names an item that is still open.
A closed item never cites an open one: the item it names can still be renamed
or turned down, and this file will not be edited again to follow it.

The relation does not need the link, though -- one link holds it from either
end -- so what a closed file needs is a name to read, not a target to follow.",
        link.target, link.line
    );
    let name = link_name(&link.target);
    let fix = format!(
        r"Write the target as a name instead, where the link is:

  `{name}`"
    );

    let other = target.display();
    if links_back(target, file) {
        return format!(
            "{head}\n\n{fix}\n\n`{other}` links back, so the relation is held from that end and\nthis name is all this file owes."
        );
    }
    format!(
        r"{head}

{fix}

`{other}` does not link back, so that name would be the only thing recording
the relation and a name is not followable. Add the link there while that file
is still open, at either end, and close this one afterwards. The rule is in
`nokori explain references`."
    )
}

/// The name a link's target is written as when the link has to go: its text
/// without the path. The text of a link is already the kind and slug (see
/// `nokori explain references`), so the name reads the same as the link did.
fn link_name(target: &str) -> String {
    let path = std::path::Path::new(target);
    let file = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(target);
    let stem = file.strip_suffix(".md").unwrap_or(file);
    // The date is a run of digits at the front and the `-` that joins it to
    // the kind, so that `20250101-bug-x` becomes `bug-x`.
    let digits = stem.chars().take_while(char::is_ascii_digit).count();
    if digits > 0 {
        let rest = &stem[digits..];
        if let Some(name) = rest.strip_prefix('-') {
            return name.to_owned();
        }
    }
    stem.to_owned()
}

/// Says whether `other` holds a link to `file`. Both are read as paths, and
/// every link in `other` is resolved from the directory it is in, so a link
/// written the way `explain references` says is found whichever end it is at.
///
/// A file that cannot be read links back to nothing: the writer has the other
/// file to look at, and the message then says the safe thing.
fn links_back(other: &std::path::Path, file: &str) -> bool {
    let Ok(content) = std::fs::read_to_string(other) else {
        return false;
    };
    let here = other.parent().unwrap_or_else(|| std::path::Path::new("."));
    let me = std::path::Path::new(file);
    links(&content)
        .iter()
        .any(|link| !is_external(&link.target) && target_path(here, &link.target) == me)
}

/// Says whether a target is something other than a path in this repository: a
/// link with a scheme, or one that names only an anchor in the same file. The
/// two are skipped by `check_links`, since neither is a file that the move
/// could change the meaning of.
///
/// A scheme is read as the colon before any `/` (`https:`, `mailto:`), so that
/// a scheme is recognized by its form rather than by a list of the ones found
/// so far. A colon further along is part of a file name and is not a scheme.
fn is_external(target: &str) -> bool {
    if target.starts_with('#') || target.is_empty() {
        return true;
    }
    let head = target.split('/').next().unwrap_or_default();
    head.contains(':')
}

/// What `close` did: the file it moved. Nothing else changed, so there is
/// nothing else to report.
struct Closing {
    path: std::path::PathBuf,
}

/// Collects the links in `content`. Only inline links (`[text](target)`) with a
/// target that is a plain path are read, since that is the form a reference is
/// written in.
///
/// A link is read only where a reader would see one. Text inside an HTML
/// comment is not rendered, and text inside a code fence or a code span is
/// rendered as code, so a `](` in any of them is not a link -- it is text about
/// a link, which no reader follows and which the item does not depend on. The
/// example links the templates put in their comments are the case that made
/// this matter: they name no file, and an item created from a template is
/// refused for one until its comment is deleted.
fn links(content: &str) -> Vec<Link> {
    let mut found = Vec::new();
    let mut fence: Option<(char, usize)> = None;
    let mut in_comment = false;

    for (index, line) in content.split('\n').enumerate() {
        if let Some((marker, length)) = fence {
            if closes_fence(line, marker, length) {
                fence = None;
            }
            continue;
        }
        if let Some(opened) = opens_fence(line) {
            fence = Some(opened);
            continue;
        }

        let text = visible_text(line, &mut in_comment);
        for link in links_in(&text) {
            found.push(Link {
                target: link,
                line: index + 1,
            });
        }
    }
    found
}

/// The parts of `line` that a reader would see as text: the line with every
/// HTML comment removed, and with the part of a comment that opened on an
/// earlier line removed as well. `in_comment` carries the state from one line
/// to the next, since a comment spans lines.
fn visible_text(line: &str, in_comment: &mut bool) -> String {
    let mut text = String::new();
    let mut rest = line;
    loop {
        if *in_comment {
            match rest.find("-->") {
                Some(end) => {
                    *in_comment = false;
                    rest = &rest[end + 3..];
                }
                None => return text,
            }
        } else {
            match rest.find("<!--") {
                Some(start) => {
                    text.push_str(&rest[..start]);
                    *in_comment = true;
                    rest = &rest[start + 4..];
                }
                None => {
                    text.push_str(rest);
                    return text;
                }
            }
        }
    }
}

/// The links in a line of visible text. Code spans are dropped by `prose_runs`
/// before the search, so a `](` between a pair of backticks is not found.
fn links_in(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for run in prose_runs(text) {
        let mut rest = run;
        while let Some(open) = rest.find("](") {
            let after = &rest[open + 2..];
            let Some(close) = after.find(')') else {
                break;
            };
            let target = after[..close].trim();
            if !target.is_empty() && !target.contains(['(', ' ', '\\']) {
                found.push(target.to_owned());
            }
            rest = &after[close + 1..];
        }
    }
    found
}

/// The parts of a line that are not code: the line split at its backticks, with
/// the part between a pair of them dropped. The parts before and after a code
/// span are kept apart rather than joined, since a `](` cannot span one: the
/// backticks would be part of its link text, and a link with backticks in its
/// text is not the form a reference is written in.
///
/// A span is closed by a run of backticks of the same length as the one that
/// opened it, so `` `x` `` and `` ``x`` `` are both one span. An unpaired run
/// leaves the rest of the line as code: the span ends at the end of the line.
fn prose_runs(text: &str) -> Vec<&str> {
    let mut runs = Vec::new();
    let mut open: Option<usize> = None;
    let mut start = 0;
    let mut at = 0;
    while at < text.len() {
        if !text[at..].starts_with('`') {
            at += text[at..].chars().next().map_or(1, char::len_utf8);
            continue;
        }
        let run = text[at..].chars().take_while(|c| *c == '`').count();
        match open {
            Some(length) if length == run => {
                open = None;
                start = at + run;
            }
            Some(_) => {}
            None => {
                if start < at {
                    runs.push(&text[start..at]);
                }
                open = Some(run);
            }
        }
        at += run;
    }
    if open.is_none() && start < text.len() {
        runs.push(&text[start..]);
    }
    runs
}

/// The character and length of the run that opens a code fence, if this line
/// starts one: three or more backticks or tildes, optionally indented.
fn opens_fence(line: &str) -> Option<(char, usize)> {
    let stripped = line.trim_start();
    let marker = stripped.chars().next().filter(|c| *c == '`' || *c == '~')?;
    let length = stripped.chars().take_while(|c| *c == marker).count();
    (length >= 3).then_some((marker, length))
}

/// Says whether this line closes a fence opened by a run of `length` `marker`s:
/// a line of the same character, at least as long, and nothing else.
fn closes_fence(line: &str, marker: char, length: usize) -> bool {
    let stripped = line.trim();
    let run = stripped.chars().take_while(|c| *c == marker).count();
    run >= length && stripped[run..].is_empty()
}

/// Reads a link target as a path from the repository root, dropping an anchor
/// and folding `.` and `..` away, so that the depth of the result says where
/// the file is rather than how the link was spelled.
///
/// A target is always a path from `from`, the directory the link is written
/// in. There is no target that is read from the root instead: that rule existed
/// to make the rewrite in `close` predictable, and neither is here any more.
fn target_path(from: &std::path::Path, target: &str) -> std::path::PathBuf {
    let target = target.split('#').next().unwrap_or(target);
    let mut path = std::path::PathBuf::new();
    for component in from.join(target).components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                path.pop();
            }
            other => path.push(other),
        }
    }
    path
}

fn today() -> Result<String, String> {
    Ok(now()?.date)
}

/// The day and the time, read from `date` in one run so that a name made of
/// both is of one moment.
///
/// `date` is called rather than a crate being added for it: the tool already
/// leans on the machine for its clock, and a dependency for two format strings
/// is more to keep up to date than it is to save.
struct Clock {
    date: String,
    time: String,
}

fn now() -> Result<Clock, String> {
    let output = std::process::Command::new("date")
        .arg("+%Y%m%d %H%M%S")
        .output()
        .map_err(|e| format!("failed to run `date`: {e}"))?;
    if !output.status.success() {
        let message = "`date` failed, so the date and the time could not be read";
        return Err(message.to_owned());
    }
    let text = String::from_utf8(output.stdout)
        .map_err(|e| format!("`date` printed non-UTF-8 text: {e}"))?;
    let text = text.trim();
    let (date, time) = text
        .split_once(' ')
        .ok_or_else(|| format!("`date` printed `{text}`, which is not a date and a time"))?;
    Ok(Clock {
        date: date.to_owned(),
        time: time.to_owned(),
    })
}

fn is_valid_slug(slug: &str) -> bool {
    let allowed = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_';
    !slug.is_empty() && slug.chars().all(allowed)
}

fn init_backlog() -> Result<(), String> {
    let backlog_readme = std::path::Path::new(BACKLOG_DIR).join("README.md");
    if backlog_readme.exists() {
        return Err(format!(
            r"`{}` already exists, so `{BACKLOG_DIR}/` is already set up in
this repository. There is nothing left for `init` to do; you can start
creating items now (see `nokori create --help`).

If this `{BACKLOG_DIR}/` belongs to something else, rename it and run `nokori init`
again, or run `nokori` from another directory to use a different one.",
            backlog_readme.display()
        ));
    }

    for (path, template) in README_FILES {
        let path = std::path::Path::new(path);
        let dir = path
            .parent()
            .expect("a README path should have a parent directory");
        std::fs::create_dir_all(dir)
            .map_err(|e| format!("failed to create `{}`: {e}", dir.display()))?;
        std::fs::write(path, template)
            .map_err(|e| format!("failed to write `{}`: {e}", path.display()))?;
    }
    Ok(())
}
