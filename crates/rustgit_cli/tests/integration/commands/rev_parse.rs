use crate::common::{git, populate_folder, rustgit};
use test_utils::TEST_DIR;

use assert_cmd::prelude::*;
use lazy_static::lazy_static;
use predicates::prelude::predicate;
use rustgit::hash::Sha1HashHexString;
use std::{fs, path::PathBuf};

lazy_static! {
    static ref WORKING_DIR: PathBuf = {
        let working_dir = TEST_DIR.join("rev-parse");
        fs::create_dir(&working_dir).unwrap();

        git(&working_dir).init();
        populate_folder(&working_dir);
        git(&working_dir).stage(".");

        git(&working_dir).commit("message");
        working_dir
    };
    static ref EXPECTED_HEAD_HASH: Sha1HashHexString =
        git(&WORKING_DIR).rev_parse(["HEAD"]).unwrap();
}

// git rev-parse <sha1>
#[test]
fn sha1() {
    let sha1 = "e9fd0d28a8db6bf20521abb3f13e840cb963def1";
    rustgit(&WORKING_DIR)
        .args(["rev-parse", sha1])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(sha1));
}

// git rev-parse HEAD
#[test]
fn head() {
    rustgit(&WORKING_DIR)
        .args(["rev-parse", "HEAD"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(&EXPECTED_HEAD_HASH.to_string()));
}

// cd dir1
// git rev-parse HEAD
#[test]
fn in_subfolder() {
    rustgit(&WORKING_DIR.join("dir1"))
        .args(["rev-parse", "HEAD"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(&EXPECTED_HEAD_HASH.to_string()));
}
