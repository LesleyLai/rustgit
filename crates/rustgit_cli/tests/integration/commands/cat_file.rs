use crate::common::{git, rustgit, test_path};
use assert_cmd::prelude::*;
use predicates::prelude::*;
use rustgit::hash::Sha1HashHexString;

// cat-file -p <blob>
#[test]
fn pretty_print_blob() -> anyhow::Result<()> {
    let file_content = "hello world";
    let file_name = "file.txt";

    let working_dir = test_path!();

    git(&working_dir).init();

    // write content to file.txt
    let file_path = working_dir.join("file.txt");
    std::fs::write(&file_path, file_content)?;

    // git hash-object -w
    let hash_object_cmd = git(&working_dir)
        .args(["hash-object", "-w", file_name])
        .assert()
        .success();
    let hash = Sha1HashHexString::from_u8_slice(&hash_object_cmd.get_output().stdout)?;

    // rustgit cat-file -p
    rustgit(&working_dir)
        .args(["cat-file", "-p", &hash])
        .assert()
        .success()
        .stdout(predicate::eq(file_content));

    Ok(())
}
