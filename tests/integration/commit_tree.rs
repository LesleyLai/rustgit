use crate::common::{
    git_command_real, git_command_rust, git_init, git_stage_current_dir, populate_folder, test_path,
};
use assert_cmd::prelude::*;
use rustgit_plumbing::hash::Sha1HashHexString;
use std::{fs, path::Path};

fn head_sha(working_dir: &Path) -> anyhow::Result<Sha1HashHexString> {
    let hash = git_command_real(working_dir)
        .args(["rev-parse", "HEAD"])
        .output()?
        .stdout;
    Sha1HashHexString::from_u8_slice(&hash)
}

// rustgit commit-tree <tree_sha> -p <commit_sha> -m <message>
#[test]
fn with_parent() -> anyhow::Result<()> {
    let working_dir = test_path!();

    git_init(&working_dir)?;

    populate_folder(&working_dir);

    git_stage_current_dir(&working_dir)?;

    // Initial commit
    git_command_real(&working_dir)
        .args(["commit", "-m", "\"initial commit\""])
        .assert()
        .success();

    // get last commit sha
    let parent_commit_hash = head_sha(&working_dir)?;

    // create another file
    fs::write(&working_dir.join("another file.txt"), "another file").unwrap();
    git_stage_current_dir(&working_dir)?;

    let tree_hash = git_command_real(&working_dir)
        .args(["write-tree"])
        .output()
        .unwrap()
        .stdout;
    let tree_hash = Sha1HashHexString::from_u8_slice(&tree_hash)?;

    let output = git_command_rust(&working_dir)
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
    let output = git_command_real(&working_dir)
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
