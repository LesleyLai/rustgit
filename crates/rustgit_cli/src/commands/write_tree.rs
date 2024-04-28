use rustgit::Repository;

pub fn write_tree() -> anyhow::Result<()> {
    let repository = Repository::search_and_open()?;
    let working_dir = std::env::current_dir()?;
    let result = rustgit::write_utils::write_tree(&repository, &working_dir)?;

    print!("{}", result);

    Ok(())
}
