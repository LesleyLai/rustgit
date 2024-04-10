use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;
use assert_cmd::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TEST_DIR: PathBuf = {
        let temp_dir = std::env::temp_dir();
        let dir = temp_dir.join("rustgit_tests");

        fs::create_dir_all(&dir).expect("cannot create a temporary directory for test!");
        dir
    };
}

pub fn clear_dir(path: &Path) -> anyhow::Result<()> {
    let _ = fs::remove_dir_all(&path); // supress error
    fs::create_dir(&path).context(format!("failed to clear {}", path.display()))
}

/// The real git command
pub fn git_command_real(working_dir: &Path) -> std::process::Command {
    let mut command = std::process::Command::new("git");
    command.current_dir(&working_dir);
    command
}

/// rustgit under test
pub fn git_command_rust(working_dir: &Path) -> std::process::Command {
    let mut command = std::process::Command::cargo_bin("rustgit").expect("Cannot find executable");
    command.current_dir(&working_dir);
    command
}
