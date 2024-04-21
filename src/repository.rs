use std::path::PathBuf;

use rustgit_plumbing::hash::Sha1Hash;
use thiserror::Error;

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
}
