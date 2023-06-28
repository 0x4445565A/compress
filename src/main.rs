mod algo;

use algo::Algorithms;
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
        cmd.print_help()?;

        println!("Requires STDIN... Nothing found");
        return Ok(());
    }

    let buf = Algorithms::get_stdin_buf()?;
    let buf = buf.to_owned();
    let c = if args.decompress {
        args.algorithm.decompress(&buf)?
    } else {
        args.algorithm.compress(&buf)?
    };

    let mut stdout = std::io::stdout();
    match stdout.write(&c) {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}
