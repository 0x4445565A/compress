use clap::Parser;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};

use std::io::{self, BufRead, Read, Write};
use std::str;

/// Simple compress tool for quick CLI STDIN compression/decompression
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Decompress the IO stream
    #[arg(short, long, default_value_t = false)]
    decompress: bool,
}

fn main() {
    let args = Args::parse();

    if atty::is(atty::Stream::Stdin) {
        use clap::CommandFactory;
        let mut cmd = Args::command();
        match cmd.print_help() {
            Ok(_) => (),
            Err(error) => panic!("{error}"),
        }

        println!("Requires STDIN... Nothing found");
        return;
    }

    if args.decompress {
        decompress();
        return;
    }

    let c = match compress() {
        Ok(writer) => writer,
        Err(error) => panic!("Unable to encode: {error}"),
    };
    let mut stdout = std::io::stdout();
    match stdout.write(&c) {
        Ok(_) => (),
        Err(error) => panic!("Unable to write to STDOUT: {error}"),
    }
}

fn compress() -> Result<Vec<u8>, std::io::Error> {
    let stdin = io::stdin();
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(line) => format!("{line}\n"),
            Err(error) => panic!("Unable to read STDIN: {error}"),
        };
        e.write_all(line.as_bytes())?;
    }

    e.finish()
}

fn decompress() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let buf = stdin.fill_buf().unwrap();

    let mut d: GzDecoder<&[u8]> = GzDecoder::new(buf);
    let mut s = String::new();
    match d.read_to_string(&mut s) {
        Ok(_) => (),
        Err(error) => panic!("Unable to decode: {error}"),
    }
    println!("{s}");

    // Consume the buffer and make sure no one else uses it.
    let len = buf.len();
    stdin.consume(len);
}
