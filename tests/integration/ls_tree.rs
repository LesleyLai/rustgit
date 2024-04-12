use crate::common::{
    git_command_real, git_command_rust, git_init, git_stage_current_dir, populate_folder, TEST_DIR,
};

use assert_cmd::prelude::*;
use lazy_static::lazy_static;
use predicates::prelude::*;
use rustgit_plumbing::hash::Sha1HashHexString;
use std::{
    fs,
    path::{Path, PathBuf},
};

lazy_static! {
    static ref LS_TREE_SETUP_DATA: (PathBuf, Sha1HashHexString) = {
        let working_dir = TEST_DIR.join("ls-tree");
        fs::create_dir(&working_dir).unwrap();

        git_init(&working_dir).unwrap();

        populate_folder(&working_dir);

        git_stage_current_dir(&working_dir).unwrap();

        let tree_hash = git_command_real(&working_dir)
            .args(["write-tree"])
            .output()
            .unwrap()
            .stdout;
        let tree_hash = Sha1HashHexString::from_u8_slice(&tree_hash).unwrap();

        (working_dir, tree_hash)
    };
}

// ls-tree --name-only <tree-sha>
#[test]
fn name_only() -> anyhow::Result<()> {
    let working_dir: &Path = &LS_TREE_SETUP_DATA.0;
    let tree_hash = LS_TREE_SETUP_DATA.1;

    let expected = "dir1
dir2
file1.txt";

    git_command_rust(&working_dir)
        .args(["ls-tree", "--name-only", &tree_hash])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(expected));

    Ok(())
}
