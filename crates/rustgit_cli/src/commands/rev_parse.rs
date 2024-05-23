use clap::Args;
use rustgit::oid::Sha1HashHexString;

use rustgit::Repository;

#[derive(Args, Debug)]
pub struct RevParseArgs {
    #[clap(name = "arg")]
    arg: String,
}

fn rev_parse_impl(repo: &Repository, arg: &str) -> anyhow::Result<()> {
    if arg == "HEAD" {
        let head_hash = repo.head_id()?;
        println!("{}", head_hash);
    } else {
        let sha1 = Sha1HashHexString::from_str(arg)?;
        println!("{}", sha1);
    }
    Ok(())
}

pub fn rev_parse(args: RevParseArgs) -> anyhow::Result<()> {
    let repository = Repository::search_and_open(&std::env::current_dir()?)?;
    let arg = args.arg;

    if rev_parse_impl(&repository, &arg).is_err() {
        anyhow::bail!(
            "ambiguous argument '{}': unknown revision or path not in the working tree.",
            arg
        )
    }

    Ok(())
}
