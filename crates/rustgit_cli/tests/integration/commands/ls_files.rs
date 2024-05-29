use crate::common::{git, populate_folder, rustgit};
use lazy_static::lazy_static;
use std::{fs, path::PathBuf};
use test_utils::TEST_DIR;

lazy_static! {
    static ref WORKING_DIR: PathBuf = {
        let working_dir = TEST_DIR.join("ls-files");
        fs::create_dir(&working_dir).unwrap();

        git(&working_dir).init();

        assert!(rustgit(&working_dir).ls_files().is_empty());

        populate_folder(&working_dir);

        assert!(rustgit(&working_dir).ls_files().is_empty());

        git(&working_dir).stage(&["."]);

        working_dir
    };
}

#[test]
fn files() {
    let working_dir = &WORKING_DIR;
    insta::assert_snapshot!(rustgit(&working_dir).ls_files());
}

#[test]
fn stage() {
    let working_dir = &WORKING_DIR;
    insta::assert_snapshot!(rustgit(&working_dir).ls_files_stage());
}
