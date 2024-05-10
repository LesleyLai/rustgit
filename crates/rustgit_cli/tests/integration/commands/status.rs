use crate::common::rustgit;
use test_utils::test_path;

#[test]
fn status() -> anyhow::Result<()> {
    let working_dir = test_path!();
    let rustgit = || rustgit(&working_dir);

    rustgit().init();

    insta::assert_snapshot!("init", rustgit().status());

    Ok(())
}
