use crate::common::git::stage_current_dir;
use crate::common::{git, populate_folder, rustgit, test_path};
use assert_cmd::prelude::*;
use rustgit_plumbing::hash::Sha1HashHexString;
use std::{fs, path::Path};

fn head_sha(working_dir: &Path) -> anyhow::Result<Sha1HashHexString> {
    let hash = git::new_command(working_dir)
        .args(["rev-parse", "HEAD"])
        .output()?
        .stdout;
    Sha1HashHexString::from_u8_slice(&hash)
}

// rustgit commit-tree <tree_sha> -p <commit_sha> -m <message>
#[test]
fn with_parent() -> anyhow::Result<()> {
    let working_dir = test_path!();

    git::init(&working_dir)?;

    populate_folder(&working_dir);

    stage_current_dir(&working_dir)?;

    // Initial commit
    git::new_command(&working_dir)
        .args(["commit", "-m", "\"initial commit\""])
        .assert()
        .success();

    // get last commit sha
    let parent_commit_hash = head_sha(&working_dir)?;

    // create another file
    fs::write(&working_dir.join("another file.txt"), "another file").unwrap();
    stage_current_dir(&working_dir)?;

    let tree_hash = git::new_command(&working_dir).write_tree()?;

    let output = rustgit::new_command(&working_dir)
        .args([
            "commit-tree",
            &tree_hash,
            "-p",
            &parent_commit_hash,
            "-m",
            "\"another commit\"",
        ])
        .output()?;

    assert!(output.status.success());

    let commit_hash = Sha1HashHexString::from_u8_slice(&output.stdout)?;

    // git cat-file commit <sha>
    let output = git::new_command(&working_dir)
        .args(["cat-file", "commit", &commit_hash])
        .output()?;
    assert!(output.status.success());

    // TODO: more accurate assertions
    let catfile_output = std::str::from_utf8(&output.stdout).unwrap();

    assert!(catfile_output.contains(&format!("tree {}", tree_hash)));
    assert!(catfile_output.contains(&format!("parent {}", parent_commit_hash)));
    assert!(catfile_output.contains("author"));
    assert!(catfile_output.contains("committer"));

    Ok(())
}
