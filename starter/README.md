# Signal Notebook — your Rust development bench

A dependency-free CLI and library for learning ownership, validated types, error handling, iterators, and tests. Built and checked with Rust 1.99.1, edition 2024, on Apple Silicon macOS.

## Start

Open this folder in Zed, or use a terminal:

```sh
cd "/Users/daedalus/Documents/Codex/2026-10-31/ca/outputs/rust-studio/starter"
source "$HOME/.cargo/env"
cargo run -- scores.txt
```

Expected output: `4 scores · mean 62.50`.

The program accepts exactly one file path. Each nonblank line must contain an integer from 0 to 100. It reports the first invalid line, returns a failing exit status on errors, and handles empty input without inventing a mean. Input must be UTF-8. It reads the full file into memory; streaming is a suggested exercise rather than an implemented capability.

## Work

```sh
cargo check --locked
cargo fmt
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo nextest run --locked
cargo test --doc --locked
cargo llvm-cov --html
cargo doc --no-deps --open
bacon
```

Nextest runs the eight unit/integration tests; the documentation test runs with `cargo test --doc`. Coverage HTML appears in `target/llvm-cov/html/index.html`. Cargo docs appear in `target/doc/signal_notebook/index.html`.

Use `cargo audit` and `cargo deny check` for dependency checks. The included deny.toml is an example policy only. It ignores the unpublished local crate for license checking; with no external dependencies, its three example license allowances generate benign unmatched-license warnings. Change the policy deliberately when adding dependencies.

## Navigate

- `src/lib.rs`: Score validation, typed parse error, parser, mean and unit tests.
- `src/main.rs`: OS-native path argument, file I/O, output and exit status.
- `tests/cli.rs`: successful invocation and missing-argument failure.
- `scores.txt`: sample input.
- `rust-toolchain.toml`: exact compiler and component pin.
- `.zed/settings.json`: project-local Clippy-on-save configuration.

The package is marked `publish = false`. No license or publication decision has been made for your future project. Add appropriate metadata if you turn it into a real package.

## Debug

```sh
cargo build
rust-lldb target/debug/signal-notebook
```

Inside LLDB:

```text
breakpoint set --name signal_notebook::main
run scores.txt
next
frame variable
continue
```

Breakpoint, launch and continue were verified. Apple's LLDB reports no Rust language plugin, so some frame inspection is limited. Zed supports CodeLLDB and may fetch the adapter on first use; a Zed GUI debugging session was not tested.

## Practice

1. Add `maximum(&[Score]) -> Option<Score>` and test empty, singleton and repeated maxima.
2. Add comment lines while preserving physical line numbers in errors.
3. Compare a streaming parser with whole-file loading on representative input.
4. Explain each ownership experiment in the adjacent HTML primer before revealing its result.

Keep Cargo.lock with this application. Use `--offline` for core builds and tests once the pinned toolchain is installed; this starter has no registry dependencies. Security checks still need their advisory data refreshed periodically.
