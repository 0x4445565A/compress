use clap::ValueEnum;
use flate2::{
    write::DeflateDecoder, write::DeflateEncoder, write::GzDecoder, write::GzEncoder,
    write::ZlibDecoder, write::ZlibEncoder, Compression,
};

use std::io::{self, BufRead, Write};

#[derive(Debug, Clone, ValueEnum)]
pub enum Algorithms {
    GZIP,
    ZLIB,
    DEFLATE,
}

impl Algorithms {
    pub fn get_stdin_buf() -> io::Result<Vec<u8>> {
        let mut stdin = io::stdin().lock();
        let buf = stdin.fill_buf()?;
        Ok(buf.to_vec())
    }

    pub fn compress(&self, buf: &[u8]) -> io::Result<Vec<u8>> {
        match self {
            Self::GZIP => GzEncoder::new(Vec::new(), Compression::default()).run(buf),
            Self::ZLIB => ZlibEncoder::new(Vec::new(), Compression::default()).run(buf),
            Self::DEFLATE => DeflateEncoder::new(Vec::new(), Compression::default()).run(buf),
        }
    }

    pub fn decompress(&self, buf: &[u8]) -> io::Result<Vec<u8>> {
        match self {
            Self::GZIP => GzDecoder::new(Vec::new()).run(buf),
            Self::ZLIB => ZlibDecoder::new(Vec::new()).run(buf),
            Self::DEFLATE => DeflateDecoder::new(Vec::new()).run(buf),
        }
    }
}

trait Method {
    fn run(mut self, buf: &[u8]) -> io::Result<Vec<u8>>
    where
        Self: Sized,
    {
        self.buffer(buf)?;
        self.encode()
    }

    fn encode(self) -> io::Result<Vec<u8>>;
    fn buffer(&mut self, buf: &[u8]) -> io::Result<()>;
}

// GZip
impl Method for GzEncoder<Vec<u8>> {
    fn encode(self) -> io::Result<Vec<u8>> {
        self.finish()
    }
    fn buffer(&mut self, buf: &[u8]) -> io::Result<()> {
        self.write_all(buf)
    }
}

impl Method for GzDecoder<Vec<u8>> {
    fn encode(self) -> io::Result<Vec<u8>> {
        self.finish()
    }
    fn buffer(&mut self, buf: &[u8]) -> io::Result<()> {
        self.write_all(buf)
    }
}

// Zlib
impl Method for ZlibEncoder<Vec<u8>> {
    fn encode(self) -> io::Result<Vec<u8>> {
        self.finish()
    }
    fn buffer(&mut self, buf: &[u8]) -> io::Result<()> {
        self.write_all(buf)
    }
}

impl Method for ZlibDecoder<Vec<u8>> {
    fn encode(self) -> io::Result<Vec<u8>> {
        self.finish()
    }
    fn buffer(&mut self, buf: &[u8]) -> io::Result<()> {
        self.write_all(buf)
    }
}

// Deflate
impl Method for DeflateEncoder<Vec<u8>> {
    fn encode(self) -> io::Result<Vec<u8>> {
        self.finish()
    }
    fn buffer(&mut self, buf: &[u8]) -> io::Result<()> {
        self.write_all(buf)
    }
}

impl Method for DeflateDecoder<Vec<u8>> {
    fn encode(self) -> io::Result<Vec<u8>> {
        self.finish()
    }
    fn buffer(&mut self, buf: &[u8]) -> io::Result<()> {
        self.write_all(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_decompress_gzip() {
        let compare = "Some wacky string that is full of. all. kinds. of ;;; text";
        let compare_data = compare.as_bytes();
        let compressed_data = Algorithms::GZIP.compress(compare_data).unwrap();
        let compressed_data = compressed_data.to_owned();
        let decompressed_data = Algorithms::GZIP.decompress(&compressed_data).unwrap();
        assert_eq!(compare.as_bytes(), decompressed_data);
    }

    #[test]
    fn compress_decompress_zlib() {
        let compare = "Some wacky string that is full of. all. kinds. of ;;; text";
        let compare_data = compare.as_bytes();
        let compressed_data = Algorithms::ZLIB.compress(compare_data).unwrap();
        let compressed_data = compressed_data.to_owned();
        let decompressed_data = Algorithms::ZLIB.decompress(&compressed_data).unwrap();
        assert_eq!(compare.as_bytes(), decompressed_data);
    }

    #[test]
    fn compress_decompress_defalte() {
        let compare = "Some wacky string that is full of. all. kinds. of ;;; text";
        let compare_data = compare.as_bytes();
        let compressed_data = Algorithms::DEFLATE.compress(compare_data).unwrap();
        let compressed_data = compressed_data.to_owned();
        let decompressed_data = Algorithms::DEFLATE.decompress(&compressed_data).unwrap();
        assert_eq!(compare.as_bytes(), decompressed_data);
    }
}
