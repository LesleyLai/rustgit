use clap::Args;

use rustgit_plumbing::references::get_head_hash;
use rustgit_plumbing::repository::Repository;

#[derive(Args, Debug)]
pub struct RevParseArgs {
    #[clap(name = "arg")]
    arg: String,
}

pub fn rev_parse(args: RevParseArgs) -> anyhow::Result<()> {
    let repository = Repository::new()?;

    if args.arg == "HEAD" {
        if let Some(head_hash) = get_head_hash(&repository.repository_dir)? {
            println!("{}", head_hash)
        } else {
            anyhow::bail!("HEAD doesn't exist")
        }
    } else {
        anyhow::bail!("rev-parse on anything other than HEAD is not currently supported");
    }

    Ok(())
}
