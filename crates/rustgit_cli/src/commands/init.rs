use std::fs;

pub fn init() -> std::io::Result<()> {
    fs::create_dir(".git")?;
    fs::create_dir(".git/objects")?;
    fs::create_dir(".git/refs")?;
    fs::write(".git/HEAD", "ref: refs/heads/main\n")?;
    println!(
        "Initialized empty Git repository in {}",
        std::env::current_dir()?.join(".git").display()
    );
    Ok(())
}
