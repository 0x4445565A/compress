# Compress
A simple CLI compression utility powered by Rust.

## Why
I wanted to create a useful program that was written in Rust.  Primarily meant as a learning experience in writing tools in Rust.

## Constraints
  - Written in Rust
  - Must have pretty CLI args
  - Must be able to compress and decompress input
  - Must be expandable to support multiple algorithms

## Usage
```
Simple compress tool for quick CLI STDIN compression/decompression

Usage: compress [OPTIONS]

Options:
  -d, --decompress             Decompress the IO stream
  -a, --algorithm <ALGORITHM>  Which algorithm should we use [default: gzip] [possible values: gzip, zlib, deflate]
  -h, --help                   Print help
  -V, --version                Print version
```

```
# This should compress and decompress the source code using gzip
cat src/main.rs | ./compress | ./compress -d

# This should compress and decompress the source code using zlib
cat src/main.rs | ./compress -a zlib | ./compress -d -a zlib
```

## Build
wowie, what crazy documentation
`cargo build --release`