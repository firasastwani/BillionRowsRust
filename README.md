## onebrc-rust

Rust playground for the [1BRC](https://github.com/gunnarmorling/1brc) (One Billion Row Challenge) temperature aggregation task.

This project intentionally does **not** contain any solution code yet; it's just the scaffolding so you can focus on implementing and optimizing your own Rust solution.

### Layout

- `onebrc-rust/` – Rust project root.  
- `onebrc-rust/src/main.rs` – starting point for your implementation.  
- `data/measurements.txt` (workspace root) – sample dataset with 10,000,000 measurements generated from the official 1BRC generator (about 150 MiB).

### How to run and time

From the workspace root:

```bash
cd onebrc-rust

# Debug build (faster to compile, slower to run)
time cargo run

# Release build (recommended for benchmarking)
time cargo run --release
```

By default `src/main.rs` just prints `"Hello, world!"`. Replace that with your own implementation that:

1. Reads the `measurements.txt` file (for example from `../data/measurements.txt` or a path passed via CLI args).  
2. Computes min/mean/max temperature per station.  
3. Prints the result in the same format as described in the 1BRC README.

# BillionRowsRust
