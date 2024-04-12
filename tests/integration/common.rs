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
