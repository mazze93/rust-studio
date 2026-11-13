use std::process::Command;

#[test]
fn summarizes_sample() {
    let result = Command::new(env!("CARGO_BIN_EXE_signal-notebook"))
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/scores.txt"))
        .output()
        .unwrap();
    assert!(result.status.success());
    assert_eq!(
        String::from_utf8(result.stdout).unwrap(),
        "4 scores · mean 62.50\n"
    );
}

#[test]
fn missing_argument_is_a_failure() {
    let result = Command::new(env!("CARGO_BIN_EXE_signal-notebook"))
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8(result.stderr).unwrap().contains("usage:"));
}
