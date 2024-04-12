use anyhow::Context;
use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
     pub(crate) static ref TEST_DIR: PathBuf = {
        let temp_dir = std::env::temp_dir();
        let dir = temp_dir.join("rustgit_tests");

        let _ = fs::remove_dir_all(&dir); // supress error
        fs::create_dir(&dir).unwrap();
        dir
    };
}

// Copied from stdext
macro_rules! function_name {
    () => {{
        // Okay, this is ugly, I get it. However, this is the best we can get on a stable rust.
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        // `3` is the length of the `::f`.
        &name[..name.len() - 3]
    }};
}
pub(crate) use function_name;

/// Generate a unique temporary working directory for each path
macro_rules! test_path {
    () => {{
        use crate::common::{function_name, TEST_DIR};

        let directory = function_name!()[13..].replace("::", "_");
        let path = TEST_DIR.join(directory);
        std::fs::create_dir(&path).unwrap();
        path
    }};
}
pub(crate) use test_path;

/// The real git command
pub(crate) fn git_command_real(working_dir: &Path) -> std::process::Command {
    let mut command = std::process::Command::new("git");
    command.current_dir(&working_dir);
    command
}

/// rustgit under test
pub(crate) fn git_command_rust(working_dir: &Path) -> std::process::Command {
    let mut command = std::process::Command::cargo_bin("rustgit").expect("Cannot find executable");
    command.current_dir(&working_dir);
    command
}

pub(crate) fn git_init(working_dir: &Path) -> anyhow::Result<()> {
    let output = git_command_real(&working_dir)
        .args(["init"])
        .output()
        .context("Failed to call git init")?;

    anyhow::ensure!(output.status.success(), "git init returns none zero");
    Ok(())
}

pub(crate) fn git_stage_current_dir(working_dir: &Path) -> anyhow::Result<()> {
    let output = git_command_real(&working_dir)
        .args(["stage", "."])
        .output()
        .context("Failed to call git stage")?;

    anyhow::ensure!(output.status.success(), "git stage returns none zero");
    Ok(())
}

/// Populate the current folder with some files for testing
pub(crate) fn populate_folder(dir: &Path) {
    let file1 = dir.join("file1.txt");
    fs::write(&file1, "hello").unwrap();

    let dir1 = dir.join("dir1");
    fs::create_dir(&dir1).unwrap();
    let file2 = dir1.join("file_in_dir1_1");
    let file3 = dir1.join("file_in_dir1_2");
    fs::write(&file2, "file_in_dir1").unwrap();
    fs::write(&file3, "file_in_dir1 2").unwrap();

    let dir2 = dir.join("dir2");
    fs::create_dir(&dir2).unwrap();
    let file4 = dir2.join("file_in_dir2_1");
    fs::write(&file4, "file_in_dir2").unwrap();
}
