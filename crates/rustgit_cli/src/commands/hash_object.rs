use clap::Args;
use rustgit::{
    object::{ObjectBuffer, ObjectType},
    oid::ObjectId,
    Repository,
};
use std::fs;

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
        std::io::read_to_string(std::io::stdin())?
    } else {
        fs::read_to_string(args.group.filename.unwrap())?
    };

    let blob = ObjectBuffer::new(ObjectType::Blob, body.as_bytes());
    let oid = ObjectId::from_object_buffer(&blob);
    println!("{}", oid.to_hex_string());

    if args.perform_write {
        let repository = Repository::search_and_open(&std::env::current_dir()?)?;
        repository.write_object_buffer(oid, &blob)?;
    }

    Ok(())
}
