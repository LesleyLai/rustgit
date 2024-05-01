mod fs_utils;
pub use fs_utils::touch;

use lazy_static::lazy_static;
use std::{fs, path::PathBuf};

lazy_static! {
    pub static ref TEST_DIR: PathBuf = {
        let temp_dir = std::env::temp_dir();
        let dir = temp_dir.join("rustgit_tests");

        fs::remove_dir_all(&dir).expect("Failed to clear the test directory");
        fs::create_dir(&dir).unwrap();
        dir
    };
}

// Copied from stdext
#[macro_export]
macro_rules! function_name {
    () => {{
        // Okay, this is ugly, I get it. However, this is the best we can get on a stable rust.
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        // `3` is the length of the `::f`.
        &name[..name.len() - 3]
    }};
}

/// Generate a unique temporary working directory for each path
#[macro_export]
macro_rules! test_path {
    () => {{
        use test_utils::{function_name, TEST_DIR};

        let mut function_name: &str = &function_name!()[13..];
        if let Some(name) = function_name.strip_prefix("commands::") {
            function_name = name;
        }

        let directory = function_name.replace("::", "_");
        let path = TEST_DIR.join(directory);
        std::fs::create_dir(&path).unwrap();
        path
    }};
}
