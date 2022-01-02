## AdventOfCode 2021

https://adventofcode.com/2021 solutions for the 25 days in Rust.

I stayed under 10ms execution limit until day 17 (bruteforce), then only focus
on goal, staying in reasonable duration (longest one is below 3 seconds).

I left part 1 solutions, which are less efficient by design, and use sometimes
overly complex methods.

The main() manages input data downloading, so you can pass your session cookie
to retrieve your input data, via:
```sh
cargo run --release -- --day <day> --part <part> --session <session>
```
The local cache in data/ is used otherwise, with my owns inputs.

To measure execution time, use:
```sh
cargo run --release -- --bench
```
