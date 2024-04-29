use crate::common::{git, populate_folder, rustgit, InstaSettingsExt};
use std::fs;
use test_utils::test_path;

fn with_log_insta_setting<F: FnOnce() -> ()>(callback: F) {
    let mut settings = insta::Settings::clone_current();
    settings.add_sha1_filter();
    settings.add_filter(r"Author: .* <.*@.*\..*>", "Author: [name] <[email]>");
    settings.add_filter(r"Date: .*\n", "Date: [date]\n");
    settings.bind(|| {
        callback();
    });
}

// rustgit commit-tree <tree_sha> -p <commit_sha> -m <message>
#[test]
fn initial_commit() -> anyhow::Result<()> {
    let working_dir = test_path!();
    let git = || git(&working_dir);

    git().init();
    populate_folder(&working_dir);
    git().stage(".");

    // Initial commit
    rustgit(&working_dir).commit("initial commit");

    // verify result with git log
    let log = git().log();
    with_log_insta_setting(|| {
        insta::assert_snapshot!(log);
    });

    Ok(())
}

#[test]
fn commit_with_parent() -> anyhow::Result<()> {
    let working_dir = test_path!();
    let git = || git(&working_dir);

    git().init();
    populate_folder(&working_dir);
    git().stage(".");

    // Initial commit
    git().commit("initial commit");

    // adds another file
    fs::write(&working_dir.join("another file.txt"), "another file").unwrap();
    git().stage(".");

    // another commit
    rustgit(&working_dir).commit("another commit");

    // verify result with git log
    let log = git().log();
    with_log_insta_setting(|| {
        insta::assert_snapshot!(log);
    });

    Ok(())
}
