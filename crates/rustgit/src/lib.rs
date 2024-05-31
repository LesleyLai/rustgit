pub mod object;
pub mod oid;
pub mod references;

mod repository;
pub mod utils;

pub use crate::repository::{Repository, RepositorySearchError};

mod head;

// TODO: should not be public
mod database;
pub mod index;
mod is_executable;
pub mod lockfile;
pub mod read_ext;
pub mod write_utils;
