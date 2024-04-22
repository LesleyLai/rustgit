use crate::common::{git, head_sha, populate_folder, rustgit, test_path, InstaSettingsExt};
use rustgit_plumbing::hash::Sha1HashHexString;
use std::fs;

// rustgit commit-tree <tree_sha> -p <commit_sha> -m <message>
#[test]
fn with_parent() -> anyhow::Result<()> {
    let working_dir = test_path!();

    git(&working_dir).init();

    populate_folder(&working_dir);

    git(&working_dir).stage(".");

    // Initial commit
    git(&working_dir).commit("initial commit");

    // get last commit sha
    let parent_commit_hash = head_sha(&working_dir)?;

    // create another file
    fs::write(&working_dir.join("another file.txt"), "another file").unwrap();
    git(&working_dir).stage(".");

    let tree_hash = git(&working_dir).write_tree()?;

    let output = rustgit(&working_dir)
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
    let output = git(&working_dir)
        .args(["cat-file", "commit", &commit_hash])
        .output()?;
    assert!(output.status.success());

    let catfile_output = std::str::from_utf8(&output.stdout).unwrap();

    let mut settings = insta::Settings::clone_current();
    settings.add_sha1_filter();
    settings.add_filter(
        r" .* <.*@.*\..*> \d{10} [+|-]\d{4}",
        " [name] <[email]> [date_seconds] [timezone]",
    );
    settings.bind(|| {
        insta::assert_snapshot!(catfile_output);
    });

    Ok(())
}
