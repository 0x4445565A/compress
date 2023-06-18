use clap::{Parser, ValueEnum};
use flate2::{
    read::GzDecoder, read::ZlibDecoder, write::GzEncoder, write::ZlibEncoder, Compression,
};

use std::io::{self, BufRead, Read, Write};
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

impl Encoder for ZlibEncoder<Vec<u8>> {
    fn encode(self) -> io::Result<Vec<u8>> {
        self.finish()
    }
    fn buffer(&mut self, buf: &[u8]) -> io::Result<()> {
        self.write_all(buf)
    }
}

trait Decoder {
    fn decode(&mut self, buf: &mut String) -> io::Result<usize>;
}

impl Decoder for GzDecoder<&[u8]> {
    fn decode(&mut self, buf: &mut String) -> io::Result<usize> {
        self.read_to_string(buf)
    }
}

impl Decoder for ZlibDecoder<&[u8]> {
    fn decode(&mut self, buf: &mut String) -> io::Result<usize> {
        self.read_to_string(buf)
    }
}

#[derive(Debug, Clone, ValueEnum)]
enum Algorithms {
    GZIP,
    ZLIB,
}

impl Algorithms {
    fn get_algo(&self) -> impl Encoder {
        match self {
            Self::GZIP => GzEncoder::new(Vec::new(), Compression::default()),
            Self::ZLIB => todo!(),
        }
    }
    pub fn compress(&self) -> io::Result<Vec<u8>> {
        let mut e = self.get_algo();

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = match line {
                Ok(line) => format!("{line}\n"),
                Err(error) => panic!("Unable to read STDIN: {error}"),
            };
            e.buffer(line.as_bytes())?;
        }

        e.encode()
    }

    pub fn decompress(&self) -> io::Result<Vec<u8>> {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        let buf = stdin.fill_buf().unwrap();

        let mut d = match self {
            Self::GZIP => GzDecoder::new(buf),
            Self::ZLIB => todo!(),
        };
        let mut s = String::new();
        d.decode(&mut s)?;

        // Consume the buffer and make sure no one else uses it.
        let len = buf.len();
        stdin.consume(len);

        Ok(s.into_bytes())
    }
}
