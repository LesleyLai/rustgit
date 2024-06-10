use flate2::read::ZlibDecoder;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

/// A reader for git objects
///
/// This object handles buffered file reading and zlib decoding
pub struct ObjectReader(BufReader<ZlibDecoder<File>>);

impl ObjectReader {
    pub(crate) fn from_file(file: File) -> Self {
        ObjectReader(BufReader::new(ZlibDecoder::new(file)))
    }
}

impl Read for ObjectReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl BufRead for ObjectReader {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.0.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.0.consume(amt)
    }

    fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        self.0.read_until(byte, buf)
    }
}
