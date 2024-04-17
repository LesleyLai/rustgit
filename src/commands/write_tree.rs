pub fn write_tree() -> anyhow::Result<()> {
    let working_dir = std::env::current_dir()?;
    let result = rustgit_plumbing::object::write_tree(&working_dir)?;

    print!("{}", result);

    Ok(())
}
