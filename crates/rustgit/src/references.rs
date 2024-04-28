use crate::hash::Sha1Hash;
use std::{io::ErrorKind, path::Path};

pub fn hash_from_reference(
    repository_path: &Path,
    reference: &str,
) -> anyhow::Result<Option<Sha1Hash>> {
    let ref_content_result = std::fs::read_to_string(&repository_path.join(".git").join(reference));
    if let Err(ref err) = ref_content_result {
        if err.kind() == ErrorKind::NotFound {
            return Ok(None);
        }
    }

    Ok(Some(Sha1Hash::from_unvalidated_hex_string(
        &ref_content_result?.trim(),
    )?))
}

pub fn get_head_hash(repository_path: &Path) -> anyhow::Result<Option<Sha1Hash>> {
    let head_content = std::fs::read_to_string(repository_path.join(".git/HEAD"))?;

    if head_content.starts_with("ref: ") {
        hash_from_reference(repository_path, head_content[5..].trim())
    } else {
        // detached head
        let hash = Sha1Hash::from_unvalidated_hex_string(head_content.trim())?;
        Ok(Some(hash))
    }
}
