use rustgit::references::hash_from_reference;
use rustgit::Repository;
use std::env::current_dir;
use std::fs;

pub fn status() -> anyhow::Result<()> {
    let repository = Repository::search_and_open(&current_dir()?)
        .map_err(|_| anyhow::anyhow!("not a git repository (or any of the parent directories)"))?;

    let head_path = repository.git_directory.join("HEAD");

    let head_content = fs::read_to_string(head_path)?;

    let head_ref = if head_content.starts_with("ref: ") {
        head_content[5..].trim()
    } else {
        unimplemented!();
    };
    let branch = if head_ref.starts_with("refs/heads/") {
        &head_ref[11..]
    } else {
        unimplemented!();
    };

    println!("On branch {}", branch);

    if let None = hash_from_reference(&repository.git_directory, &head_ref)? {
        println!("\nNo commits yet\n");
    }

    println!("nothing to commit (create/copy files and use \"rustgit add\" to track)");

    Ok(())
}
