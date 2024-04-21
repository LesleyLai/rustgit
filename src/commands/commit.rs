use crate::repository::Repository;
use clap::Args;
use rustgit_plumbing::object::{commit_tree, CommitTreeArgs};

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
    let tree_sha = crate::write_utils::write_tree(&repository, &working_dir)?;

    // git commit-tree
    let _commit_sha = commit_tree(CommitTreeArgs {
        parent_commit_sha: None,
        message,
        tree_sha,
    })?;

    // TODO: git update-ref for the current branch

    Ok(())
}
