use crate::common::{git, populate_folder, rustgit};
use test_utils::{test_path, touch};

#[test]
fn write_tree() {
    let working_dir = test_path!();
    let git = || git(&working_dir);

    git().init();

    populate_folder(&working_dir);

    git().stage(&["."]);

    let tree_hash = rustgit(&working_dir).write_tree();

    insta::assert_snapshot!(git().ls_tree(tree_hash));
}

// Git actually sorts the file list for the entire project before building the tree, rather than
// sorting entries within trees themselves
#[test]
fn sort_in_correct_order() {
    let working_dir = test_path!();

    let git = || git(&working_dir);
    git().init();

    std::fs::create_dir(working_dir.join("foo")).unwrap();
    touch(&working_dir.join("foo.txt")).unwrap();
    touch(&working_dir.join("foo").join("bar.txt")).unwrap();

    git().stage(&["."]);

    let tree_hash = rustgit(&working_dir).write_tree();

    insta::assert_snapshot!(git().ls_tree(tree_hash));
}

// When the index doesn't contain all the working tree
// This test that write-tree actually use the index rather than the working tree
#[test]
fn partial_index() {
    let working_dir = test_path!();
    let git = || git(&working_dir);

    git().init();

    populate_folder(&working_dir);

    git().stage(&["file1.txt", "dir1"]);

    let tree_hash = rustgit(&working_dir).write_tree();
    insta::assert_snapshot!(git().ls_tree(tree_hash));
}
