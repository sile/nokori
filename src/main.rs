//! The `nokori` binary: parse arguments, do the I/O, and print.
//!
//! The Sans I/O core lives in the `nokori` library. This file is the edge: it
//! owns the argument specs, opens and writes the files, runs `date` for the
//! clock, reads standard input, and prints. Every decision it makes is one the
//! core cannot make without a file system or a terminal, and everything else
//! is a call into `nokori`.

#![forbid(unsafe_code)]

use std::io::Write;
use std::path::{Path, PathBuf};

fn main() -> noargs::Result<()> {
    let mut args = noargs::raw_args();
    args.metadata_mut().app_name = env!("CARGO_PKG_NAME");
    args.metadata_mut().app_description = env!("CARGO_PKG_DESCRIPTION");

    if noargs::VERSION_FLAG.take(&mut args).is_present() {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    noargs::HELP_FLAG.take_help(&mut args);

    // Each handler takes `RawArgs` by value, so that it can call `finish`
    // itself: the help text and the argument checks both happen inside the
    // handler, after its `take`s and before any of its work. That is what
    // keeps a stray argument from being reported once the work is done.
    if noargs::cmd("init")
        .doc(
            "Set up a backlog in this repository\n\nFind out what a backlog is here with `nokori explain overview`.",
        )
        .take(&mut args)
        .is_present()
    {
        return run_init(args);
    }
    if noargs::cmd("create")
        .doc(
            "Create a new item\n\nChoose the kind with `nokori explain kinds`; `explain flow` says the rest.",
        )
        .take(&mut args)
        .is_present()
    {
        return run_create(args);
    }
    if noargs::cmd("feedback")
        .doc(
            "Write down what somebody said, as a feedback item\n\nSee how it differs from the other kinds with `nokori explain feedback`.",
        )
        .take(&mut args)
        .is_present()
    {
        return run_feedback(args);
    }
    if noargs::cmd("check")
        .doc(
            "Say whether an open item is written yet\n\nSee where a check fits with `nokori explain flow`.",
        )
        .take(&mut args)
        .is_present()
    {
        return run_check(args);
    }
    if noargs::cmd("close")
        .doc("Close an item\n\nSee what closing means for the file with `nokori explain flow`.")
        .take(&mut args)
        .is_present()
    {
        return run_close(args);
    }
    if noargs::cmd("tree")
        .doc(
            "Print the items reachable from one item\n\nSee how items refer to each other with `nokori explain references`.",
        )
        .take(&mut args)
        .is_present()
    {
        return run_tree(args);
    }
    if noargs::cmd("explain")
        .doc(
            "Print the rules behind the commands\n\n`overview`, `kinds`, `flow`, `references`, and `feedback` are the topics.",
        )
        .take(&mut args)
        .is_present()
    {
        return run_explain(args);
    }

    // No subcommand: top-level help, and the top-level argument checks.
    if let Some(help) = args.finish()? {
        print!("{help}");
    }

    Ok(())
}

fn run_init(args: noargs::RawArgs) -> noargs::Result<()> {
    if let Some(help) = args.finish()? {
        print!("{help}");
        return Ok(());
    }

    init_backlog()?;

    for (path, _) in nokori::README_FILES {
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

    Ok(())
}

fn run_create(mut args: noargs::RawArgs) -> noargs::Result<()> {
    let kind: String = noargs::arg("<KIND>")
        .doc("Kind of item to create (`nokori explain kinds` lists them)")
        .example("bug")
        .take(&mut args)
        .then(|a| a.value().parse())?;
    let slug: String = noargs::arg("<SLUG>")
        .doc("Short name for the item, used in its filename")
        .example("panic-on-empty-input")
        .take(&mut args)
        .then(|a| a.value().parse())?;

    if let Some(help) = args.finish()? {
        print!("{help}");
        return Ok(());
    }

    let path = create_item(&kind, &slug)?;
    let file = path.display();

    println!("Created {file}");
    println!();
    println!(
        "Next, do the work and write it down: replace each `{}`",
        nokori::TODO_MARKER
    );
    println!("comment with what it asks for, and delete the comment.");
    println!();
    println!("Run `nokori check {file}` to see what is left.");
    println!();
    println!(
        "When nothing is, say what happened in `{}` and close",
        nokori::OUTCOME_SECTION
    );
    println!("the item with `nokori close {file}`.");

    Ok(())
}

fn run_feedback(mut args: noargs::RawArgs) -> noargs::Result<()> {
    let path: String = noargs::arg("<PATH>")
        .doc("Repository to write the item in, given as the directory holding `backlog/`")
        .example("../other")
        .take(&mut args)
        .then(|a| a.value().parse())?;
    let message: Option<String> = noargs::opt("message")
        .short('m')
        .doc("Take the words from this argument, or give `--file` or standard input")
        .take(&mut args)
        .present_and_then(|a| a.value().parse())?;
    let source: Option<String> = noargs::opt("file")
        .short('f')
        .doc("Take the words from this file")
        .take(&mut args)
        .present_and_then(|a| a.value().parse())?;

    if let Some(help) = args.finish()? {
        print!("{help}");
        return Ok(());
    }

    let words = feedback_words(message, source)?;
    let path = write_feedback(Path::new(&path), &words)?;
    let file = path.display();

    println!("Created {file}");
    println!();
    println!("The item belongs to the repository it was written into, so it is read");
    println!("and closed there. Nothing is committed and the file was not opened.");

    Ok(())
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

/// Writes the feedback item under `root`, and returns the path.
///
/// The name is the day and the second, with no slug: a slug is a summary, and
/// this run has not read the words yet. The title line comes out of the file
/// for the same reason, since its content would be the first line of the
/// section a little higher up.
fn write_feedback(root: &Path, words: &str) -> Result<PathBuf, String> {
    let template = nokori::feedback_template()?;

    check_backlog_at(root)?;

    let path = root.join(nokori::feedback_name(&now()?));
    if path.exists() {
        return Err(format!(
            r"`{}` already exists, so an item was written in this second already.
Wait a second and run the command again; if it was this run, nothing was lost.",
            path.display()
        ));
    }

    let content = nokori::feedback_content(template, words)?;
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
fn check_backlog_at(root: &Path) -> Result<(), String> {
    if !root.is_dir() {
        return Err(format!(
            r"`{}` is not a directory, so there is no repository to write the item in.
Give the path of the directory that holds that repository's `{}/`, as
this command read it -- a relative path is read from where you ran `nokori`.",
            root.display(),
            nokori::BACKLOG_DIR
        ));
    }

    if !root.join(nokori::BACKLOG_DIR).join("README.md").is_file() {
        return Err(format!(
            r"`{}` has no `{}/README.md`, so its backlog is not set up.

Run `nokori init` in that directory, or give the path of a repository that has
one. Whether to make a backlog there is that repository's decision, so this
command does not make it.",
            root.display(),
            nokori::BACKLOG_DIR
        ));
    }

    if !root.join(nokori::OPEN_DIR).is_dir() {
        return Err(format!(
            r"`{}` has a `{}/README.md` but no `{}/`, so its layout is
not the one this version of nokori writes.

`{}/` is where an open item goes. If `{}/` was made by an older
nokori, `nokori --version` here will say which one this run is.",
            root.display(),
            nokori::BACKLOG_DIR,
            nokori::OPEN_DIR,
            nokori::OPEN_DIR,
            nokori::BACKLOG_DIR
        ));
    }

    Ok(())
}

fn run_check(mut args: noargs::RawArgs) -> noargs::Result<()> {
    let file: String = noargs::arg("<FILE>")
        .doc("Item to check, given as its path")
        .example("backlog/open/20250101-bug-panic-on-empty-input.md")
        .take(&mut args)
        .then(|a| a.value().parse())?;

    if let Some(help) = args.finish()? {
        print!("{help}");
        return Ok(());
    }

    let path = Path::new(&file);
    // The text is read only for a file, since a path that is not one is
    // refused before its content would be used, and reading it would only
    // replace the core's message with an I/O error.
    let content = match path.is_file() {
        true => Some(
            std::fs::read_to_string(path).map_err(|e| format!("failed to read `{file}`: {e}"))?,
        ),
        false => None,
    };
    let exists = |p: &Path| p.exists();
    nokori::check_item(nokori::ItemSource {
        file: &file,
        is_file: content.is_some(),
        content: content.as_deref().unwrap_or(""),
        exists: &exists,
    })?;

    println!(
        "{file} is written, so it can be closed once `{}`",
        nokori::OUTCOME_SECTION
    );
    println!("says what happened:");
    println!();
    println!("  nokori close {file}");

    Ok(())
}

fn run_close(mut args: noargs::RawArgs) -> noargs::Result<()> {
    let file: String = noargs::arg("<FILE>")
        .doc("Item to close, given as its path")
        .example("backlog/open/20250101-bug-panic-on-empty-input.md")
        .take(&mut args)
        .then(|a| a.value().parse())?;

    if let Some(help) = args.finish()? {
        print!("{help}");
        return Ok(());
    }

    let (closed, broken) = close_item(&file)?;

    println!("Links checked; the move will not break one in the file.");
    println!("Closed {}", closed.display());
    if !broken.is_empty() {
        print_linking_open_items(&broken);
    }
    println!();
    println!("Commit the move, and leave the file as it is from then on:");
    println!(
        "`{}` is a record, so later work cites it rather than editing it.",
        nokori::CLOSED_DIR
    );

    Ok(())
}

/// Says which open items link the one that just moved, so their links can be
/// repaired. The list is gathered before the move, while the links still
/// resolve; after it they would read as broken files rather than as links that
/// were right until now.
fn print_linking_open_items(broken: &[PathBuf]) {
    println!();
    if broken.len() == 1 {
        println!("An open item links it and would break. Update it:");
    } else {
        println!(
            "{} open items link it and would break. Update them:",
            broken.len()
        );
    }
    for path in broken {
        println!("  {}", path.display());
    }
}

/// Reads `file`, hands the decision to the core, and performs the move it
/// asks for. Returns where the item is now, and the open items whose links to
/// it the move broke.
fn close_item(file: &str) -> Result<(PathBuf, Vec<PathBuf>), String> {
    let path = Path::new(file);
    let content = match path.is_file() {
        true => Some(
            std::fs::read_to_string(path).map_err(|e| format!("failed to read `{file}`: {e}"))?,
        ),
        false => None,
    };
    let exists = |p: &Path| p.exists();
    let read = |p: &Path| std::fs::read_to_string(p).ok();
    let decision = nokori::close_item(nokori::CloseSource {
        file,
        is_file: content.is_some(),
        content: content.as_deref().unwrap_or(""),
        exists: &exists,
        read: &read,
    })?;

    let source = decision.source();
    let destination = decision.destination();
    // Read before the move, since the links being gathered are the ones the
    // move is about to break.
    let items = read_items()?;
    let broken = nokori::open_items_linking(source, &items);
    std::fs::create_dir_all(nokori::CLOSED_DIR)
        .map_err(|e| format!("failed to create `{}`: {e}", nokori::CLOSED_DIR))?;
    std::fs::rename(source, destination).map_err(|e| {
        format!(
            "failed to move `{}` to `{}`: {e}",
            source.display(),
            destination.display()
        )
    })?;

    Ok((destination.to_path_buf(), broken))
}

fn run_tree(mut args: noargs::RawArgs) -> noargs::Result<()> {
    let invert = noargs::flag("invert")
        .short('i')
        .doc(
            "Follow the links that point at the item instead of the links it holds\n\nThe default follows what the item needs, which is the half an open item asks about. This follows what came from it, which is the half a settled one asks about.",
        )
        .take(&mut args)
        .is_present();
    // Any number of items, in the order given. The name says `[<ITEM>]...` for
    // the help text; a missing first one is not an error, since the run without
    // one is the run that walks every open item.
    let item_arg = noargs::arg("[<ITEM>]...")
        .doc(
            "Items to start from, given as their paths. With none, every item in `backlog/open/` is walked.",
        )
        .example("backlog/open/20250101-rfc-panic-on-empty-input.md");
    let mut files = Vec::<String>::new();
    while let Some(file) = item_arg
        .take(&mut args)
        .present_and_then(|a| a.value().parse())?
    {
        files.push(file);
    }

    if let Some(help) = args.finish()? {
        print!("{help}");
        return Ok(());
    }

    for file in &files {
        if !Path::new(file).is_file() {
            return Err(format!(
                r"`{file}` is not a file, so there is no item to walk from. Pass the item as a
path, such as `nokori tree {}/20250101-bug-panic-on-empty-input.md`.",
                nokori::OPEN_DIR
            )
            .into());
        }
    }

    let items = read_items()?;
    let direction = if invert {
        nokori::Direction::PointsAt
    } else {
        nokori::Direction::Holds
    };

    // No argument is the question about every open item, and the run says which
    // items it answered for, since those were not the ones it was handed.
    let defaulted = files.is_empty();
    if defaulted {
        println!("No item was given, so every open item is walked.");
        files = open_item_files()?;
        if files.is_empty() {
            eprintln!(
                "No item is open: `{}/` has nothing in it but its README.",
                nokori::OPEN_DIR
            );
            return Ok(());
        }
        // The line above is the run speaking, so the tree follows it directly;
        // the blank line keeps the two apart.
        println!();
    }

    let refs: Vec<&str> = files.iter().map(String::as_str).collect();
    let tree = nokori::Tree::of(&refs, &items, direction)?;

    // Written in one go through a locked handle. A run with no argument walks
    // every open item, and its text can be long enough that one `write_all` is
    // both cheaper and kinder than a line-by-line print.
    let mut stdout = std::io::stdout().lock();
    stdout
        .write_all(tree.as_str().as_bytes())
        .map_err(|e| format!("failed to write the tree: {e}"))?;

    Ok(())
}

/// The files in `backlog/open/`, sorted by path: the items walked when `tree`
/// is given none.
///
/// Only the open half is read: the closed items are what an open one needs, and
/// a run that walked them as roots too would print the whole backlog every time.
/// A directory that is not there is a backlog with nothing open, not a reason
/// to refuse the walk.
fn open_item_files() -> Result<Vec<String>, String> {
    let dir = nokori::OPEN_DIR;
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(format!("failed to read `{dir}`: {e}")),
    };

    let mut files = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("failed to read `{dir}`: {e}"))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let path = Path::new(dir).join(&name);
        if nokori::is_item_path(&nokori::normalize(&path)) {
            files.push(path.to_string_lossy().into_owned());
        }
    }
    files.sort();
    Ok(files)
}

/// Reads every item under `backlog/open/` and `backlog/closed/`, with the links
/// each one holds, keyed by the path the links are resolved against.
///
/// Nothing is cached between runs: a link from a closed item is a relation like
/// any other, and a copy of the graph would be right until somebody edited a
/// file.
fn read_items() -> Result<Vec<nokori::Item>, String> {
    let mut items = Vec::new();
    for dir in [nokori::OPEN_DIR, nokori::CLOSED_DIR] {
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
            if name == "README.md" || !Path::new(&name).extension().is_some_and(|e| e == "md") {
                continue;
            }
            let path = Path::new(dir).join(&name);
            let content = std::fs::read_to_string(&path)
                .map_err(|e| format!("failed to read `{}`: {e}", path.display()))?;
            items.push(item_from(&path, &name, &content));
        }
    }
    Ok(items)
}

