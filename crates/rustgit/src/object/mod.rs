// Utilities related to Git Object

mod blob;
mod commit;

pub use crate::object::{
    blob::Blob,
    commit::{Author, Commit},
};
use crate::oid::ObjectId;

use std::fmt::{Display, Formatter};

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

pub trait Object {
    /// Convert the object to an in-memory buffer
    fn to_buffer(&self) -> ObjectBuffer;
}

/// In memory data representation of the buffer of a git objects
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
    pub parent_commit_sha: Option<ObjectId>,
    pub message: String,
    pub tree_sha: ObjectId,
}
