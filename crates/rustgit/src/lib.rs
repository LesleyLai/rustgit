pub mod hash;
pub mod object;
pub mod references;
mod repository;
pub mod utils;

pub use crate::repository::{Repository, RepositoryInitError};

// TODO: should not be public
pub mod lockfile;
pub mod write_utils;
