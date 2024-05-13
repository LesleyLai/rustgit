use crate::hash::Sha1Hash;
use crate::read_ext::ReadExt;
use anyhow::Context;
use std::collections::BTreeMap;
use std::{
    fs::File,
    io,
    io::{BufRead, BufReader, ErrorKind, Read},
    path::{Path, PathBuf},
};

// None-path part of an entry
#[derive(Debug, Copy, Clone)]
struct EntryData {
    metadata: EntryMetadata,
    oid: Sha1Hash,
}

/// Memory representation of an index file.
pub struct Index {
    // note: paths here should already stripe repository path prefix
    entries: BTreeMap<PathBuf, EntryData>,
}

/// A reference to an entry
#[derive(Debug, Copy, Clone)]
pub struct EntryRef<'index> {
    pub metadata: EntryMetadata,
    pub oid: Sha1Hash,
    pub path: &'index Path,
}

// Read header of index and return the number of entries
fn read_header(reader: &mut impl Read) -> anyhow::Result<usize> {
    let header_signature = reader.read_exact_4()?;
    anyhow::ensure!(
        matches!(&header_signature, b"DIRC"),
        "index signature is not DIRC"
    );

    let version_number = u32::from_be_bytes(reader.read_exact_4()?);
    anyhow::ensure!(version_number == 2, "rustgit only support index version 2");
    let entry_count = u32::from_be_bytes(reader.read_exact_4()?);
    Ok(entry_count as usize)
}

#[derive(Debug, Copy, Clone)]
pub struct EntryMetadata {
    pub ctime_seconds: u32,
    pub ctime_nanoseconds: u32,
    pub mtime_seconds: u32,
    pub mtime_nanoseconds: u32,
    pub dev: u32,
    pub ino: u32,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub file_size: u32,
}

fn read_metadata(reader: &mut impl Read) -> io::Result<EntryMetadata> {
    let ctime_seconds = u32::from_be_bytes(reader.read_exact_4()?);
    let ctime_nanoseconds = u32::from_be_bytes(reader.read_exact_4()?);
    let mtime_seconds = u32::from_be_bytes(reader.read_exact_4()?);
    let mtime_nanoseconds = u32::from_be_bytes(reader.read_exact_4()?);

    let dev = u32::from_be_bytes(reader.read_exact_4()?);
    let ino = u32::from_be_bytes(reader.read_exact_4()?);
    let mode = u32::from_be_bytes(reader.read_exact_4()?);

    let uid = u32::from_be_bytes(reader.read_exact_4()?);
    let gid = u32::from_be_bytes(reader.read_exact_4()?);
    let file_size = u32::from_be_bytes(reader.read_exact_4()?);

    Ok(EntryMetadata {
        ctime_seconds,
        ctime_nanoseconds,
        mtime_seconds,
        mtime_nanoseconds,
        dev,
        ino,
        mode,
        uid,
        gid,
        file_size,
    })
}

fn write_metadata(writer: &mut impl io::Write, metadata: &EntryMetadata) -> io::Result<()> {
    writer.write(&u32::to_be_bytes(metadata.ctime_seconds))?;
    writer.write(&u32::to_be_bytes(metadata.ctime_nanoseconds))?;

    writer.write(&u32::to_be_bytes(metadata.mtime_seconds))?;
    writer.write(&u32::to_be_bytes(metadata.mtime_nanoseconds))?;

    writer.write(&u32::to_be_bytes(metadata.dev))?;
    writer.write(&u32::to_be_bytes(metadata.ino))?;
    writer.write(&u32::to_be_bytes(metadata.mode))?;
    writer.write(&u32::to_be_bytes(metadata.uid))?;
    writer.write(&u32::to_be_bytes(metadata.gid))?;
    writer.write(&u32::to_be_bytes(metadata.file_size))?;
    Ok(())
}

impl Index {
    /// Open an on-memory version of a git index from .git/index file
    ///
    /// If .git/index file doesn't exist, create an empty index
    pub fn open(index_path: &Path) -> anyhow::Result<Self> {
        let index_file = match File::open(index_path) {
            Err(e) if e.kind() == ErrorKind::NotFound => {
                return Ok(Index {
                    entries: Default::default(),
                })
            }
            index_file => index_file.context("open .git/index file")?,
        };
        let mut entries = BTreeMap::new();

        let mut reader = BufReader::new(index_file);
        let entry_count = read_header(&mut reader)?;

        for _ in 0..entry_count {
            const METADATA_SIZE: usize = 40;
            const SHA_SIZE: usize = 20;
            const PATH_LEN_SIZE: usize = 2;

            const MIN_ENTRY_SIZE: usize = METADATA_SIZE + SHA_SIZE + PATH_LEN_SIZE;

            let metadata = read_metadata(&mut reader)?;

            let oid = Sha1Hash(reader.read_exact_n::<SHA_SIZE>()?);

            let path_length = u16::from_be_bytes(reader.read_exact_n::<2>()?);

            let mut path = vec![];
            let length = reader
                .read_until(0, &mut path)
                .context("failed to read header from tree object")?;
            assert_eq!(length, usize::from(path_length + 1));

            // Exclude null byte in path
            let path = PathBuf::from(std::str::from_utf8(&path[..path.len() - 1])?);

            entries.insert(path, EntryData { metadata, oid });

            // consume padding bits
            reader.seek_relative(
                i64::try_from(8 - (MIN_ENTRY_SIZE + path_length as usize) % 8 - 1).unwrap(),
            )?;
        }

        Ok(Index { entries })
    }

    /// Create an iterator that will return every entry contained in the index at the time of creation.
    /// Entries are returned in order, sorted by path.
    pub fn iter(&self) -> impl Iterator<Item = EntryRef> {
        self.entries.iter().map(|(path, data)| EntryRef {
            metadata: data.metadata,
            oid: data.oid,
            path,
        })
    }

    pub fn add(&mut self, path: PathBuf, oid: Sha1Hash) {
        // TODO: properly get metadata
        let metadata = EntryMetadata {
            ctime_seconds: 0,
            ctime_nanoseconds: 0,
            mtime_seconds: 0,
            mtime_nanoseconds: 0,
            dev: 0,
            ino: 0,
            mode: 0o100644,
            uid: 0,
            gid: 0,
            file_size: 0,
        };
        self.entries.insert(path, EntryData { oid, metadata });
    }

    pub fn write_to(&self, file: &mut impl io::Write) -> anyhow::Result<()> {
        let entry_size = u32::try_from(self.entries.len()).unwrap();

        file.write(b"DIRC")?;
        file.write(&u32::to_be_bytes(2))?;
        file.write(&u32::to_be_bytes(entry_size))?;

        // todo: write other data properly
        for (entry_path, entry_data) in &self.entries {
            write_metadata(file, &entry_data.metadata)?;

            // hash
            file.write(&entry_data.oid.0)?;

            let path_bytes = entry_path.to_str().unwrap().as_bytes();
            let path_len = path_bytes.len();

            // path size
            file.write(&u16::to_be_bytes(u16::try_from(path_bytes.len())?))?;

            let total_size = 62 + path_len;
            let padded_size = (total_size / 8 + 1) * 8;

            file.write(path_bytes)?;

            // write paddings
            for _ in 0..(padded_size - total_size) {
                file.write(&[0])?;
            }
        }

        Ok(())
    }
}
