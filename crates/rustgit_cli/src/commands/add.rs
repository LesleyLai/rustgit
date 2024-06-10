use anyhow::Context;
use clap::Args;
use rustgit::index::{EntryMetadata, Index};
use rustgit::lockfile::Lockfile;
use rustgit::object::Blob;
use rustgit::Repository;
use std::collections::BTreeSet;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

#[derive(Args, Debug)]
pub struct AddArgs {
    pathspecs: Vec<String>,
}

fn parse_pathspec(pathspec: &str, current_dir: &Path, repo_path: &Path) -> anyhow::Result<PathBuf> {
    let mut path = PathBuf::from(pathspec);

    if path.is_relative() {
        path = current_dir.join(path);
    }

    let path = match path.canonicalize() {
        Err(err) if err.kind() == ErrorKind::NotFound => {
            anyhow::bail!(format!("pathspec '{}' did not match any files", pathspec))
        }
        res => res?,
    };

    anyhow::ensure!(
        path.starts_with(repo_path.canonicalize().unwrap()),
        "'{}' is outside repository",
        pathspec
    );
    Ok(path)
}

#[cfg(windows)]
fn get_metadata(path: &Path) -> anyhow::Result<EntryMetadata> {
    let metadata = fs::metadata(path)?;

    Ok(EntryMetadata {
        ctime_seconds: 0,     // TODO
        ctime_nanoseconds: 0, // TODO
        mtime_seconds: 0,     // TODO
        mtime_nanoseconds: 0, // TODO
        dev: 0,
        ino: 0,
        mode: 0o100644, // TODO
        uid: 0,
        gid: 0,
        file_size: metadata.len() as u32,
    })
}

#[cfg(unix)]
fn get_metadata(path: &Path) -> anyhow::Result<EntryMetadata> {
    use std::os::unix::fs::MetadataExt;

    let metadata = fs::metadata(path)?;

    Ok(EntryMetadata {
        ctime_seconds: metadata.ctime() as u32,
        ctime_nanoseconds: metadata.ctime_nsec() as u32,
        mtime_seconds: metadata.mtime() as u32,
        mtime_nanoseconds: metadata.mtime_nsec() as u32,
        dev: metadata.dev() as u32,
        ino: metadata.ino() as u32,
        mode: metadata.mode(),
        uid: metadata.uid(),
        gid: metadata.gid(),
        file_size: metadata.size() as u32,
    })
}

// Recursively search all files in a path
fn add_files_inside(
    path: PathBuf,
    repository_dir: &Path,
    output: &mut BTreeSet<PathBuf>,
) -> anyhow::Result<()> {
    // Ignore .git folder
    if path.ends_with(".git") {
        return Ok(());
    }

    if path.is_file() {
        output.insert(path.strip_prefix(repository_dir)?.to_path_buf());
    } else if path.is_dir() {
        for entry in fs::read_dir(path)? {
            add_files_inside(entry?.path(), repository_dir, output)?;
        }
    } else {
        anyhow::bail!("Doesn't know how to handle symlink");
    }
    Ok(())
}

pub fn add(args: AddArgs) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;

    let repo = Repository::search_and_open(&current_dir)?;

    let mut index_lockfile = Lockfile::new(&repo.git_dir.join("index"))?;

    let paths = args
        .pathspecs
        .iter()
        .map(|pathspec| parse_pathspec(pathspec, &current_dir, &repo.repository_dir))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let mut files: BTreeSet<_> = BTreeSet::new();
    for path in paths {
        add_files_inside(path, &repo.repository_dir, &mut files)?;
    }

    let mut index = Index::open(&repo.git_dir.join("index"))?;

    for file_path in files {
        let body = fs::read_to_string(&file_path)?;
        let blob = Blob::new(body.into_bytes().into_boxed_slice());
        let oid = repo.write_object(&blob)?;
        let metadata = get_metadata(&file_path)?;

        index.add(file_path, oid, metadata)
    }
    index.write_to(&mut index_lockfile)?;
    index_lockfile.commit().context("commit lockfile")
}
