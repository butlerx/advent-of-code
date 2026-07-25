# aoc_shared

Shared library for the Advent of Code workspace. Provides reusable types and
helpers used across all year crates.

## Exports

- `Grid` — 2D grid structure for storing and manipulating values.
- `Point` — 2D integer coordinates with neighbor and distance utilities.
- `Args` — CLI argument parser for selecting day/part.
- `read_input` — Reads puzzle input from a crate's `input/` directory.
- `time_execution` / `time_execution_us` — Timer helpers for benchmarking
  solutions.

## Binaries

### `new_year`

Scaffolds a new year's directory with the standard layout:

```sh
cargo run --package aoc_shared --bin new_year [YEAR]
```

If `YEAR` is omitted, the current calendar year is used. Creates:

```
<year>/
├── Cargo.toml
├── input/
│   ├── 01.txt ... 12.txt
└── src/
    ├── lib.rs
    ├── main.rs
    └── day_01.rs ... day_12.rs
```
