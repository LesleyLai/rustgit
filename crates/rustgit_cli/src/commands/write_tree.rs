use rustgit::Repository;

pub fn write_tree() -> anyhow::Result<()> {
    let working_dir = std::env::current_dir()?;
    let repository = Repository::search_and_open(&working_dir)?;

    let result = repository.write_tree();

    print!("{}", result);

    Ok(())
}
