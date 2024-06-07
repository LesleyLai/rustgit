// Utilities related to Git Object

mod blob;
mod commit;
mod tree;

pub use crate::object::{
    blob::Blob,
    commit::{Author, Commit},
    tree::{Tree, TreeEntry, TreeEntryRef},
};

use crate::utils::remove_last;
use anyhow::Context;
use chrono::Local;
use flate2::read::ZlibDecoder;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{prelude::*, BufReader};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseU64Error {
    #[error("invalid digit")]
    InvalidDigit { got: u8 },

    #[error("number is too big to parse into a u64")]
    NumberTooBig,
}

pub fn parse_usize(bytes: &[u8]) -> Result<usize, ParseU64Error> {
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

fn get_env_var(key: &str) -> anyhow::Result<Option<String>> {
    use std::env::VarError;
    let str = match std::env::var(key) {
        Ok(name) => Some(name),
        Err(VarError::NotPresent) => None,
        Err(VarError::NotUnicode(_)) => anyhow::bail!("${} is invalid utf-8", key),
    };
    Ok(str)
}
pub fn get_author() -> anyhow::Result<Author> {
    // TODO: don't hardcode author names
    let author_name = get_env_var("GIT_AUTHOR_NAME")?.unwrap_or("lesley lai".to_string());
    let author_email =
        get_env_var("GIT_AUTHOR_EMAIL")?.unwrap_or("lesley@lesleylai.info".to_string());

    Ok(Author {
        name: author_name,
        email: author_email,
        time: Local::now(),
    })
}

pub struct ObjectHeader {
    pub typ: ObjectType,
    pub size: usize,
}

pub fn read_header(decoder: &mut BufReader<ZlibDecoder<&File>>) -> anyhow::Result<ObjectHeader> {
    let mut output = vec![];
    decoder
        .read_until(0, &mut output)
        .context("failed to read header from .git/objects")?;

    parse_header(&output)
}

fn parse_header(buffer: &[u8]) -> anyhow::Result<ObjectHeader> {
    let separate_point = buffer
        .iter()
        .position(|&c| c == b' ')
        .context("header has space separator")?;
    let (typ, mut size) = remove_last(&buffer).split_at(separate_point);
    let typ = match typ {
        b"blob" => ObjectType::Blob,
        b"tree" => ObjectType::Tree,
        b"commit" => ObjectType::Commit,
        _ => anyhow::bail!("unknown object type!"),
    };
    size = &size[1..];
    let size: usize = parse_usize(size)?;

    Ok(ObjectHeader { typ, size })
}
