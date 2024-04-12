use crate::common::{
    git_command_real, git_command_rust, git_init, git_stage_current_dir, TEST_DIR,
};

use assert_cmd::prelude::*;
use lazy_static::lazy_static;
use predicates::prelude::*;
use rustgit_plumbing::utils::remove_last;
use std::{
    fs,
    path::{Path, PathBuf},
    str::from_utf8,
};

fn setup_test_folder(dir: &Path) {
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

lazy_static! {
    static ref LS_TREE_SETUP_DATA: (PathBuf, [u8; 40]) = {
        let working_dir = TEST_DIR.join("ls-tree");
        fs::create_dir(&working_dir).unwrap();

        git_init(&working_dir).unwrap();

        setup_test_folder(&working_dir);

        git_stage_current_dir(&working_dir).unwrap();

        let tree_hash = git_command_real(&working_dir)
            .args(["write-tree"])
            .output()
            .unwrap()
            .stdout;

        let tree_hash: [u8; 40] = remove_last(&tree_hash).try_into().unwrap();

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
        .args([
            "ls-tree",
            "--name-only",
            from_utf8(tree_hash.as_slice()).unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(expected));

    Ok(())
}
