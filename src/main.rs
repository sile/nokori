#![forbid(unsafe_code)]

const BACKLOG_README_TEMPLATE: &str = include_str!("../templates/backlog/README.md");
const BACKLOG_README_PATH: &str = "backlog/README.md";

fn main() -> noargs::Result<()> {
    let mut args = noargs::raw_args();
    args.metadata_mut().app_name = env!("CARGO_PKG_NAME");
    args.metadata_mut().app_description = env!("CARGO_PKG_DESCRIPTION");

    if noargs::VERSION_FLAG.take(&mut args).is_present() {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    noargs::HELP_FLAG.take_help(&mut args);

    let _ = try_run_init(&mut args)?;

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
