use crate::git_object_util::write_object;
use crate::sha1hash::Sha1Hash;
use clap::Args;
use std::fs;

#[derive(Args, Debug)]
pub struct HashObjectArgs {
    /// Actually write the object into the object database.
    #[clap(short = 'w')]
    perform_write: bool,

    filename: String,
}

pub fn hash_object(args: HashObjectArgs) -> anyhow::Result<()> {
    let body = fs::read_to_string(args.filename)?;
    let header = format!("blob {}\0", body.len());
    let blob_content = header + &body;
    let object_hash = Sha1Hash::from_contents(blob_content.as_bytes());
    println!("{}", object_hash.to_hex_string());

    if args.perform_write {
        write_object(blob_content.as_bytes(), &object_hash)?;
    }

    Ok(())
}
