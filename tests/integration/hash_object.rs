use crate::common::{git_command_real, git_command_rust, git_init, test_path};

use assert_cmd::prelude::*;
use predicates::prelude::*;
use rustgit_plumbing::hash::Sha1HashHexString;
use std::{
    io::{Read, Write},
    process::Stdio,
};

#[test]
fn no_required_arg() -> anyhow::Result<()> {
    let working_dir = test_path!();

    git_init(&working_dir)?;

    git_command_rust(&working_dir)
        .args(["hash-object"])
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "required arguments were not provided",
        ));

    Ok(())
}

// hash-object -w <blob>
#[test]
fn write_blob() -> anyhow::Result<()> {
    let file_content = "hello world";
    let file_name = "file.txt";
    let expected_hash = b"95d09f2b10159347eece71399a7e2e907ea3df4f";

    let working_dir = test_path!();

    git_init(&working_dir)?;

    // write content to file.txt
    let file_path = working_dir.join("file.txt");
    std::fs::write(&file_path, file_content)?;

    // rustgit hash-object -w
    let hash_object_cmd = git_command_rust(&working_dir)
        .args(["hash-object", "-w", file_name])
        .assert()
        .success();
    let hash = Sha1HashHexString::from_u8_slice(&hash_object_cmd.get_output().stdout)?;
    assert_eq!(expected_hash, &hash.0);

    // check file content with git cat-file -p
    git_command_real(&working_dir)
        .args(["cat-file", "-p", &hash])
        .assert()
        .success()
        .stdout(predicate::eq(file_content));

    Ok(())
}

// hash-object --stdin <blob>
#[test]
fn stdin() -> anyhow::Result<()> {
    let content = "hello world";
    let expected_hash = b"95d09f2b10159347eece71399a7e2e907ea3df4f";

    let working_dir = test_path!();

    git_init(&working_dir)?;

    // rustgit hash-object
    let mut child_process = git_command_rust(&working_dir)
        .args(["hash-object", "--stdin"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    child_process
        .stdin
        .as_mut()
        .unwrap()
        .write_all(content.as_bytes())?;

    assert!(child_process.wait()?.success());
    let mut hash = String::new();
    child_process.stdout.unwrap().read_to_string(&mut hash)?;
    assert_eq!(&Sha1HashHexString::from_str(&hash)?.0, expected_hash);

    Ok(())
}
