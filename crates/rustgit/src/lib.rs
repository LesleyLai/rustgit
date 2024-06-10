pub mod oid;
pub mod references;

pub mod object;

pub mod utils;

mod repository;

pub use crate::repository::Repository;

pub mod head;

// TODO: should not be public
mod database;
pub mod index;
mod is_executable;
pub mod lockfile;
mod object_reader;
mod parse_utils;
mod read_ext;
pub mod write_utils;
