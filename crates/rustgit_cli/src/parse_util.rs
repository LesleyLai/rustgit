use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseU64Error {
    #[error("invalid digit")]
    InvalidDigit { got: u8 },

    #[error("number is too big to parse into a u64")]
    NumberTooBig,
}

pub fn parse_usize(bytes: &[u8]) -> Result<usize, ParseU64Error> {
    let mut n: usize = 0;
    for &byte in bytes {
        let digit = match byte.checked_sub(b'0') {
            None => return Err(ParseU64Error::InvalidDigit { got: byte }),
            Some(digit) if digit > 9 => return Err(ParseU64Error::InvalidDigit { got: byte }),
            Some(digit) => {
                debug_assert!((0..=9).contains(&digit));
                usize::from(digit)
            }
        };
        n = n
            .checked_mul(10)
            .and_then(|n| n.checked_add(digit))
            .ok_or_else(|| ParseU64Error::NumberTooBig)?;
    }
    Ok(n)
}
