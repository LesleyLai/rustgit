use crate::common::{git, rustgit};
use assert_cmd::prelude::*;
use predicates::prelude::predicate;
use std::fs;
use test_utils::test_path;

#[test]
fn files() {
    let working_dir = test_path!();

    let git = || git(&working_dir);
    let rustgit = || rustgit(&working_dir);

    git().init();

    let dir = working_dir.join("dir");
    fs::create_dir(&dir).unwrap();
    fs::write(&dir.join("file1.txt"), "file1").unwrap();
    fs::write(&working_dir.join("file.txt"), "file").unwrap();

    rustgit().stage(["dir/file1.txt", "file.txt"]);

    insta::assert_snapshot!(git().ls_files());
}

#[test]
fn folder() {
    let working_dir = test_path!();

    let git = || git(&working_dir);
    let rustgit = || rustgit(&working_dir);

    git().init();

    let dir = working_dir.join("dir");
    fs::create_dir(&dir).unwrap();
    fs::write(&dir.join("file1.txt"), "file1").unwrap();
    fs::write(&working_dir.join("file.txt"), "file").unwrap();
    let inner_dir = dir.join("inner");
    fs::create_dir(&inner_dir).unwrap();
    fs::write(&inner_dir.join("inner_file.txt"), "inner file").unwrap();

    rustgit().stage(["."]);

    insta::assert_snapshot!(git().ls_files());
}

#[test]
fn incremental() {
    let working_dir = test_path!();

    let git = || git(&working_dir);
    let rustgit = || rustgit(&working_dir);

    git().init();

    let dir = working_dir.join("dir");
    fs::create_dir(&dir).unwrap();
    fs::write(&dir.join("file1.txt"), "file1").unwrap();
    fs::write(&working_dir.join("file.txt"), "file").unwrap();
    let inner_dir = dir.join("inner");
    fs::create_dir(&inner_dir).unwrap();
    fs::write(&inner_dir.join("inner_file.txt"), "inner file").unwrap();

    rustgit().stage(["file.txt"]);
    insta::assert_snapshot!("incremental 1", git().ls_files());

    rustgit().stage(["dir"]);
    insta::assert_snapshot!("incremental 2", git().ls_files());
}

#[test]
fn non_exist() {
    let working_dir = test_path!();

    let git = || git(&working_dir);
    let rustgit = || rustgit(&working_dir);

    git().init();

    rustgit()
        .args(["stage", "file.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::starts_with(
            "fatal: pathspec 'file.txt' did not match any files",
        ));
}

#[test]
fn outside_of_repo() {
    let working_dir = test_path!();

    let git = || git(&working_dir);
    let rustgit = || rustgit(&working_dir);

    git().init();

    rustgit().args(["stage", "../file.txt"]).assert().failure();
}
