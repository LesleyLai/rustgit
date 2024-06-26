// Object Database at .git/objects
// Also including an in-memory cache

use crate::{
    object::{Object, ObjectBuffer},
    object_reader::ObjectReader,
    oid::ObjectId,
    Repository,
};
use std::{
    fs,
    fs::File,
    path::{Path, PathBuf},
};
use thiserror::Error;

pub(crate) struct Database {
    objects_dir: PathBuf,
}

#[derive(Error, Debug)]
#[error("failed to write to git object at {path}")]
pub struct DatabaseWriteError {
    path: PathBuf,
    source: std::io::Error,
}

impl DatabaseWriteError {
    pub fn new(path: PathBuf, source: std::io::Error) -> Self {
        DatabaseWriteError { path, source }
    }
}

impl Database {
    pub(crate) fn open(git_dir: &Path) -> Self {
        Self {
            objects_dir: git_dir.join("objects"),
        }
    }

    pub(crate) fn object_path_from_oid(&self, oid: ObjectId) -> PathBuf {
        // TODO: support shortest unique object hashes
        let hash_hex_string = oid.to_hex_string().0;
        let (s1, s2) = hash_hex_string.split_at(2);

        let mut path = self.objects_dir.clone();
        path.reserve(hash_hex_string.len() + 1);
        path.push(std::str::from_utf8(s1).unwrap());
        path.push(std::str::from_utf8(s2).unwrap());
        path
    }

    /// Given an object id, give back an object reader for the object on disk
    pub(crate) fn object_reader(&self, oid: ObjectId) -> std::io::Result<ObjectReader> {
        let file = fs::OpenOptions::new()
            .read(true)
            .create(false)
            .open(self.object_path_from_oid(oid))?;

        Ok(ObjectReader::from_file(file))
    }

    // Write an already in-memory object
    pub(crate) fn write_object_buffer(
        &self,
        oid: ObjectId,
        object_buffer: &ObjectBuffer,
    ) -> Result<(), DatabaseWriteError> {
        use flate2::read::ZlibEncoder;
        use std::io::prelude::*;

        // TODO: write to a temporary object first

        let object_path = self.object_path_from_oid(oid);
        if object_path.exists() {
            // already exist. Quit
            return Ok(());
        }

        let parent_path = object_path
            .parent()
            .expect("object path should have parent");
        fs::create_dir_all(parent_path).map_err(|source| DatabaseWriteError {
            path: parent_path.to_path_buf(),
            source,
        })?;

        let to_database_write_error = |source| DatabaseWriteError {
            path: object_path.clone(),
            source,
        };

        let mut encoder = ZlibEncoder::new(object_buffer.data(), Default::default());
        let mut output = vec![];
        encoder
            .read_to_end(&mut output)
            .map_err(to_database_write_error)?;

        let mut file = File::create(&object_path).map_err(to_database_write_error)?;
        file.write_all(&output).map_err(to_database_write_error)?;

        Ok(())
    }
}

impl Repository {
    pub fn object_reader(&self, oid: ObjectId) -> std::io::Result<ObjectReader> {
        self.database.object_reader(oid)
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
