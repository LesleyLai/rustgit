use clap::Args;
use rustgit::lockfile::Lockfile;
use rustgit::{
    object::{commit_tree, CommitTreeArgs},
    Repository,
};
use std::io::Write;

#[derive(Args, Debug)]
pub struct CommitArgs {
    #[clap(short = 'm')]
    message: String,
}

pub fn commit(args: CommitArgs) -> anyhow::Result<()> {
    let repository = Repository::search_and_open()?;
    let working_dir = std::env::current_dir()?;

    let CommitArgs { message } = args;

    // TODO: check whether we have something to commit (is working tree clean?)

    // git write-tree
    let tree_sha = rustgit::write_utils::write_tree(&repository, &working_dir)?;

    // get current commit
    let parent_commit_sha = repository.head()?;

    let repository_path = repository.repository_directory;

    // git commit-tree
    let commit_sha = commit_tree(CommitTreeArgs {
        parent_commit_sha,
        message,
        tree_sha,
    })?;

    // update-ref for the current branch
    let head_path = repository_path.join(".git").join("HEAD");
    let head_content = {
        let _head_lock = Lockfile::new(&head_path);
        std::fs::read_to_string(head_path)?
    };

    if head_content.starts_with("ref: ") {
        let reference = head_content[5..].trim();
        let reference_path = repository_path.join(".git").join(reference);
        let mut reference_lock = Lockfile::new(&reference_path)?;
        reference_lock.write_all(&commit_sha.to_hex_string().0)?;
        reference_lock.commit()?;
    } else {
        // TODO: detached head
        anyhow::bail!("`rustgit commit` on detached head is not supported");
    }

    Ok(())
}
