// Utilities related to Git Object

mod blob;
mod commit;
mod header;
mod tree;

pub use {
    blob::Blob,
    commit::{Author, Commit},
    header::{read_header, ObjectHeader},
    tree::{read_tree_object, Tree, TreeEntry},
};

use crate::parse_utils::ParseU64Error;
use chrono::Local;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ObjectReadError {
    #[error("expect a {0} object, get a {1}")]
    MismatchObjectType(ObjectType, ObjectType),

    #[error("object file misses a \0 seperator between header and content")]
    HeaderReadError(std::io::Error),

    #[error("object file misses a space separator in header")]
    MissingSpaceSeparator,

    #[error("unknown object type")]
    UnknownObjectType,

    #[error("failed to parse an u64 number")]
    ParseNumError(#[from] ParseU64Error),

    #[error("error while reading content of the object")]
    ContentReadError(std::io::Error),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
}

impl ObjectType {
    fn parse(bytes: &[u8]) -> Option<ObjectType> {
        match bytes {
            b"blob" => Some(ObjectType::Blob),
            b"tree" => Some(ObjectType::Tree),
            b"commit" => Some(ObjectType::Commit),
            _ => None,
        }
    }
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

fn get_env_var(key: &str) -> Option<String> {
    std::env::var(key).ok()
}
pub fn get_author() -> Author {
    // TODO: don't hardcode author names
    let author_name = get_env_var("GIT_AUTHOR_NAME").unwrap_or("lesley lai".to_string());
    let author_email =
        get_env_var("GIT_AUTHOR_EMAIL").unwrap_or("lesley@lesleylai.info".to_string());

    Author {
        name: author_name,
        email: author_email,
        time: Local::now(),
    }
}
