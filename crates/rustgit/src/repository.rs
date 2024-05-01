use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::hash::Sha1Hash;
use crate::lockfile::Lockfile;
use crate::references::hash_from_reference;

/// Abstraction for a Git Repository
pub struct Repository {
    pub repository_directory: PathBuf,
    pub git_directory: PathBuf,
}

#[derive(Copy, Clone, Error, Debug)]
pub enum RepositorySearchError {
    #[error("not a git repository (or any of the parent directories)")]
    NotARepository,
}

impl Repository {
    /// Creates a new Git repository in the given folder.
    pub fn init(path: &Path) -> std::io::Result<Repository> {
        let git_directory = path.join(".git");
        fs::create_dir(&git_directory)?;
        fs::create_dir(&git_directory.join("objects"))?;
        fs::create_dir(&git_directory.join("refs"))?;
        fs::write(&git_directory.join("HEAD"), "ref: refs/heads/main\n")?;

        Ok(Repository {
            repository_directory: path.to_path_buf(),
            git_directory,
        })
    }

    /// Upward search a git repository from a path, and open the repository if find one
    pub fn search_and_open(path: &Path) -> Result<Self, RepositorySearchError> {
        let mut repository_directory = path.to_path_buf();
        loop {
            if repository_directory.join(".git").exists() {
                break;
            }

            if !repository_directory.pop() {
                return Err(RepositorySearchError::NotARepository);
            }
        }

        let git_directory = repository_directory.join(".git");

        Ok(Repository {
            repository_directory,
            git_directory,
        })
    }

    pub fn object_path_from_hash(&self, object_hash: Sha1Hash) -> PathBuf {
        // TODO: support shortest unique object hashes
        let hash_hex_string = object_hash.to_hex_string().0;
        let (s1, s2) = hash_hex_string.split_at(2);

        let mut path = self.git_directory.clone();
        path.reserve(10 + s1.len() + s2.len());
        path.push("objects");
        path.push(std::str::from_utf8(s1).unwrap());
        path.push(std::str::from_utf8(s2).unwrap());
        path
    }

    /// Retrieve and resolve the reference pointed at by HEAD.
    pub fn head(&self) -> anyhow::Result<Option<Sha1Hash>> {
        let head_path = self.git_directory.join("HEAD");

        let head_content = {
            let _head_lock = Lockfile::new(&head_path)?;
            fs::read_to_string(head_path)?
        };

        if head_content.starts_with("ref: ") {
            hash_from_reference(&self.git_directory, head_content[5..].trim())
        } else {
            // detached head
            let hash = Sha1Hash::from_unvalidated_hex_string(head_content.trim())?;
            Ok(Some(hash))
        }
    }
}
