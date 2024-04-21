use crate::repository::Repository;
use anyhow::Context;
use rustgit_plumbing::object::ObjectType;
use rustgit_plumbing::{hash::Sha1Hash, object::ObjectBuffer};
use std::fs;

/// Given full data of a git object and its Sha1 hash, write it to disk
pub fn write_object(
    repository: &Repository,
    object_buffer: &ObjectBuffer,
    object_hash: Sha1Hash,
) -> anyhow::Result<()> {
    use flate2::read::ZlibEncoder;
    use std::io::prelude::*;

    // TODO: write to a temporary object first

    let tree_object_path = repository.object_path_from_hash(object_hash);
    fs::create_dir_all(tree_object_path.parent().unwrap()).with_context(|| {
        format!(
            "Failed to create parent directory for object {}",
            object_hash
        )
    })?;

    let mut encoder = ZlibEncoder::new(object_buffer.data(), Default::default());
    let mut output = vec![];
    encoder.read_to_end(&mut output)?;

    // TODO: make sure that the object file doesn't already exist

    let mut file = fs::File::create(&tree_object_path)
        .with_context(|| format!("Failed to create file at {}", &tree_object_path.display()))?;
    file.write_all(&output)
        .with_context(|| format!("fail to writing file to {}", &tree_object_path.display()))?;

    Ok(())
}

// Recursively create a tree object and return the tree SHA
// TODO: should write index rather than a directory
pub fn write_tree(repository: &Repository, path: &std::path::Path) -> anyhow::Result<Sha1Hash> {
    // TODO: windows support
    use std::io::Write;
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
            let blob = ObjectBuffer::new(ObjectType::Blob, content.as_bytes());
            let object_hash = Sha1Hash::from_object(&blob);
            write_object(repository, &blob, object_hash)?;

            object_hash
        } else if child_path.is_dir() {
            if child_path.ends_with(".git") {
                // Ignore .git directory!
                continue;
            } else {
                // For some reason mode for directory is always 40000, is that correct?
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
    let hash = Sha1Hash::from_object(&tree);
    write_object(repository, &tree, hash)?;
    Ok(hash)
}