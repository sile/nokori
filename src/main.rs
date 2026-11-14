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

/// The topics `explain` prints, each with the text. They carry the material a
/// command cannot: the rules about the states and the kinds, which read the
/// same whatever command the reader arrived from.
const TOPICS: &[(&str, &str)] = &[
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
        || try_run_close(&mut args)?
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
        .doc("Create a new item in backlog/")
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

    println!("Created {}", path.display());
    println!();
    println!("Next, replace the `{TODO_MARKER}` comments in the file with what");
    println!("they ask for, and commit it. Every comment has to be gone before");
    println!("the file moves, so delete each one as its section is written.");
    println!();
    println!("Close the item with `nokori close <file>` once it is settled.");

    Ok(true)
}

fn try_run_close(args: &mut noargs::RawArgs) -> noargs::Result<bool> {
    if !noargs::cmd("close")
        .doc("Close an item by moving it to backlog/closed/")
        .take(args)
        .is_present()
    {
        return Ok(false);
    }

    let file: String = noargs::arg("<FILE>")
        .doc("Item to close, as a path under backlog/open/")
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

fn try_run_explain(args: &mut noargs::RawArgs) -> noargs::Result<bool> {
    if !noargs::cmd("explain")
        .doc("Print the rules behind the commands (see `<TOPIC>`)")
        .take(args)
        .is_present()
    {
        return Ok(false);
    }

    let topic: String = noargs::arg("<TOPIC>")
        .doc("Topic to explain (`kinds`, `flow`, or `references`)")
        .example("kinds")
        .take(args)
        .then(|a| a.value().parse())?;

    if args.metadata().help_mode {
        return Ok(true);
    }

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
    check_committed(file)?;

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
    if let Some(todo) = content.find(TODO_MARKER) {
        let line = content[..todo].lines().count();
        let message = format!(
            r"`{file}` still has a `{TODO_MARKER}` comment (line {line}), so it is not
finished. Each comment says what its section needs; write that, delete the
comment, and commit the file. If a section has nothing to say, delete its
comment along with the heading."
        );
        return match content[..todo]
            .lines()
            .rev()
            .find(|line| line.starts_with("## "))
        {
            Some(heading) => Err(format!(
                "{message}\n\nThe comment is in `{heading}`, so that section is not written yet."
            )),
            None => Err(message),
        };
    }

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
            return Err(format!(
                r"`{file}` links `{}` (line {}), which names an item in `{OPEN_DIR}/`.
A closed item never cites an open one: the item it names can still be renamed
or turned down, and this file will not be edited again to follow it.

Wait until that item is closed and point the link at its file in `{CLOSED_DIR}/`,
or take the link out and say what the item proposes in your own words. The rule
is in `nokori explain references`.",
                link.target, link.line
            ));
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
/// written in; a reference inside a code fence is read too, which is accepted,
/// because a fence showing a link is a rare thing to write and a check that
/// skipped fences would be more code than the mistake costs.
fn links(content: &str) -> Vec<Link> {
    let mut found = Vec::new();
    for (index, line) in content.split('\n').enumerate() {
        let mut rest = line;
        while let Some(open) = rest.find("](") {
            let after = &rest[open + 2..];
            let Some(close) = after.find(')') else {
                break;
            };
            let field = &after[..close];
            let target = field.trim();
            if !target.is_empty() && !target.contains(['(', ' ', '\\']) {
                found.push(Link {
                    target: target.to_owned(),
                    line: index + 1,
                });
            }
            rest = &after[close + 1..];
        }
    }
    found
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

/// Checks that the file being moved has no changes git has not seen yet.
fn check_committed(file: &str) -> Result<(), String> {
    let output = std::process::Command::new("git")
        .args(["status", "--porcelain", "--", file])
        .output()
        .map_err(|e| format!("failed to run `git status`: {e}"))?;
    if !output.status.success() {
        let message = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            r"`git status` failed while checking `{file}`, and `close` refuses to move
a file it cannot check. The commit that moves an item is meant to record
only the move, so the file has to be committed first.

{message}"
        ));
    }

    if !output.stdout.is_empty() {
        return Err(format!(
            r"`{file}` has changes that are not committed yet, so moving it would mix
the close with an edit of what the item says. Commit the file first, so that
the commit that closes it records only the move."
        ));
    }

    Ok(())
}

fn today() -> Result<String, String> {
    let output = std::process::Command::new("date")
        .arg("+%Y%m%d")
        .output()
        .map_err(|e| format!("failed to run `date`: {e}"))?;
    if !output.status.success() {
        let message = "`date +%Y%m%d` failed, so today's date could not be read";
        return Err(message.to_owned());
    }
    let date = String::from_utf8(output.stdout)
        .map_err(|e| format!("`date +%Y%m%d` printed non-UTF-8 text: {e}"))?;
    Ok(date.trim().to_owned())
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
