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
fn init_creates_backlog_readme() {
    let dir = temp_dir("creates");
    let output = nokori(&dir, &["init"]);
    assert!(output.status.success(), "{output:?}");

    let readme = dir.join("backlog/README.md");
    assert!(readme.is_file(), "{} was not created", readme.display());

    let content = std::fs::read_to_string(&readme).expect("failed to read generated README");
    assert!(!content.is_empty());

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
