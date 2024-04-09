use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;

use crate::common::{clear_dir, TEST_DIR};

#[test]
fn init() -> anyhow::Result<()> {
    let working_dir = TEST_DIR.join("init");
    clear_dir(&working_dir)?;

    let mut cmd = Command::cargo_bin("rustgit")?;
    let cmd = cmd.args(["init"]).current_dir(&working_dir);

    // First call to git init should succeed
    let git_dir = working_dir.join(".git");

    cmd.assert()
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
