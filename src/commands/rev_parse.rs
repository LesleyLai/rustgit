use clap::Args;

use rustgit_plumbing::references::get_head_hash;

#[derive(Args, Debug)]
pub struct RevParseArgs {
    #[clap(name = "arg")]
    arg: String,
}

pub fn rev_parse(args: RevParseArgs) -> anyhow::Result<()> {
    //let working_dir = std::env::current_dir()?;

    if args.arg == "HEAD" {
        let maybe_head_hash = get_head_hash()?;
        match maybe_head_hash {
            Some(hash) => println!("{}", hash),
            None => anyhow::bail!("HEAD doesn't exist"),
        }
    } else {
        anyhow::bail!("rev-parse on anything other than HEAD is not currently supported");
    }

    Ok(())
}
