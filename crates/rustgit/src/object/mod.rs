// Utilities related to Git Object

mod blob;
mod commit;
mod tree;

pub use {
    blob::Blob,
    commit::{Author, Commit},
    tree::{Tree, TreeEntry, TreeEntryRef},
};

use crate::utils::remove_last;
use chrono::Local;
use std::{
    fmt::{Display, Formatter},
    io::prelude::*,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseU64Error {
    #[error("invalid digit")]
    InvalidDigit { got: u8 },

    #[error("number is too big to parse into a u64")]
    NumberTooBig,
}

#[derive(Debug, Error)]
pub enum ObjectReadError {
    #[error("object file misses a \0 seperator between header and content")]
    ReadHeaderError(std::io::Error),

    #[error("object file misses a space separator in header")]
    MissingSpaceSeparator,

    #[error("unknown object type")]
    UnknownObjectType,

    #[error("failed to parse an u64 number")]
    ParseNumError(#[from] ParseU64Error),
}

fn parse_usize(bytes: &[u8]) -> Result<usize, ParseU64Error> {
    let mut n: usize = 0;
    for &byte in bytes {
        let digit = match byte.checked_sub(b'0') {
            None => return Err(ParseU64Error::InvalidDigit { got: byte }),
            Some(digit) if digit > 9 => return Err(ParseU64Error::InvalidDigit { got: byte }),
            Some(digit) => {
                debug_assert!((0..=9).contains(&digit));
                usize::from(digit)
            }
        };
        n = n
            .checked_mul(10)
            .and_then(|n| n.checked_add(digit))
            .ok_or_else(|| ParseU64Error::NumberTooBig)?;
    }
    Ok(n)
}

#[derive(Eq, PartialEq, Copy, Clone)]
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

pub struct ObjectHeader {
    pub typ: ObjectType,
    pub size: usize,
}

pub fn read_header(decoder: &mut impl BufRead) -> Result<ObjectHeader, ObjectReadError> {
    let mut output = vec![];
    decoder
        .read_until(0, &mut output)
        .map_err(|err| ObjectReadError::ReadHeaderError(err))?;

    parse_header(&output)
}

fn parse_header(buffer: &[u8]) -> Result<ObjectHeader, ObjectReadError> {
    use ObjectReadError::*;

    let separate_point = buffer.iter().position(|&c| c == b' ');
    let separate_point = separate_point.ok_or(MissingSpaceSeparator)?;

    let (typ, mut size) = remove_last(&buffer).split_at(separate_point);
    let typ = ObjectType::parse(typ).ok_or(UnknownObjectType)?;
    size = &size[1..];
    let size: usize = parse_usize(size)?;

    Ok(ObjectHeader { typ, size })
}
