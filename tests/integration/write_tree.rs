use crate::common::{git, populate_folder, rustgit, test_path};
use assert_cmd::prelude::*;
use std::str::from_utf8;

#[test]
fn write_tree() -> anyhow::Result<()> {
    let working_dir = test_path!();

    git::init(&working_dir)?;

    populate_folder(&working_dir);

    git::stage_current_dir(&working_dir)?;

    let tree_hash = rustgit::new_command(&working_dir).write_tree()?;

    let command = git::new_command(&working_dir)
        .args(["ls-tree", &tree_hash])
        .assert()
        .success();
    let output = from_utf8(&command.get_output().stdout)?;
    insta::assert_snapshot!(output);

    Ok(())
}
