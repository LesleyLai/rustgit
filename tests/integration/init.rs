use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;

use crate::common::{clear_dir, git_command, TEST_DIR};

#[test]
fn init() -> anyhow::Result<()> {
    let working_dir = TEST_DIR.join("init");
    clear_dir(&working_dir)?;

    let git_dir = working_dir.join(".git");

    let mut cmd: Command = git_command();
    cmd.args(["init"])
        .current_dir(&working_dir)
        .assert()
        .success()
        .stdout(predicate::str::starts_with(format!(
            "Initialized empty Git repository in {}",
            git_dir.display()
        )));

    assert!(git_dir.exists());
    assert!(git_dir.join("objects").exists());
    assert!(git_dir.join("refs").exists());

    let head_path = git_dir.join("HEAD");
    assert!(head_path.exists());
    assert_eq!(fs::read_to_string(&head_path)?, "ref: refs/heads/main\n");

    Ok(())
}
