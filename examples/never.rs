// expect: pass
// Rust 1.100: `!` is a type you can name, and Infallible is now `!`.
fn byte_len(input: &str) -> Result<usize, !> {
    Ok(input.len()) // this operation cannot fail, and the type says so
}
fn main() {
    let Ok(len) = byte_len("signal"); // no Err arm: it cannot exist
    let flag = "--verbose".trim_prefix("--");
    println!("{len} bytes, flag {flag}");
}
