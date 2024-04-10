use crate::common::{clear_dir, git_command_real, git_command_rust, TEST_DIR};
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::str::from_utf8;

// cat-file -p <blob>
#[test]
fn pretty_print_blob() -> anyhow::Result<()> {
    let file_content = "hello world";
    let file_name = "file.txt";

    let working_dir = TEST_DIR.join("cat-file");
    clear_dir(&working_dir).unwrap();

    // git init
    git_command_real(&working_dir).args(["init"]).output()?;

    // write content to file.txt
    let file_path = working_dir.join("file.txt");
    std::fs::write(&file_path, file_content)?;

    // git hash-object -w
    let hash_object_cmd = git_command_real(&working_dir)
        .args(["hash-object", "-w", file_name])
        .assert()
        .success();
    let hash = &hash_object_cmd.get_output().stdout;

    // rustgit cat-file -p
    git_command_rust(&working_dir)
        .args(["cat-file", "-p", from_utf8(hash).unwrap().trim()])
        .assert()
        .success()
        .stdout(predicate::eq(file_content));

    Ok(())
}
