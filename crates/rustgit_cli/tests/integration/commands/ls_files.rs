use crate::common::{git, populate_folder, rustgit};
use test_utils::test_path;

#[test]
fn files() {
    let working_dir = test_path!();

    let git = || git(&working_dir);
    let rustgit = || rustgit(&working_dir);

    git().init();

    assert!(rustgit().ls_files().is_empty());
    populate_folder(&working_dir);

    assert!(rustgit().ls_files().is_empty());

    git().stage(".");

    insta::assert_snapshot!("after stage", rustgit().ls_files());
}
