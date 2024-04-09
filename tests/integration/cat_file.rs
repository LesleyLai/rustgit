use crate::common::{clear_dir, git_command, TEST_DIR};
#[test]
fn cat_file() -> anyhow::Result<()> {
    let working_dir = TEST_DIR.join("cat-file");
    clear_dir(&working_dir)?;

    git_command()
        .args(["init"])
        .current_dir(&working_dir)
        .output()?;

    // TODO: continue working on this after refactoring

    Ok(())
}
