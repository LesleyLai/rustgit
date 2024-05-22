use assert_cmd::assert::Assert;
use assert_cmd::prelude::*;
use rustgit::oid::Sha1HashHexString;
use std::str::from_utf8;
use std::{ffi::OsStr, fs, path::Path, process::Command};

pub(crate) struct GitCommand(Command);

impl GitCommand {
    fn new(mut command: Command, working_dir: &Path) -> Self {
        command.current_dir(&working_dir);
        GitCommand(command)
    }

    pub fn env<K, V>(mut self, key: K, val: V) -> Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.0.env(key, val);
        self
    }

    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut GitCommand {
        self.0.arg(arg);
        self
    }

    pub(crate) fn args<I, S>(&mut self, args: I) -> &mut GitCommand
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.0.args(args);
        self
    }

    pub(crate) fn as_command(&mut self) -> &mut Command {
        &mut self.0
    }

    pub(crate) fn init(mut self) {
        self.arg("init").0.assert().success();
    }

    pub(crate) fn ls_files(mut self) -> String {
        let assert = self.arg("ls-files").assert().success();

        from_utf8(&assert.get_output().stdout).unwrap().to_string()
    }

    pub(crate) fn ls_files_stage(mut self) -> String {
        let assert = self.args(["ls-files", "-s"]).assert().success();

        from_utf8(&assert.get_output().stdout).unwrap().to_string()
    }

    pub(crate) fn status(mut self) -> String {
        let assert = self.arg("status").assert().success();
        from_utf8(&assert.get_output().stdout).unwrap().to_string()
    }

    pub(crate) fn write_tree(mut self) -> anyhow::Result<Sha1HashHexString> {
        let assert = self.args(["write-tree"]).assert().success();
        Sha1HashHexString::from_u8_slice(&assert.get_output().stdout)
    }

    pub(crate) fn rev_parse<I, S>(mut self, args: I) -> anyhow::Result<Sha1HashHexString>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let output = self.0.arg("rev-parse").args(args).output()?;
        anyhow::ensure!(output.status.success());
        Sha1HashHexString::from_u8_slice(&output.stdout)
    }

    pub(crate) fn log(mut self) -> String {
        let git_log_command = self.arg("log").assert().success();
        String::from_utf8_lossy(&git_log_command.get_output().stdout).to_string()
    }

    pub(crate) fn stage(mut self, dir: &str) {
        self.args(["stage", dir]).assert().success();
    }

    pub(crate) fn commit_tree(
        mut self,
        tree_hash: Sha1HashHexString,
        parent_hash: Option<Sha1HashHexString>,
        msg: &str,
    ) -> Sha1HashHexString {
        let commit_tree = self.args(["commit-tree", &tree_hash, "-m", msg]);
        if let Some(parent_hash) = parent_hash {
            commit_tree.args(["-p", &parent_hash]);
        }

        let commit_tree = commit_tree.assert().success();

        Sha1HashHexString::from_u8_slice(&commit_tree.get_output().stdout)
            .expect("commit-tree does not give back a valid sha1")
    }

    pub(crate) fn commit(mut self, msg: &str) {
        self.args(["commit", "-m", msg]).assert().success();
    }

    pub(crate) fn cat_file<I, S>(mut self, args: I) -> String
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let cat_file = self.0.arg("cat-file").args(args).assert().success();
        from_utf8(&cat_file.get_output().stdout)
            .expect("output of cat-file is not valid utf8")
            .to_string()
    }
}

impl<'c> OutputAssertExt for &'c mut GitCommand {
    fn assert(self) -> Assert {
        self.0.assert()
    }
}

/// Create a command for the real git
pub(crate) fn git(working_dir: &Path) -> GitCommand {
    let command = Command::new("git");
    GitCommand::new(command, &working_dir)
}

/// Create a command for rustgit
pub(crate) fn rustgit(working_dir: &Path) -> GitCommand {
    let command = Command::cargo_bin("rustgit").expect("Cannot find rustgit executable");
    GitCommand::new(command, &working_dir)
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

pub(crate) trait InstaSettingsExt {
    fn add_sha1_filter(&mut self);
}

impl InstaSettingsExt for insta::Settings {
    fn add_sha1_filter(&mut self) {
        self.add_filter(r"\b[[:xdigit:]]{40}\b", "[sha1]");
    }
}

pub(crate) fn head_sha(working_dir: &Path) -> anyhow::Result<Sha1HashHexString> {
    let hash = git(working_dir)
        .args(["rev-parse", "HEAD"])
        .as_command()
        .output()?
        .stdout;
    Sha1HashHexString::from_u8_slice(&hash)
}
