# hanoi

⛩️ An implementation of the Tower of Hanoi. Written in Rust.

![Tower of Hanoi](https://upload.wikimedia.org/wikipedia/commons/thumb/0/07/Tower_of_Hanoi.jpeg/300px-Tower_of_Hanoi.jpeg)

This implementation takes about `13 ns` to perform a move. The algorithm for calculating required moves is:
`(2 ^ n - 1)` where `n` is the amount of rings.

That means it will take approximately `~55.86` seconds to complete the puzzle with `32` rings.

## Installation

```
git clone https://github.com/ajmwagar/hanoi
cd hanoi
cargo install --path ./
```

## Usage
- `hanoi --help`: Opens a help menu
- `hanoi 8`: Solves the puzzle with 8 rings
- `hanoi 8 -v`: Solves the puzzle while printing out each step

# Tests & Benchmarks
- `cargo test`: to run tests
- `cargo bench`: run benchmarks
