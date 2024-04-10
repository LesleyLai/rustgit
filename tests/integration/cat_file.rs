use crate::common::{clear_dir, git_command, TEST_DIR};
#[test]
fn cat_file() -> anyhow::Result<()> {
    let working_dir = TEST_DIR.join("cat-file");
    clear_dir(&working_dir)?;

    git_command()
        .args(["init"])
        .current_dir(&working_dir)
        .output()?;

    let file_path = working_dir.join("file.txt");
    std::fs::write(&file_path, "hello world")?;

    Ok(())
}
