use crate::hash::Sha1Hash;
use std::env::current_dir;
use std::io::ErrorKind;

pub fn get_head_hash() -> anyhow::Result<Option<Sha1Hash>> {
    // TODO: find repository path
    let head_content = std::fs::read_to_string(".git/HEAD")?;

    let hash = if head_content.starts_with("ref: ") {
        let ref_path = current_dir()?
            .join(".git")
            .join(&head_content[5..].trim_end());

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
