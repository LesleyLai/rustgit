use crate::is_executable::IsExecutable;
use crate::object::{CommitTreeArgs, ObjectType};
use crate::repository::Repository;
use crate::{object::ObjectBuffer, oid::ObjectId};
use anyhow::Context;
use std::{fs, path::Path};

use chrono::prelude::*;

// Recursively create a tree object and return the tree SHA
// TODO: should write index rather than a directory
pub fn write_tree(repository: &Repository, path: &Path) -> anyhow::Result<ObjectId> {
    use std::io::Write;

    assert!(path.is_dir());

    let mut entries: Vec<_> = fs::read_dir(path)
        .context("read directory in git write-tree")?
        .map(|entry| entry.unwrap())
        .collect();
    // sort entries alphabetically
    entries.sort_by_key(|e1| {
        if e1.path().is_dir() {
            let mut str = e1.path().as_os_str().to_os_string();
            // Adds trailing slash
            str.push("/");
            str
        } else {
            e1.path().as_os_str().to_os_string()
        }
    });

    let mut content: Vec<u8> = vec![];
    for entry in entries {
        let mode;
        let name = entry.file_name();

        let child_path = entry.path();

        let object_hash = if child_path.is_file() {
            // TODO: ensures that the objects exist in the object database

            mode = if child_path.is_executable() {
                0o100755
            } else {
                0o100644
            };

            let content = fs::read_to_string(child_path.to_str().unwrap())?;
            let blob = ObjectBuffer::new(ObjectType::Blob, content.as_bytes());
            let object_hash = ObjectId::from_object_buffer(&blob);

            object_hash
        } else if child_path.is_dir() {
            if child_path.ends_with(".git") {
                // Ignore .git directory!
                continue;
            } else {
                mode = 0o40000;

                // Recurse
                write_tree(repository, &child_path)?
            }
        } else {
            anyhow::bail!("We don't support symlink");
        };

        write!(&mut content, "{:o} {}\0", mode, name.to_string_lossy())?;
        content.extend_from_slice(&object_hash.0);
    }

    let tree = ObjectBuffer::new(ObjectType::Tree, &content);
    let hash = ObjectId::from_object_buffer(&tree);
    repository.write_object_buffer(hash, &tree)?;
    Ok(hash)
}

fn get_env_var(key: &str) -> anyhow::Result<Option<String>> {
    use std::env::VarError;
    let str = match std::env::var(key) {
        Ok(name) => Some(name),
        Err(VarError::NotPresent) => None,
        Err(VarError::NotUnicode(_)) => anyhow::bail!("${} is invalid utf-8", key),
    };
    Ok(str)
}

/// Commit a tree and returns commit sha
pub fn commit_tree(repository: &Repository, args: CommitTreeArgs) -> anyhow::Result<ObjectId> {
    // TODO: don't hardcode author names
    let author_name = get_env_var("GIT_AUTHOR_NAME")?.unwrap_or("lesley lai".to_string());
    let author_email =
        get_env_var("GIT_AUTHOR_EMAIL")?.unwrap_or("lesley@lesleylai.info".to_string());

    let author = crate::object::Author {
        name: author_name,
        email: author_email,
        time: Local::now(),
    };

    let commit =
        crate::object::Commit::new(args.tree_sha, args.parent_commit_sha, author, args.message);
    Ok(repository.write_object(&commit)?)
}
