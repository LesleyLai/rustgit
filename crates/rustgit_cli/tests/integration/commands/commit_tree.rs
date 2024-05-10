use crate::common::{git, head_sha, populate_folder, rustgit, InstaSettingsExt};
use std::fs;
use test_utils::test_path;

#[test]
fn with_author_as_env_var() -> anyhow::Result<()> {
    let working_dir = test_path!();
    let git = || git(&working_dir);

    git().init();

    populate_folder(&working_dir);

    git().stage(".");

    let tree_hash = git().write_tree()?;

    let commit_hash = rustgit(&working_dir)
        .env("GIT_AUTHOR_NAME", "Jane Doe")
        .env("GIT_AUTHOR_EMAIL", "jane@doe.com")
        .commit_tree(tree_hash, None, "initial commit");

    // git cat-file commit <sha>
    let output = git().cat_file(["commit", &commit_hash]);
    let mut settings = insta::Settings::clone_current();
    settings.add_sha1_filter();
    settings.add_filter(r"\d{10} [+|-]\d{4}", "[date_seconds] [timezone]");
    settings.bind(|| {
        insta::assert_snapshot!(output);
    });

    Ok(())
}

// rustgit commit-tree <tree_sha> -p <commit_sha> -m <message>
#[test]
fn with_parent() -> anyhow::Result<()> {
    let working_dir = test_path!();
    let git = || git(&working_dir);

    git().init();

    populate_folder(&working_dir);

    git().stage(".");

    // Initial commit
    git().commit("initial commit");

    // get last commit sha
    let parent_commit_hash = head_sha(&working_dir)?;

    // create another file
    fs::write(&working_dir.join("another file.txt"), "another file").unwrap();
    git().stage(".");

    let tree_hash = git().write_tree()?;

    let commit_hash =
        rustgit(&working_dir).commit_tree(tree_hash, Some(parent_commit_hash), "another commit");

    // git cat-file commit <sha>
    let output = git().cat_file(["commit", &commit_hash]);

    let mut settings = insta::Settings::clone_current();
    settings.add_sha1_filter();
    settings.add_filter(
        r" .* <.*@.*\..*> \d{10} [+|-]\d{4}",
        " [name] <[email]> [date_seconds] [timezone]",
    );
    settings.bind(|| {
        insta::assert_snapshot!(output);
    });

    Ok(())
}
