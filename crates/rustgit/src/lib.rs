pub mod hash;
pub mod object;
pub mod references;

mod repository;
pub mod utils;

pub use crate::repository::{Repository, RepositorySearchError};

// TODO: should not be public
mod is_executable;
pub mod lockfile;
pub mod write_utils;
