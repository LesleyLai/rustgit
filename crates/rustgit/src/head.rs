use crate::references::ReferenceError;
use crate::{oid::ObjectId, references::Ref, Repository};

#[derive(Debug)]
pub enum Head {
    /// An existing reference that HEAD point to
    Symbolic { name: String, reference: Ref },

    /// The yet-to-be-created reference the symbolic HEAD refers to
    ///
    /// This is the case of a new repository without a commit
    Unborn(String),

    /// A detached head
    Detached(ObjectId),
}

impl Head {
    /// Get the name of the reference of the symbolic HEAD
    ///
    /// Returns None if it is a detached head
    pub fn referent_name(&self) -> Option<&str> {
        match self {
            Head::Symbolic { name, .. } => Some(&name),
            Head::Unborn(name) => Some(&name),
            Head::Detached(_) => None,
        }
    }

    /// HEAD points to a yet-to-be-created reference
    ///
    /// This is the case of a new repository without a commit
    pub fn is_unborn(&self) -> bool {
        matches!(self, Head::Unborn(_))
    }
}

impl Repository {
    /// Retrieve the state of `HEAD` reference
    pub fn head(&self) -> Result<Head, ReferenceError> {
        let head_ref = self.try_find_reference("HEAD")?.expect("HEAD should exist");
        let head_state = match head_ref {
            Ref::Peeled(oid) => Head::Detached(oid),
            Ref::Symbolic(name) => {
                let inner = self.try_find_reference(&name)?;
                match inner {
                    Some(reference) => Head::Symbolic { name, reference },
                    None => Head::Unborn(name),
                }
            }
        };
        Ok(head_state)
    }

    /// Retrieve and resolve the object ID pointed at by HEAD.
    ///
    /// Note that this can fail for various reason, notably when the repository is freshly created
    /// (In this case we will have a `ReferenceError::NotExist`)
    pub fn head_id(&self) -> Result<ObjectId, ReferenceError> {
        let head = self.try_find_reference("HEAD")?.expect("HEAD should exist");
        self.peel_reference(&head)
    }
}
