use anyhow::Context;
use clap::Args;
use flate2::read::ZlibDecoder;
use rustgit_plumbing::hash::Sha1Hash;
use rustgit_plumbing::object::object_path_from_hash;
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
    tree_ish: String,
}

pub fn ls_tree(args: LsTreeArgs) -> anyhow::Result<()> {
    anyhow::ensure!(args.name_only, "Only implemented name_only for now");

    let path = object_path_from_hash(
        &Sha1Hash::from_unvalidated_hex_string(&args.tree_ish)?.to_hex_string(),
    );

    let file = File::open(&path)?;

    let decoder = ZlibDecoder::new(&file);
    let mut decoder = BufReader::new(decoder);

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
        let output_str = std::str::from_utf8(&output[..output.len() - 1]).unwrap();
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
