use lazy_static::lazy_static;
use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

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
use rustgit_plumbing::hash::Sha1HashHexString;

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

pub(crate) struct GitCommand(Command);

impl GitCommand {
    pub(crate) fn new(mut command: Command, working_dir: &Path) -> Self {
        command.current_dir(&working_dir);
        GitCommand(command)
    }
    pub(crate) fn args<I, S>(&mut self, args: I) -> &mut Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.0.args(args)
    }

    pub(crate) fn write_tree(mut self) -> anyhow::Result<Sha1HashHexString> {
        let output = self.args(["write-tree"]).output()?;
        anyhow::ensure!(output.status.success());
        Sha1HashHexString::from_u8_slice(&output.stdout)
    }
}

/// rustgit under test
pub(crate) mod rustgit {
    use crate::common::GitCommand;
    use assert_cmd::prelude::*;
    use std::path::Path;

    pub(crate) fn new_command(working_dir: &Path) -> GitCommand {
        let command = std::process::Command::cargo_bin("rustgit").expect("Cannot find executable");
        GitCommand::new(command, &working_dir)
    }
}

/// Contains command for the real git
pub(crate) mod git {
    use crate::common::GitCommand;
    use anyhow::Context;
    use std::path::Path;

    /// The real git command
    pub(crate) fn new_command(working_dir: &Path) -> GitCommand {
        let command = std::process::Command::new("git");
        GitCommand::new(command, &working_dir)
    }

    pub(crate) fn init(working_dir: &Path) -> anyhow::Result<()> {
        let output = new_command(&working_dir)
            .args(["init"])
            .output()
            .context("Failed to call git init")?;

        anyhow::ensure!(output.status.success(), "git init returns none zero");
        Ok(())
    }

    pub(crate) fn stage_current_dir(working_dir: &Path) -> anyhow::Result<()> {
        let output = new_command(&working_dir)
            .args(["stage", "."])
            .output()
            .context("Failed to call git stage")?;

        anyhow::ensure!(output.status.success(), "git stage returns none zero");
        Ok(())
    }
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

pub(crate) fn with_common_insta_settings(f: impl FnOnce() -> ()) {
    insta::with_settings!({filters => vec![
        (r"\b[[:xdigit:]]{40}\b", "[sha1]"),
    ]}, {
        f()
    })
}
