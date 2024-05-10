use clap::Args;
use rustgit::hash::Sha1HashHexString;

use rustgit::Repository;

#[derive(Args, Debug)]
pub struct RevParseArgs {
    #[clap(name = "arg")]
    arg: String,
}

pub fn rev_parse(args: RevParseArgs) -> anyhow::Result<()> {
    let repository = Repository::search_and_open(&std::env::current_dir()?)?;

    let arg = args.arg;
    if arg == "HEAD" {
        if let Some(head_hash) = repository.head()? {
            println!("{}", head_hash)
        } else {
            anyhow::bail!("HEAD doesn't exist")
        }
    } else {
        match Sha1HashHexString::from_str(&arg) {
            Ok(sha1) => println!("{}", sha1),
            Err(_) => {
                anyhow::bail!(
                    "ambiguous argument '{}': unknown revision or path not in the working tree.",
                    arg
                )
            }
        }
    }

    Ok(())
}
