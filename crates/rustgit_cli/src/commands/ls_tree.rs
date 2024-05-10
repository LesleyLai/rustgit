use anyhow::Context;
use clap::Args;
use flate2::read::ZlibDecoder;
use rustgit::{hash::Sha1Hash, utils::remove_last, Repository};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

#[derive(Args, Debug)]
pub struct LsTreeArgs {
    /// List only filenames (instead of the "long" output), one per line.
    #[clap(long = "name-only")]
    name_only: bool,

    /// Id of a tree-ish
    #[clap(name = "tree-ish")]
    tree_ish: String,
}

pub fn ls_tree(args: LsTreeArgs) -> anyhow::Result<()> {
    let repository = Repository::search_and_open(&std::env::current_dir()?)?;

    anyhow::ensure!(args.name_only, "Only implemented name_only for now");

    let tree_hash = Sha1Hash::from_unvalidated_hex_string(&args.tree_ish)?;
    let tree_object_path = repository.object_path_from_hash(tree_hash);

    let file = File::open(&tree_object_path)?;

    let mut decoder = BufReader::new(ZlibDecoder::new(&file));

    // header
    let mut output = vec![];
    decoder
        .read_until(0, &mut output)
        .context("failed to read header from tree object")?;

    loop {
        output.clear();
        let n = decoder
            .read_until(0, &mut output)
            .context("failed to read item from tree object")?;
        // EOF
        if n == 0 {
            break;
        }
        let output_str = std::str::from_utf8(remove_last(&output)).unwrap();
        if output_str.is_empty() {
            break;
        }

        let (_permission, name) = output_str
            .split_once(' ')
            .context("corrupted file for tree object")?;

        println!("{}", name);

        // hash
        let mut hash_buffer = [0u8; 20];
        decoder
            .read_exact(&mut hash_buffer)
            .context("failed to read item hash from tree object")?;
    }

    Ok(())
}
