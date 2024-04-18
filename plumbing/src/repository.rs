use std::path::PathBuf;

use thiserror::Error;

/// Abstraction for a Git Repository
pub struct Repository {
    pub repository_dir: PathBuf,
}

#[derive(Copy, Clone, Error, Debug)]
pub enum RepositoryInitError {
    #[error("not a git repository (or any of the parent directories)")]
    NotARepository,
}

impl Repository {
    pub fn new() -> Result<Self, RepositoryInitError> {
        let mut repository_dir = std::env::current_dir().unwrap();
        loop {
            if repository_dir.join(".git").exists() {
                break;
            }

            if !repository_dir.pop() {
                return Err(RepositoryInitError::NotARepository);
            }
        }

        Ok(Repository { repository_dir })
    }
}
