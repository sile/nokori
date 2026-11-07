#![forbid(unsafe_code)]

const BACKLOG_README_TEMPLATE: &str = include_str!("../templates/backlog/README.md");

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
        .doc("Create the backlog/ directory and its README.md")
        .take(args)
        .is_present()
    {
        return Ok(false);
    }

    if args.metadata().help_mode {
        return Ok(true);
    }

    let backlog_dir = std::path::Path::new("backlog");
    let readme_path = backlog_dir.join("README.md");
    if readme_path.exists() {
        return Err(noargs::Error::other(
            args,
            format!("{} already exists", readme_path.display()),
        ));
    }

    std::fs::create_dir_all(backlog_dir).map_err(|e| noargs::Error::other(args, e))?;
    std::fs::write(&readme_path, BACKLOG_README_TEMPLATE)
        .map_err(|e| noargs::Error::other(args, e))?;
    println!("Created {}", readme_path.display());
    println!();
    println!("Next, create an item:");
    println!();
    println!("  nokori new <kind> <slug>          # ready to start");
    println!("  nokori new <kind> <slug> --draft  # not ready to start yet");
    println!();
    println!("Run `nokori new --help` to see the usage and `nokori explain kinds`");
    println!("to choose a kind.");

    Ok(true)
}
