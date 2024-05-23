use crate::object::ObjectBuffer;
use crate::utils::trim_whitespace;
use anyhow::Context;
use sha1::Digest;
use std::fmt::{Debug, Display, Formatter};

/// 20-bytes raw hash
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ObjectId(pub [u8; 20]);

// 40-char Hex string
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sha1HashHexString(pub [u8; 40]);

// Byte to 2-char hex string representation
#[inline]
#[must_use]
fn byte2hex(byte: u8) -> (u8, u8) {
    const TABLE: &[u8; 16] = b"0123456789abcdef";
    let high = TABLE[((byte & 0xf0) >> 4) as usize];
    let low = TABLE[(byte & 0x0f) as usize];

    (high, low)
}

#[derive(thiserror::Error, Debug)]
pub enum SHA1ValidationError {
    #[error("InvalidHexString: {0}")]
    InvalidHexString(String),
    #[error("An sha1 hash should be 20 bytes long. Get: {0}")]
    SHA1Error(String),
}

impl ObjectId {
    /// Compute a hash from a git object
    pub fn from_object_buffer(object: &ObjectBuffer) -> Self {
        Self::from_data(&object.data())
    }

    pub fn from_data(data: &[u8]) -> Self {
        let mut hasher = sha1::Sha1::new();
        hasher.update(data);
        let output = hasher.finalize();

        ObjectId(
            output
                .try_into()
                .expect("Sha1 hash should be 20 bytes long"),
        )
    }

    pub fn from_unvalidated_sh1_hex_string(s: &str) -> Result<Self, SHA1ValidationError> {
        let data =
            hex::decode(s).map_err(|_| SHA1ValidationError::InvalidHexString(s.to_string()))?;

        Ok(ObjectId(data.as_slice().try_into().map_err(|_| {
            SHA1ValidationError::SHA1Error(s.to_string())
        })?))
    }

    pub fn to_hex_string(&self) -> Sha1HashHexString {
        let mut output = [0; 40];
        for (i, &c) in self.0.iter().enumerate() {
            let (high, low) = byte2hex(c);
            output[2 * i] = high;
            output[2 * i + 1] = low;
        }

        Sha1HashHexString(output)
    }
}

impl Debug for ObjectId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex_string())
    }
}

impl Display for ObjectId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex_string())
    }
}

impl Sha1HashHexString {
    pub fn from_str(s: &str) -> anyhow::Result<Self> {
        Self::from_u8_slice(s.as_bytes())
    }
    pub fn from_u8_slice(bytes: &[u8]) -> anyhow::Result<Self> {
        let data: [u8; 40] = trim_whitespace(bytes).try_into().with_context(|| {
            format!(
                "Byte slice is not a valid sha1 hash. It has a length of {}\nContent: {}",
                bytes.len(),
                String::from_utf8_lossy(bytes)
            )
        })?;
        // TODO: validate the result
        Ok(Sha1HashHexString(data))
    }
}

impl Display for Sha1HashHexString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

impl Debug for Sha1HashHexString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::ops::Deref for Sha1HashHexString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::{ObjectBuffer, ObjectType};

    #[test]
    fn from_unvalidated_hex_string() {
        assert!(ObjectId::from_unvalidated_sh1_hex_string("asdfs").is_err());
        assert!(ObjectId::from_unvalidated_sh1_hex_string("0f46").is_err());

        const HASH: &str = "0f46983e0baf73ba9bf82a7317223d2eebc728d8";
        assert_eq!(
            &ObjectId::from_unvalidated_sh1_hex_string(HASH)
                .unwrap()
                .to_string(),
            HASH
        );
    }

    #[test]
    fn from_object_buffer() {
        let blob = ObjectBuffer::new(ObjectType::Blob, "hello world\n".as_bytes());

        assert_eq!(
            &ObjectId::from_object_buffer(&blob).to_string(),
            "3b18e512dba79e4c8300dd08aeb37f8e728b8dad"
        )
    }
}
