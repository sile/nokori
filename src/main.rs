#![forbid(unsafe_code)]

const BACKLOG_README_TEMPLATE: &str = include_str!("../templates/backlog/README.md");
const BACKLOG_README_PATH: &str = "backlog/README.md";
const BACKLOG_DIR: &str = "backlog";

/// The kinds `create` accepts, each with the template written into the file.
const KINDS: &[(&str, &str)] = &[("bug", include_str!("../templates/backlog/bug.md"))];

fn main() -> noargs::Result<()> {
    let mut args = noargs::raw_args();
    args.metadata_mut().app_name = env!("CARGO_PKG_NAME");
    args.metadata_mut().app_description = env!("CARGO_PKG_DESCRIPTION");

    if noargs::VERSION_FLAG.take(&mut args).is_present() {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    noargs::HELP_FLAG.take_help(&mut args);

    let _ = try_run_init(&mut args)? || try_run_create(&mut args)?;

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

    println!("Created {BACKLOG_README_PATH}");
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
    println!("Next, fill in the sections marked `TODO` and commit the file.");
    println!("When the item is settled, close it with `nokori close <file>`.");

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

    if !std::path::Path::new(BACKLOG_README_PATH).is_file() {
        return Err(format!(
            r"`{BACKLOG_README_PATH}` does not exist, so this repository has no
backlog yet. Run `nokori init` first."
        ));
    }

    let date = today()?;
    let name = format!("{date}-{kind}-{slug}.md");
    let path = std::path::Path::new(BACKLOG_DIR).join(name);
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
    let readme_path = std::path::Path::new(BACKLOG_README_PATH);
    if readme_path.exists() {
        return Err(format!(
            r"`{BACKLOG_README_PATH}` already exists, so `backlog/` is already set up in
this repository. There is nothing left for `init` to do; you can start
creating items now (see `nokori create --help`).

If this `backlog/` belongs to something else, rename it and run `nokori init`
again, or run `nokori` from another directory to use a different one."
        ));
    }

    let backlog_dir = readme_path
        .parent()
        .expect("BACKLOG_README_PATH should have a parent directory");
    std::fs::create_dir_all(backlog_dir)
        .map_err(|e| format!("failed to create `{}`: {e}", backlog_dir.display()))?;
    std::fs::write(readme_path, BACKLOG_README_TEMPLATE)
        .map_err(|e| format!("failed to write `{}`: {e}", readme_path.display()))?;
    Ok(())
}
