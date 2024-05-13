use clap::Args;
use rustgit::hash::Sha1Hash;
use rustgit::index::Index;
use rustgit::lockfile::Lockfile;
use rustgit::object::{ObjectBuffer, ObjectType};
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
        let oid = Sha1Hash::from_object(&blob);

        write_object(&repo, &blob, oid)?;

        index.add(
            file_path
                .strip_prefix(&repo.repository_directory)?
                .to_path_buf(),
            oid,
        )
    }
    index.write_to(&mut index_lockfile)?;
    index_lockfile.commit()
}
