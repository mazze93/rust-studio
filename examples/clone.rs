fn main() {
    let first = String::from("signal");
    let mut second = first.clone();
    second.push('!');
    assert_eq!(first, "signal");
    assert_eq!(second, "signal!");
}
