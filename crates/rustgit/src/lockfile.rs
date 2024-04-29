use anyhow::Context;
use std::fmt::{Display, Formatter};
use std::{fs, fs::File, io, path::Path, path::PathBuf};
use thiserror::Error;

pub struct Lockfile {
    pub path: PathBuf,
    file: Option<File>,
}

impl Lockfile {
    /// Create a lockfile at the given path.
    /// Returns an error if the path does not have a parent directory, or if the parent directory could not be created.
    ///
    /// * `path` - A path without the .lock extension
    pub fn new(path: &Path) -> Result<Lockfile, LockfileError> {
        let lock_path = path.with_extension("lock");
        let maybe_file = File::options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&lock_path);
        match maybe_file {
            Ok(file) => Ok(Self {
                path: lock_path,
                file: Some(file),
            }),
            Err(err) => Err(LockfileError::from_io(err, lock_path)),
        }
    }

    /// Rename <lockfile>.lock to <lockfile> and release the lock
    pub fn commit(mut self) -> anyhow::Result<()> {
        let path = self.path.clone();
        let file = self.file.take().unwrap();
        drop(file);

        let no_extension_path = path.with_extension("");
        fs::rename(&path, &no_extension_path)
            .with_context(|| format!("commit lockfile at {}", path.display()))
    }
}

impl Drop for Lockfile {
    fn drop(&mut self) {
        if self.file.is_some() {
            self.file = None;
            fs::remove_file(&self.path).expect("Failed to remove lock file. This is likely a bug");
        }
    }
}

// Error Handling
#[derive(Debug)]
pub enum LockfileErrorKind {
    LockTaken,
    Io,
}

#[derive(Debug, Error)]
pub struct LockfileError {
    path: PathBuf,
    kind: LockfileErrorKind,
    #[source]
    source: io::Error,
}

impl Display for LockfileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to create '{}'", self.path.display())?;
        match self.kind {
            LockfileErrorKind::LockTaken => {
                write!(f, "File exists")
            }
            LockfileErrorKind::Io => {
                write!(f, "IO error: {}", self.source)
            }
        }
    }
}

impl LockfileError {
    fn from_io(e: io::Error, path: PathBuf) -> Self {
        let kind = match e.kind() {
            io::ErrorKind::AlreadyExists => LockfileErrorKind::LockTaken,
            _ => LockfileErrorKind::Io,
        };
        Self {
            path,
            kind,
            source: e,
        }
    }
}

// Implement Read/Write for lockfile
impl io::Read for Lockfile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file.as_ref().unwrap().read(buf)
    }
}

impl io::Write for Lockfile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file.as_ref().unwrap().write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.file.as_ref().unwrap().flush()
    }
}

impl io::Seek for Lockfile {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.file.as_ref().unwrap().seek(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_utils::test_path;

    #[test]
    fn lockfile() {
        let test_path = test_path!();

        let head_path = test_path.join("HEAD");
        {
            let _lock = Lockfile::new(&head_path).expect("Grab a lock file");

            // can't grab another lock
            assert!(matches!(
                Lockfile::new(&head_path),
                Err(LockfileError {
                    kind: LockfileErrorKind::LockTaken,
                    ..
                })
            ));

            let _lock = Lockfile::new(&test_path.join("index"))
                .expect("Grab another lock file of different name");
        }

        {
            let _lock = Lockfile::new(&head_path)
                .expect("Can grab another lock after the previous lock expired");
        }
    }

    #[test]
    fn existing_lockfile() {
        let test_path = test_path!();
        let head_path = test_path.join("HEAD");

        std::fs::write(&head_path.with_extension("lock"), "")
            .expect("Failed to create a lock file");

        // can't grab another lock when there is an existing lock
        assert!(matches!(
            Lockfile::new(&head_path),
            Err(LockfileError {
                kind: LockfileErrorKind::LockTaken,
                ..
            })
        ));
    }
}
