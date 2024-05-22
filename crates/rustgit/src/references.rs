use crate::oid::ObjectId;
//use std::path::PathBuf;
use std::{io::ErrorKind, path::Path};

// // A ref is a variable that holds a single object identifier. The object identifier can be any valid Git object (blob, tree, commit, tag).
// #[derive(Debug, PartialEq, Eq)]
// pub enum Ref {
//     OId {
//         oid: Sha1Hash,
//     },
//     SymRef {
//         // The path relative to .git/refs
//         path: PathBuf,
//     },
//     HEAD,
// }

pub fn hash_from_reference(git_path: &Path, reference: &str) -> anyhow::Result<Option<ObjectId>> {
    let ref_path = git_path.join(reference);

    let ref_content_result = std::fs::read_to_string(&ref_path);
    if let Err(ref err) = ref_content_result {
        if err.kind() == ErrorKind::NotFound {
            return Ok(None);
        }
    }

    Ok(Some(ObjectId::from_unvalidated_hex_string(
        &ref_content_result?.trim(),
    )?))
}
