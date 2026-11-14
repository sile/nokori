use std::path::Path;
use std::process::Command;

fn nokori(dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_nokori"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("failed to run nokori")
}

fn temp_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "nokori-test-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock before epoch")
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

#[test]
fn init_creates_the_three_readmes() {
    // One for the backlog and one for each state, so that neither state is a
    // directory that appears when it is first used: a repository that has
    // closed nothing still has the directory an item is closed into.
    let dir = temp_dir("creates");
    let output = nokori(&dir, &["init"]);
    assert!(output.status.success(), "{output:?}");

    for path in [
        "backlog/README.md",
        "backlog/open/README.md",
        "backlog/closed/README.md",
    ] {
        let readme = dir.join(path);
        assert!(readme.is_file(), "{path} was not created");
        let content = std::fs::read_to_string(&readme).expect("failed to read generated README");
        // Each one is short and says where the rest is: a file `init` writes
        // once cannot follow a change of policy, so the policy cannot be in
        // it.
        assert!(
            content.contains("nokori explain flow"),
            "{path} does not point at the rule: {content}"
        );
    }

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}

#[test]
fn init_fails_when_readme_already_exists() {
    let dir = temp_dir("exists");
    std::fs::create_dir_all(dir.join("backlog")).expect("failed to create backlog dir");
    std::fs::write(dir.join("backlog/README.md"), "existing").expect("failed to seed README");

    let output = nokori(&dir, &["init"]);
    assert!(!output.status.success(), "init should have failed");

    let content =
        std::fs::read_to_string(dir.join("backlog/README.md")).expect("failed to read README");
    assert_eq!(content, "existing", "existing README was overwritten");

    std::fs::remove_dir_all(&dir).expect("failed to clean up");
}
