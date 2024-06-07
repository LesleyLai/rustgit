use anyhow::Context;
use clap::Args;
use flate2::read::ZlibDecoder;
use rustgit::{
    object::{read_header, ObjectHeader, ObjectType, Tree, TreeEntry},
    oid::ObjectId,
    utils::remove_last,
    Repository,
};
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

    let tree_hash = ObjectId::from_unvalidated_sh1_hex_string(&args.tree_ish)?;
    let tree_object_path = repository.object_path_from_obj(tree_hash);

    let file = File::open(&tree_object_path)?;

    let mut decoder = BufReader::new(ZlibDecoder::new(&file));

    let ObjectHeader { typ, .. } = read_header(&mut decoder)?;
    if typ != ObjectType::Tree {
        anyhow::bail!("not a tree object");
    }

    let mut output = vec![];

    let mut tree = Tree::new();
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

        let (mode, name) = output_str
            .split_once(' ')
            .context("corrupted file for tree object")?;

        let mode = u32::from_str_radix(mode, 8)?;

        // hash
        let mut oid_buffer = [0u8; 20];
        decoder
            .read_exact(&mut oid_buffer)
            .context("failed to read item hash from tree object")?;
        let oid = ObjectId(oid_buffer);

        tree.add_entry(TreeEntry {
            name: name.to_string(),
            oid,
            mode,
        })
    }

    for entry in tree.iter() {
        if args.name_only {
            println!("{}", entry.name);
        } else {
            let object_path = repository.object_path_from_obj(entry.oid);
            let file = File::open(object_path)?;
            let mut decoder = BufReader::new(ZlibDecoder::new(&file));

            let mut buffer = vec![];
            decoder.read_until(b' ', &mut buffer)?;

            let typ = std::str::from_utf8(&buffer[0..buffer.len() - 1])?;
            assert!(typ == "blob" || typ == "tree");
            println!("{:06o} {} {}    {}", entry.mode, typ, entry.oid, entry.name)
        }
    }

    Ok(())
}
