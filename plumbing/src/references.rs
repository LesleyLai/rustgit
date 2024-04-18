use crate::hash::Sha1Hash;
use std::{io::ErrorKind, path::Path};

pub fn get_head_hash(repository_path: &Path) -> anyhow::Result<Option<Sha1Hash>> {
    let head_content = std::fs::read_to_string(repository_path.join(".git/HEAD"))?;

    let hash = if head_content.starts_with("ref: ") {
        let ref_path = repository_path.join(".git").join(&head_content[5..].trim());

        let ref_content_result = std::fs::read_to_string(&ref_path);
        if let Err(ref err) = ref_content_result {
            if err.kind() == ErrorKind::NotFound {
                return Ok(None);
            }
        }

        Sha1Hash::from_unvalidated_hex_string(&ref_content_result?.trim())?
    } else {
        // detached head
        Sha1Hash::from_unvalidated_hex_string(&head_content.trim())?
    };

    Ok(Some(hash))
}
