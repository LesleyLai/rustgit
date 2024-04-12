use crate::common::{
    git_command_real, git_command_rust, git_init, git_stage_current_dir, populate_folder, test_path,
};
use assert_cmd::prelude::*;
use predicates::prelude::*;

use rustgit_plumbing::hash::Sha1HashHexString;

#[test]
fn write_tree() -> anyhow::Result<()> {
    let working_dir = test_path!();

    git_init(&working_dir)?;

    populate_folder(&working_dir);

    git_stage_current_dir(&working_dir)?;

    let tree_hash = git_command_rust(&working_dir)
        .args(["write-tree"])
        .output()?
        .stdout;
    let tree_hash = Sha1HashHexString::from_u8_slice(&tree_hash)?;

    let expected = "040000 tree 91e1483644d087af54a6e8aac15a08c482bb9fb1\tdir1
040000 tree cf8e933fedbe540f9881ba4dc34b034785834227\tdir2
100644 blob b6fc4c620b67d95f953a5c1c1230aaab5db5a1b0\tfile1.txt";

    git_command_real(&working_dir)
        .args(["ls-tree", &tree_hash])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(expected));

    Ok(())
}
