// Utilities related to Git Object

use crate::hash::{Sha1Hash, Sha1HashHexString};
use anyhow::Context;
use std::fmt::{Display, Formatter};
use std::{fs, io::prelude::*};

#[allow(dead_code)]
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use ObjectType::*;
        match *self {
            Blob => write!(f, "blob"),
            Tree => write!(f, "tree"),
            Commit => write!(f, "commit"),
        }
    }
}

pub struct Object {
    // The byte string that represent a git object
    pub data: Vec<u8>,
}

impl Object {
    // Given the type of object and content of a file, create a valid git object
    pub fn new(typ: ObjectType, content: &[u8]) -> Self {
        let mut data = format!("{} {}\0", typ, content.len()).into_bytes();
        data.extend_from_slice(&content);
        Self { data }
    }
}

/// Given an SHA1 hash of a git object, return back its path in .git/objects
pub fn object_path_from_hash(object_hash: &Sha1HashHexString) -> std::path::PathBuf {
    // TODO: support shortest unique object hashes
    let path = std::env::current_dir().expect("Cannot get working directory");
    let (s1, s2) = object_hash.0.split_at(2);
    path.join(".git/objects")
        .join(std::str::from_utf8(s1).unwrap())
        .join(std::str::from_utf8(s2).unwrap())
}

/// Given full data of a git object and its Sha1 hash, write it to disk
pub fn write_object(data: &[u8], object_hash: &Sha1Hash) -> anyhow::Result<()> {
    use flate2::read::ZlibEncoder;
    use std::{fs::File, io::prelude::*};

    // TODO: write to a temporary object first

    let tree_object_path = object_path_from_hash(&object_hash.to_hex_string());
    fs::create_dir_all(tree_object_path.parent().unwrap()).with_context(|| {
        format!(
            "Failed to create parent directory for object {}",
            object_hash
        )
    })?;

    let mut encoder = ZlibEncoder::new(data, Default::default());
    let mut output = vec![];
    encoder.read_to_end(&mut output)?;
    let mut file = File::create(&tree_object_path)
        .with_context(|| format!("Failed to create file at {}", &tree_object_path.display()))?;
    file.write_all(&output)
        .with_context(|| format!("fail to writing file to {}", &tree_object_path.display()))?;

    Ok(())
}

// Recursively create a tree object and return the tree SHA
pub fn write_tree(path: &std::path::Path) -> anyhow::Result<Sha1Hash> {
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
                write_tree(&child_path)?
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

pub struct CommitTreeArgs {
    pub parent_commit_sha: Option<Sha1Hash>,
    pub message: String,
    pub tree_sha: Sha1Hash,
}

/// Commit a tree and returns commit sha
pub fn commit_tree(args: CommitTreeArgs) -> anyhow::Result<Sha1Hash> {
    let mut content = String::new();
    content.push_str(&format!("tree {}\n", args.tree_sha));
    if let Some(parent_commit_sha) = &args.parent_commit_sha {
        content.push_str(&format!("parent {parent_commit_sha}\n"));
    }

    // TODO: don't hardcode author names
    content.push_str(&format!(
        "author Lesley Lai <lesley@lesleylai.info> 1243040974 -0700
committer Lesley Lai <lesley@lesleylai.info> 1243040974 -0700

{}
",
        args.message
    ));

    let header = format!("commit {}\0", content.len());
    let data = header + &content;

    let hash = Sha1Hash::from_data(data.as_bytes());
    write_object(data.as_bytes(), &hash).context("failed to write commit object to disk")?;

    Ok(hash)
}
