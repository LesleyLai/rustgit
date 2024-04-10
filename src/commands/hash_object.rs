use anyhow::Context;
use clap::Args;
use rustgit_plumbing::hash::Sha1Hash;
use rustgit_plumbing::object::{write_object, Object, ObjectType};
use std::fs;
use std::io::Read;

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct HashObjectArgsGroup {
    #[clap(long)]
    stdin: bool,
    filename: Option<String>,
}

#[derive(Args, Debug)]
pub struct HashObjectArgs {
    /// Actually write the object into the object database.
    #[clap(short = 'w')]
    perform_write: bool,

    #[clap(flatten)]
    group: HashObjectArgsGroup,
}

pub fn hash_object(args: HashObjectArgs) -> anyhow::Result<()> {
    let body = if args.group.stdin {
        let mut input = String::new();
        std::io::stdin()
            .read_to_string(&mut input)
            .context("Failed to read from stdin")?;
        input
    } else {
        fs::read_to_string(args.group.filename.unwrap())?
    };

    let blob = Object::new(ObjectType::Blob, body.as_bytes());
    let object_hash = Sha1Hash::from_data(&blob.data);
    println!("{}", object_hash.to_hex_string());

    if args.perform_write {
        write_object(&blob.data, &object_hash)?;
    }

    Ok(())
}
