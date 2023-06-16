# compress
A quick CLI compression utility powered by Rust.

## Why
I wanted to create a useful program that was written in Rust.  Primarily meant as a learning experience in writing tools in Rust.

## Constraints
  - Written in Rust
  - Must have pretty CLI args
  - Must be able to compress and decompress input
  - Must be expandable to support multiple algorithms

## Usage
```
# This should compress and decompress the source code
cat src/main.rs | ./compress | ./compress -d
```
