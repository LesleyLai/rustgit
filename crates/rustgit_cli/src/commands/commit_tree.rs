use clap::Args;
use rustgit::{object::Commit, oid::ObjectId, Repository};

#[derive(Args, Debug)]
pub struct CommitTreeArgs {
    #[clap(short = 'p')]
    parent_commit_sha: Option<String>,

    #[clap(short = 'm')]
    message: String,

    tree_sha: String,
}

pub fn commit_tree(args: CommitTreeArgs) -> anyhow::Result<()> {
    let tree_sha = ObjectId::from_unvalidated_sh1_hex_string(&args.tree_sha)?;

    let parent_commit_sha = args
        .parent_commit_sha
        .map(|sha| ObjectId::from_unvalidated_sh1_hex_string(&sha))
        .transpose()?;

    let repository = Repository::search_and_open(&std::env::current_dir()?)?;
    let author = rustgit::object::get_author();
    let commit_hash = repository.write_object(&Commit::new(
        tree_sha,
        parent_commit_sha,
        author,
        args.message,
    ))?;

    println!("{}", commit_hash);

    Ok(())
}
