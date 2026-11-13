//! `studio-verify` — the Rust half of the site build.
//!
//! Every example under `examples/` declares its expected outcome on its first
//! line (`// expect: pass` or `// expect: E0382`). This tool compiles each one
//! with the pinned rustc, runs the ones that should pass, and records the real
//! source, stdout and diagnostics in `site/src/data/verified.json`. The Astro
//! site renders only that file, so the page cannot show code or an error the
//! compiler did not actually produce.
//!
//! Modes:
//!   `write` — regenerate the JSON.
//!   `check` — (default) regenerate in memory and fail if the committed JSON
//!             differs. CI runs this, so a stale page fails the build.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const DATA_PATH: &str = "site/src/data/verified.json";
const EDITION: &str = "2024";

type Fallible<T> = Result<T, String>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expect {
    Pass,
    Error(String),
}

impl Expect {
    fn parse(first_line: &str) -> Fallible<Self> {
        let value = first_line
            .strip_prefix("// expect:")
            .ok_or("first line must be `// expect: pass` or `// expect: E0000`")?
            .trim();
        match value {
            "pass" => Ok(Self::Pass),
            code if is_error_code(code) => Ok(Self::Error(code.to_owned())),
            other => Err(format!("unrecognised expectation `{other}`")),
        }
    }

    fn label(&self) -> &str {
        match self {
            Self::Pass => "pass",
            Self::Error(code) => code,
        }
    }
}

fn is_error_code(code: &str) -> bool {
    code.len() == 5 && code.starts_with('E') && code[1..].bytes().all(|b| b.is_ascii_digit())
}

struct Verified {
    name: String,
    expect: Expect,
    source: String,
    stdout: String,
    stderr: String,
}

fn main() -> ExitCode {
    let mode = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "check".to_owned());
    match run(&mode) {
        Ok(message) => {
            println!("{message}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("studio-verify: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(mode: &str) -> Fallible<String> {
    if mode != "write" && mode != "check" {
        return Err(format!("unknown mode `{mode}`; use `write` or `check`"));
    }
    let root = repo_root()?;
    let toolchain = consistent_pin(&root)?;
    let rustc = rustc_version(&root, &toolchain)?;
    let examples = verify_examples(&root)?;
    let starter_stdout = run_starter(&root)?;
    let json = render_json(&toolchain, &rustc, &examples, &starter_stdout);

    let data_path = root.join(DATA_PATH);
    let summary = format!(
        "{} examples verified with {rustc}; starter prints `{}`",
        examples.len(),
        starter_stdout.trim_end()
    );
    if mode == "write" {
        fs::write(&data_path, &json).map_err(|e| format!("writing {DATA_PATH}: {e}"))?;
        return Ok(format!("wrote {DATA_PATH}: {summary}"));
    }
    let committed = fs::read_to_string(&data_path).unwrap_or_default();
    if committed != json {
        return Err(format!(
            "{DATA_PATH} is stale: it does not match what {rustc} produces now.\n\
             Run `cargo run -p studio-verify -- write` and commit the result."
        ));
    }
    Ok(format!("{DATA_PATH} is current: {summary}"))
}

fn repo_root() -> Fallible<PathBuf> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "verify/ has no parent directory".to_owned())
}

fn read_pin(path: &Path) -> Fallible<String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    text.lines()
        .filter_map(|line| line.trim().strip_prefix("channel"))
        .filter_map(|rest| rest.trim().strip_prefix('='))
        .map(|value| value.trim().trim_matches('"').to_owned())
        .next()
        .ok_or_else(|| format!("{}: no `channel = \"…\"` line", path.display()))
}

/// The root pin and the starter's own pin must name the same exact release.
fn consistent_pin(root: &Path) -> Fallible<String> {
    let root_pin = read_pin(&root.join("rust-toolchain.toml"))?;
    let starter_pin = read_pin(&root.join("starter/rust-toolchain.toml"))?;
    if root_pin != starter_pin {
        return Err(format!(
            "toolchain pins disagree: rust-toolchain.toml = {root_pin}, \
             starter/rust-toolchain.toml = {starter_pin}"
        ));
    }
    let exact = root_pin.split('.').count() == 3
        && root_pin.split('.').all(|part| part.parse::<u32>().is_ok());
    if !exact {
        return Err(format!(
            "pin `{root_pin}` must be an exact release such as 1.100.0"
        ));
    }
    Ok(root_pin)
}

/// A command with the environment of the `cargo run` that launched us removed,
/// so child tools resolve the toolchain from the files on disk.
fn clean_command(program: &str, dir: &Path) -> Command {
    let mut command = Command::new(program);
    command.current_dir(dir);
    for (key, _) in std::env::vars_os() {
        let Some(key) = key.to_str() else { continue };
        let inherited = key == "CARGO"
            || key == "RUSTUP_TOOLCHAIN"
            || key == "CARGO_PRIMARY_PACKAGE"
            || [
                "CARGO_PKG_",
                "CARGO_MANIFEST_",
                "CARGO_CRATE_",
                "CARGO_BIN_",
            ]
            .iter()
            .any(|prefix| key.starts_with(prefix));
        if inherited {
            command.env_remove(key);
        }
    }
    command
}

fn output_text(bytes: &[u8], what: &str) -> Fallible<String> {
    String::from_utf8(bytes.to_vec()).map_err(|_| format!("{what} was not UTF-8"))
}

fn rustc_version(root: &Path, toolchain: &str) -> Fallible<String> {
    let output = clean_command("rustc", root)
        .arg("--version")
        .output()
        .map_err(|e| format!("running rustc: {e}"))?;
    let version = output_text(&output.stdout, "rustc --version")?
        .trim()
        .to_owned();
    if !version.starts_with(&format!("rustc {toolchain} ")) {
        return Err(format!("expected rustc {toolchain}, found `{version}`"));
    }
    Ok(version)
}

