use clap::Args;
use rustgit::{lockfile::Lockfile, object::Commit, references::ReferenceError, Repository};
use std::io::Write;

#[derive(Args, Debug)]
pub struct CommitArgs {
    #[clap(short = 'm')]
    message: String,
}

pub fn commit(args: CommitArgs) -> anyhow::Result<()> {
    let repository = Repository::search_and_open(&std::env::current_dir()?)?;
    let working_dir = std::env::current_dir()?;

    let CommitArgs { message } = args;

    // TODO: check whether we have something to commit (is working tree clean?)

    // git write-tree
    let tree_sha = rustgit::write_utils::write_tree(&repository, &working_dir)?;

    // get current commit
    let parent_commit_sha = match repository.head_id() {
        Ok(sha) => Ok(Some(sha)),
        Err(ReferenceError::NotExist(_)) => Ok(None),
        Err(e) => Err(e),
    }?;

    // git commit-tree
    let author = rustgit::object::get_author()?;
    let commit_hash =
        repository.write_object(&Commit::new(tree_sha, parent_commit_sha, author, message))?;

    // update-ref for the current branch
    let head_path = repository.git_dir.join("HEAD");
    let _head_lock = Lockfile::new(&head_path)?;

    let head_content = std::fs::read_to_string(head_path)?;

    if head_content.starts_with("ref: ") {
        let reference = head_content[5..].trim();
        let reference_path = repository.git_dir.join(reference);
        let mut reference_lock = Lockfile::new(&reference_path)?;
        reference_lock.write_all(&commit_hash.to_hex_string().0)?;
        reference_lock.commit()?;
    } else {
        // TODO: detached head
        anyhow::bail!("`rustgit commit` on detached head is not supported");
    }

    Ok(())
}
