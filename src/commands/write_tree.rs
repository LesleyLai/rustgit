use rustgit_plumbing::hash::Sha1Hash;
use rustgit_plumbing::object::{write_object, Object, ObjectType};

use anyhow::Context;
use io::prelude::*;
use std::{fs, io};

// Recursively create a tree object and return the tree SHA
fn write_tree_impl(path: &std::path::Path) -> anyhow::Result<Sha1Hash> {
    // TODO: windows support
    use std::os::unix::fs::PermissionsExt;

    assert!(path.is_dir());

    let mut content: Vec<u8> = vec![];

    let mut entries: Vec<_> = fs::read_dir(path)
        .context("read directory in git write-tree")?
        .map(|entry| entry.unwrap())
        .collect();
    entries.sort_by(|e1, e2| e1.path().cmp(&e2.path()));

    for entry in entries {
        let mut mode = fs::metadata(&entry.path())?.permissions().mode();
        let name = entry.file_name();

        let child_path = entry.path();

        let object_hash = if child_path.is_file() {
            // create a blob object
            let content = fs::read_to_string(child_path.to_str().unwrap())?;
            let blob = Object::new(ObjectType::Blob, content.as_bytes());
            let object_hash = Sha1Hash::from_object(&blob);
            write_object(&blob.data, &object_hash)?;

            object_hash
        } else if child_path.is_dir() {
            if child_path.ends_with(".git") {
                // Ignore .git directory!
                continue;
            } else {
                // For some reason mode for directory is always 40000, is that correct?
                mode = 0o40000;

                // Recurse
                write_tree_impl(&child_path)?
            }
        } else {
            anyhow::bail!("We don't support symlink");
        };

        write!(&mut content, "{:o} {}\0", mode, name.to_string_lossy())?;
        content.extend_from_slice(&object_hash.0);
    }

    let tree = Object::new(ObjectType::Tree, &content);
    let hash = Sha1Hash::from_object(&tree);
    write_object(&tree.data, &hash)?;
    Ok(hash)
}

pub fn write_tree() -> anyhow::Result<()> {
    let working_dir = std::env::current_dir()?;
    let result = write_tree_impl(&working_dir)?;

    print!("{}", result);

    Ok(())
}
