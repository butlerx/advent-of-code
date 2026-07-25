# Systems Check

Solutions for [systemscheck.dev](https://systemscheck.dev) missions, written in
Rust.

## Structure

- `src/main.rs` — CLI entrypoint, dispatches to the selected mission solver.
- `src/mission_XX.rs` — Solution for each mission (parts 1 and 2).
- `input/XX.txt` — Puzzle input for each mission.

## Running

From the workspace root:

```sh
cargo run --package systems_check -- --day 1
```

Run a single part:

```sh
cargo run --package systems_check -- --day 1 --part 1
```

## Testing

```sh
cargo test --package systems_check
```
