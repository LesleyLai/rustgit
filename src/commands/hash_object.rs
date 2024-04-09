use crate::object::{write_object, Object, ObjectType};
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
    let blob = Object::new(ObjectType::Blob, body.as_bytes());
    let object_hash = Sha1Hash::from_data(&blob.data);
    println!("{}", object_hash.to_hex_string());

    if args.perform_write {
        write_object(&blob.data, &object_hash)?;
    }

    Ok(())
}
