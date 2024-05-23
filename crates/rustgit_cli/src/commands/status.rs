use rustgit::Repository;
use std::env::current_dir;

fn print_nothing_to_commit(has_no_commit_yet: bool) {
    print!("nothing to commit");
    if has_no_commit_yet {
        println!(" (create/copy files and use \"rustgit add\" to track)");
    } else {
        println!(", working tree clean");
    }
}

pub fn status() -> anyhow::Result<()> {
    let repository = Repository::search_and_open(&current_dir()?)
        .map_err(|_| anyhow::anyhow!("not a git repository (or any of the parent directories)"))?;

    let head = repository.head()?;

    let head_ref_name = head
        .referent_name()
        .expect("git status for detached head is not implemented");

    let branch = if head_ref_name.starts_with("refs/heads/") {
        &head_ref_name[11..]
    } else {
        unimplemented!("Head reference has a bad format: {}", head_ref_name);
    };

    println!("On branch {}", branch);

    let has_no_commit_yet = head.is_unborn();
    if has_no_commit_yet {
        println!("\nNo commits yet\n");
    }

    print_nothing_to_commit(has_no_commit_yet);

    Ok(())
}
