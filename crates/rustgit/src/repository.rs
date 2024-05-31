use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::database::DatabaseWriteError;
use crate::object::{Object, ObjectBuffer};
use crate::{database::Database, oid::ObjectId};

/// Abstraction for a Git Repository
pub struct Repository {
    pub repository_dir: PathBuf,
    pub git_dir: PathBuf,
    database: Database,
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
        let database = Database::open(&git_dir);
        Repository {
            repository_dir,
            git_dir,
            database,
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

    pub fn object_path_from_obj(&self, oid: ObjectId) -> PathBuf {
        self.database.object_path_from_oid(oid)
    }

    /// Calculate the oid of an object, write the object to the database, and return the oid
    pub fn write_object(&self, object: &impl Object) -> Result<ObjectId, DatabaseWriteError> {
        let buffer = object.to_buffer();
        let oid = ObjectId::from_object_buffer(&buffer);
        self.database.write_object_buffer(oid, &buffer)?;
        Ok(oid)
    }

    /// Write an already in-memory object
    pub fn write_object_buffer(
        &self,
        oid: ObjectId,
        object_buffer: &ObjectBuffer,
    ) -> Result<(), DatabaseWriteError> {
        self.database.write_object_buffer(oid, object_buffer)
    }
}
