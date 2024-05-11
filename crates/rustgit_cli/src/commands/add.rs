use clap::Args;
use rustgit::hash::Sha1Hash;
use rustgit::lockfile::Lockfile;
use rustgit::object::{ObjectBuffer, ObjectType};
use rustgit::write_utils::write_object;
use rustgit::Repository;
use std::collections::HashSet;
use std::fs;
use std::io::{ErrorKind, Write};
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
        path.starts_with(repo_path),
        "'{}' is outside repository",
        pathspec
    );
    Ok(path)
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

    let mut file_list: HashSet<&Path> = HashSet::new();
    for path in &paths {
        if path.is_file() {
            file_list.insert(path);
        }
    }

    let entry_count = u32::try_from(file_list.len())?;

    index_lockfile.write(b"DIRC")?;
    index_lockfile.write(&u32::to_be_bytes(2))?;
    index_lockfile.write(&u32::to_be_bytes(entry_count))?;

    for file_path in file_list {
        let body = fs::read_to_string(file_path)?;
        let blob = ObjectBuffer::new(ObjectType::Blob, body.as_bytes());
        let object_hash = Sha1Hash::from_object(&blob);

        write_object(&repo, &blob, object_hash)?;

        // TODO: proper meta data
        let metadata_bytes = [0u8; 24];
        index_lockfile.write(&metadata_bytes)?;

        // file mode
        index_lockfile.write(&u32::to_be_bytes(0o100644))?;

        let metadata_bytes = [0u8; 12];
        index_lockfile.write(&metadata_bytes)?;

        // hash
        index_lockfile.write(&object_hash.0)?;

        let relative_path = file_path.strip_prefix(&repo.repository_directory)?;
        let relative_path_bytes = relative_path.to_str().unwrap().as_bytes();
        let relative_path_len = relative_path_bytes.len();

        // path size
        index_lockfile.write(&u16::to_be_bytes(u16::try_from(relative_path_bytes.len())?))?;

        let total_size = 62 + relative_path_len;
        let padded_size = (total_size / 8 + 1) * 8;

        index_lockfile.write(relative_path_bytes)?;

        // write paddings
        for _ in 0..(padded_size - total_size) {
            index_lockfile.write(&[0])?;
        }

        println!("{}", file_path.display());
    }

    index_lockfile.commit()
}
