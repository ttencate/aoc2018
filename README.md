Advent of Code 2018
===================

This repository contains my solutions for the
[Advent of Code 2018](https://adventofcode.com/2018).

I'm using this year's edition to learn the Rust language. Expect to see
unidiomatic code, but hopefully also a positive trend in quality.

The main binary (`src/main.rs`) is a simple program to fetch today's input and
place it in the right directory. It requires a file called `.session_cookie`
containing the value of the `session` cookie from a valid AoC login session.

The solution to each day's puzzle can be run with `cargo run --bin XX` where
`XX` is the zero-padded day number.
