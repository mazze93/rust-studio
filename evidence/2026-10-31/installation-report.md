# Rust development kit — installation and verification

Verified on **31 October 2026**, America/New_York. Host: Apple Silicon, macOS 27.0.1. This is a complete general-purpose native Rust development setup, with project-specific SDKs and advanced nightly tools left as optional extensions.

## Installed or completed

| Tool/component | Version or channel | Work performed |
|---|---|---|
| Rust / Cargo | 1.99.1 | Existing stable checked against the live channel; already current. |
| Exact Rust toolchain | 1.99.1 | Added alongside stable for the starter's reproducible pin. |
| rust-src | stable and 1.99.1 | Added standard-library sources for editor analysis. |
| llvm-tools-preview | stable and 1.99.1 | Added LLVM tools for source coverage. |
| rustfmt, Clippy, rust-analyzer, offline docs | 1.99.1 toolchain | Existing stable components verified; included in exact toolchain. |
| cargo-nextest | 0.9.150 | Installed Homebrew binary bottle. |
| cargo-llvm-cov | 0.9.1 | Installed Homebrew binary bottle. |
| cargo-deny | 0.20.2 | Installed Homebrew binary bottle. |
| Bacon | 3.26.0, Homebrew revision 1 | Installed Homebrew binary bottle. |

Homebrew formula metadata and pinned formula source were inspected before installation. The recipes referenced the expected upstream projects, declared checksums, and supplied compatible Apple Silicon bottles. The install used those bottles, not local source compilation. No third-party tap was trusted or enabled. This provenance check is not an audit of the tools' entire dependency trees.

## Existing tools retained

- rustup 1.29.2, managed by Homebrew.
- cargo-audit 0.22.2 and cargo-about 0.9.2, already installed under ~/.cargo/bin.
- Zed 1.26.2, with native Rust support.
- Xcode's Clang and LLDB; LLDB reports version 2103.0.34.407.
- Existing 1.98.1 toolchain.
- Stable target libraries for aarch64-apple-darwin, x86_64-apple-darwin, and wasm32-unknown-unknown.

The automation shell initially lacked ~/.cargo/bin in PATH. Inspection showed that the user's interactive zsh configuration already contained it. An interactive zsh check resolved Cargo successfully. No shell configuration was changed. `source "$HOME/.cargo/env"` activates it for a shell that does not load that configuration.

## Deliverables

- `rust-primer.html`: 15 chapters, six ownership experiments, 14 standalone Rust examples, 26 researched source links, topic search, copy controls and local exercise progress. Self-contained, with no remote resources loaded by the page.
- `starter/`: dependency-free CLI plus library, sample input, tests, documentation, exact toolchain pin, project-local Zed configuration, example dependency policy and Cargo.lock. `publish = false` is set.
- `examples/`: original teaching programs and expected compiler outcomes.
- `sources.json`: machine-readable sources, annotations and access date.
- `tool-inventory.json`, `validation.json`, `validation.log`, `dependency-validation.json`, `dependency-validation.log`, `browser-validation.json`: installation and verification evidence.

## Verified outcomes

1. **Compilation and execution:** all 11 positive standalone examples compiled with Rust 1.99.1 / edition 2024 and ran successfully. The three negative examples produced the intended E0382, E0502 and E0515 diagnostics.
2. **Starter tests:** six unit tests, two CLI integration tests and one documentation test passed. Nextest independently ran the eight unit/integration tests successfully. The documentation test was also checked separately.
3. **Build quality:** cargo check, rustfmt check, Clippy with warnings denied, a release build, and a sample CLI run succeeded. Cargo documentation generated successfully.
4. **Coverage:** cargo-llvm-cov generated HTML. The observed combined line coverage was 97.67% and region coverage was 97.01%; library line/region coverage was 100%. These figures do not establish that all behavior is correct. Stable coverage did not measure doc tests or branch coverage here.
5. **Dependency tools:** cargo audit and cargo deny completed with exit status 0 after fetching public advisory data. The project has no third-party dependencies, so this verifies the workflow on a small input, not a substantive supply-chain review. cargo-deny emitted three unmatched-license-allowance warnings from the example policy.
6. **Debugger:** rust-lldb launched the sample, resolved and hit a breakpoint at signal_notebook::main, continued, printed the expected result, and exited with status 0.
7. **HTML:** all six prediction paths worked; topic filtering and progress toggling worked; copy-success and clipboard-denied fallback branches were checked with simulated clipboard behavior. Desktop layout and a 390-pixel iframe viewport had no document horizontal overflow. Internal anchor targets resolved. No external resources were loaded. Desktop rendering was visually inspected.

## Limits and corrected attempts

- Apple's LLDB emitted a warning that it has no Rust language plugin. Breakpoints and process control worked; some frame-variable inspection remains limited. Zed's CodeLLDB GUI session was not tested and may fetch its adapter on first use.
- Bacon's executable/version/help were verified; a long-running edit/watch session was not exercised.
- The first saved dependency verification used an unsupported `cargo audit --offline` flag and ran cargo-deny where its advisory lock path was read-only. Those failures remain visible in `validation.json` / `validation.log`. Corrected checks with supported flags and writable locks passed and are recorded separately in `dependency-validation.*`.
- The mobile check used an iframe viewport, not a physical phone. This is not an exhaustive accessibility or cross-browser audit.
- Rust 1.99.1 was the current stable release at verification time. The HTML is a dated snapshot, not a self-updating product. [Official release announcement](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/).
- Nightly, Miri, cargo-fuzz and platform-specific SDKs were not installed. Existing cross-target libraries were preserved; full cross-compilation/deployment workflows were not validated.
- No existing project, global editor configuration, shell profile, publication setting outside the new project, or repository was changed. Nothing was published or deployed.

## Maintenance and recovery

Rustup manages compiler/toolchain components; Homebrew manages the four new utilities. Keep that ownership distinction when updating.

```sh
rustup check
rustup update stable --no-self-update
brew outdated cargo-nextest cargo-deny bacon cargo-llvm-cov
brew upgrade cargo-nextest cargo-deny bacon cargo-llvm-cov
cargo install --list
```

Update the starter's rust-toolchain.toml deliberately when moving to a new compiler. Stable updates do not change that exact pin.

If reversing this installation, the four Homebrew packages can be uninstalled individually. The added exact toolchain can be removed with `rustup toolchain uninstall 1.99.1` after removing or changing project pins that require it. Do not remove pre-existing stable components merely to reverse this kit: other projects may now use them. No removal was performed.
