use clap::Args;
use rustgit::{index::Index, Repository};

#[derive(Args, Debug)]
pub struct LsFilesArgs {
    /// Show staged contents' mode bits, object name and stage number in the output.
    #[clap(short = 's')]
    stage: bool,
}

pub fn ls_files(args: LsFilesArgs) -> anyhow::Result<()> {
    let repository = Repository::search_and_open(&std::env::current_dir()?)?;

    let index = Index::open(&repository.git_directory.join("index"))?;
    for entry in index.iter() {
        if args.stage {
            println!(
                "{:0>6o} {} 0\t{}",
                entry.metadata.mode,
                entry.oid,
                entry.path.display()
            );
        } else {
            println!("{}", entry.path.display());
        }
    }

    Ok(())
}
