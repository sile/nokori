use std::process::Command;

/// Runs `nokori` with `args` and returns what it printed on standard output.
///
/// Help is answered before any file is read, so these runs need no backlog.
fn help(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_nokori"))
        .args(args)
        .output()
        .expect("failed to run nokori");
    assert!(output.status.success(), "{output:?}");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn the_help_points_a_new_reader_at_the_overview() {
    // The first thing somebody who has just installed this reads is the help,
    // and nothing else in it says where to start. It is `--help` and not `-h`
    // that carries this: `-h` keeps only the first line of the description,
    // which is the one sentence about what the tool is.
    let stdout = help(&["--help"]);
    assert!(
        stdout.contains("nokori explain overview"),
        "the help does not name the overview: {stdout}"
    );
}
