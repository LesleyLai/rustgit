use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::oid::ObjectId;

/// Abstraction for a Git Repository
pub struct Repository {
    pub repository_dir: PathBuf,
    pub git_dir: PathBuf,
}

#[derive(Copy, Clone, Error, Debug)]
pub enum RepositorySearchError {
    #[error("not a git repository (or any of the parent directories)")]
    NotARepository,
}

impl Repository {
    /// Creates a new Git repository in the given folder.
    pub fn init(path: &Path) -> std::io::Result<Repository> {
        let git_dir = path.join(".git");
        fs::create_dir(&git_dir)?;
        fs::create_dir(&git_dir.join("objects"))?;
        fs::create_dir(&git_dir.join("refs"))?;
        fs::write(&git_dir.join("HEAD"), "ref: refs/heads/main\n")?;

        Ok(Self::open(path.to_path_buf(), git_dir))
    }

    /// Open an existing git repository
    pub fn open(repository_dir: PathBuf, git_dir: PathBuf) -> Repository {
        Repository {
            repository_dir,
            git_dir,
        }
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
        Ok(Self::open(repository_directory, git_directory))
    }

    pub fn object_path_from_hash(&self, object_hash: ObjectId) -> PathBuf {
        // TODO: support shortest unique object hashes
        let hash_hex_string = object_hash.to_hex_string().0;
        let (s1, s2) = hash_hex_string.split_at(2);

        let mut path = self.git_dir.clone();
        path.reserve(10 + s1.len() + s2.len());
        path.push("objects");
        path.push(std::str::from_utf8(s1).unwrap());
        path.push(std::str::from_utf8(s2).unwrap());
        path
    }
}
