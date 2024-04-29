use crate::common::{git, populate_folder, rustgit};
use assert_cmd::prelude::*;
use std::str::from_utf8;
use test_utils::test_path;

#[test]
fn write_tree() -> anyhow::Result<()> {
    let working_dir = test_path!();

    git(&working_dir).init();

    populate_folder(&working_dir);

    git(&working_dir).stage(".");

    let tree_hash = rustgit(&working_dir).write_tree()?;

    let command = git(&working_dir)
        .args(["ls-tree", &tree_hash])
        .assert()
        .success();
    let output = from_utf8(&command.get_output().stdout)?;
    insta::assert_snapshot!(output);

    Ok(())
}
