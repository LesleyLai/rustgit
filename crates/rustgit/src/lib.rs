pub mod oid;
pub mod references;

pub mod object;

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
