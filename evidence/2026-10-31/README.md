# Delivery record — 2026-10-31

These files are the verification evidence from the original build of this kit
(Rust 1.99.1, Apple Silicon macOS), kept unchanged as a dated record. They are
not live instructions.

- Absolute paths inside them (`~/Documents/Codex/2026-10-31/…`) name the
  directory the kit was built in that day. That directory no longer exists;
  the kit now lives in this repository.
- `SHA256SUMS` covers the files as delivered. Those exact bytes are the
  repository's first commit (`import: rust-studio kit as delivered
  2026-10-31`); check them with `git show <that commit>:<path> | shasum -a 256`,
  not against the working tree, which has moved on.
- Current verification is not a document: `cargo run -p studio-verify` and CI
  (`.github/workflows/ci.yml`) re-check every example and the starter on each
  push, against the toolchain pinned in `rust-toolchain.toml`.