fn verify_examples(root: &Path) -> Fallible<Vec<Verified>> {
    let mut paths: Vec<PathBuf> = fs::read_dir(root.join("examples"))
        .map_err(|e| format!("examples/: {e}"))?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "rs"))
        .collect();
    paths.sort();
    if paths.is_empty() {
        return Err("examples/ contains no .rs files".to_owned());
    }
    let out_dir = root.join("target/examples");
    fs::create_dir_all(&out_dir).map_err(|e| format!("{}: {e}", out_dir.display()))?;
    paths
        .iter()
        .map(|path| verify_one(root, path, &out_dir))
        .collect()
}

fn verify_one(root: &Path, path: &Path, out_dir: &Path) -> Fallible<Verified> {
    let name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| format!("{}: unusable file name", path.display()))?
        .to_owned();
    let relative = format!("examples/{name}.rs");
    let source = fs::read_to_string(path).map_err(|e| format!("{relative}: {e}"))?;
    let expect = Expect::parse(source.lines().next().unwrap_or_default())
        .map_err(|e| format!("{relative}: {e}"))?;

    let binary = out_dir.join(&name);
    // Relative input path from the repo root: diagnostics then cite
    // `examples/x.rs`, never an absolute path from whichever machine built it.
    let compile = clean_command("rustc", root)
        .args(["--edition", EDITION, "--color=never", "-D", "warnings"])
        .arg(&relative)
        .arg("-o")
        .arg(&binary)
        .output()
        .map_err(|e| format!("{relative}: running rustc: {e}"))?;
    let diagnostics = output_text(&compile.stderr, "rustc diagnostics")?;
    let codes: BTreeSet<&str> = diagnostics
        .match_indices("error[")
        .filter_map(|(at, _)| diagnostics.get(at + 6..at + 11))
        .filter(|code| is_error_code(code))
        .collect();

    let stdout = match &expect {
        Expect::Pass => {
            if !compile.status.success() || !diagnostics.is_empty() {
                return Err(format!(
                    "{relative}: expected a clean compile, rustc said:\n{diagnostics}"
                ));
            }
            let run = Command::new(&binary)
                .output()
                .map_err(|e| format!("{relative}: running binary: {e}"))?;
            if !run.status.success() {
                return Err(format!(
                    "{relative}: compiled but exited with {}",
                    run.status
                ));
            }
            output_text(&run.stdout, "example stdout")?
        }
        Expect::Error(code) => {
            let only_expected = codes.len() == 1 && codes.contains(code.as_str());
            if compile.status.success() || !only_expected {
                return Err(format!(
                    "{relative}: expected exactly {code}, rustc reported {codes:?}:\n{diagnostics}"
                ));
            }
            String::new()
        }
    };
    Ok(Verified {
        name,
        expect,
        source,
        stdout,
        stderr: diagnostics,
    })
}

fn run_starter(root: &Path) -> Fallible<String> {
    let output = clean_command("cargo", &root.join("starter"))
        .args(["run", "--locked", "--quiet", "--", "scores.txt"])
        .output()
        .map_err(|e| format!("starter: running cargo: {e}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "starter: `cargo run -- scores.txt` failed:\n{stderr}"
        ));
    }
    output_text(&output.stdout, "starter stdout")
}

fn json_string(out: &mut String, value: &str) {
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if u32::from(c) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", u32::from(c));
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

fn field(out: &mut String, indent: &str, key: &str, value: &str, last: bool) {
    out.push_str(indent);
    json_string(out, key);
    out.push_str(": ");
    json_string(out, value);
    out.push_str(if last { "\n" } else { ",\n" });
}

fn render_json(toolchain: &str, rustc: &str, examples: &[Verified], starter: &str) -> String {
    let mut out = String::from("{\n");
    field(&mut out, "  ", "toolchain", toolchain, false);
    field(&mut out, "  ", "rustc", rustc, false);
    field(&mut out, "  ", "edition", EDITION, false);
    out.push_str("  \"starter\": {\n");
    field(
        &mut out,
        "    ",
        "command",
        "cargo run -- scores.txt",
        false,
    );
    field(&mut out, "    ", "stdout", starter, true);
    out.push_str("  },\n  \"examples\": [\n");
    for (index, example) in examples.iter().enumerate() {
        out.push_str("    {\n");
        field(&mut out, "      ", "name", &example.name, false);
        field(&mut out, "      ", "expect", example.expect.label(), false);
        field(&mut out, "      ", "source", &example.source, false);
        field(&mut out, "      ", "stdout", &example.stdout, false);
        field(&mut out, "      ", "stderr", &example.stderr, true);
        out.push_str(if index + 1 == examples.len() {
            "    }\n"
        } else {
            "    },\n"
        });
    }
    out.push_str("  ]\n}\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_expectations() {
        assert_eq!(Expect::parse("// expect: pass"), Ok(Expect::Pass));
        assert_eq!(
            Expect::parse("// expect: E0382"),
            Ok(Expect::Error("E0382".to_owned()))
        );
        assert!(Expect::parse("fn main() {}").is_err());
        assert!(Expect::parse("// expect: E38").is_err());
        assert!(Expect::parse("// expect: maybe").is_err());
    }

    #[test]
    fn escapes_json() {
        let mut out = String::new();
        json_string(&mut out, "a\"b\\c\nd\u{1}é");
        assert_eq!(out, r#""a\"b\\c\nd\u0001é""#);
    }

    #[test]
    fn error_codes_are_exact() {
        assert!(is_error_code("E0502"));
        assert!(!is_error_code("E050"));
        assert!(!is_error_code("W0502"));
        assert!(!is_error_code("E05x2"));
    }
}
