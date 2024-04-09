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

pub fn git_command() -> std::process::Command {
    std::process::Command::cargo_bin("rustgit").expect("Cannot file executable")
}
