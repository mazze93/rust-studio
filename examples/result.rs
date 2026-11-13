use std::num::ParseIntError;
fn double(input: &str) -> Result<u16, ParseIntError> {
    let value: u8 = input.parse()?;
    Ok(u16::from(value) * 2)
}
fn main() -> Result<(), ParseIntError> {
    assert_eq!(double("128")?, 256);
    assert!(double("invalid").is_err());
    Ok(())
}
