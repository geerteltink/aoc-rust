# adventofcode

https://adventofcode.com/

## Getting started

```bash
# Update dependencies
cargo update

# Run a specific test
cargo test --bin aoc_2023_day_01

# Get the solution
cargo run --bin aoc_2023_day_01

# Get the solution fast !!!
cargo run --release --bin aoc_2023_day_01
```

## Use --release for optimized fast binaries

```bash
$ cargo run --release --bin aoc_2023_day_04
    Finished release [optimized] target(s) in 0.04s
     Running `target/release/aoc_2023_day_04`
Elapsed time: 784.483µs
Answer day 04 part one: 28750 in 784.483µs
Answer day 04 part two: 10212704 in 12.531681622s

$ cargo run --bin aoc_2023_day_04
    Finished dev [unoptimized + debuginfo] target(s) in 0.93s
     Running `target/debug/aoc_2023_day_04`
Elapsed time: 6.892419ms
Answer day 04 part one: 28750 in 6.892419ms
Answer day 04 part two: 10212704 in 28.537935884s
```
