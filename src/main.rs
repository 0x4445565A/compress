use clap::{Parser, ValueEnum};
use flate2::{
    write::GzDecoder, write::GzEncoder, write::ZlibDecoder, write::ZlibEncoder, Compression,
};

use std::io::{self, BufRead, Write};
use std::str;

/// Simple compress tool for quick CLI STDIN compression/decompression
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Decompress the IO stream
    #[clap(short, long, default_value_t = false)]
    decompress: bool,
    /// Which algorithm should we use
    #[clap(short, long, value_enum, default_value_t = Algorithms::GZIP)]
    algorithm: Algorithms,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if atty::is(atty::Stream::Stdin) {
        use clap::CommandFactory;
        let mut cmd = Args::command();
        match cmd.print_help() {
            Ok(_) => (),
            Err(error) => panic!("{error}"),
        }
        println!("Requires STDIN... Nothing found");
        return Ok(());
    }

    let c = if args.decompress {
        args.algorithm.decompress()
    } else {
        args.algorithm.compress()
    };
    let c = match c {
        Ok(writer) => writer,
        Err(error) => panic!("Unable to encode: {error}"),
    };

    let mut stdout = std::io::stdout();
    match stdout.write(&c) {
        Ok(_) => Ok(()),
        Err(error) => panic!("Unable to write to STDOUT: {error}"),
    }
}

trait Encoder {
    fn run(mut self) -> io::Result<Vec<u8>>
    where
        Self: Sized,
    {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        let buf = stdin.fill_buf().unwrap();
        self.buffer(buf)?;
        self.encode()
    }

    fn encode(self) -> io::Result<Vec<u8>>;
    fn buffer(&mut self, buf: &[u8]) -> io::Result<()>;
}

impl Encoder for GzEncoder<Vec<u8>> {
    fn encode(self) -> io::Result<Vec<u8>> {
        self.finish()
    }
    fn buffer(&mut self, buf: &[u8]) -> io::Result<()> {
        self.write_all(buf)
    }
}

impl Encoder for GzDecoder<Vec<u8>> {
    fn encode(self) -> io::Result<Vec<u8>> {
        self.finish()
    }
    fn buffer(&mut self, buf: &[u8]) -> io::Result<()> {
        self.write_all(buf)
    }
}

impl Encoder for ZlibEncoder<Vec<u8>> {
    fn encode(self) -> io::Result<Vec<u8>> {
        self.finish()
    }
    fn buffer(&mut self, buf: &[u8]) -> io::Result<()> {
        self.write_all(buf)
    }
}

impl Encoder for ZlibDecoder<Vec<u8>> {
    fn encode(self) -> io::Result<Vec<u8>> {
        self.finish()
    }
    fn buffer(&mut self, buf: &[u8]) -> io::Result<()> {
        self.write_all(buf)
    }
}

#[derive(Debug, Clone, ValueEnum)]
enum Algorithms {
    GZIP,
    ZLIB,
}

impl Algorithms {
    pub fn compress(&self) -> io::Result<Vec<u8>> {
        match self {
            Self::GZIP => GzEncoder::new(Vec::new(), Compression::default()).run(),
            Self::ZLIB => ZlibEncoder::new(Vec::new(), Compression::default()).run(),
        }
    }

    pub fn decompress(&self) -> io::Result<Vec<u8>> {
        match self {
            Self::GZIP => GzDecoder::new(Vec::new()).run(),
            Self::ZLIB => ZlibDecoder::new(Vec::new()).run(),
        }
    }
}
