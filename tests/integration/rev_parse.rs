use crate::common::{git, populate_folder, rustgit, TEST_DIR};

use assert_cmd::prelude::*;
use lazy_static::lazy_static;
use predicates::prelude::predicate;
use rustgit_plumbing::hash::Sha1HashHexString;
use std::{fs, path::PathBuf};

lazy_static! {
    static ref WORKING_DIR: PathBuf = {
        let working_dir = TEST_DIR.join("rev-parse");
        fs::create_dir(&working_dir).unwrap();

        git::init(&working_dir).unwrap();
        populate_folder(&working_dir);
        git::stage_current_dir(&working_dir).unwrap();

        git::new_command(&working_dir).commit("message");
        working_dir
    };
    static ref EXPECTED_HEAD_HASH: Sha1HashHexString =
        git::new_command(&WORKING_DIR).rev_parse(["HEAD"]).unwrap();
}

// git rev-parse HEAD
#[test]
fn head() {
    rustgit::new_command(&WORKING_DIR)
        .args(["rev-parse", "HEAD"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(&EXPECTED_HEAD_HASH.to_string()));
}

// cd dir1
// git rev-parse HEAD
#[test]
fn in_subfolder() {
    rustgit::new_command(&WORKING_DIR.join("dir1"))
        .args(["rev-parse", "HEAD"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(&EXPECTED_HEAD_HASH.to_string()));
}
