use clap::Args;
use rustgit_plumbing::hash::Sha1HashHexString;

use rustgit_plumbing::references::get_head_hash;
use rustgit_plumbing::repository::Repository;

#[derive(Args, Debug)]
pub struct RevParseArgs {
    #[clap(name = "arg")]
    arg: String,
}

pub fn rev_parse(args: RevParseArgs) -> anyhow::Result<()> {
    let repository = Repository::new()?;

    let arg = args.arg;
    if arg == "HEAD" {
        if let Some(head_hash) = get_head_hash(&repository.repository_dir)? {
            println!("{}", head_hash)
        } else {
            anyhow::bail!("HEAD doesn't exist")
        }
    } else {
        match Sha1HashHexString::from_str(&arg) {
            Ok(sha1) => println!("{}", sha1),
            Err(_) => {
                anyhow::bail!("Needed a single revision, get: {}", arg)
            }
        }
    }

    Ok(())
}
