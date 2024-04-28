use clap::Args;
use rustgit::hash::Sha1Hash;

#[derive(Args, Debug)]
pub struct CommitTreeArgs {
    #[clap(short = 'p')]
    parent_commit_sha: Option<String>,

    #[clap(short = 'm')]
    message: String,

    tree_sha: String,
}

pub fn commit_tree(args: CommitTreeArgs) -> anyhow::Result<()> {
    let tree_sha = Sha1Hash::from_unvalidated_hex_string(&args.tree_sha)?;

    let parent_commit_sha = if let Some(sha) = &args.parent_commit_sha {
        Some(Sha1Hash::from_unvalidated_hex_string(sha)?)
    } else {
        None
    };

    let commit_hash = rustgit::object::commit_tree(rustgit::object::CommitTreeArgs {
        parent_commit_sha,
        message: args.message,
        tree_sha,
    })?;

    println!("{}", commit_hash);

    Ok(())
}
