use crate::common::{git, populate_folder, rustgit};
use assert_cmd::prelude::*;
use std::str::from_utf8;
use test_utils::{test_path, touch};

#[test]
fn write_tree() -> anyhow::Result<()> {
    let working_dir = test_path!();
    let git = || git(&working_dir);

    git().init();

    populate_folder(&working_dir);

    git().stage(".");

    let tree_hash = rustgit(&working_dir).write_tree()?;

    let command = git().args(["ls-tree", &tree_hash]).assert().success();
    let output = from_utf8(&command.get_output().stdout)?;
    insta::assert_snapshot!(output);

    Ok(())
}

// Git actually sorts the file list for the entire project before building the tree, rather than
// sorting entries within trees themselves
#[test]
fn sort_in_correct_order() -> anyhow::Result<()> {
    let working_dir = test_path!();

    let git = || git(&working_dir);
    git().init();

    std::fs::create_dir(working_dir.join("foo"))?;
    touch(&working_dir.join("foo.txt"))?;
    touch(&working_dir.join("foo").join("bar.txt"))?;

    git().stage(".");

    let tree_hash = rustgit(&working_dir).write_tree()?;

    let command = git().args(["ls-tree", &tree_hash]).assert().success();
    let output = from_utf8(&command.get_output().stdout)?;
    insta::assert_snapshot!(output);

    Ok(())
}
