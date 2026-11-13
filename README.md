# Rust Studio

A working field guide to Rust, published at **https://rust.mazzeleczzare.com**.
Every code sample and compiler message on the site was produced by the
pinned compiler (currently Rust 1.100.0, edition 2024) when the site was built.

| Path | What it is |
|---|---|
| `examples/` | 15 standalone programs. Line 1 declares the outcome: `// expect: pass` or `// expect: E0382`. |
| `verify/` | `studio-verify`, a dependency-free Rust tool that compiles each example and runs the starter. It writes the results to `site/src/data/verified.json`. |
| `starter/` | *Signal Notebook*, a standalone CLI and library for practice. See its README. |
| `site/` | Astro site, fully static, served by Cloudflare Workers Static Assets. |
| `evidence/2026-10-31/` | Dated record of the original build. Historical, not instructions. |

## Work

```sh
cargo run -p studio-verify            # check: is verified.json current?
cargo run -p studio-verify -- write   # regenerate after changing examples or the pin
cd site && npm ci && npm run dev      # local site at http://localhost:4321
```

Moving to a new Rust release:

1. Edit `rust-toolchain.toml` and `starter/rust-toolchain.toml`. They must match.
2. Run `cargo run -p studio-verify -- write`.
3. Review the diff of `verified.json`. It shows exactly which compiler messages changed.

## Deploy

```sh
cd site
npm run deploy:dry   # build + validate wrangler config, no upload
npm run deploy       # verify → astro check → build → wrangler deploy
```

The Worker `rust-studio` has no script. Static assets are served at the edge
without invoking compute. Security and cache headers live in
`site/public/_headers`, and the custom domain is declared in
`site/wrangler.jsonc`.
