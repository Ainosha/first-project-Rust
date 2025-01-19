# Sorting Algorithm Project

This project implements a merge sort algorithm in both C and Rust. It includes a performance test to measure the time taken to sort arrays of random values.

## Project Structure

- `sort.c`: C implementation of the merge sort algorithm.
- `src/main.rs`: Rust implementation of the merge sort algorithm.

## Requirements

- GCC (for compiling C code)
- Rust (for compiling Rust code)

## Lauch code

To test the code

```bash
cargo run --release
```

## Lauch Test

```bash
gcc -O3 -o name_exec .\sort.c
cargo test --release  -- --nocapture 
```
