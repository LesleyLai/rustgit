use rustgit::Repository;

pub fn init() -> std::io::Result<()> {
    let current_dir = &std::env::current_dir()?;
    let repo = Repository::init(&current_dir)?;
    println!(
        "Initialized empty Git repository in {}",
        repo.git_dir.display()
    );
    Ok(())
}
