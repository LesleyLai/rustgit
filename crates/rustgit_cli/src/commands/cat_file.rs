use crate::parse_util::parse_usize;
use anyhow::Context;
use clap::Args;
use flate2::read::ZlibDecoder;
use rustgit::{hash::Sha1Hash, utils::remove_last, Repository};
use std::{
    fs::File,
    io::{prelude::*, BufReader, Write},
};

#[derive(Args, Debug)]
pub struct CatFileArgs {
    /// Pretty-print the contents of <object> based on its type
    #[clap(short = 'p')]
    pretty_print: bool,
    object_hash: String,
}

pub fn cat_file(args: CatFileArgs) -> anyhow::Result<()> {
    assert!(args.pretty_print, "Only works with -p now");

    let repository = Repository::search_and_open(&std::env::current_dir()?)?;

    // TODO: support shortest unique object hashes
    let object_hash = Sha1Hash::from_unvalidated_hex_string(&args.object_hash)?;
    let path = repository.object_path_from_hash(object_hash);

    let file = File::open(&path)?;
    let mut decoder = BufReader::new(ZlibDecoder::new(&file));

    let mut output = vec![];
    decoder
        .read_until(0, &mut output)
        .context("read header from .git/objects")?;

    let separate_point = output
        .iter()
        .position(|&c| c == b' ')
        .context("header has space separator")?;
    let (typ, mut size) = remove_last(&output).split_at(separate_point);
    // TODO: support tree and commits
    if typ != b"blob" {
        anyhow::bail!(".git/object file header does not start with a known type");
    }

    size = &size[1..];

    let size: usize = parse_usize(size).unwrap();

    output.clear();
    output.resize(size, 0);
    decoder.read_exact(&mut output)?;

    let n = decoder
        .read(&mut [0])
        .context("validate EOF in .git/object file")?;
    anyhow::ensure!(
        n == 0,
        "size of .git/object file is larger than its declared size, with {n} trailing bytes"
    );

    let mut stdout = std::io::stdout().lock();
    stdout.write_all(&output)?;

    Ok(())
}
