mod commands;

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

    /// Add file contents to the index
    Add(AddArgs),

    /// Provide contents or details of repository objects
    CatFile(CatFileArgs),

    /// Compute object ID and optionally create an object from a file
    HashObject(HashObjectArgs),

    /// List the contents of a tree object
    LsTree(LsTreeArgs),

    /// Show information about files in the index and the working tree
    LsFiles(LsFilesArgs),

    /// Create a tree object from the current index
    WriteTree,

    /// Create a new commit object
    CommitTree(CommitTreeArgs),

    /// Record changes to the repository
    Commit(CommitArgs),

    /// Print the SHA1 hashes given a revision specifier
    RevParse(RevParseArgs),

    /// Add file contents to the staging area
    Stage(AddArgs),

    /// Show the working tree status
    Status,
}

fn main() {
    let args = Cli::parse();

    use Command::*;
    let result = match args.command {
        Init => init().map_err(anyhow::Error::from),
        Add(args) => add(args),
        CatFile(args) => cat_file(args),
        HashObject(args) => hash_object(args),
        LsFiles(args) => ls_files(args),
        LsTree(args) => ls_tree(args),
        WriteTree => write_tree(),
        CommitTree(args) => commit_tree(args),
        Commit(args) => commit(args),
        RevParse(args) => rev_parse(args),
        Status => status(),
        Stage(args) => add(args),
    };
    if let Err(e) = result {
        eprintln!("fatal: {}", e);
        std::process::exit(128);
    }
}
