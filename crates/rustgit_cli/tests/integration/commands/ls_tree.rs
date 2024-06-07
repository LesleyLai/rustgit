use crate::common::{git, populate_folder, rustgit};
use test_utils::TEST_DIR;

use assert_cmd::prelude::*;
use lazy_static::lazy_static;
use predicates::prelude::*;
use rustgit::oid::Sha1HashHexString;
use std::{fs, path::PathBuf};

lazy_static! {
    static ref WORKING_DIR: PathBuf = {
        let working_dir = TEST_DIR.join("ls-tree");
        fs::create_dir(&working_dir).unwrap();

        git(&working_dir).init();
        populate_folder(&working_dir);
        git(&working_dir).stage(&["."]);

        working_dir
    };
    static ref TREE_HASH: Sha1HashHexString = git(&WORKING_DIR).write_tree();
}

// ls-tree --name-only <tree-sha>
#[test]
fn name_only() -> anyhow::Result<()> {
    let expected = "dir1
dir2
file1.txt";

    rustgit(&WORKING_DIR)
        .args(["ls-tree", "--name-only", &TREE_HASH])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(expected));

    Ok(())
}

// ls-tree --name-only <tree-sha>
#[test]
fn tree() -> anyhow::Result<()> {
    let expected = "040000 tree 91e1483644d087af54a6e8aac15a08c482bb9fb1    dir1
040000 tree cf8e933fedbe540f9881ba4dc34b034785834227    dir2
100644 blob b6fc4c620b67d95f953a5c1c1230aaab5db5a1b0    file1.txt";

    rustgit(&WORKING_DIR)
        .args(["ls-tree", &TREE_HASH])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(expected));

    Ok(())
}

// ls-tree --name-only <tree-sha>
#[test]
fn not_tree() -> anyhow::Result<()> {
    let file1_oid_cmd = git(&WORKING_DIR)
        .args(["hash-object", "file1.txt"])
        .assert()
        .success();
    let file1_oid = Sha1HashHexString::from_u8_slice(&file1_oid_cmd.get_output().stdout)?;

    rustgit(&WORKING_DIR)
        .args(["ls-tree", &file1_oid])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not a tree object"));

    Ok(())
}
