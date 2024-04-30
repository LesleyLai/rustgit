// Utilities related to Git Object

use crate::hash::Sha1Hash;
use crate::write_utils::write_object;
use crate::Repository;
use anyhow::Context;
use std::fmt::{Display, Formatter};

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

/// Given full data of a git object and its Sha1 hash, write it to disk

pub struct CommitTreeArgs {
    pub parent_commit_sha: Option<Sha1Hash>,
    pub message: String,
    pub tree_sha: Sha1Hash,
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
pub fn commit_tree(repository: &Repository, args: CommitTreeArgs) -> anyhow::Result<Sha1Hash> {
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

    let object = ObjectBuffer::new(ObjectType::Commit, &content.as_bytes());
    let hash = Sha1Hash::from_object(&object);
    write_object(&repository, &object, hash).context("failed to write commit object to disk")?;

    Ok(hash)
}
