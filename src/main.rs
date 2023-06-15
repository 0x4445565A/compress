extern crate atty;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{self, BufRead, Write};

fn main() {
    if atty::is(atty::Stream::Stdin) {
        println!("Requires STDIN... Nothing found");
        return;
    }

    let stdin = io::stdin();
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(line) => format!("{line}\n"),
            Err(error) => panic!("Unable to read STDIN: {error}"),
        };
        match e.write_all(line.as_bytes()) {
            Ok(_) => (),
            Err(error) => panic!("Unable to encode: {error}"),
        };
    }

    let compressed_bytes = match e.finish() {
        Ok(writer) => writer,
        Err(error) => panic!("Unable to encode: {error}"),
    };
    let mut stdout = std::io::stdout();
    match stdout.write(&compressed_bytes) {
        Ok(_) => (),
        Err(error) => panic!("Unable to write to STDOUT: {error}"),
    }
}
