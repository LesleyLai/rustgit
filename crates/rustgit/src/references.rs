use crate::oid::{ObjectId, SHA1ValidationError};
use crate::Repository;
use std::{fs, io::ErrorKind};

// A ref is a variable that holds a single object identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ref {
    /// A peeled reference contains an object id
    Peeled(ObjectId),

    /// A symbolic reference contains a fully-qualified name
    Symbolic(String),
}

#[derive(thiserror::Error, Debug)]
pub enum ReferenceError {
    #[error("Reference does not exist: {0}")]
    NotExist(String),
    #[error("IO Error")]
    IOError(#[from] std::io::Error),

    #[error("SHA1 Error")]
    SHA1Error(#[from] SHA1ValidationError),
}

type RefResult<T> = Result<T, ReferenceError>;

impl Repository {
    /// Given a name, trying to find the corresponding reference
    /// Returns None if no references exist
    pub fn try_find_reference(&self, name: &str) -> RefResult<Option<Ref>> {
        let ref_path = self.git_dir.join(name);
        let ref_content = match fs::read_to_string(ref_path) {
            Err(e) if e.kind() == ErrorKind::NotFound => return Ok(None),
            ref_content => ref_content,
        }?;

        let reference = if ref_content.starts_with("ref: ") {
            Ref::Symbolic(ref_content[5..].trim().to_string())
        } else {
            let oid = ObjectId::from_unvalidated_sh1_hex_string(ref_content.trim())?;
            Ref::Peeled(oid)
        };
        Ok(Some(reference))
    }

    /// Given a reference, recursively try to find the underlying object id
    pub fn peel_reference(&self, reference: &Ref) -> RefResult<ObjectId> {
        match reference {
            Ref::Peeled(oid) => Ok(*oid),
            Ref::Symbolic(name) => {
                let inner = self.try_find_reference(name)?;
                match inner {
                    Some(inner) => self.peel_reference(&inner),
                    None => Err(ReferenceError::NotExist(name.clone())),
                }
            }
        }
    }
}
