use crate::{
    object::{Object, ObjectBuffer, ObjectType},
    oid::ObjectId,
};
use chrono::prelude::*;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub time: DateTime<Local>,
}

impl Display for Author {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} <{}> {}",
            self.name,
            self.email,
            self.time.format("%s %z")
        )
    }
}

/// In memory data representation of a git blob object
pub struct Commit {
    tree: ObjectId,
    parent_commit: Option<ObjectId>,
    author: Author,
    message: String,
}

impl Commit {
    pub fn new(
        tree: ObjectId,
        parent_commit: Option<ObjectId>,
        author: Author,
        message: String,
    ) -> Self {
        Commit {
            tree,
            parent_commit,
            author,
            message,
        }
    }
}

impl Object for Commit {
    fn to_buffer(&self) -> ObjectBuffer {
        let mut content = String::new();
        content.push_str(&format!("tree {}\n", self.tree));
        if let Some(parent_commit_sha) = &self.parent_commit {
            content.push_str(&format!("parent {parent_commit_sha}\n"));
        }

        content.push_str(&format!(
            "author {}
committer {}

{}
",
            &self.author, &self.author, &self.message
        ));

        ObjectBuffer::new(ObjectType::Commit, &content.as_bytes())
    }
}
