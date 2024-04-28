// Utilities related to Git Object

use crate::hash::Sha1Hash;
use anyhow::Context;
use std::env::VarError;
use std::fmt::{Display, Formatter};
use std::fs;

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

// In memory data representation of a git objects
pub struct ObjectBuffer {
    // The byte string that represent a git object
    data: Box<[u8]>,
}

impl ObjectBuffer {
    // Given the type of object and content of a file, create a valid git object
    pub fn new(typ: ObjectType, content: &[u8]) -> Self {
        let mut data = format!("{} {}\0", typ, content.len()).into_bytes();
        data.extend_from_slice(&content);
        Self {
            data: data.into_boxed_slice(),
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

/// Given an SHA1 hash of a git object, return back its path in .git/objects
fn object_path_from_hash(object_hash: Sha1Hash) -> std::path::PathBuf {
    // TODO: delete this function

    // TODO: support shortest unique object hashes
    let path = std::env::current_dir().expect("Cannot get working directory");
    let hash_hex_string = object_hash.to_hex_string().0;
    let (s1, s2) = hash_hex_string.split_at(2);
    path.join(".git/objects")
        .join(std::str::from_utf8(s1).unwrap())
        .join(std::str::from_utf8(s2).unwrap())
}

/// Given full data of a git object and its Sha1 hash, write it to disk
fn write_object(data: &[u8], object_hash: Sha1Hash) -> anyhow::Result<()> {
    use flate2::read::ZlibEncoder;
    use std::{fs::File, io::prelude::*};

    // TODO: write to a temporary object first

    let tree_object_path = object_path_from_hash(object_hash);
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

pub struct CommitTreeArgs {
    pub parent_commit_sha: Option<Sha1Hash>,
    pub message: String,
    pub tree_sha: Sha1Hash,
}

fn get_env_var(key: &str) -> anyhow::Result<Option<String>> {
    let str = match std::env::var(key) {
        Ok(name) => Some(name),
        Err(VarError::NotPresent) => None,
        Err(VarError::NotUnicode(_)) => anyhow::bail!("${} is invalid utf-8", key),
    };
    Ok(str)
}

/// Commit a tree and returns commit sha
pub fn commit_tree(args: CommitTreeArgs) -> anyhow::Result<Sha1Hash> {
    let mut content = String::new();
    content.push_str(&format!("tree {}\n", args.tree_sha));
    if let Some(parent_commit_sha) = &args.parent_commit_sha {
        content.push_str(&format!("parent {parent_commit_sha}\n"));
    }

    let author_name = get_env_var("GIT_AUTHOR_NAME")?.unwrap_or("lesley lai".to_string());
    let author_email =
        get_env_var("GIT_AUTHOR_EMAIL")?.unwrap_or("lesley@lesleylai.info".to_string());

    // TODO: don't hardcode author names
    content.push_str(&format!(
        "author {author_name} <{author_email}> 1243040974 -0700
committer {author_name} <{author_email}> 1243040974 -0700

{}
",
        args.message
    ));

    let header = format!("commit {}\0", content.len());
    let data = header + &content;

    let hash = Sha1Hash::from_data(data.as_bytes());
    write_object(data.as_bytes(), hash).context("failed to write commit object to disk")?;

    Ok(hash)
}
