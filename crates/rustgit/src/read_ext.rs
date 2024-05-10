// Extension methods to std::io::Read

use std::io::{Read, Result};

pub trait ReadExt {
    fn read_exact_n<const N: usize>(&mut self) -> Result<[u8; N]>;

    fn read_exact_4(&mut self) -> Result<[u8; 4]> {
        self.read_exact_n::<4>()
    }
}

impl<R: Read> ReadExt for R {
    fn read_exact_n<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut bytes: [u8; N] = [0u8; N];
        self.read_exact(&mut bytes)?;
        Ok(bytes)
    }
}