/// Builds an item from its path and text: the label a line prints, and the
/// items it links as resolved paths.
///
/// A link that is not a path in this repository, and one that does not name an
/// item, is dropped: the walk is of the item graph, and a link to a source file
/// in a `Summary` is prose rather than part of it.
fn item_from(path: &Path, name: &str, content: &str) -> nokori::Item {
    let here = path.parent().unwrap_or_else(|| Path::new("."));
    let links = nokori::links(content)
        .into_iter()
        .filter(|link| !nokori::is_external(&link.target))
        .map(|link| nokori::target_path(here, &link.target))
        .filter(|target| nokori::is_item_path(target))
        .collect();
    nokori::Item {
        path: nokori::normalize(path),
        name: nokori::item_label(name),
        links,
    }
}

fn run_explain(mut args: noargs::RawArgs) -> noargs::Result<()> {
    let topic = noargs::arg("[<TOPIC>]")
        .doc("Topic to explain (`overview`, `kinds`, `flow`, `references`, or `feedback`)")
        .example("kinds")
        .take(&mut args)
        .present();
    let topic: Option<String> = topic.map(|a| a.value().parse()).transpose()?;

    if let Some(help) = args.finish()? {
        print!("{help}");
        return Ok(());
    }

    // With no topic, the list itself is the answer: it is short enough to print
    // whole, and `overview` comes first so that a reader who asked for nothing
    // in particular is handed the one to read first.
    let Some(topic) = topic else {
        println!("The topics are:");
        println!();
        for (name, _) in nokori::TOPICS {
            println!("- {name}");
        }
        println!();
        println!("Run `nokori explain <topic>` for one of them.");
        return Ok(());
    };

    let Some(text) = nokori::topic(&topic) else {
        let known: Vec<&str> = nokori::TOPICS.iter().map(|(t, _)| *t).collect();
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

    Ok(())
}

fn create_item(kind: &str, slug: &str) -> Result<PathBuf, String> {
    // `item_name` checks the kind and the slug, and refuses with the list of
    // kinds when it does not know one, so this lookup cannot fail after it.
    let path = nokori::item_name(&now()?, kind, slug)?;
    let template =
        nokori::template_for(kind).ok_or_else(|| format!("`{kind}` is not a known kind."))?;

    if !Path::new(nokori::BACKLOG_DIR).join("README.md").is_file() {
        return Err(format!(
            r"`{}/README.md` does not exist, so this repository has no
backlog yet. Run `nokori init` first.",
            nokori::BACKLOG_DIR
        ));
    }

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

/// The day and the time, read from `date` in one run so that a name made of
/// both is of one moment.
///
/// `date` is called rather than a crate being added for it: the tool already
/// leans on the machine for its clock, and a dependency for two format strings
/// is more to keep up to date than it is to save.
fn now() -> Result<nokori::Clock, String> {
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
    Ok(nokori::Clock {
        date: date.to_owned(),
        time: time.to_owned(),
    })
}

fn init_backlog() -> Result<(), String> {
    let backlog_readme = Path::new(nokori::BACKLOG_DIR).join("README.md");
    if backlog_readme.exists() {
        return Err(format!(
            r"`{}` already exists, so `{}/` is already set up in
this repository. There is nothing left for `init` to do; you can start
creating items now (see `nokori create --help`).

If this `{}/` belongs to something else, rename it and run `nokori init`
again, or run `nokori` from another directory to use a different one.",
            backlog_readme.display(),
            nokori::BACKLOG_DIR,
            nokori::BACKLOG_DIR
        ));
    }

    for (path, template) in nokori::README_FILES {
        let path = Path::new(path);
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
