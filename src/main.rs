mod algo;

use clap::Parser;
use std::io::{self, Write};
use std::str;

/// Simple compress tool for quick CLI STDIN compression/decompression
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Decompress the IO stream
    #[clap(short, long, default_value_t = false)]
    decompress: bool,
    /// Which algorithm should we use
    #[clap(short, long, value_enum, default_value_t = algo::Algorithms::GZIP)]
    algorithm: algo::Algorithms,
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
