use anyhow::Context;
use clap::Args;

use crate::object::write_object;
use crate::sha1hash::Sha1Hash;

#[derive(Args, Debug)]
pub struct CommitTreeArgs {
    #[clap(short = 'p')]
    parent_commit_sha: Option<String>,

    #[clap(short = 'm')]
    message: String,

    tree_sha: String,
}

pub fn commit_tree(args: CommitTreeArgs) -> anyhow::Result<()> {
    // TODO: validate those sha

    let mut content = String::new();
    content.push_str(&format!("tree {}\n", args.tree_sha));
    if let Some(parent_commit_sha) = &args.parent_commit_sha {
        content.push_str(&format!("parent {parent_commit_sha}\n"));
    }

    // TODO: don't hardcode author names
    content.push_str(&format!(
        "author Lesley Lai <lesley@lesleylai.info> 1243040974 -0700
committer Lesley Lai <lesley@lesleylai.info> 1243040974 -0700

{}
",
        args.message
    ));

    let header = format!("commit {}\0", content.len());
    let data = header + &content;

    let hash = Sha1Hash::from_data(data.as_bytes());
    write_object(data.as_bytes(), &hash).context("failed to write commit object to disk")?;

    println!("{}", hash.to_hex_string());

    Ok(())
}
