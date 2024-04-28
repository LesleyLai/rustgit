use clap::Args;
use rustgit::{
    object::{commit_tree, CommitTreeArgs},
    Repository,
};

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

    let repository_path = repository.repository_directory;

    // get current commit
    let parent_commit_sha = rustgit::references::get_head_hash(&repository_path)?;

    // git commit-tree
    let commit_sha = commit_tree(CommitTreeArgs {
        parent_commit_sha,
        message,
        tree_sha,
    })?;

    // TODO: lock
    // update-ref for the current branch
    let head_content = std::fs::read_to_string(repository_path.join(".git").join("HEAD"))?;

    if head_content.starts_with("ref: ") {
        let reference = head_content[5..].trim();
        std::fs::write(
            repository_path.join(".git").join(reference),
            &commit_sha.to_hex_string().0,
        )?;
    } else {
        // TODO: detached head
        anyhow::bail!("`rustgit commit` on detached head is not supported");
    }

    Ok(())
}
