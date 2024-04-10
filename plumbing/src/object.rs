// Utilities related to Git Object

use crate::hash::{Sha1Hash, Sha1HashHexString};
use anyhow::Context;

#[allow(dead_code)]
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
}

pub struct Object {
    // The byte string that represent a git object
    pub data: Vec<u8>,
}

impl Object {
    // Given the type of object and content of a file, create a valid git object
    pub fn new(typ: ObjectType, content: &[u8]) -> Self {
        if typ != ObjectType::Blob {
            unimplemented!("only blobs are supported for now!");
        }

        let mut data = format!("blob {}\0", content.len()).into_bytes();
        data.extend_from_slice(&content);
        Self { data }
    }
}

// Given an SHA1 hash of a git object, return back its path in .git/objects
pub fn object_path_from_hash(object_hash: &Sha1HashHexString) -> std::path::PathBuf {
    // TODO: support shortest unique object hashes
    let path = std::env::current_dir().expect("Cannot get working directory");
    let (s1, s2) = object_hash.0.split_at(2);
    path.join(".git/objects")
        .join(std::str::from_utf8(s1).unwrap())
        .join(std::str::from_utf8(s2).unwrap())
}

/// Given content of a git object and its Sha1 hash, write it to disk
pub fn write_object(content: &[u8], object_hash: &Sha1Hash) -> anyhow::Result<()> {
    use flate2::read::ZlibEncoder;
    use std::{fs, fs::File, io::prelude::*};

    // TODO: write to a temporary object first

    let tree_object_path = object_path_from_hash(&object_hash.to_hex_string());
    fs::create_dir_all(tree_object_path.parent().unwrap()).with_context(|| {
        format!(
            "Failed to create parent directory for object {}",
            object_hash
        )
    })?;

    let mut encoder = ZlibEncoder::new(content, Default::default());
    let mut output = vec![];
    encoder.read_to_end(&mut output)?;
    let mut file = File::create(&tree_object_path)
        .with_context(|| format!("Failed to create file at {}", &tree_object_path.display()))?;
    file.write_all(&output)
        .with_context(|| format!("fail to writing file to {}", &tree_object_path.display()))?;

    Ok(())
}
