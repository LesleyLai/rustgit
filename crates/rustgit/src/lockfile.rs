use std::fmt::{Display, Formatter};
use std::{fs::File, io, mem::ManuallyDrop, path::Path, path::PathBuf};
use thiserror::Error;

struct LockFile {
    path: PathBuf,
    pub file: ManuallyDrop<File>,
}

impl LockFile {
    /// path: A path without .lock extension
    #[allow(dead_code)]
    fn new(path: &Path) -> Result<LockFile, LockFileError> {
        let lock_path = path.with_extension("lock");
        let maybe_file = File::options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&lock_path);
        match maybe_file {
            Ok(file) => Ok(Self {
                path: lock_path,
                file: ManuallyDrop::new(file),
            }),
            Err(err) => Err(LockFileError::from_io(err, lock_path)),
        }
    }
}

impl Drop for LockFile {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.file);
        }
        std::fs::remove_file(&self.path).expect("Failed to remove lock file. This is likely a bug");
    }
}

// Error Handling
#[derive(Debug)]
#[allow(dead_code)]
enum LockFileErrorKind {
    LockTaken,
    Io,
}

#[derive(Debug, Error)]
#[allow(dead_code)]
struct LockFileError {
    path: PathBuf,
    kind: LockFileErrorKind,
    #[source]
    source: io::Error,
}

impl Display for LockFileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to create '{}'", self.path.display())?;
        match self.kind {
            LockFileErrorKind::LockTaken => {
                write!(f, "File exists")
            }
            LockFileErrorKind::Io => {
                write!(f, "IO error: {}", self.source)
            }
        }
    }
}

impl LockFileError {
    #[allow(dead_code)]
    fn from_io(e: io::Error, path: PathBuf) -> Self {
        let kind = match e.kind() {
            io::ErrorKind::AlreadyExists => LockFileErrorKind::LockTaken,
            _ => LockFileErrorKind::Io,
        };
        Self {
            path,
            kind,
            source: e,
        }
    }
}
