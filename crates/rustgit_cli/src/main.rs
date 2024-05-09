mod commands;
mod parse_util;

use crate::commands::*;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Create an empty Git repository
    Init,

    /// Provide contents or details of repository objects
    CatFile(CatFileArgs),

    /// Compute object ID and optionally create an object from a file
    HashObject(HashObjectArgs),

    /// List the contents of a tree object
    LsTree(LsTreeArgs),

    /// Show information about files in the index and the working tree
    LsFiles,

    /// Create a tree object from the current index
    WriteTree,

    /// Create a new commit object
    CommitTree(CommitTreeArgs),

    /// Record changes to the repository
    Commit(CommitArgs),

    /// Print the SHA1 hashes given a revision specifier
    RevParse(RevParseArgs),

    /// Show the working tree status
    Status,
}

fn main() {
    let args = Cli::parse();

    use Command::*;
    let result = match args.command {
        Init => init().map_err(anyhow::Error::from),
        CatFile(args) => cat_file(args),
        HashObject(args) => hash_object(args),
        LsFiles => ls_files(),
        LsTree(args) => ls_tree(args),
        WriteTree => write_tree(),
        CommitTree(args) => commit_tree(args),
        Commit(args) => commit(args),
        RevParse(args) => rev_parse(args),
        Status => status(),
    };
    if let Err(e) = result {
        eprintln!("fatal: {}", e);
        std::process::exit(1);
    }
}
