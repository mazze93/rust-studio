# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Posture: **HIGH** (`apps/`, public site at rust.mazzeleczzare.com). Registered in
`~/Projects/WORKSPACE.md`.

## What this is

A Rust learning kit whose website cannot claim a compiler result it did not
observe. Rust is the source of truth, and Astro only renders what Rust recorded:

```
examples/*.rs ──┐
starter/ ───────┼─► studio-verify (verify/, Rust) ─► site/src/data/verified.json ─► Astro (site/) ─► Workers Static Assets
pins ───────────┘        compiles, runs, checks            committed, CI-checked          static HTML only
```

- **`// expect:` on line 1 of every example** (`pass` or exactly one `E0000`
  code). `studio-verify` fails if a passing example emits any diagnostic
  (warnings are denied), exits non-zero, or if a failing one raises any error
  set other than exactly its declared code. Diagnostic line numbers include the
  header line, and the site shows the file verbatim so they match.
- **`verified.json` is generated, never hand-edited.** CI's `check` mode
  recompiles and fails on any byte difference, so a pin bump or example edit
  without `-- write` fails CI.
- **Two toolchain pins, one value.** `rust-toolchain.toml` (root) and
  `starter/rust-toolchain.toml` must name the same exact release (the starter
  keeps its own so it works when copied out alone). `studio-verify` enforces
  this and checks `rustc --version` against it.
- `studio-verify` strips the env vars that `cargo run` leaks into its children,
  so each child resolves the toolchain from the files on disk. It compiles from
  the repo root with relative paths, so no absolute path can enter the
  published diagnostics.
- `starter/` is excluded from the root workspace. It has its own `Cargo.lock`,
  is dependency-free, and has `publish = false`. Its README and sources are
  rendered on `/starter/` via Vite `?raw` and markdown imports from outside
  `site/` (`vite.server.fs.allow: [".."]`).

## Commands

```sh
cargo run -p studio-verify                  # check mode (CI): is verified.json current?
cargo run -p studio-verify -- write         # regenerate after editing examples/ or a pin
cargo test --locked -p studio-verify <name> # single verifier unit test
cargo clippy --locked --all-targets -- -D warnings && cargo fmt --all -- --check

cd starter && cargo test --locked <name>    # single starter test; nextest also works
cd starter && cargo clippy --locked --all-targets -- -D warnings

cd site && npm run dev | npm run check | npm run build
cd site && npm run deploy:dry               # build + wrangler validation, no upload
cd site && npm run deploy                   # verify → astro check → build → wrangler deploy
```

`cargo` may not be on PATH in a non-interactive shell; run
`source "$HOME/.cargo/env"` first.

## Site constraints that are easy to break

- **CSP is `script-src 'self'; style-src 'self'`** (`site/public/_headers`).
  Everything must stay external:
  - `vite.build.assetsInlineLimit: 0` stops Vite inlining small scripts.
  - `markdown.syntaxHighlight: false`, because Shiki emits `style=""` attributes.
  - No `define:vars` and no `style=` attributes.
  - CI greps `dist/` for inline `<script>`, `<style>` and `style=`.
- **No Worker script.** `wrangler.jsonc` has `assets` only, with no `main`, so
  requests never invoke compute. Other settings:
  - `workers_dev` and `preview_urls` are off.
  - The custom domain is declared in config.
  - `html_handling: auto-trailing-slash` matches Astro's `trailingSlash: "always"`.
- `_headers`: `/_astro/*` uses `! Cache-Control` to drop the `/*` revalidate
  rule before setting `immutable`. Without it Cloudflare merges both values.
- **Never `studio.mazzeleczzare.com`.** That is an unrelated live site; this
  one is `rust.`.
- Prose in `site/src/chapters/*.astro` is HTML inside Astro. A literal `{` or
  `}` in text must be written `&#123;` / `&#125;`. Rust code belongs in
  `examples/` and is rendered by `<Example name="…" />`, never pasted inline.
- `site/src/data/sources.json` drives the reading-room list. Each entry carries
  the date it was consulted (`accessed`); update it when you re-read a source.

## Records

`evidence/2026-10-31/` is the original Codex-built delivery record (Rust
1.99.1). Its absolute `~/Documents/Codex/...` paths are historical; leave them
as they are, and don't follow them either. The bytes it checksums are the
first commit.
