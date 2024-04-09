use crate::object::write_object;
use crate::sha1hash::Sha1Hash;

use anyhow::Context;
use io::prelude::*;
use std::{fs, io};

// Recursively create a tree object and return the tree SHA
fn write_tree_impl(path: &std::path::Path) -> anyhow::Result<Sha1Hash> {
    // TODO: windows support
    use std::os::unix::fs::PermissionsExt;

    assert!(path.is_dir());

    let mut body: Vec<u8> = vec![];

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
            let body = fs::read_to_string(child_path.to_str().unwrap())?;
            let header = format!("blob {}\0", body.len());
            let blob_data = header + &body;
            let object_hash = Sha1Hash::from_data(blob_data.as_bytes());
            write_object(blob_data.as_bytes(), &object_hash)?;

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

        write!(&mut body, "{:o} {}\0", mode, name.to_string_lossy())?;
        body.extend_from_slice(&object_hash.0);
    }

    let mut content: Vec<u8> = vec![];
    write!(content, "tree {}\0", body.len()).unwrap(); // header
    content.extend_from_slice(&body);

    let hash = Sha1Hash::from_data(&content);
    write_object(&content, &hash).context("failed to write tree object to disk")?;
    Ok(hash)
}

pub fn write_tree() -> anyhow::Result<()> {
    let working_dir = std::env::current_dir()?;
    let result = write_tree_impl(&working_dir)?;

    print!("{}", result.to_hex_string());

    Ok(())
}
