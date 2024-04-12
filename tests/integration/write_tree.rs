use crate::common::{git, populate_folder, rustgit, test_path};
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn write_tree() -> anyhow::Result<()> {
    let working_dir = test_path!();

    git::init(&working_dir)?;

    populate_folder(&working_dir);

    git::stage_current_dir(&working_dir)?;

    let tree_hash = rustgit::new_command(&working_dir).write_tree()?;

    let expected = "040000 tree 91e1483644d087af54a6e8aac15a08c482bb9fb1\tdir1
040000 tree cf8e933fedbe540f9881ba4dc34b034785834227\tdir2
100644 blob b6fc4c620b67d95f953a5c1c1230aaab5db5a1b0\tfile1.txt";

    git::new_command(&working_dir)
        .args(["ls-tree", &tree_hash])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(expected));

    Ok(())
}
