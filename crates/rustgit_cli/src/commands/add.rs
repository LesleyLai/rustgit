use clap::Args;
use rustgit::index::{EntryMetadata, Index};
use rustgit::lockfile::Lockfile;
use rustgit::object::{ObjectBuffer, ObjectType};
use rustgit::oid::ObjectId;
use rustgit::write_utils::write_object;
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
    println!("current_dir: {}", current_dir.display());

    println!("path: {}", path.display());

    if path.is_relative() {
        path = current_dir.join(path);
    }
    println!("path2: {}", path.display());

    let path = match path.canonicalize() {
        Err(err) if err.kind() == ErrorKind::NotFound => {
            anyhow::bail!(format!("pathspec '{}' did not match any files", pathspec))
        }
        res => res?,
    };

    println!("path3: {}", path.display());
    println!("repo path: {}", repo_path.display());

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
        mode: metadata.mode(), // TODO
        uid: metadata.uid(),
        gid: metadata.gid(),
        file_size: metadata.size() as u32,
    })
}

pub fn add(args: AddArgs) -> anyhow::Result<()> {
    let current_dir = std::env::current_dir()?;

    let repo = Repository::search_and_open(&current_dir)?;

    let mut index_lockfile = Lockfile::new(&repo.git_directory.join("index"))?;

    let paths = args
        .pathspecs
        .iter()
        .map(|pathspec| parse_pathspec(pathspec, &current_dir, &repo.repository_directory))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let mut file_list: BTreeSet<&Path> = BTreeSet::new();
    for path in &paths {
        if path.is_file() {
            file_list.insert(path);
        }
    }

    let mut index = Index::open(&repo.git_directory.join("index"))?;

    for file_path in file_list {
        let body = fs::read_to_string(file_path)?;
        let blob = ObjectBuffer::new(ObjectType::Blob, body.as_bytes());
        let oid = ObjectId::from_object_buffer(&blob);

        write_object(&repo, &blob, oid)?;

        let metadata = get_metadata(&file_path)?;

        let path = file_path
            .strip_prefix(&repo.repository_directory.canonicalize()?)?
            .to_path_buf();
        index.add(path, oid, metadata)
    }
    index.write_to(&mut index_lockfile)?;
    index_lockfile.commit()
}
