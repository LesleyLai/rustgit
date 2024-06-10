use clap::Args;
use rustgit::{object::read_tree_object, oid::ObjectId, Repository};
use std::io::prelude::*;

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

    let tree_hash = ObjectId::from_unvalidated_sh1_hex_string(&args.tree_ish)?;
    let mut reader = repository.object_reader(tree_hash)?;
    let tree = read_tree_object(&mut reader)?;

    for entry in tree.iter() {
        if args.name_only {
            println!("{}", entry.name);
        } else {
            let mut reader = repository.object_reader(entry.oid)?;

            let mut buffer = vec![];
            reader.read_until(b' ', &mut buffer)?;

            let typ = std::str::from_utf8(&buffer[0..buffer.len() - 1])?;
            assert!(typ == "blob" || typ == "tree");
            println!("{:06o} {} {}    {}", entry.mode, typ, entry.oid, entry.name)
        }
    }

    Ok(())
}
