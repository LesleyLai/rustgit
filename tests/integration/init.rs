use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;

use crate::common::{git_command_rust, test_path};

#[test]
fn init() -> anyhow::Result<()> {
    let working_dir = test_path!();

    let git_dir = working_dir.join(".git");

    git_command_rust(&working_dir)
        .args(["init"])
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
