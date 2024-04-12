use crate::common::{
    git_command_real, git_command_rust, git_init, git_stage_current_dir, test_path,
};
use assert_cmd::prelude::*;
use rustgit_plumbing::utils::remove_last_if_endline;
use std::fs;
use std::path::Path;
use std::str::from_utf8;

fn setup_test_folder(dir: &Path) {
    let file1 = dir.join("file1.txt");
    fs::write(&file1, "hello").unwrap();

    let dir1 = dir.join("dir1");
    fs::create_dir(&dir1).unwrap();
    let file2 = dir1.join("file_in_dir1_1");
    let file3 = dir1.join("file_in_dir1_2");
    fs::write(&file2, "file_in_dir1").unwrap();
    fs::write(&file3, "file_in_dir1 2").unwrap();

    let dir2 = dir.join("dir2");
    fs::create_dir(&dir2).unwrap();
    let file4 = dir2.join("file_in_dir2_1");
    fs::write(&file4, "file_in_dir2").unwrap();
}

fn head_sha(working_dir: &Path) -> anyhow::Result<[u8; 40]> {
    // get last commit sha
    let hash_str = git_command_real(working_dir)
        .args(["rev-parse", "HEAD"])
        .output()?
        .stdout;
    let hash: [u8; 40] = remove_last_if_endline(&hash_str).try_into().unwrap();
    Ok(hash)
}

// rustgit commit-tree <tree_sha> -p <commit_sha> -m <message>
#[test]
fn with_parent() -> anyhow::Result<()> {
    let working_dir = test_path!();

    git_init(&working_dir)?;

    setup_test_folder(&working_dir);

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

    let output = git_command_rust(&working_dir)
        .args([
            "commit-tree",
            from_utf8(remove_last_if_endline(&tree_hash)).unwrap(),
            "-p",
            from_utf8(parent_commit_hash.as_slice()).unwrap(),
            "-m",
            "\"another commit\"",
        ])
        .output()?;

    assert!(output.status.success());

    let commit_hash = remove_last_if_endline(&output.stdout);

    // git cat-file commit <sha>
    let output = git_command_real(&working_dir)
        .args(["cat-file", "commit", from_utf8(commit_hash).unwrap()])
        .output()?;
    assert!(output.status.success());
    let catfile_output = from_utf8(&output.stdout).unwrap();

    // TODO: more accurate assertions
    assert!(catfile_output.contains(&format!("tree {}", from_utf8(&tree_hash).unwrap())));
    assert!(catfile_output.contains(&format!(
        "parent {}",
        from_utf8(&parent_commit_hash).unwrap()
    )));
    assert!(catfile_output.contains("author"));
    assert!(catfile_output.contains("committer"));

    Ok(())
}
