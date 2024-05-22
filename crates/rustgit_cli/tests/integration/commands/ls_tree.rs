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
        git(&working_dir).stage(".");

        working_dir
    };
    static ref TREE_HASH: Sha1HashHexString = git(&WORKING_DIR).write_tree().unwrap();
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
