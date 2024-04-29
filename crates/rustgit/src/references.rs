use crate::hash::Sha1Hash;
use crate::lockfile::Lockfile;
use std::{io::ErrorKind, path::Path};

pub fn hash_from_reference(
    repository_path: &Path,
    reference: &str,
) -> anyhow::Result<Option<Sha1Hash>> {
    let ref_path = repository_path.join(".git").join(reference);

    let ref_content_result = {
        let _lock = Lockfile::new(&ref_path);
        std::fs::read_to_string(&ref_path)
    };
    if let Err(ref err) = ref_content_result {
        if err.kind() == ErrorKind::NotFound {
            return Ok(None);
        }
    }

    Ok(Some(Sha1Hash::from_unvalidated_hex_string(
        &ref_content_result?.trim(),
    )?))
}