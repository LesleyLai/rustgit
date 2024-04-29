use std::path::PathBuf;
use thiserror::Error;

use crate::hash::Sha1Hash;
use crate::lockfile::Lockfile;
use crate::references::hash_from_reference;

/// Abstraction for a Git Repository
pub struct Repository {
    pub repository_directory: PathBuf,
    #[allow(dead_code)]
    git_directory: PathBuf,
    objects_directory: PathBuf,
}

#[derive(Copy, Clone, Error, Debug)]
pub enum RepositoryInitError {
    #[error("not a git repository (or any of the parent directories)")]
    NotARepository,
}

impl Repository {
    pub fn search_and_open() -> Result<Self, RepositoryInitError> {
        let mut repository_directory = std::env::current_dir().unwrap();
        loop {
            if repository_directory.join(".git").exists() {
                break;
            }

            if !repository_directory.pop() {
                return Err(RepositoryInitError::NotARepository);
            }
        }

        let git_directory = repository_directory.join(".git");

        let objects_directory = git_directory.join("objects");

        Ok(Repository {
            repository_directory,
            git_directory,
            objects_directory,
        })
    }

    pub fn object_path_from_hash(&self, object_hash: Sha1Hash) -> PathBuf {
        // TODO: support shortest unique object hashes
        let hash_hex_string = object_hash.to_hex_string().0;
        let (s1, s2) = hash_hex_string.split_at(2);

        self.objects_directory
            .join(std::str::from_utf8(s1).unwrap())
            .join(std::str::from_utf8(s2).unwrap())
    }

    /// Retrieve and resolve the reference pointed at by HEAD.
    pub fn head(&self) -> anyhow::Result<Option<Sha1Hash>> {
        let head_path = self.git_directory.join("HEAD");

        let head_content = {
            let _head_lock = Lockfile::new(&head_path)?;
            std::fs::read_to_string(head_path)?
        };

        if head_content.starts_with("ref: ") {
            hash_from_reference(&self.repository_directory, head_content[5..].trim())
        } else {
            // detached head
            let hash = Sha1Hash::from_unvalidated_hex_string(head_content.trim())?;
            Ok(Some(hash))
        }
    }
}