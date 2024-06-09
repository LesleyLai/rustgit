use anyhow::Context;
use clap::Args;
use rustgit::{
    object::{read_header, ObjectHeader, ObjectType},
    oid::ObjectId,
    Repository,
};
use std::io::{prelude::*, Write};

#[derive(Args, Debug)]
pub struct CatFileArgs {
    /// Pretty-print the contents of \<object\> based on its type
    #[clap(short = 'p')]
    pretty_print: bool,
    object_hash: String,
}

pub fn cat_file(args: CatFileArgs) -> anyhow::Result<()> {
    assert!(args.pretty_print, "Only works with -p now");

    let repository = Repository::search_and_open(&std::env::current_dir()?)?;

    // TODO: support shortest unique object hashes
    let object_hash = ObjectId::from_unvalidated_sh1_hex_string(&args.object_hash)?;
    let mut decoder = repository.object_reader(object_hash)?;

    let ObjectHeader { typ, size } = read_header(&mut decoder)?;
    // TODO: support tree and commits
    if typ != ObjectType::Blob {
        unimplemented!("cat-file for non-blob is not implemented yet");
    }

    let mut output = vec![];
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
